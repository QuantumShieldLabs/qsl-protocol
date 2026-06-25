Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25
Replaces: n/a
Superseded-By: n/a

# NA-0539 QSL Website / Repository Public Evidence Sync Implementation Harness

## Result

Classification: `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_PASS`

NA-0539 implements the D-1066 selected repository public evidence sync path
bundle and claim policy. It updates repository-facing public docs and
governance records so readers can find QSL goals, bounded qsc evidence,
residual limits, and review invitation without a public-readiness,
production-readiness, external-review-complete, or crypto/security-completeness
claim.

## D450 / D449 / D448 / D446 / D441 / D439 / D419 Inheritance

- D450 closed NA-0538 and restored NA-0539 as the sole READY item; no NA-0539
  implementation was performed in D450.
- D449 selected `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_READY`.
- D449 selected the exact path bundle consumed here: `README.md`,
  `docs/README.md`, `docs/public/INDEX.md`,
  `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`,
  `docs/public/EXTERNAL_REVIEW_PACKAGE.md`,
  `docs/public/WEBSITE_CLAIM_MATRIX.md`, this evidence doc, the NA-0539
  testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- D449 selected the public claim wording policy and proof/redaction rules used
  here.
- D448 closed NA-0537 and restored NA-0538.
- D446 recorded repeated-run cleanup/freshness pass, retained-qsc freshness,
  no stale state reuse, selected wrong-peer and stale/replaced-peer negatives,
  no qsl-server, no qsl-attachments, and no public/production/security
  completion claim.
- D441 recorded selected wrong-peer/stale-trust pass.
- D439 recorded the port diagnostic marker traversal and ACK pass.
- D419 recorded selected replay/corrupt negative pass.
- No qsl-server or qsl-attachments evidence is promoted into production or
  public-internet readiness.
- No inherited evidence is treated as public readiness, production readiness,
  external review completion, crypto completeness, identity completeness,
  trust completeness, replay-proof status, downgrade-proof status,
  side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## Path Bundle Implemented

- `README.md`
- `docs/README.md`
- `docs/public/INDEX.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/public/WEBSITE_CLAIM_MATRIX.md`
- `docs/governance/evidence/NA-0539_qsl_website_repository_public_evidence_sync_implementation_harness.md`
- `tests/NA-0539_qsl_website_repository_public_evidence_sync_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No `public/` or `website/` path was created or mutated.

## Public Evidence Sync Content

The public docs now describe:

- QSL goals and research-stage mission;
- bounded direct qsc client-to-client E2EE evidence using synthetic data;
- same-host qsc tests;
- retained qsc staging/restaging and freshness checks;
- SSH reverse-forward marker/ACK evidence;
- Build-to-Inspiron qsc E2EE success;
- selected replay/corrupt delivery negatives;
- selected wrong-peer and stale/replaced-peer negatives;
- repeated-run cleanup/freshness;
- public-safety/advisories green gates;
- quinn-proto RUSTSEC-2026-0185 remediation baseline;
- bounded formal/model checks;
- corpus validators and secret-material scans;
- qsl-server and qsl-attachments deferral;
- review invitation focused on evidence, limits, and next steps.

## Claim Policy Application

Permitted wording is bounded to engineering evidence. The docs explicitly keep:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no identity-complete claim;
- no trust-complete claim;
- no replay-proof claim;
- no downgrade-proof claim;
- no secret-material-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## Proof / Redaction Rules

The public docs cite decision IDs, evidence doc names, testplans,
TRACEABILITY, bounded classifications, and no-claim boundaries. They do not
copy raw proof logs, private keys, passphrases, tokens, passwords, production
endpoints, backup material, raw qsc runtime material, route-token/capability
material, raw SSH config, `authorized_keys`, `known_hosts`, or detailed remote
topology.

## Boundaries

- No qsc source/test/fuzz/Cargo mutation occurred.
- No dependency or lockfile mutation occurred.
- No workflow/script/helper mutation occurred.
- No corpus/vector/input mutation occurred.
- No formal/refimpl/service/backup mutation occurred.
- No qsl-server or qsl-attachments use or mutation occurred.
- No remote action, SSH execution, qsc send/receive, remote E2EE, or qsc
  protocol command occurred in NA-0539.
- Codex did not run qwork, qstart, qresume, qsl-backup, qsl-server, or
  qsl-attachments.

## Validation Summary

Required local validation for this docs/governance-only implementation includes:

- `git diff --check`;
- exact allowed-path scope guard, including untracked files;
- queue/decision proof with READY_COUNT 1 and D-1068 exactly once;
- link checks;
- changed-public-file link checks;
- added-line/new-file private-material scans;
- changed-public-file private-material scans;
- added-line/new-file overclaim scans;
- changed-public-file overclaim scans;
- claim matrix proof;
- docs-only classifier;
- PR body preflight and goal-lint;
- marker proof;
- root and nested qsc fuzz lock cargo audits;
- `cargo fmt --check`;
- `sh -n scripts/ci/qsc_adversarial.sh`;
- `bash -n scripts/ci/qsc_adversarial.sh`.

Focused qsc runtime tests are not required for this lane because the mutation is
docs/public/governance-only and does not touch qsc runtime/source/dependency or
workflow files.

## Successor Selection

Success classification selects:

`NA-0540 -- QSL Local Ops SSD Hygiene / Shared Cargo Target Authorization Plan`

NA-0540 must be authorization-only unless a later directive explicitly
implements local-ops scripts, timers, qwork changes, or shared target behavior.
