use thiserror::Error;
use crate::utils::Chunk;

#[derive(Error, Debug)]
pub enum B3dError {
    #[error("Invalid Chunk: {0}")]
    InvalidChunk(Chunk),
}