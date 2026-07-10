# NA-0630 — ENG-0019 partial remediation test plan

Goals: G4
Directive: QSL-DIR-2026-07-10-567 (D567). Decision: D-1254. Base: `main == f6e217d5`.

## What is under test
ENG-0019 partial remediation, three edits, no shipped code-path behavior change:
1. **(d) De-attest** — `.github/workflows/release-auth.yml` no longer builds/`test -f`s/copies (and thus
   no longer sha256s/provenance-attests/uploads) `refimpl_actor`.
2. **(a) Banner** — `qsp/handshake.rs` + `qsp/ratchet.rs` carry a `//! NOT PRODUCTION — auth-unsafe
   (ENG-0019)` module inner-doc (doc-comment only).
3. **CI-coverage** — `cargo +stable test -p quantumshield_refimpl --locked` runs in the required
   `ci-4a` job, so the NA-0628 anti-regression scan + DH-guard tests are enforced on every code PR.

## Verification performed (local, pre-PR)
| Check | Result |
|---|---|
| `git diff qsp/` contains only `//!` doc-comment additions (machine-checked) | PASS — no logic line changed |
| `cargo build --workspace --all-targets --locked` (WF-0013 — the banner must not break compilation) | PASS (40 s) |
| `cargo test -p quantumshield_refimpl --locked` (the exact command the new `ci-4a` step runs) | PASS — 89 lib incl. `na0628_every_dh_call_site_is_guarded_or_allowlisted` + all integration targets |
| `cargo fmt -p quantumshield_refimpl -- --check` | PASS |
| `cargo clippy -p quantumshield_refimpl --all-targets -- -D warnings` | PASS |
| Scope: only 2 `.github` files + 2 qsp source files + governance | PASS — no `qsp` logic, no `tools/actors/**`, no canonical/vector/Cargo change |

## De-attest safety
`release-auth.yml` triggers on `release: published` + `workflow_dispatch` only — it is NOT a required
PR check, so removing `refimpl_actor` from it touches no branch protection and no PR gate. The
`attest-build-provenance` (subject-path `release_artifacts/*`) and `upload-artifact` steps use the glob,
so they need no edit: the binary is simply no longer in the set. `qsc` and `qshield` are unaffected.

## CI-coverage placement
`ci-4a` is a required PR context that already has stable Rust and resolves cheaply for docs-only PRs
(`if: docs_only != 'true'`), so the refimpl coverage runs exactly when code changes. Post-merge (Phase 6)
confirms AT JOB LEVEL that the new step ran and `ci-4b`/`ci-4d-dur` stayed green.

## Out of scope (deferred to a P3 successor)
ENG-0019 (b) library type-extraction (`RatchetError`/`HandshakeInit`/`PrekeyBundle` → a neutral module,
to enable feature-gating) and (c) the Suite-1/1B conformance retirement (a product decision touching
branch protection + the 4b/4d-dur harnesses). This lane changes no `qsp` logic and retires nothing.

## Claim boundary
UNCHANGED. No shipped code path changed; no claim moved.
