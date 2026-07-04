# NA-0601 Tailnet Setup Deferral / LAN Full-Stack Pivot Testplan

Goals: G1, G2, G3, G4, G5

This testplan records governance/readiness validation for NA-0601. It does not
authorize LAN runtime tests, remote tests, Tailnet action, workflow mutation,
deployment mutation, source mutation, dependency mutation, lockfile mutation, or
private-material publication.

## Required Markers

- `NA0601_D1191_OPERATOR_SETUP_REVIEW_CONSUMED_OK`
- `NA0601_D1192_CLOSEOUT_CONSUMED_OK`
- `NA0601_FRESH_QWORK_PROOF_OK`
- `NA0601_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0601_TAILNET_SETUP_DEFERRED_OK`
- `NA0601_LAN_PIVOT_SELECTED_OK`
- `NA0601_LAN_TOPOLOGY_CLASSIFIED_OK`
- `NA0601_LAN_ACCESS_MODEL_MATRIX_OK`
- `NA0601_OPERATOR_CODEX_BOUNDARY_OK`
- `NA0601_REDACTED_LAN_DIAGNOSTIC_PLAN_OK`
- `NA0601_LAN_READINESS_MATRIX_OK`
- `NA0601_SECURITY_METADATA_CLAIM_REVIEW_OK`
- `NA0601_PRIVATE_MATERIAL_SCAN_OK`
- `NA0601_NO_SECRET_VALUE_ACCESS_OK`
- `NA0601_NO_ENDPOINT_VALUE_PUBLISHED_OK`
- `NA0601_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK`
- `NA0601_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK`
- `NA0601_NO_CAPABILITY_VALUE_PUBLISHED_OK`
- `NA0601_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK`
- `NA0601_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK`
- `NA0601_NO_LAN_RUNTIME_TEST_OK`
- `NA0601_NO_REMOTE_TAILSCALE_WORKFLOW_MUTATION_OK`
- `NA0601_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK`
- `NA0601_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0601_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0601_NO_REMOTE_READY_CLAIM_OK`
- `NA0601_NO_TAILNET_READY_CLAIM_OK`
- `NA0601_NO_LAN_READY_OVERCLAIM_OK`
- `NA0601_RESULT_CLASSIFICATION_SELECTED_OK`
- `NA0601_SUCCESSOR_SELECTED_OK`
- `NA0601_ONE_READY_INVARIANT_OK`

## Validation Plan

- Verify fresh qwork proof was copied and parsed before fetch and mutation.
- Verify READY_COUNT 1 with READY NA-0601 before implementation patch.
- Verify D-1191 and D-1192 each exist once and are Accepted.
- Verify D-1193 was absent before implementation patch and exists once after
  implementation patch.
- Verify D-1194 remains absent before optional closeout.
- Verify duplicate decision count is zero.
- Verify D532/D-1191/D-1192 inheritance.
- Verify Tailnet setup deferral and LAN pivot classifications.
- Verify LAN topology class-only record.
- Verify LAN access model matrix and selected initial model.
- Verify operator/Codex responsibility boundary.
- Verify redacted future LAN diagnostic plan.
- Verify LAN readiness matrix.
- Verify security, metadata, and claim-boundary review.
- Verify no secret values, endpoint values, private ports, hostnames, topology
  details beyond classes, tokens, Authorization values, capabilities, payloads,
  plaintext, ciphertext bodies, seeds, key material, raw logs, raw artifacts, or
  private material are published.
- Verify no LAN runtime test occurred.
- Verify no remote action, Tailscale action, workflow dispatch/rerun, workflow
  mutation, GitHub secret/variable mutation, DNS mutation, Cloudflare mutation,
  public-site mutation, qsl-server deployment mutation, qsl-attachments
  deployment mutation, source mutation, dependency mutation, or lockfile
  mutation occurred.
- Verify selected result classification is
  `TAILNET_OPERATOR_SETUP_DEFERRED_LAN_PIVOT_SELECTED`.
- Verify selected successor is exactly
  `NA-0602 -- QSL LAN Full-Stack Reintroduction Readiness Harness`.
- Verify exactly one READY item remains before optional closeout.

## Local Validation Commands

- `git diff --check`
- scope guard over tracked, staged, and untracked changes
- queue/decision proof
- marker proof
- markdown link-check
- added-line/new-file private-material scan
- secret/prohibited-material scan
- overclaim scan
- LAN/private-topology publication scan
- remote/Tailnet/private-topology publication scan
- crypto/triple-ratchet/attachment/remote-readiness/LAN-readiness
  claim-boundary scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz cargo audit
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused runtime tests are skipped because NA-0601 is pivot/readiness-only, does
not mutate qsc/qsl-server/qsl-attachments source or runtime behavior, does not
mutate workflows/dependencies/lockfiles, and does not execute LAN or
remote/Tailnet verification.
