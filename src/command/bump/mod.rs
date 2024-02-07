use crate::conventional::changelog::release::Release;
use crate::conventional::commit::Commit;
use crate::git::error::TagError;
use crate::git::oid::OidOf;
use crate::git::revspec::RevspecPattern;
use crate::git::tag::Tag;
use crate::hook::{Hook, HookVersion, Hooks};
use crate::settings::{HookType, MonoRepoPackage, Settings};
use crate::BumpError;
use crate::{CocoGitto, COMMITS_METADATA, SETTINGS};
use anyhow::Result;
use anyhow::{anyhow, bail, ensure, Context};
use colored::Colorize;
use conventional_commit_parser::commit::CommitType;
use globset::Glob;
use itertools::Itertools;
use log::{error, info, warn};
use std::fmt;
use std::fmt::Write;
use std::process::exit;

mod monorepo;
mod package;
mod standard;

struct HookRunOptions<'a> {
    hook_type: HookType,
    current_tag: Option<&'a HookVersion>,
    next_version: Option<&'a HookVersion>,
    hook_profile: Option<&'a str>,
    package_name: Option<&'a str>,
    package: Option<&'a MonoRepoPackage>,
}

impl<'a> HookRunOptions<'a> {
    pub fn post_bump() -> Self {
        Self {
            hook_type: HookType::PostBump,
            current_tag: None,
            next_version: None,
            hook_profile: None,
            package_name: None,
            package: None,
        }
    }

    pub fn pre_bump() -> Self {
        Self {
            hook_type: HookType::PreBump,
            current_tag: None,
            next_version: None,
            hook_profile: None,
            package_name: None,
            package: None,
        }
    }

    pub fn current_tag<'b>(mut self, version: Option<&'b HookVersion>) -> Self
    where
        'b: 'a,
    {
        self.current_tag = version;
        self
    }

    pub fn next_version<'b>(mut self, version: &'b HookVersion) -> Self
    where
        'b: 'a,
    {
        self.next_version = Some(version);
        self
    }

    pub fn hook_profile<'b>(mut self, profile: Option<&'b str>) -> Self
    where
        'b: 'a,
    {
        self.hook_profile = profile;
        self
    }

    pub fn package<'b>(mut self, name: &'b str, package: &'b MonoRepoPackage) -> Self
    where
        'b: 'a,
    {
        self.package_name = Some(name);
        self.package = Some(package);
        self
    }
}

fn ensure_tag_is_greater_than_previous(current: &Tag, next: &Tag) -> Result<()> {
    if next < current {
        let comparison = format!("{current} <= {next}").red();
        let cause_key = "cause:".red();
        let cause = format!("{cause_key} version MUST be greater than current one: {comparison}");
        bail!("{}:\n\t{}\n", "SemVer Error".red().to_string(), cause);
    };

    Ok(())
}

fn tag_or_fallback_to_zero(tag: Result<Tag, TagError>) -> Result<Tag> {
    match tag {
        Ok(ref tag) => Ok(tag.clone()),
        Err(TagError::NoTag) => Ok(Tag::default()),
        Err(err) => Err(anyhow!(err)),
    }
}

impl CocoGitto {
    pub fn unwrap_or_stash_and_exit<T>(&mut self, tag: &Tag, result: Result<T>) -> T {
        match result {
            Ok(res) => res,
            Err(err) => {
                self.repository
                    .stash_failed_version(tag.clone())
                    .expect("stash");
                error!(
                    "{}",
                    BumpError {
                        cause: err.to_string(),
                        version: tag.to_string(),
                        stash_number: 0,
                    }
                );

                exit(1);
            }
        }
    }

    fn pre_bump_checks(&mut self, skip_untracked: bool) -> Result<()> {
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

        if skip_untracked || SETTINGS.skip_untracked {
            eprintln!("{}", self.repository.get_statuses()?);
        } else {
            ensure!(statuses.0.is_empty(), "{}", self.repository.get_statuses()?);
        }

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

    /// The target version is not created yet when generating the changelog.
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

    /// The target package version is not created yet when generating the changelog.
    pub fn get_package_changelog_with_target_version(
        &self,
        pattern: RevspecPattern,
        tag: Tag,
        package: &str,
    ) -> Result<Release> {
        let commit_range = self
            .repository
            .get_commit_range_for_package(&pattern, package)?;

        let mut release = Release::from(commit_range);
        release.version = OidOf::Tag(tag);
        Ok(release)
    }

    /// The target global monorepo version is not created yet when generating the changelog.
    pub fn get_monorepo_global_changelog_with_target_version(
        &self,
        pattern: RevspecPattern,
        tag: Tag,
    ) -> Result<Release> {
        let commit_range = self
            .repository
            .get_commit_range_for_monorepo_global(&pattern)?;

        let mut release = Release::from(commit_range);
        release.version = OidOf::Tag(tag);
        Ok(release)
    }

    fn run_hooks(&self, options: HookRunOptions) -> Result<()> {
        let settings = Settings::get(&self.repository)?;

        let hooks: Vec<Hook> = match (options.package, options.hook_profile) {
            (None, Some(profile)) => settings
                .get_profile_hooks(profile, options.hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| {
                    result.context(format!(
                        "Cannot parse bump profile {profile} hook at index {idx}"
                    ))
                })
                .try_collect()?,

            (Some(package), Some(profile)) => {
                let hooks = package.get_profile_hooks(profile, options.hook_type);

                hooks
                    .iter()
                    .map(|s| s.parse())
                    .enumerate()
                    .map(|(idx, result)| {
                        result.context(format!(
                            "Cannot parse bump profile {profile} hook at index {idx}"
                        ))
                    })
                    .try_collect()?
            }
            (Some(package), None) => package
                .get_hooks(options.hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| result.context(format!("Cannot parse hook at index {idx}")))
                .try_collect()?,
            (None, None) => settings
                .get_hooks(options.hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| result.context(format!("Cannot parse hook at index {idx}")))
                .try_collect()?,
        };

        if !hooks.is_empty() {
            let hook_type = match options.hook_type {
                HookType::PreBump => "pre-bump",
                HookType::PostBump => "post-bump",
            };

            match options.package_name {
                None => {
                    let msg = format!("[{hook_type}]").underline().white().bold();
                    info!("{msg}")
                }
                Some(package_name) => {
                    let msg = format!("[{hook_type}-{package_name}]")
                        .underline()
                        .white()
                        .bold();
                    info!("{msg}")
                }
            }
        }

        for mut hook in hooks {
            hook.insert_versions(options.current_tag, options.next_version)?;
            let command = hook.to_string();
            let command = if command.chars().count() > 78 {
                &command[0..command.len()]
            } else {
                &command
            };
            info!("[{command}]");
            let package_path = options.package.map(|p| p.path.as_path());
            hook.run(package_path).context(hook.to_string())?;
            println!();
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

impl Release<'_> {
    fn pretty_print_bump_summary(&self) -> Result<(), fmt::Error> {
        let conventional_commits: Vec<&Commit> = self
            .commits
            .iter()
            .map(|ch_commit| &ch_commit.commit)
            .collect();

        // Commits which type are neither feat, fix nor breaking changes
        // won't affect the version number.
        let mut non_bump_commits: Vec<&CommitType> = conventional_commits
            .iter()
            .filter_map(|commit: &&Commit| {
                let commit_config = COMMITS_METADATA.get(&commit.conventional.commit_type);
                match commit_config {
                    Some(commit_config) if commit_config.bump_minor || commit_config.bump_patch => {
                        None
                    }
                    _ if commit.conventional.is_breaking_change => None,
                    _ => Some(&commit.conventional.commit_type),
                }
            })
            .collect();

        non_bump_commits.sort();

        let non_bump_commits: Vec<(usize, &CommitType)> = non_bump_commits
            .into_iter()
            .dedup_by_with_count(|c1, c2| c1 == c2)
            .collect();

        if !non_bump_commits.is_empty() {
            let mut skip_message = "  Skipping irrelevant commits:\n".to_string();
            for (count, commit_type) in non_bump_commits {
                writeln!(skip_message, "    - {}: {}", commit_type.as_ref(), count)?;
            }

            info!("{}", skip_message);
        }

        let bump_commits =
            conventional_commits
                .iter()
                .filter(|commit| match &commit.conventional.commit_type {
                    CommitType::Feature | CommitType::BugFix => true,
                    _commit_type if commit.conventional.is_breaking_change => true,
                    _ => false,
                });

        for commit in bump_commits {
            match &commit.conventional.commit_type {
                _commit_type if commit.conventional.is_breaking_change => {
                    info!(
                        "\t Found {} commit {} with type: {}",
                        "BREAKING CHANGE".red(),
                        commit.shorthand().blue(),
                        commit.conventional.commit_type.as_ref().yellow()
                    )
                }
                CommitType::Feature => {
                    info!("\tFound feature commit {}", commit.shorthand().blue())
                }
                CommitType::BugFix => info!("\tFound bug fix commit {}", commit.shorthand().blue()),
                _ => (),
            }
        }

        Ok(())
    }
}
