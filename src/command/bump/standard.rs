use crate::command::bump::{ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero};
use crate::conventional::version::VersionIncrement;
use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::settings::HookType;
use crate::{settings, CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;
use log::info;
use semver::Prerelease;

impl CocoGitto {
    pub fn create_version(
        &mut self,
        increment: VersionIncrement,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
        dry_run: bool,
    ) -> Result<()> {
        self.pre_bump_checks()?;

        let current_tag = self.repository.get_latest_tag();
        let current_tag = tag_or_fallback_to_zero(current_tag)?;
        let mut tag = current_tag.bump(increment, &self.repository)?;

        ensure_tag_is_greater_than_previous(&current_tag, &tag)?;

        if let Some(pre_release) = pre_release {
            tag.version.pre = Prerelease::new(pre_release)?;
        }

        let tag = Tag::create(tag.version, None);

        if dry_run {
            print!("{}", tag);
            return Ok(());
        }

        let pattern = self.get_revspec_for_tag(&current_tag)?;
        let changelog = self.get_changelog_with_target_version(pattern, tag.clone())?;

        let path = settings::changelog_path();
        let template = SETTINGS.get_changelog_template()?;
        changelog.write_to_file(path, template)?;

        let current = self.repository.get_latest_tag().map(HookVersion::new).ok();

        let next_version = HookVersion::new(tag.clone());

        let hook_result = self.run_hooks(
            HookType::PreBump,
            current.as_ref(),
            &next_version,
            hooks_config,
            None,
        );

        self.repository.add_all()?;

        // Hook failed, we need to stop here and reset
        // the repository to a clean state
        if let Err(err) = hook_result {
            self.stash_failed_version(&tag, err)?;
        }

        let sign = self.repository.gpg_sign();

        self.repository.commit(
            &format!("chore(version): {}", next_version.prefixed_tag),
            sign,
        )?;

        self.repository.create_tag(&tag)?;

        self.run_hooks(
            HookType::PostBump,
            current.as_ref(),
            &next_version,
            hooks_config,
            None,
        )?;

        let current = current
            .map(|current| current.prefixed_tag.to_string())
            .unwrap_or_else(|| "...".to_string());
        let bump = format!("{} -> {}", current, next_version.prefixed_tag).green();
        info!("Bumped version: {}", bump);

        Ok(())
    }
}
