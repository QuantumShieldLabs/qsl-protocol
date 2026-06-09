# NA-0447 QSL RNG Failure Behavior Scope Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0447 consumes F-0441-03, classifies RNG failure behavior
surfaces across qsc/refimpl/qshield-cli/formal/fuzz/vector boundaries, selects
an exact NA-0448 successor, and preserves no implementation mutation or public
claim expansion.

## Protected invariants

- READY_COUNT remains 1.
- NA-0447 remains READY until optional closeout.
- NA-0446 through NA-0435 remain DONE.
- NA-0434 remains BLOCKED.
- NA-0429 remains BLOCKED.
- D-0879 exists once.
- D-0880 exists once.
- D-0881 exists once after the evidence patch.
- D-0882 remains absent until optional closeout.
- Duplicate decision count remains zero.
- F-0441-03 is consumed as an RNG failure behavior evidence gap.
- Selected primary classification is `RNG_FAILURE_SCOPE_QSC_TEST_SEAM_NEXT`.
- Selected successor is
  `NA-0448 -- QSL qsc RNG Failure Test Seam Authorization Plan`.
- No runtime, crypto, dependency, Cargo manifest, lockfile, workflow,
  executable test source, fuzz target, vector, formal model, service,
  public-surface, qwork, qsl-backup, backup, restore, status, plan, rollback,
  or backup tree path is changed.
- No public claim expansion is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0447_qsl_rng_failure_behavior_scope_authorization_plan.md`
- `tests/NA-0447_qsl_rng_failure_behavior_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test source, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands,
workflow mutation, branch-protection mutation, public technical paper work, and
NA-0448 implementation.

## qwork proof and startup checks

Run:

```bash
python3 - <<'PY'
import json, pathlib
kv = pathlib.Path('/srv/qbuild/work/NA-0447/.qwork/startup.qsl-protocol.kv')
js = pathlib.Path('/srv/qbuild/work/NA-0447/.qwork/startup.qsl-protocol.json')
assert kv.exists()
assert js.exists()
data = {}
for line in kv.read_text().splitlines():
    if '=' in line:
        k, v = line.split('=', 1)
        data[k] = v
required = {
    'startup_result': 'OK',
    'lane': 'NA-0447',
    'repo': 'qsl-protocol',
    'path': '/srv/qbuild/work/NA-0447/qsl-protocol',
    'head_equals_origin_main': 'yes',
    'worktree_clean': 'yes',
    'index_clean': 'yes',
    'untracked_clean': 'yes',
    'ready_count': '1',
    'queue_top_ready': 'NA-0447',
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
for na in ['NA-0447','NA-0446','NA-0445','NA-0444','NA-0443','NA-0442','NA-0441','NA-0440','NA-0439','NA-0438','NA-0437','NA-0436','NA-0435','NA-0434','NA-0429']:
    m = re.search(rf'^### {na} .*?\nStatus: ([A-Z]+)', next_actions, re.M)
    print(f'{na} {m.group(1) if m else "MISSING"}')
decisions = pathlib.Path('DECISIONS.md').read_text()
ids = re.findall(r'^- \*\*ID:\*\* (D-\d{4})$', decisions, re.M)
for did in ['D-0879','D-0880','D-0881','D-0882']:
    print(did, ids.count(did))
print('DUPLICATE_COUNT', len(ids) - len(set(ids)))
PY
```

Required:

- READY_COUNT 1.
- READY NA-0447.
- NA-0446 through NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- latest decision is D-0881 after patching.
- D-0879 once.
- D-0880 once.
- D-0881 once after patching.
- D-0882 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0447_qsl_rng_failure_behavior_scope_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0447_qsl_rng_failure_behavior_scope_authorization_testplan.md`

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
- leak scan reports zero secret findings.
- overclaim scan, if run separately, reports zero affirmative public overclaim
  lines.
- PR body preflight passes.
- classifier reports docs/governance scope only.
- goal-lint passes with a valid PR event payload.

## Rust, dependency, and formal validation

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- qsc adversarial shell syntax checks pass.
- qsc key-lifecycle test passes.
- provider-error no-mutation test passes.
- qsc `send_commit` passes.
- refimpl `pqkem768` passes.
- root cargo audit is green.
- nested qsc fuzz lock audit is green.
- `rustls-webpki` is `v0.103.13` or newer safe version.
- root pqcrypto inverse probes remain absent or explicitly explained.
- formatting passes.
- formal model checks pass.

## qsc adversarial smoke

If feasible without environment drift, run:

```bash
scripts/ci/qsc_adversarial.sh
```

If the script is not executable, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

Required:

- local stable phases pass, or exact local cargo-fuzz/tooling unavailability is
  recorded and PR CI `qsc-adversarial-smoke` remains required.
- `handshake_provider_error_no_mutation` marker remains present in the script.

## Public-safety and PR checks

Before merge:

- PR checks must be attached.
- required checks must be green or accepted skipped/neutral by repo policy.
- public-safety must complete success.

After merge:

- public-safety must complete success on the merge commit.
- D-0881 must be present on main.
- queue remains READY NA-0447 until optional closeout.

Use REST polling only. Do not use watch mode.

## Success criteria

- `NA0447_RNG_FAILURE_SCOPE_CONSUMED_OK`
- `NA0447_QSC_RNG_TEST_SEAM_NEXT_OK`
- `NA0447_NO_RUNTIME_CHANGE_OK`
- `NA0447_NO_CRYPTO_CHANGE_OK`
- `NA0447_NO_DEPENDENCY_CHANGE_OK`
- `NA0447_NO_WORKFLOW_CHANGE_OK`
- `NA0447_NO_PUBLIC_OVERCLAIM_OK`
- `NA0447_STEWARD_REVIEW_USED_OK`
- `NA0447_ONE_READY_INVARIANT_OK`

The evidence PR succeeds only if all required checks pass and changed paths are
limited to the five allowed NA-0447 governance paths.
