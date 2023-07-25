pub mod errors;

use self::errors::Error;
use crate::env;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};
use whattheshell::Shell;

static SHELL: OnceLock<Shell> = OnceLock::new();

pub fn get_shell() -> Shell {
    SHELL
        .get()
        .expect("failed getting shell instance; this is very likely a bug")
        .clone()
}

pub fn set_shell(shell: Shell) {
    SHELL
        .set(shell)
        .expect("failed setting shell instance; this is very likely a bug")
}

pub trait ShellEnv {
    fn get_setenv_command(&self, key: &str, val: &str) -> Result<String, Error>;
    fn append_to_path(&self, curr: &str, new: &str) -> Result<String, Error>;
    fn get_profile_dir(&self) -> Result<PathBuf, Error>;
    fn path_to_string(&self, path: &Path) -> Result<String, Error>;
}

impl ShellEnv for Shell {
    fn get_setenv_command(&self, key: &str, val: &str) -> Result<String, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh => Ok(format!("export {key}=\"{val}\"")),
            Self::Cmd | Self::PowerShell => Ok(format!("$env:{key} = \"{val}\"")),
            _ => Err(Error::UnsupportedShell),
        }
    }

    fn append_to_path(&self, curr: &str, new: &str) -> Result<String, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh => {
                #[cfg(not(windows))]
                return Ok(format!("{curr}:{new}"));

                #[cfg(windows)]
                return Ok(format!("{}:{}", env::to_gitbash_path_var(curr), new));
            }
            Self::Cmd | Self::PowerShell => Ok(format!("{curr};{new}")),
            _ => Err(Error::UnsupportedShell),
        }
    }

    fn get_profile_dir(&self) -> Result<PathBuf, Error> {
        let home = env::get_home_dir()?;

        match self {
            Self::Bash | Self::Sh => Ok(home.join(".profile")),
            Self::Zsh => Ok(home.join(".zshenv")),
            Self::PowerShell | Self::Cmd => Ok(home
                .join("Documents")
                .join("WindowsPowerShell")
                .join("Microsoft.PowerShell_profile.ps1")),
            _ => Err(Error::UnsupportedShell),
        }
    }

    fn path_to_string(&self, path: &Path) -> Result<String, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh => {
                #[cfg(not(windows))]
                return Ok(path.to_string_lossy().to_string());

                #[cfg(windows)]
                return Ok(env::to_gitbash_path(&path.to_string_lossy()));
            }
            Self::Cmd | Self::PowerShell => Ok(path.to_string_lossy().to_string()),
            _ => Err(Error::UnsupportedShell),
        }
    }
}
