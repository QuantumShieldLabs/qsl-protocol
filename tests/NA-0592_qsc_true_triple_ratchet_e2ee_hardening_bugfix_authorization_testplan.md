Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0592 qsc true triple-ratchet E2EE hardening / bug-fix authorization testplan

## Scope

This testplan records the NA-0592 authorization lane. It classifies the qsc seed
fallback/demo-fixture shortcut, selects the hardening strategy, and selects the
exact NA-0593 implementation successor. It does not implement hardening, does
not mutate qsc source/tests/scripts, does not run full qsl-attachments
send/receive integration, does not make a public readiness claim, does not make
a production readiness claim, does not make a crypto-complete claim, and does
not make a triple-ratchet-complete claim.

## Required Markers

- NA0592_D1173_TRIPLE_RATCHET_FINDING_CONSUMED_OK
- NA0592_D1174_CLOSEOUT_CONSUMED_OK
- NA0592_FRESH_QWORK_PROOF_OK
- NA0592_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0592_SEED_FALLBACK_SOURCE_INVENTORY_OK
- NA0592_SEED_FALLBACK_CALL_GRAPH_OK
- NA0592_RUNTIME_REACHABILITY_CLASSIFIED_OK
- NA0592_PRIMARY_RISK_CLASSIFIED_OK
- NA0592_HOSTILE_CRYPTOGRAPHER_REVIEW_OK
- NA0592_RED_TEAM_REVIEW_OK
- NA0592_SIDE_CHANNEL_CAVEAT_OK
- NA0592_HARDENING_OPTIONS_REVIEWED_OK
- NA0592_SELECTED_HARDENING_STRATEGY_OK
- NA0592_EXACT_IMPLEMENTATION_PLAN_OK
- NA0592_FOCUSED_VALIDATION_OK
- NA0592_PRIVATE_MATERIAL_SCAN_OK
- NA0592_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0592_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0592_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0592_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0592_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0592_NO_FULL_ATTACHMENT_INTEGRATION_OK
- NA0592_NO_QSL_SERVER_MUTATION_OK
- NA0592_NO_QSL_ATTACHMENTS_MUTATION_OK
- NA0592_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0592_NO_PUBLIC_READINESS_CLAIM_OK
- NA0592_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0592_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0592_NO_TRIPLE_RATCHET_COMPLETE_OVERCLAIM_OK
- NA0592_RESULT_CLASSIFICATION_SELECTED_OK
- NA0592_SUCCESSOR_SELECTED_OK
- NA0592_ONE_READY_INVARIANT_OK

## Evidence Requirements

- Verify qwork proof before fetch, source-result publication, qsc tests,
  GitHub metadata review, repository mutation, or proof publication.
- Verify clean startup worktree, index, and untracked state.
- Verify `HEAD` and `origin/main` against qwork proof before fetch.
- Verify disk and mount gates before fetch.
- Verify D-1173 and D-1174 exist once and are Accepted.
- Verify D-1175 and D-1176 are absent before patch.
- Verify READY_COUNT 1 with READY NA-0592.
- Verify NA-0591 and NA-0590 are DONE.
- Verify current main public-safety, advisories, suite2-vectors, required-check
  visibility, cargo audit, nested qsc fuzz audit, locked metadata, and Cargo
  drift health.
- Map seed fallback source, call graph, gates, and command reachability.
- Classify primary risk exactly once.
- Complete BKM, hostile cryptographer, red-team, Production SRE, side-channel,
  formal mapping, and release-claim reviews.
- Review hardening options A through H.
- Select exact hardening strategy and exact successor.
- Run focused qsc tests sufficient for authorization.
- Scan publishable evidence and response text for private material and
  overclaims.

## Validation Commands

- `git diff --check`
- scope guard over tracked, staged, and untracked files
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- secret/prohibited-material scan
- overclaim scan
- crypto/triple-ratchet claim-boundary scan
- docs/governance/source-diagnostic classifier
- PR body preflight
- goal-lint when PR body exists
- root `cargo audit`
- nested qsc fuzz `cargo audit`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`
- `cargo test -p qsc --test qsp_status_truthy`
- `cargo test -p qsc --test qsp_protocol_gate`

## Expected Result

NA-0592 records D-1175, classifies the seed fallback as
`SEED_FALLBACK_RUNTIME_PRODUCTION_PATH_RISK`, selects
`SEED_FALLBACK_HARDENING_IMPLEMENTATION_READY`, and selects
`NA-0593 -- QSL qsc True Triple-Ratchet Seed Fallback Hardening Implementation
Harness` as the sole implementation successor. NA-0592 itself does not implement
hardening and does not start full qsl-attachments integration.
