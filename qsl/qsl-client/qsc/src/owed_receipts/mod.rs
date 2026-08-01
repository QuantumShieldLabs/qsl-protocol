//! Owed delivery receipts — the durable hold that makes "defer" mean defer.
//!
//! ⚠ WHY THIS EXISTS. NA-0688 reversed A6: **a control send never originates crypto, including
//! establishment.** An ack therefore requires an ALREADY-ESTABLISHED sending chain — and the
//! recipient of a first-ever message has none, because a Suite-2 responder's send chain is zero
//! until their first ratchet. Without somewhere to put the obligation, "the ack cannot be sent
//! yet" degrades into "the ack is dropped", which is exactly the receipt loss that was measured
//! and refused: alice would sit on SENT forever for everything she sent before her peer's first
//! reply.
//!
//! So the obligation is written down, durably, and flushed on the peer's first real send — the
//! one that establishes the chain normally.
//!
//! ## Why a VAULT SECRET, when the msgqueue deliberately refused one
//!
//! `msgqueue` records why it did NOT use vault secrets (D617 F4 / census C9): `vault::secret_set`
//! decrypts, re-serialises and re-encrypts the WHOLE vault, measured at ~18 ms (ENG-0053, 95–97%
//! Argon2id) — and Slice 3 makes per-message state transitions **the common case**, so that cost
//! would be paid per transition.
//!
//! ⚠ **THE PROPERTY THAT MADE THAT TRUE IS THE ONE THAT CHANGES HERE, and it inverts.** An owed
//! receipt is written **only when the peer's sending chain is unseeded** — at most once per
//! conversation direction, before the first-ever reply. It is the RARE case, not the common one,
//! so the ~18 ms objection does not carry. The ruling that chose this pattern is explicit that
//! **the frequency assumption is its load-bearing premise**: if owed-receipt writes ever stop
//! being rare (a retry path rewriting them per attempt, say), this choice must be reopened, not
//! quietly kept.
//!
//! What it buys: no third store-key secret, no extra `secret_get` at process start, and — the
//! reason it was preferred over reusing `msgqueue_store_key_v1` — **nothing in `msgqueue` is
//! touched**, so ENG-0096's boundary stays exactly where it was left.
//!
//! ## ⚠ This is a THIRD persistence home for in-flight state, accepted deliberately
//!
//! ENG-0083 already files that in-flight ratchet state lives in TWO places; ENG-0096 will add a
//! fourth (a control-message kind on the queue row). Consolidating them is **ENG-0083's job and
//! explicitly not this lane's**. This module is recorded there as an input to that work so it is
//! not forgotten.

use crate::clock;
use crate::output::emit_marker;
use crate::vault;
use serde::{Deserialize, Serialize};

/// The vault secret holding the owed-receipt list.
pub(crate) const OWED_RECEIPTS_SECRET: &str = "owed_receipts_v1";

/// ⚠ Per-PEER, not global, and the distinction is the point: a peer that floods us with messages
/// while we have never replied must not be able to evict another peer's owed receipt.
pub(crate) const OWED_RECEIPTS_MAX_PER_PEER: usize = 256;

/// ⚠ MEASURED, not chosen: both deployed relays report `retention.ttl_secs = 604800` in
/// `/v1/server-info`, so this is the message-delivery TTL. An owed receipt must not outlive the
/// message it acknowledges — acking something the relay has already dropped tells the sender
/// nothing true.
pub(crate) const OWED_RECEIPTS_TTL_SECS: u64 = 604_800;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct OwedReceipt {
    pub peer: String,
    pub msg_id: String,
    pub owed_at_unix: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct OwedReceiptStore {
    #[serde(default)]
    version: u32,
    #[serde(default)]
    entries: Vec<OwedReceipt>,
}

const STORE_VERSION: u32 = 1;

fn load() -> Result<OwedReceiptStore, &'static str> {
    match vault::secret_get(OWED_RECEIPTS_SECRET) {
        Ok(None) => Ok(OwedReceiptStore {
            version: STORE_VERSION,
            entries: Vec::new(),
        }),
        // ⚠ A store we cannot parse resets to empty rather than failing the receive. The cost of
        // a reset is bounded and known (the owed receipts in it are lost, and the sender stays on
        // SENT); the cost of failing closed is that a corrupt store makes the client unable to
        // RECEIVE at all, which is strictly worse for a diagnostic-grade obligation.
        Ok(Some(v)) => Ok(serde_json::from_str::<OwedReceiptStore>(&v).unwrap_or_default()),
        Err("vault_missing" | "vault_locked") => Err("owed_receipts_vault_locked"),
        Err(_) => Err("owed_receipts_unavailable"),
    }
}

fn save(store: &OwedReceiptStore) -> Result<(), &'static str> {
    let json = serde_json::to_string(store).map_err(|_| "owed_receipts_unavailable")?;
    match vault::secret_set(OWED_RECEIPTS_SECRET, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("owed_receipts_vault_locked"),
        Err(_) => Err("owed_receipts_unavailable"),
    }
}

/// Drop expired entries, emitting a witness for each.
///
/// ⚠ THE MARKER IS NOT DECORATION. An owed receipt that expires is a real, if bounded, loss — the
/// sender never learns their message arrived. A drop with no witness is the defect, not the drop
/// (the ENG-0099 lesson), so every expiry is announced and none is silent.
fn prune_expired(entries: &mut Vec<OwedReceipt>, now: u64) {
    let before = entries.len();
    entries.retain(|e| now.saturating_sub(e.owed_at_unix) <= OWED_RECEIPTS_TTL_SECS);
    let expired = before.saturating_sub(entries.len());
    if expired > 0 {
        let n = expired.to_string();
        emit_marker(
            "owed_receipt_expired",
            None,
            &[("count", n.as_str()), ("ttl_secs", "604800")],
        );
    }
}

/// Record an owed receipt for a peer whose sending chain is not yet established.
///
/// Idempotent on `(peer, msg_id)`: a redelivered message must not owe two receipts.
pub(crate) fn record(peer: &str, msg_id: &str) -> Result<(), &'static str> {
    let now = clock::now_unix_s();
    let mut store = load()?;
    prune_expired(&mut store.entries, now);
    if store
        .entries
        .iter()
        .any(|e| e.peer == peer && e.msg_id == msg_id)
    {
        return Ok(());
    }
    store.entries.push(OwedReceipt {
        peer: peer.to_string(),
        msg_id: msg_id.to_string(),
        owed_at_unix: now,
    });
    // ⚠ Cap PER PEER, evicting that peer's oldest — a flood from one contact cannot displace
    // another contact's obligation. The eviction is announced for the same reason expiry is.
    let peer_count = store.entries.iter().filter(|e| e.peer == peer).count();
    if peer_count > OWED_RECEIPTS_MAX_PER_PEER {
        let excess = peer_count - OWED_RECEIPTS_MAX_PER_PEER;
        let mut dropped = 0usize;
        store.entries.retain(|e| {
            if e.peer == peer && dropped < excess {
                dropped += 1;
                return false;
            }
            true
        });
        let n = dropped.to_string();
        emit_marker(
            "owed_receipt_evicted",
            None,
            &[("count", n.as_str()), ("reason", "per_peer_cap")],
        );
    }
    store.version = STORE_VERSION;
    save(&store)
}

/// Take every owed receipt for a peer, removing them from the store.
///
/// Returns the `msg_id`s in the order they were owed. The caller sends them; anything that fails
/// to send is re-recorded by the caller rather than silently lost.
pub(crate) fn take_for_peer(peer: &str) -> Result<Vec<String>, &'static str> {
    let now = clock::now_unix_s();
    let mut store = load()?;
    prune_expired(&mut store.entries, now);
    let mut taken: Vec<String> = Vec::new();
    store.entries.retain(|e| {
        if e.peer == peer {
            taken.push(e.msg_id.clone());
            false
        } else {
            true
        }
    });
    if !taken.is_empty() {
        store.version = STORE_VERSION;
        save(&store)?;
    }
    Ok(taken)
}

/// Whether anything is owed to a peer, without taking it. Used by the send path so the common
/// case (nothing owed) costs one read and no write.
pub(crate) fn any_owed(peer: &str) -> bool {
    match load() {
        Ok(store) => store.entries.iter().any(|e| e.peer == peer),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expiry_prunes_and_is_bounded_by_the_delivery_ttl() {
        let mut v = vec![
            // ⚠ GENUINELY older than the TTL, not sitting ON the boundary. The first version of
            // this fixture used an age of EXACTLY the TTL, which the `<=` rule keeps — putting it
            // in direct contradiction with `an_entry_exactly_at_the_ttl_boundary_survives` below,
            // which asserts that same keep and passes. A self-contradicting pair is worse than an
            // off-by-one: whichever way the code moved, one of the two would have gone green and
            // looked like proof. The boundary belongs in the boundary test; this one tests expiry.
            OwedReceipt {
                peer: "a".into(),
                msg_id: "old".into(),
                owed_at_unix: 0,
            },
            OwedReceipt {
                peer: "a".into(),
                msg_id: "fresh".into(),
                owed_at_unix: 1_000 + OWED_RECEIPTS_TTL_SECS,
            },
        ];
        // now = exactly TTL past the fresh one's owed time; the old one is well past.
        prune_expired(&mut v, 1_000 + OWED_RECEIPTS_TTL_SECS);
        assert_eq!(v.len(), 1, "the expired entry must be dropped");
        assert_eq!(v[0].msg_id, "fresh");
    }

    #[test]
    fn an_entry_exactly_at_the_ttl_boundary_survives() {
        // The boundary is `<=`, so an entry aged exactly TTL is still owed. Pinned because an
        // off-by-one here silently shortens every owed receipt's life by a full tick.
        let mut v = vec![OwedReceipt {
            peer: "a".into(),
            msg_id: "edge".into(),
            owed_at_unix: 0,
        }];
        prune_expired(&mut v, OWED_RECEIPTS_TTL_SECS);
        assert_eq!(v.len(), 1);
        prune_expired(&mut v, OWED_RECEIPTS_TTL_SECS + 1);
        assert!(v.is_empty(), "one second past the TTL it must expire");
    }

    #[test]
    fn the_ttl_matches_the_measured_relay_retention() {
        // ⚠ Not a style pin. Both deployed relays report retention.ttl_secs = 604800; an owed
        // receipt must not outlive the message it acknowledges.
        assert_eq!(OWED_RECEIPTS_TTL_SECS, 604_800);
    }
}
