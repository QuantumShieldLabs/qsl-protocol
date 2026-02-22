use crate::codec::{CodecError, Reader, Writer};
use crate::qsp::ProtocolMessage;

pub const QSE_ENV_VERSION_V1: u16 = 0x0100;
const FLAG_BUCKET_PADDED: u16 = 0x0001;

#[derive(Debug, Clone, Copy)]
pub enum EnvelopeProfile {
    Standard,
    Enhanced,
    Private,
}

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
    pub route_token: Vec<u8>,  // varbytes<u16>
    pub timestamp_bucket: u32, // set by service edge (normative)
    pub payload: Vec<u8>,      // QSP wire bytes
    pub padding: Vec<u8>,      // random bytes; contents ignored
}

impl Envelope {
    fn is_bucket_padded(&self) -> bool {
        (self.flags & FLAG_BUCKET_PADDED) != 0
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_u16(self.env_version);
        w.write_u16(self.flags);
        w.write_varbytes_u16(&self.route_token);
        w.write_u32(self.timestamp_bucket);
        if self.is_bucket_padded() {
            // Bucket mode must not expose precise payload/padding lengths.
            w.write_u16(0);
            w.write_u32(0);
        } else {
            w.write_u16(self.padding.len() as u16);
            w.write_u32(self.payload.len() as u32);
        }
        w.write_bytes(&self.payload);
        w.write_bytes(&self.padding);
        w.into_vec()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, CodecError> {
        let mut r = Reader::new(buf);
        let env_version = r.read_u16()?;
        let flags = r.read_u16()?;
        if env_version != QSE_ENV_VERSION_V1 {
            return Err(CodecError::Invalid("env_version"));
        }
        // QSE v1.x: unknown flags must be rejected
        if flags & !FLAG_BUCKET_PADDED != 0 {
            return Err(CodecError::Invalid("flags"));
        }
        let route_token = r.read_varbytes_u16()?;
        let timestamp_bucket = r.read_u32()?;
        let pad_len = r.read_u16()? as usize;
        let payload_len = r.read_u32()? as usize;
        let bucket_padded = (flags & FLAG_BUCKET_PADDED) != 0;
        if bucket_padded {
            // Bucket mode keeps length fields constant to avoid leaking exact sizes.
            if pad_len != 0 || payload_len != 0 {
                return Err(CodecError::Invalid("bucket_len_fields"));
            }
            let remaining = r.read_bytes(r.remaining())?;
            let payload_len = locate_protocol_message_prefix_len(&remaining)?;
            let (payload, padding) = remaining.split_at(payload_len);
            Ok(Self {
                env_version,
                flags,
                route_token,
                timestamp_bucket,
                payload: payload.to_vec(),
                padding: padding.to_vec(),
            })
        } else {
            if r.remaining() < payload_len + pad_len {
                return Err(CodecError::LengthOutOfRange);
            }
            let payload = r.read_bytes(payload_len)?;
            let padding = r.read_bytes(pad_len)?;
            r.finish()?;
            Ok(Self {
                env_version,
                flags,
                route_token,
                timestamp_bucket,
                payload,
                padding,
            })
        }
    }

    /// Apply padding to meet the profile's minimum envelope size (QSE §6).
    /// `rng_bytes` should be random bytes; its length must be at least the required padding length.
    pub fn pad_to_profile(
        mut self,
        profile: EnvelopeProfile,
        rng_bytes: &[u8],
    ) -> Result<Self, CodecError> {
        self.flags |= FLAG_BUCKET_PADDED;
        let encoded_len = self.encode().len();
        let min_len = profile.min_size_bytes();
        if encoded_len >= min_len {
            return Ok(self);
        }
        let need = min_len - encoded_len;
        if rng_bytes.len() < need {
            return Err(CodecError::Invalid("insufficient rng padding bytes"));
        }
        self.padding.extend_from_slice(&rng_bytes[..need]);
        Ok(self)
    }
}

fn locate_protocol_message_prefix_len(buf: &[u8]) -> Result<usize, CodecError> {
    for payload_len in 1..=buf.len() {
        if ProtocolMessage::decode(&buf[..payload_len]).is_ok() {
            return Ok(payload_len);
        }
    }
    Err(CodecError::Invalid("payload_boundary_not_found"))
}
