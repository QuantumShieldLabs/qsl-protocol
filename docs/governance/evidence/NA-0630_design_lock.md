# NA-0630 — DESIGN-LOCK (ENG-0019 partial remediation per D567)

Goals: G4. Directive: QSL-DIR-2026-07-10-567 (D567, APPROVED). Base: `main == f6e217d5`. 2026-07-10.

Surface re-verified live at Phase 0: release-auth.yml ships refimpl_actor (3 refs); qsp modules carry
no banner; no full-crate `cargo test -p quantumshield_refimpl` in CI; ENG-0019 P2. All hold.

## (d) De-attest — `.github/workflows/release-auth.yml`
Remove exactly the three `refimpl_actor` lines from the "Build release artifacts" step:
- `cargo build -p refimpl_actor --release --locked`
- `test -f target/release/refimpl_actor`
- `cp target/release/refimpl_actor release_artifacts/`
`qsc` and `qshield` stay. `sha256sum release_artifacts/*`, `attest-build-provenance` (subject-path
`release_artifacts/*`), and `upload-artifact` use the glob and need NO edit — the binary is simply no
longer in the set. **release-auth runs on `release: published` + `workflow_dispatch` only → NOT a
required PR check → touches no branch protection.** No PR check changes.

## (a) Banner — doc-comment only, no behavior change
Prepend a module inner-doc banner to `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` and
`.../qsp/ratchet.rs` (before the first `use`):
```
//! NOT PRODUCTION — auth-unsafe reference implementation (ENG-0019).
//! `responder_process` defers KT identity verification to the caller and `pq_rcv_a_priv` is left
//! empty, so peer authentication is MITM-able if this is wired into a real deployment. The shipped
//! client uses `qsc`'s own QSC.HS.* handshake, not this. Kept only as the Suite-1/1B conformance
//! reference exercised by ci-4b / ci-4d-dur.
```
`//!` inner docs are valid at file top (the submodule already has none). No item, no logic touched.

## CI-coverage — `ci-4a` (required, PR-running, skips cheaply on docs-only)
Add `cargo +stable test -p quantumshield_refimpl --locked` to the "Run ci-4a" step (after the
qsp_protocol_gate line). ci-4a is a REQUIRED context, runs on PRs, already has stable Rust and runs
targeted crate tests, and resolves cheaply for docs-only PRs (`if: docs_only != 'true'`) — so the
refimpl coverage runs exactly when code changes. This makes the NA-0628 anti-regression `.dh(` scan
(`na0628_every_dh_call_site_is_guarded_or_allowlisted`) and the DH-guard tests run on every code PR —
closing the coverage gap NA-0628 filed. **Measured wall-clock:** the refimpl unit tests are ~0.5 s
(NA-0628); the added cost is a `cargo build` of the crate + that run — negligible against ci-4a's
existing qsc build/test. Decision-3 recommendation (full crate) taken; no subset fallback needed.
**STOP if `cargo test -p quantumshield_refimpl` is not green in the CI env** (it is green locally — the
scan resolves the repo root via `CARGO_MANIFEST_DIR/../../..`, valid in a CI checkout).

## Scope / no-forbidden proof
Two `.github` files (release-auth.yml, ci.yml), two qsp source files (doc-comment only). No `qsp` LOGIC,
no `tools/actors/**`, no canonical, no vector, no Cargo, no KDF/AEAD/KEM. Deferred: (b) library
type-extraction, (c) Suite-1/1B retirement. ENG-0019 → P3 after this.
