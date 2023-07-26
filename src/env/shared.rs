use crate::{shell::ShellEnv, versions::Version, warning};
use anyhow::Result;
use directories::UserDirs;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};
use whattheshell::Shell;

/// The file where the currently selected version is
/// written into.
const CURRENT_VERSION_FILE: &str = ".current_version";

/// Returns all required environment variables.
pub fn get_env_vars(shell: &Shell) -> Result<String> {
    let path = std::env::var("PATH")?;

    let vars = vec![
        (
            "PATH",
            shell.append_to_path(&path, &shell.path_to_string(get_current_bin_dir()?)?)?,
        ),
        ("GOROOT", shell.path_to_string(get_current_install_dir()?)?),
    ];

    let lines: Result<Vec<_>, _> = vars
        .iter()
        .map(|(k, v)| shell.get_setenv_command(k, v))
        .collect();

    Ok(lines?.join("\n"))
}

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

/// Reads and returns the content of the profile file in
/// the current users `$HOME` directory.
///
/// See [`get_profile_dir`] for more information.
pub fn read_profile(shell: &Shell) -> Result<String> {
    let profile_dir = shell.get_profile_dir()?;

    if !profile_dir.exists() {
        return Ok(String::new());
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
pub fn append_to_profile(shell: &Shell, content: &str) -> Result<()> {
    let profile_dir = shell.get_profile_dir()?;

    let mut f = if profile_dir.exists() {
        File::options().append(true).open(profile_dir)?
    } else {
        if let Some(parent) = profile_dir.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        File::create(profile_dir)?
    };

    f.write_all(content.as_bytes())?;

    Ok(())
}

/// Checks if the `GOROOT` environment variable is applied and prints a
/// warning message to the terminal if not.
pub fn check_env_applied(shell: &Shell) -> Result<()> {
    if !shell.is_env_applied()? {
        warning!(
            "Seems like necessary environment variables have not been applied. \
            This results in the selected SDK version not being available in the terminal.\n\
            Please see `goup help env` to setup required environment variables.\n"
        );
    }
    Ok(())
}

/// Takes a path string and transforms it to a GitBash style path.
#[cfg(windows)]
pub fn to_gitbash_path(pth: &str) -> String {
    let pth = pth.replace('\\', "/");

    let mut chars = pth.chars();

    if chars.nth(1) == Some(':') && chars.next() == Some('/') {
        format!("/{}{}", &pth[..1].to_lowercase(), &pth[2..])
    } else {
        pth
    }
}

/// Takes an path concatenation and transforms it to a bash style path
/// concatenation. Every contained path is transformed to a GitBash
/// style path.
#[cfg(windows)]
pub fn to_gitbash_path_var(curr: &str) -> String {
    curr.split(';')
        .map(to_gitbash_path)
        .collect::<Vec<_>>()
        .join(":")
}

#[cfg(test)]
mod test {

    #[cfg(windows)]
    #[test]
    fn test_to_gitbash_path() {
        use super::*;

        assert_eq!("/c/users/foo/bar", to_gitbash_path(r"C:\users\foo\bar"));
        assert_eq!("/c/users/foo/bar", to_gitbash_path(r"C:/users/foo/bar"));
        assert_eq!("users/foo/bar", to_gitbash_path(r"users\foo\bar"));
    }
}
