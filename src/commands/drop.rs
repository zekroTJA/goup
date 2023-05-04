use super::Command;
use crate::{env::*, tui::*, versions::Version};
use clap::Args;
use console::style;

/// Drop an installed SDK.
#[derive(Args)]
pub struct Drop {
    /// The version which should be dropped.
    pub version: String,
}

impl Command for Drop {
    fn run(&self) -> anyhow::Result<()> {
        let target: Version = self.version.parse()?;

        let versions = get_installed_versions()?;
        let versions: Vec<_> = versions.iter().filter(|v| target.covers(v)).collect();

        if versions.is_empty() {
            anyhow::bail!("No SDK found matching the given version.");
        }

        if versions.len() > 1 {
            let v: Vec<_> = versions.iter().map(|v| v.to_string()).collect();
            anyhow::bail!(
                "The given version matches multiple SDKs. Please supply a less ambigious version.\n\n{}\n{}",
                style("Matching versions:").underlined(),
                style(v.join("\n")).red()
            );
        }

        let target = versions[0];

        let current = get_current_version()?;
        let is_current = matches!(current, Some(c) if &c == target);
        if is_current
            && !accept(
                "The selected version is the currently used SDK. Do you still want to drop it?",
                false,
            )?
        {
            print_note("Aborted.");
            return Ok(());
        }

        if is_current {
            link_current_version(None)?;
            write_current_version(None)?;
        }

        print_status("Removing SDK ...");
        drop_version(target)?;

        print_success("SDK has been removed.");

        Ok(())
    }
}
