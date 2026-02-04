# NA-0096 First Ratchet Step Plan

## Scope
- Enable first ratchet step for send/receive.

## Send/Recv Chain Advancement Rules
- Deterministic advancement per message.

## Skipped Message Handling Bounds
- Bounded skipped key cache.

## PCS/FS Test Strategy
- Add tests demonstrating PCS/FS properties.

## Verification Checklist
- Deterministic behavior, bounded memory.
- Gates: fmt/test/clippy pass.

## Rollback
- Disable ratchet advancement and keep handshake-only.
