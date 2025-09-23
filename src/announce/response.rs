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
