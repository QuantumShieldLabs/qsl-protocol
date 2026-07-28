// NA-0155 ratchet durability — ⚠ THIS FILE INTENTIONALLY CONTAINS NO TESTS.
//
// Its three guards were retired by NA-0682 under the operator's binding condition, and every
// helper they used went with them (NA-0682 STOP 021, Ruling 5: the orphaned dead code was
// removed once clippy showed it). ⚠ The retirement RECORD below is the point of the file and
// is deliberately kept in place: it is where a reader looking for the old test names finds
// out which guards now hold those properties.

// ---------------------------------------------------------------------------
// ⚠ NA-0682 (D617 §2b/§2c, operator-ruled Option A + BINDING CONDITION): the three tests
// that lived here are RETIRED, and this note is the record of where their properties went.
//
// They asserted against `outbox.json` -- the single GLOBAL in-flight slot -- by reading its
// ciphertext off disk. The default send path no longer uses that slot: it commits each
// message to the per-contact message queue, with in-flight ratchet state held PER MESSAGE
// (which is what makes contacts independent, §2c). The mechanism these tests observed is
// gone, so they cannot be repointed; the PROPERTIES they defended are re-proven, and each
// new guard was shown RED before this retirement, per the binding condition:
//
//   retry_resends_identical_ciphertext_no_reencrypt
//     -> msgqueue::tests::a_packed_record_is_never_repacked_across_retries
//        (4 attempts, EXACTLY 1 pack, identical bytes pushed every time -- strictly
//        stronger: it COUNTS the packs, which the old test could not see)
//     -> msgqueue::tests::in_flight_state_survives_a_round_trip_so_a_retry_replays_identical_bytes
//        (control: `#[serde(skip)]` on ciphertext -> RED, `left: None`)
//
//   crash_recovery_sends_from_outbox_not_recomputed_payload
//     -> NA_0682_kill_in_the_send_window::a1_...  (SIGKILLs a real process INSIDE the
//        persist-before-network window and proves the row survives and drains -- the old
//        test only simulated a restart by running the binary again)
//
//   abort_burns_state_and_prevents_nonce_reuse_on_next_send
//     -> msgqueue::tests::abandoning_a_packed_message_advances_the_ratchet_first
//        (control: revert `retire_packed` -> RED, `left: 0` commits)
//     -> msgqueue::tests::a_failed_ratchet_commit_keeps_the_message_queued_rather_than_dropping_state
//        (fail-closed: never drop the advance)
//     -> outbox_abort::discard_burns_state_and_prevents_nonce_reuse_on_next_send
//        (the same property at the CLI, on the named discard that replaced `send abort`)
//
// ⚠ The nonce-reuse property is the reason this file mattered, and it is the one this lane
// nearly broke: `clear_inflight()` on a terminal failure dropped the ratchet advance. It was
// caught by READING THIS FILE'S TEST NAME before editing near it. Retiring the tests does
// not retire the lesson -- see the as-built.
// ---------------------------------------------------------------------------
