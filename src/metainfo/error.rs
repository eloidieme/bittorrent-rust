use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing key `{key}`")]
    MissingKey { key: &'static str },

    #[error("key `{key}` has wrong type (expected {expected})")]
    WrongType {
        key: &'static str,
        expected: &'static str,
    },

    #[error("key `{key}` is not valid UTF-8")]
    Utf8 {
        key: &'static str,
        #[source]
        source: std::str::Utf8Error,
    },

    #[error("`{key}` must be non-negative, got {got}")]
    Negative { key: &'static str, got: i64 },

    #[error("`piece length` must be > 0, got {got}")]
    PieceLengthZero { got: u64 },

    #[error("`pieces` length must be a multiple of 20, got {len}")]
    PiecesNonMultipleOf20 { len: usize },

    #[error("`info` must contain exactly one of `length` (single-file) or `files` (multi-files)")]
    LengthOrFilesMissing,

    #[error("`info` contains both `length` and `files` (mutually exclusive)")]
    BothLengthAndFiles,

    #[error("`files` must be a non-empty list")]
    FilesEmpty,

    #[error("file #{index}: missing key `{key}`")]
    FileMissingKey { index: usize, key: &'static str },

    #[error("file #{index}: key `{key}` has wrong type (expected {expected})")]
    FileWrongType {
        index: usize,
        key: &'static str,
        expected: &'static str,
    },

    #[error("file #{index}: path list must be non-empty")]
    FileEmptyPath { index: usize },

    #[error("file #{index}: path component #{component_index} is empty")]
    FileEmptyPathComponent {
        index: usize,
        component_index: usize,
    },

    #[error("file #{index}: invalid UTF-8 in `path`")]
    FileUtf8 {
        index: usize,
        #[source]
        source: std::str::Utf8Error,
    },

    #[error(transparent)]
    Bencode(#[from] crate::bencode::Error),
}
