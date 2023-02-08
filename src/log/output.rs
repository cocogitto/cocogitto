/*
This file is based on the work of the developers of `bat` and `delta` projects.

Copyright (c) 2020 Dan Davison (https://github.com/dandavison/delta).
Copyright (c) 2018 bat-developers (https://github.com/sharkdp/bat).

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use crate::Result;

use anyhow::{anyhow, Context};

#[derive(Debug)]
pub enum Output {
    Pager(Child),
    Stdout(io::Stdout),
}

#[derive(Clone, Debug)]
pub struct OutputBuilder {
    pager_cmd: Option<String>,
    file_name: Option<String>,
}

impl Output {
    pub fn builder() -> OutputBuilder {
        OutputBuilder::new()
    }

    pub fn stdout() -> Self {
        Output::Stdout(io::stdout())
    }

    pub fn handle(&mut self) -> Result<&mut dyn Write> {
        Ok(match self {
            Output::Pager(process) => process
                .stdin
                .as_mut()
                .ok_or_else(|| anyhow!("Could not open pager stdin"))?,
            Output::Stdout(stdout) => stdout,
        })
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        if let Output::Pager(process) = self {
            process.wait().ok();
        }
    }
}

impl OutputBuilder {
    pub(crate) fn new() -> Self {
        OutputBuilder {
            pager_cmd: None,
            file_name: None,
        }
    }

    /// Try to get path to pager from the given environment variable
    /// If this method is called several times, the last existing environment variable will be used
    #[must_use]
    pub fn with_pager_from_env(self, env_key: &str) -> Self {
        let pager_cmd = std::env::var(env_key).ok();

        OutputBuilder {
            pager_cmd: pager_cmd.or(self.pager_cmd),
            ..self
        }
    }

    /// Specify the file name to be displayed in the pager header (corresponds to bat `--file-name` parameter)
    #[must_use]
    pub fn with_file_name(self, file_name: impl Into<String>) -> Self {
        OutputBuilder {
            file_name: Some(file_name.into()),
            ..self
        }
    }

    /// Try to construct an output. If no pager was configured, defaults to 'bat'
    /// If 'bat' is not available, defaults to 'less'
    /// If no pager is available, defaults to plain stdout
    pub fn build(self) -> Result<Output> {
        let bat = which::which("bat").ok();
        let less = which::which("less").ok();

        if self.pager_cmd.is_none() && bat.is_none() && less.is_none() {
            return Ok(Output::stdout());
        }

        let (pager, args) = self
            .specified_pager()?
            .or_else(|| bat.or(less).map(|def| (def, vec![])))
            .expect(
                "must have a value, as ensured by checking existence of at least one default pager",
            );

        let mut cmd = self.make_pager_command(&pager, &args);
        let output = cmd
            .spawn()
            .map(Output::Pager)
            .unwrap_or_else(|_| Output::stdout());

        Ok(output)
    }

    /// Separate pager_cmd into command and arguments, and ensure the command exists
    fn specified_pager(&self) -> Result<Option<(PathBuf, Vec<String>)>> {
        if let Some(pager_cmd) = self.pager_cmd.as_ref() {
            let mut words = shell_words::split(pager_cmd)?.into_iter();
            let pager = words
                .next()
                .ok_or_else(|| anyhow!("Pager command must not be empty"))?;

            // Check that pager binary file exists or try to find it in PATH
            let pager = if AsRef::<Path>::as_ref(&pager).exists() {
                PathBuf::from(pager)
            } else {
                which::which(&pager).context(format!("Cannot find pager `{pager}`"))?
            };

            let args = words.collect();

            Ok(Some((pager, args)))
        } else {
            Ok(None)
        }
    }

    /// Setup Command to run the pager
    fn make_pager_command(&self, pager: &Path, args: &[String]) -> Command {
        let mut cmd = Command::new(pager);

        let is_less = pager.file_stem() == Some(&OsString::from("less"));
        let is_bat = pager.file_stem() == Some(&OsString::from("bat"));

        if is_less {
            let has_r_flag = args
                .iter()
                .any(|arg| arg == "-R" || arg == "--RAW-CONTROL-CHARS");

            let has_no_init_flag = args.iter().any(|arg| arg == "--no-init");

            if !has_r_flag {
                cmd.arg("--RAW-CONTROL-CHARS");
            }

            // Passing '--no-init' fixes a bug with '--quit-if-one-screen' in older
            // versions of 'less'. Unfortunately, it also breaks mouse-wheel support.
            //
            // See: http://www.greenwoodsoftware.com/less/news.530.html
            //
            // For newer versions (530 or 558 on Windows), we omit '--no-init' as it
            // is not needed anymore.
            if !has_no_init_flag {
                match less_version() {
                    None => {
                        cmd.arg("--no-init");
                    }
                    Some(version) if (version < 530 || (cfg!(windows) && version < 558)) => {
                        cmd.arg("--no-init");
                    }
                    _ => {}
                }
            }

            cmd.env("LESSCHARSET", "UTF-8");
        }

        if is_bat {
            self.file_name
                .as_ref()
                .map(|name| cmd.args(["--file-name", name]));
        }

        cmd.args(args)
            .env("LESSANSIENDCHARS", "mK")
            .stdin(Stdio::piped());

        cmd
    }
}

fn less_version() -> Option<usize> {
    let cmd = Command::new("less").arg("--version").output().ok()?;
    parse_less_version(&cmd.stdout)
}

fn parse_less_version(output: &[u8]) -> Option<usize> {
    if output.starts_with(b"less ") {
        let version = std::str::from_utf8(&output[5..]).ok()?;
        let end = version.find(' ')?;
        version[..end].parse::<usize>().ok()
    } else {
        None
    }
}
