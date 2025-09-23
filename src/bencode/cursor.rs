use crate::bencode::error::{Error, Offset, Result};

#[derive(Debug)]
pub struct Cursor<'l> {
    pub buf: &'l [u8],
    pub pos: usize,
}

impl<'l> Cursor<'l> {
    pub fn new(buf: &'l [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn rest(&self) -> &'l [u8] {
        &self.buf[self.pos..]
    }

    pub fn offset(&self) -> Offset {
        Offset(self.pos)
    }

    pub fn peek(&self) -> Option<u8> {
        self.buf.get(self.pos).copied()
    }

    pub fn advance(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    pub fn expect_byte(&mut self, expected: u8) -> Result<()> {
        match self.advance() {
            Some(b) if b == expected => Ok(()),
            Some(found) => Err(Error::UnexpectedByte {
                at: self.offset_minus(1),
                found,
                expected: expected_desc(expected),
            }),
            None => Err(Error::UnexpectedEof { at: self.offset() }),
        }
    }

    pub fn take(&mut self, n: usize) -> Result<&'l [u8]> {
        if self.pos + n > self.buf.len() {
            return Err(Error::UnexpectedEof { at: self.offset() });
        }
        let start = self.pos;
        self.pos += n;
        Ok(&self.buf[start..self.pos])
    }

    fn offset_minus(&self, n: usize) -> Offset {
        Offset(self.pos.saturating_sub(n))
    }
}

fn expected_desc(b: u8) -> &'static str {
    match b {
        b':' => "':'",
        b'e' => "'e'",
        b'i' => "'i'",
        b'l' => "'l'",
        b'd' => "'d'",
        _ => "specific byte",
    }
}
