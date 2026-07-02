Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0591 local qsc true triple-ratchet E2EE path verification harness

## Executive Summary

NA-0591 resumed D520, verified fresh qwork proof from `2026-07-02T06:39:33Z`, restored the preserved qsc diagnostic test, refreshed qsc source/path review, reran focused qsc tests, reran a proof-root-only local qsc/qsl-server E2EE path proof, and classified the result as `TRUE_TRIPLE_RATCHET_DEMO_OR_FIXTURE_BYPASS_FOUND`.

The validated no-seed qsc send/receive path is Suite2/triple-ratchet-bound for the local proof path: qsc send reaches qsp pack and Suite2 `send_wire`, qsc receive reaches qsp unpack and Suite2 `recv_wire`, ratchet send/receive advance markers are present, and qsl-server sees opaque envelope bytes rather than plaintext for the validated path.

The remaining blocker is an explicit seed fallback/demo-fixture shortcut in qsc. That shortcut is not hardened in this directive. It selects `NA-0592 -- QSL qsc True Triple-Ratchet E2EE Hardening / Bug Fix Authorization Harness` before full qsl-attachments send/receive integration.

## qwork / Startup Proof

- Fresh qwork proof was verified before fetch, qsl-server action, qsc test restoration, repository mutation, PR action, source-analysis publication, or proof publication.
- qwork startup result: OK.
- lane: NA-0591.
- repo/path: qsl-protocol at `/srv/qbuild/work/NA-0591/qsl-protocol`.
- qwork proof timestamp: `2026-07-02T06:39:33Z`.
- qsl-protocol startup head/origin/main: `c9d3b81c93e7`.
- startup worktree, index, and untracked state: clean.
- READY_COUNT: 1.
- READY item: NA-0591.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.
- Disk and mount gates passed; `/backup/qsl` was mounted and root usage was below the stop threshold.

Current main health at startup:

- public-safety: success.
- advisories: success.
- suite2-vectors: success.
- no failed required checks were attached.
- root cargo audit: pass.
- nested qsc fuzz cargo audit: pass.
- locked cargo metadata: pass.
- Cargo manifest/lock drift: absent.

One current-main check classifier recovery was recorded: absent `goal-lint` on current main was separated from failed required checks; attached required checks were green. Optional remote check failures were recorded as non-blocking startup evidence.

## D520 / D519 / D-1171 / D-1172 Inheritance

D520 stopped before D-1173, TRACEABILITY mutation, NEXT_ACTIONS mutation, evidence/testplan commit, PR creation, and closeout. D-1171 and D-1172 were verified once each and Accepted. D-1173 and D-1174 were absent before this implementation patch. NA-0591 remained the sole READY item.

The operator-preserved D520 qsc diagnostic test was verified with SHA-256 `3c8547c1b00e` short evidence and restored to `qsl/qsl-client/qsc/tests/na_0591_true_triple_ratchet_path.rs`.

## qsl-server Audit Gate Recovery

The initial qsl-server workspace used a stale local mirror remote. That mirror reproduced the D520 audit-failure shape and scratch proof showed a Cargo.lock-only advisory recovery would have worked there. Before merging anything, Codex fetched true GitHub main and found the authoritative qsl-server main was already advanced to `6bf61d439fa2` and audit-clear.

Corrected qsl-server result:

- qsl-server true GitHub main: `6bf61d439fa2`.
- `cargo metadata --locked --format-version=1`: pass.
- `cargo audit --deny warnings`: pass.
- `cargo fmt --check`: pass.
- `cargo test --locked`: pass after one local log-test rerun; GitHub required `rust` check was success.
- `cargo build --locked`: pass.
- qsl-server PR #58 was opened from stale mirror evidence, then closed unmerged after true GitHub main validation proved the audit gate already clear.
- No qsl-server Cargo.lock recovery merge was needed.
- No qsl-server source, Cargo.toml, runtime, protocol, auth, route, storage, deployment, or workflow semantic mutation was merged.

## qsc Diagnostic Test Restoration

The restored test `qsl/qsl-client/qsc/tests/na_0591_true_triple_ratchet_path.rs` is test-only. It creates two local qsc identities, completes a handshake-backed state without seed fallback, sends through the relay transport, verifies the queued relay item is an opaque QSE envelope rather than plaintext, requeues it, receives it, and asserts qsp pack/unpack plus ratchet send/receive advance markers.

Focused qsc validation:

- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`: pass.
- `cargo test -p qsc --test ratchet_step`: pass.
- `cargo test -p qsc --test qsp_qse_onwire`: pass.
- `cargo test -p qsc --test qsp_protocol_gate`: pass.
- `cargo test -p qsc --test na_0303_handshake_activation_negotiation`: pass.
- `cargo test -p qsc --test suite2_runtime_equivalence_na0198`: pass.
- `cargo test -p qsc --test relay_auth_header`: pass.
- Corrected current peer-separation coverage `cargo test -p qsc --test receive_e2e receive_mailbox_peer_separation_fail_closed`: pass.

The obsolete D520 test-target name `receive_e2e_peer_separation` no longer exists as an integration-test target in this checkout; that was recorded as recovered command/test-name drift.

## qsc CLI / Source Path Mapping

Mapped send path:

- `qsc send`.
- `transport::send_execute`.
- `relay_send_with_payload`.
- `qsp_pack`.
- Suite2 `send_wire`.

Mapped receive path:

- `qsc receive`.
- relay pull.
- `qsp_unpack`.
- Suite2 `recv_wire`.

Attachment descriptor send source review maps into the same qsc message plane through `relay_send_with_payload` and qsp pack. Full qsl-attachments send/receive integration was not run.

## True Triple-Ratchet Classification

- true triple-ratchet implementation identified: yes.
- qsc send uses true triple-ratchet: yes for the validated no-seed path.
- qsc receive uses true triple-ratchet: yes for the validated no-seed path.
- seed fallback present: yes.
- demo or fixture bypass present: yes.
- plaintext relay possible in validated path: not found.
- qsl-server sees plaintext in validated path: no.
- qsl-attachments future path plaintext/key boundary: deferred review-only; no source requirement found for plaintext/key visibility.
- key material logged in selected outputs: no evidence found.
- formal proof claim: no.
- side-channel-free claim: no.
- triple-ratchet-complete claim: no.

## Dynamic Local E2EE Path Proof

The resumed local proof used qsl-server true GitHub-main validation and a proof-root-only qsc/qsl-server loopback harness.

Classification: `DYNAMIC_LOCAL_E2EE_PATH_PASS`.

- no seed fallback marker present.
- qsc handshake-backed state completed.
- qsp pack marker present.
- qsp unpack marker present.
- Suite2 ratchet send advance marker present.
- Suite2 ratchet receive advance marker present.
- relay payload class: opaque envelope bytes.
- qsl-server plaintext class: not observed for the validated path.
- receive/decrypt/validate succeeded locally.
- owned qsl-server process cleanup completed.

Raw endpoint values, private port values, route-token values, bearer or Authorization values, payload/body/plaintext bytes, ciphertext bodies, key material, command lines, process details, and raw logs remain proof-root-only.

## Negative / Adversarial Refresh

Current focused tests plus D520 proof classify the reviewed negative cases as fail-closed or detected/rejected:

- ciphertext tamper.
- envelope/AAD/header tamper.
- wrong peer.
- stale state.
- replay-like duplicate.
- missing state.
- malformed relay payload.
- wrong route token.
- wrong bearer.

No unsupported or untested broader property was converted into a pass.

## Crypto Review Packet

- Best-Known-Method review: complete.
- Hostile Cryptographer review: complete; seed fallback/demo-fixture shortcut remains the main finding.
- Red-Team review: complete for selected tamper, wrong-peer, stale/replay-like, missing-state, malformed, route, and bearer negatives.
- Production SRE review: complete; local loopback proof is not remote/public/production readiness.
- Side-Channel caveat: no side-channel-free claim.
- Formal Mapping review: no formal proof completion claim.
- External-Review readiness: evidence is review-useful, but no external review completion claim.
- Release-Claim boundary: no public-readiness, production-readiness, vulnerability-free, bug-free, crypto-complete, attachment-complete, or triple-ratchet-complete claim.
- Assurance Trigger review: seed fallback hardening remains required before full attachment integration.

## Private-Material Review

Publication-selected summaries passed private-material and overclaim scans after a recovered scanner false positive on compact negated claim-boundary wording. The restored qsc test contains deterministic test fixture route-token literals; they are not private operator material. Evidence and final-response text do not publish raw endpoint values, private ports, route-token values, bearer values, Authorization values, payload/body/plaintext bytes, ciphertext bodies, key material, command lines, raw logs, private topology, or process identities.

## Result / Successor

Result classification: `TRUE_TRIPLE_RATCHET_DEMO_OR_FIXTURE_BYPASS_FOUND`.

Selected successor: `NA-0592 -- QSL qsc True Triple-Ratchet E2EE Hardening / Bug Fix Authorization Harness`.

NA-0592 must classify whether the seed fallback is test-only, demo-only, operator-controlled diagnostic behavior, or a runtime production-path risk, then decide the exact remove/gate/rename/restrict/test-hardening action before full qsl-attachments integration. NA-0592 is not implemented by NA-0591.

## Required Markers

- NA0591_D1171_ATTACHMENTS_READINESS_CONSUMED_OK
- NA0591_D1172_CLOSEOUT_CONSUMED_OK
- NA0591_D520_STOP_CONSUMED_OK
- NA0591_FRESH_QWORK_PROOF_OK
- NA0591_QSL_SERVER_AUDIT_RECOVERED_OR_CLEAR_OK
- NA0591_QSC_DIAGNOSTIC_TEST_RESTORED_OK
- NA0591_QSC_COMMAND_SURFACE_REVIEW_OK
- NA0591_QSC_CLI_SOURCE_MAP_OK
- NA0591_TRUE_TRIPLE_RATCHET_IMPLEMENTATION_IDENTIFIED_OK
- NA0591_SEND_PATH_BINDING_CLASSIFIED_OK
- NA0591_RECEIVE_PATH_BINDING_CLASSIFIED_OK
- NA0591_DYNAMIC_LOCAL_E2EE_PATH_PASS_OK
- NA0591_NEGATIVES_CLASSIFIED_OK
- NA0591_SEED_FALLBACK_FINDING_RECORDED_OK
- NA0591_HOSTILE_CRYPTOGRAPHER_REVIEW_OK
- NA0591_PRIVATE_MATERIAL_SCAN_OK
- NA0591_NO_FULL_ATTACHMENT_INTEGRATION_OK
- NA0591_NO_QSL_ATTACHMENTS_MUTATION_OK
- NA0591_NO_QSL_SERVER_SEMANTIC_MUTATION_OK
- NA0591_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0591_NO_PUBLIC_READINESS_CLAIM_OK
- NA0591_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0591_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0591_NO_TRIPLE_RATCHET_COMPLETE_OVERCLAIM_OK
- NA0591_RESULT_CLASSIFICATION_SELECTED_OK
- NA0591_SUCCESSOR_SELECTED_OK
- NA0591_ONE_READY_INVARIANT_OK
