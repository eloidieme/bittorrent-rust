mod announce;
mod bencode;
mod infohash;
mod metainfo;

use std::{
    env,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    str::FromStr,
};

use crate::{
    announce::{AnnounceParams, PeersList, perform_announce},
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
        ..Default::default()
    };
    let client = announce::new_client()?;
    let resp = perform_announce(&client, metainfo.announce, announce_qp)?;
    let decoded_response = parse_bencoded_value(&resp)?.0;
    let peers_list = PeersList::from(decoded_response)?;
    // println!("{}", serde_json::to_string_pretty(&metainfo)?);
    // println!("{}", serde_json::to_string_pretty(&peers_list)?);

    let peer = peers_list.peers.first().unwrap();
    let peer_address = std::net::Ipv4Addr::from_str(&format!("{}", &peer.ip))?;
    let addr = SocketAddr::new(std::net::IpAddr::V4(peer_address), peer.port as u16);
    println!("{addr}");

    let mut stream = TcpStream::connect(addr)?;
    // this can be a func -> message assembly
    let mut handshake: Vec<u8> = vec![19];
    handshake.extend_from_slice("BitTorrent protocol".as_bytes());
    handshake.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
    handshake.extend_from_slice(&info_hash);
    handshake.extend_from_slice("12345678901234567890".as_bytes());
    println!("{handshake:?}");
    // then comms on a top-level func that takes only a tcp stream and a message, and returns a buffer
    stream.write(&handshake)?;

    let mut resp_buf = [0u8; 68];
    stream.read(&mut resp_buf)?;
    println!("{resp_buf:?}");

    // stream is closed at the end of the scope
    // comms will be a collection of funcs, always taking in a stream and a buffer, (and returning status code?? or panics?)

    Ok(())
}

// fn message_assembly<'l>(info_hash: &'l [u8], peer_id: &'l [u8]) -> &'l [u8] {
//     let mut handshake: Vec<u8> = vec![19];
//     handshake.extend_from_slice("BitTorrent protocol".as_bytes());
//     handshake.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
//     handshake.extend_from_slice(&info_hash);
//     handshake.extend_from_slice("12345678901234567890".as_bytes());
//     println!("{handshake:?}");
// }
