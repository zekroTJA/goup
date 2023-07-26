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
pub struct Check {}

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

        checkprint("pre-release", &current, new_pre);
        checkprint("minor", &current, new_minor);
        checkprint("patch", &current, new_patch);

        if new_pre.is_none() && new_minor.is_none() && new_patch.is_none() {
            success!("You are up to date with the latest upstream version!");
        }

        Ok(())
    }
}

fn checkprint(typ: &str, current: &Version, v: Option<&Version>) {
    if let Some(new) = v {
        success!("New {typ} version is available!");
        println!(
            "{} → {}",
            style(current.to_string()).dim(),
            style(new).cyan()
        );
    }
}
