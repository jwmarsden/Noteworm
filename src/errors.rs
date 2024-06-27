use thiserror::Error;

#[derive(Error, Debug)]
pub enum BookwormError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}