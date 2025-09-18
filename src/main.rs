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

    let mut stream = TcpStream::connect(addr)?;
    let handshake_message = build_handshake_message(&info_hash, "12345678901234567890");
    let response = send_message(&mut stream, &handshake_message)?;
    println!("{response:?}");

    Ok(())
}

fn build_handshake_message(info_hash: &[u8], peer_id: &str) -> Vec<u8> {
    let mut handshake: Vec<u8> = vec![19];
    handshake.extend_from_slice("BitTorrent protocol".as_bytes());
    handshake.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
    handshake.extend_from_slice(info_hash);
    handshake.extend_from_slice(peer_id.as_bytes());

    handshake
}

fn send_message(stream: &mut TcpStream, message: &[u8]) -> std::io::Result<Vec<u8>> {
    stream.write_all(message)?;

    let mut resp = vec![0u8; 68];
    let mut got = 0;
    while got < 68 {
        let n = stream.read(&mut resp[got..])?;
        if n == 0 { break; } // peer closed
        got += n;
    }
    resp.truncate(got);
    Ok(resp)
}
