use super::Command;
use crate::{env::*, warning};
use clap::Args;
use console::style;

/// Display currently installed SDKs.
#[derive(Args)]
pub struct Ls {}

impl Command for Ls {
    fn run(&self) -> anyhow::Result<()> {
        let mut versions = list_installed_versions()?;
        if versions.is_empty() {
            warning!("There are no versions currently installed.");
            return Ok(());
        }

        versions.sort();

        let current = get_current_version()?;

        for v in versions {
            if let Some(c) = &current {
                if c == &v {
                    println!("{}", style(format!("* {v}")).green().bold());
                    continue;
                }
            }
            println!("  {v}");
        }

        Ok(())
    }
}
