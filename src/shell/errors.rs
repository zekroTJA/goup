#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unsupported shell")]
    UnsupportedShell,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
