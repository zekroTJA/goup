use super::*;
use crate::{
    env::{get_current_install_dir, get_home_dir},
    versions::Version,
};
use anyhow::Result;
use std::{
    fs::{self, File},
    io::{Read, Write},
    os::windows::fs::symlink_dir,
    path::PathBuf,
};

/// Returns all required environment variables.
pub fn get_env_vars() -> Result<String> {
    let path = std::env::var("PATH")?;

    let vars = vec![
        (
            "PATH",
            format!("{};{}", path_to_string(get_current_bin_dir()?), path),
        ),
        ("GOROOT", path_to_string(get_current_install_dir()?)),
    ];

    let lines: Vec<_> = vars
        .iter()
        .map(|(k, v)| format!("$env:{k} = \"{v}\""))
        .collect();

    Ok(lines.join("\n"))
}

/// Creates a symlink to the SDK installation dir of
/// the given [`Version`].
///
/// If [`Some(Version)`] is passed, the specified SDK
/// version directory is set.
/// If [`None`] is passed, the symlink is remoevd, if
/// existent.
pub fn link_current_version(v: Option<&Version>) -> Result<()> {
    let link = get_current_link_dir()?;

    match v {
        None => {
            if link.exists() {
                fs::remove_dir(&link)?;
            }
        }
        Some(v) => {
            let original = get_version_installation_dir(v)?;

            if link.exists() {
                fs::remove_dir(&link)?;
            }

            symlink_dir(original, link)?;
        }
    }

    Ok(())
}

/// Returns the path to the current users .zshenv file,
/// if it exists. Otherwise, the path to the .profile
/// file is returned.
pub fn get_profile_dir() -> Result<PathBuf> {
    get_home_dir().map(|p| {
        p.join("Documents")
            .join("WindowsPowerShell")
            .join("Microsoft.PowerShell_profile.ps1")
    })
}

fn path_to_string(p: PathBuf) -> String {
    p.to_string_lossy().to_string()
}
