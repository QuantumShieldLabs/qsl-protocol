Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0591 local qsc true triple-ratchet E2EE path verification testplan

## Scope

This testplan records the resumed NA-0591 qsc true triple-ratchet path verification. It restores a qsc diagnostic test and records governance evidence. It does not harden the seed fallback, does not implement NA-0592, does not run full qsl-attachments send/receive integration, and does not claim public, production, formal-proof, side-channel-free, crypto-complete, attachment-complete, or triple-ratchet-complete readiness.

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

## Evidence Requirements

- Verify fresh qwork proof and clean qsl-protocol startup state before fetch, qsl-server action, qsc test restoration, repository mutation, GitHub publication, or proof publication.
- Verify D-1171 and D-1172 inheritance, D520 stop state, READY NA-0591, D-1173 absence, D-1174 absence, and duplicate decision count zero before patch.
- Verify the preserved qsc diagnostic test SHA before restoration.
- Reproduce and classify the qsl-server audit blocker using true GitHub main as source of truth.
- If qsl-server audit requires recovery, prove Cargo.lock-only sufficiency before PR/merge. If true GitHub main is already clear, record the corrected audit-clear classification and do not merge stale recovery.
- Restore the qsc diagnostic test under `qsl/qsl-client/qsc/tests/`.
- Run focused qsc tests for the restored test, ratchet/QSP/QSE/protocol/handshake/Suite2/relay/peer-separation coverage, correcting stale test target names only through current valid test discovery.
- Refresh qsc send/receive source mapping and true triple-ratchet path classification.
- Run proof-root-only local qsc/qsl-server dynamic proof with no seed fallback and raw values quarantined.
- Classify selected negative/adversarial tests as fail-closed, detected/rejected, not supported, diagnostic gap, or ambiguous.
- Complete crypto review packet and private-material review.
- Select exactly one result classification and exactly one successor.

## Validation Commands

- `git diff --check`
- scope guard over tracked, staged, and untracked files
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- dynamic proof/artifact/log private-material scan
- secret/prohibited-material scan
- overclaim scan
- crypto/triple-ratchet claim-boundary scan
- docs/governance/source-diagnostic classifier
- PR body preflight
- goal-lint when available
- root `cargo audit`
- nested qsc fuzz `cargo audit`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`
- focused qsc ratchet/QSP/QSE/protocol/handshake/Suite2/relay/peer tests
- qsl-server metadata, audit, fmt, test, and build when qsl-server is used

## Expected Result

NA-0591 records D-1173, restores the qsc diagnostic test, verifies the no-seed qsc send/receive path is Suite2/triple-ratchet-bound for the validated local path, records the explicit seed fallback/demo-fixture bypass finding, selects `TRUE_TRIPLE_RATCHET_DEMO_OR_FIXTURE_BYPASS_FOUND`, and selects `NA-0592 -- QSL qsc True Triple-Ratchet E2EE Hardening / Bug Fix Authorization Harness` before full qsl-attachments integration.
