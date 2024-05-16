use cmd_lib::{run_cmd, run_fun};
use cocogitto::git::repository::Repository;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use anyhow::Result;
use cocogitto_config::monorepo::MonoRepoPackage;
use cocogitto_config::{Settings, CONFIG_PATH};
use speculoos::assert_that;
use speculoos::iter::ContainingIntoIterAssertions;
use speculoos::option::OptionAssertions;

use cocogitto::git::tag::Tag;

/// Init a new repository on branch master and disable gpg signing
pub fn git_init_no_gpg() -> anyhow::Result<Repository> {
    run_cmd!(
        git init -b master;
        git config --local commit.gpgsign false;
    )?;

    Ok(Repository::open(".")?)
}

/// TODO: Remove this (duplicates with `git_commit`)
pub fn commit(message: &str) -> anyhow::Result<String> {
    Ok(run_fun!(
        git commit --allow-empty -q -m $message;
        git log --format=%H -n 1;
    )?)
}

/// Init a repository with a cog.toml fixture containing a single package named 'one'
pub fn init_monorepo(settings: &mut Settings) -> Result<()> {
    let mut packages = HashMap::new();
    packages.insert(
        "one".to_string(),
        MonoRepoPackage {
            path: PathBuf::from("one"),
            ..Default::default()
        },
    );
    settings.packages = packages;
    let settings = toml::to_string(&settings)?;

    git_init()?;
    run_cmd!(
        echo $settings > cog.toml;
        git add .;
        git commit -m "chore: first commit";
        mkdir one;
        echo "changes" > one/file;
        git add .;
        git commit -m "feat: package one feature";
    )?;

    Ok(())
}

/// - Init a repository in the current directory
/// - Setup a local git user named Tom <toml.bombadil@themail.org>
pub fn git_init() -> Result<()> {
    run_cmd!(
        git init -b master;
        git config --local commit.gpgsign false;
        git config --local user.name Tom;
        git config --local user.email toml.bombadil@themail.org;
    )?;

    Ok(())
}

/// - Init a repository in the given path
/// - Change the current directory to the newly created repository
/// - Setup a local git user named Tom <toml.bombadil@themail.org>
pub fn git_init_and_set_current_path(path: &str) -> Result<()> {
    run_cmd!(
        git init $path;
        cd $path;
        git config --local commit.gpgsign false;
    )?;

    std::env::set_current_dir(path).expect("U nable to move into to test repository");

    Ok(())
}

/// Can be used to make assertion on 'git status' output.
pub fn git_status() -> Result<String> {
    run_fun!(git status).map_err(|e| anyhow!(e))
}

/// Write the given content to the provided path and add it to the git index
pub fn git_add<S>(content: &str, path: S) -> Result<()>
where
    S: ToString + AsRef<Path>,
{
    let path = path.to_string();
    run_cmd!(
        echo "writing $content to $path";
        echo $content > $path;
        git add $path;
    )
    .map_err(|e| anyhow!(e))
}

/// Create an empty git commit and return its sha1.
/// Note that empty commits allowed
pub fn git_commit(message: &str) -> Result<String> {
    run_fun!(
        git commit --allow-empty -q -m $message;
        git log --format=%H -n 1;
    )
    .map_err(|e| anyhow!(e))
}

/// Create a git tag on the current repository
pub fn git_tag(tag: &str) -> Result<()> {
    run_cmd!(
        git tag $tag;
    )
    .map_err(|e| anyhow!(e))
}

/// Assert a tag exists in the repository
pub fn assert_tag_exists(tag: &str) -> Result<()> {
    let tags = run_fun!(git --no-pager tag)?;
    let tags: Vec<&str> = tags.split('\n').collect();
    assert_that!(tags).contains(tag);
    Ok(())
}

/// Assert a tag does not exist in the repository
pub fn assert_tag_does_not_exist(tag: &str) -> Result<()> {
    let tags = run_fun!(git --no-pager tag)?;
    let tags: Vec<&str> = tags.split('\n').collect();
    assert_that!(tags).does_not_contain(tag);
    Ok(())
}

/// Assert the latest tag in the repository is the one provided
pub fn assert_latest_tag(tag: &str) -> Result<()> {
    let tags = run_fun!(git --no-pager tag)?;
    let tag = Tag::from_str(tag, None, None)?;
    let mut tags: Vec<Tag> = tags
        .split('\n')
        .filter_map(|tag| Tag::from_str(tag, None, None).ok())
        .collect();

    tags.sort();

    assert_that!(tags.last()).is_some().is_equal_to(&tag);
    Ok(())
}

/// Ensure the given tag is an annotated annotated tag
pub fn assert_tag_is_annotated(tag: &str) -> Result<()> {
    let objtype = run_fun!(git for-each-ref --format="%(objecttype)" refs/tags/$tag)?;
    let objtype: Vec<&str> = objtype.split('\n').collect();
    assert_that!(objtype.first()).is_some().is_equal_to(&"tag");
    Ok(())
}

/// Git log showing only the HEAD commit message, this can be used to make assertion on the last commit
pub fn git_log_head_message() -> Result<String> {
    run_fun!(git log -1 --pretty=%B).map_err(|e| anyhow!(e))
}

/// Git log showing only the HEAD commit sha, this can be used to make assertion on the last commit
pub fn git_log_head_sha() -> Result<String> {
    run_fun!(git log -1 --pretty=%H).map_err(|e| anyhow!(e))
}

/// Create an empty `cog.toml` config file in the current directory
pub fn create_empty_config() -> Result<()> {
    std::fs::File::create(CONFIG_PATH)?;
    Ok(())
}
