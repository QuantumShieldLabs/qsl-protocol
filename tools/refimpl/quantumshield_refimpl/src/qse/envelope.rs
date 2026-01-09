use crate::codec::{Reader, Writer, CodecError};

pub const QSE_ENV_VERSION_V1: u16 = 0x0100;

#[derive(Debug, Clone, Copy)]
pub enum EnvelopeProfile { Standard, Enhanced, Private }

impl EnvelopeProfile {
    pub fn min_size_bytes(&self) -> usize {
        match self {
            EnvelopeProfile::Standard => 1024,
            EnvelopeProfile::Enhanced => 2048,
            EnvelopeProfile::Private => 4096,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Envelope {
    pub env_version: u16,
    pub flags: u16,
    pub route_token: Vec<u8>,      // varbytes<u16>
    pub timestamp_bucket: u32,     // set by service edge (normative)
    pub payload: Vec<u8>,          // QSP wire bytes
    pub padding: Vec<u8>,          // random bytes; contents ignored
}

impl Envelope {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_u16(self.env_version);
        w.write_u16(self.flags);
        w.write_varbytes_u16(&self.route_token);
        w.write_u32(self.timestamp_bucket);
        w.write_u16(self.padding.len() as u16);
        w.write_u32(self.payload.len() as u32);
        w.write_bytes(&self.payload);
        w.write_bytes(&self.padding);
        w.into_vec()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, CodecError> {
        let mut r = Reader::new(buf);
        let env_version = r.read_u16()?;
        let flags = r.read_u16()?;
        if env_version != QSE_ENV_VERSION_V1 { return Err(CodecError::Invalid("env_version")); }
        // QSE v1.x: unknown flags must be rejected
        if flags != 0 { return Err(CodecError::Invalid("flags")); }
        let route_token = r.read_varbytes_u16()?;
        let timestamp_bucket = r.read_u32()?;
        let pad_len = r.read_u16()? as usize;
        let payload_len = r.read_u32()? as usize;
        if r.remaining() < payload_len + pad_len { return Err(CodecError::LengthOutOfRange); }
        let payload = r.read_bytes(payload_len)?;
        let padding = r.read_bytes(pad_len)?;
        r.finish()?;
        Ok(Self { env_version, flags, route_token, timestamp_bucket, payload, padding })
    }

    /// Apply padding to meet the profile's minimum envelope size (QSE §6).
    /// `rng_bytes` should be random bytes; its length must be at least the required padding length.
    pub fn pad_to_profile(mut self, profile: EnvelopeProfile, rng_bytes: &[u8]) -> Result<Self, CodecError> {
        let encoded_len = self.encode().len();
        let min_len = profile.min_size_bytes();
        if encoded_len >= min_len { return Ok(self); }
        let need = min_len - encoded_len;
        if rng_bytes.len() < need { return Err(CodecError::Invalid("insufficient rng padding bytes")); }
        self.padding.extend_from_slice(&rng_bytes[..need]);
        Ok(self)
    }
}
