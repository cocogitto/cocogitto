use crate::conventional::commit::Commit;
use colored::*;
use conventional_commit_parser::commit::CommitType;
use git2::Commit as Git2Commit;
use itertools::Itertools;
use log::info;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
    Auto,
    Manual(String),
}

impl VersionIncrement {
    // TODO: move that to a dedicated CLI display module
    pub(crate) fn display_history(commits: &[&Git2Commit]) -> Result<(), fmt::Error> {
        let conventional_commits: Vec<Result<_, _>> = commits
            .iter()
            .map(|commit| Commit::from_git_commit(commit))
            .collect();

        // Commits which type are neither feat, fix nor breaking changes
        // won't affect the version number.
        let mut non_bump_commits: Vec<&CommitType> = conventional_commits
            .iter()
            .filter_map(|commit| match commit {
                Ok(commit) => match commit.message.commit_type {
                    CommitType::Feature | CommitType::BugFix => None,
                    _ => Some(&commit.message.commit_type),
                },
                Err(_) => None,
            })
            .collect();

        non_bump_commits.sort();

        let non_bump_commits: Vec<(usize, &CommitType)> = non_bump_commits
            .into_iter()
            .dedup_by_with_count(|c1, c2| c1 == c2)
            .collect();

        if !non_bump_commits.is_empty() {
            let mut skip_message = "\tSkipping irrelevant commits:\n".to_string();
            for (count, commit_type) in non_bump_commits {
                writeln!(skip_message, "\t\t- {}: {}", commit_type.as_ref(), count)?;
            }

            info!("{}", skip_message);
        }

        let bump_commits = conventional_commits
            .iter()
            .filter_map(|commit| match commit {
                Ok(commit) => match commit.message.commit_type {
                    CommitType::Feature | CommitType::BugFix => Some(Ok(commit)),
                    _ => None,
                },
                Err(err) => Some(Err(err)),
            });

        for commit in bump_commits {
            match commit {
                Ok(commit) if commit.message.is_breaking_change => {
                    info!(
                        "\t Found {} commit {} with type: {}",
                        "BREAKING CHANGE".red(),
                        commit.shorthand().blue(),
                        commit.message.commit_type.as_ref().yellow()
                    )
                }
                Ok(commit) if commit.message.commit_type == CommitType::BugFix => {
                    info!("\tFound bug fix commit {}", commit.shorthand().blue())
                }
                Ok(commit) if commit.message.commit_type == CommitType::Feature => {
                    info!("\tFound feature commit {}", commit.shorthand().blue())
                }
                _ => (),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
// Auto version tests resides in test/ dir since it rely on git log
// To generate the version
mod test {
    // TODO
}
