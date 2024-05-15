use cmd_lib::{run_cmd, run_fun};
use cocogitto::git::repository::Repository;

pub fn git_init_no_gpg() -> anyhow::Result<Repository> {
    run_cmd!(
        git init -b master;
        git config --local commit.gpgsign false;
    )?;

    Ok(Repository::open(".")?)
}

pub fn commit(message: &str) -> anyhow::Result<String> {
    Ok(run_fun!(
        git commit --allow-empty -q -m $message;
        git log --format=%H -n 1;
    )?)
}

pub fn git_tag(version: &str) -> anyhow::Result<()> {
    run_fun!(git tag $version;)?;
    Ok(())
}
