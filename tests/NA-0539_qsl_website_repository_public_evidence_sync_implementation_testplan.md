Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25
Replaces: n/a
Superseded-By: n/a

# NA-0539 QSL Website / Repository Public Evidence Sync Implementation Testplan

## Scope

This testplan covers the docs/public/governance-only NA-0539 implementation.
It verifies the D-1066 selected path bundle, public claim policy, evidence
citations, redaction rules, queue/decision invariants, and docs-only validation
posture.

## Required Markers

- `NA0539_D1066_AUTHORIZATION_CONSUMED_OK`
- `NA0539_SELECTED_PATH_BUNDLE_ONLY_OK`
- `NA0539_PUBLIC_CLAIM_POLICY_APPLIED_OK`
- `NA0539_EVIDENCE_CITATIONS_PRESENT_OK`
- `NA0539_NO_RAW_PROOF_LOGS_OK`
- `NA0539_NO_PRIVATE_MATERIAL_OK`
- `NA0539_NO_QSC_SOURCE_MUTATION_OK`
- `NA0539_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0539_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0539_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0539_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0539_ONE_READY_INVARIANT_OK`

## Required Validation

- Verify fresh qwork proof files without running qwork/qstart/qresume.
- Verify READY_COUNT 1 and READY NA-0539 before implementation.
- Verify D-1066 and D-1067 exist once, D-1068 is absent before patch, D-1069 is
  absent, and duplicate decision count is zero.
- Verify selected path inventory for the D-1066 path bundle.
- Verify `public/` and `website/` paths are not created or mutated.
- Run `git diff --check`.
- Run exact allowed-path scope guard, including untracked files.
- Run queue/decision proof after patch: READY_COUNT 1, READY NA-0539,
  D-1068 once, D-1069 absent, duplicate decision count zero.
- Run full local markdown link check and changed-public-file link check.
- Run added-line/new-file private-material scan and changed-public-file
  private-material scan.
- Run added-line/new-file overclaim scan and changed-public-file overclaim scan.
- Verify claim matrix contains permitted wording, forbidden wording, evidence
  source, and required qualifier entries.
- Run docs-only classifier.
- Run PR body preflight and goal-lint.
- Run marker proof for all required markers above.
- Run root `cargo audit --deny warnings`.
- Run nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- Run `cargo fmt --check`.
- Run `sh -n scripts/ci/qsc_adversarial.sh`.
- Run `bash -n scripts/ci/qsc_adversarial.sh`.

## Runtime Test Rationale

Focused qsc runtime tests may be skipped because this lane changes only
README/docs/public/governance/testplan files and does not mutate qsc runtime,
qsc source, qsc tests, fuzz files, Cargo files, dependencies, workflows, or
scripts. Cargo audits, formatting, shell syntax, link checks, claim scans, and
scope guards remain required.

## Stop Conditions

Stop if any validation finds:

- more than one READY item;
- READY item other than NA-0539 before closeout;
- D-1068 missing or duplicated after patch;
- D-1069 present during implementation;
- mutation outside the selected path bundle;
- creation or mutation of `public/` or `website/`;
- qsc source/test/fuzz/Cargo mutation;
- dependency or lockfile mutation;
- workflow/script/helper mutation;
- qsl-server or qsl-attachments use or mutation;
- qwork/qstart/qresume execution by Codex;
- qsl-backup execution;
- remote action, SSH execution, qsc send/receive, remote E2EE, or qsc protocol
  command;
- raw proof logs or private material in public docs;
- route-token/capability material in public docs;
- public-readiness, production-readiness, public-internet-readiness,
  external-review-complete, crypto-complete, identity-complete, trust-complete,
  replay-proof, downgrade-proof, secret-material-complete, side-channel-free,
  vulnerability-free, bug-free, or perfect-crypto claim.
