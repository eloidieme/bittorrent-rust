use std::{env, net::SocketAddr, str::FromStr};

use bittorrent_rust::{
    announce::{AnnounceEvent, AnnounceParams, PeersList, new_client, perform_announce},
    bencode::parse_bencoded_value,
    generate_peer_id,
    infohash::compute_info_hash,
    metainfo::{Info, Metainfo},
    peer_protocol::PeerConnection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("bittorrent-rust: Minimal BitTorrent client (WIP)");
    println!("=============================================");

    let mut args = env::args().skip(1);
    let torrent_path = match args.next() {
        Some(fp) => fp,
        None => {
            eprintln!("Usage: bittorrent-rust <torrent-file>");
            std::process::exit(1);
        }
    };

    let file = std::fs::read(&torrent_path)?;
    let (bencode_val, _) = parse_bencoded_value(&file)?;
    let metainfo = Metainfo::from(&bencode_val)?;
    let info_hash = compute_info_hash(&file)?;

    println!("Loaded torrent: {}", metainfo.name());
    println!("Info hash: {:x?}", info_hash);

    let peer_id = generate_peer_id();
    println!("Generated peer ID: {}", peer_id);

    let port = 6881; // TODO: select an available port dynamically
    let left = match &metainfo.info {
        Info::SingleFile { length, .. } => *length,
        Info::MultiFile { files, .. } => files.iter().map(|f| f.length).sum(),
    };

    let announce_params = AnnounceParams::new(
        &info_hash,
        peer_id.as_bytes(),
        port,
        0, // uploaded
        0, // downloaded
        left,
    )?
    .with_event(AnnounceEvent::Started)
    .with_numwant(50); // Request up to 50 peers

    let client = new_client()?;
    println!("Announcing to tracker: {}", metainfo.announce);
    let tracker_resp = perform_announce(&client, metainfo.announce, announce_params)?;

    let (decoded_response, _) = parse_bencoded_value(&tracker_resp)?;
    let peers_list = PeersList::from(decoded_response)?;

    println!("Found {} peer(s) from tracker.", peers_list.peers.len());
    if peers_list.peers.is_empty() {
        println!("No peers found. Exiting.");
        return Ok(());
    }

    let peer = &peers_list.peers[0];
    let peer_address = std::net::Ipv4Addr::from_str(&peer.ip.to_string())?;
    let addr = SocketAddr::new(std::net::IpAddr::V4(peer_address), peer.port as u16);

    println!("Connecting to peer: {}:{}", peer.ip, peer.port);
    let mut peer_connection = match PeerConnection::connect(addr, &info_hash, &peer_id) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to peer: {}", e);
            return Ok(());
        }
    };

    println!("Connected to peer: {}", peer_connection.peer_addr());
    if let Some(peer_id) = peer_connection.peer_id() {
        println!("Peer ID: {}", peer_id);
    }

    let keep_alive = vec![0, 0, 0, 0];
    peer_connection.send_message_no_response(&keep_alive)?;
    println!("Sent keep-alive message to peer.");

    // TODO: Expand here - download pieces, handle messages, etc.

    Ok(())
}
