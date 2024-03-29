use super::Command;
use crate::{env::*, shell, warning};
use clap::Args;
use console::style;

/// Display currently installed SDKs.
#[derive(Args)]
#[command(visible_aliases = ["list"])]
pub struct Ls {}

impl Command for Ls {
    fn run(&self) -> anyhow::Result<()> {
        check_env_applied(&shell::get_shell())?;

        let mut versions = get_installed_versions()?;
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
