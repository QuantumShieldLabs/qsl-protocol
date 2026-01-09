//! Suite-2 ratchet message parsing (strict, fail-closed).

use crate::suite2::types;

pub struct Suite2ParsedRatchetMsg {
    pub dh_pub: [u8; 32],
    pub flags: u16,
    pub pq_prefix: Vec<u8>,
    pub pq_adv_id: Option<u32>,
    pub pq_adv_pub: Option<Vec<u8>>,
    pub pq_target_id: Option<u32>,
    pub pq_ct: Option<Vec<u8>>,
    pub hdr_ct: Vec<u8>,
    pub body_ct: Vec<u8>,
}

fn parse_ratchet_header(
    header: &[u8],
) -> Result<(Suite2ParsedRatchetMsg, usize), &'static str> {
    const HDR_CT_LEN: usize = 24;
    const PQ_ADV_PUB_LEN: usize = 1184;
    const PQ_CT_LEN: usize = 1088;

    if header.len() < 32 + 2 {
        return Err("REJECT_S2_PARSE_PREFIX");
    }

    let mut off = 0usize;
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&header[0..32]);
    off += 32;

    let flags = u16::from_be_bytes([header[off], header[off + 1]]);
    off += 2;

    let known_flags = types::FLAG_PQ_ADV | types::FLAG_PQ_CTXT | types::FLAG_BOUNDARY;
    if (flags & !known_flags) != 0 {
        return Err("REJECT_S2_PARSE_FLAGS");
    }
    if (flags & types::FLAG_PQ_ADV) != 0 && (flags & types::FLAG_BOUNDARY) == 0 {
        return Err("REJECT_S2_PARSE_FLAGS");
    }
    if (flags & types::FLAG_PQ_CTXT) != 0 && (flags & types::FLAG_BOUNDARY) == 0 {
        return Err("REJECT_S2_PARSE_FLAGS");
    }

    let mut pq_prefix = Vec::new();
    let mut pq_adv_id = None;
    let mut pq_adv_pub = None;
    let mut pq_target_id = None;
    let mut pq_ct = None;

    if (flags & types::FLAG_PQ_ADV) != 0 {
        if header.len() < off + 4 + PQ_ADV_PUB_LEN {
            return Err("REJECT_S2_PQPREFIX_PARSE");
        }
        let id = u32::from_be_bytes([header[off], header[off + 1], header[off + 2], header[off + 3]]);
        off += 4;
        let pub_bytes = header[off..off + PQ_ADV_PUB_LEN].to_vec();
        off += PQ_ADV_PUB_LEN;
        pq_prefix.extend_from_slice(&id.to_be_bytes());
        pq_prefix.extend_from_slice(&pub_bytes);
        pq_adv_id = Some(id);
        pq_adv_pub = Some(pub_bytes);
    }

    if (flags & types::FLAG_PQ_CTXT) != 0 {
        if header.len() < off + 4 + PQ_CT_LEN {
            return Err("REJECT_S2_PQPREFIX_PARSE");
        }
        let id = u32::from_be_bytes([header[off], header[off + 1], header[off + 2], header[off + 3]]);
        off += 4;
        let ct = header[off..off + PQ_CT_LEN].to_vec();
        off += PQ_CT_LEN;
        pq_prefix.extend_from_slice(&id.to_be_bytes());
        pq_prefix.extend_from_slice(&ct);
        pq_target_id = Some(id);
        pq_ct = Some(ct);
    }

    if (flags & types::FLAG_PQ_CTXT) != 0 && pq_target_id.is_none() {
        return Err("REJECT_S2_PQPREFIX_PARSE");
    }

    if header.len() < off + HDR_CT_LEN {
        return Err("REJECT_S2_PARSE_HDR_LEN");
    }
    let hdr_ct = header[off..off + HDR_CT_LEN].to_vec();
    off += HDR_CT_LEN;

    Ok((
        Suite2ParsedRatchetMsg {
            dh_pub,
            flags,
            pq_prefix,
            pq_adv_id,
            pq_adv_pub,
            pq_target_id,
            pq_ct,
            hdr_ct,
            body_ct: Vec::new(),
        },
        off,
    ))
}

pub fn decode_suite2_ratchet_message(buf: &[u8]) -> Result<Suite2ParsedRatchetMsg, &'static str> {
    const BODY_CT_MIN: usize = 16;
    let (mut parsed, off) = parse_ratchet_header(buf)?;
    if buf.len() != off {
        if buf.len() < off {
            return Err("REJECT_S2_PARSE_HDR_LEN");
        }
    }
    let body_ct = buf[off..].to_vec();
    if body_ct.len() < BODY_CT_MIN {
        return Err("REJECT_S2_PARSE_BODY_LEN");
    }
    parsed.body_ct = body_ct;
    Ok(parsed)
}

pub fn decode_suite2_wire(buf: &[u8]) -> Result<(u16, u16, u8, Suite2ParsedRatchetMsg), &'static str> {
    const ENVELOPE_HDR_LEN: usize = 10;
    if buf.len() < ENVELOPE_HDR_LEN {
        return Err("REJECT_S2_PARSE_PREFIX");
    }
    let mut off = 0usize;
    let protocol_version = u16::from_be_bytes([buf[0], buf[1]]);
    off += 2;
    let suite_id = u16::from_be_bytes([buf[2], buf[3]]);
    off += 2;
    let msg_type = buf[4];
    off += 1;
    let _env_flags = buf[5];
    off += 1;
    let header_len = u16::from_be_bytes([buf[6], buf[7]]) as usize;
    off += 2;
    let body_len = u16::from_be_bytes([buf[8], buf[9]]) as usize;
    off += 2;

    if protocol_version != types::SUITE2_PROTOCOL_VERSION
        || suite_id != types::SUITE2_SUITE_ID
        || msg_type != 0x02
    {
        return Err("REJECT_S2_PARSE_PREFIX");
    }

    if buf.len() < off + header_len + body_len {
        return Err("REJECT_S2_PARSE_PREFIX");
    }
    let header = &buf[off..off + header_len];
    off += header_len;
    let body = &buf[off..off + body_len];
    if off + body_len != buf.len() {
        return Err("REJECT_S2_PARSE_PREFIX");
    }

    let (mut parsed, used) = parse_ratchet_header(header)?;
    if used != header.len() {
        return Err("REJECT_S2_PARSE_HDR_LEN");
    }
    if body.len() < 16 {
        return Err("REJECT_S2_PARSE_BODY_LEN");
    }
    parsed.body_ct = body.to_vec();

    Ok((protocol_version, suite_id, msg_type, parsed))
}
