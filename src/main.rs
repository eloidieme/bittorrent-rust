mod bencode;
mod infohash;
mod metainfo;

use std::env;

use crate::{bencode::parse_bencoded_value, infohash::compute_info_hash, metainfo::Metainfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let fp = args.next().unwrap();
    let file = std::fs::read(fp)?;
    let val = parse_bencoded_value(&file)?.0;
    let metainfo = Metainfo::from(&val)?;
    let info_hash = compute_info_hash(&file)?;
    // println!("{}", serde_json::to_string_pretty(&metainfo)?);
    println!("{info_hash:#04X?}");

    Ok(())
}
