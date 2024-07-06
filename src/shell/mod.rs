pub mod errors;

use self::errors::Error;
use crate::env;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};
use whattheshell::Shell;

/// A singleton OnceLock instance of the inferred current shell.
static SHELL: OnceLock<Shell> = OnceLock::new();

/// Returns the inferred current shell. If it has not been initialized already,
/// it will be initialized. This might fail with a panic.
pub fn get_shell() -> Shell {
    SHELL
        .get_or_init(|| Shell::infer().expect("failed inferring current shell"))
        .clone()
}

/// Provides an abstraction for shell-dependent actions.
pub trait ShellEnv {
    /// Returns the assembled shell command to set the given environment
    /// variable by key and value.
    fn get_setenv_command(&self, key: &str, val: &str) -> Result<String, Error>;
    /// Appends the given path string `new` to the passed `current` path
    /// concatenation string.
    fn append_to_path(&self, curr: &str, new: &str) -> Result<String, Error>;
    /// Returns the directory of the users profile file.
    fn get_profile_dir(&self) -> Result<PathBuf, Error>;
    /// Takes a path reference and converts it to a shell-compatible string.
    fn path_to_string<T: AsRef<Path>>(&self, path: T) -> Result<String, Error>;
    /// Returns the command to apply the env variables to the current
    /// shell session using the output of `goup env`.
    fn get_apply_env_command(&self) -> Result<&'static str, Error>;
    /// Returns true if the env variable `GOROOT` is correcly applied in
    /// the current shell environment.
    fn is_env_applied(&self) -> Result<bool, Error>;
}

impl ShellEnv for Shell {
    fn get_setenv_command(&self, key: &str, val: &str) -> Result<String, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh => Ok(format!("export {key}=\"{val}\"")),
            Self::Cmd | Self::PowerShell => Ok(format!("$env:{key} = \"{val}\"")),
            // use toml format to let Nushell parse the values
            Self::Nushell => Ok(format!("{key} = '''{val}'''")),
            Self::Fish => Ok(format!("set -gx {key} {val}")),
        }
    }

    fn append_to_path(&self, curr: &str, new: &str) -> Result<String, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh => {
                #[cfg(not(windows))]
                return Ok(format!("{new}:{curr}"));

                #[cfg(windows)]
                return Ok(format!("{}:{}", env::to_gitbash_path_var(curr), new));
            }
            Self::Nushell => {
                #[cfg(not(windows))]
                return Ok(format!("{new}:{curr}"));

                #[cfg(windows)]
                return Ok(format!("{new};{curr}"));
            }
            Self::Cmd | Self::PowerShell => Ok(format!("{new};{curr}")),
            Self::Fish => Ok(format!("{new} $PATH")),
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
            #[cfg(not(windows))]
            Self::Nushell => Ok(home.join(".config").join("nushell").join("config.nu")),
            #[cfg(windows)]
            Self::Nushell => Ok(home
                .join("AppData")
                .join("Roaming")
                .join("nushell")
                .join("config.nu")),
            Self::Fish => Ok(home
                .join(".config")
                .join("fish")
                .join("conf.d")
                .join("goup.fish")),
        }
    }

    fn path_to_string<T: AsRef<Path>>(&self, path: T) -> Result<String, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh => {
                #[cfg(not(windows))]
                return Ok(path.as_ref().to_string_lossy().to_string());

                #[cfg(windows)]
                return Ok(env::to_gitbash_path(&path.as_ref().to_string_lossy()));
            }
            Self::Cmd | Self::PowerShell | Self::Nushell => {
                Ok(path.as_ref().to_string_lossy().to_string())
            }
            Self::Fish => Ok(path.as_ref().to_string_lossy().to_string()),
        }
    }

    fn get_apply_env_command(&self) -> Result<&'static str, Error> {
        match self {
            Self::Bash | Self::Sh | Self::Zsh | Self::Fish => Ok(r#"eval "$(goup env)""#),
            Self::Cmd | Self::PowerShell => Ok("goup env | Out-String | Invoke-Expression"),

            // Nushell doesn't support eval
            // https://www.nushell.sh/book/how_nushell_code_gets_run.html#eval-function
            Self::Nushell => {
                #[cfg(not(windows))]
                return Ok("load-env (\
                    goup env \
                    | from toml \
                    | update PATH {do $env.ENV_CONVERSIONS.PATH.from_string $in}\
                )");

                #[cfg(windows)]
                return Ok("load-env (\
                    goup env \
                    | from toml \
                    | rename -c {PATH: Path} \
                    | update Path {do $env.ENV_CONVERSIONS.Path.from_string $in}\
                )");
            }
        }
    }

    fn is_env_applied(&self) -> Result<bool, Error> {
        let current_install_dir = env::get_current_install_dir()?;
        let current_install_dir = self.path_to_string(current_install_dir)?;
        let set_install_dir = self.path_to_string(std::env::var("GOROOT").unwrap_or_default())?;
        Ok(set_install_dir == current_install_dir)
    }
}
