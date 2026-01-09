use crate::command::bump::{BumpOptions, HookRunOptions};
use crate::conventional::changelog::context::{
    MonoRepoContext, PackageBumpContext, PackageContext,
};
use crate::conventional::changelog::ReleaseType;
use crate::conventional::version::{Increment, IncrementCommand};
use crate::git::error::TagError;
use crate::git::tag::{Tag, TagLookUpOptions};
use crate::hook::HookVersion;
use crate::settings::MonoRepoPackage;
use crate::{settings, CocoGitto, SETTINGS};
use anyhow::{bail, Result};

use log::{info, warn};
use tera::Tera;

use crate::git::oid::OidOf;

#[derive(Debug)]
struct PackageBumpData {
    package_name: String,
    package_path: String,
    public_api: bool,
    current: Tag,
    old_version: Option<HookVersion>,
    new_version: HookVersion,
    increment: Increment,
}

#[derive(Debug)]
pub struct PackageData {
    pub package_name: String,
    pub package_path: String,
    pub version: Tag,
}

impl CocoGitto {
    pub fn create_monorepo_version(&mut self, opts: BumpOptions) -> Result<()> {
        if opts.increment == IncrementCommand::Auto || opts.include_packages {
            if SETTINGS.generate_mono_repository_global_tag {
                self.create_monorepo_version_auto(opts)
            } else {
                if opts.annotated.is_some() {
                    warn!(
                        "--annotated flag is not supported for package bumps without a global tag"
                    );
                }
                self.create_all_package_version_auto(opts)
            }
        } else {
            self.create_monorepo_version_manual(opts)
        }
    }

    pub fn create_all_package_version_auto(&mut self, opts: BumpOptions) -> Result<()> {
        self.pre_bump_checks(opts.skip_untracked)?;
        // Get package bumps
        let bumps = self.get_packages_bumps(&opts)?;

        if bumps.is_empty() {
            print!("No conventional commits found for your packages that required a bump. Changelogs will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skipped.\n");
            return Ok(());
        }

        if opts.dry_run {
            for bump in bumps {
                println!("{}", bump.new_version.prefixed_tag)
            }
            return Ok(());
        }

        let hook_result =
            self.run_hooks(HookRunOptions::pre_bump().hook_profile(opts.hooks_config));

        let disable_bump_commit = opts.disable_bump_commit || SETTINGS.disable_bump_commit;

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&Tag::default(), hook_result);
        self.bump_packages(opts.hooks_config, &bumps)?;

        if !disable_bump_commit {
            let sign = self.repository.gpg_sign();
            if opts.skip_ci || opts.skip_ci_override.is_some() {
                let skip_ci_pattern = opts.skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
                self.repository.commit(
                    &format!("chore(version): bump packages {skip_ci_pattern}"),
                    sign,
                    true,
                )?;
            } else {
                self.repository
                    .commit("chore(version): bump packages", sign, true)?;
            }
        }

        if SETTINGS.generate_mono_repository_package_tags {
            for bump in &bumps {
                self.repository
                    .create_tag(&bump.new_version.prefixed_tag, disable_bump_commit)?;
            }
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
                    .hook_profile(opts.hooks_config)
                    .package(&bump.package_name, package),
            )?;
        }

        // Run global post hooks
        self.run_hooks(HookRunOptions::post_bump().hook_profile(opts.hooks_config))?;

        Ok(())
    }

    fn create_monorepo_version_auto(&mut self, opts: BumpOptions) -> Result<()> {
        self.pre_bump_checks(opts.skip_untracked)?;
        // Get package bumps
        let bumps = self.get_packages_bumps(&opts)?;
        if bumps.is_empty() {
            print!("No conventional commits found for your packages that required a bump. Changelogs will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skipped.\n");
            return Ok(());
        }

        // Manual bump with `--include-packages` -> don't override increment command
        let increment = if opts.increment != IncrementCommand::Auto {
            opts.increment.clone()
        } else if SETTINGS.generate_mono_repository_package_tags {
            // Get the greatest package increment among public api packages
            IncrementCommand::AutoMonoRepoGlobal(
                bumps
                    .iter()
                    .filter(|bump| bump.public_api)
                    .map(|bump| bump.increment)
                    .max(),
            )
        } else {
            IncrementCommand::Auto
        };

        let bump_res = opts.get_new_version(&self.repository, None, false, Some(increment))?;

        let tag = Tag::create(bump_res.next.version, None);

        if opts.dry_run {
            for bump in bumps {
                println!("{}", bump.new_version.prefixed_tag)
            }
            print!("{tag}");
            return Ok(());
        }

        let mut template_context = vec![];
        for bump in &bumps {
            template_context.push(PackageBumpContext {
                package_name: bump.package_name.clone(),
                package_path: bump.package_path.clone(),
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
            let pattern = self.get_bump_revspec(&bump_res.current);
            let changelog = self.get_monorepo_global_changelog_for_version(
                &pattern,
                OidOf::Tag(bump_res.current.clone()),
                tag.clone(),
            )?;

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

        let current = self
            .repository
            .get_latest_tag(TagLookUpOptions::default())
            .map(HookVersion::new)
            .ok();
        let next_version = HookVersion::new(tag.clone());

        let hook_result = self.run_hooks(
            HookRunOptions::pre_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(opts.hooks_config),
        );

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&tag, hook_result);
        self.bump_packages(opts.hooks_config, &bumps)?;

        let disable_bump_commit = opts.disable_bump_commit || SETTINGS.disable_bump_commit;

        if !disable_bump_commit {
            let sign = self.repository.gpg_sign();
            if opts.skip_ci || opts.skip_ci_override.is_some() {
                let skip_ci_pattern = opts.skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
                self.repository.commit(
                    &format!(
                        "chore(version): {} {}",
                        next_version.prefixed_tag, skip_ci_pattern
                    ),
                    sign,
                    true,
                )?;
            } else {
                self.repository.commit(
                    &format!("chore(version): {}", next_version.prefixed_tag),
                    sign,
                    true,
                )?;
            }
        }

        if SETTINGS.generate_mono_repository_package_tags {
            for bump in &bumps {
                self.repository
                    .create_tag(&bump.new_version.prefixed_tag, disable_bump_commit)?;
            }
        }

        if let Some(msg_tmpl) = opts.annotated {
            let mut context = tera::Context::new();
            context.insert("latest", &bump_res.current.version.to_string());
            context.insert("version", &tag.version.to_string());
            let msg = Tera::one_off(&msg_tmpl, &context, false)?;
            self.repository
                .create_annotated_tag(&tag, &msg, disable_bump_commit)?;
        } else {
            self.repository.create_tag(&tag, disable_bump_commit)?;
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
                    .hook_profile(opts.hooks_config)
                    .package(&bump.package_name, package),
            )?;
        }

        // Run global post hooks
        self.run_hooks(
            HookRunOptions::post_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(opts.hooks_config),
        )?;

        Ok(())
    }

    fn create_monorepo_version_manual(&mut self, opts: BumpOptions) -> Result<()> {
        self.pre_bump_checks(opts.skip_untracked)?;
        // Get package bumps
        let bumps = self.get_current_packages()?;

        let bump_res = opts.get_new_version(&self.repository, None, false, None)?;

        let tag = Tag::create(bump_res.next.version, None);

        if opts.dry_run {
            print!("{tag}");
            return Ok(());
        }

        let mut template_context = vec![];
        for bump in &bumps {
            template_context.push(PackageBumpContext {
                package_name: bump.package_name.clone(),
                package_path: bump.package_path.clone(),
                version: OidOf::Tag(bump.version.clone()),
                from: None,
            })
        }

        if !SETTINGS.disable_changelog {
            let pattern = self.get_bump_revspec(&bump_res.current);
            let changelog = self.get_monorepo_global_changelog_for_version(
                &pattern,
                OidOf::Tag(bump_res.current.clone()),
                tag.clone(),
            )?;

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

        let current = self
            .repository
            .get_latest_tag(TagLookUpOptions::default())
            .map(HookVersion::new)
            .ok();
        let next_version = HookVersion::new(tag.clone());

        let hook_result = self.run_hooks(
            HookRunOptions::pre_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(opts.hooks_config),
        );

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&Tag::default(), hook_result);

        let disable_bump_commit = opts.disable_bump_commit || SETTINGS.disable_bump_commit;

        if !disable_bump_commit {
            let sign = self.repository.gpg_sign();

            if opts.skip_ci || opts.skip_ci_override.is_some() {
                let skip_ci_pattern = opts.skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
                self.repository.commit(
                    &format!(
                        "chore(version): {} {}",
                        next_version.prefixed_tag, skip_ci_pattern
                    ),
                    sign,
                    true,
                )?;
            } else {
                self.repository.commit(
                    &format!("chore(version): {}", next_version.prefixed_tag),
                    sign,
                    true,
                )?;
            }
        }

        if let Some(msg_tmpl) = opts.annotated {
            let mut context = tera::Context::new();
            context.insert("latest", &bump_res.current.version.to_string());
            context.insert("version", &tag.version.to_string());
            let msg = Tera::one_off(&msg_tmpl, &context, false)?;
            self.repository
                .create_annotated_tag(&tag, &msg, disable_bump_commit)?;
        } else {
            self.repository.create_tag(&tag, disable_bump_commit)?;
        }

        // Run global post hooks
        self.run_hooks(
            HookRunOptions::post_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(opts.hooks_config),
        )?;

        Ok(())
    }

    pub fn get_current_packages(&self) -> Result<Vec<PackageData>> {
        let mut packages = vec![];
        for (package_name, package) in SETTINGS.packages.iter() {
            let tag = match self.repository.get_latest_package_tag(package_name) {
                Ok(tag) => tag,
                Err(TagError::NoTag) => Tag::default(),
                Err(other) => bail!(other),
            };
            packages.push(PackageData {
                package_name: package_name.to_string(),
                package_path: package.path.to_string_lossy().to_string(),
                version: tag,
            })
        }

        Ok(packages)
    }

    // Calculate all package bump
    fn get_packages_bumps(&self, opts: &BumpOptions) -> Result<Vec<PackageBumpData>> {
        let mut package_bumps = vec![];
        let mut packages: Vec<(&String, &MonoRepoPackage)> = SETTINGS.packages.iter().collect();
        packages.sort_by(|a, b| a.1.bump_order.cmp(&b.1.bump_order));

        for (package_name, package) in packages {
            let increment = if opts.increment != IncrementCommand::Auto {
                opts.increment.clone()
            } else {
                IncrementCommand::AutoPackage(package_name.to_string())
            };

            let bump_res =
                opts.get_new_version(&self.repository, Some(package_name), true, Some(increment))?;
            if bump_res.no_change() || !bump_res.had_commits {
                continue;
            }

            let tag = Tag::create(bump_res.next.version, Some(package_name.to_string()));
            let increment = tag.get_increment_from(&bump_res.current);

            if let Some(increment) = increment {
                let old_version = if bump_res.current.is_zero() {
                    None
                } else {
                    Some(HookVersion::new(bump_res.current.clone()))
                };

                package_bumps.push(PackageBumpData {
                    package_name: package_name.to_string(),
                    package_path: package.path.to_string_lossy().to_string(),
                    public_api: package.public_api,
                    current: bump_res.current,
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
        hooks_config: Option<&str>,
        package_bumps: &Vec<PackageBumpData>,
    ) -> Result<()> {
        for bump in package_bumps {
            let package_name = &bump.package_name;
            let tag = &bump.new_version.prefixed_tag;

            let package = SETTINGS
                .packages
                .get(package_name.as_str())
                .expect("package exists");

            if !SETTINGS.disable_changelog {
                let pattern = self.get_bump_revspec(&bump.current);
                let changelog = self.get_package_changelog_with_target_version(
                    &pattern,
                    tag.clone(),
                    package_name.as_str(),
                )?;

                changelog.pretty_print_bump_summary()?;

                let path = package.changelog_path();
                let template = SETTINGS.get_package_changelog_template()?;

                let additional_context = ReleaseType::Package(PackageContext {
                    package_name: package_name.clone(),
                });

                changelog.write_to_file(&path, template, additional_context)?;
                info!("\tChangelog updated {:?}", path);
            }

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
            self.unwrap_or_stash_and_exit(tag, hook_result);
        }

        Ok(())
    }
}
