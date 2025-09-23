use std::num::ParseIntError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offset(pub usize);

#[derive(Debug, Error)]
pub enum Error {
    // Structure / Framing
    #[error("unexpected end of input at {at:?}")]
    UnexpectedEof { at: Offset },

    #[error("unexpected byte {found:#04x} at {at:?}, expected {expected}")]
    UnexpectedByte {
        at: Offset,
        found: u8,
        expected: &'static str,
    },

    #[error("missing terminator 'e' for {context} starting at {start:?}")]
    MissingTerminator {
        context: &'static str,
        start: Offset,
    },

    // String lengths
    #[error("invalid string length `{raw}` at {at:?}")]
    InvalidStringLength {
        at: Offset,
        raw: String,
        #[source]
        source: ParseIntError,
    },

    #[error("string claims {expected} bytes but only {available} available at {at:?}")]
    InsufficientStringBytes {
        at: Offset,
        expected: usize,
        available: usize,
    },

    // Integers
    #[error("invalid integer syntax `{raw}` at {at:?}: {reason}")]
    InvalidIntegerSyntax {
        at: Offset,
        raw: String,
        reason: &'static str, // "empty", "leading zero", "negative zero"
    },

    #[error("failed to parse integer `{raw}` at {at:?}")]
    InvalidIntegerValue {
        at: Offset,
        raw: String,
        #[source]
        source: ParseIntError,
    },

    // Dictionaries
    #[error("non-string dictionary key at {at:?}")]
    NonStringDictKey { at: Offset },

    #[error("key `{key}` not found in dict at {at:?}")]
    DictKeyNotFound { at: Offset, key: String },

    // Limits
    #[error("nesting depth limit exceeded at {at:?}")]
    DepthLimit { at: Offset },
}
