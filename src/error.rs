use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DemoticError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
    #[error("Unknown error")]
    UnknownError,
}
