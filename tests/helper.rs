use anyhow::Result;
use std::process::{Command, Stdio};
use rand::Rng;

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
pub fn git_add() -> Result<()> {
    Command::new("git")
        .arg("add")
        .arg(".")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}


#[allow(dead_code)]
pub fn git_commit(message: &str) -> Result<()> {
    let mut rng = rand::thread_rng();
    let random: f64 = rng.gen();
    std::fs::write(random.to_string(), "dummy")?;

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
    Command::new("git")
        .arg("tag")
        .arg(tag)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

#[allow(dead_code)]
pub fn assert_tag(tag: &str) -> Result<()> {
    let out = Command::new("ls")
        .arg(".git/refs/tags")
        .output()?.stdout;

    let out = String::from_utf8(out)?;
    let tags: Vec<&str> = out.split('\n').collect();
    assert!(tags.contains(&tag));
    Ok(())
}

#[allow(dead_code)]
pub fn create_empty_config() -> Result<()> {
    std::fs::File::create("coco.toml")?;
    Ok(())
}
