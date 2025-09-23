use thiserror::Error;

#[derive(Error, Debug)]
pub enum PeerProtocolError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid handshake response: expected {expected} bytes, got {actual}")]
    InvalidHandshakeLength { expected: usize, actual: usize },

    #[error("Invalid protocol identifier in handshake response")]
    InvalidProtocolIdentifier,

    #[error("Info hash mismatch in handshake response")]
    InfoHashMismatch,

    #[error("Connection closed by peer")]
    ConnectionClosed,

    #[error("Invalid peer address: {0}")]
    InvalidPeerAddress(String),

    #[error("Failed to connect to peer: {0}")]
    ConnectionFailed(String),
}

pub type Result<T> = std::result::Result<T, PeerProtocolError>;
