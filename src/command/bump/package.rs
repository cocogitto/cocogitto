use crate::command::bump::{ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero};
use crate::conventional::changelog::template::PackageContext;
use crate::conventional::changelog::ReleaseType;
use crate::conventional::version::IncrementCommand;
use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::settings::{HookType, MonoRepoPackage};
use crate::{CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;
use log::info;
use semver::Prerelease;

impl CocoGitto {
    pub fn create_package_version(
        &mut self,
        (package_name, package): (&str, &MonoRepoPackage),
        increment: IncrementCommand,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        dry_run: bool,
    ) -> Result<()> {
        self.pre_bump_checks()?;

        let current_tag = self.repository.get_latest_package_tag(package_name);
        let current_tag = tag_or_fallback_to_zero(current_tag)?;
        let mut next_version = current_tag.bump(increment, &self.repository)?;
        ensure_tag_is_greater_than_previous(&current_tag, &next_version)?;
        if let Some(pre_release) = pre_release {
            next_version.version.pre = Prerelease::new(pre_release)?;
        }

        let tag = Tag::create(next_version.version.clone(), Some(package_name.to_string()));

        if dry_run {
            print!("{}", tag);
            return Ok(());
        }

        let pattern = self.get_revspec_for_tag(&current_tag)?;

        let changelog =
            self.get_package_changelog_with_target_version(pattern, tag.clone(), package_name)?;

        let path = package.changelog_path();
        let template = SETTINGS.get_changelog_template()?;
        let additional_context = ReleaseType::Package(PackageContext { package_name });
        changelog.write_to_file(path, template, additional_context)?;

        let current = self
            .repository
            .get_latest_package_tag(package_name)
            .map(HookVersion::new)
            .ok();

        let next_version = HookVersion::new(Tag::create(
            next_version.version,
            Some(package_name.to_string()),
        ));

        let hook_result = self.run_hooks(
            HookType::PreBump,
            current.as_ref(),
            &next_version,
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

        self.repository
            .commit(&format!("chore(version): {}", tag), false)?;

        self.repository.create_tag(&tag)?;

        self.run_hooks(
            HookType::PostBump,
            current.as_ref(),
            &next_version,
            hooks_config,
            Some(package_name),
            Some(package),
        )?;

        let current = current
            .map(|current| current.prefixed_tag.to_string())
            .unwrap_or_else(|| "...".to_string());
        let bump = format!("{} -> {}", current, next_version.prefixed_tag).green();
        info!("Bumped package {package_name} version: {}", bump);

        Ok(())
    }
}
