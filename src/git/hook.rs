use std::fs::{self, Permissions};
use std::io;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::CocoGitto;

use anyhow::{anyhow, Result};

pub(crate) static PRE_PUSH_HOOK: &[u8] = include_bytes!("assets/pre-push");
pub(crate) static PREPARE_COMMIT_HOOK: &[u8] = include_bytes!("assets/commit-msg");
const PRE_COMMIT_HOOK_PATH: &str = ".git/hooks/commit-msg";
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

fn create_hook(path: &Path, kind: HookKind) -> io::Result<()> {
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
    use std::fs::File;

    use crate::git::hook::HookKind;
    use crate::CocoGitto;

    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;
    use speculoos::prelude::*;
    use std::path::Path;

    #[sealed_test]
    fn add_pre_commit_hook() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_hook(HookKind::PrepareCommit)?;

        // Assert
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        assert_that!(Path::new(".git/hooks/pre-push")).does_not_exist();
        Ok(())
    }

    #[sealed_test]
    fn add_pre_push_hook() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_hook(HookKind::PrePush)?;

        // Assert
        assert_that!(Path::new(".git/hooks/pre-push")).exists();
        assert_that!(Path::new(".git/hooks/pre-commit")).does_not_exist();
        Ok(())
    }

    #[sealed_test]
    fn add_all() -> Result<()> {
        // Arrange
        run_cmd!(git init)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_hook(HookKind::All)?;

        // Assert
        assert_that!(Path::new(".git/hooks/pre-push")).exists();
        assert_that!(Path::new(".git/hooks/commit-msg")).exists();
        Ok(())
    }

    #[sealed_test]
    #[cfg(target_family = "unix")]
    fn should_have_perm_755_on_unix() -> Result<()> {
        // Arrange
        use std::os::unix::fs::PermissionsExt;
        run_cmd!(git init)?;

        let cog = CocoGitto::get()?;

        // Act
        cog.install_hook(HookKind::PrePush)?;

        // Assert
        let prepush = File::open(".git/hooks/pre-push")?;
        let metadata = prepush.metadata()?;
        assert_that!(Path::new(".git/hooks/pre-push")).exists();
        assert_that!(metadata.permissions().mode() & 0o777).is_equal_to(0o755);
        Ok(())
    }
}
