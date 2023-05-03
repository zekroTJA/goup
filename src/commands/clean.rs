use crate::{
    env::{drop_install_dir, write_current_version},
    tui::{accept, print_note, print_status, print_succes},
};

use super::Command;
use clap::Args;

/// Print env variables required to use goup.
#[derive(Args)]
pub struct Clean {}

impl Command for Clean {
    fn run(&self) -> anyhow::Result<()> {
        if !accept("Do you really want to delete all installed SDKs?", false)? {
            print_note("Aborted.");
            return Ok(());
        }

        print_status("Removing SDKs ...");

        write_current_version(None)?;
        drop_install_dir()?;

        print_succes("All SDKs have been removed.");

        Ok(())
    }
}
