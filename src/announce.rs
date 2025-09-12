use bytes::Bytes;
use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde::Serialize;
use std::time::Duration;
use thiserror::Error;
use url::Url;

use crate::bencode::Value;
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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AnnounceEvent {
    Started,
    Stopped,
    Completed,
    Empty,
}

impl AnnounceEvent {
    fn as_str(self) -> &'static str {
        match self {
            AnnounceEvent::Started => "started",
            AnnounceEvent::Stopped => "stopped",
            AnnounceEvent::Completed => "completed",
            AnnounceEvent::Empty => "empty",
        }
    }
}

#[derive(Debug, Default)]
pub struct AnnounceParams<'a> {
    pub info_hash: &'a [u8], // must be 20 bytes
    pub peer_id: &'a [u8],   // must be 20 bytes
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,

    pub compact: bool,
    pub numwant: Option<u16>,
    pub event: Option<AnnounceEvent>,
    pub key: Option<&'a str>,
    pub ip: Option<&'a str>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct PeersList {
    interval: u64,
    peers: Vec<Peer>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Peer {
    ip: String,
    port: u64,
}

pub fn new_client() -> Result<Client> {
    let client = Client::builder()
        .user_agent("bittorrent-rust/0.1")
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
}

/// Perform a tracker announce and return the raw response body.
/// - `announce_url` should be the full announce endpoint (http/https).
pub fn perform_announce(
    client: &Client,
    announce_url: &str,
    params: AnnounceParams<'_>,
) -> Result<Bytes> {
    if params.info_hash.len() != 20 {
        return Err(Error::InvalidInfoHash(params.info_hash.len()));
    }
    if params.peer_id.len() != 20 {
        return Err(Error::InvalidPeerId(params.peer_id.len()));
    }

    let mut url = Url::parse(announce_url)?;

    let mut encoded_tail = url::form_urlencoded::Serializer::new(String::new());
    encoded_tail.append_pair("port", &params.port.to_string());
    encoded_tail.append_pair("uploaded", &params.uploaded.to_string());
    encoded_tail.append_pair("downloaded", &params.downloaded.to_string());
    encoded_tail.append_pair("left", &params.left.to_string());

    if params.compact {
        encoded_tail.append_pair("compact", "1");
    }
    if let Some(nw) = params.numwant {
        encoded_tail.append_pair("numwant", &nw.to_string());
    }
    if let Some(ev) = params.event {
        encoded_tail.append_pair("event", ev.as_str());
    }
    if let Some(k) = params.key {
        encoded_tail.append_pair("key", k);
    }
    if let Some(ip) = params.ip {
        encoded_tail.append_pair("ip", ip);
    }

    let tail = encoded_tail.finish();

    let ih = urlencoding::encode_binary(params.info_hash);
    let pid = urlencoding::encode_binary(params.peer_id);

    let final_qs = if tail.is_empty() {
        format!("info_hash={}&peer_id={}", ih, pid)
    } else {
        format!("info_hash={}&peer_id={}&{}", ih, pid, tail)
    };
    url.set_query(Some(&final_qs));

    let resp = client.get(url).send()?;
    let status = resp.status();

    if !status.is_success() {
        let bytes = resp.bytes().unwrap_or_default();
        let snippet = String::from_utf8_lossy(&bytes[..bytes.len().min(256)]).to_string();
        return Err(Error::Status { status, snippet });
    }

    let body = resp.bytes()?;
    Ok(body)
}

impl PeersList {
    pub fn from(decoded_value: Value) -> Result<Self> {
        let root = metainfo::helpers::dict(&decoded_value, "<root>")?;
        let peers = metainfo::helpers::as_list(metainfo::helpers::get(root, "peers")?, "peers")?
            .iter()
            .map(|peer| -> Result<Peer> {
                let d = metainfo::helpers::dict(peer, "peer")?;
                let (ip, port) = (
                    metainfo::helpers::as_str(metainfo::helpers::get(d, "ip")?, "ip")?,
                    metainfo::helpers::as_u64(metainfo::helpers::get(d, "port")?, "port")?,
                );
                Ok(Peer {
                    ip: ip.to_owned(),
                    port,
                })
            })
            .filter_map(|p| p.ok())
            .collect();

        let interval =
            metainfo::helpers::as_u64(metainfo::helpers::get(root, "interval")?, "interval")?;

        Ok(PeersList { interval, peers })
    }
}
