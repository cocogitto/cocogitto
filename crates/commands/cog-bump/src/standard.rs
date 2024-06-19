use cocogitto_changelog::ReleaseType;

use crate::{
    ensure_tag_is_greater_than_previous, get_bump_revspec, get_changelog_with_target_version,
    pre_bump_checks, pretty_print_bump_summary, run_hooks, tag_or_fallback_to_zero,
    unwrap_or_stash_and_exit, BumpOptions, HookRunOptions,
};
use anyhow::Result;
use cocogitto_bump::bump;
use cocogitto_changelog::get_changelog_template;
use cocogitto_config::SETTINGS;
use cocogitto_git::tag::TagLookUpOptions;
use cocogitto_git::Repository;
use cocogitto_hook::HookVersion;
use cocogitto_tag::Tag;
use colored::*;
use log::info;
use semver::{BuildMetadata, Prerelease};
use tera::Tera;

pub fn create_version(mut repository: Repository, opts: BumpOptions) -> Result<()> {
    pre_bump_checks(&mut repository, opts.skip_untracked)?;

    let current_tag = repository.get_latest_tag(TagLookUpOptions::default());
    let current_tag = tag_or_fallback_to_zero(current_tag)?;
    let mut tag = bump(&current_tag, opts.increment, &repository)?;
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

    let tag = Tag::create(
        tag.version,
        None,
        SETTINGS.tag_prefix(),
        SETTINGS.monorepo_separator(),
    );

    if opts.dry_run {
        print!("{tag}");
        return Ok(());
    }

    let pattern = get_bump_revspec(&current_tag);

    if !SETTINGS.disable_changelog {
        let changelog = get_changelog_with_target_version(&repository, &pattern, tag.clone())?;
        pretty_print_bump_summary(&changelog)?;

        let path = cocogitto_config::changelog_path();
        let remote = SETTINGS.changelog.remote.as_ref().cloned();
        let repository = SETTINGS.changelog.repository.as_ref().cloned();
        let owner = SETTINGS.changelog.owner.as_ref().cloned();
        let template = SETTINGS.changelog.template.as_deref().unwrap_or("default");
        let template = get_changelog_template(remote, repository, owner, template)?;

        changelog.write_to_file(path, template, ReleaseType::Standard)?;
    }

    let current = repository
        .get_latest_tag(TagLookUpOptions::default())
        .map(HookVersion::new)
        .ok();

    let next_version = HookVersion::new(tag.clone());

    let hook_result = run_hooks(
        HookRunOptions::pre_bump()
            .current_tag(current.as_ref())
            .next_version(&next_version)
            .hook_profile(opts.hooks_config),
    );

    repository.add_all()?;
    unwrap_or_stash_and_exit(
        &mut repository,
        &Tag::create_default(SETTINGS.tag_prefix(), SETTINGS.monorepo_separator()),
        hook_result,
    );

    let disable_bump_commit = opts.disable_bump_commit || SETTINGS.disable_bump_commit;

    if !disable_bump_commit {
        let sign = repository.gpg_sign();

        if opts.skip_ci || opts.skip_ci_override.is_some() {
            let skip_ci_pattern = opts.skip_ci_override.unwrap_or(SETTINGS.skip_ci.clone());
            repository.commit(
                &format!(
                    "chore(version): {} {}",
                    next_version.prefixed_tag, skip_ci_pattern
                ),
                sign,
                true,
            )?;
        } else {
            repository.commit(
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
        repository.create_annotated_tag(&tag, &msg, disable_bump_commit)?;
    } else {
        repository.create_tag(&tag, disable_bump_commit)?;
    }

    run_hooks(
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
