pub mod errors;

use self::errors::{Error, ErrorKind};
use std::process::Command;

pub fn exec(cmd: &[&str]) -> Result<String, Error> {
    if cmd.is_empty() {
        return Err(ErrorKind::Message("command is empty".into()).into());
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
