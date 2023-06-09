use super::Command;
use crate::{
    env::*,
    success,
    versions::{get_upstream_versions, Version, VersionPart},
    warning,
};
use clap::Args;
use console::style;

/// Check for updates.
#[derive(Args)]
pub struct Check {}

impl Command for Check {
    fn run(&self) -> anyhow::Result<()> {
        check_env_applied()?;

        let Some(current) = get_current_version()? else {
            warning!("No version has been selected.\n\
                      Use `goup use` to select an SDK version.");
            return Ok(());
        };

        let upstream_versions = get_upstream_versions()?;

        let new_minor = upstream_versions
            .iter()
            .rev()
            .find(|v| v.minor > current.minor && current.strip_after(VersionPart::Major).covers(v));

        let new_patch = upstream_versions
            .iter()
            .rev()
            .find(|v| v.patch > current.patch && current.strip_after(VersionPart::Minor).covers(v));

        let mut new_pre = None;
        if !current.is_stable() {
            new_pre = upstream_versions
                .iter()
                .rev()
                .find(|v| v.pre > current.pre && current.strip_after(VersionPart::Patch).covers(v));
        }

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
