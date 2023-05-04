use crate::versions::Version;
use anyhow::Result;
use directories::UserDirs;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

const CURRENT_VERSION_FILE: &str = ".current_version";

/// Returns the current users home directory.
///
/// # Example
/// ```
/// let dir = get_home_dir().unwrap();
/// // -> "/home/me"
/// ```
pub fn get_home_dir() -> Result<PathBuf> {
    UserDirs::new()
        .ok_or_else(|| anyhow::anyhow!("could not find user directories"))
        .map(|dirs| dirs.home_dir().to_path_buf())
}

/// Returns the current working directory.
///
/// This directory contains all status files,
/// installed SDKs and the symlink to the
/// currently selected SDK.
///
/// # Example
/// ```
/// let dir = get_work_dir().unwrap();
/// // -> "/home/me/.local/goup"
/// ```
pub fn get_work_dir() -> Result<PathBuf> {
    get_home_dir().map(|dir| dir.join(".local").join("goup"))
}

/// Returns the SDK installations dirrectory.
///
/// This directory contains all installed
/// SDK versions.
///
/// # Example
/// ```
/// let dir = get_installations_dir().unwrap();
/// // -> "/home/me/.local/goup/installations"
/// ```
pub fn get_installations_dir() -> Result<PathBuf> {
    get_work_dir().map(|dir| dir.join("installations"))
}

/// Returns the symlink directory pointing to
/// the currently selected SDK version.
///
/// # Example
/// ```
/// let dir = get_current_link_dir().unwrap();
/// // -> "/home/me/.local/goup/current"
/// ```
pub fn get_current_link_dir() -> Result<PathBuf> {
    get_work_dir().map(|dir| dir.join("current"))
}

/// Returns the directory to the currently selected
/// SDK files.
///
/// # Example
/// ```
/// let dir = get_current_install_dir().unwrap();
/// // -> "/home/me/.local/goup/current/go"
/// ```
pub fn get_current_install_dir() -> Result<PathBuf> {
    get_current_link_dir().map(|dir| dir.join("go"))
}

/// Returns the directory to the currently selected
/// SDK binary files.
///
/// # Example
/// ```
/// let dir = get_current_bin_dir().unwrap();
/// // -> "/home/me/.local/goup/current/go/bin"
/// ```
pub fn get_current_bin_dir() -> Result<PathBuf> {
    get_current_install_dir().map(|dir| dir.join("bin"))
}

/// Returns the directory to an installed SDK
/// by the given [`Version`].
///
/// # Example
/// ```
/// let version: Version = "1.20.4".parse().unwrap();
/// let dir = get_version_installation_dir(&version).unwrap();
/// // -> "/home/me/.local/goup/installations/1.20.4"
/// ```
pub fn get_version_installation_dir(version: &Version) -> Result<PathBuf> {
    get_installations_dir().map(|v| v.join(version.to_string()))
}

/// Checks if the passed directory exists and
/// tries to create it if it does not exist.
pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Tries to read the currently selected [`Version`] from
/// the workdir location.
///
/// If no version has been selected, [`None`] is returend.
pub fn get_current_version() -> Result<Option<Version>> {
    let vfile_path = get_work_dir()?.join(CURRENT_VERSION_FILE);
    let mut vfile = match File::open(vfile_path) {
        Ok(f) => f,
        Err(err) => {
            if matches!(err.kind(), std::io::ErrorKind::NotFound) {
                return Ok(None);
            } else {
                return Err(err.into());
            }
        }
    };

    let mut ver = String::new();
    vfile.read_to_string(&mut ver)?;

    let v = ver.parse()?;
    Ok(Some(v))
}

/// Returns a list of all installed SDK versions.
pub fn get_installed_versions() -> Result<Vec<Version>> {
    let dir = match get_installations_dir()?.read_dir() {
        Ok(v) => v,
        Err(err) if matches!(err.kind(), io::ErrorKind::NotFound) => return Ok(vec![]),
        Err(err) => return Err(err.into()),
    };

    let dir: Result<Vec<_>, _> = dir.collect();
    let versions: Result<Vec<Version>, _> = dir?
        .iter()
        .map(|v| v.file_name().to_string_lossy().parse())
        .collect();

    versions
}

/// Writes the given [`Version`] to the working directory.
///
/// If [`Some(Version)`] is passed, the passed [`Version`] is set.
/// if [`None`] is passed, the current version will be unset.
pub fn write_current_version(version: Option<&Version>) -> Result<()> {
    let vfile_path = get_work_dir()?.join(CURRENT_VERSION_FILE);
    match version {
        Some(v) => File::create(vfile_path)?.write_all(v.to_string().as_bytes())?,
        None => fs::remove_file(vfile_path)?,
    }
    Ok(())
}

/// Deletes an installed SDK by its [`Version`].
pub fn drop_version(version: &Version) -> Result<()> {
    let dir = get_version_installation_dir(version)?;
    fs::remove_dir_all(dir)?;
    Ok(())
}

/// Deletes the installation drirectory *(see [`get_installations_dir`])*
/// and all of its contents.
pub fn drop_install_dir() -> Result<()> {
    let dir = get_installations_dir()?;
    fs::remove_dir_all(dir)?;
    Ok(())
}
