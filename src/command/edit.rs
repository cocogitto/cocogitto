use crate::conventional::commit::{verify, Commit};
use crate::git::revspec::RevspecPattern;
use crate::{CocoGitto, SETTINGS};
use anyhow::{anyhow, Result};
use colored::*;
use git2::{Oid, RebaseOptions};
use log::{error, info, warn};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

impl CocoGitto {
    pub fn check_and_edit(&self, from_latest_tag: bool) -> Result<()> {
        let commits = if from_latest_tag {
            self.repository
                .get_commit_range(&RevspecPattern::default())?
        } else {
            self.repository.all_commits()?
        };

        let editor = std::env::var("EDITOR")
            .map_err(|_err| anyhow!("the 'EDITOR' environment variable was not found"))?;

        let dir = TempDir::new()?;

        let errored_commits: Vec<Oid> = commits
            .commits
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

            let editor = &editor;

            while let Some(op) = rebase.next() {
                if let Ok(rebase_operation) = op {
                    let oid = rebase_operation.id();
                    let original_commit = self.repository.0.find_commit(oid)?;
                    if errored_commits.contains(&oid) {
                        warn!("Found errored commits:{}", &oid.to_string()[0..7]);
                        let file_path = dir.path().join(commit.id().to_string());
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

                        Command::new(editor)
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
                        let ignore_merge_commit = SETTINGS.ignore_merge_commits;
                        match verify(
                            self.repository.get_author().ok(),
                            &new_message,
                            ignore_merge_commit,
                        ) {
                            Ok(_) => {
                                info!("Changed commit message to:\"{}\"", &new_message.trim_end())
                            }
                            Err(err) => error!(
                                "Error: {}\n\t{}",
                                "Edited message is still not compliant".red(),
                                err
                            ),
                        }
                    } else {
                        rebase.commit(None, &original_commit.committer(), None)?;
                    }
                } else {
                    error!("{:?}", op);
                }
            }

            rebase.finish(None)?;
        } else {
            info!("{}", "No errored commit, skipping rebase".green());
        }

        Ok(())
    }
}
