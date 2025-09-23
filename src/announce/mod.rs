//! BitTorrent tracker announce functionality
//!
//! This module provides functionality for announcing to BitTorrent trackers,
//! including parameter validation, HTTP client management, and response parsing.

pub mod announce;
pub mod client;
pub mod error;
pub mod params;
pub mod response;

#[cfg(test)]
mod tests;

// Re-export the main types and functions for easy access
pub use announce::perform_announce;
pub use client::new_client;
pub use error::{Error, Result};
pub use params::{AnnounceEvent, AnnounceParams};
pub use response::{Peer, PeersList};
