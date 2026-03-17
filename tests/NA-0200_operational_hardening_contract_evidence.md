# NA-0200 Operational Hardening Contract Evidence

## Current operational gaps
- `qsl-attachments` is still only a single-node local-disk runtime with local JSON journals and minimal `rust` CI / branch protection.
- No implementation-grade deployment profile set exists yet.
- No constrained-host validation ladder exists yet for deployed `qsl-attachments` plus the real relay.
- No frozen promotion/deprecation gate exists yet for default-path promotion or legacy deprecation.

## Current trustworthy state
- Attachment contract/runtime/client correctness is already grounded.
- The current coexistence rule is already directly validated.
- qsl-server remains transport-only and out of the blob plane.
- No runtime/service/client semantic changes are needed to define the operational contract.

## Weak-system input
- Constrained service hosts and weak relays, especially the AWS relay, are explicit engineering inputs.
- The contract distinguishes saturation from correctness failure.
- The contract does not invent a hardware floor that current evidence cannot support; it instead requires measurement capture and honest classification.

## Doc ID proof
- `DOC-ATT-002` was unused before this item.
- Verified by repo search before document creation.

## Contract outputs frozen in this item
- Deployment profiles: local development, constrained-host validation, stronger reference deployment.
- Readiness categories: storage/recovery, retention/expiry, quota/abuse, observability, ingress/TLS, restart/resume, resource/load characterization, rollout gates.
- Constrained-host validation ladder: single-flow, threshold, large-file, interruption/resume, expiry/quota/reject, secret-hygiene, resource observation, limited concurrency.
- Promotion gates: default-path promotion and legacy deprecation remain blocked until the ladder is executed truthfully.

## Queue implication
- The next truthful implementation step is operational hardening plus constrained-host real-world validation, not another contract/design pass.
