mod cursor;
mod error;
mod helpers;
mod parser;

pub use error::Error;

use cursor::Cursor;
use error::Result;
use parser::parse_value;
use std::collections::HashMap;

pub enum Value<'l> {
    String(&'l [u8]),
    Integer(i64),
    List(Vec<Value<'l>>),
    Dict(HashMap<&'l [u8], Value<'l>>),
}

pub fn parse_bencoded_value(input: &[u8]) -> Result<(Value<'_>, &[u8])> {
    let mut cur = Cursor::new(input);
    let val = parse_value(&mut cur, 0)?;
    Ok((val, cur.rest()))
}
