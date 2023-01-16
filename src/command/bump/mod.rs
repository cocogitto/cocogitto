use crate::conventional::changelog::release::Release;
use crate::git::error::TagError;
use crate::git::hook::Hooks;
use crate::git::oid::OidOf;
use crate::git::revspec::RevspecPattern;
use crate::git::tag::Tag;
use crate::hook::{Hook, HookVersion};
use crate::settings::{HookType, MonoRepoPackage, Settings};
use crate::PreHookError;
use crate::{CocoGitto, SETTINGS};
use anyhow::Result;
use anyhow::{anyhow, bail, ensure, Context, Error};
use colored::Colorize;
use globset::Glob;
use itertools::Itertools;
use log::{error, warn};
use std::process::exit;

mod monorepo;
mod package;
mod standard;

fn ensure_tag_is_greater_than_previous(current: &Tag, next: &Tag) -> Result<()> {
    if next <= current {
        let comparison = format!("{} <= {}", current, next).red();
        let cause_key = "cause:".red();
        let cause = format!(
            "{} version MUST be greater than current one: {}",
            cause_key, comparison
        );

        bail!("{}:\n\t{}\n", "SemVer Error".red().to_string(), cause);
    };

    Ok(())
}

fn tag_or_fallback_to_zero(tag: Result<Tag, TagError>) -> Result<Tag> {
    match tag {
        Ok(ref tag) => Ok(tag.clone()),
        Err(ref err) if err == &TagError::NoTag => Ok(Tag::default()),
        Err(err) => Err(anyhow!(err)),
    }
}

impl CocoGitto {
    /// Used for cog bump. Get all commits that modify at least one path belonging to the given
    /// package. Target version is not created yet when generating the changelog.
    fn get_changelog_with_target_package_version(
        &self,
        pattern: RevspecPattern,
        starting_tag: Option<OidOf>,
        target_tag: Tag,
        package: &MonoRepoPackage,
    ) -> Result<Option<Release>> {
        let mut release = self
            .repository
            .get_commit_range_filtered(starting_tag, &pattern, |path| {
                path.starts_with(&package.path)
            })?
            .map(Release::from);

        if let Some(release) = &mut release {
            release.version = OidOf::Tag(target_tag);
        }

        Ok(release)
    }

    fn stash_failed_version(&mut self, tag: &Tag, err: Error) -> Result<()> {
        self.repository.stash_failed_version(tag.clone())?;
        error!(
            "{}",
            PreHookError {
                cause: err.to_string(),
                version: tag.to_string(),
                stash_number: 0,
            }
        );

        exit(1);
    }

    fn pre_bump_checks(&mut self) -> Result<()> {
        if *SETTINGS == Settings::default() {
            let part1 = "Warning: using".yellow();
            let part2 = "with the default configuration. \n".yellow();
            let part3 = "You may want to create a".yellow();
            let part4 = "file in your project root to configure bumps.\n".yellow();
            warn!(
                "{} 'cog bump' {}{} 'cog.toml' {}",
                part1, part2, part3, part4
            );
        }
        let statuses = self.repository.get_statuses()?;

        // Fail if repo contains un-staged or un-committed changes
        ensure!(statuses.0.is_empty(), "{}", self.repository.get_statuses()?);

        if !SETTINGS.branch_whitelist.is_empty() {
            if let Some(branch) = self.repository.get_branch_shorthand() {
                let whitelist = &SETTINGS.branch_whitelist;
                let is_match = whitelist.iter().any(|pattern| {
                    let glob = Glob::new(pattern)
                        .expect("invalid glob pattern")
                        .compile_matcher();
                    glob.is_match(&branch)
                });

                ensure!(
                    is_match,
                    "No patterns matched in {:?} for branch '{}', bump is not allowed",
                    whitelist,
                    branch
                )
            }
        };

        Ok(())
    }

    /// Used for cog bump. the target version
    /// is not created yet when generating the changelog.
    pub fn get_changelog_with_target_version(
        &self,
        pattern: RevspecPattern,
        tag: Tag,
    ) -> Result<Release> {
        let commit_range = self.repository.get_commit_range(&pattern)?;

        let mut release = Release::from(commit_range);
        release.version = OidOf::Tag(tag);
        Ok(release)
    }

    fn run_hooks(
        &self,
        hook_type: HookType,
        current_tag: Option<&HookVersion>,
        next_version: &HookVersion,
        hook_profile: Option<&str>,
        package: Option<&MonoRepoPackage>,
    ) -> Result<()> {
        let settings = Settings::get(&self.repository)?;

        let hooks: Vec<Hook> = match (package, hook_profile) {
            (None, Some(profile)) => settings
                .get_profile_hooks(profile, hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| {
                    result.context(format!(
                        "Cannot parse bump profile {} hook at index {}",
                        profile, idx
                    ))
                })
                .try_collect()?,

            (Some(package), Some(profile)) => {
                let hooks = package.get_profile_hooks(profile, hook_type);

                hooks
                    .iter()
                    .map(|s| s.parse())
                    .enumerate()
                    .map(|(idx, result)| {
                        result.context(format!(
                            "Cannot parse bump profile {} hook at index {}",
                            profile, idx
                        ))
                    })
                    .try_collect()?
            }
            (Some(package), None) => package
                .get_hooks(hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| result.context(format!("Cannot parse hook at index {}", idx)))
                .try_collect()?,
            (None, None) => settings
                .get_hooks(hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| result.context(format!("Cannot parse hook at index {}", idx)))
                .try_collect()?,
        };

        for mut hook in hooks {
            hook.insert_versions(current_tag, next_version)?;
            hook.run().context(hook.to_string())?;
        }

        Ok(())
    }

    fn get_revspec_for_tag(&mut self, tag: &Tag) -> Result<RevspecPattern> {
        let origin = if tag.is_zero() {
            self.repository.get_first_commit()?.to_string()
        } else {
            tag.oid_unchecked().to_string()
        };

        let target = self.repository.get_head_commit_oid()?.to_string();
        let pattern = (origin.as_str(), target.as_str());
        Ok(RevspecPattern::from(pattern))
    }
}
