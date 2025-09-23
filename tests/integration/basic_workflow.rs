use bittorrent_rust::{
    announce::{AnnounceParams, PeersList, new_client, perform_announce},
    bencode::parse_bencoded_value,
    generate_peer_id,
    infohash::compute_info_hash,
    metainfo::Metainfo,
};

#[test]
fn test_basic_torrent_parsing() {
    // This test would require a sample torrent file
    // For now, it's a placeholder for integration testing
    assert!(true);
}

#[test]
fn test_peer_id_generation() {
    let peer_id1 = generate_peer_id();
    let peer_id2 = generate_peer_id();

    // Peer IDs should be 20 characters long
    assert_eq!(peer_id1.len(), 20);
    assert_eq!(peer_id2.len(), 20);

    // Peer IDs should be different
    assert_ne!(peer_id1, peer_id2);

    // Should start with client identifier
    assert!(peer_id1.starts_with("-BR0001-"));
    assert!(peer_id2.starts_with("-BR0001-"));
}
