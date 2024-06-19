use anyhow::anyhow;
use cocogitto::CogCommand;
use cocogitto_commit::verify;
use cocogitto_commit::{Commit, CommitType};
use cocogitto_git::tag::TagLookUpOptions;
use cocogitto_git::Repository;
use colored::Colorize;
use git2::{Oid, RebaseOptions};
use log::{error, info, warn};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

pub struct CogEditCommand {
    pub from_latest_tag: bool,
}

impl CogCommand for CogEditCommand {
    fn execute(self) -> anyhow::Result<()> {
        let repository = &Self::repository()?;
        let path = Self::default_path()?;
        let settings = Self::settings(path.as_path())?;
        let from_latest_tag = self.from_latest_tag || settings.from_latest_tag;
        let allowed_commits = settings.allowed_commit_types();

        check_and_edit(
            repository,
            from_latest_tag,
            &allowed_commits,
            settings.ignore_merge_commits,
        )
    }
}

fn check_and_edit(
    repository: &Repository,
    from_latest_tag: bool,
    allowed_commits: &[CommitType],
    ignore_merge_commits: bool,
) -> anyhow::Result<()> {
    let commits = if from_latest_tag {
        let tag = repository.get_latest_tag(TagLookUpOptions::default())?;
        repository.revwalk(&format!("{tag}.."))?
    } else {
        repository.revwalk("..")?
    };

    let editor = std::env::var("EDITOR")
        .map_err(|_err| anyhow!("the 'EDITOR' environment variable was not found"))?;

    let dir = TempDir::new()?;

    let errored_commits: Vec<Oid> = commits
        .iter_commits()
        .map(|commit| {
            let conv_commit = Commit::from_git_commit(commit, allowed_commits);
            (commit.id(), conv_commit)
        })
        .filter(|commit| commit.1.is_err())
        .map(|commit| commit.0)
        .collect();

    // Get the last commit oid on the list as a starting point for our rebase
    let last_errored_commit = errored_commits.last();
    if let Some(last_errored_commit) = last_errored_commit {
        let commit = repository.find_commit(last_errored_commit.to_owned())?;

        let rebase_start = if commit.parent_count() == 0 {
            commit.id()
        } else {
            commit.parent_id(0)?
        };

        let commit = repository.find_annotated_commit(rebase_start)?;
        let mut options = RebaseOptions::new();

        let mut rebase = repository.rebase(None, Some(&commit), None, Some(&mut options))?;

        let editor = &editor;

        while let Some(op) = rebase.next() {
            if let Ok(rebase_operation) = op {
                let oid = rebase_operation.id();
                let original_commit = repository.find_commit(oid)?;
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
                    let ignore_merge_commit = ignore_merge_commits;
                    match verify(
                        repository.get_author().ok(),
                        &new_message,
                        ignore_merge_commit,
                        allowed_commits,
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
