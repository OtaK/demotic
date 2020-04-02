use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DemoParseError {
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
    #[error("Unknown error")]
    UnknownError,
}
