#![cfg(not(tarpaulin_include))]

use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;
use cmd_lib::{init_builtin_logger, run_cmd, run_fun};
use speculoos::assert_that;
use speculoos::iter::ContainingIntoIterAssertions;

use cocogitto::CONFIG_PATH;

pub fn git_init() -> Result<()> {
    init_builtin_logger();
    run_cmd!(
        git init;
        git config --local user.name Tom;
        git config --local user.email toml.bombadil@themail.org;
    )?;

    Ok(())
}

pub fn git_init_and_set_current_path(path: &str) -> Result<()> {
    init_builtin_logger();
    run_cmd!(
        git init $path;
        git config --local user.name Tom;
        git config --local user.email toml.bombadil@themail.org;
    )?;

    std::env::set_current_dir(path).expect("Unable to move into to test repository");

    Ok(())
}

pub fn git_status() -> Result<String> {
    run_fun!(git status).map_err(|e| anyhow!(e))
}

pub fn git_add<S: AsRef<Path>>(content: &str, path: S) -> Result<()>
where
    S: ToString,
{
    run_cmd!(
        echo $content > $path;
        git add $path;
    )
    .map_err(|e| anyhow!(e))
}

pub fn git_commit(message: &str) -> Result<String> {
    run_fun!(
        git commit --allow-empty -q -m $message;
        git log --format=%H -n 1;
    )
    .map_err(|e| anyhow!(e))
}

pub fn git_tag(tag: &str) -> Result<()> {
    run_cmd!(
        git tag $tag;
    )
    .map_err(|e| anyhow!(e))
}

pub fn assert_tag(tag: &str) -> Result<()> {
    let tags = run_fun!(git --no_pager tag)?;
    let tags: Vec<&str> = tags.split('\n').collect();
    assert_that!(tags).contains(&tag);
    Ok(())
}

pub fn git_log_head() -> Result<String> {
    run_fun!(git log -1 --pretty=%B).map_err(|e| anyhow!(e))
}

pub fn create_empty_config() -> Result<()> {
    std::fs::File::create(CONFIG_PATH)?;
    Ok(())
}
