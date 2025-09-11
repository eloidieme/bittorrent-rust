mod announce;
mod bencode;
mod infohash;
mod metainfo;

use std::env;

use crate::{
    announce::{AnnounceEvent, AnnounceParams, perform_announce},
    bencode::parse_bencoded_value,
    infohash::compute_info_hash,
    metainfo::Metainfo,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let fp = args.next().unwrap();
    let file = std::fs::read(fp)?;
    let val = parse_bencoded_value(&file)?.0;
    let metainfo = Metainfo::from(&val)?;
    let info_hash = compute_info_hash(&file)?;
    let announce_qp = AnnounceParams {
        info_hash: &info_hash,
        peer_id: "12345678901234567890".as_bytes(),
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: match metainfo.info {
            metainfo::Info::SingleFile {
                name: _,
                piece_length: _,
                pieces: _,
                length,
            } => length,
            metainfo::Info::MultiFile {
                name: _,
                piece_length: _,
                pieces: _,
                files,
            } => files.iter().map(|f| f.length).sum(),
        },
        event: Some(AnnounceEvent::Empty),
        ..Default::default()
    };
    let client = announce::new_client()?;
    let resp = perform_announce(&client, metainfo.announce, announce_qp)?;
    let decoded_response = parse_bencoded_value(&resp)?.0;
    // println!("{}", serde_json::to_string_pretty(&metainfo)?);
    println!("{decoded_response:?}");

    Ok(())
}
