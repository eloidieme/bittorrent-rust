use std::collections::HashMap;

use crate::{
    bencode::Value,
    metainfo::error::{Error, Result},
};

pub(crate) fn dict<'l>(
    v: &'l Value<'l>,
    key: &'static str,
) -> Result<&'l HashMap<&'l [u8], Value<'l>>> {
    match v {
        Value::Dict(d) => Ok(d),
        _ => Err(Error::WrongType {
            key: key,
            expected: "dict",
        }),
    }
}

pub(crate) fn get<'l>(
    d: &'l HashMap<&'l [u8], Value<'l>>,
    key: &'static str,
) -> Result<&'l Value<'l>> {
    d.get(key.as_bytes()).ok_or(Error::MissingKey { key })
}

pub(crate) fn maybe<'l>(
    d: &'l HashMap<&'l [u8], Value<'l>>,
    key: &'static str,
) -> Option<&'l Value<'l>> {
    d.get(key.as_bytes())
}

pub(crate) fn as_bytes<'l>(v: &'l Value<'l>, key: &'static str) -> Result<&'l [u8]> {
    match v {
        Value::String(b) => Ok(*b),
        _ => Err(Error::WrongType {
            key,
            expected: "byte string",
        }),
    }
}

pub(crate) fn as_str<'l>(v: &'l Value<'l>, key: &'static str) -> Result<&'l str> {
    let b = as_bytes(v, key)?;
    std::str::from_utf8(b).map_err(|e| Error::Utf8 { key, source: e })
}

pub(crate) fn as_u64(v: &Value<'_>, key: &'static str) -> Result<u64> {
    match v {
        Value::Integer(i) => {
            if *i < 0 {
                Err(Error::Negative { key, got: *i })
            } else {
                Ok(*i as u64)
            }
        }
        _ => Err(Error::WrongType {
            key,
            expected: "integer",
        }),
    }
}

pub(crate) fn as_list<'l>(v: &'l Value<'l>, key: &'static str) -> Result<&'l [Value<'l>]> {
    match v {
        Value::List(vec) => Ok(vec.as_slice()),
        _ => Err(Error::WrongType {
            key,
            expected: "list",
        }),
    }
}

pub(crate) fn read_path<'l>(
    file_dict: &'l HashMap<&'l [u8], Value<'l>>,
    index: usize,
) -> Result<Vec<&'l str>> {
    let v = get(file_dict, "path").map_err(|_| Error::FileMissingKey { index, key: "path" })?;
    let list = as_list(v, "path").map_err(|_| Error::FileWrongType {
        index,
        key: "path",
        expected: "list",
    })?;
    if list.is_empty() {
        return Err(Error::FileEmptyPath { index });
    }

    let mut out = Vec::with_capacity(list.len());
    for (i, comp) in list.iter().enumerate() {
        let b = as_bytes(comp, "path").map_err(|_| Error::FileWrongType {
            index,
            key: "path",
            expected: "list of strings",
        })?;
        if b.is_empty() {
            return Err(Error::FileEmptyPathComponent {
                index,
                component_index: i,
            });
        }
        let s = std::str::from_utf8(b).map_err(|e| Error::FileUtf8 { index, source: e })?;
        out.push(s);
    }

    Ok(out)
}
