use serde::Serialize;

use crate::announce::error::Result;
use crate::bencode::Value;
use crate::metainfo;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct PeersList {
    pub interval: u64,
    pub peers: Vec<Peer>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Peer {
    pub ip: String,
    pub port: u64,
}

impl PeersList {
    pub fn from(decoded_value: Value) -> Result<Self> {
        let root = metainfo::helpers::dict(&decoded_value, "<root>")?;

        let peers = match metainfo::helpers::get(root, "peers")? {
            Value::String(peers_bytes) => {
                // Compact binary format: each peer is 6 bytes (4 bytes IP + 2 bytes port)
                Self::parse_compact_peers(peers_bytes)?
            }
            Value::List(peers_list) => {
                // Dictionary format: list of dictionaries with ip and port
                peers_list
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
                    .collect()
            }
            _ => {
                return Err(metainfo::error::Error::WrongType {
                    key: "peers",
                    expected: "string or list",
                }
                .into());
            }
        };

        let interval =
            metainfo::helpers::as_u64(metainfo::helpers::get(root, "interval")?, "interval")?;

        Ok(PeersList { interval, peers })
    }

    /// Parses peers from compact binary format
    fn parse_compact_peers(peers_bytes: &[u8]) -> Result<Vec<Peer>> {
        let mut peers = Vec::new();

        // Each peer is 6 bytes: 4 bytes IP + 2 bytes port
        if peers_bytes.len() % 6 != 0 {
            return Err(metainfo::error::Error::WrongType {
                key: "peers",
                expected: "binary data with length divisible by 6",
            }
            .into());
        }

        for chunk in peers_bytes.chunks_exact(6) {
            let ip = format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]);
            let port = u16::from_be_bytes([chunk[4], chunk[5]]) as u64;

            peers.push(Peer { ip, port });
        }

        Ok(peers)
    }
}
