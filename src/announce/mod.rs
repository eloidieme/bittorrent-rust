//! BitTorrent tracker announce functionality
//!
//! This module provides functionality for announcing to BitTorrent trackers,
//! including parameter validation, HTTP client management, and response parsing.

use bytes::Bytes;
use reqwest::blocking::Client;
use url::Url;

pub mod client;
pub mod error;
pub mod params;
pub mod response;

#[cfg(test)]
mod tests;

pub use client::new_client;
pub use error::{Error, Result};
pub use params::{AnnounceEvent, AnnounceParams};
pub use response::{Peer, PeersList};

/// Perform a tracker announce and return the raw response body.
///
/// This function sends an announce request to a BitTorrent tracker and returns
/// the raw response body. The response is typically a bencoded dictionary
/// containing peer information and tracker metadata.
///
/// # Arguments
///
/// * `client` - The HTTP client to use for the request
/// * `announce_url` - The full announce endpoint URL (http/https)
/// * `params` - The announce parameters (must be validated)
///
/// # Errors
///
/// This function will return an error if:
/// - The announce URL is invalid
/// - The HTTP request fails
/// - The tracker returns a non-success status code
/// - The response body cannot be read
///
/// # Example
///
/// ```rust,no_run
/// use bittorrent_rust::announce::{new_client, AnnounceParams, AnnounceEvent, perform_announce};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = new_client()?;
/// let info_hash = [0u8; 20]; // Example 20-byte info hash
/// let peer_id = [0u8; 20];    // Example 20-byte peer ID
/// let params = AnnounceParams::new(
///     &info_hash,
///     &peer_id,
///     6881,
///     0,
///     0,
///     1000,
/// )?.with_event(AnnounceEvent::Started);
///
/// let response = perform_announce(&client, "http://tracker.example.com/announce", params)?;
/// # Ok(())
/// # }
/// ```
pub fn perform_announce(
    client: &Client,
    announce_url: &str,
    params: AnnounceParams<'_>,
) -> Result<Bytes> {
    params.validate()?;

    let mut url = Url::parse(announce_url)?;
    url.set_query(Some(&params.build_query_string()));

    let resp = client.get(url).send()?;
    let status = resp.status();

    if !status.is_success() {
        let bytes = resp.bytes().unwrap_or_default();
        let snippet = String::from_utf8_lossy(&bytes[..bytes.len().min(256)]).to_string();
        return Err(crate::announce::error::Error::Status { status, snippet });
    }

    resp.bytes().map_err(Into::into)
}
