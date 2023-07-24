use std::{process::ExitStatus, str::Utf8Error};

/// The error type for command executions.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The command executoable could not be found.
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
    Unknown(#[from] std::io::Error),
}

// impl Error {
//     pub fn kind(&self) -> &ErrorKind {
//         &self.0
//     }
// }

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self.kind() {
//             ErrorKind::NotFound => write!(f, "command not found"),
//             ErrorKind::Status((status, stderr)) => write!(f, "status error {status}: {stderr}"),
//             ErrorKind::Parameters(v) => write!(f, "parameter error: {v}"),
//             ErrorKind::OutputFormat(err) => write!(f, "output format error: {err}"),
//             ErrorKind::Unknown(err) => err.fmt(f),
//         }
//     }
// }

// impl std::error::Error for Error {}

// impl From<io::Error> for Error {
//     fn from(value: io::Error) -> Self {
//         match value.kind() {
//             io::ErrorKind::NotFound => ErrorKind::NotFound,
//             _ => ErrorKind::Unknown(Box::new(value)),
//         }
//         .into()
//     }
// }

// impl From<Utf8Error> for Error {
//     fn from(value: Utf8Error) -> Self {
//         ErrorKind::OutputFormat(value).into()
//     }
// }

// impl From<ErrorKind> for Error {
//     fn from(value: ErrorKind) -> Self {
//         Error(Box::new(value))
//     }
// }
