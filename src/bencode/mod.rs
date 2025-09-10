mod cursor;
mod error;
mod helpers;
mod parser;

pub use {cursor::Cursor, error::Error, parser::extract_dict_value_range};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_single_valid_string() {
        let Value::String(out) = parse_bencoded_value("5:hello".as_bytes()).unwrap().0 else {
            panic!("Wrong parsed value")
        };
        assert_eq!(out, "hello".as_bytes())
    }
}
