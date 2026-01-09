use crate::codec::{Reader, Writer, CodecError};
use super::constants::*;

#[derive(Debug, Clone)]
pub struct PrekeyBundle {
    pub user_id: Vec<u8>,          // varbytes<u16>
    pub device_id: u32,
    pub valid_from: u32,
    pub valid_to: u32,
    pub ik_sig_ec_pub: [u8; SZ_ED25519_PUB],
    pub ik_sig_pq_pub: Vec<u8>,    // SZ_MLDSA65_PUB
    pub spk_dh_pub: [u8; SZ_X25519_PUB],
    pub spk_pq_pub: Vec<u8>,       // SZ_MLKEM768_PUB
    pub pq_rcv_id: u32,
    pub pq_rcv_pub: Vec<u8>,       // SZ_MLKEM768_PUB
    pub opk_dh: Option<(u32, [u8; SZ_X25519_PUB])>,
    pub opk_pq: Option<(u32, Vec<u8>)>, // id + SZ_MLKEM768_PUB
    pub sig_ec: Vec<u8>,           // 64
    pub sig_pq: Vec<u8>,           // 3309
    pub kt_log_id: [u8; 32],
    pub kt_sth: Vec<u8>,
    pub kt_inclusion_proof: Vec<u8>,
    pub kt_consistency_proof: Vec<u8>,
}

impl PrekeyBundle {
    /// Canonical encoding (QSP §4.3). Used for signature verification and vector stability.
    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_varbytes_u16(&self.user_id);
        w.write_u32(self.device_id);
        w.write_u32(self.valid_from);
        w.write_u32(self.valid_to);
        w.write_bytes(&self.ik_sig_ec_pub);
        w.write_bytes(&self.ik_sig_pq_pub);
        w.write_bytes(&self.spk_dh_pub);
        w.write_bytes(&self.spk_pq_pub);
        w.write_u32(self.pq_rcv_id);
        w.write_bytes(&self.pq_rcv_pub);

        w.write_u16(self.opk_dh.is_some() as u16);
        if let Some((id, pubk)) = &self.opk_dh {
            w.write_u32(*id);
            w.write_bytes(pubk);
        }
        w.write_u16(self.opk_pq.is_some() as u16);
        if let Some((id, pubk)) = &self.opk_pq {
            w.write_u32(*id);
            w.write_bytes(pubk);
        }

        w.write_bytes(&self.sig_ec);
        w.write_bytes(&self.sig_pq);
        w.write_bytes(&self.kt_log_id);
        w.write_varbytes_u16(&self.kt_sth);
        w.write_varbytes_u16(&self.kt_inclusion_proof);
        w.write_varbytes_u16(&self.kt_consistency_proof);
        w.into_vec()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, CodecError> {
        let mut r = Reader::new(buf);
        let user_id = r.read_varbytes_u16()?;
        let device_id = r.read_u32()?;
        let valid_from = r.read_u32()?;
        let valid_to = r.read_u32()?;
        let ik_sig_ec_pub = r.read_exact::<SZ_ED25519_PUB>()?;
        let ik_sig_pq_pub = r.read_bytes(SZ_MLDSA65_PUB)?;
        let spk_dh_pub = r.read_exact::<SZ_X25519_PUB>()?;
        let spk_pq_pub = r.read_bytes(SZ_MLKEM768_PUB)?;
        let pq_rcv_id = r.read_u32()?;
        let pq_rcv_pub = r.read_bytes(SZ_MLKEM768_PUB)?;

        let opk_dh_present = r.read_u16()? != 0;
        let opk_dh = if opk_dh_present {
            let id = r.read_u32()?;
            let pubk = r.read_exact::<SZ_X25519_PUB>()?;
            Some((id, pubk))
        } else { None };

        let opk_pq_present = r.read_u16()? != 0;
        let opk_pq = if opk_pq_present {
            let id = r.read_u32()?;
            let pubk = r.read_bytes(SZ_MLKEM768_PUB)?;
            Some((id, pubk))
        } else { None };

        let sig_ec = r.read_bytes(SZ_ED25519_SIG)?;
        let sig_pq = r.read_bytes(SZ_MLDSA65_SIG)?;
        let kt_log_id = r.read_exact::<32>()?;
        let kt_sth = r.read_varbytes_u16()?;
        let kt_inclusion_proof = r.read_varbytes_u16()?;
        let kt_consistency_proof = r.read_varbytes_u16()?;
        r.finish()?;

        Ok(Self {
            user_id, device_id, valid_from, valid_to,
            ik_sig_ec_pub, ik_sig_pq_pub, spk_dh_pub, spk_pq_pub,
            pq_rcv_id, pq_rcv_pub, opk_dh, opk_pq,
            sig_ec, sig_pq, kt_log_id, kt_sth, kt_inclusion_proof, kt_consistency_proof,
        })
    }
}

#[derive(Debug, Clone)]
pub struct HandshakeInit {
    pub protocol_version: u16,
    pub suite_id: u16,
    pub session_id: [u8; SZ_SESSION_ID],
    pub user_id_b: Vec<u8>,
    pub device_id_b: u32,
    pub ek_dh_a_pub: [u8; SZ_X25519_PUB],
    pub ct1: Vec<u8>,              // 1088
    pub opk_used: bool,
    pub ct2: Option<Vec<u8>>,      // 1088
    pub opk_dh_id: Option<u32>,
    pub opk_pq_id: Option<u32>,
    pub pq_rcv_a_id: u32,
    pub pq_rcv_a_pub: Vec<u8>,     // 1184
    pub ik_sig_ec_a_pub: [u8; SZ_ED25519_PUB],
    pub ik_sig_pq_a_pub: Vec<u8>,  // 1952
    pub sig_ec_a: Vec<u8>,         // 64
    pub sig_pq_a: Vec<u8>,         // 3309
}

impl HandshakeInit {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_u16(self.protocol_version);
        w.write_u16(self.suite_id);
        w.write_bytes(&self.session_id);
        w.write_varbytes_u16(&self.user_id_b);
        w.write_u32(self.device_id_b);
        w.write_bytes(&self.ek_dh_a_pub);
        w.write_bytes(&self.ct1);
        w.write_u16(self.opk_used as u16);
        if self.opk_used {
            w.write_bytes(self.ct2.as_ref().expect("ct2"));
            w.write_u32(self.opk_dh_id.expect("opk_dh_id"));
            w.write_u32(self.opk_pq_id.expect("opk_pq_id"));
        }
        w.write_u32(self.pq_rcv_a_id);
        w.write_bytes(&self.pq_rcv_a_pub);
        w.write_bytes(&self.ik_sig_ec_a_pub);
        w.write_bytes(&self.ik_sig_pq_a_pub);
        w.write_bytes(&self.sig_ec_a);
        w.write_bytes(&self.sig_pq_a);
        w.into_vec()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, CodecError> {
        let mut r = Reader::new(buf);
        let protocol_version = r.read_u16()?;
        let suite_id = r.read_u16()?;
        if protocol_version != QSP_PROTOCOL_VERSION { return Err(CodecError::Invalid("protocol_version")); }
        if suite_id != QSP_SUITE_ID { return Err(CodecError::Invalid("suite_id")); }
        let session_id = r.read_exact::<SZ_SESSION_ID>()?;
        let user_id_b = r.read_varbytes_u16()?;
        let device_id_b = r.read_u32()?;
        let ek_dh_a_pub = r.read_exact::<SZ_X25519_PUB>()?;
        let ct1 = r.read_bytes(SZ_MLKEM768_CT)?;
        let opk_used = r.read_u16()? != 0;
        let (ct2, opk_dh_id, opk_pq_id) = if opk_used {
            let ct2 = r.read_bytes(SZ_MLKEM768_CT)?;
            let opk_dh_id = r.read_u32()?;
            let opk_pq_id = r.read_u32()?;
            (Some(ct2), Some(opk_dh_id), Some(opk_pq_id))
        } else { (None, None, None) };
        let pq_rcv_a_id = r.read_u32()?;
        let pq_rcv_a_pub = r.read_bytes(SZ_MLKEM768_PUB)?;
        let ik_sig_ec_a_pub = r.read_exact::<SZ_ED25519_PUB>()?;
        let ik_sig_pq_a_pub = r.read_bytes(SZ_MLDSA65_PUB)?;
        let sig_ec_a = r.read_bytes(SZ_ED25519_SIG)?;
        let sig_pq_a = r.read_bytes(SZ_MLDSA65_SIG)?;
        r.finish()?;
        Ok(Self {
            protocol_version, suite_id, session_id, user_id_b, device_id_b,
            ek_dh_a_pub, ct1, opk_used, ct2, opk_dh_id, opk_pq_id,
            pq_rcv_a_id, pq_rcv_a_pub, ik_sig_ec_a_pub, ik_sig_pq_a_pub, sig_ec_a, sig_pq_a
        })
    }

    /// HS1 transcript per QSP §5.2.1: SHA-512("QSP4.3/HS1" || HS1_input)
    /// where HS1_input is HandshakeInit with signature fields set to zero bytes.
    pub fn hs1_transcript(&self, hash: &dyn crate::crypto::traits::Hash) -> [u8; 64] {
        let mut tmp = self.clone();
        tmp.sig_ec_a = vec![0u8; SZ_ED25519_SIG];
        tmp.sig_pq_a = vec![0u8; SZ_MLDSA65_SIG];
        let mut m = b"QSP4.3/HS1".to_vec();
        m.extend_from_slice(&tmp.encode());
        hash.sha512(&m)
    }
}

#[derive(Debug, Clone)]
pub struct HandshakeResp {
    pub protocol_version: u16,
    pub suite_id: u16,
    pub session_id: [u8; SZ_SESSION_ID],
    pub dh0_b_pub: [u8; SZ_X25519_PUB],
    pub pq_rcv_b_id: u32,
    pub pq_rcv_b_pub: Vec<u8>,     // 1184
    pub ct3: Vec<u8>,              // 1088 (encap to PQ_RCV_A_pub)
    pub conf_b: [u8; 32],
    pub ik_sig_ec_b_pub: [u8; SZ_ED25519_PUB],
    pub ik_sig_pq_b_pub: Vec<u8>,  // 1952
    pub sig_ec_b: Vec<u8>,         // 64
    pub sig_pq_b: Vec<u8>,         // 3309
}

impl HandshakeResp {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_u16(self.protocol_version);
        w.write_u16(self.suite_id);
        w.write_bytes(&self.session_id);
        w.write_bytes(&self.dh0_b_pub);
        w.write_u32(self.pq_rcv_b_id);
        w.write_bytes(&self.pq_rcv_b_pub);
        w.write_bytes(&self.ct3);
        w.write_bytes(&self.conf_b);
        w.write_bytes(&self.ik_sig_ec_b_pub);
        w.write_bytes(&self.ik_sig_pq_b_pub);
        w.write_bytes(&self.sig_ec_b);
        w.write_bytes(&self.sig_pq_b);
        w.into_vec()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, CodecError> {
        let mut r = Reader::new(buf);
        let protocol_version = r.read_u16()?;
        let suite_id = r.read_u16()?;
        if protocol_version != QSP_PROTOCOL_VERSION { return Err(CodecError::Invalid("protocol_version")); }
        if suite_id != QSP_SUITE_ID { return Err(CodecError::Invalid("suite_id")); }
        let session_id = r.read_exact::<SZ_SESSION_ID>()?;
        let dh0_b_pub = r.read_exact::<SZ_X25519_PUB>()?;
        let pq_rcv_b_id = r.read_u32()?;
        let pq_rcv_b_pub = r.read_bytes(SZ_MLKEM768_PUB)?;
        let ct3 = r.read_bytes(SZ_MLKEM768_CT)?;
        let conf_b = r.read_exact::<32>()?;
        let ik_sig_ec_b_pub = r.read_exact::<SZ_ED25519_PUB>()?;
        let ik_sig_pq_b_pub = r.read_bytes(SZ_MLDSA65_PUB)?;
        let sig_ec_b = r.read_bytes(SZ_ED25519_SIG)?;
        let sig_pq_b = r.read_bytes(SZ_MLDSA65_SIG)?;
        r.finish()?;
        Ok(Self {
            protocol_version, suite_id, session_id, dh0_b_pub, pq_rcv_b_id, pq_rcv_b_pub,
            ct3, conf_b, ik_sig_ec_b_pub, ik_sig_pq_b_pub, sig_ec_b, sig_pq_b
        })
    }

    /// HS2 transcript per QSP §5.3.1: SHA-512("QSP4.3/HS2" || HandshakeInit || HS2_input)
    /// where HS2_input is HandshakeResp with conf_b and signature fields set to zero bytes.
    pub fn hs2_transcript(&self, hs1: &HandshakeInit, hash: &dyn crate::crypto::traits::Hash) -> [u8; 64] {
        let mut tmp = self.clone();
        tmp.conf_b.fill(0);
        tmp.sig_ec_b = vec![0u8; SZ_ED25519_SIG];
        tmp.sig_pq_b = vec![0u8; SZ_MLDSA65_SIG];
        let mut m = b"QSP4.3/HS2".to_vec();
        m.extend_from_slice(&hs1.encode());
        m.extend_from_slice(&tmp.encode());
        hash.sha512(&m)
    }
}

#[derive(Debug, Clone)]
pub struct ProtocolMessage {
    pub protocol_version: u16,
    pub suite_id: u16,
    pub session_id: [u8; SZ_SESSION_ID],
    pub dh_pub: [u8; SZ_X25519_PUB],
    pub flags: u16,
    pub nonce_hdr: [u8; SZ_NONCE],
    pub pq_adv_id: Option<u32>,
    pub pq_adv_pub: Option<Vec<u8>>,  // 1184
    pub pq_target_id: Option<u32>,
    pub pq_ct: Option<Vec<u8>>,       // 1088
    pub hdr_ct: Vec<u8>,
    pub body_ct: Vec<u8>,
}

impl ProtocolMessage {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_u16(self.protocol_version);
        w.write_u16(self.suite_id);
        w.write_bytes(&self.session_id);
        w.write_bytes(&self.dh_pub);
        w.write_u16(self.flags);
        w.write_bytes(&self.nonce_hdr);

        if (self.flags & FLAG_PQ_ADV) != 0 {
            w.write_u32(self.pq_adv_id.expect("pq_adv_id"));
            w.write_bytes(self.pq_adv_pub.as_ref().expect("pq_adv_pub"));
        }
        if (self.flags & FLAG_PQ_CTXT) != 0 {
            w.write_u32(self.pq_target_id.expect("pq_target_id"));
            w.write_bytes(self.pq_ct.as_ref().expect("pq_ct"));
        }

        w.write_u16(self.hdr_ct.len() as u16);
        w.write_bytes(&self.hdr_ct);
        w.write_u32(self.body_ct.len() as u32);
        w.write_bytes(&self.body_ct);
        w.into_vec()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, CodecError> {
        let mut r = Reader::new(buf);
        let protocol_version = r.read_u16()?;
        let suite_id = r.read_u16()?;
        if protocol_version != QSP_PROTOCOL_VERSION { return Err(CodecError::Invalid("protocol_version")); }
        if suite_id != QSP_SUITE_ID { return Err(CodecError::Invalid("suite_id")); }
        let session_id = r.read_exact::<SZ_SESSION_ID>()?;
        let dh_pub = r.read_exact::<SZ_X25519_PUB>()?;
        let flags = r.read_u16()?;
        // Unknown flags must be rejected (QSP §6.3, §2.4). Only bits 0x0001/0x0002/0x0004 are defined.
        if flags & !(FLAG_PQ_ADV | FLAG_PQ_CTXT | FLAG_BOUNDARY) != 0 { return Err(CodecError::Invalid("flags")); }
        let nonce_hdr = r.read_exact::<SZ_NONCE>()?;

        let (pq_adv_id, pq_adv_pub) = if (flags & FLAG_PQ_ADV) != 0 {
            let id = r.read_u32()?;
            let pubk = r.read_bytes(SZ_MLKEM768_PUB)?;
            (Some(id), Some(pubk))
        } else { (None, None) };

        let (pq_target_id, pq_ct) = if (flags & FLAG_PQ_CTXT) != 0 {
            let id = r.read_u32()?;
            let ct = r.read_bytes(SZ_MLKEM768_CT)?;
            (Some(id), Some(ct))
        } else { (None, None) };

        let hdr_ct_len = r.read_u16()? as usize;
        const QSP_HDR_CT_LEN: usize = 24;
        if hdr_ct_len != QSP_HDR_CT_LEN { return Err(CodecError::Invalid("hdr_ct_len")); }
        if r.remaining() < hdr_ct_len { return Err(CodecError::LengthOutOfRange); }
        let hdr_ct = r.read_bytes(hdr_ct_len)?;
        let body_ct_len = r.read_u32()? as usize;
        const QSP_BODY_CT_MIN: usize = 16;
        if body_ct_len < QSP_BODY_CT_MIN { return Err(CodecError::Invalid("body_ct_len")); }
        if r.remaining() < body_ct_len { return Err(CodecError::LengthOutOfRange); }
        let body_ct = r.read_bytes(body_ct_len)?;
        r.finish()?;

        Ok(Self { protocol_version, suite_id, session_id, dh_pub, flags, nonce_hdr, pq_adv_id, pq_adv_pub, pq_target_id, pq_ct, hdr_ct, body_ct })
    }
}
