use cocogitto_tag::error::TagError;

use crate::monorepo::create_monorepo_version;
use crate::package::create_package_version;
use crate::standard::create_version;
use anyhow::{anyhow, bail, Context};
use anyhow::{ensure, Result};
use cocogitto_bump::increment::IncrementCommand;
use cocogitto_changelog::release::Release;
use cocogitto_changelog::release_from_commits;
use cocogitto_commit::{Commit, CommitType};
use cocogitto_config::hook::{HookType, Hooks};
use cocogitto_config::monorepo::MonoRepoPackage;
use cocogitto_config::{Settings, COMMITS_METADATA, SETTINGS};
use cocogitto_git::Repository;
use cocogitto_hook::{Hook, HookVersion};
use cocogitto_oid::OidOf;
use cocogitto_tag::Tag;
use cog_command::CogCommand;
use colored::Colorize;
use error::BumpError;
use globset::Glob;
use itertools::Itertools;
use log::{error, info, warn};
use std::default::Default;
use std::fmt;
use std::fmt::Write;
use std::process::exit;

mod error;
mod monorepo;
mod package;
mod standard;

pub struct CogBumpCommand {
    pub version: Option<String>,
    pub auto: bool,
    pub major: bool,
    pub minor: bool,
    pub patch: bool,
    pub pre: Option<String>,
    pub build: Option<String>,
    pub hook_profile: Option<String>,
    pub package: Option<String>,
    pub annotated: Option<String>,
    pub dry_run: bool,
    pub skip_ci: bool,
    pub skip_ci_override: Option<String>,
    pub skip_untracked: bool,
    pub disable_bump_commit: bool,
}

impl CogCommand for CogBumpCommand {
    fn execute(self) -> Result<()> {
        let repository = Self::repository()?;
        let is_monorepo = !SETTINGS.packages.is_empty();

        let increment = match self.version {
            Some(version) => IncrementCommand::Manual(version),
            None if self.auto => match self.package.as_ref() {
                Some(package) => {
                    if !is_monorepo {
                        bail!("Cannot create package version on non mono-repository config")
                    };

                    IncrementCommand::AutoPackage(package.to_string())
                }
                None => IncrementCommand::Auto,
            },
            None if self.major => IncrementCommand::Major,
            None if self.minor => IncrementCommand::Minor,
            None if self.patch => IncrementCommand::Patch,
            _ => unreachable!(),
        };

        if is_monorepo {
            match self.package {
                Some(package_name) => {
                    // Safe unwrap here, package name is validated by clap
                    let package = SETTINGS.packages.get(&package_name).unwrap();

                    let opts = PackageBumpOptions {
                        package_name: &package_name,
                        package,
                        increment,
                        pre_release: self.pre.as_deref(),
                        build: self.build.as_deref(),
                        hooks_config: self.hook_profile.as_deref(),
                        annotated: self.annotated,
                        dry_run: self.dry_run,
                        skip_ci: self.skip_ci,
                        skip_ci_override: self.skip_ci_override,
                        skip_untracked: self.skip_untracked,
                        disable_bump_commit: self.disable_bump_commit,
                    };

                    create_package_version(repository, opts)
                }
                None => {
                    let opts = BumpOptions {
                        increment,
                        pre_release: self.pre.as_deref(),
                        build: self.build.as_deref(),
                        hooks_config: self.hook_profile.as_deref(),
                        annotated: self.annotated,
                        dry_run: self.dry_run,
                        skip_ci: self.skip_ci,
                        skip_ci_override: self.skip_ci_override,
                        skip_untracked: self.skip_untracked,
                        disable_bump_commit: self.disable_bump_commit,
                    };

                    create_monorepo_version(repository, opts)
                }
            }
        } else {
            let opts = BumpOptions {
                increment,
                pre_release: self.pre.as_deref(),
                build: self.build.as_deref(),
                hooks_config: self.hook_profile.as_deref(),
                annotated: self.annotated,
                dry_run: self.dry_run,
                skip_ci: self.skip_ci,
                skip_ci_override: self.skip_ci_override,
                skip_untracked: self.skip_untracked,
                disable_bump_commit: self.disable_bump_commit,
            };

            create_version(repository, opts)
        }
    }
}

#[derive(Default)]
pub struct BumpOptions<'a> {
    pub increment: IncrementCommand,
    pub pre_release: Option<&'a str>,
    pub build: Option<&'a str>,
    pub hooks_config: Option<&'a str>,
    pub annotated: Option<String>,
    pub dry_run: bool,
    pub skip_ci: bool,
    pub skip_ci_override: Option<String>,
    pub skip_untracked: bool,
    pub disable_bump_commit: bool,
}

#[derive(Default)]
pub struct PackageBumpOptions<'a> {
    pub package_name: &'a str,
    pub package: &'a MonoRepoPackage,
    pub increment: IncrementCommand,
    pub pre_release: Option<&'a str>,
    pub build: Option<&'a str>,
    pub hooks_config: Option<&'a str>,
    pub annotated: Option<String>,
    pub dry_run: bool,
    pub skip_ci: bool,
    pub skip_ci_override: Option<String>,
    pub skip_untracked: bool,
    pub disable_bump_commit: bool,
}

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
        Err(TagError::NoTag) => Ok(Tag::create_default(
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
        )),
        Err(err) => Err(anyhow!(err)),
    }
}

fn get_bump_revspec(current_tag: &Tag) -> String {
    if current_tag.is_zero() {
        "..".to_string()
    } else {
        format!("{current_tag}..")
    }
}

pub fn unwrap_or_stash_and_exit<T>(repository: &mut Repository, tag: &Tag, result: Result<T>) -> T {
    match result {
        Ok(res) => res,
        Err(err) => {
            repository.stash_failed_version(tag.clone()).expect("stash");
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

fn pre_bump_checks(repository: &mut Repository, skip_untracked: bool) -> Result<()> {
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
    let statuses = repository.get_statuses()?;

    if skip_untracked || SETTINGS.skip_untracked {
        eprintln!("{}", repository.get_statuses()?);
    } else {
        ensure!(statuses.0.is_empty(), "{}", repository.get_statuses()?);
    }

    if !SETTINGS.branch_whitelist.is_empty() {
        if let Some(branch) = repository.get_branch_shorthand() {
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
pub fn get_changelog_with_target_version<'a>(
    repository: &'a Repository,
    pattern: &'a str,
    tag: Tag,
) -> Result<Release<'a>> {
    let commit_range = repository.revwalk(pattern)?;
    let allowed_commits = &SETTINGS.allowed_commit_types();
    let omitted_commits = &SETTINGS.commit_omitted_from_changelog();
    let changelog_titles = &SETTINGS.changelog_titles();
    let usernames = &SETTINGS.commit_usernames();
    let mut release = release_from_commits(
        commit_range,
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    )?;
    release.version = OidOf::Tag(tag);
    Ok(release)
}

/// The target package version is not created yet when generating the changelog.
pub fn get_package_changelog_with_target_version<'a>(
    repository: &'a Repository,
    pattern: &'a str,
    tag: Tag,
    package: &'a str,
) -> Result<Release<'a>> {
    let commit_range = repository.get_commit_range_for_package(pattern, package)?;
    let allowed_commits = &SETTINGS.allowed_commit_types();
    let omitted_commits = &SETTINGS.commit_omitted_from_changelog();
    let changelog_titles = &SETTINGS.changelog_titles();
    let usernames = &SETTINGS.commit_usernames();

    let mut release = release_from_commits(
        commit_range,
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    )?;
    release.version = OidOf::Tag(tag);
    Ok(release)
}

/// The target global monorepo version is not created yet when generating the changelog.
pub fn get_monorepo_global_changelog_for_version<'a>(
    repository: &'a Repository,
    pattern: &'a str,
    from: OidOf,
    tag: Tag,
) -> Result<Release<'a>> {
    let commit_range = repository.get_commit_range_for_monorepo_global(pattern)?;
    let allowed_commits = &SETTINGS.allowed_commit_types();
    let omitted_commits = &SETTINGS.commit_omitted_from_changelog();
    let changelog_titles = &SETTINGS.changelog_titles();
    let usernames = &SETTINGS.commit_usernames();

    let release = match release_from_commits(
        commit_range,
        allowed_commits,
        omitted_commits,
        changelog_titles,
        usernames,
    ) {
        Ok(mut release) => {
            release.version = OidOf::Tag(tag);
            release
        }
        Err(_) => Release {
            version: OidOf::Tag(tag),
            from,
            date: Default::default(),
            commits: vec![],
            previous: None,
        },
    };

    Ok(release)
}

fn run_hooks(options: HookRunOptions) -> Result<()> {
    let hooks: Vec<Hook> = match (options.package, options.hook_profile) {
        (None, Some(profile)) => SETTINGS
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
        (None, None) => SETTINGS
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
        hook.insert_versions(
            options.current_tag,
            options.next_version,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
        )?;
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

fn pretty_print_bump_summary(release: &Release) -> Result<(), fmt::Error> {
    let conventional_commits: Vec<&Commit> = release
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
                Some(commit_config) if commit_config.bump_minor || commit_config.bump_patch => None,
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::Path;
    use std::path::PathBuf;

    use anyhow::Result;
    use cmd_lib::run_cmd;
    use cocogitto_config::monorepo::MonoRepoPackage;
    use cocogitto_config::Settings;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::monorepo::{create_all_package_version_auto, create_monorepo_version};
    use crate::package::create_package_version;
    use crate::standard::create_version;
    use crate::{BumpOptions, PackageBumpOptions};
    use cocogitto_bump::increment::IncrementCommand;
    use cocogitto_git::rev::cache::clear_cache;
    use cocogitto_git::Repository;
    use cocogitto_test_helpers::*;

    #[sealed_test]
    fn bump_ok() -> Result<()> {
        // Arrange
        let repository = git_init_no_gpg()?;
        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;
        git_tag("1.0.0")?;
        git_commit("feat: add another feature commit")?;

        // Act
        let result = create_version(
            repository,
            BumpOptions {
                increment: IncrementCommand::Auto,
                ..Default::default()
            },
        );

        // Assert
        assert_that!(result).is_ok();
        assert_latest_tag("1.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn annotated_bump_ok() -> Result<()> {
        // Arrange
        let repository = git_init_no_gpg()?;
        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;
        git_tag("1.0.0")?;
        git_commit("feat: add another feature commit")?;

        // Act
        let result = create_version(
            repository,
            BumpOptions {
                annotated: Some(String::from("Release version {{version}}")),
                ..Default::default()
            },
        );

        // Assert
        assert_that!(result).is_ok();
        assert_latest_tag("1.1.0")?;
        assert_tag_is_annotated("1.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn monorepo_bump_ok() -> Result<()> {
        // Arrange
        let mut settings = Settings {
            ..Default::default()
        };

        init_monorepo(&mut settings)?;
        let repository = Repository::open(".")?;

        // Act
        let result = create_monorepo_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();
        assert_tag_exists("0.1.0")?;
        assert_tag_exists("one-0.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn monorepo_bump_manual_ok() -> Result<()> {
        // Arrange
        let mut settings = Settings {
            ..Default::default()
        };

        init_monorepo(&mut settings)?;
        run_cmd!(
            git tag "one-0.1.0";
        )?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_monorepo_version(
            repository,
            BumpOptions {
                increment: IncrementCommand::Major,
                ..Default::default()
            },
        );

        // Assert
        assert_that!(result).is_ok();
        assert_tag_exists("1.0.0")?;
        Ok(())
    }

    #[sealed_test]
    fn monorepo_bump_manual_disable_changelog_ok() -> Result<()> {
        // Arrange
        let mut settings = Settings {
            disable_changelog: true,
            ..Default::default()
        };

        init_monorepo(&mut settings)?;
        run_cmd!(
            git tag "one-0.1.0";
        )?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_monorepo_version(
            repository,
            BumpOptions {
                increment: IncrementCommand::Major,
                ..Default::default()
            },
        );

        // Assert
        assert_that!(result).is_ok();
        assert_tag_exists("1.0.0")?;
        assert_that!(Path::new("CHANGELOG.md")).does_not_exist();
        Ok(())
    }

    #[sealed_test]
    fn monorepo_with_tag_prefix_bump_ok() -> Result<()> {
        // Arrange
        let mut settings = Settings {
            tag_prefix: Some("v".to_string()),
            ..Default::default()
        };

        init_monorepo(&mut settings)?;
        let repository = Repository::open(".")?;

        // Act
        let result = create_monorepo_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();
        assert_tag_exists("v0.1.0")?;
        assert_tag_exists("one-v0.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn package_bump_ok() -> Result<()> {
        // Arrange
        let mut settings = Settings {
            ..Default::default()
        };

        init_monorepo(&mut settings)?;
        let package = settings.packages.get("one").unwrap();
        let repository = Repository::open(".")?;

        // Act
        let result = create_package_version(
            repository,
            PackageBumpOptions {
                package_name: "one",
                package,
                increment: IncrementCommand::AutoPackage("one".to_string()),
                ..Default::default()
            },
        );

        // Assert
        assert_that!(result).is_ok();
        assert_tag_does_not_exist("0.1.0")?;
        assert_tag_exists("one-0.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn consecutive_package_bump_ok() -> Result<()> {
        // Arrange
        let mut packages = HashMap::new();
        let jenkins = || MonoRepoPackage {
            path: PathBuf::from("jenkins"),
            public_api: false,
            changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("jenkins".to_owned(), jenkins());

        let thumbor = || MonoRepoPackage {
            path: PathBuf::from("thumbor"),
            public_api: false,
            changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("thumbor".to_owned(), thumbor());

        let settings = Settings {
            packages,
            ignore_merge_commits: true,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        git_init()?;
        run_cmd!(
            echo Hello > README.md;
            git add .;
            git commit -m "first commit";
            mkdir jenkins;
            echo "some jenkins stuff" > jenkins/file;
            git add .;
            git commit -m "feat(jenkins): add jenkins stuffs";
            mkdir thumbor;
            echo "some thumbor stuff" > thumbor/file;
            git add .;
            git commit -m "feat(thumbor): add thumbor stuffs";
            echo $settings > cog.toml;
            git add .;
            git commit -m "chore: add cog.toml";
        )?;

        let repository = Repository::open(".")?;

        // Act
        create_package_version(
            repository,
            PackageBumpOptions {
                package_name: "thumbor",
                package: &thumbor(),
                increment: IncrementCommand::AutoPackage("thumbor".to_owned()),
                ..Default::default()
            },
        )?;

        clear_cache();

        let repository = Repository::open(".")?;
        create_package_version(
            repository,
            PackageBumpOptions {
                package_name: "jenkins",
                package: &jenkins(),
                increment: IncrementCommand::AutoPackage("jenkins".to_owned()),
                ..Default::default()
            },
        )?;

        run_cmd!(
            echo "fix jenkins bug" > jenkins/fix;
            git add .;
            git commit -m "fix(jenkins): bug fix on jenkins package";
        )?;

        clear_cache();
        let repository = Repository::open(".")?;
        create_package_version(
            repository,
            PackageBumpOptions {
                package_name: "jenkins",
                package: &jenkins(),
                increment: IncrementCommand::AutoPackage("jenkins".to_owned()),
                ..Default::default()
            },
        )?;

        // Assert
        assert_tag_exists("jenkins-0.1.0")?;
        assert_tag_exists("thumbor-0.1.0")?;
        assert_tag_exists("jenkins-0.1.1")?;
        assert_tag_does_not_exist("jenkins-0.2.0")?;
        assert_tag_does_not_exist("0.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn should_fallback_to_0_0_0_when_there_is_no_tag() -> Result<()> {
        // Arrange
        git_init()?;
        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();
        assert_latest_tag("0.1.0")?;
        Ok(())
    }

    #[sealed_test]
    fn should_ignore_latest_prerelease_tag() -> Result<()> {
        // Arrange
        git_init()?;
        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;

        let repository = Repository::open(".")?;

        create_version(
            repository,
            BumpOptions {
                pre_release: Some("alpha1"),
                ..Default::default()
            },
        )?;

        git_commit("feat: more features")?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();
        assert_tag_exists("0.1.0-alpha1")?;
        assert_latest_tag("0.1.0")?;

        Ok(())
    }

    #[sealed_test]
    fn auto_bump_package_only_ok() -> Result<()> {
        // Arrange
        let mut packages = HashMap::new();
        let jenkins = || MonoRepoPackage {
            path: PathBuf::from("jenkins"),
            public_api: false,
            changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("jenkins".to_owned(), jenkins());

        let thumbor = || MonoRepoPackage {
            path: PathBuf::from("thumbor"),
            public_api: false,
            changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("thumbor".to_owned(), thumbor());

        let settings = Settings {
            packages,
            generate_mono_repository_global_tag: false,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        git_init()?;
        run_cmd!(
            echo Hello > README.md;
            git add .;
            git commit -m "first commit";
            mkdir jenkins;
            echo "some jenkins stuff" > jenkins/file;
            git add .;
            git commit -m "feat(jenkins): add jenkins stuffs";
            mkdir thumbor;
            echo "some thumbor stuff" > thumbor/file;
            git add .;
            git commit -m "feat(thumbor): add thumbor stuffs";
            echo $settings > cog.toml;
            git add .;
            git commit -m "chore: add cog.toml";
        )?;

        let repository = Repository::open(".")?;

        // Act
        create_all_package_version_auto(repository, BumpOptions::default())?;

        assert_tag_exists("jenkins-0.1.0")?;
        assert_tag_exists("thumbor-0.1.0")?;
        assert_tag_does_not_exist("0.1.0")?;

        clear_cache();

        run_cmd!(
            echo "fix jenkins bug" > jenkins/fix;
            git add .;
            git commit -m "fix(jenkins): bug fix on jenkins package";
        )?;

        let repository = Repository::open(".")?;
        create_all_package_version_auto(repository, BumpOptions::default())?;

        // Assert
        assert_tag_exists("jenkins-0.1.1")?;
        Ok(())
    }

    // FIXME: Failing on non compliant tag should be configurable
    //  until it's implemented we will ignore non compliant tags
    // #[sealed_test]
    // fn should_fail_when_latest_tag_is_not_semver_compliant() -> Result<()> {
    //     // Arrange
    //     git_init()?;
    //     git_commit("chore: first commit")?;
    //     git_commit("feat: add a feature commit")?;
    //     git_tag("toto")?;
    //     git_commit("feat: add another feature commit")?;
    //
    //     let mut cocogitto = CocoGitto::get()?;
    //
    //     // Act
    //     let result = cocogitto.create_version(VersionIncrement::Auto, None, None, false);
    //     let error = result.unwrap_err().to_string();
    //     let error = error.as_str();
    //
    //     // Assert
    //     assert_that!(error).is_equal_to(indoc!(
    //         "
    //         tag `toto` is not SemVer compliant
    //         \tcause: unexpected character 't' while parsing major version number
    //         "
    //     ));
    //     Ok(())
    // }

    #[sealed_test]
    fn bump_with_whitelisted_branch_ok() -> Result<()> {
        // Arrange
        let settings = r#"branch_whitelist = [ "master" ]"#;

        git_init()?;
        run_cmd!(
            echo $settings > cog.toml;
            git add .;
        )?;

        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();

        Ok(())
    }

    #[sealed_test]
    fn bump_with_whitelisted_branch_fails() -> Result<()> {
        // Arrange
        let settings = r#"branch_whitelist = [ "main" ]"#;

        git_init()?;
        run_cmd!(
            echo $settings > cog.toml;
            git add .;
        )?;

        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result.unwrap_err().to_string()).is_equal_to(
            "No patterns matched in [\"main\"] for branch 'master', bump is not allowed"
                .to_string(),
        );

        Ok(())
    }

    #[sealed_test]
    fn bump_with_whitelisted_branch_pattern_ok() -> Result<()> {
        // Arrange
        let settings = r#"branch_whitelist = [ "main", "release/**" ]"#;

        git_init()?;
        run_cmd!(
            echo $settings > cog.toml;
            git add .;
        )?;

        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;

        run_cmd!(git checkout -b release/1.0.0;)?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();

        Ok(())
    }

    #[sealed_test]
    fn bump_with_whitelisted_branch_pattern_err() -> Result<()> {
        // Arrange
        let settings = r#"branch_whitelist = [ "release/**" ]"#;

        git_init()?;
        run_cmd!(
            echo $settings > cog.toml;
            git add .;
        )?;

        git_commit("chore: first commit")?;
        git_commit("feat: add a feature commit")?;

        let repository = Repository::open(".")?;

        // Act
        let result = create_version(repository, BumpOptions::default());

        // Assert
        assert_that!(result).is_err();

        Ok(())
    }

    #[sealed_test]
    fn bump_no_error_should_be_thrown_on_only_chore_docs_commit() -> Result<()> {
        // Arrange
        let mut packages = HashMap::new();
        let jenkins = || MonoRepoPackage {
            path: PathBuf::from("jenkins"),
            changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("jenkins".to_owned(), jenkins());

        let thumbor = || MonoRepoPackage {
            path: PathBuf::from("thumbor"),
            changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("thumbor".to_owned(), thumbor());

        let settings = Settings {
            packages,
            ignore_merge_commits: true,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        git_init()?;
        run_cmd!(
            echo Hello > README.md;
            git add .;
            git commit -m "first commit";
            mkdir jenkins;
            echo "some jenkins stuff" > jenkins/file;
            git add .;
            git commit -m "feat(jenkins): add jenkins stuffs";
            mkdir thumbor;
            echo "some thumbor stuff" > thumbor/file;
            git add .;
            git commit -m "feat(thumbor): add thumbor stuffs";
            echo $settings > cog.toml;
            git add .;
            git commit -m "chore: add cog.toml";
        )?;

        // Act
        create_monorepo_version(Repository::open(".")?, BumpOptions::default())?;

        run_cmd!(
            echo "chore on jenkins" > jenkins/fix;
            git add .;
            git commit -m "chore(jenkins): jenkins chore";
            echo "docs on jenkins" > jenkins/fix;
            git add .;
            git commit -m "docs(jenkins): jenkins docs";
        )?;
        clear_cache();
        create_monorepo_version(Repository::open(".")?, BumpOptions::default())?;

        clear_cache();
        create_package_version(
            Repository::open(".")?,
            PackageBumpOptions {
                package_name: "jenkins",
                package: &jenkins(),
                increment: IncrementCommand::AutoPackage("jenkins".to_owned()),
                ..Default::default()
            },
        )?;

        run_cmd!(
            echo "more feat on thumbor" > thumbor/feat;
            git add .;
            git commit -m "feat(thumbor): more feat on thumbor";
        )?;

        clear_cache();
        create_monorepo_version(Repository::open(".")?, BumpOptions::default())?;

        // Assert
        assert_tag_exists("jenkins-0.1.0")?;
        assert_tag_exists("thumbor-0.1.0")?;
        assert_tag_exists("thumbor-0.2.0")?;
        assert_tag_exists("0.1.0")?;
        assert_tag_exists("0.2.0")?;

        assert_tag_does_not_exist("jenkins-0.1.1")?;
        assert_tag_does_not_exist("jenkins-0.2.0")?;
        assert_tag_does_not_exist("jenkins-1.0.0")?;
        Ok(())
    }

    #[sealed_test]
    fn error_on_no_conventionnal_commits_found_for_monorepo() -> Result<()> {
        let settings = Settings {
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        git_init()?;

        run_cmd!(
            echo Hello > README.md;
            git add .;
            git commit -m "chore: first commit";

            echo $settings > cog.toml;
            git add .;

            echo "first feature" > file;
            git add .;
            git commit -m "feat: feature commit";
        )?;

        // Act
        let first_result = create_version(Repository::open(".")?, BumpOptions::default());

        // Assert
        assert_that!(first_result).is_ok();

        run_cmd!(
            echo "second feature" > file;
            git add .;
        )?;

        git_commit("second unconventional feature commit")?;

        // Act
        let second_result = create_version(Repository::open(".")?, BumpOptions::default());

        // Assert
        assert_that!(second_result).is_err();

        Ok(())
    }

    #[sealed_test]
    fn error_on_no_conventionnal_commits_found_for_package() -> Result<()> {
        // Arrange
        let mut packages = HashMap::new();
        let jenkins = || MonoRepoPackage {
            path: PathBuf::from("jenkins"),
            changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("jenkins".to_owned(), jenkins());

        let settings = Settings {
            packages,
            ignore_merge_commits: true,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        git_init()?;
        run_cmd!(
            echo Hello > README.md;
            git add .;
            git commit -m "first commit";

            echo $settings > cog.toml;
            git add .;
            git commit -m "chore: cog config";

            mkdir jenkins;
            echo "some jenkins stuff" > jenkins/file;
            git add .;
            git commit -m "feat(jenkins): some jenkins stuff";
        )?;

        let first_result = create_package_version(
            Repository::open(".")?,
            PackageBumpOptions {
                package_name: "jenkins",
                package: &jenkins(),
                increment: IncrementCommand::AutoPackage("jenkins".to_owned()),
                ..Default::default()
            },
        );

        assert_that!(first_result).is_ok();

        run_cmd!(
            echo "some other jenkins stuff" > jenkins/file;
            git add .;
            git commit -m "some other jenkins stuff";
        )?;

        let second_result = create_package_version(
            Repository::open(".")?,
            PackageBumpOptions {
                package_name: "jenkins",
                package: &jenkins(),
                increment: IncrementCommand::AutoPackage("jenkins".to_owned()),
                ..Default::default()
            },
        );

        assert_that!(second_result).is_err();

        Ok(())
    }

    #[sealed_test]
    fn bump_with_unconventionnal_and_conventional_commits_found_for_packages() -> Result<()> {
        // Arrange
        let mut packages = HashMap::new();
        let jenkins = || MonoRepoPackage {
            path: PathBuf::from("jenkins"),
            changelog_path: Some("jenkins/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("jenkins".to_owned(), jenkins());

        let thumbor = || MonoRepoPackage {
            path: PathBuf::from("thumbor"),
            changelog_path: Some("thumbor/CHANGELOG.md".to_owned()),
            ..Default::default()
        };

        packages.insert("thumbor".to_owned(), thumbor());

        let settings = Settings {
            packages,
            ignore_merge_commits: true,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;

        git_init()?;
        run_cmd!(
            echo Hello > README.md;
            git add .;
            git commit -m "first commit";
            mkdir jenkins;
            echo "unconventional jenkins stuff" > jenkins/file;
            git add .;
            git commit -m "unconventional jenkins stuff";
            mkdir thumbor;
            echo "conventional thumbor stuff" > thumbor/file;
            git add .;
            git commit -m "feat(thumbor): conventional thumbor stuff";
            echo $settings > cog.toml;
            git add .;
            git commit -m "chore: add cog.toml";
        )?;

        // Act
        let result = create_monorepo_version(Repository::open(".")?, BumpOptions::default());

        // Assert
        assert_that!(result).is_ok();
        assert_tag_exists("thumbor-0.1.0")?;
        assert_tag_exists("0.1.0")?;

        assert_tag_does_not_exist("jenkins-0.1.0")?;

        Ok(())
    }
}
