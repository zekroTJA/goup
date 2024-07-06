#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
