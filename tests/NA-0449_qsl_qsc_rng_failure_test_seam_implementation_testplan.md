# NA-0449 QSL qsc RNG Failure Test Seam Implementation Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0449 implements only the exact qsc RNG failure test-only seam
authorized by D-0883, proves selected fail-closed/no-write behavior, preserves
normal production semantics without the custom cfg, and avoids dependency,
workflow, public-claim, service, backup, and qwork mutation.

## Protected invariants

- READY_COUNT remains 1.
- NA-0449 remains READY until optional closeout.
- NA-0448 through NA-0435 remain DONE.
- NA-0434 remains BLOCKED.
- NA-0429 remains BLOCKED.
- D-0885 exists once after the implementation patch.
- D-0886 remains absent until optional closeout.
- Duplicate decision ID count remains zero.
- Exact implementation paths are limited to:
  - `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
  - `qsl/qsl-client/qsc/src/handshake/mod.rs`
  - `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
  - `qsl/qsl-client/qsc/src/vault/mod.rs`
- Exact governance paths are limited to this NA-0449 evidence/testplan,
  `DECISIONS.md`, `TRACEABILITY.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- No dependency, Cargo manifest, lockfile, workflow, fuzz target, vector,
  formal model, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
  public-doc, website, README, START_HERE, qwork/qstart/qresume/qshell,
  backup, restore, qsl-backup, status, plan, rollback, or backup tree path is
  changed.
- No fallback random bytes are introduced.
- No deterministic replacement random bytes are introduced.
- No fallback-to-success behavior is introduced.
- Normal builds without `--cfg qsc_rng_failure_test_seam` do not read the seam
  selector and keep production behavior unchanged.
- No public claim expansion is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `docs/governance/evidence/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime/crypto source outside the exact three
qsc source paths, dependency metadata, Cargo manifests, lockfiles, workflows,
executable test source outside the exact qsc test file, fuzz target source,
vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands,
workflow mutation, branch-protection mutation, and public technical paper work.

## Required test names

The exact qsc test must include:

- `normal_build_ignores_test_seam_selector`
- `common_na0449_markers`
- `handshake_session_id_rng_failure_writes_no_pending_state`
- `vault_rng_failure_writes_no_vault_file`
- `session_store_rng_failure_writes_no_session_blob`

## Required markers

The exact qsc test output or source must include:

- `NA0449_RNG_FAILURE_AUTHORIZATION_CONSUMED_OK`
- `NA0449_RNG_FAILURE_TEST_SEAM_IMPLEMENTED_OK`
- `NA0449_RNG_FAILURE_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK`
- `NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK`
- `NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK`
- `NA0449_NO_DEPENDENCY_CHANGE_OK`
- `NA0449_NO_WORKFLOW_CHANGE_OK`
- `NA0449_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0449_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0449_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0449_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0449_ONE_READY_INVARIANT_OK`

## qwork proof and startup checks

Run:

```bash
python3 - <<'PY'
import json, pathlib
kv = pathlib.Path('/srv/qbuild/work/NA-0449/.qwork/startup.qsl-protocol.kv')
js = pathlib.Path('/srv/qbuild/work/NA-0449/.qwork/startup.qsl-protocol.json')
assert kv.exists()
assert js.exists()
data = {}
for line in kv.read_text().splitlines():
    if '=' in line:
        k, v = line.split('=', 1)
        data[k] = v
required = {
    'startup_result': 'OK',
    'lane': 'NA-0449',
    'repo': 'qsl-protocol',
    'path': '/srv/qbuild/work/NA-0449/qsl-protocol',
    'head_equals_origin_main': 'yes',
    'worktree_clean': 'yes',
    'index_clean': 'yes',
    'untracked_clean': 'yes',
    'ready_count': '1',
    'queue_top_ready': 'NA-0449',
    'requested_lane_status': 'READY',
}
for k, v in required.items():
    assert data.get(k) == v, (k, data.get(k), v)
j = json.loads(js.read_text())
for k in ('lane', 'repo', 'path', 'head', 'origin_main', 'ready_count',
          'queue_top_ready', 'requested_lane_status', 'head_equals_origin_main',
          'worktree_clean', 'index_clean', 'untracked_clean'):
    assert str(j.get(k)) == str(data.get(k)), k
print('QWORK_PROOF_OK')
PY
```

Required:

- qwork proof files exist.
- `.kv` markers match the directive.
- JSON parses and mirrors `.kv`.
- Codex does not run qwork, qstart, or qresume.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
import pathlib, re
next_actions = pathlib.Path('NEXT_ACTIONS.md').read_text()
for na in ['NA-0449','NA-0448','NA-0447','NA-0446','NA-0445','NA-0444','NA-0443','NA-0442','NA-0441','NA-0440','NA-0439','NA-0438','NA-0437','NA-0436','NA-0435','NA-0434','NA-0429']:
    m = re.search(rf'^### {na} .*?\nStatus: ([A-Z]+)', next_actions, re.M)
    print(f'{na} {m.group(1) if m else "MISSING"}')
decisions = pathlib.Path('DECISIONS.md').read_text()
ids = re.findall(r'^- \*\*ID:\*\* (D-\d{{4}})$', decisions, re.M)
for did in ['D-0883','D-0884','D-0885','D-0886']:
    print(did, ids.count(did))
print('DUPLICATE_COUNT', len(ids) - len(set(ids)))
PY
```

Required:

- READY_COUNT 1.
- READY NA-0449.
- NA-0448 through NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- latest decision is D-0885 after patching.
- D-0883 once.
- D-0884 once.
- D-0885 once after patching.
- D-0886 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
python3 scripts/ci/qsl_evidence_helper.py scope-guard \
  --base origin/main \
  --allowed qsl/qsl-client/qsc/tests/rng_failure_behavior.rs \
  --allowed qsl/qsl-client/qsc/src/handshake/mod.rs \
  --allowed qsl/qsl-client/qsc/src/protocol_state/mod.rs \
  --allowed qsl/qsl-client/qsc/src/vault/mod.rs \
  --allowed docs/governance/evidence/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_harness.md \
  --allowed tests/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

The combined changed-path set must be exactly the nine allowed paths above.

## Link, leak, classifier, PR-body, and goal checks

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
python3 tools/goal_lint.py
```

Required:

- diff check passes.
- link check reports no missing links.
- added-line leak scan has zero findings.
- added-line overclaim scan, if run separately, has zero affirmative findings.
- PR body preflight passes.
- classifier accepts the implementation-critical scope.
- goal-lint passes with a valid PR event payload.

## Rust, dependency, and formal validation

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- cfg seam test passes and emits all required NA-0449 markers.
- normal test passes and proves selector inert without custom cfg.
- normal qsc tests pass.
- refimpl provider test passes.
- qsc adversarial syntax checks pass.
- root cargo audit passes.
- nested qsc fuzz lock audit passes.
- rustls-webpki remains on a safe version.
- root pqcrypto inverse probes remain expected absence evidence.
- nested qsc fuzz lock pqcrypto scan remains zero-match evidence.
- formatting passes.
- formal checks pass.

If local qsc adversarial smoke stops because `cargo fuzz` is unavailable:

- record exact output;
- do not mutate dependencies or install tooling;
- require PR CI `qsc-adversarial-smoke` for cargo-fuzz-backed evidence before
  merge if attached/required.

## Public claim boundary

No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No crypto-complete claim is made.
No RNG-failure-complete claim is made.
No side-channel-free claim is made.
No vulnerability-free claim is made.
No bug-free claim is made.
No perfect-crypto claim is made.
No public technical paper content is created.

Cargo audit green remains dependency-health evidence only.

## Successor boundary

Selected closeout successor:

`NA-0450 -- QSL qsc RNG Failure Residual Surface Triage Authorization Plan`

NA-0450 must remain authorization/triage scope unless a later exact directive
authorizes implementation paths. NA-0450 must preserve no runtime, crypto,
dependency, Cargo, lockfile, workflow, public, service, backup, restore,
qsl-backup, qwork/qstart/qresume/qshell, status, plan, rollback, or backup
tree mutation.

## PR and post-merge checks

Before merge:

- local validation passes.
- PR required checks pass.
- public-safety is green.
- qsc-adversarial-smoke is green or accepted by required check policy.

After merge:

- verify READY remains NA-0449 until optional closeout.
- verify D-0885 on main.
- verify public-safety is green on the merge commit.
- do not run qwork post-merge.
