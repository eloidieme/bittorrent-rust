mod cursor;
mod error;
mod helpers;
mod parser;

#[cfg(test)]
mod tests;

pub use {
    cursor::Cursor,
    error::{Error, Offset},
    parser::extract_dict_value_range,
};

use error::Result;
use parser::parse_value;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
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

pub fn parse_bencoded_all(input: &[u8]) -> Result<Value<'_>> {
    let (v, rest) = parse_bencoded_value(input)?;
    if !rest.is_empty() {
        return Err(Error::UnexpectedByte {
            at: Offset(input.len() - rest.len()),
            found: rest[0],
            expected: "end of input",
        });
    }
    Ok(v)
}

impl<'l> Value<'l> {
    pub fn as_bytes(&self) -> Option<&'l [u8]> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Value::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn get<'a>(&'a self, k: &[u8]) -> Option<&'a Value<'l>> {
        match self {
            Value::Dict(d) => d.get(k),
            _ => None,
        }
    }
}
