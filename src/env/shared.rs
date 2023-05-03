use crate::versions::Version;
use anyhow::Result;
use directories::UserDirs;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

const CURRENT_VERSION_FILE: &str = ".current_version";

pub fn get_home_dir() -> Result<PathBuf> {
    UserDirs::new()
        .ok_or_else(|| anyhow::anyhow!("could not find user directories"))
        .map(|dirs| dirs.home_dir().to_path_buf())
}

pub fn get_work_dir() -> Result<PathBuf> {
    get_home_dir().map(|dir| dir.join(".local").join("goup"))
}

pub fn get_installations_dir() -> Result<PathBuf> {
    get_work_dir().map(|dir| dir.join("installations"))
}

pub fn get_current_install_dir() -> Result<PathBuf> {
    get_work_dir().map(|dir| dir.join("current"))
}

pub fn get_current_bin_dir() -> Result<PathBuf> {
    get_current_install_dir().map(|dir| dir.join("go").join("bin"))
}

pub fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

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

pub fn list_installed_versions() -> Result<Vec<Version>> {
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

pub fn get_version_installation_dir(version: &Version) -> Result<PathBuf> {
    get_installations_dir().map(|v| v.join(version.to_string()))
}

pub fn write_current_version(version: &Version) -> Result<()> {
    let vfile_path = get_work_dir()?.join(CURRENT_VERSION_FILE);
    File::create(vfile_path)?.write_all(version.to_string().as_bytes())?;
    Ok(())
}
