use crate::{env::*, progress::Spinner, tui::*};

use super::Command;
use clap::Args;

/// Remove all installed SDKs.
#[derive(Args)]
#[command(visible_aliases = ["purge", "prune"])]
pub struct Clean {
    /// Clean up **all** installed SDK versions
    #[arg(short, long)]
    all: bool,
}

impl Command for Clean {
    fn run(&self) -> anyhow::Result<()> {
        Spinner::new("Removing SDKs ...");

        if self.all {
            link_current_version(None)?;
            write_current_version(None)?;
            drop_install_dir()?;
        } else {
            let versions = get_installed_versions()?;
            let curr = get_current_version()?;
            let errs: Vec<_> = versions
                .iter()
                .filter(|v| Some(*v) != curr.as_ref())
                .map(|v| (v, drop_version(v)))
                .filter(|(_, r)| r.is_err())
                .map(|(v, r)| format!("- {}: {}", v, r.unwrap_err()))
                .collect();

            if !errs.is_empty() {
                anyhow::bail!(
                    "Failed removing the following versions:\n{}",
                    errs.join("\n")
                )
            }
        }

        print_success("SDKs have been cleaned up.");

        Ok(())
    }
}
