//! BitTorrent Peer Protocol Implementation
//!
//! This module provides a high-level API for implementing the BitTorrent peer protocol.
//! It handles TCP connections, handshakes, and message exchange with BitTorrent peers.
//!
//! # Example
//!
//! ```rust,no_run
//! use bittorrent_rust::peer_protocol::PeerConnection;
//! use std::net::SocketAddr;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let peer_addr = "127.0.0.1:6881".parse::<SocketAddr>()?;
//! let info_hash = b"example_info_hash_20_bytes";
//! let peer_id = "MyClient123456789012";
//!
//! let mut connection = PeerConnection::connect(peer_addr, info_hash, peer_id)?;
//! println!("Connected to peer: {}", connection.peer_addr());
//!
//! // Send a keep-alive message
//! let keep_alive = vec![0, 0, 0, 0];
//! connection.send_message_no_response(&keep_alive)?;
//! # Ok(())
//! # }
//! ```

mod connection;
mod error;

pub use connection::PeerConnection;
pub use error::PeerProtocolError;
