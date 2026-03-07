Goals: G3, G4

# NA-0012 — Suite-2 establishment vectors (CAT-S2-ESTABLISH-001)

This file anchors the CAT-S2-ESTABLISH-001 test plan only. No implementation is in this step.

Anchors:
- Vectors: `inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json`
- Runner: `scripts/ci/run_suite2_establish_vectors.py`
- Actor ops: `suite2.establish.run`, `suite2.e2e.send`, `suite2.e2e.recv`
- Spec: DOC-CAN-003 §6.1–§6.6 and §8.2

Required cases (minimum):
- Accept: valid establishment inputs (`session_id`=16, `dh_init`=32, `pq_init_ss`=32, `dh_self_pub`=32, `dh_peer_pub`=32), authenticated prerequisite satisfied, negotiated `(0x0500,0x0002)`, then send 1 non-boundary message (`flags==0`) and receive it.
- Reject: `msg_type != 0x01` → `REJECT_S2_ESTABLISH_BAD_MSG_TYPE`.
- Reject: any bad input length (`session_id`/`dh_init`/`pq_init_ss`/`dh_self_pub`/`dh_peer_pub`) → `REJECT_S2_ESTABLISH_BAD_INPUT_LEN`.
- Reject: unauthenticated prerequisite not satisfied → `REJECT_S2_ESTABLISH_UNAUTHENTICATED`.
- Reject: negotiated `protocol_version/suite_id` not `(0x0500,0x0002)` when Suite-2 policy requires establishment → `REJECT_S2_LOCAL_UNSUPPORTED`, `REJECT_S2_PEER_UNSUPPORTED`, `REJECT_S2_SUITE_MISMATCH`.
