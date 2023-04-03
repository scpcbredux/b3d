use thiserror::Error;
use crate::utils::Chunk;

#[derive(Error, Debug)]
pub enum B3dError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Invalid Chunk: {0}")]
    InvalidChunk(Chunk),
}