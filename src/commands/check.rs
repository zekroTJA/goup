use std::env;

use super::Command;
use crate::{
    env::*,
    shell, success,
    versions::{self, get_upstream_versions, Version},
    warning,
};
use clap::Args;
use console::style;

/// Check for updates.
#[derive(Args)]
pub struct Check {
    #[arg(short, long)]
    /// Only print when updates are available;
    /// Designed to be used in profile file.
    notify: bool,
}

impl Command for Check {
    fn run(&self) -> anyhow::Result<()> {
        check_env_applied(&shell::get_shell())?;

        let Some(current) = get_current_version()? else {
            warning!("No version has been selected.\n\
                      Use `goup use` to select an SDK version.");
            return Ok(());
        };

        let upstream_versions = get_upstream_versions()?;

        let new_minor = versions::get_new_minor(&upstream_versions, &current);
        let new_patch = versions::get_new_patch(&upstream_versions, &current);
        let new_pre = versions::get_new_pre(&upstream_versions, &current);

        if new_pre.is_none() && new_minor.is_none() && new_patch.is_none() {
            if !self.notify {
                success!("You are up to date with the latest upstream version!");
            }
        } else {
            success!("New Go versions are available!");
            checkprint("pre-release", &current, new_pre);
            checkprint("minor", &current, new_minor);
            checkprint("patch", &current, new_patch);

            if self.notify {
                let binname = env::current_exe()
                    .ok()
                    .and_then(|pb| pb.file_name().map(|s| s.to_string_lossy().to_string()))
                    .unwrap_or("goup".into());
                success!("\nUse `{binname} use` to upgrade.");
            }
        }

        Ok(())
    }
}

fn checkprint(typ: &str, current: &Version, v: Option<&Version>) {
    if let Some(new) = v {
        println!(
            "{}:  {} → {}",
            style(typ).magenta(),
            style(current.to_string()).dim(),
            style(new).cyan()
        );
    }
}
