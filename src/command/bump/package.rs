use crate::command::bump::{
    ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero, HookRunOptions,
    PackageBumpOptions,
};
use crate::conventional::changelog::template::PackageContext;
use crate::conventional::changelog::ReleaseType;
use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::{CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;
use log::info;
use semver::{BuildMetadata, Prerelease};
use tera::Tera;

impl CocoGitto {
    pub fn create_package_version(&mut self, opts: PackageBumpOptions) -> Result<()> {
        self.pre_bump_checks(opts.skip_untracked)?;

        let current_tag = self.repository.get_latest_package_tag(opts.package_name);
        let current_tag = tag_or_fallback_to_zero(current_tag)?;
        let mut next_version = current_tag.bump(opts.increment, &self.repository)?;
        if current_tag == next_version {
            print!("No conventional commits found for {} that required a bump. Changelog will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skipped.\n",
                   opts.package_name
            );
            return Ok(());
        }

        ensure_tag_is_greater_than_previous(&current_tag, &next_version)?;

        if let Some(pre_release) = opts.pre_release {
            next_version.version.pre = Prerelease::new(pre_release)?;
        }

        if let Some(build) = opts.build {
            next_version.version.build = BuildMetadata::new(build)?;
        }

        let tag = Tag::create(
            next_version.version.clone(),
            Some(opts.package_name.to_string()),
        );

        if opts.dry_run {
            print!("{tag}");
            return Ok(());
        }

        if !SETTINGS.disable_changelog {
            let pattern = self.get_bump_revspec(&current_tag);
            let changelog = self.get_package_changelog_with_target_version(
                &pattern,
                tag.clone(),
                opts.package_name,
            )?;

            changelog.pretty_print_bump_summary()?;

            let path = opts.package.changelog_path();
            let template = SETTINGS.get_package_changelog_template()?;
            let additional_context = ReleaseType::Package(PackageContext {
                package_name: opts.package_name,
            });
            changelog.write_to_file(path, template, additional_context)?;
        }

        let current = self
            .repository
            .get_latest_package_tag(opts.package_name)
            .map(HookVersion::new)
            .ok();

        let next_version = HookVersion::new(Tag::create(
            next_version.version,
            Some(opts.package_name.to_string()),
        ));

        let hook_result = self.run_hooks(
            HookRunOptions::pre_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(opts.hooks_config)
                .package(opts.package_name, opts.package),
        );

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&Tag::default(), hook_result);

        let disable_bump_commit = opts.disable_bump_commit || SETTINGS.disable_bump_commit;

        if !disable_bump_commit {
            let sign = self.repository.gpg_sign();

            if opts.skip_ci || opts.skip_ci_override.is_some() {
                let skip_ci_pattern = opts.skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
                self.repository.commit(
                    &format!("chore(version): {tag} {}", skip_ci_pattern),
                    sign,
                    true,
                )?;
            } else {
                self.repository
                    .commit(&format!("chore(version): {tag}"), sign, true)?;
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
                .hook_profile(opts.hooks_config)
                .package(opts.package_name, opts.package),
        )?;

        let current = current
            .map(|current| current.prefixed_tag.to_string())
            .unwrap_or_else(|| "...".to_string());
        let bump = format!("{} -> {}", current, next_version.prefixed_tag).green();
        info!("Bumped package {} version: {}", opts.package_name, bump);

        Ok(())
    }
}
