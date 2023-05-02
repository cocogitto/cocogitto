use crate::git::hook::install_git_hook;
use crate::settings::GitHookType;
use crate::{CocoGitto, SETTINGS};
use anyhow::{anyhow, Result};

impl CocoGitto {
    pub fn install_all_hooks(&self) -> Result<()> {
        let repodir = &self
            .repository
            .get_repo_dir()
            .ok_or_else(|| anyhow!("Repository root directory not found"))?
            .to_path_buf();

        for (hook_type, hook) in SETTINGS.git_hooks.iter() {
            install_git_hook(repodir, hook_type, hook)?;
        }

        Ok(())
    }

    pub fn install_git_hooks(&self, hook_types: Vec<GitHookType>) -> Result<()> {
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
            install_git_hook(repodir, &hook_type, hook)?
        }

        Ok(())
    }
}
