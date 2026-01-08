//! Canonical encoding/parsing (QSP §2, QSE §2).
//!
//! - Unsigned integers are big-endian.
//! - varbytes<u16> = u16 len || len bytes
//! - varbytes<u32> = u32 len || len bytes
//! - trailing bytes are rejected by message-specific decoders.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("truncated input")]
    Truncated,
    #[error("length exceeds remaining bytes")]
    LengthOutOfRange,
    #[error("trailing bytes not permitted")]
    TrailingBytes,
    #[error("invalid value: {0}")]
    Invalid(&'static str),
}

#[derive(Clone)]
pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self { Self { buf, pos: 0 } }
    pub fn remaining(&self) -> usize { self.buf.len().saturating_sub(self.pos) }

    fn take(&mut self, n: usize) -> Result<&'a [u8], CodecError> {
        if self.remaining() < n { return Err(CodecError::Truncated); }
        let s = &self.buf[self.pos..self.pos+n];
        self.pos += n;
        Ok(s)
    }

    pub fn read_u16(&mut self) -> Result<u16, CodecError> {
        let b = self.take(2)?;
        Ok(u16::from_be_bytes([b[0], b[1]]))
    }
    pub fn read_u32(&mut self) -> Result<u32, CodecError> {
        let b = self.take(4)?;
        Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
    pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, CodecError> {
        Ok(self.take(n)?.to_vec())
    }
    pub fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], CodecError> {
        let b = self.take(N)?;
        let mut out = [0u8; N];
        out.copy_from_slice(b);
        Ok(out)
    }
    pub fn read_varbytes_u16(&mut self) -> Result<Vec<u8>, CodecError> {
        let len = self.read_u16()? as usize;
        if self.remaining() < len { return Err(CodecError::LengthOutOfRange); }
        self.read_bytes(len)
    }
    pub fn read_varbytes_u32(&mut self) -> Result<Vec<u8>, CodecError> {
        let len = self.read_u32()? as usize;
        if self.remaining() < len { return Err(CodecError::LengthOutOfRange); }
        self.read_bytes(len)
    }
    pub fn finish(&self) -> Result<(), CodecError> {
        if self.remaining() != 0 { Err(CodecError::TrailingBytes) } else { Ok(()) }
    }
}

#[derive(Default, Clone)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self { Self { buf: Vec::new() } }
    pub fn into_vec(self) -> Vec<u8> { self.buf }
    pub fn write_u16(&mut self, v: u16) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub fn write_u32(&mut self, v: u32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub fn write_bytes(&mut self, b: &[u8]) { self.buf.extend_from_slice(b); }
    pub fn write_varbytes_u16(&mut self, b: &[u8]) {
        self.write_u16(b.len() as u16);
        self.write_bytes(b);
    }
    pub fn write_varbytes_u32(&mut self, b: &[u8]) {
        self.write_u32(b.len() as u32);
        self.write_bytes(b);
    }
}
