# NA-0600 remote / Tailnet operator setup proof review testplan

Goals: G1, G2, G3, G4, G5

This testplan records the governance-only proof-review validation for NA-0600.
It does not authorize source, workflow, dependency, remote, Tailnet,
deployment, DNS, Cloudflare, GitHub secret, GitHub variable, public-site, or
runtime mutation.

## Required Markers

- NA0600_D1189_READINESS_CONSUMED_OK
- NA0600_D1190_CLOSEOUT_CONSUMED_OK
- NA0600_FRESH_QWORK_PROOF_OK
- NA0600_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0600_OPERATOR_PROOF_INVENTORY_OK
- NA0600_GITHUB_SECRET_NAME_REVIEW_OK
- NA0600_GITHUB_VARIABLE_NAME_REVIEW_OK
- NA0600_WORKFLOW_SURFACE_REVIEW_OK
- NA0600_TAILNET_ACCESS_PROOF_REVIEW_OK
- NA0600_QSL_SERVER_SERVICE_READINESS_REVIEW_OK
- NA0600_QSL_ATTACHMENTS_SERVICE_READINESS_REVIEW_OK
- NA0600_ACCESS_MODEL_RECHECK_OK
- NA0600_REDACTED_DIAGNOSTIC_PLAN_REFRESH_OK
- NA0600_SECURITY_METADATA_CLAIM_REVIEW_OK
- NA0600_READINESS_MATRIX_OK
- NA0600_PRIVATE_MATERIAL_SCAN_OK
- NA0600_NO_SECRET_VALUE_ACCESS_OK
- NA0600_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0600_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0600_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0600_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0600_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0600_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0600_NO_REMOTE_TAILSCALE_WORKFLOW_MUTATION_OK
- NA0600_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0600_NO_PUBLIC_READINESS_CLAIM_OK
- NA0600_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0600_NO_REMOTE_READY_CLAIM_OK
- NA0600_NO_TAILNET_READY_CLAIM_OK
- NA0600_RESULT_CLASSIFICATION_SELECTED_OK
- NA0600_SUCCESSOR_SELECTED_OK
- NA0600_ONE_READY_INVARIANT_OK

## Validation Plan

- qwork proof verification before fetch, metadata review, mutation, PR creation,
  source-analysis result publication, or proof publication.
- Queue/decision proof: READY_COUNT 1, READY NA-0600, NA-0599 DONE, NA-0598
  DONE, D-1187 once, D-1188 once, D-1189 once, D-1190 once, D-1191 absent
  before patch, D-1192 absent, duplicate decision count zero.
- Current main health: public-safety success, advisories success,
  suite2-vectors success or conclusively satisfied, no failed required checks,
  no pending required checks after allowed visibility recovery, root cargo audit
  success, nested qsc fuzz cargo audit success, locked cargo metadata success,
  and Cargo drift absent.
- Inheritance review: D531, D-1189, D-1190, NA-0599 evidence, and NA-0599
  closeout proof consumed.
- Operator proof inventory: classify supplied proof as complete, partial,
  absent, declined, private-material stop, or ambiguous; publish classes only.
- GitHub metadata review: classify secret-name and variable-name presence
  without reading values.
- Workflow surface review: classify remote handshake, relay, attachment,
  Tailnet join, redacted diagnostics, artifact/log redaction, and mutation need
  without editing workflows.
- Tailnet and service proof review: classify only access, runner, endpoint
  source, service readiness, boundary, and rollback classes.
- Private-material and claim scans: endpoint values, private ports, tokens,
  Authorization values, capabilities, payload/body/plaintext, seed/key material,
  raw topology, raw logs, raw artifacts, and restricted overclaims absent from
  publishable evidence.
- Scope guard: changed files limited to allowed NA-0600 governance/proof-review
  paths.

## Result

Selected result classification:
`REMOTE_TAILNET_OPERATOR_SETUP_PROOF_STILL_REQUIRED`.

Selected successor:
`NA-0601 -- QSL Remote / Tailnet Operator Setup Completion Harness`.

Focused runtime tests may be skipped because NA-0600 is proof-review/readiness
only, mutates no qsc/qsl-server/qsl-attachments source or runtime surface,
mutates no workflow/dependency/lockfile surface, and performs no remote/Tailnet
execution.
