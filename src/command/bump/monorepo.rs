use crate::command::bump::{ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero};

use crate::conventional::changelog::template::{
    MonoRepoContext, PackageBumpContext, PackageContext,
};
use crate::conventional::changelog::ReleaseType;

use crate::conventional::version::{Increment, IncrementCommand};

use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::settings::HookType;
use crate::{settings, CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;

use log::info;
use semver::Prerelease;

use crate::conventional::error::BumpError;
use crate::git::oid::OidOf;

struct PackageBumpData {
    package_name: String,
    package_path: String,
    public_api: bool,
    old_version: Option<HookVersion>,
    new_version: HookVersion,
    increment: Increment,
}

impl CocoGitto {
    pub fn create_monorepo_version(
        &mut self,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        dry_run: bool,
    ) -> Result<()> {
        self.pre_bump_checks()?;
        // Get package bumps
        let bumps = self.get_packages_bumps(pre_release)?;

        // Get the greatest package increment among public api packages
        let increment_from_package_bumps = bumps
            .iter()
            .filter(|bump| bump.public_api)
            .map(|bump| bump.increment)
            .max();

        // Get current global tag
        let old = self.repository.get_latest_tag();
        let old = tag_or_fallback_to_zero(old)?;
        let mut tag = old.bump(
            IncrementCommand::AutoMonoRepoGlobal(increment_from_package_bumps),
            &self.repository,
        )?;
        ensure_tag_is_greater_than_previous(&old, &tag)?;

        if let Some(pre_release) = pre_release {
            tag.version.pre = Prerelease::new(pre_release)?;
        }

        let tag = Tag::create(tag.version, None);

        if dry_run {
            for bump in bumps {
                println!("{}", bump.new_version.prefixed_tag)
            }
            print!("{}", tag);
            return Ok(());
        }

        let mut template_context = vec![];
        for bump in &bumps {
            template_context.push(PackageBumpContext {
                package_name: &bump.package_name,
                package_path: &bump.package_path,
                version: OidOf::Tag(bump.new_version.prefixed_tag.clone()),
                from: bump
                    .old_version
                    .as_ref()
                    .map(|v| OidOf::Tag(v.prefixed_tag.clone()))
                    .unwrap_or_else(|| {
                        let first = self
                            .repository
                            .get_first_commit()
                            .expect("non empty repository");
                        OidOf::Other(first)
                    }),
            })
        }

        let pattern = self.get_revspec_for_tag(&old)?;
        let changelog =
            self.get_monorepo_global_changelog_with_target_version(pattern, tag.clone())?;

        changelog.pretty_print_bump_summary()?;

        let path = settings::changelog_path();
        let template = SETTINGS.get_monorepo_changelog_template()?;

        changelog.write_to_file(
            path,
            template,
            ReleaseType::MonoRepo(MonoRepoContext {
                packages: template_context,
            }),
        )?;

        let current = self.repository.get_latest_tag().map(HookVersion::new).ok();
        let next_version = HookVersion::new(tag.clone());

        let hook_result = self.run_hooks(
            HookType::PreBump,
            current.as_ref(),
            &next_version,
            hooks_config,
            None,
            None,
        );

        self.repository.add_all()?;

        if let Err(err) = hook_result {
            self.stash_failed_version(&tag, err)?;
        }

        self.bump_packages(pre_release, hooks_config, &bumps)?;

        let sign = self.repository.gpg_sign();
        self.repository.commit(
            &format!("chore(version): {}", next_version.prefixed_tag),
            sign,
        )?;

        for bump in &bumps {
            self.repository.create_tag(&bump.new_version.prefixed_tag)?;
        }

        self.repository.create_tag(&tag)?;

        // Run per package post hooks
        for bump in bumps {
            let package = SETTINGS
                .packages
                .get(&bump.package_name)
                .expect("package exists");
            self.run_hooks(
                HookType::PostBump,
                bump.old_version.as_ref(),
                &bump.new_version,
                hooks_config,
                Some(&bump.package_name),
                Some(package),
            )?;
        }

        // Run global post hooks
        self.run_hooks(
            HookType::PostBump,
            current.as_ref(),
            &next_version,
            hooks_config,
            None,
            None,
        )?;

        Ok(())
    }

    // Calculate all package bump
    fn get_packages_bumps(&self, pre_release: Option<&str>) -> Result<Vec<PackageBumpData>> {
        let mut package_bumps = vec![];
        for (package_name, package) in SETTINGS.packages.iter() {
            let old = self.repository.get_latest_package_tag(package_name);
            let old = tag_or_fallback_to_zero(old)?;

            let next_version = old.bump(
                IncrementCommand::AutoPackage(package_name.to_string()),
                &self.repository,
            );

            if let Err(BumpError::NoCommitFound) = next_version {
                continue;
            }

            let mut next_version = next_version.unwrap();

            if let Some(pre_release) = pre_release {
                next_version.version.pre = Prerelease::new(pre_release)?;
            }

            let tag = Tag::create(next_version.version, Some(package_name.to_string()));
            let increment = tag.get_increment_from(&old);

            if let Some(increment) = increment {
                let old_version = if old.is_zero() {
                    None
                } else {
                    Some(HookVersion::new(old))
                };

                package_bumps.push(PackageBumpData {
                    package_name: package_name.to_string(),
                    package_path: package.path.to_string_lossy().to_string(),
                    public_api: package.public_api,
                    old_version,
                    new_version: HookVersion::new(tag),
                    increment,
                })
            }
        }

        Ok(package_bumps)
    }

    // Run pre hooks and generate changelog for each package and git add the generated content
    fn bump_packages(
        &mut self,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        package_bumps: &Vec<PackageBumpData>,
    ) -> Result<()> {
        for bump in package_bumps {
            let package_name = &bump.package_name;
            let old = self.repository.get_latest_package_tag(package_name);
            let old = tag_or_fallback_to_zero(old)?;
            let msg = format!(
                "Bump for package {}, starting from version {old}",
                package_name.bold()
            )
            .white();

            info!("{msg}");

            let mut next_version = old.bump(
                IncrementCommand::AutoPackage(package_name.to_string()),
                &self.repository,
            )?;
            ensure_tag_is_greater_than_previous(&old, &next_version)?;

            if let Some(pre_release) = pre_release {
                next_version.version.pre = Prerelease::new(pre_release)?;
            }

            let tag = Tag::create(next_version.version, Some(package_name.to_string()));
            let pattern = self.get_revspec_for_tag(&old)?;

            let package = SETTINGS
                .packages
                .get(package_name.as_str())
                .expect("package exists");

            let changelog = self.get_package_changelog_with_target_version(
                pattern,
                tag.clone(),
                package_name.as_str(),
            )?;

            changelog.pretty_print_bump_summary()?;

            let path = package.changelog_path();
            let template = SETTINGS.get_package_changelog_template()?;

            let additional_context = ReleaseType::Package(PackageContext {
                package_name: package_name.as_ref(),
            });

            changelog.write_to_file(&path, template, additional_context)?;
            info!("\tChangelog updated {:?}", path);

            let old_version = self
                .repository
                .get_latest_package_tag(package_name)
                .map(HookVersion::new)
                .ok();

            let new_version = HookVersion::new(tag.clone());

            let hook_result = self.run_hooks(
                HookType::PreBump,
                old_version.as_ref(),
                &new_version,
                hooks_config,
                Some(package_name),
                Some(package),
            );

            self.repository.add_all()?;

            // Hook failed, we need to stop here and reset
            // the repository to a clean state
            if let Err(err) = hook_result {
                self.stash_failed_version(&tag, err)?;
            }
        }

        Ok(())
    }
}
