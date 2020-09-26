use anyhow::Result;
use chrono::Utc;
use std::process::{Command, Stdio};

// Why those are picked as dead code by rustc ?

#[allow(dead_code)]
pub fn git_init(path: &str) -> Result<()> {
    Command::new("git")
        .arg("init")
        .arg(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

#[allow(dead_code)]
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
        .arg(&format!("{}", message))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

#[allow(dead_code)]
pub fn git_tag(tag: &str) -> Result<()> {
    Command::new("tag")
        .arg(tag)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

#[allow(dead_code)]
pub fn create_empty_config() -> Result<()> {
    std::fs::File::create("coco.toml")?;
    Ok(())
}
