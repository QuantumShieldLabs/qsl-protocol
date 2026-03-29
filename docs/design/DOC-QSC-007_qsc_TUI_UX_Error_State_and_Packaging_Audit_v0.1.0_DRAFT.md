Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-03-29

# DOC-QSC-007 — qsc TUI UX / Error-State / Packaging Audit v0.1.0 DRAFT

Purpose:
- execute the `NA-0214` audit against the current qsc product surface without changing runtime semantics;
- classify which TUI/operator surfaces are already strong versus which now need direct polish; and
- freeze the smallest truthful next blocker implied by the current evidence.

Non-goals:
- no qsc runtime/client changes in this item;
- no qsl-attachments changes;
- no qsl-server changes;
- no website, `.github`, or workflow changes; and
- no protocol, relay, attachment-service, or cryptographic semantic change.

## 1. Authoritative inputs reviewed

This audit is grounded by the current merged state of:
- qsl-protocol governance:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
- current qsc product contracts:
  - `docs/qsc/QSC_TUI_SPEC.md`
  - `docs/qsc/QSC_TUI_INVARIANTS.md`
  - `docs/qsc/DOC-QSC-003_Demo_Packaging_Runbook_v1.0.0_DRAFT.md`
  - `docs/qsc/DOC-QSC-004_Demo_Full_Run_Addendum_v1.0.0_DRAFT.md`
  - `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
  - `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
  - `qsl/qsl-client/qsc/REMOTE_SOAK_PLAYBOOK.md`
  - `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- qsc implementation anchors:
  - `qsl/qsl-client/qsc/src/main.rs`
  - `qsl/qsl-client/qsc/src/cmd/mod.rs`
  - `qsl/qsl-client/qsc/src/store/mod.rs`
  - `qsl/qsl-client/qsc/scripts/remote_soak.py`
- qsc tests proving the audited surfaces:
  - `qsl/qsl-client/qsc/tests/tui_nav_selection.rs`
  - `qsl/qsl-client/qsc/tests/tui_focus_modes.rs`
  - `qsl/qsl-client/qsc/tests/tui_locked_first.rs`
  - `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
  - `qsl/qsl-client/qsc/tests/tui_command_output_routing.rs`
  - `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
  - `qsl/qsl-client/qsc/tests/tui_autolock.rs`
  - `qsl/qsl-client/qsc/tests/tui_startup_hardening_na0177.rs`
  - `qsl/qsl-client/qsc/tests/tui_perf_no_vault_on_nav.rs`
  - `qsl/qsl-client/qsc/tests/cli.rs`
  - `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
  - `qsl/qsl-client/qsc/tests/two_client_local_runbook_na0182.rs`
  - `qsl/qsl-client/qsc/tests/lifecycle.rs`
  - `qsl/qsl-client/qsc/tests/diagnostics.rs`
  - `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/vault_attempt_limit.rs`
  - `qsl/qsl-client/qsc/tests/unlock_gate.rs`
- the merged attachment/relay decisions that define the current operator surface:
  - `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
  - `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
- sibling repo proof shaping the qsc boundary:
  - qsl-attachments `NEXT_ACTIONS.md`, `TRACEABILITY.md`, `DECISIONS.md`
  - qsl-server `NEXT_ACTIONS.md`, `TRACEABILITY.md`, `DECISIONS.md`

## 2. Audit inventory

| Surface | Current judgment | Evidence | Implementation consequence |
| --- | --- | --- | --- |
| Focus and keyboard consistency | already good | `QSC_TUI_SPEC.md`; `QSC_TUI_INVARIANTS.md`; `tui_nav_selection.rs`; `tui_focus_modes.rs`; `tui_lock_unlock_polish.rs` | Preserve the current explicit focus model, no-focus-steal rule, single-nav-selection rule, and lock shortcut semantics while polishing copy/layout only. |
| State restoration and config/vault edge cases | needs polish | `session_state_at_rest.rs`; `identity_secret_at_rest.rs`; `vault_attempt_limit.rs`; `unlock_gate.rs`; `tui_perf_no_vault_on_nav.rs`; `tui_locked_first.rs` | The fail-closed restore/migration semantics are already strong, but the operator-facing explanation of invalid session state, vault lock, wipe threshold, and restored-vs-invalid state is still terse. |
| Operator-visible error-state wording | needs polish | `tui_command_output_routing.rs`; `tui_locked_cmd_init_ux.rs`; `tui_startup_hardening_na0177.rs`; `lifecycle.rs`; `qsl/qsl-client/qsc/src/main.rs` relay/status rendering | Command routing and deterministic reason codes are already present, but several user-facing states still resolve to generic redactions or terse codes rather than clear action-oriented product wording. |
| Packaging/release surfaces and smoke assumptions | needs polish | `DOC-QSC-003_*`; `DOC-QSC-004_*`; `LOCAL_TWO_CLIENT_RUNBOOK.md`; `REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`; `REMOTE_SOAK_PLAYBOOK.md`; `route_header_migration_docs_na0195a.rs`; `two_client_local_runbook_na0182.rs` | The qbuild/local path is now the truthful validated baseline, but packaging and smoke guidance remains fragmented across local/demo and older remote/AWS surfaces. |
| Runbook/help parity with actual behavior | already good | `cli.rs`; `tui_help_render.rs`; `tui_help_fullscreen.rs`; `route_header_migration_docs_na0195a.rs`; `two_client_local_runbook_na0182.rs` | Current CLI help, TUI help, and the local two-client runbook are test-backed and already reflect route-header and post-`w0` truths. |
| Migration and retirement-mode UX friction | needs polish | `DOC-ATT-009_*`; `LOCAL_TWO_CLIENT_RUNBOOK.md`; `REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`; `cli.rs`; `qsl/qsl-client/qsc/src/cmd/mod.rs` | The current semantics are frozen, but understanding the validated `w2` / retired receive posture still depends too heavily on CLI help and long runbook text rather than concise in-product cues. |
| Secret hygiene of TUI-visible outputs | already good | `DOC-G5-004_*`; `diagnostics.rs`; `lifecycle.rs`; `tui_focus_modes.rs`; `tui_startup_hardening_na0177.rs`; `route_header_migration_docs_na0195a.rs`; `tui_locked_cmd_init_ux.rs` | Preserve current redaction, header-only route-token discipline, no-long-hex markers, and locked-shell leak resistance. |

## 3. Ambiguity review

The audit did find rough edges, but none of them are a load-bearing blocker that requires another review-only lane first.

### 3.1 Non-blocking rough edges

- `qsl/qsl-client/qsc/src/main.rs` still renders relay status with `tls: enabled (pinning: TBD)`.
  - This is a direct product-surface polish target, not an audit-finalization gap.
- `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md` still contains a small `TBD` note and workflow-era references.
  - This is not a blocker for the qbuild-first, AWS-free baseline established by the current directive.
- Remote/AWS runbooks remain in-tree and truthful as compatibility evidence, but they are not the current validated front door.
  - This is packaging/runbook consolidation work, not missing audit proof.

### 3.2 Why no extra audit-finalization lane is required

- The deterministic focus/lock/error-routing semantics already have broad headless regression coverage.
- The fail-closed restore/vault/session behavior is already proven by dedicated at-rest and no-mutation tests.
- The local validated runbook/help surface is already test-backed.
- The remaining work is now product polish: clearer operator wording, tighter validated-lane packaging guidance, and in-product treatment of the already-frozen migration posture.

## 4. Option set

| Option | Summary | Evidence sufficiency | Result |
| --- | --- | --- | --- |
| `TU0` | direct TUI product-polish implementation is next | Sufficient: current tests and docs already make the focus model, locked-shell behavior, restore semantics, route-header posture, and post-`w0` controls explicit enough to move straight into implementation/polish work | chosen |
| `TU1` | one smaller audit/finalization gap still blocks implementation | Insufficient: the remaining issues are implementation targets inside qsc product surfaces, not another proof gap about what the next lane should do | rejected |

Why `TU0` wins:
- the real unresolved work is not whether qsc needs polish, but executing that polish without regressing current fail-closed and secret-safe behavior;
- the qbuild/local validated path is already explicit enough to serve as the packaging baseline;
- remote/AWS artifacts are now clearly secondary evidence surfaces rather than the decision-driving product baseline; and
- no remaining question requires new relay, attachment-service, or protocol semantics before qsc polish begins.

## 5. Accumulated evidence review

The recent transport transition and hardening waves matter here only because they define what qsc is now allowed to present as truth.

What is already frozen by prior evidence:
- qsl-server is back to the canonical header-based relay API and remains transport-only.
- qsl-attachments returns to `READY=0` after the durability/recovery implementation and cleanup waves; its current behavior is explicit enough that qsc does not need another attachment-lane blocker first.
- `DOC-G5-004` already freezes the current metadata/logging and secret-hygiene contract for qsc-visible operator surfaces.
- `DOC-ATT-009` already freezes the validated post-`w0` activation and retired receive posture that qsc must present truthfully.
- route-header carriage, attachment-path activation, retired-mode reject markers, and honest delivery state are already test-backed in qsc docs/help/tests.

What is therefore left:
- make the qsc product surface clearer and less fragmented without changing those underlying truths.

That means the next blocker is now the qsc product surface itself:
- focus/navigation discoverability and polish;
- operator-facing error and restore wording;
- validated-lane packaging/install/smoke consolidation; and
- migration/retired-mode UX clarity in the TUI/help/status surfaces.

## 6. Decision

Chosen result:
- `TU0`

Closeout path:
- `AY1`

Exact reason:
- the current repo state is already decision-grade and unambiguous enough to separate stable TUI safety semantics from the remaining user-facing rough edges; and
- the unresolved work sits squarely inside qsc product polish rather than another audit-finalization pass.

Exact remaining blocker:
- implement the qsc product-surface polish implied by this audit while preserving current fail-closed, no-mutation, delivery-truth, route-header, and secret-hygiene behavior.

Smallest truthful successor lane:
- `NA-0214A — qsc TUI Product Polish Implementation`

## 7. Frozen implementation boundary

The next implementation lane should stay inside qsc product surfaces and should not invent new protocol or service behavior.

It should focus on:
1. clearer operator-visible error and remediation wording for vault, restore, relay/auth, and retired-mode rejects;
2. focus/navigation and status-surface polish that preserves the current explicit focus model;
3. qbuild-first packaging/install/smoke guidance as the canonical front door, with remote/AWS artifacts clearly demoted to non-baseline compatibility evidence; and
4. in-product migration/retired-mode cues that explain the already-frozen validated deployment posture more directly.

It must preserve:
- no silent break of validated deployment flows;
- no dishonest delivery semantics;
- no capability-like secrets in canonical URLs or passive output;
- no regression to route-token/header carriage behavior;
- qsl-server transport-only posture; and
- qsl-attachments opaque ciphertext-only posture.

## 8. References

- `docs/qsc/QSC_TUI_SPEC.md`
- `docs/qsc/QSC_TUI_INVARIANTS.md`
- `docs/qsc/DOC-QSC-003_Demo_Packaging_Runbook_v1.0.0_DRAFT.md`
- `docs/qsc/DOC-QSC-004_Demo_Full_Run_Addendum_v1.0.0_DRAFT.md`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_SOAK_PLAYBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/scripts/remote_soak.py`
- `qsl/qsl-client/qsc/tests/tui_nav_selection.rs`
- `qsl/qsl-client/qsc/tests/tui_focus_modes.rs`
- `qsl/qsl-client/qsc/tests/tui_locked_first.rs`
- `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`
- `qsl/qsl-client/qsc/tests/tui_command_output_routing.rs`
- `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`
- `qsl/qsl-client/qsc/tests/tui_autolock.rs`
- `qsl/qsl-client/qsc/tests/tui_startup_hardening_na0177.rs`
- `qsl/qsl-client/qsc/tests/tui_perf_no_vault_on_nav.rs`
- `qsl/qsl-client/qsc/tests/cli.rs`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/tests/two_client_local_runbook_na0182.rs`
- `qsl/qsl-client/qsc/tests/lifecycle.rs`
- `qsl/qsl-client/qsc/tests/diagnostics.rs`
- `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `qsl/qsl-client/qsc/tests/vault_attempt_limit.rs`
- `qsl/qsl-client/qsc/tests/unlock_gate.rs`
- `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
