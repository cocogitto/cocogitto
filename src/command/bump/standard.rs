use crate::command::bump::{
    ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero, BumpOptions, HookRunOptions,
};

use crate::conventional::changelog::ReleaseType;

use crate::git::tag::{Tag, TagLookUpOptions};
use crate::hook::HookVersion;
use crate::{settings, CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;
use log::info;
use semver::{BuildMetadata, Prerelease};
use tera::Tera;

impl CocoGitto {
    pub fn create_version(&mut self, opts: BumpOptions) -> Result<()> {
        self.pre_bump_checks(opts.skip_untracked)?;

        let current_tag = self.repository.get_latest_tag(TagLookUpOptions::default());
        let current_tag = tag_or_fallback_to_zero(current_tag)?;
        let mut tag = current_tag.bump(opts.increment, &self.repository)?;
        if current_tag == tag {
            print!("No conventional commits for your repository that required a bump. Changelogs will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skipped.\n");
            return Ok(());
        }

        ensure_tag_is_greater_than_previous(&current_tag, &tag)?;

        if let Some(pre_release) = opts.pre_release {
            tag.version.pre = Prerelease::new(pre_release)?;
        }

        if let Some(build) = opts.build {
            tag.version.build = BuildMetadata::new(build)?;
        }

        let tag = Tag::create(tag.version, None);

        if opts.dry_run {
            print!("{tag}");
            return Ok(());
        }

        let pattern = self.get_bump_revspec(&current_tag);

        if !SETTINGS.disable_changelog {
            let changelog = self.get_changelog_with_target_version(&pattern, tag.clone())?;
            changelog.pretty_print_bump_summary()?;

            let path = settings::changelog_path();
            let template = SETTINGS.get_changelog_template()?;

            changelog.write_to_file(path, template, ReleaseType::Standard)?;
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
            context.insert("latest", &current_tag.version.to_string());
            context.insert("version", &tag.version.to_string());
            let msg = Tera::one_off(&msg_tmpl, &context, false)?;
            self.repository
                .create_annotated_tag(&tag, &msg, disable_bump_commit)?;
        } else {
            self.repository.create_tag(&tag, disable_bump_commit)?;
        }

        self.run_hooks(
            HookRunOptions::post_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(opts.hooks_config),
        )?;

        let current = current
            .map(|current| current.prefixed_tag.to_string())
            .unwrap_or_else(|| "...".to_string());
        let bump = format!("{} -> {}", current, next_version.prefixed_tag).green();
        info!("Bumped version: {}", bump);

        Ok(())
    }
}
