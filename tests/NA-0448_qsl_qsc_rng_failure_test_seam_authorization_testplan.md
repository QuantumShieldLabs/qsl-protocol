# NA-0448 QSL qsc RNG Failure Test Seam Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0448 consumes F-0441-03 and NA-0447, classifies qsc
RNG-dependent surfaces, selects an exact future qsc RNG failure test-seam
successor, and preserves no implementation mutation or public claim expansion.

## Protected invariants

- READY_COUNT remains 1.
- NA-0448 remains READY until optional closeout.
- NA-0447 through NA-0435 remain DONE.
- NA-0434 remains BLOCKED.
- NA-0429 remains BLOCKED.
- D-0881 exists once.
- D-0882 exists once.
- D-0883 exists once after the evidence patch.
- D-0884 remains absent until optional closeout.
- Duplicate decision count remains zero.
- F-0441-03 and NA-0447 are consumed as scope/evidence inputs, not as
  completed RNG failure testing.
- Selected classification is `QSC_RNG_TEST_SEAM_IMPLEMENTATION_READY`.
- Selected successor is
  `NA-0449 -- QSL qsc RNG Failure Test Seam Implementation Harness`.
- Future exact implementation paths are limited to:
  - `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
  - `qsl/qsl-client/qsc/src/handshake/mod.rs`
  - `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
  - `qsl/qsl-client/qsc/src/vault/mod.rs`
- No runtime, crypto, dependency, Cargo manifest, lockfile, workflow,
  executable test source, fuzz target, vector, formal model, service,
  public-surface, qwork, qsl-backup, backup, restore, status, plan, rollback,
  or backup tree path is changed by NA-0448.
- No public claim expansion is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_plan.md`
- `tests/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime code, crypto code, dependency metadata,
Cargo manifests, lockfiles, workflows, executable test source, fuzz target
source, vectors, formal model files, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status files, backup plan
files, rollback subtree paths, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands,
workflow mutation, branch-protection mutation, public technical paper work, and
NA-0449 implementation.

## qwork proof and startup checks

Run:

```bash
python3 - <<'PY'
import json, pathlib
kv = pathlib.Path('/srv/qbuild/work/NA-0448/.qwork/startup.qsl-protocol.kv')
js = pathlib.Path('/srv/qbuild/work/NA-0448/.qwork/startup.qsl-protocol.json')
assert kv.exists()
assert js.exists()
data = {}
for line in kv.read_text().splitlines():
    if '=' in line:
        k, v = line.split('=', 1)
        data[k] = v
required = {
    'startup_result': 'OK',
    'lane': 'NA-0448',
    'repo': 'qsl-protocol',
    'path': '/srv/qbuild/work/NA-0448/qsl-protocol',
    'head_equals_origin_main': 'yes',
    'worktree_clean': 'yes',
    'index_clean': 'yes',
    'untracked_clean': 'yes',
    'ready_count': '1',
    'queue_top_ready': 'NA-0448',
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
for na in ['NA-0448','NA-0447','NA-0446','NA-0445','NA-0444','NA-0443','NA-0442','NA-0441','NA-0440','NA-0439','NA-0438','NA-0437','NA-0436','NA-0435','NA-0434','NA-0429']:
    m = re.search(rf'^### {na} .*?\nStatus: ([A-Z]+)', next_actions, re.M)
    print(f'{na} {m.group(1) if m else "MISSING"}')
decisions = pathlib.Path('DECISIONS.md').read_text()
ids = re.findall(r'^- \*\*ID:\*\* (D-\d{{4}})$', decisions, re.M)
for did in ['D-0881','D-0882','D-0883','D-0884']:
    print(did, ids.count(did))
print('DUPLICATE_COUNT', len(ids) - len(set(ids)))
PY
```

Required:

- READY_COUNT 1.
- READY NA-0448.
- NA-0447 through NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- latest decision is D-0883 after patching.
- D-0881 once.
- D-0882 once.
- D-0883 once after patching.
- D-0884 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
python3 scripts/ci/qsl_evidence_helper.py scope-guard \
  --base origin/main \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/governance/evidence/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_plan.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_testplan.md
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0448_qsl_qsc_rng_failure_test_seam_authorization_testplan.md`

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
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
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
- nested qsc fuzz lock pqcrypto scan remains zero-match evidence.
- formatting passes.
- formal model checks pass.

## qsc adversarial smoke

Run if feasible without environment drift:

```bash
if [ -x scripts/ci/qsc_adversarial.sh ]; then
  scripts/ci/qsc_adversarial.sh
else
  sh scripts/ci/qsc_adversarial.sh
fi
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke` if attached/required. Do not install tools or mutate
dependencies to force this local smoke.

## Future NA-0449 path proof

After patching, confirm the evidence and D-0883 select exactly these future
implementation paths:

- `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`

Required future seam command:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Required future behavior:

- forced qsc handshake session ID RNG failure aborts before pending/session
  mutation.
- forced qsc session-store RNG failure aborts before session blob or
  session-store secret mutation.
- forced qsc vault RNG failure aborts before vault file or secret mutation.
- ordinary qsc tests without the custom cfg remain green.
- no Cargo, lockfile, dependency, workflow, fuzz, vector, formal, public,
  service, backup, restore, qsl-backup, or qwork mutation occurs.

## Public claim boundary check

Scan the patch for prohibited affirmative claims. Required result:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no RNG-failure-complete claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no bug-free claim.
- no perfect-crypto claim.
- no public technical paper content.
- cargo audit green is described only as dependency-health evidence.

## PR and post-merge checks

Before PR:

- READY_COUNT 1.
- READY NA-0448.
- D-0883 exists once.
- D-0884 absent.
- no duplicate decision IDs.
- changed paths are exactly the five allowed NA-0448 governance paths.
- no implementation mutation occurs.

After PR merge:

- verify D-0883 on main.
- verify post-merge public-safety is completed success.
- do not run qwork post-merge.

Optional closeout to NA-0449 may run only after the evidence PR merges and
post-merge public-safety is green.
