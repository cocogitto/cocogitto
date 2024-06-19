use anyhow::bail;
use cocogitto_commit::{validate_and_get_message, Commit};
use cocogitto_git::error::Git2Error;
use cocogitto_git::Repository;
use cog_command::CogCommand;
use conventional_commit_parser::commit::Separator;
use itertools::Itertools;
use log::info;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{fs, io};

pub struct CogCommitCommand<'a> {
    pub typ: &'a str,
    pub message: String,
    pub scope: Option<String>,
    pub breaking_change: bool,
    pub edit: bool,
    pub sign: bool,
    pub skip_ci: bool,
    pub skip_ci_override: Option<String>,
    pub add_files: bool,
    pub update_files: bool,
}

pub enum CommitHook {
    PreCommit,
    PrepareCommitMessage(String),
    CommitMessage,
    PostCommit,
}

impl CogCommand for CogCommitCommand<'_> {
    fn execute(self) -> anyhow::Result<()> {
        let repository = &Self::repository()?;
        let path = Self::default_path()?;
        let settings = &Self::settings(path.as_path())?;

        Self::run_commit_hook(repository, CommitHook::PreCommit)?;
        let commit_message_path = Self::prepare_edit_message_path(repository);

        let commit_message = if self.skip_ci || self.skip_ci_override.is_some() {
            format!(
                "{} {}",
                self.message,
                self.skip_ci_override.unwrap_or(settings.skip_ci.clone())
            )
        } else {
            self.message.to_string()
        };

        let template = prepare_edit_message(
            self.typ,
            &commit_message,
            self.scope.as_deref(),
            self.breaking_change,
            &commit_message_path,
        )?;
        Self::run_commit_hook(repository, CommitHook::PrepareCommitMessage(template))?;

        let (body, footer, breaking) = if self.edit {
            edit_message(&commit_message_path, self.breaking_change)?
        } else {
            (None, None, self.breaking_change)
        };

        let conventional_message =
            validate_and_get_message(self.typ, self.scope, commit_message, body, footer, breaking)?;

        if self.add_files {
            repository.add_all()?;
        }

        if self.update_files {
            repository.update_all()?;
        }

        // Git commit
        let sign = self.sign || repository.gpg_sign();
        let edit_message_path = Self::prepare_edit_message_path(repository);
        fs::write(edit_message_path, &conventional_message)?;
        Self::run_commit_hook(repository, CommitHook::CommitMessage)?;
        let oid = repository.commit(&conventional_message, sign, false)?;

        // Pretty print a conventional commit summary
        let commit = repository.find_commit(oid)?;
        let commit = Commit::from_git_commit(&commit, &settings.allowed_commit_types())?;
        info!("{}", commit);

        Ok(())
    }
}

impl CogCommitCommand<'_> {
    fn run_commit_hook(repository: &Repository, hook: CommitHook) -> anyhow::Result<(), Git2Error> {
        let repo_dir = repository.get_repo_dir().expect("git repository");
        let hooks_dir = repo_dir.join(".git/hooks");
        let edit_message = repo_dir.join(".git/COMMIT_EDITMSG");
        let edit_message = edit_message.to_string_lossy();

        let (hook_path, args) = match hook {
            CommitHook::PreCommit => (hooks_dir.join("pre-commit"), vec![]),
            CommitHook::PrepareCommitMessage(template) => (
                hooks_dir.join("prepare-commit-msg"),
                vec![edit_message.to_string(), template],
            ),
            CommitHook::CommitMessage => {
                (hooks_dir.join("commit-msg"), vec![edit_message.to_string()])
            }
            CommitHook::PostCommit => (hooks_dir.join("post-commit"), vec![]),
        };

        if hook_path.exists() {
            let status = Command::new(hook_path)
                .args(args)
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?
                .status;

            if !status.success() {
                return Err(Git2Error::GitHookNonZeroExit(status.code().unwrap_or(1)));
            }
        }

        Ok(())
    }

    fn prepare_edit_message_path(repository: &Repository) -> PathBuf {
        repository
            .get_repo_dir()
            .map(|path| path.join(".git/COMMIT_EDITMSG"))
            .expect("git repository")
    }
}

pub fn prepare_edit_message<P: AsRef<Path>>(
    typ: &str,
    message: &str,
    scope: Option<&str>,
    breaking: bool,
    path: P,
) -> io::Result<String> {
    let template = prepare_edit_template(typ, message, scope, breaking);
    fs::write(path, &template)?;
    Ok(template)
}

pub fn edit_message<P: AsRef<Path>>(
    path: P,
    breaking: bool,
) -> anyhow::Result<(Option<String>, Option<String>, bool)> {
    let template = fs::read_to_string(path.as_ref())?;
    let edited = edit::edit(template)?;

    if edited.lines().all(|line| {
        let trimmed = line.trim_start();
        trimmed.is_empty() || trimmed.starts_with('#')
    }) {
        bail!("Aborted commit message edit");
    }

    let content = edited
        .lines()
        .filter(|&line| !line.trim_start().starts_with('#'))
        .join("\n");

    let cc = conventional_commit_parser::parse(content.trim())?;

    let footers: Option<String> = if cc.footers.is_empty() {
        None
    } else {
        Some(
            cc.footers
                .iter()
                .map(|footer| {
                    let separator = match footer.token_separator {
                        Separator::Colon => ": ",
                        Separator::Hash => " #",
                        Separator::ColonWithNewLine => " \n",
                    };
                    format!("{}{}{}", footer.token, separator, footer.content)
                })
                .join("\n"),
        )
    };

    Ok((
        cc.body.map(|s| s.trim().to_string()),
        footers,
        cc.is_breaking_change || breaking,
    ))
}

const EDIT_TEMPLATE: &str = "# Enter the commit message for your changes.
# Lines starting with # will be ignored, and empty body/footer are allowed.
# Once you are done, save the changes and exit the editor.
# Remove all non-comment lines to abort.
#
";

fn prepare_header(typ: &str, message: &str, scope: Option<&str>) -> String {
    let mut header = typ.to_string();

    if let Some(scope) = scope {
        write!(&mut header, "({scope})").unwrap();
    }

    write!(&mut header, ": {message}").unwrap();

    header
}

fn prepare_edit_template(typ: &str, message: &str, scope: Option<&str>, breaking: bool) -> String {
    let mut template: String = EDIT_TEMPLATE.into();
    let header = prepare_header(typ, message, scope);

    if breaking {
        template.push_str("# WARNING: This will be marked as a breaking change!\n");
    }

    write!(
        &mut template,
        "{header}\n\n# Message body\n\n\n# Message footer\n# For example, foo: bar\n\n\n"
    )
    .unwrap();

    template
}
