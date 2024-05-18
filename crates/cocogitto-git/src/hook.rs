use std::path::Path;
use std::{fs, io};

use anyhow::Result;
use cocogitto_config::git_hook::{GitHook, GitHookType};

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
