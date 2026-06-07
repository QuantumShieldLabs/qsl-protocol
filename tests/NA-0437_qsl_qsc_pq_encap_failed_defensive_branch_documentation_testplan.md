# NA-0437 qsc pq_encap_failed Defensive Branch Documentation Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0437 documents `pq_encap_failed` as a current defensive
provider-error branch without claiming executable coverage, references the
NA-0436 `pq_decap_failed` no-mutation test only within its true boundary, and
selects the exact NA-0438 successor without implementation mutation.

## Protected invariants

- `pq_encap_failed` is documented as defensive under current active provider and
  qsc external API evidence.
- `pq_encap_failed` executable coverage is not claimed.
- `pq_decap_failed` test evidence remains bounded to that marker.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, or vector path is changed.
- No qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
  START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
  plan, rollback, or `/backup/qsl` path is changed.
- Exactly one READY item remains.
- Public-safety remains green before and after merge.

## Allowed scope

- `docs/governance/evidence/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_evidence_plan.md`
- `tests/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback,
and `/backup/qsl` paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, and dependency remediation commands.

## qwork proof check

Confirm the qwork proof files exist and are read without rerunning qwork:

```bash
test -f /srv/qbuild/work/NA-0437/.qwork/startup.qsl-protocol.kv
test -f /srv/qbuild/work/NA-0437/.qwork/startup.qsl-protocol.json
```

Required markers:

- `startup_result=OK`
- `lane=NA-0437`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0437/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0437`
- `requested_lane_status=READY`

The JSON proof must parse and mirror the `.kv` proof for lane, repo, path, head,
origin/main, clean-state fields, READY count, queue top, and requested lane
status.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required before PR creation:

- READY_COUNT 1.
- READY NA-0437.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0433 DONE.
- NA-0432 DONE.
- NA-0431 DONE.
- NA-0430 DONE.
- NA-0429 BLOCKED.
- D-0859 exists once.
- D-0860 exists once.
- D-0861 exists once after this patch.
- D-0862 absent.
- duplicate decision count zero.

## Source-review checks

Confirm source review covers:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`;
- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`;
- `tools/refimpl/quantumshield_refimpl/src/crypto/`;
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`;
- D278, D279, D280, D281, and D282 response evidence.

The evidence must state:

- where `pq_encap_failed` appears;
- the condition required to reach it;
- that wrong-length A1 KEM public keys fail qsc frame decode before provider
  encapsulation;
- that D278 correct-length malformed public-key byte patterns did not make the
  active provider fail encapsulation;
- that NA-0436 is decap-only;
- that future executable `pq_encap_failed` coverage would require future exact
  authorization for a provider fake, test seam, provider behavior differential,
  or equivalent proof.

## Defensive branch classification check

Required classification:

`PQ_ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTED`

Required caveat:

`PQ_ENCAP_FAILED_PROVIDER_BEHAVIOR_DEPENDENT`

The documentation must not classify this as executable coverage.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_evidence_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_testplan.md`

## Link, leak, classifier, and PR-body checks

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/governance/evidence/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_evidence_plan.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_testplan.md
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
```

Required:

- no whitespace errors;
- link check passes;
- added-line leak scan has zero findings;
- scope guard accepts exactly the allowed paths;
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`;
- PR body does not contain prohibited public-claim phrases.

## Dependency and provider-health checks

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required:

- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- `rustls-webpki` is `v0.103.13` or newer safe version;
- `ml-kem` remains the active provider family;
- root pqcrypto inverse-tree probes are absent or explicitly explained as
  expected zero-match proofs;
- nested qsc fuzz lock pqcrypto residual scan returns zero matches.

## Regression validation checks

Run:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- the NA-0436 `pq_decap_failed` test still passes;
- markers remain present:
  - `NA0436_PQ_DECAP_FAILED_MARKER_OK`
  - `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
  - `NA0436_NO_RUNTIME_HOOK_USED_OK`
- qsc send_commit tests pass;
- provider `pqkem768` tests pass;
- formatting check passes;
- formal model checks pass.

## qsc adversarial smoke

Run if feasible:

```bash
scripts/ci/qsc_adversarial.sh
```

If the script is not executable:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record the exact output and require PR CI
`qsc-adversarial-smoke` as the authoritative smoke proof.

## Public claim and website boundary check

Confirm:

- this is internal governance evidence only;
- no production readiness claim is introduced;
- no public-internet readiness claim is introduced;
- no external-review completion claim is introduced;
- no crypto-complete claim is introduced;
- no side-channel-free claim is introduced;
- no bug-free claim is introduced;
- no vulnerability-free claim is introduced;
- no perfect-crypto claim is introduced;
- no public technical paper content is introduced;
- no README, START_HERE, public docs, or website path is changed;
- cargo audit green is dependency-health evidence only;
- `pq_encap_failed` defensive branch documentation does not claim executable
  coverage;
- `pq_decap_failed` test evidence remains bounded to that marker.

## Post-merge checks

After merge, verify:

- READY remains NA-0437 until optional closeout;
- D-0861 exists on main;
- public-safety is green on the merge commit;
- no qwork post-merge command was run by Codex.
