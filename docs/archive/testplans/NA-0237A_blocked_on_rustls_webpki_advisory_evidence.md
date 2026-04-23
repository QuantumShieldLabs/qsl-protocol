Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-22

# NA-0237A Blocked on rustls-webpki Advisory Evidence

Goals: G4

## Purpose

This archive evidence records why `NA-0237A` must be marked `BLOCKED` and why `NA-0237B` is the next truthful READY item.

## Stopped Implementation Attempt Proof

- The local `NA-0237A` implementation attempt reproduced and repaired the bounded qsc `send_commit` fallout locally.
- The repaired seam stayed within the authorized implementation scope: `qsl/qsl-client/qsc/tests/send_commit.rs` plus the already-authorized clippy-only validation seam in `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`.
- The implementation attempt stopped during the required local validation bundle when `cargo audit --deny warnings` failed.

## Advisory Proof

- Failing command: `cargo audit --deny warnings`
- Advisory: `RUSTSEC-2026-0104`
- Affected crate/version: `rustls-webpki 0.103.12`
- Tool-reported patched floor: `>= 0.103.13`
- Runtime reachability proof from local audit/tree output includes:
  - `qsc` through `reqwest`
  - `qsl-tui` through `reqwest`
  - `qshield-cli` through `ureq`

## Scope Conclusion

- `NA-0237A` explicitly forbids `Cargo.toml` and `Cargo.lock` edits.
- Remediating or replacing the vulnerable dependency requires dependency manifest and/or lockfile authority.
- The blocker is therefore outside live `NA-0237A` scope even though the `send_commit` repair itself is bounded and locally implemented.

## Governance-Only Statement

This PR is governance-only. It changes queue truth and evidence only; it does not change runtime semantics, dependency versions, Cargo manifests, lockfiles, workflow policy, qsc-desktop, qsl-server, or qsl-attachments.
