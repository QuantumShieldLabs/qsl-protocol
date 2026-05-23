Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0338 Closeout and NA-0339 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0338 is closed after the authorization/design PR merged and
that the exact NA-0339 successor selected by D-0658 is restored as the sole
READY queue item without implementing NA-0339.

## Protected Invariants

- NA-0338 is DONE.
- Exactly one READY item exists: NA-0339.
- NA-0339 is exactly
  `Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`.
- D-0658 exists exactly once.
- D-0659 exists exactly once.
- D-0660 is absent.
- No NA-0339 implementation is included.
- No attachment size-class padding implementation is included.
- No qshield runtime behavior changes are included.
- No qsl-server or qsl-attachments behavior changes are included.
- No qsc/qsp/protocol/crypto/key-schedule behavior changes are included.
- No dependency, workflow, branch-protection, public-safety, website, README,
  START_HERE, docs/public, qsc-desktop, formal, input, tool/refimpl, app
  runtime, or service implementation change is included.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- No claim says attachment size, timing metadata, traffic shape, or all
  metadata is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0338_closeout_restore_na0339_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, service, workflow, dependency, or public
  copy paths
- branch-protection or public-safety configuration
- branch deletion

## Packet K Prerequisites

Before closeout:

- qsl-protocol PR #938 must be merged.
- Post-merge main `public-safety` must be completed with conclusion success.
- READY_COUNT must be 1 and READY must be NA-0338.
- D-0658 must exist.
- D-0659 must be absent.
- The selected successor must be exact.

## Queue Requirements

`NEXT_ACTIONS.md` must:

- mark NA-0338 DONE;
- record qsl-protocol PR #938 merge evidence;
- record the attachment size-class authorization result;
- record selected successor
  `NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`;
- record D-0658 and D-0659;
- restore NA-0339 as READY;
- preserve exactly one READY item.

## Decision Requirements

`DECISIONS.md` must add D-0659 with:

- title `NA-0338 closeout and NA-0339 restoration`;
- Goals G1, G2, G3, G4, G5;
- statement that NA-0338 delivered the attachment size-class authorization
  result;
- statement that NA-0339 is selected based on NA-0338 evidence;
- statement that no NA-0339 implementation is authorized by closeout;
- statement that metadata reduction remains bounded and not overclaimed.

## Traceability Requirements

`TRACEABILITY.md` must link:

- qsl-protocol PR #938;
- D-0659;
- this closeout testplan;
- selected NA-0339 successor;
- qshield embedded relay/demo and qsl-server/qsl-attachments production
  boundaries.

## Claim-Boundary Requirements

Do not claim:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- padding hides all metadata;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public internet readiness;
- external review completion;
- quantum-proof hype, unbreakable, military-grade, or guaranteed secure
  properties.

Allowed only when explicitly negated, prohibited, or classified as future/
unproven.

## Required Local Checks

Run:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD` with exact allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-path classifier proof
- high-risk phrase scan over changed lines
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- goal-lint after PR creation

## CI Expectations

Before merge:

- required checks must attach to the PR head;
- required checks must complete with no failing conclusion;
- `public-safety` must be required by branch protection and green;
- merge must use normal merge with `--match-head-commit`;
- no admin bypass, squash, rebase, direct push, branch deletion, or
  delete-branch flag is allowed.

After merge:

- `origin/main` must contain D-0659;
- READY_COUNT must be 1 and READY must be NA-0339;
- D-0660 must remain absent;
- post-merge main `public-safety` must complete success.

## Successor Handoff

The restored successor is:

`NA-0339 -- Metadata Runtime qshield Demo Attachment Size-Class Implementation Harness`

NA-0339 must not be implemented by this closeout.
