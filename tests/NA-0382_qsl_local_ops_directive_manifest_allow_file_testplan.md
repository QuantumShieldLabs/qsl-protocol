Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0382 QSL Local Ops Directive Manifest and Allow-File Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the standalone qsl-protocol directive manifest and allow-file harness
without workflow, dependency, runtime, qsl-server, qsl-attachments, backup, or
local-history mutation.

## Protected Invariants

- READY_COUNT remains `1` with READY `NA-0382` until closeout.
- D-0746 exists once after implementation.
- D-0747 remains absent until optional closeout.
- public-safety remains required and green.
- No public/readiness/privacy claim is expanded.

## Allowed Scope

- `scripts/ci/qsl_directive_manifest_validate.py`
- `inputs/local_ops/directive_manifest_fixtures/`
- `inputs/local_ops/scope_allow_file_fixtures/`
- `docs/governance/evidence/NA-0382_qsl_local_ops_directive_manifest_allow_file_harness.md`
- `tests/NA-0382_qsl_local_ops_directive_manifest_allow_file_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden scope includes `.github/**`, workflows, Cargo files, dependencies,
runtime/service/protocol/crypto paths, qsl-server, qsl-attachments,
qshield runtime, qsc-desktop, website/public docs, README, START_HERE,
backup scripts/timers/fstab/source lists, `/srv/qbuild/tools/**`, and
local history roots except the required final response file.

## Helper CLI Requirements

Required commands:

- `validate-manifest`
- `validate-allow-file`
- `validate`
- `emit-scope-files`
- `fixture`

Each command must have deterministic exit codes and human-readable output.
JSON summary output is required when `--json` is supplied.

## Manifest Schema Requirements

The manifest must use schema version `qsl.directive_manifest.v1` and strict
top-level key validation. Missing, malformed, mismatched, unknown, or
secret-shaped input must fail closed.

## Allow-File Requirements

Allow-files must support exact repo-relative paths by default, comments,
blank lines, scoped `glob:` entries, and manifest-authorized `local:` entries.
Parent traversal, absolute repo paths, broad globs, hidden repo-wide globs,
normalization ambiguity, and unauthorized local paths must fail closed.

## Fixture Requirements

Fixtures must cover valid manifest, valid allow-file, valid changed paths,
malformed JSON, missing fields, unknown keys, identity mismatches, expected-main
mismatch, successor mismatch, missing public-claim boundary, missing stop
conditions, secret-shaped values, parent traversal, broad globs, forbidden
paths, unlisted paths, forbidden overlay precedence, comments/blanks, and exact
path acceptance.

## Positive Validation Requirements

Positive cases must prove valid manifest, allow-file, changed paths, exact path,
comments/blanks, and temp scope-file emission.

## Negative / Fail-Closed Requirements

Negative cases must prove malformed input, missing required fields, unknown
keys, wrong directive, wrong target, wrong expected main, wrong successor,
missing public boundary, missing stop condition, secret-shaped values, parent
traversal, broad glob, forbidden path, unlisted path, and forbidden overlay
rejection.

## Integration Requirements

`emit-scope-files` must generate helper-compatible files under an explicit
`/srv/qbuild/tmp` directory. Existing `qsl_evidence_helper.py scope-guard`
must consume those files without mutating `qsl_evidence_helper.py`.

## No-Network / No-Mutation Requirements

The helper must not call network, GitHub, branch mutation, push, merge,
workflow, dependency, runtime, backup, restore, qsl-server, or qsl-attachments
operations.

## No-Secret Requirements

Secret-shaped values must fail closed. Fixtures must not contain real secrets,
tokens, credentials, keys, passphrases, or private material.

## Backup-Impact Requirements

Tracked helper and fixtures require no backup-plan update. Proof logs and
emitted compatibility files must remain temporary under `/srv/qbuild/tmp`.

## Public-Claim Boundary Requirements

The lane must not claim production readiness or public internet readiness, and
must make no external review completion, anonymity, metadata-free messaging,
untraceability, complete disaster recovery, off-host backup completion, target
setup, host identity verification, real restore completion, or key
custody/recovery claim.

## Successor Selection Requirements

Selected successor is:

`NA-0383 -- QSL Local Ops Response Writer Implementation Authorization Plan`

NA-0383 must not be implemented by NA-0382.

## Required Local Checks

Run and record:

```bash
python3 scripts/ci/qsl_directive_manifest_validate.py --help
python3 scripts/ci/qsl_directive_manifest_validate.py fixture --fixture-dir inputs/local_ops/directive_manifest_fixtures --allow-fixture-dir inputs/local_ops/scope_allow_file_fixtures
python3 -m py_compile scripts/ci/qsl_directive_manifest_validate.py
python3 scripts/ci/qsl_bounded_check_poll.py --help
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed-file <allowed> --forbidden-file <forbidden>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI Expectations

PR checks must attach and pass normally. public-safety must remain required and
green before merge and after merge. No admin bypass, squash, rebase, direct
push, force-push, amend, or branch deletion is authorized.

## Successor Handoff

If Packet K runs, mark NA-0382 DONE and restore NA-0383 READY without
implementing NA-0383.
