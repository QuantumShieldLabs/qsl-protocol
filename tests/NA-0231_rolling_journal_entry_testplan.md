Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-10

# NA-0231 Rolling Journal Entry Testplan

## Scope

- Implementation/evidence-only resolution for `NA-0231`.
- Truth target: prove whether the staged ML-DSA timing-oracle verify-path claim is still live on refreshed current `main` without fabricating a runtime fix.
- Required companions: `.cargo/audit.toml`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md`, and one rolling-journal entry in `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- Runtime test surface: `qsl/qsl-client/qsc/tests/handshake_*.rs` and `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs` only as directly needed for verify-path evidence.

## Validation bundle

- Local `goal-lint` via a synthesized `GITHUB_EVENT_PATH`.
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo test --test handshake_security_closure`
- `cargo test --test qsp_protocol_gate`
- `cargo test --test handshake_contract_na0217i`
- `cargo test --test identity_binding`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked`
- Markdown inventory commands from `AGENTS.md`
- Manual markdown link-integrity runbook from `AGENTS.md`
- Added-line leak-safe scan with summary counts only

## Non-goals

- No `NEXT_ACTIONS.md` closeout or queue promotion.
- No out-of-scope runtime, sibling-repo, `.github`, website, or docs-surface expansion.
- No fabricated runtime remediation if refreshed current-main plus upstream advisory truth prove the staged verify-path finding stale.
