use crate::command::bump::{
    ensure_tag_is_greater_than_previous, tag_or_fallback_to_zero, HookRunOptions,
};
use crate::conventional::changelog::template::PackageContext;
use crate::conventional::changelog::ReleaseType;
use crate::conventional::version::IncrementCommand;
use crate::git::tag::Tag;
use crate::hook::HookVersion;
use crate::settings::MonoRepoPackage;
use crate::{CocoGitto, SETTINGS};
use anyhow::Result;
use colored::*;
use log::info;
use semver::Prerelease;
use tera::Tera;

impl CocoGitto {
    #[allow(clippy::too_many_arguments)]
    pub fn create_package_version(
        &mut self,
        (package_name, package): (&str, &MonoRepoPackage),
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

        let current_tag = self.repository.get_latest_package_tag(package_name);
        let current_tag = tag_or_fallback_to_zero(current_tag)?;
        let mut next_version = current_tag.bump(increment, &self.repository)?;
        if current_tag == next_version {
            print!("No conventional commits found for {package_name} that required a bump. Changelog will be updated on the next bump.\nPre-Hooks and Post-Hooks have been skiped.\n");
            return Ok(());
        }

        ensure_tag_is_greater_than_previous(&current_tag, &next_version)?;

        if let Some(pre_release) = pre_release {
            next_version.version.pre = Prerelease::new(pre_release)?;
        }

        let tag = Tag::create(next_version.version.clone(), Some(package_name.to_string()));

        if dry_run {
            print!("{tag}");
            return Ok(());
        }

        if !SETTINGS.disable_changelog {
            let pattern = self.get_revspec_for_tag(&current_tag)?;
            let changelog =
                self.get_package_changelog_with_target_version(pattern, tag.clone(), package_name)?;

            changelog.pretty_print_bump_summary()?;

            let path = package.changelog_path();
            let template = SETTINGS.get_package_changelog_template()?;
            let additional_context = ReleaseType::Package(PackageContext { package_name });
            changelog.write_to_file(path, template, additional_context)?;
        }

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
            HookRunOptions::pre_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(hooks_config)
                .package(package_name, package),
        );

        self.repository.add_all()?;
        self.unwrap_or_stash_and_exit(&tag, hook_result);

        let sign = self.repository.gpg_sign();

        let mut skip_ci_pattern = String::new();

        if skip_ci || skip_ci_override.is_some() {
            skip_ci_pattern = skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
        }

        self.repository.commit(
            &format!("chore(version): {tag} {}", skip_ci_pattern),
            sign,
            true,
        )?;

        if let Some(msg_tmpl) = annotated {
            let mut context = tera::Context::new();
            context.insert("latest", &current_tag.version.to_string());
            context.insert("version", &tag.version.to_string());
            let msg = Tera::one_off(&msg_tmpl, &context, false)?;
            self.repository.create_annotated_tag(&tag, &msg)?;
        } else {
            self.repository.create_tag(&tag)?;
        }

        self.run_hooks(
            HookRunOptions::post_bump()
                .current_tag(current.as_ref())
                .next_version(&next_version)
                .hook_profile(hooks_config)
                .package(package_name, package),
        )?;

        let current = current
            .map(|current| current.prefixed_tag.to_string())
            .unwrap_or_else(|| "...".to_string());
        let bump = format!("{} -> {}", current, next_version.prefixed_tag).green();
        info!("Bumped package {package_name} version: {}", bump);

        Ok(())
    }
}
