use crate::CocoGitto;
use anyhow::{anyhow, Result};
use cocogitto_config::{git_hook::GitHookType, SETTINGS};
use cocogitto_git::hook::install_git_hook;

impl CocoGitto {
    pub fn install_all_hooks(&self, overwrite_existing_hooks: bool) -> Result<()> {
        let repodir = &self
            .repository
            .get_repo_dir()
            .ok_or_else(|| anyhow!("Repository root directory not found"))?
            .to_path_buf();

        for (hook_type, hook) in SETTINGS.git_hooks.iter() {
            install_git_hook(repodir, overwrite_existing_hooks, hook_type, hook)?;
        }

        Ok(())
    }

    pub fn install_git_hooks(
        &self,
        overwrite_existing_hooks: bool,
        hook_types: Vec<GitHookType>,
    ) -> Result<()> {
        let repodir = &self
            .repository
            .get_repo_dir()
            .ok_or_else(|| anyhow!("Repository root directory not found"))?
            .to_path_buf();

        for hook_type in hook_types {
            let hook = SETTINGS
                .git_hooks
                .get(&hook_type)
                .ok_or(anyhow!("git-hook {hook_type} was not found in cog.toml"))?;
            install_git_hook(repodir, overwrite_existing_hooks, &hook_type, hook)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use cocogitto_test_helpers::git_init_no_gpg;

    use crate::CocoGitto;
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use cocogitto_config::{
        git_hook::{GitHook, GitHookType},
        Settings,
    };
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::path::Path;

    #[sealed_test]
    fn add_pre_commit_hook() -> Result<()> {
        // Arrange
        let _ = git_init_no_gpg()?;
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
        let _ = git_init_no_gpg()?;
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
