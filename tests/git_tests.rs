use anyhow::Result;
use chrono::Utc;
use cocogitto::CocoGitto;
use std::process::{Command, Stdio};
use std::time::Duration;

fn git_init() -> Result<()> {
    let temp_dir = temp_testdir::TempDir::default().permanent();
    println!("{:?}", &temp_dir.as_ref());
    std::env::set_current_dir(temp_dir.as_ref())?;

    println!("on {:?}", std::env::current_dir()?);
    println!("git init");

    Command::new("git")
        .arg("init")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

fn git_commit(message: &str) -> Result<()> {
    std::thread::sleep(Duration::from_millis(100));
    let file_name = Utc::now().timestamp().to_string();
    std::fs::write(file_name, "dummy")?;

    println!("git add .");
    Command::new("git")
        .arg("add")
        .arg(".")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    println!("git commit -m \"{}\"", message);
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&format!("\"{}\"", message))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

fn _git_tag(tag: &str) -> Result<()> {
    Command::new("tag")
        .arg(tag)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

fn create_empty_config() -> Result<()> {
    std::fs::File::create("coco.toml")?;
    Ok(())
}

#[test]
fn should_open_repo() -> Result<()> {
    git_init()?;
    create_empty_config()?;

    let gitto = CocoGitto::get();

    assert!(gitto.is_ok());
    Ok(())
}

#[test]
fn should_check_commit_history_ok() -> Result<()> {
    git_init()?;
    create_empty_config()?;
    git_commit("feat: a valid commit")?;
    git_commit("chore(test): another valid commit")?;

    let gitto = CocoGitto::get()?;
    assert!(gitto.check().is_ok());
    Ok(())
}

// check stderr and std out instead of exit status
// #[test]
// fn should_check_commit_history_err() -> Result<()> {
//     git_init()?;
//     create_empty_config()?;
//     git_commit("feat: a valid commit")?;
//     git_commit("errored commit")?;
//
//     let gitto = CocoGitto::get()?;
//
//     assert!(gitto.check().is_err());
//     Ok(())
// }
