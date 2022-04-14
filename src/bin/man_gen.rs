use clap::CommandFactory;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let man_pages_dir = &PathBuf::from("cocogitto_man_pages");

    if man_pages_dir.exists() {
        fs::remove_dir_all(man_pages_dir)?;
    }

    fs::create_dir(man_pages_dir)?;

    let mut command = cocogitto::cli::Cli::command();
    let cmd_name = command.get_name().to_string();

    for subcommand in command.get_subcommands_mut() {
        let name = subcommand.get_name().clone();
        let name = format!("{}-{}", cmd_name, name);
        let sub = subcommand.clone().name(&name);
        let man = clap_mangen::Man::new(sub);
        let mut buffer: Vec<u8> = Default::default();
        man.render(&mut buffer)?;
        std::fs::write(man_pages_dir.join(&format!("{}.1", name)), buffer)?;
    }

    let man = clap_mangen::Man::new(command);

    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_pages_dir.join(&format!("{}.1", cmd_name)), buffer)?;

    Ok(())
}
