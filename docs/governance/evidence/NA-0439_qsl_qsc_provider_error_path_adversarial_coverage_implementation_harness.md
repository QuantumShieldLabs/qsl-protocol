Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

# NA-0439 qsc Provider Error Path Adversarial Coverage Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0439 implements the NA-0438-authorized qsc adversarial harness integration by
adding the already-existing deterministic provider-error no-mutation test to:

`scripts/ci/qsc_adversarial.sh`

The integrated command is:

```bash
cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test handshake_provider_error_no_mutation -- --test-threads=1
```

The command runs after the existing stable qsc adversarial Rust test phases and
before the cargo-fuzz smoke targets. The script also emits:

`NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP`

This makes qsc-adversarial smoke consume the bounded `pq_decap_failed`
no-mutation evidence from NA-0436. It does not create executable coverage for
`pq_encap_failed`.

## Live NA-0439 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0439 -- QSL qsc Provider Error Path Adversarial Coverage Implementation Harness`

Status: READY.

Allowed NA-0439 implementation path:

- `scripts/ci/qsc_adversarial.sh`

Allowed NA-0439 governance paths:

- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden current-lane mutation scope includes runtime code, crypto code,
dependencies, Cargo files, lockfiles, workflows, executable tests, fuzz targets,
vectors, qsl-server, qsl-attachments, qshield runtime, website, public docs,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback subtree, backup tree, and backup/local-ops state.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

The qwork proof files were read from:

- `/srv/qbuild/work/NA-0439/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0439/.qwork/startup.qsl-protocol.json`

Required proof markers passed:

- `startup_result=OK`
- `lane=NA-0439`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0439/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0439`
- `requested_lane_status=READY`

The JSON proof was valid and mirrored the `.kv` proof for lane, repo, path,
HEAD, origin/main, ready count, top READY item, requested lane status, and clean
state. Live `HEAD` and `origin/main` both matched proof SHA
`c930e8eff627`. `origin/main` equals or descends from PR #1146 merge commit
`c930e8eff627`.

Proof root:

`/srv/qbuild/tmp/NA0439_provider_error_adversarial_impl_20260607T170013Z`

## NA-0438 authorization inheritance

NA-0438 selected:

`PROVIDER_ERROR_ADVERSARIAL_COVERAGE_IMPLEMENTATION_AUTHORIZED`

D-0863 authorized the future NA-0439 mutable implementation path as exactly:

- `scripts/ci/qsc_adversarial.sh`

NA-0438 also authorized only NA-0439 governance evidence/testplan, DECISIONS,
TRACEABILITY, and rolling journal updates. It did not authorize runtime, crypto,
dependency, Cargo, lockfile, workflow, executable-test, fuzz-target, vector,
public-surface, service, backup, qsl-backup, or qwork mutation.

Inherited provider-error boundary:

- NA-0436 provides bounded executable no-mutation evidence for
  `pq_decap_failed`.
- NA-0437 documents `pq_encap_failed` as a defensive branch under current active
  provider and qsc external API evidence.
- NA-0439 must not claim executable coverage for `pq_encap_failed`.

## Pre-mutation adversarial script review

Preimage:

- SHA256: `562933d06325c0146e05d9e5ecf062e2a1a0f5c7871409679ff3bda019584655`
- Size: `775`
- Owner/group: `victor:victor`
- Mode: `664`
- Executable: no
- Shebang: `#!/bin/sh`

Existing script phases:

1. `cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_properties`
2. `cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test adversarial_miri`
3. `run_fuzz_target qsc_route_http`
4. `run_fuzz_target qsc_payload_boundaries`
5. `run_fuzz_target qsc_vault_envelope`

Rollback copy:

`$PROOF_DIR/rollback/qsc_adversarial.sh.preimage`

Rollback SHA matched the script preimage SHA before mutation.

## Script integration implementation

Only `scripts/ci/qsc_adversarial.sh` was changed for implementation.

The new step was inserted after the stable qsc adversarial Rust phases and
before the cargo-fuzz targets:

```sh
echo "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP"
cargo +stable test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test handshake_provider_error_no_mutation -- --test-threads=1
```

This uses the script's existing `cargo +stable test --manifest-path
qsl/qsl-client/qsc/Cargo.toml --locked --test ...` convention. It is equivalent
to the workspace-package command authorized by the directive and keeps the qsc
test invocation aligned with the existing script style.

Post-edit script state:

- SHA256: `763f9c636eb3d0f65da97d0039f5edecc8cac326ec0d45eea06384c3ce87a141`
- Mode: `664`
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS

No cargo-fuzz target, workflow, Cargo file, lockfile, runtime file, crypto file,
test source, or vector was changed.

## Provider-error test command proof

Direct command:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Result: PASS, one test.

Observed markers:

- `NA0436_PQ_DECAP_FAILED_MARKER_OK`
- `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
- `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
- `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
- `NA0436_NO_RUNTIME_HOOK_USED_OK`

## Local adversarial script validation

Local script command:

```bash
sh scripts/ci/qsc_adversarial.sh
```

Local result: exited 101 after the new provider-error step passed.

Passing pre-fuzz phases:

- `adversarial_properties`: PASS, 8 tests.
- `adversarial_miri`: PASS, 6 tests.
- `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP`: emitted once.
- `handshake_provider_error_no_mutation`: PASS, 1 test.

Recovered local tooling caveat:

- Failing command: `sh scripts/ci/qsc_adversarial.sh`
- Classification: recoverable local cargo-fuzz availability caveat, because all
  stable Rust adversarial phases and the new provider-error test passed before
  the script reached cargo-fuzz.
- Failure output: `error: no such command: fuzz`
- Corrective action: no local toolchain or dependency mutation; require PR CI
  `qsc-adversarial-smoke` as the cargo-fuzz-backed proof before merge.
- Final local result: new provider-error no-mutation test ran and passed before
  cargo-fuzz unavailability stopped the local script.

## PR/CI qsc-adversarial validation

Before PR creation, PR CI status is pending by definition.

Merge requirement:

- `public-safety` must complete success.
- `qsc-adversarial-smoke` must complete success.
- `qsc-adversarial-miri` must complete success if attached/required by the check
  shape.

The PR must not merge while PR `qsc-adversarial-smoke` is failed, missing after
bounded wait, or ambiguous.

## `pq_decap_failed` adversarial coverage proof

The integrated test is bounded to `pq_decap_failed`.

The test corrupts Alice's test-local pending KEM secret state after a valid B1
is generated, observes `pq_decap_failed`, and asserts that Alice/Bob session
state plus Alice pending/vault state are unchanged by the reject.

The qsc adversarial script now consumes that evidence before any cargo-fuzz
phase can run or fail.

## `pq_encap_failed` caveat preservation proof

`pq_encap_failed` remains documented as a defensive branch under current active
provider and qsc external API evidence.

NA-0439 does not add executable `pq_encap_failed` coverage. Future executable
coverage for that branch would require separately authorized test seam,
provider fake, provider behavior change, or equivalent exact scope.

The direct test marker `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK` passed.

## Root and nested dependency health proof

Root dependency health:

- `cargo audit --deny warnings`: PASS.
- `cargo tree -i rustls-webpki --locked`: PASS, `rustls-webpki v0.103.13`.
- `cargo tree -i ml-kem --locked`: PASS, `ml-kem v0.2.1`.
- `cargo tree -i pqcrypto-mlkem --locked`: expected package-ID absence.
- `cargo tree -i pqcrypto-traits --locked`: expected package-ID absence.
- `cargo tree -i pqcrypto-internals --locked`: expected package-ID absence.

Nested qsc fuzz lock health:

- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`: PASS.
- Residual `pqcrypto-*` scan in nested fuzz lock: zero matches.

Cargo audit green is dependency-health evidence only.

## No runtime / crypto / dependency / Cargo / lockfile / workflow / test / fuzz target / vector mutation proof

Implementation mutation was limited to:

- `scripts/ci/qsc_adversarial.sh`

Governance mutation was limited to:

- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable-test,
fuzz-target, vector, qsl-server, qsl-attachments, qshield runtime, website,
public-doc, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback subtree, or backup tree path was mutated.

## Public claim/external review/website boundary

This adversarial script integration is internal engineering evidence only.

It is not production readiness, not public-internet readiness, not external
review completion, not crypto completeness, not side-channel freedom, not
bug-free status, not vulnerability-free status, and not perfect-crypto status.

No README, START_HERE, website, docs/public, or public technical paper content
was updated.

## Rejected alternatives

- Runtime hook or provider seam: rejected as out of NA-0439 scope.
- Fuzz target mutation: rejected as out of NA-0439 scope.
- Workflow mutation: rejected as unnecessary and out of scope.
- Dependency or Cargo change: rejected as unnecessary and out of scope.
- Claiming `pq_encap_failed` executable coverage: rejected because current
  evidence remains defensive-branch documentation only.
- Skipping local cargo-fuzz failure by editing the script: rejected because it
  would weaken qsc-adversarial coverage.

## Backup-impact statement

Codex did not run backup, restore, sudo, qwork, qstart, or qresume.

Read-only qsl-backup boundary proof:

- `/usr/local/sbin/qsl-backup` SHA256:
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`
- `/home/victor/work/qsl/codex/ops` source inclusion count: `1`

No qsl-backup, backup status, backup plan, rollback subtree, or backup tree path
was mutated.

## Selected successor

Selected successor after successful PR CI and merge:

`NA-0440 -- QSL qsc Provider Error Path Formal / Model Alignment Authorization Plan`

NA-0440 must be authorization-only unless separately scoped. It should consume
the `pq_decap_failed` deterministic/adversarial evidence and the
`pq_encap_failed` defensive-branch caveat without changing runtime code,
dependencies, workflows, tests, fuzz targets, vectors, public surfaces, or
backup/local-ops state.

## Next recommendation

Merge NA-0439 only after `public-safety` and `qsc-adversarial-smoke` pass. Then
close out NA-0439 and restore NA-0440 as the sole READY item if post-merge
public-safety and qsc-adversarial evidence are green.
