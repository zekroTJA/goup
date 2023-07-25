use super::Command;
use crate::{
    cmd::{self, exec},
    env::*,
    error, shell, warning,
};
use clap::Args;

/// Display the currently selected version of Go.
#[derive(Args)]
#[command(visible_aliases = ["c"])]
pub struct Current;

impl Command for Current {
    fn run(&self) -> anyhow::Result<()> {
        check_env_applied(&shell::get_shell())?;

        if let Some(v) = get_current_version()? {
            println!("{v}");
            return Ok(());
        }

        warning!("No version installed via goup");
        match exec(&["go", "version"]) {
            Ok(v) => println!("from system: {v}"),
            Err(err) if matches!(err, cmd::errors::Error::NotFound) => {
                error!("no local go version found");
            }
            Err(err) => return Err(err.into()),
        }

        Ok(())
    }
}
