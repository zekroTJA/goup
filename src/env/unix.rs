use super::*;
use crate::{
    env::{get_current_install_dir, get_home_dir},
    versions::Version,
};
use anyhow::Result;
use std::{
    fs::{self, File},
    io::{Read, Write},
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

/// Reads and returns the content of the profile file in
/// the current users `$HOME` directory.
///
/// See [`get_profile_dir`] for more information.
pub fn read_profile() -> Result<String> {
    let profile_dir = get_profile_dir()?;

    if !profile_dir.exists() {
        anyhow::bail!("profile file does not exist in your home directory.");
    }

    let mut f = File::open(profile_dir)?;
    let mut res = String::new();
    f.read_to_string(&mut res)?;

    Ok(res)
}

/// Appends the given `content` to the profile file in
/// the current users `$HOME` directory.
///
/// See [`get_profile_dir`] for more information.
pub fn append_to_profile(content: &str) -> Result<()> {
    let profile_dir = get_profile_dir()?;

    if !profile_dir.exists() {
        anyhow::bail!("profile file does not exist in your home directory.");
    }

    let mut f = File::options().append(true).open(profile_dir)?;
    f.write_all(content.as_bytes())?;

    Ok(())
}

fn path_to_string(p: PathBuf) -> String {
    p.to_string_lossy().to_string()
}
