#![cfg(not(tarpaulin_include))]
use anyhow::Result;
use cocogitto::CONFIG_PATH;
use std::panic;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::TempDir;

pub struct TestContext {
    pub current_dir: PathBuf,
    pub test_dir: PathBuf,
}

// Save the current directory in the test context
// Change current dir to a temp directory
// Execute the test in context
// Reset temp directory
pub fn run_test_with_context<T>(test: T) -> Result<()>
where
    T: FnOnce(&TestContext) -> Result<()> + panic::UnwindSafe,
{
    let current_dir = std::env::current_dir()?;
    let temp_dir = TempDir::new()?;
    std::env::set_current_dir(&temp_dir)?;

    let context = TestContext {
        current_dir,
        test_dir: temp_dir.into_path(),
    };

    test(&context)?;

    Ok(std::env::set_current_dir(context.current_dir)?)
}

fn setup_git_config() -> Result<()> {
    Command::new("git")
        .arg("config")
        .arg("--local")
        .arg("user.name")
        .arg("Tom")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Unable to set local git user");

    Command::new("git")
        .arg("config")
        .arg("--local")
        .arg("user.email")
        .arg("toml.bombadil@themail.org")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Unable to set local git user email");

    std::fs::File::open(".git/config")?
        .sync_all()
        .expect("Error syncing `.git/config` file");

    Ok(())
}

pub fn git_init() -> Result<()> {
    Command::new("git")
        .arg("init")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Error initializing empty test repository in current directory");

    setup_git_config().expect("Error setting local git config");

    Ok(())
}

pub fn git_init_and_set_current_path(path: &str) -> Result<()> {
    Command::new("git")
        .arg("init")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .expect("Error creating empty repo with target path");

    std::fs::File::open(path)
        .expect("Error opening test repository")
        .sync_all()
        .expect("Error syncing file system while creating test repository");

    let repo_dir = std::env::current_dir()
        .expect("Unable to get current directory in test context")
        .join(path);

    std::env::set_current_dir(repo_dir).expect("Unable to move into to test repository");

    setup_git_config().expect("Error setting up local git config for test");

    Ok(())
}

pub fn git_log() -> Result<()> {
    Command::new("git")
        .arg("log")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

pub fn git_add() -> Result<()> {
    Command::new("git")
        .arg("add")
        .arg(".")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

pub fn git_commit(message: &str) -> Result<()> {
    Command::new("git")
        .arg("add")
        .arg(".")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    println!("git commit -m \"{}\"", message);
    Command::new("git")
        .arg("commit")
        .arg("--allow-empty")
        .arg("-m")
        .arg(message)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

pub fn git_tag(tag: &str) -> Result<()> {
    Command::new("git")
        .arg("tag")
        .arg(tag)
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

pub fn assert_tag(tag: &str) -> Result<()> {
    let out = Command::new("ls").arg(".git/refs/tags").output()?.stdout;

    let out = String::from_utf8(out)?;
    let tags: Vec<&str> = out.split('\n').collect();
    assert!(tags.contains(&tag));
    Ok(())
}

pub fn git_log_head() -> Result<String> {
    let out = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=%B")
        .output()?;

    let head = String::from_utf8_lossy(&out.stdout).to_string();

    Ok(head)
}

pub fn git_status() -> Result<String> {
    let out = Command::new("git").arg("status").output()?;

    let head = String::from_utf8_lossy(&out.stdout).to_string();

    Ok(head)
}

pub fn get_git_user_name() -> Result<String> {
    let username = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()?
        .stdout;

    Ok(String::from_utf8(username)?.trim_end().to_string())
}

pub fn create_empty_config() -> Result<()> {
    std::fs::File::create(CONFIG_PATH)?;
    Ok(())
}
