use crate::command::bump::{
    ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero, HookRunOptions,
};

use crate::conventional::changelog::template::{
    MonoRepoContext, PackageBumpContext, PackageContext,
};
use crate::conventional::changelog::ReleaseType;

use crate::conventional::version::{Increment, IncrementCommand};

use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::{settings, CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;

use log::{info, warn};
use semver::Prerelease;
use tera::Tera;

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

struct PackageData {
    package_name: String,
    package_path: String,
    version: Tag,
}

impl CocoGitto {
    #[allow(clippy::too_many_arguments)]
    pub fn create_monorepo_version(
        &mut self,
        increment: IncrementCommand,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        annotated: Option<String>,
        dry_run: bool,
        skip_ci: bool,
        skip_ci_override: Option<String>,
        skip_untracked: bool,
    ) -> Result<()> {
        match increment {
            IncrementCommand::Auto => {
                if SETTINGS.generate_mono_repository_global_tag {
                    self.create_monorepo_version_auto(
                        pre_release,
                        hooks_config,
                        annotated,
                        dry_run,
                        skip_ci,
                        skip_ci_override,
                        skip_untracked,
                    )
                } else {
                    if annotated.is_some() {
                        warn!("--annotated flag is not supported for package bumps without a global tag");
                    }
                    self.create_all_package_version_auto(
                        pre_release,
                        hooks_config,
                        dry_run,
                        skip_ci,
                        skip_ci_override,
                        skip_untracked,
                    )
                }
            }
            _ => self.create_monorepo_version_manual(
                increment,
                pre_release,
                hooks_config,
                annotated,
                dry_run,
                skip_ci,
                skip_ci_override,
                skip_untracked,
            ),
        }
    }

    pub fn create_all_package_version_auto(
        &mut self,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        dry_run: bool,
        skip_ci: bool,
        skip_ci_override: Option<String>,
        skip_untracked: bool,
    ) -> Result<()> {
        self.pre_bump_checks(skip_untracked)?;
        // Get package bumps
        let bumps = self.get_packages_bumps(pre_release)?;

        if bumps.is_empty() {
            print!("No conventional commits found for your packages that required a bump. Changelogs will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skiped.\n");
            return Ok(());
        }

        if dry_run {
            for bump in bumps {
                println!("{}", bump.new_version.prefixed_tag)
            }
            return Ok(());
        }

        let hook_result = self.run_hooks(HookRunOptions::pre_bump().hook_profile(hooks_config));

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&Tag::default(), hook_result);
        self.bump_packages(pre_release, hooks_config, &bumps)?;

        let sign = self.repository.gpg_sign();

        let mut skip_ci_pattern = String::new();

        if skip_ci || skip_ci_override.is_some() {
            skip_ci_pattern = skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
        }

        self.repository.commit(
            &format!("chore(version): bump packages {}", skip_ci_pattern),
            sign,
            true,
        )?;

        for bump in &bumps {
            self.repository.create_tag(&bump.new_version.prefixed_tag)?;
        }

        // Run per package post hooks
        for bump in bumps {
            let package = SETTINGS
                .packages
                .get(&bump.package_name)
                .expect("package exists");

            self.run_hooks(
                HookRunOptions::post_bump()
                    .current_tag(bump.old_version.as_ref())
                    .next_version(&bump.new_version)
                    .hook_profile(hooks_config)
                    .package(&bump.package_name, package),
            )?;
        }

        // Run global post hooks
        self.run_hooks(HookRunOptions::post_bump().hook_profile(hooks_config))?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn create_monorepo_version_auto(
        &mut self,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        annotated: Option<String>,
        dry_run: bool,
        skip_ci: bool,
        skip_ci_override: Option<String>,
        skip_untracked: bool,
    ) -> Result<()> {
        self.pre_bump_checks(skip_untracked)?;
        // Get package bumps
        let bumps = self.get_packages_bumps(pre_release)?;

        if bumps.is_empty() {
            print!("No conventional commits found for your packages that required a bump. Changelogs will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skiped.\n");
            return Ok(());
        }

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
            print!("{tag}");
            return Ok(());
        }

        let mut template_context = vec![];
        for bump in &bumps {
            template_context.push(PackageBumpContext {
                package_name: &bump.package_name,
                package_path: &bump.package_path,
                version: OidOf::Tag(bump.new_version.prefixed_tag.clone()),
                from: Some(
                    bump.old_version
                        .as_ref()
                        .map(|v| OidOf::Tag(v.prefixed_tag.clone()))
                        .unwrap_or_else(|| {
                            let first = self
                                .repository
                                .get_first_commit()
                                .expect("non empty repository");
                            OidOf::Other(first)
                        }),
                ),
            })
        }

        if !SETTINGS.disable_changelog {
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
                    package_lock: false,
                    packages: template_context,
                }),
            )?;
        }

        let current = self.repository.get_latest_tag().map(HookVersion::new).ok();
        let next_version = HookVersion::new(tag.clone());

        let hook_result = self.run_hooks(
            HookRunOptions::pre_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(hooks_config),
        );

        self.repository.add_all()?;

        self.unwrap_or_stash_and_exit(&tag, hook_result);

        self.bump_packages(pre_release, hooks_config, &bumps)?;

        let sign = self.repository.gpg_sign();

        let mut skip_ci_pattern = String::new();

        if skip_ci || skip_ci_override.is_some() {
            skip_ci_pattern = skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
        }

        self.repository.commit(
            &format!(
                "chore(version): {} {}",
                next_version.prefixed_tag, skip_ci_pattern
            ),
            sign,
            true,
        )?;

        for bump in &bumps {
            self.repository.create_tag(&bump.new_version.prefixed_tag)?;
        }

        if let Some(msg_tmpl) = annotated {
            let mut context = tera::Context::new();
            context.insert("latest", &old.version.to_string());
            context.insert("version", &tag.version.to_string());
            let msg = Tera::one_off(&msg_tmpl, &context, false)?;
            self.repository.create_annotated_tag(&tag, &msg)?;
        } else {
            self.repository.create_tag(&tag)?;
        }

        // Run per package post hooks
        for bump in bumps {
            let package = SETTINGS
                .packages
                .get(&bump.package_name)
                .expect("package exists");
            self.run_hooks(
                HookRunOptions::post_bump()
                    .current_tag(bump.old_version.as_ref())
                    .next_version(&bump.new_version)
                    .hook_profile(hooks_config)
                    .package(&bump.package_name, package),
            )?;
        }

        // Run global post hooks
        self.run_hooks(
            HookRunOptions::post_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(hooks_config),
        )?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn create_monorepo_version_manual(
        &mut self,
        increment: IncrementCommand,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        annotated: Option<String>,
        dry_run: bool,
        skip_ci: bool,
        skip_ci_override: Option<String>,
        skip_untracked: bool,
    ) -> Result<()> {
        self.pre_bump_checks(skip_untracked)?;
        // Get package bumps
        let bumps = self.get_current_packages()?;

        // Get current global tag
        let old = self.repository.get_latest_tag();
        let old = tag_or_fallback_to_zero(old)?;
        let mut tag = old.bump(increment, &self.repository)?;
        ensure_tag_is_greater_than_previous(&old, &tag)?;

        if let Some(pre_release) = pre_release {
            tag.version.pre = Prerelease::new(pre_release)?;
        }

        let tag = Tag::create(tag.version, None);

        if dry_run {
            print!("{tag}");
            return Ok(());
        }

        let mut template_context = vec![];
        for bump in &bumps {
            template_context.push(PackageBumpContext {
                package_name: &bump.package_name,
                package_path: &bump.package_path,
                version: OidOf::Tag(bump.version.clone()),
                from: None,
            })
        }

        if !SETTINGS.disable_changelog {
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
                    package_lock: true,
                    packages: template_context,
                }),
            )?;
        }

        let current = self.repository.get_latest_tag().map(HookVersion::new).ok();
        let next_version = HookVersion::new(tag.clone());

        let hook_result = self.run_hooks(
            HookRunOptions::pre_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(hooks_config),
        );

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&tag, hook_result);

        let sign = self.repository.gpg_sign();

        let mut skip_ci_pattern = String::new();

        if skip_ci || skip_ci_override.is_some() {
            skip_ci_pattern = skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
        }

        self.repository.commit(
            &format!(
                "chore(version): {} {}",
                next_version.prefixed_tag, skip_ci_pattern
            ),
            sign,
            true,
        )?;

        if let Some(msg_tmpl) = annotated {
            let mut context = tera::Context::new();
            context.insert("latest", &old.version.to_string());
            context.insert("version", &tag.version.to_string());
            let msg = Tera::one_off(&msg_tmpl, &context, false)?;
            self.repository.create_annotated_tag(&tag, &msg)?;
        } else {
            self.repository.create_tag(&tag)?;
        }

        // Run global post hooks
        self.run_hooks(
            HookRunOptions::post_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(hooks_config),
        )?;

        Ok(())
    }

    fn get_current_packages(&self) -> Result<Vec<PackageData>> {
        let mut packages = vec![];
        for (package_name, package) in SETTINGS.packages.iter() {
            let tag = self.repository.get_latest_package_tag(package_name);
            let tag = tag_or_fallback_to_zero(tag)?;
            packages.push(PackageData {
                package_name: package_name.to_string(),
                package_path: package.path.to_string_lossy().to_string(),
                version: tag,
            })
        }

        Ok(packages)
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

            if next_version == old {
                continue;
            }

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
                HookRunOptions::pre_bump()
                    .current_tag(old_version.as_ref())
                    .next_version(&new_version)
                    .hook_profile(hooks_config)
                    .package(package_name, package),
            );

            self.repository.add_all()?;
            self.unwrap_or_stash_and_exit(&tag, hook_result);
        }

        Ok(())
    }
}
