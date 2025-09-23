mod error;

use sha1::{Digest, Sha1};

use crate::{
    bencode::{Cursor, extract_dict_value_range},
    infohash::error::Result,
};

pub fn compute_info_hash(input: &[u8]) -> Result<Vec<u8>> {
    let mut cur = Cursor::new(input);
    let raw_info_dict_range = extract_dict_value_range(&mut cur, "info")?;
    let raw_info_dict = &input[raw_info_dict_range];

    let mut hasher = Sha1::new();
    hasher.update(raw_info_dict);

    Ok(hasher.finalize()[..].to_vec())
}
