Goals: G2, G5
Status: DRAFT

Scope:
- Demo CLI store lifecycle only; no protocol-core changes.

Objective:
- Define deterministic deletion/rotation policy for demo store artifacts.
- Enforce permission and rotation invariants in CI.

CI-gated assertions:
- Store directory is 0700 and config/state files are 0600 on Unix.
- `qshield rotate` removes config/state artifacts (best-effort overwrite + delete).

Evidence:
- metadata-conformance-smoke CI logs.
