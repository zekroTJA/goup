pub mod errors;

use self::errors::{Error, ErrorKind};
use std::process::Command;

/// Execute a given command and return its
/// Stdout output as string.
///
/// # Errors
/// If the given command has a non-zero status
/// code, an [`Error`] of [`ErrorKind::Status`]
/// is returned.
///
/// # Example
/// ```
/// let output = exec(&["ls", "-lisah"]).unwrap();
/// ```
pub fn exec(cmd: &[&str]) -> Result<String, Error> {
    if cmd.is_empty() {
        return Err(ErrorKind::Parameters("command is empty".into()).into());
    }

    let prog = cmd[0];
    let args = &cmd[1..];
    let res = Command::new(prog).args(args).output()?;

    if !res.status.success() {
        let stderr = std::str::from_utf8(&res.stderr)?;
        return Err(ErrorKind::Status((res.status, stderr.into())).into());
    }

    let stdout = std::str::from_utf8(&res.stdout)?;
    Ok(stdout.to_string())
}
