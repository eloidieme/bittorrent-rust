use crate::bencode::Value;
use std::fmt;

impl<'l> fmt::Debug for Value<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", String::from_utf8_lossy(s)),
            Value::Integer(i) => write!(f, "\"{i}\""),
            Value::List(l) => f.debug_list().entries(l.iter()).finish(),
            Value::Dict(d) => {
                let mut dbg = f.debug_map();
                for (k, v) in d {
                    dbg.entry(&String::from_utf8_lossy(k), v);
                }
                dbg.finish()
            }
        }
    }
}

impl<'l> fmt::Display for Value<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", String::from_utf8_lossy(s)),
            Value::Integer(i) => write!(f, "{i}"),
            Value::List(l) => write!(f, "List(len={})", l.len()),
            Value::Dict(d) => write!(f, "Dict(len={})", d.len()),
        }
    }
}
