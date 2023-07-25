use super::Command;
use crate::{
    env::{download::get_download_url, *},
    shell,
    tui::{print_status, print_success},
    versions::*,
};
use clap::Args;
use flate2::bufread::GzDecoder;
use std::io::BufReader;
use tar::Archive;
use zip::read::ZipArchive;

/// Install a version of Go.
#[derive(Args)]
#[command(visible_aliases = ["u", "select", "install"])]
pub struct Use {
    /// Specify a specific version or select the latest
    /// stable or unstable release.
    #[arg(default_value = "stable")]
    version: String,
}

impl Command for Use {
    fn run(&self) -> anyhow::Result<()> {
        check_env_applied(&shell::get_shell())?;

        let version = self.version.clone().to_lowercase();

        let version: Version = match version.to_lowercase().as_str() {
            "stable" => get_latest_upstream_version(false)?,
            "unstable" => get_latest_upstream_version(true)?,
            v => find_upstream_version(&v.parse()?)?,
        };

        let install_dir = get_version_installation_dir(&version)?;

        if !get_installed_versions()?.contains(&version) {
            ensure_dir(&install_dir)?;

            let dl_url = get_download_url(&version);

            print_status("Downloading SDK ...");
            let mut res = reqwest::blocking::get(get_download_url(&version))?;

            match get_url_extension(&dl_url) {
                "gz" | "tgz" => {
                    print_status("Unpacking SDK ...");
                    let mut arch = Archive::new(GzDecoder::new(BufReader::new(res)));
                    arch.unpack(&install_dir)?;
                }
                "zip" => {
                    let mut tmp = tempfile::tempfile()?;
                    res.copy_to(&mut tmp)?;
                    print_status("Unpacking SDK ...");
                    let mut arch = ZipArchive::new(tmp)?;
                    arch.extract(&install_dir)?;
                }
                _ => {}
            }
        }

        link_current_version(Some(&version))?;
        write_current_version(Some(&version))?;

        print_success(&format!("Switched to SDK version {version}!"));

        Ok(())
    }
}

fn get_url_extension(url: &str) -> &str {
    url.split('.').last().unwrap_or_default()
}
