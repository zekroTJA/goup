use super::*;
use crate::{
    env::{get_current_install_dir, get_home_dir},
    versions::Version,
};
use anyhow::Result;
use std::{
    fs,
    os::unix::fs::symlink,
    path::PathBuf,
};

/// Returns all required environment variables.
pub fn get_env_vars() -> Result<String> {
    let vars = vec![
        (
            "PATH",
            format!("{}:$PATH", path_to_string(get_current_bin_dir()?)),
        ),
        ("GOROOT", path_to_string(get_current_install_dir()?)),
    ];

    let lines: Vec<_> = vars
        .iter()
        .map(|(k, v)| format!("export {k}=\"{v}\""))
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
                fs::remove_file(&link)?;
            }
        }
        Some(v) => {
            let original = get_version_installation_dir(v)?;

            if link.exists() {
                fs::remove_file(&link)?;
            }

            symlink(original, link)?;
        }
    }

    Ok(())
}

/// Returns the path to the current users .zshenv file,
/// if it exists. Otherwise, the path to the .profile
/// file is returned.
pub fn get_profile_dir() -> Result<PathBuf> {
    let home = get_home_dir()?;

    let zshenv = home.join(".zshenv");
    if zshenv.exists() {
        Ok(zshenv)
    } else {
        Ok(home.join(".profile"))
    }
}

fn path_to_string(p: PathBuf) -> String {
    p.to_string_lossy().to_string()
}
