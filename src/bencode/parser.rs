use std::collections::HashMap;

use crate::bencode::{
    Value,
    cursor::Cursor,
    error::{Error, Offset, Result},
};

const MAX_DEPTH: usize = 1024;

pub fn parse_value<'l>(cur: &mut Cursor<'l>, depth: usize) -> Result<Value<'l>> {
    if depth > MAX_DEPTH {
        return Err(Error::DepthLimit { at: cur.offset() });
    }
    let Some(b) = cur.peek() else {
        return Err(Error::UnexpectedEof { at: cur.offset() });
    };
    match b {
        b'0'..=b'9' => parse_string(cur),
        b'i' => parse_integer(cur),
        b'l' => parse_list(cur, depth + 1),
        b'd' => parse_dict(cur, depth + 1),
        found => Err(Error::UnexpectedByte {
            at: cur.offset(),
            found,
            expected: "one of: digit / 'i' / 'l' / 'd'",
        }),
    }
}

fn parse_string<'l>(cur: &mut Cursor<'l>) -> Result<Value<'l>> {
    let start = cur.offset();
    let mut i = 0usize;
    while let Some(b) = cur.buf.get(cur.pos + i).copied() {
        if b == b':' {
            break;
        }
        if !b.is_ascii_digit() {
            return Err(Error::UnexpectedByte {
                at: Offset(cur.pos + i),
                found: b,
                expected: "ASCII digit or ':'",
            });
        }
        i += 1;
    }

    if cur.buf.get(cur.pos + i).copied() != Some(b':') {
        return Err(Error::UnexpectedEof { at: cur.offset() });
    }

    let header = cur.take(i)?;
    cur.expect_byte(b':')?;

    let raw = std::str::from_utf8(header).unwrap_or("");
    let len: usize = raw.parse().map_err(|e| Error::InvalidStringLength {
        at: start,
        raw: raw.to_string(),
        source: e,
    })?;

    let remaining = cur.buf.len().saturating_sub(cur.pos);
    if remaining < len {
        return Err(Error::InsufficientStringBytes {
            at: cur.offset(),
            expected: len,
            available: remaining,
        });
    }

    let s = cur.take(len)?;
    Ok(Value::String(s))
}

fn parse_integer<'l>(cur: &mut Cursor<'l>) -> Result<Value<'l>> {
    // "i" <digits or '-digits'> "e"
    let start = cur.offset();
    cur.expect_byte(b'i')?;

    let mut i = 0usize;
    while let Some(b) = cur.buf.get(cur.pos + i).copied() {
        if b == b'e' {
            break;
        }
        i += 1;
    }

    if cur.buf.get(cur.pos + i).copied() != Some(b'e') {
        return Err(Error::MissingTerminator {
            context: "integer",
            start,
        });
    }

    let raw_bytes = cur.take(i)?;
    cur.expect_byte(b'e')?;

    let raw = String::from_utf8_lossy(raw_bytes).into_owned();

    // Policy checks per BEP 3:
    // - not empty
    // - "0" is the only zero; no leading zeros like "01"
    // - "-0" is invalid
    // - otherwise optional '-' followed by digits
    if raw.is_empty() {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw,
            reason: "empty",
        });
    }
    if raw == "-0" {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw,
            reason: "negative zero",
        });
    }
    if raw.starts_with("0") && raw.len() > 1 {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw,
            reason: "leading zero",
        });
    }
    if raw.starts_with("-0") && raw.len() > 2 {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw,
            reason: "leading zero after '-'",
        });
    }

    let val: i64 = raw.parse().map_err(|e| Error::InvalidIntegerValue {
        at: start,
        raw: raw.clone(),
        source: e,
    })?;
    Ok(Value::Integer(val))
}

fn parse_list<'l>(cur: &mut Cursor<'l>, depth: usize) -> Result<Value<'l>> {
    cur.expect_byte(b'l')?;
    let mut items = Vec::new();
    loop {
        match cur.peek() {
            Some(b'e') => {
                cur.next();
                break;
            }
            Some(_) => items.push(parse_value(cur, depth)?),
            None => {
                return Err(Error::MissingTerminator {
                    context: "list",
                    start: cur.offset(),
                });
            }
        }
    }
    Ok(Value::List(items))
}

fn parse_dict<'l>(cur: &mut Cursor<'l>, depth: usize) -> Result<Value<'l>> {
    cur.expect_byte(b'd')?;
    let mut map = HashMap::new();
    loop {
        match cur.peek() {
            Some(b'e') => {
                cur.next();
                break;
            }
            Some(_) => {
                let key_pos = cur.offset();
                let key = parse_string(cur)?;
                let Value::String(k) = key else {
                    return Err(Error::NonStringDictKey { at: key_pos });
                };
                let val = parse_value(cur, depth)?;
                map.insert(k, val);
            }
            None => {
                return Err(Error::MissingTerminator {
                    context: "dict",
                    start: cur.offset(),
                });
            }
        }
    }

    Ok(Value::Dict(map))
}
