use std::path::Path;
use std::{fs, io};

use anyhow::anyhow;
use anyhow::Result;

use cog_command::CogCommand;

pub struct CogInstallGitHookCommand<'a> {
    pub hooks: Vec<Hook<'a>>,
    pub overwrite: bool,
}

impl CogCommand for CogInstallGitHookCommand<'_> {
    fn execute(self) -> anyhow::Result<()> {
        let repodir = &Self::repository()?
            .get_repo_dir()
            .ok_or_else(|| anyhow!("Repository root directory not found"))?
            .to_path_buf();

        for hook in self.hooks {
            install_git_hook(repodir, self.overwrite, &hook)?
        }

        Ok(())
    }
}

pub enum Hook<'a> {
    Script { script: &'a str, r#type: &'a str },
    File { path: &'a Path, r#type: &'a str },
}

impl Hook<'_> {
    fn get_type(&self) -> &str {
        match self {
            Hook::Script { r#type, .. } | Hook::File { r#type, .. } => r#type,
        }
    }
}

fn install_git_hook(repodir: &Path, overwrite_existing_hooks: bool, hook: &Hook) -> Result<()> {
    let hook_path = repodir.join(".git/hooks");
    let hook_path = hook_path.join::<&str>(hook.get_type());

    if !overwrite_existing_hooks && hook_path.exists() {
        let mut answer = String::new();
        println!(
            "Git hook `{}` exists. (Overwrite Y/n)",
            hook_path.to_string_lossy()
        );
        io::stdin().read_line(&mut answer)?;

        if !answer.trim().eq_ignore_ascii_case("y") {
            println!("Aborting");
            return Ok(());
        }
    }

    match hook {
        Hook::Script { script, .. } => fs::write(&hook_path, script)?,
        Hook::File { path, .. } => {
            fs::copy(path, &hook_path)?;
        }
    };

    #[cfg(not(target_os = "windows"))]
    {
        use std::fs::Permissions;
        use std::os::unix::fs::PermissionsExt;
        let permissions = Permissions::from_mode(0o755);
        fs::set_permissions(hook_path, permissions)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use anyhow::Result;
    use cmd_lib::run_cmd;
    use cocogitto_test_helpers::git_init_no_gpg;
    use cog_command::CogCommand;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;

    use crate::{CogInstallGitHookCommand, Hook};

    #[sealed_test]
    fn add_pre_commit_hook() -> Result<()> {
        // Arrange
        let _ = git_init_no_gpg()?;
        let script = r#"
if cog check; then
    exit 0
fi

echo "Invalid commits were found, force push with '--no-verify'"
exit 1"#;

        // Act
        CogInstallGitHookCommand {
            hooks: vec![Hook::Script {
                script,
                r#type: "commit-msg",
            }],
            overwrite: true,
        }
        .execute()?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks.as_str()).is_equal_to(script);
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();
        Ok(())
    }

    #[sealed_test]
    fn add_all() -> Result<()> {
        // Arrange
        let _ = git_init_no_gpg()?;
        run_cmd!(echo "echo toto" > pre-push;)?;
        let script = r#"
if cog check; then
    exit 0
fi

echo "Invalid commits were found, force push with '--no-verify'"
exit 1"#;

        // Act
        CogInstallGitHookCommand {
            hooks: vec![
                Hook::Script {
                    script,
                    r#type: "commit-msg",
                },
                Hook::File {
                    path: Path::new("pre-push"),
                    r#type: "pre-push",
                },
            ],
            overwrite: true,
        }
        .execute()?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hook = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hook.as_str()).is_equal_to(script);

        assert_that!(Path::new(".git/hooks/pre-push")).exists();
        let hook = fs::read_to_string(".git/hooks/pre-push")?;
        assert_that!(hook.as_str()).is_equal_to("echo toto\n");
        Ok(())
    }

    #[sealed_test]
    fn overwrite_pre_commit_hook() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;
        let script = r#"
if cog check; then
    exit 0
fi

echo "Invalid commits were found, force push with '--no-verify'"
exit 1"#;

        // Act
        CogInstallGitHookCommand {
            hooks: vec![Hook::Script {
                script,
                r#type: "commit-msg",
            }],
            overwrite: false,
        }
        .execute()?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks.as_str()).is_equal_to(script);
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();

        // Prepare: create empty file
        fs::write(".git/hooks/commit-msg", "")?;

        // Act 2: reinstall hooks with overwrite true
        CogInstallGitHookCommand {
            hooks: vec![Hook::Script {
                script,
                r#type: "commit-msg",
            }],
            overwrite: true,
        }
        .execute()?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks.as_str()).is_equal_to(script);
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();

        // Prepare: create empty file
        fs::write(".git/hooks/commit-msg", "")?;

        // Act 3: reinstall hooks without overwrite true
        CogInstallGitHookCommand {
            hooks: vec![Hook::Script {
                script,
                r#type: "commit-msg",
            }],
            overwrite: false,
        }
        .execute()?;

        // Assert: file must still be empty
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks).is_empty();
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();

        Ok(())
    }
}
