#![cfg_attr(debug_assertions, allow(dead_code))]
use anyhow::Result;
use chrono::Utc;
use std::process::{Command, Stdio};

pub fn git_init() -> Result<()> {
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

pub fn git_commit(message: &str) -> Result<()> {
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

pub fn _git_tag(tag: &str) -> Result<()> {
    Command::new("tag")
        .arg(tag)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

pub fn create_empty_config() -> Result<()> {
    std::fs::File::create("coco.toml")?;
    Ok(())
}
