#![allow(dead_code)]
#![cfg(not(tarpaulin_include))]

use anyhow::Result;
use cocogitto::CONFIG_PATH;
use rand::Rng;
use std::process::{Command, Stdio};

pub fn git_init(path: &str) -> Result<()> {
    Command::new("git")
        .arg("init")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

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
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen();
    std::fs::write(random.to_string(), "dummy")?;

    println!("git add .");
    Command::new("git")
        .arg("add")
        .arg(".")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    println!("git commit -m \"{}\"", message);
    Command::new("git")
        .arg("commit")
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
