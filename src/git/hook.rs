use crate::CocoGitto;
use anyhow::Result;
use std::path::PathBuf;

use std::fs::Permissions;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use std::fs;

pub static PRE_PUSH_HOOK: &[u8] = include_bytes!("assets/pre-push");
pub static PREPARE_COMMIT_HOOK: &[u8] = include_bytes!("assets/prepare-commit-msg");
const PRE_COMMIT_HOOK_PATH: &str = ".git/hooks/prepare-commit-msg";
const PRE_PUSH_HOOK_PATH: &str = ".git/hooks/pre-push";

pub enum HookKind {
    PrepareCommit,
    PrePush,
    All,
}

impl CocoGitto {
    pub fn install_hook(&self, kind: HookKind) -> Result<()> {
        let repodir = &self
            .repository
            .get_repo_dir()
            .ok_or_else(|| anyhow!("Repository root directory not found"))?
            .to_path_buf();

        match kind {
            HookKind::PrepareCommit => create_hook(repodir, HookKind::PrepareCommit)?,
            HookKind::PrePush => create_hook(repodir, HookKind::PrePush)?,
            HookKind::All => {
                create_hook(repodir, HookKind::PrepareCommit)?;
                create_hook(repodir, HookKind::PrePush)?
            }
        };

        Ok(())
    }
}

fn create_hook(path: &PathBuf, kind: HookKind) -> Result<()> {
    let (hook_path, hook_content) = match kind {
        HookKind::PrepareCommit => (path.join(PRE_COMMIT_HOOK_PATH), PREPARE_COMMIT_HOOK),
        HookKind::PrePush => (path.join(PRE_PUSH_HOOK_PATH), PRE_PUSH_HOOK),
        HookKind::All => unreachable!(),
    };

    fs::write(&hook_path, hook_content)?;

    #[cfg(target_family = "unix")]
    {
        let permissions = Permissions::from_mode(0o755);
        fs::set_permissions(&hook_path, permissions)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::git::hook::HookKind;
    use crate::CocoGitto;
    use anyhow::Result;
    use std::env;
    use std::fs::File;
    use std::ops::Not;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn add_pre_commit_hook() -> Result<()> {
        let temp = TempDir::new()?;
        let temp = temp.path().to_path_buf();
        env::set_current_dir(&temp)?;

        Command::new("git").arg("init").output()?;

        let cog = CocoGitto::get()?;

        cog.install_hook(HookKind::PrepareCommit)?;

        assert!(PathBuf::from(".git/hooks/prepare-commit-msg").exists());
        assert!(PathBuf::from(".git/hooks/pre-push").exists().not());
        Ok(())
    }

    #[test]
    fn add_pre_push_hook() -> Result<()> {
        let tmp = TempDir::new()?;
        let temp = tmp.path().to_path_buf();
        env::set_current_dir(&temp)?;

        Command::new("git").arg("init").output()?;

        let cog = CocoGitto::get()?;

        cog.install_hook(HookKind::PrePush)?;

        assert!(PathBuf::from(".git/hooks/pre-push").exists());
        assert!(PathBuf::from(".git/hooks/prepare-commit-msg")
            .exists()
            .not());
        Ok(())
    }

    #[test]
    fn add_all() -> Result<()> {
        let tmp = TempDir::new()?;
        let tmp = tmp.path().to_path_buf();
        env::set_current_dir(&tmp)?;

        Command::new("git").arg("init").output()?;

        let cog = CocoGitto::get()?;

        cog.install_hook(HookKind::All)?;

        assert!(PathBuf::from(".git/hooks/pre-push").exists());
        assert!(PathBuf::from(".git/hooks/prepare-commit-msg").exists());
        Ok(())
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn should_have_perm_755_on_unix() -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let tmp = TempDir::new()?;
        let tmp = tmp.path().to_path_buf();
        env::set_current_dir(&tmp)?;

        Command::new("git").arg("init").output()?;

        let cog = CocoGitto::get()?;

        cog.install_hook(HookKind::PrePush)?;

        let prepush = File::open(".git/hooks/pre-push")?;
        let metadata = prepush.metadata()?;
        assert!(PathBuf::from(".git/hooks/pre-push").exists());
        assert_eq!(metadata.permissions().mode() & 0o777, 0o755);
        Ok(())
    }
}
