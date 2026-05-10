Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# NA-0262 Demo Adversarial Stress Audit

Directive: QSL-DIR-2026-05-10-060 / NA-0262

## Objective

Add and run a bounded non-production public-demo adversarial stress, chaos, and
abuse harness. The harness must preserve fail-closed behavior, secret hygiene,
panic-safe operator output, and non-production posture without protocol/crypto
state-machine changes, qsl-server or qsl-attachments production hardening,
website changes, workflow changes, branch-protection changes, public-safety
changes, or Cargo dependency changes.

## Baseline Proof

- Starting `origin/main`: `5fe9ec1ccc`.
- Starting PR #776 state: merged as `5fe9ec1ccc`.
- PRs #775 through #757 and PR #708: merged.
- PR #750 and PR #722: closed and unmerged.
- Branch protection: expected protected contexts present, including
  `public-safety`; force pushes and deletions disabled; admin enforcement
  enabled.
- Latest starting-main `public-safety`: success.
- Queue proof before edits: `READY_COUNT 1`, sole READY `NA-0262`.
- Decision proof before edits: D-0493 existed once; D-0494 and D-0495 absent;
  duplicate decision count zero.
- NA-0262A cost-control self-test and path-classification smoke passed.

## Packet A Surfaces Inspected

- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- `apps/qshield-cli/**`.
- `docs/demo/**`.
- `docs/governance/evidence/NA-0260_attachment_demo_readiness_audit.md`.
- `docs/governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md`.
- `docs/governance/evidence/NA-0257_cross_host_demo_reproducibility_audit.md`.
- `docs/governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md`.
- `tests/NA-0261_closeout_restore_na0262_testplan.md`.
- `NEXT_ACTIONS.md` NA-0262 entry.

## Packet A Result

The current demo surface already supports the required minimum proof categories:

- positive local qshield send/receive/decrypt;
- missing-auth reject;
- malformed-input reject;
- establish replay reject;
- invalid relay-id reject;
- attachment integrity reject;
- KT-negative reject and KT accepted-state no-mutation proof;
- queue/cap/rate proof through current relay behavior; and
- no-secret/no-panic transcript checks.

The selected implementation is a bounded wrapper harness that reuses existing
demo smoke evidence for the positive, attachment, and KT paths, and adds a
direct local relay abuse pass for wrong-token, wrong-scheme, empty-auth,
malformed, queue-cap, no-mutation, replay, invalid-id, and controlled
kill/restart checks.

## Implementation Summary

- Added `scripts/ci/demo_adversarial_stress.sh`.
- Added `docs/demo/DEMO_ADVERSARIAL_STRESS_TESTING.md`.
- Added this audit.
- Added `tests/NA-0262_demo_adversarial_stress_testplan.md`.
- Added D-0494 and traceability links.
- Added a rolling operations journal entry for the directive.

No qshield CLI source hardening was required because the harness did not find a
token/secret/plaintext leak or user-facing panic in the tested demo paths.

## Commands Run

Hard-start and preflight proof included:

```bash
df -BG /srv/qbuild
git status --porcelain=v1 --branch
git fetch --all --prune
git rev-parse origin/main
gh pr view 776 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 775 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 774 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 773 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 772 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 771 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 770 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 769 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 768 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 767 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 766 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 765 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 764 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 763 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 762 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 761 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 750 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0262A_closeout_restore_na0262_testplan.md
bash scripts/ci/classify_ci_scope.sh qsl/qsl-client/qsc/src/main.rs
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md qsl/qsl-client/qsc/src/main.rs
bash scripts/ci/classify_ci_scope.sh
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Stress proof command:

```bash
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
```

Additional validation commands are recorded in the directive response and
rolling journal after the validation bundle completes.

## Transcript Artifact

Latest local artifact directory:

```text
/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260510T213151Z/
```

Expected transcript files:

- `demo_adversarial_stress_transcript.log`
- `demo_adversarial_stress_markers.log`
- `direct_relay_abuse.log`
- `demo_cli_smoke.log`

Baseline runtime:

```text
11 seconds
```

## Markers

The baseline harness must emit:

```text
DEMO_STRESS_POSITIVE_BASELINE_OK
DEMO_STRESS_AUTH_REJECT_OK
DEMO_STRESS_MALFORMED_REJECT_OK
DEMO_STRESS_REPLAY_REJECT_OK
DEMO_STRESS_RELAY_ID_REJECT_OK
DEMO_STRESS_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_STRESS_KT_REJECT_OK
DEMO_STRESS_QUEUE_OR_RATE_BOUND_OK
DEMO_STRESS_CHAOS_RECOVERY_OK
DEMO_STRESS_NO_SECRET_LEAK_OK
DEMO_STRESS_NO_PANIC_OK
NA0262_DEMO_ADVERSARIAL_STRESS_OK
```

Unsupported baseline-only category:

```text
UNSUPPORTED_PORT_IN_USE_BASELINE: extended profile only
```

The local baseline run emitted all required markers listed above.

## Positive Outcome

The positive path is accepted only after `scripts/ci/demo_cli_smoke.sh` passes
after the direct stress phase and emits `DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK`.
This proves the normal non-production qshield demo path still works after the
direct abuse/chaos checks.

## Negative Outcomes

The harness directly proves missing, wrong, wrong-scheme, and empty auth
rejects; malformed JSON, wrong content type, empty body, and bounded oversized
body rejects; invalid relay-id reject; establish replay reject; recipient
queue cap; and controlled relay restart recovery.

The harness reuses the existing demo smoke for attachment tamper/integrity and
KT-negative markers because those are the current truthful demo proof surfaces.

## No-Leak and No-Panic Outcome

The direct relay pass injects a sentinel and relay token into rejected inputs
and scans the artifact bundle without printing the live token. The child demo
smoke retains its own no-leak checks for its generated token and sentinels.

The final harness scan rejects panic, backtrace, and unwrap-output patterns in
artifact output before printing `DEMO_STRESS_NO_PANIC_OK`.

## No-Mutation Outcome

The direct relay pass proves an unauthorized send does not mutate an observable
recipient queue. KT no-mutation remains tied to the existing canonical verifier
test invoked by the demo smoke. Attachment reject no-output behavior remains
tied to the existing tampered attachment demo smoke. No broader relay-internal
state no-mutation claim is made.

## No-Production-Overclaim

This audit supports only non-production local public-demo stress evidence. It
does not support production hardening, public internet exposure, qsl-server
production readiness, qsl-attachments production readiness, or release
approval.

## Residual Gaps

- Real cross-host/private-network stress remains a successor lane.
- Desktop/sidecar stress remains separate.
- Port-in-use is extended-profile evidence, not baseline evidence.
- qsl-server and qsl-attachments production hardening remain separate.
- The harness is not unbounded fuzzing or denial-of-service testing.

## Recommendations

1. Merge only after the baseline harness, demo smoke, metadata smoke, helper
   checks, leak/link checks, dependency checks, and required CI pass normally.
2. Keep NA-0262 READY after the implementation/evidence PR until a separate
   closeout promotes exactly one successor.
3. Use NA-0263 for cross-host/private-network stress proof rather than
   overstating this local harness.
