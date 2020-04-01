use thiserror::Error;

#[derive(Debug, Error)]
pub enum DemoParseError {
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
    #[error("Unknown error")]
    UnknownError,
}
