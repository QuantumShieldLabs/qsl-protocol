# NA-0096 First Ratchet Step Plan

## Scope
- Enable first ratchet step for send/receive.

## Send/Recv Chain Advancement Rules
- Deterministic advancement per message.

## Skipped Message Handling Bounds
- Bounded skipped key cache.

## Deterministic Marker Expectations
- ratchet_send_advance msg_idx=… ck_idx=…
- ratchet_recv_advance msg_idx=…
- ratchet_skip_store count=…
- ratchet_replay_reject

## Test Vectors
- In-order send/recv advances chains.
- Out-of-order receive stores skipped key and later consumes.
- Replay is rejected deterministically (no mutation).
- Tamper is rejected deterministically (no mutation).
- Skip-cap exceeded triggers deterministic eviction.

## PCS/FS Test Strategy
- Add tests demonstrating PCS/FS properties.

## Verification Checklist
- Deterministic behavior, bounded memory.
- Gates: fmt/test/clippy pass.

## Rollback
- Disable ratchet advancement and keep handshake-only.
