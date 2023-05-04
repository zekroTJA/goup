use core::fmt;
use std::{io, process::ExitStatus, str::Utf8Error};

/// The error type for command executions.
///
/// You can use the `kind` method to match
/// the errors kind.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

/// Command execution error kind.
#[derive(Debug)]
pub enum ErrorKind {
    /// The command executoable could not be found.
    NotFound,
    /// The command failed with a non-zero status code.
    Status((ExitStatus, String)),
    /// The execution failed due to invalid input
    /// parameters.
    Parameters(String),
    /// The formatting of the output data to string
    /// has failed.
    OutputFormat(Utf8Error),
    /// The command execution failed unexpectedly.
    Unknown(Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind() {
            ErrorKind::NotFound => write!(f, "command not found"),
            ErrorKind::Status((status, stderr)) => write!(f, "status error {status}: {stderr}"),
            ErrorKind::Parameters(v) => write!(f, "parameter error: {v}"),
            ErrorKind::OutputFormat(err) => write!(f, "output format error: {err}"),
            ErrorKind::Unknown(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            _ => ErrorKind::Unknown(Box::new(value)),
        }
        .into()
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        ErrorKind::OutputFormat(value).into()
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error(Box::new(value))
    }
}
