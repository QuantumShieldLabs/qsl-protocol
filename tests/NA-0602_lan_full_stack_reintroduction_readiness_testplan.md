# NA-0602 LAN Full-Stack Reintroduction Readiness Testplan

Goals: G1, G2, G3, G4, G5

This testplan records governance/readiness validation for NA-0602. It does not
authorize LAN runtime tests, qsc LAN commands, qsl-server startup,
qsl-attachments runtime, remote/Tailnet action, workflow dispatch/rerun,
deployment mutation, source mutation, dependency mutation, lockfile mutation, or
private-material publication.

## Required Markers

- `NA0602_D1193_LAN_PIVOT_CONSUMED_OK`
- `NA0602_D1194_CLOSEOUT_CONSUMED_OK`
- `NA0602_FRESH_QWORK_PROOF_OK`
- `NA0602_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0602_LAN_ROLE_TOPOLOGY_READINESS_OK`
- `NA0602_LAPTOP_READINESS_PLAN_OK`
- `NA0602_BUILD_SERVER_READINESS_PLAN_OK`
- `NA0602_LAN_ACCESS_MODEL_MATRIX_OK`
- `NA0602_SELECTED_LAN_ACCESS_MODEL_OK`
- `NA0602_OPERATOR_CODEX_BOUNDARY_OK`
- `NA0602_REDACTED_LAN_TINY_MESSAGE_PLAN_OK`
- `NA0602_LAN_READINESS_MATRIX_OK`
- `NA0602_SECURITY_METADATA_CLAIM_REVIEW_OK`
- `NA0602_PRIVATE_MATERIAL_SCAN_OK`
- `NA0602_NO_SECRET_VALUE_ACCESS_OK`
- `NA0602_NO_ENDPOINT_VALUE_PUBLISHED_OK`
- `NA0602_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK`
- `NA0602_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK`
- `NA0602_NO_CAPABILITY_VALUE_PUBLISHED_OK`
- `NA0602_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK`
- `NA0602_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK`
- `NA0602_NO_LAN_RUNTIME_TEST_OK`
- `NA0602_NO_CODEX_SSH_TO_LAPTOP_OK`
- `NA0602_NO_LAPTOP_SSH_SERVER_SETUP_OK`
- `NA0602_NO_SECOND_CODEX_ON_LAPTOP_OK`
- `NA0602_NO_REMOTE_TAILSCALE_WORKFLOW_MUTATION_OK`
- `NA0602_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK`
- `NA0602_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0602_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0602_NO_REMOTE_READY_CLAIM_OK`
- `NA0602_NO_TAILNET_READY_CLAIM_OK`
- `NA0602_NO_LAN_READY_OVERCLAIM_OK`
- `NA0602_RESULT_CLASSIFICATION_SELECTED_OK`
- `NA0602_SUCCESSOR_SELECTED_OK`
- `NA0602_ONE_READY_INVARIANT_OK`

## Validation Plan

- Verify fresh qwork proof was copied and parsed before fetch, mutation,
  GitHub metadata review, source-analysis result publication, PR creation, or
  proof publication.
- Verify READY_COUNT 1 with READY NA-0602 before implementation patch.
- Verify NA-0600 and NA-0601 are DONE.
- Verify D-1191, D-1192, D-1193, and D-1194 each exist once and are Accepted.
- Verify D-1195 was absent before implementation patch and exists once after
  implementation patch.
- Verify D-1196 remains absent before optional closeout.
- Verify duplicate decision count is zero.
- Verify D533/D-1193/D-1194 inheritance.
- Verify LAN role/topology readiness.
- Verify operator laptop readiness plan and checklist.
- Verify build-server readiness plan.
- Verify LAN access model matrix and selected Model A.
- Verify NA-0603 operator/Codex boundary.
- Verify redacted LAN tiny-message diagnostic plan.
- Verify LAN readiness matrix.
- Verify security, metadata, private-material, and claim-boundary review.
- Verify no secret values, endpoint values, private port values, hostnames,
  topology details beyond classes, token values, Authorization values,
  capability values, payload/body/plaintext content, ciphertext bodies, seed
  values, key material, raw command lines, raw logs, raw artifacts, or private
  material are published.
- Verify no LAN runtime test occurred.
- Verify no Codex SSH to laptop occurred.
- Verify no laptop SSH server setup occurred.
- Verify no second Codex on laptop was set up.
- Verify no remote action, Tailscale action, workflow dispatch/rerun, workflow
  mutation, GitHub secret/variable mutation, DNS mutation, Cloudflare mutation,
  public-site mutation, qsl-server deployment mutation, qsl-attachments
  deployment mutation, source mutation, dependency mutation, or lockfile
  mutation occurred.
- Verify selected result classification is
  `LAN_FULL_STACK_REINTRODUCTION_READINESS_SELECTED`.
- Verify selected successor is exactly
  `NA-0603 -- QSL LAN Minimal qsc E2EE Relay Verification Harness`.
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

Focused runtime tests are skipped because NA-0602 is readiness-only, does not
mutate qsc/qsl-server/qsl-attachments source or runtime behavior, does not
mutate workflows/dependencies/lockfiles, and does not execute LAN or
remote/Tailnet verification.
