use std::io::Result as IoResult;
use std::path::Path;

use clap::{Command, CommandFactory};
use clap_mangen::Man;

use crate::Cli;

pub fn generate_manpages(out_dir: &Path) -> IoResult<()> {
    std::fs::create_dir_all(out_dir)?;

    let cog = Cli::command();

    for mut subcmd in cog.get_subcommands().filter(|c| !c.is_hide_set()).cloned() {
        let name = subcmd.get_name();
        let full_name = format!("cog-{}", name);
        let man_name = format!("{}.1", full_name);

        subcmd = subcmd.name(&full_name);

        render_command(&out_dir.join(&man_name), subcmd)?;
    }

    render_command(&out_dir.join("cog.1"), cog)?;

    Ok(())
}

fn render_command(file: &Path, cmd: Command) -> IoResult<()> {
    let man = Man::new(cmd);
    let mut buffer = Vec::new();

    man.render(&mut buffer)?;
    std::fs::write(file, buffer)?;

    Ok(())
}
