//! Suite-2 SCKA integration surface (minimal reseed helper).

use std::collections::BTreeSet;

use crate::crypto::traits::{Hash, Kmac};

#[derive(Debug)]
pub enum Suite2Reject {
    Code(&'static str),
}

pub struct ApplyReseedOut {
    pub ck_pq_seed_a2b: [u8; 32],
    pub ck_pq_seed_b2a: [u8; 32],
    pub ck_pq_send_after: [u8; 32],
    pub ck_pq_recv_after: [u8; 32],
    pub peer_max_adv_id_seen_after: u32,
    pub consumed_targets_after: BTreeSet<u32>,
    pub tombstoned_targets_after: BTreeSet<u32>,
}

fn kmac32(kmac: &dyn Kmac, key: &[u8], label: &str, data: &[u8]) -> [u8; 32] {
    let out = kmac.kmac256(key, label, data, 32);
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out);
    arr
}

/// Apply PQ reseed derivation and SCKA checks for a single ciphertext event.
pub fn apply_pq_reseed(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    role_is_a: bool,
    rk: &[u8; 32],
    pq_ct: &[u8],
    pq_epoch_ss: &[u8],
    peer_adv_id: u32,
    peer_max_adv_id_seen: u32,
    known_targets: &BTreeSet<u32>,
    consumed_targets: &BTreeSet<u32>,
    tombstoned_targets: &BTreeSet<u32>,
    pq_target_id: u32,
    commit: bool,
    ck_pq_send: &[u8; 32],
    ck_pq_recv: &[u8; 32],
) -> Result<ApplyReseedOut, Suite2Reject> {
    const MLKEM768_CT_LEN: usize = 1088;
    if pq_ct.len() != MLKEM768_CT_LEN {
        return Err(Suite2Reject::Code("REJECT_SCKA_CTXT_BAD_CT_LEN"));
    }
    if peer_adv_id <= peer_max_adv_id_seen {
        return Err(Suite2Reject::Code("REJECT_SCKA_ADV_NONMONOTONIC"));
    }
    if tombstoned_targets.contains(&pq_target_id) {
        return Err(Suite2Reject::Code("REJECT_SCKA_TARGET_TOMBSTONED"));
    }
    if !known_targets.contains(&pq_target_id) {
        return Err(Suite2Reject::Code("REJECT_SCKA_TARGET_UNKNOWN"));
    }
    if consumed_targets.contains(&pq_target_id) {
        return Err(Suite2Reject::Code("REJECT_SCKA_TARGET_CONSUMED"));
    }

    let h = hash.sha512(pq_ct);
    let ct_hash = &h[0..32];
    let mut ctx = Vec::new();
    ctx.extend_from_slice(b"QSP5.0/SCKA/CTXT");
    ctx.extend_from_slice(&pq_target_id.to_be_bytes());
    ctx.extend_from_slice(ct_hash);
    ctx.extend_from_slice(pq_epoch_ss);

    let ck_pq_seed_a2b = kmac32(kmac, rk, "QSP5.0/PQSEED/A->B", &ctx);
    let ck_pq_seed_b2a = kmac32(kmac, rk, "QSP5.0/PQSEED/B->A", &ctx);

    let (ck_pq_send_after, ck_pq_recv_after) = if role_is_a {
        (ck_pq_seed_a2b, ck_pq_seed_b2a)
    } else {
        (ck_pq_seed_b2a, ck_pq_seed_a2b)
    };

    let mut consumed_after = consumed_targets.clone();
    let mut tombstoned_after = tombstoned_targets.clone();
    let peer_max_after = if commit {
        consumed_after.insert(pq_target_id);
        tombstoned_after.insert(pq_target_id);
        peer_adv_id
    } else {
        peer_max_adv_id_seen
    };

    let out_send = if commit { ck_pq_send_after } else { *ck_pq_send };
    let out_recv = if commit { ck_pq_recv_after } else { *ck_pq_recv };

    Ok(ApplyReseedOut {
        ck_pq_seed_a2b,
        ck_pq_seed_b2a,
        ck_pq_send_after: out_send,
        ck_pq_recv_after: out_recv,
        peer_max_adv_id_seen_after: peer_max_after,
        consumed_targets_after: consumed_after,
        tombstoned_targets_after: tombstoned_after,
    })
}
