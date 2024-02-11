use std::{
    io::{self, Write},
    process::{self, Stdio},
};

use clap::Args;

use crate::{env::get_env_vars, shell, Command};

/// Display the currently selected version of Go.
#[derive(Args)]
#[command(visible_aliases = ["e", "run"])]
pub struct Exec {
    args: Vec<String>,
}

impl Command for Exec {
    fn run(&self) -> anyhow::Result<()> {
        let shell = shell::get_shell();

        let env_vars = get_env_vars(&shell)?;

        let out = process::Command::new("go")
            .envs(env_vars)
            .args(&self.args)
            .output()?;

        io::stdout().write_all(&out.stdout)?;

        Ok(())
    }
}
