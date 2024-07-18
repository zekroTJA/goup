use super::Command;
use crate::{
    env::{download::get_download_url, *},
    progress, shell,
    tui::{print_status, print_success},
    versions::*,
};
use clap::Args;
use flate2::bufread::GzDecoder;
use indicatif::ProgressDrawTarget;
use std::io::{self, BufReader};
use tar::Archive;
use zip::read::ZipArchive;

/// Install a version of Go.
#[derive(Args)]
#[command(visible_aliases = ["u", "up", "select", "install"])]
pub struct Use {
    /// Specify a specific version or select the latest
    /// stable or unstable release.
    version: Option<String>,
}

impl Command for Use {
    fn run(&self) -> anyhow::Result<()> {
        check_env_applied(&shell::get_shell())?;

        let version_inpt = self.version.as_ref().map(|v| v.to_lowercase());
        let version: Version = match version_inpt.as_deref() {
            Some("stable") | Some("latest") | Some("s") => get_latest_upstream_version(false)?,
            Some("unstable") | Some("rc") => get_latest_upstream_version(true)?,
            Some(v) => find_upstream_version(&v.parse()?)?,
            None => {
                let current = get_current_version()?;
                get_latest_upstream_version(current.is_some_and(|c| !c.is_stable()))?
            }
        };

        let install_dir = get_version_installation_dir(&version)?;

        if !get_installed_versions()?.contains(&version) {
            ensure_dir(&install_dir)?;

            let dl_url = get_download_url(&version);

            let res = reqwest::blocking::get(get_download_url(&version))?;
            let mut reader =
                progress::Reader::new(res.content_length(), res, ProgressDrawTarget::stdout());

            match get_url_extension(&dl_url) {
                "gz" | "tgz" => {
                    let mut arch = Archive::new(GzDecoder::new(BufReader::new(reader)));
                    arch.unpack(&install_dir)?;
                }
                "zip" => {
                    let mut tmp = tempfile::tempfile()?;
                    io::copy(&mut reader, &mut tmp)?;
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
