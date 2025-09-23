use reqwest::StatusCode;
use thiserror::Error;

use crate::metainfo;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error("tracker returned non-success status {status}: {snippet}")]
    Status { status: StatusCode, snippet: String },
    #[error("invalid info_hash length {0}, expected 20")]
    InvalidInfoHash(usize),
    #[error("invalid peer_id length {0}, expected 20")]
    InvalidPeerId(usize),
    #[error(transparent)]
    ParsingHelpers(#[from] metainfo::error::Error),
    #[error(transparent)]
    IntParsing(#[from] std::num::ParseIntError),
}
