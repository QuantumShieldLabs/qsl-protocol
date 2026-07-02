Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0588 Closeout and NA-0589 Restoration Testplan

## Required Markers

- NA0588_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0588_CLOSEOUT_D1167_ACCEPTED_OK
- NA0588_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0588_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0588_CLOSEOUT_SUITE2_GREEN_OK
- NA0588_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0588_CLOSEOUT_D1168_RESTORED_NA0589_OK
- NA0588_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0588_CLOSEOUT_NO_NA0589_IMPLEMENTATION_OK
- NA0588_CLOSEOUT_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0588_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0588_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0588_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0588_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0588_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0588_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0588_CLOSEOUT_ONE_READY_INVARIANT_OK

## Classification Proof

- Implementation PR: #1450 merged at `94a3b50d0189`.
- Implementation head: `9ae74ed2d867`.
- D-1167 exists once and is Accepted.
- D-1167 result classification:
  `LOCAL_QSC_QSL_SERVER_E2EE_ADVERSARIAL_METADATA_STRESS_PASS`.
- D-1167 selected exact successor:
  `NA-0589 -- QSL Local qsl-attachments Integration Readiness Harness`.
- Post-merge public-safety: success.
- Post-merge advisories: success.
- Post-merge suite2 checks: success.
- Failed or pending checks before closeout: none.
- D-1168 restores NA-0589 READY and marks NA-0588 DONE.

## Boundary Proof

No NA-0589 implementation is introduced. No qsl-attachments command, runtime,
integration, direct build lane, source mutation, dependency mutation, or
lockfile mutation is introduced.

No endpoint value beyond loopback class, private port value, token value,
Authorization value, route-token/capability value, bearer value, payload,
response body, plaintext fixture content, envelope body, process identity,
private topology, key material, secret environment value, raw qsl-server log,
or raw qsc output is published.

No qsl-protocol source/script/workflow/dependency mutation, qsl-server
source/test/runtime mutation, qsc send/receive, qwork/qstart/qresume execution,
remote action, Tailscale action, workflow dispatch/rerun, public-site mutation,
or Cloudflare mutation is introduced.

No public readiness claim is introduced. No production readiness claim is
introduced. No security-completion claim is introduced. No vulnerability-free
claim is introduced. No bug-free claim is introduced. No perfect-build claim is
introduced. No perfect-crypto claim is introduced.

## Validation

Required validation covers diff/scope/queue/marker proof, link check,
private-material scan, overclaim scan, PR body preflight, goal-lint where
available, root cargo audit, nested qsc fuzz cargo audit, locked cargo metadata,
cargo fmt, and qsc adversarial shell syntax checks.
