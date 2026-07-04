# NA-0599 remote / Tailnet full-stack reintroduction readiness testplan

Goals: G1, G2, G3, G4, G5

This testplan records the governance-only readiness validation for NA-0599. It
does not authorize source, workflow, dependency, remote, Tailnet, deployment,
DNS, Cloudflare, GitHub secret, GitHub variable, or public-site mutation.

## Required Markers

- NA0599_D1187_QSL_SERVER_FIX_CONSUMED_OK
- NA0599_D1188_CLOSEOUT_CONSUMED_OK
- NA0599_FRESH_QWORK_PROOF_OK
- NA0599_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0599_LOCAL_EVIDENCE_SUMMARY_OK
- NA0599_PRIOR_REMOTE_FAILURE_REVIEW_OK
- NA0599_WORKFLOW_SURFACE_REVIEW_OK
- NA0599_GITHUB_METADATA_SECRET_NAME_REVIEW_OK
- NA0599_ACCESS_MODEL_MATRIX_OK
- NA0599_SELECTED_ACCESS_MODEL_OK
- NA0599_OPERATOR_CODEX_BOUNDARY_OK
- NA0599_REDACTED_DIAGNOSTIC_PLAN_OK
- NA0599_SECURITY_METADATA_REVIEW_OK
- NA0599_PRIVATE_MATERIAL_SCAN_OK
- NA0599_READINESS_MATRIX_OK
- NA0599_NO_SECRET_VALUE_ACCESS_OK
- NA0599_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0599_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0599_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0599_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0599_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0599_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0599_NO_REMOTE_TAILSCALE_WORKFLOW_MUTATION_OK
- NA0599_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0599_NO_PUBLIC_READINESS_CLAIM_OK
- NA0599_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0599_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0599_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0599_RESULT_CLASSIFICATION_SELECTED_OK
- NA0599_SUCCESSOR_SELECTED_OK
- NA0599_ONE_READY_INVARIANT_OK

## Validation Plan

- qwork proof verification: PASS required before fetch, metadata review,
  mutation, PR creation, source-review publication, or proof publication.
- Queue/decision proof: READY_COUNT 1, READY NA-0599, NA-0598 DONE, NA-0597
  DONE, D-1187 once, D-1188 once, D-1189 absent before patch, D-1190 absent,
  duplicate decision count zero.
- Current main health: public-safety success, advisories success,
  suite2-vectors success or conclusively satisfied, no failed or pending
  required checks, root cargo audit success, nested qsc fuzz cargo audit
  success, locked cargo metadata success, and Cargo drift absent.
- Inheritance review: D530, D-1187, and D-1188 consumed.
- Local evidence summary: local qsc/qsl-server, qsl-attachments, true
  triple-ratchet, seed-fallback, exact 4 MiB, and qsl-server fix evidence
  summarized without rerunning local integration.
- Prior remote review: NA-0577 through NA-0586 historical blockers classified.
- Workflow/GitHub metadata review: workflow names, triggers, secret names, and
  variable names reviewed without reading secret values.
- Access model matrix: GitHub-hosted Tailnet runner, self-hosted Tailnet runner,
  public endpoint, local simulation, and manual operator proof compared.
- Redacted diagnostic plan: phase classes defined for Tailnet join, endpoint
  source, DNS, TCP, TLS-or-HTTP, qsl-server route shape, qsl-attachments shape,
  qsc handshake, qsc relay E2EE, and qsc attachment send/receive.
- Private-material and claim scans: endpoint values, private ports, tokens,
  Authorization values, capabilities, payload/body/plaintext, seed/key material,
  raw topology, and overclaims absent from publishable evidence.
- Scope guard: changed files limited to the NA-0599 allowed
  governance/readiness paths.

## Result

Selected result classification:
`REMOTE_TAILNET_REINTRODUCTION_READINESS_OPERATOR_SETUP_REQUIRED`.

Selected successor:
`NA-0600 -- QSL Remote / Tailnet Operator Setup Proof Review Harness`.

Focused runtime tests may be skipped because NA-0599 is readiness and
authorization only, mutates no qsc/qsl-server/qsl-attachments source or runtime
surface, mutates no workflow/dependency/lockfile surface, and performs no
remote/Tailnet execution.
