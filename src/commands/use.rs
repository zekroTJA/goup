use std::io::BufReader;

use super::Command;
use crate::{
    env::{download::get_download_url, *},
    tui::{print_status, print_succes},
    versions::*,
};
use clap::Args;
use flate2::bufread::GzDecoder;
use tar::Archive;

/// Install a version of Go.
#[derive(Args)]
pub struct Use {
    /// Specify a specific version or select the latest
    /// stable or unstable release.
    #[arg(default_value = "stable")]
    pub version: String,
}

impl Command for Use {
    fn run(&self) -> anyhow::Result<()> {
        let version = self.version.clone().to_lowercase();

        let version: Version = match version.to_lowercase().as_str() {
            "stable" => get_latest_stable()?,
            "unstable" => get_latest_unstable()?,
            v => find_version(&v.parse()?)?,
        };

        let install_dir = get_version_installation_dir(&version)?;

        if !list_installed_versions()?.contains(&version) {
            ensure_dir(&install_dir)?;

            print_status("Downloading SDK ...");
            let res = reqwest::blocking::get(get_download_url(&version))?;
            let mut arch = Archive::new(GzDecoder::new(BufReader::new(res)));
            print_status("Unpacking SDK ...");
            arch.unpack(&install_dir)?;
        }

        link_current_version(&version)?;
        write_current_version(&version)?;

        print_succes(&format!("Switched to SDK version {version}!"));

        Ok(())
    }
}
