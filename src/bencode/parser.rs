use std::{collections::HashMap, ops::Range};

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

pub fn extract_dict_value_range<'l>(cur: &mut Cursor<'l>, key: &'l str) -> Result<Range<usize>> {
    let saved_pos = cur.pos;
    cur.expect_byte(b'd')?;
    let key_bytes = key.as_bytes();
    let start;
    let end;

    loop {
        match cur.peek() {
            Some(b'e') => {
                cur.advance();
                cur.pos = saved_pos;
                return Err(Error::DictKeyNotFound {
                    at: Offset(saved_pos),
                    key: key.to_string(),
                });
            }
            Some(_) => {
                let Value::String(k) = parse_string(cur)? else {
                    let at = cur.offset();
                    cur.pos = saved_pos;
                    return Err(Error::NonStringDictKey { at });
                };
                if k == key_bytes {
                    start = cur.pos;
                    parse_value(cur, 0)?;
                    end = cur.pos;
                    break;
                } else {
                    parse_value(cur, 0)?;
                }
            }
            None => {
                cur.pos = saved_pos;
                return Err(Error::MissingTerminator {
                    context: "dict",
                    start: Offset(saved_pos),
                });
            }
        }
    }

    cur.pos = saved_pos;
    Ok(start..end)
}

fn parse_string<'l>(cur: &mut Cursor<'l>) -> Result<Value<'l>> {
    let start = cur.offset();
    let mut i = 0usize;
    let mut len: usize = 0;
    let mut seen_digit = false;

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
        seen_digit = true;
        len = len
            .checked_mul(10)
            .and_then(|v| v.checked_add((b - b'0') as usize))
            .ok_or_else(|| Error::InvalidStringLength {
                at: start,
                raw: String::from_utf8_lossy(&cur.buf[cur.pos..cur.pos + i + 1]).into_owned(),
                source: "18446744073709551616".parse::<usize>().unwrap_err(),
            })?;
        i += 1;
    }

    if !seen_digit || cur.buf.get(cur.pos + i).copied() != Some(b':') {
        return Err(Error::UnexpectedEof { at: cur.offset() });
    }
    cur.pos += i + 1; // skip header and ':'

    let remaining = cur.buf.len().saturating_sub(cur.pos);
    if remaining < len {
        return Err(Error::InsufficientStringBytes {
            at: cur.offset(),
            expected: len,
            available: remaining,
        });
    }
    let s = &cur.buf[cur.pos..cur.pos + len];
    cur.pos += len;
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
    let raw = &cur.buf[cur.pos..cur.pos + i];
    cur.pos += i + 1; // skip payload + 'e'

    // syntax policy checks:
    if raw.is_empty() {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw: "".into(),
            reason: "empty",
        });
    }
    let (neg, digits) = if raw[0] == b'-' {
        (true, &raw[1..])
    } else {
        (false, raw)
    };
    if digits.is_empty() {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw: "-".into(),
            reason: "no digits",
        });
    }
    if neg {
        if digits == b"0" {
            return Err(Error::InvalidIntegerSyntax {
                at: start,
                raw: String::from_utf8_lossy(raw).into_owned(),
                reason: "negative zero",
            });
        } else if digits[0] == b'0' {
            return Err(Error::InvalidIntegerSyntax {
                at: start,
                raw: String::from_utf8_lossy(raw).into_owned(),
                reason: "leading zero after '-'",
            });
        }
    } else if digits[0] == b'0' && digits.len() > 1 {
        return Err(Error::InvalidIntegerSyntax {
            at: start,
            raw: String::from_utf8_lossy(raw).into_owned(),
            reason: "leading zero",
        });
    }
    // Special case for i64::MIN
    if neg && digits == b"9223372036854775808" {
        return Ok(Value::Integer(i64::MIN));
    }

    let mut val: i64 = 0;
    for &d in digits {
        if !d.is_ascii_digit() {
            return Err(Error::InvalidIntegerSyntax {
                at: start,
                raw: String::from_utf8_lossy(raw).into_owned(),
                reason: "non-digit",
            });
        }
        let digit = (d - b'0') as i64;
        val = val
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
            .ok_or_else(|| Error::InvalidIntegerValue {
                at: start,
                raw: String::from_utf8_lossy(raw).into_owned(),
                source: "overflow".parse::<i64>().unwrap_err(),
            })?;
    }
    if neg {
        val = -val;
    }
    Ok(Value::Integer(val))
}

fn parse_list<'l>(cur: &mut Cursor<'l>, depth: usize) -> Result<Value<'l>> {
    cur.expect_byte(b'l')?;
    let mut items = Vec::new();
    loop {
        match cur.peek() {
            Some(b'e') => {
                cur.advance();
                break;
            }
            Some(_) => items.push(parse_value(cur, depth + 1)?),
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
                cur.advance();
                break;
            }
            Some(_) => {
                let key_pos = cur.offset();
                let key = parse_string(cur)?;
                let Value::String(k) = key else {
                    return Err(Error::NonStringDictKey { at: key_pos });
                };
                let val = parse_value(cur, depth + 1)?;
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
