//! BitTorrent client library

pub mod announce;
pub mod bencode;
pub mod infohash;
pub mod metainfo;
pub mod peer_protocol;

/// Generates a unique peer ID for BitTorrent protocol
///
/// The peer ID follows the format: `-XX0000-` followed by 12 random characters
/// where XX is a client identifier. This creates a 20-byte string as required
/// by the BitTorrent protocol.
pub fn generate_peer_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    let hash = hasher.finish();

    let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut result = String::with_capacity(20);

    result.push_str("-BR0001-");

    let mut seed = hash;
    for _ in 0..12 {
        let idx = (seed % chars.len() as u64) as usize;
        result.push(chars.chars().nth(idx).unwrap());
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345); // Linear congruential generator
    }

    result
}
