use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotewormError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}