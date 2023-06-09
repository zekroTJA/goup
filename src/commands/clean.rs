use crate::{env::*, tui::*};

use super::Command;
use clap::Args;

/// Remove all installed SDKs.
#[derive(Args)]
#[command(visible_aliases = ["purge", "prune"])]
pub struct Clean {
    /// Run cleanup without prompt
    #[arg(short, long)]
    yes: bool,
}

impl Command for Clean {
    fn run(&self) -> anyhow::Result<()> {
        if !self.yes && !accept("Do you really want to delete all installed SDKs?", false)? {
            print_note("Aborted.");
            return Ok(());
        }

        print_status("Removing SDKs ...");

        link_current_version(None)?;
        write_current_version(None)?;
        drop_install_dir()?;

        print_success("All SDKs have been removed.");

        Ok(())
    }
}
