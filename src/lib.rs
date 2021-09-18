#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

pub mod error;

pub mod conventional;
pub mod git;
pub mod hook;
pub mod log;
pub mod settings;

use crate::conventional::commit::verify;
use crate::error::PreHookError;
use crate::error::{CocogittoError, CogCheckReport};
use crate::settings::{HookType, Settings};
use anyhow::{Context, Error, Result};
use chrono::Utc;
use colored::*;
use conventional::changelog::{Changelog, ChangelogWriter};
use conventional::commit::Commit;
use conventional::commit::CommitConfig;
use conventional::version::{parse_pre_release, VersionIncrement};
use conventional_commit_parser::commit::{CommitType, ConventionalCommit};
use conventional_commit_parser::parse_footers;
use git::repository::Repository;
use git2::{Oid, RebaseOptions};
use hook::Hook;
use itertools::Itertools;
use log::filter::CommitFilters;
use semver::Version;
use settings::AuthorSetting;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use tempfile::TempDir;

pub type CommitsMetadata = HashMap<CommitType, CommitConfig>;

pub const CONFIG_PATH: &str = "cog.toml";

lazy_static! {
    // This cannot be carried by `Cocogitto` struct since we need it to be available in `Changelog`,
    // `Commit` etc. Be ensure that `CocoGitto::new` is called before using this  in order to bypass
    // unwrapping in case of error.
    pub static ref COMMITS_METADATA: CommitsMetadata = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().commit_types()
        } else {
            Settings::default().commit_types()
        }
    };

        pub static ref HOOK_PROFILES: Vec<String> = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().bump_profiles
            .keys()
            .cloned()
            .collect()
        } else {
            vec![]
        }
    };

    static ref REMOTE_URL: Option<String> = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().github
        } else {
            None
        }
    };

        static ref AUTHORS: Vec<AuthorSetting> = {
        if let Ok(repo) = Repository::open(".") {
            Settings::get(&repo).unwrap_or_default().authors
        } else {
            vec![]
        }
    };
}

pub fn init<S: AsRef<Path> + ?Sized>(path: &S) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        std::fs::create_dir(&path)
            .map_err(|err| anyhow!("Could not create directory `{:?}`: {}", path, err))?;
    }

    let mut is_init_commit = false;
    let repository = match Repository::open(&path) {
        Ok(repo) => {
            println!(
                "Found git repository in {:?}, skipping initialisation",
                &path
            );
            repo
        }
        Err(_) => match Repository::init(&path) {
            Ok(repo) => {
                println!("Empty git repository initialized in {:?}", &path);
                is_init_commit = true;
                repo
            }
            Err(err) => panic!("Unable to init repository on {:?}: {}", &path, err),
        },
    };

    let settings = Settings::default();
    let settings_path = path.join(CONFIG_PATH);
    if settings_path.exists() {
        eprint!("Found {} in {:?}, Nothing to do", CONFIG_PATH, &path);
        exit(1);
    } else {
        std::fs::write(
            &settings_path,
            toml::to_string(&settings)
                .map_err(|err| anyhow!("Failed to serialize {} : {}", CONFIG_PATH, err))?,
        )
        .map_err(|err| anyhow!("Could not write file `{:?}` : {}", &settings_path, err))?;
    }

    // TODO: add cog.toml only
    repository
        .add_all()
        .map_err(|err| anyhow!("Could not add file to repository index : {}", err))?;

    if is_init_commit {
        repository.commit("chore: initial commit")?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct CocoGitto {
    repository: Repository,
    changelog_path: PathBuf,
}

impl CocoGitto {
    pub fn get() -> Result<Self> {
        let repository = Repository::open(&std::env::current_dir()?)?;
        let settings = Settings::get(&repository)?;
        let changelog_path = settings
            .changelog_path
            .unwrap_or_else(|| PathBuf::from("CHANGELOG.md"));

        Ok(CocoGitto {
            repository,
            changelog_path,
        })
    }

    pub fn get_committer(&self) -> Result<String> {
        self.repository.get_author()
    }

    pub fn get_repo_tag_name(&self) -> Option<String> {
        let repo_path = self.repository.get_repo_dir()?.iter().last()?;
        let mut repo_tag_name = repo_path.to_str()?.to_string();

        if let Some(branch_shorthand) = self.repository.get_branch_shorthand() {
            write!(&mut repo_tag_name, " on {}", branch_shorthand).unwrap();
        }

        if let Ok(latest_tag) = self.repository.get_latest_tag() {
            write!(&mut repo_tag_name, " {}", latest_tag).unwrap();
        };

        Some(repo_tag_name)
    }

    pub fn check_and_edit(&self) -> Result<()> {
        let from = self.repository.get_first_commit()?;
        let head = self.repository.get_head_commit()?;
        let commits = self.repository.get_commit_range(from, head.id())?;
        let editor = std::env::var("EDITOR")?;
        let dir = TempDir::new()?;

        let errored_commits: Vec<Oid> = commits
            .iter()
            .map(|commit| {
                let conv_commit = Commit::from_git_commit(commit);
                (commit.id(), conv_commit)
            })
            .filter(|commit| commit.1.is_err())
            .map(|commit| commit.0)
            .collect();

        // Get the last commit oid on the list as a starting point for our rebase
        let last_errored_commit = errored_commits.last();
        if let Some(last_errored_commit) = last_errored_commit {
            let commit = self
                .repository
                .0
                .find_commit(last_errored_commit.to_owned())?;

            let rebase_start = if commit.parent_count() == 0 {
                commit.id()
            } else {
                commit.parent_id(0)?
            };

            let commit = self.repository.0.find_annotated_commit(rebase_start)?;
            let mut options = RebaseOptions::new();

            let mut rebase =
                self.repository
                    .0
                    .rebase(None, Some(&commit), None, Some(&mut options))?;

            while let Some(op) = rebase.next() {
                if let Ok(rebase_operation) = op {
                    let oid = rebase_operation.id();
                    let original_commit = self.repository.0.find_commit(oid)?;
                    if errored_commits.contains(&oid) {
                        println!("Found errored commits : {}", &oid.to_string()[0..7]);
                        let file_path = dir.path().join(&commit.id().to_string());
                        let mut file = File::create(&file_path)?;

                        let hint = format!(
                            "# Editing commit {}\
                        \n# Replace this message with a conventional commit compliant one\
                        \n# Save and exit to edit the next errored commit\n",
                            original_commit.id()
                        );

                        let mut message_bytes: Vec<u8> = hint.clone().into();
                        message_bytes.extend_from_slice(original_commit.message_bytes());
                        file.write_all(&message_bytes)?;

                        Command::new(&editor)
                            .arg(&file_path)
                            .stdout(Stdio::inherit())
                            .stdin(Stdio::inherit())
                            .stderr(Stdio::inherit())
                            .output()?;

                        let new_message: String = std::fs::read_to_string(&file_path)?
                            .lines()
                            .filter(|line| !line.starts_with('#'))
                            .filter(|line| !line.trim().is_empty())
                            .collect();

                        rebase.commit(None, &original_commit.committer(), Some(&new_message))?;
                        match verify(self.repository.get_author().ok(), &new_message) {
                            Ok(_) => println!(
                                "Changed commit message to : \"{}\"",
                                &new_message.trim_end()
                            ),
                            Err(err) => eprintln!(
                                "Error: {}\n\t{}",
                                "Edited message is still not compliant".red(),
                                err
                            ),
                        }
                    } else {
                        rebase.commit(None, &original_commit.committer(), None)?;
                    }
                } else {
                    eprintln!("{:?}", op);
                }
            }

            rebase.finish(None)?;
        } else {
            println!("{}", "No errored commit, skipping rebase".green());
        }

        Ok(())
    }

    pub fn check(&self, check_from_latest_tag: bool) -> Result<()> {
        let from = if check_from_latest_tag {
            self.repository
                .get_latest_tag_oidof()
                .unwrap_or_else(|_err| {
                    println!("No previous tag found, falling back to first commit");
                    self.repository.get_first_commit_oidof().unwrap()
                })
        } else {
            self.repository.get_first_commit_oidof()?
        };

        let to = self.repository.get_head_commit_oid()?;
        let commits = self.repository.get_commit_range(from.get_oid(), to)?;

        let (successes, mut errors): (Vec<_>, Vec<_>) = commits
            .iter()
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge "))
            .map(Commit::from_git_commit)
            .partition_result();

        let type_errors: Vec<Error> = successes
            .iter()
            .map(|commit| {
                let commit_type = &commit.message.commit_type;
                match &COMMITS_METADATA.get(commit_type) {
                    Some(_) => Ok(()),
                    None => Err(anyhow!(CocogittoError::CommitTypeNotAllowed {
                        oid: commit.oid.clone(),
                        summary: commit.format_summary(),
                        commit_type: commit.message.commit_type.to_string(),
                        author: commit.author.clone()
                    })),
                }
            })
            .filter_map(Result::err)
            .collect();

        errors.extend(type_errors);

        if errors.is_empty() {
            let msg = "No errored commits".green();
            println!("{}", msg);
            Ok(())
        } else {
            let report = CogCheckReport { from, errors };
            Err(anyhow!("{}", report))
        }
    }

    pub fn get_log(&self, filters: CommitFilters) -> Result<String> {
        let from = self.repository.get_first_commit()?;
        let to = self.repository.get_head_commit_oid()?;
        let commits = self.repository.get_commit_range(from, to)?;
        let logs = commits
            .iter()
            // Remove merge commits
            .filter(|commit| !commit.message().unwrap_or("").starts_with("Merge"))
            .filter(|commit| filters.filter_git2_commit(commit))
            .map(|commit| Commit::from_git_commit(commit))
            // Apply filters
            .filter(|commit| match commit {
                Ok(commit) => filters.filters(commit),
                Err(_) => filters.no_error(),
            })
            // Format
            .map(|commit| match commit {
                Ok(commit) => commit.get_log(),
                Err(err) => err.to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(logs)
    }

    pub fn conventional_commit(
        &self,
        commit_type: &str,
        scope: Option<String>,
        summary: String,
        body: Option<String>,
        footer: Option<String>,
        is_breaking_change: bool,
    ) -> Result<()> {
        // Ensure commit type is known
        let commit_type = CommitType::from(commit_type);

        // Ensure footers are correctly formatted
        let footers = match footer {
            Some(footers) => parse_footers(&footers)?,
            None => Vec::with_capacity(0),
        };

        let conventional_message = ConventionalCommit {
            commit_type,
            scope,
            body,
            footers,
            summary,
            is_breaking_change,
        }
        .to_string();

        // Validate the message
        conventional_commit_parser::parse(&conventional_message)?;

        // Git commit
        let oid = self.repository.commit(&conventional_message)?;

        // Pretty print a conventional commit summary
        let commit = self.repository.0.find_commit(oid)?;
        let commit = Commit::from_git_commit(&commit)?;
        println!("{}", commit);

        Ok(())
    }

    pub fn create_version(
        &mut self,
        increment: VersionIncrement,
        pre_release: Option<&str>,
        hooks_config: Option<&str>,
    ) -> Result<()> {
        let statuses = self.repository.get_statuses()?;

        // Fail if repo contains un-staged or un-committed changes
        ensure!(statuses.0.is_empty(), "{}", self.repository.get_statuses()?);

        let current_tag = self
            .repository
            .get_latest_tag()
            .unwrap_or_else(|_| Version::new(0, 0, 0).to_string());

        let current_version = Version::parse(&current_tag)?;

        let mut next_version = increment
            .bump(&current_version)
            .map_err(|err| anyhow!("Cannot bump version : {}", err))?;

        if next_version.le(&current_version) || next_version.eq(&current_version) {
            let comparison = format!("{} <= {}", current_version, next_version).red();
            let cause_key = "cause:".red();
            let cause = format!(
                "{} version MUST be greater than current one: {}",
                cause_key, comparison
            );

            bail!(CocogittoError::Semver {
                level: "SemVer Error".red().to_string(),
                cause
            });
        };

        if let Some(pre_release) = pre_release {
            next_version.pre = parse_pre_release(pre_release)?;
        }

        let version_str = next_version.to_string();

        let origin = if current_tag == "0.0.0" {
            self.repository.get_first_commit()?.to_string()
        } else {
            current_tag
        };

        let changelog = self.get_changelog(
            Some(&origin),
            Some(&self.repository.get_head_commit_oid()?.to_string()),
            Some(next_version.to_string()),
        )?;

        let mut writer = ChangelogWriter {
            changelog,
            path: self.changelog_path.clone(),
        };

        writer
            .write()
            .map_err(|err| anyhow!("Unable to write CHANGELOG.md : {}", err))?;

        let hook_result = self.run_hooks(HookType::PreBump, &version_str, hooks_config);
        self.repository.add_all()?;

        // Hook failed, we need to stop here and reset
        // the repository to a clean state
        if let Err(err) = hook_result {
            self.repository.stash_failed_version(&version_str)?;
            eprintln!(
                "{}",
                PreHookError {
                    cause: err.to_string(),
                    version: version_str,
                    stash_number: 0,
                }
            );

            exit(1);
        }

        self.repository
            .commit(&format!("chore(version): {}", next_version))?;
        self.repository.create_tag(&version_str)?;

        self.run_hooks(HookType::PostBump, &version_str, hooks_config)?;

        let bump = format!("{} -> {}", current_version, next_version).green();
        println!("Bumped version : {}", bump);

        Ok(())
    }

    pub fn get_colored_changelog(&self, from: Option<&str>, to: Option<&str>) -> Result<String> {
        let mut changelog = self.get_changelog(from, to, None)?;
        Ok(changelog.markdown(true))
    }

    pub fn get_colored_changelog_at_tag(&self, tag: &str) -> Result<String> {
        let (from, to) = self.repository.get_tag_commits(tag)?;
        let from = from.get_oid().to_string();
        let to = to.get_oid();
        let to = self
            .repository
            .0
            .find_commit(to)?
            .parent_id(0)
            .expect("Unexpected error : Unable to get parent commit")
            .to_string();
        let mut changelog = self.get_changelog(
            Some(from.as_str()),
            Some(to.as_str()),
            Some(tag.to_string()),
        )?;
        Ok(changelog.markdown(true))
    }

    /// ## Get a changelog between two oids
    /// - `from` default value : latest tag or else first commit
    /// - `to` default value : `HEAD` or else first commit
    pub(crate) fn get_changelog(
        &self,
        from: Option<&str>,
        to: Option<&str>,
        tag_name: Option<String>,
    ) -> Result<Changelog> {
        let from = self.resolve_from_arg(from)?;
        let to = self.resolve_to_arg(to)?;
        let from_oid = from.get_oid();
        let to_oid = to.get_oid();

        let mut commits = vec![];

        for commit in self
            .repository
            .get_commit_range(from_oid, to_oid)
            .map_err(|err| anyhow!("Could not get commit range {}...{} : {}", from, to, err))?
        {
            // We skip the origin commit (ex: from 0.1.0 to 1.0.0)
            if commit.id() == from_oid {
                break;
            }

            // Ignore merge commits
            if let Some(message) = commit.message() {
                if message.starts_with("Merge") {
                    continue;
                }
            }

            match Commit::from_git_commit(&commit) {
                Ok(commit) => commits.push(commit),
                Err(err) => {
                    let err = err.to_string().red();
                    eprintln!("{}", err);
                }
            };
        }

        let date = Utc::now().naive_utc().date().to_string();

        Ok(Changelog {
            from,
            to,
            date,
            commits,
            tag_name,
        })
    }

    // TODO : revparse
    fn resolve_to_arg(&self, to: Option<&str>) -> Result<OidOf> {
        if let Some(to) = to {
            self.get_raw_oid_or_tag_oid(to)
        } else {
            self.repository
                .get_head_commit_oid()
                .map(OidOf::Head)
                .or_else(|_err| self.repository.get_first_commit().map(OidOf::Other))
        }
    }

    // TODO : revparse
    fn resolve_from_arg(&self, from: Option<&str>) -> Result<OidOf> {
        if let Some(from) = from {
            self.get_raw_oid_or_tag_oid(from)
                .map_err(|err| anyhow!("Could not resolve from arg {} : {}", from, err))
        } else {
            self.repository
                .get_latest_tag_oidof()
                .or_else(|_err| self.repository.get_first_commit().map(OidOf::Other))
        }
    }

    fn get_raw_oid_or_tag_oid(&self, input: &str) -> Result<OidOf> {
        if let Ok(_version) = Version::parse(input) {
            self.repository
                .resolve_lightweight_tag(input)
                .map(|oid| OidOf::Tag(input.to_owned(), oid))
                .map_err(|err| anyhow!("tag {} not found : {} ", input, err))
        } else {
            Oid::from_str(input)
                .map(OidOf::Other)
                .map_err(|err| anyhow!("`{}` is not a valid oid : {}", input, err))
        }
    }

    fn run_hooks(
        &self,
        hook_type: HookType,
        next_version: &str,
        hook_profile: Option<&str>,
    ) -> Result<()> {
        let settings = Settings::get(&self.repository)?;

        let hooks: Vec<Hook> = match hook_profile {
            Some(profile) => settings
                .get_profile_hook(profile, hook_type)
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
            None => settings
                .get_hooks(hook_type)
                .iter()
                .map(|s| s.parse())
                .enumerate()
                .map(|(idx, result)| result.context(format!("Cannot parse hook at index {}", idx)))
                .try_collect()?,
        };

        let current_version = self.repository.get_latest_tag().ok();

        for mut hook in hooks {
            hook.insert_versions(current_version.clone(), next_version)?;
            hook.run().context(hook.to_string())?;
        }

        Ok(())
    }
}

/// A wrapper for git2 oid including tags and HEAD ref
#[derive(Debug, PartialEq, Eq)]
enum OidOf {
    Tag(String, Oid),
    Head(Oid),
    Other(Oid),
}

impl OidOf {
    fn get_oid(&self) -> Oid {
        match self {
            OidOf::Tag(_, v) | OidOf::Head(v) | OidOf::Other(v) => v.to_owned(),
        }
    }
}

impl Display for OidOf {
    /// Print the oid according to it's type
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OidOf::Tag(tag, _) => write!(f, "{}", tag),
            OidOf::Head(_) => write!(f, "HEAD"),
            OidOf::Other(oid) => write!(f, "{}", &oid.to_string()[0..6]),
        }
    }
}
