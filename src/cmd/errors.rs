use std::{io, process::ExitStatus, str::Utf8Error};

/// The error type for command executions.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The command executable could not be found.
    #[error("command not found")]
    NotFound,
    /// The command failed with a non-zero status code.
    #[error("status error {0}: {1}")]
    Status(ExitStatus, String),
    /// The execution failed due to invalid input
    /// parameters.
    #[error("parameter error: {0}")]
    Parameters(String),
    /// The formatting of the output data to string
    /// has failed.
    #[error("output format error: {0}")]
    OutputFormat(#[from] Utf8Error),
    /// The command execution failed unexpectedly.
    #[error(transparent)]
    Unknown(std::io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::Unknown(value),
        }
    }
}
