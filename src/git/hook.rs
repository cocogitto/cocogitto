use std::path::Path;
use std::{fs, io};

use crate::settings::{GitHook, GitHookType};
use anyhow::Result;

pub fn install_git_hook(
    repodir: &Path,
    overwrite_existing_hooks: bool,
    hook_type: &GitHookType,
    hook: &GitHook,
) -> Result<()> {
    let hook_path = repodir.join(".git/hooks");
    let hook_path = hook_path.join::<&str>((*hook_type).into());

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
        GitHook::Script { script } => fs::write(&hook_path, script)?,
        GitHook::File { path } => {
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
    use std::collections::HashMap;
    use std::fs;

    use crate::CocoGitto;

    use crate::settings::{GitHook, GitHookType, Settings};
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::path::Path;

    #[sealed_test]
    fn add_pre_commit_hook() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;
        let mut git_hooks = HashMap::new();
        let hooks_script = r#"
if cog check; then
    exit 0
fi

echo "Invalid commits were found, force push with '--no-verify'"
exit 1"#
            .to_string();

        git_hooks.insert(
            GitHookType::CommitMsg,
            GitHook::Script {
                script: hooks_script.clone(),
            },
        );

        let settings = Settings {
            git_hooks,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_git_hooks(true, vec![GitHookType::CommitMsg])?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks).is_equal_to(&hooks_script);
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();
        Ok(())
    }

    #[sealed_test]
    fn add_all() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;
        run_cmd!(echo "echo toto" > pre-push;)?;

        let mut git_hooks = HashMap::new();
        let hooks_script = r#"
if cog check; then
    exit 0
fi

echo "Invalid commits were found, force push with '--no-verify'"
exit 1"#
            .to_string();

        git_hooks.insert(
            GitHookType::CommitMsg,
            GitHook::Script {
                script: hooks_script.clone(),
            },
        );

        git_hooks.insert(
            GitHookType::PrePush,
            GitHook::File {
                path: "pre-push".into(),
            },
        );

        let settings = Settings {
            git_hooks,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_all_hooks(true)?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hook = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hook).is_equal_to(&hooks_script);

        assert_that!(Path::new(".git/hooks/pre-push")).exists();
        let hook = fs::read_to_string(".git/hooks/pre-push")?;
        assert_that!(hook).is_equal_to("echo toto\n".to_string());
        Ok(())
    }

    #[sealed_test]
    fn overwrite_pre_commit_hook() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;
        let mut git_hooks = HashMap::new();
        let hooks_script = r#"
if cog check; then
    exit 0
fi

echo "Invalid commits were found, force push with '--no-verify'"
exit 1"#
            .to_string();

        git_hooks.insert(
            GitHookType::CommitMsg,
            GitHook::Script {
                script: hooks_script.clone(),
            },
        );

        let settings = Settings {
            git_hooks,
            ..Default::default()
        };

        let settings = toml::to_string(&settings)?;
        fs::write("cog.toml", settings)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_git_hooks(false, vec![GitHookType::CommitMsg])?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks).is_equal_to(&hooks_script);
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();

        // Prepare: create empty file
        fs::write(".git/hooks/commit-msg", "")?;

        // Act 2: reinstall hooks with overwrite true
        cog.install_git_hooks(true, vec![GitHookType::CommitMsg])?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks).is_equal_to(&hooks_script);
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();

        // Prepare: create empty file
        fs::write(".git/hooks/commit-msg", "")?;

        // Act 3: reinstall hooks without overwrite true
        cog.install_git_hooks(false, vec![GitHookType::CommitMsg])?;

        // Assert: file must still be empty
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        let hooks = fs::read_to_string(".git/hooks/commit-msg")?;
        assert_that!(hooks).is_empty();
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();

        Ok(())
    }
}
