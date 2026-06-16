Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0488 Binding Fuzz Corpus / Seed Strategy Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0488 authorization-only lane. The lane must decide binding fuzz
corpus/seed strategy after NA-0487 without mutating implementation, corpus,
vectors, qsc source, qsc fuzz Cargo, qsc-adversarial scripts, workflows,
dependencies, lockfiles, formal models, refimpl, services, public docs, backup
paths, or qsl-backup paths.

## Protected invariants

- qwork proof files are read, not regenerated.
- READY_COUNT is exactly 1 at startup.
- READY item is NA-0488 at startup.
- D-0963 exists once.
- D-0964 exists once.
- D-0965 is absent before the patch and exists once after the patch.
- D-0966 remains absent before closeout.
- duplicate decision count is zero.
- PR #1246 is merged at `d78d163535ff`.
- qsc binding fuzz helper and `qsc_binding_semantics` target exist.
- no binding target checked-in corpus is introduced.
- no vector/input mutation is introduced.
- no qsc source, fuzz target, fuzz Cargo, qsc-adversarial script, workflow,
  dependency, lockfile, formal, refimpl, service, public-doc, backup, or
  qsl-backup mutation is introduced.
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim, no
  downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, and no perfect-crypto claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0488_qsl_binding_fuzz_corpus_seed_strategy_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- implementation mutation
- qsc source mutation
- qsc fuzz target mutation
- qsc fuzz Cargo mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency mutation
- lockfile mutation
- checked-in corpus mutation
- vector/input mutation
- formal mutation
- refimpl mutation
- qsl-server, qsl-attachments, qshield runtime, or qshield-cli mutation
- public docs, README, START_HERE, website, or public technical paper mutation
- qwork, qstart, qresume, or qshell mutation
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree mutation

## Startup validation

Run:

```bash
python3 - <<'PY'
import json, pathlib, subprocess, sys
kv = {}
for line in pathlib.Path('/srv/qbuild/work/NA-0488/.qwork/startup.qsl-protocol.kv').read_text().splitlines():
    if '=' in line:
        k, v = line.split('=', 1)
        kv[k] = v
js = json.loads(pathlib.Path('/srv/qbuild/work/NA-0488/.qwork/startup.qsl-protocol.json').read_text())
required = {
    'startup_result': 'OK',
    'lane': 'NA-0488',
    'repo': 'qsl-protocol',
    'path': '/srv/qbuild/work/NA-0488/qsl-protocol',
    'head_equals_origin_main': 'yes',
    'worktree_clean': 'yes',
    'index_clean': 'yes',
    'untracked_clean': 'yes',
    'ready_count': '1',
    'queue_top_ready': 'NA-0488',
    'requested_lane_status': 'READY',
}
for key, value in required.items():
    assert kv.get(key) == value, (key, kv.get(key), value)
    assert js.get(key) == value, (key, js.get(key), value)
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
origin = subprocess.check_output(['git', 'rev-parse', 'origin/main'], text=True).strip()
assert kv['head'] == head
assert kv['origin_main'] == origin
PY
```

Required: PASS before fetch.

## Queue and decision validation

Run a direct parser over `NEXT_ACTIONS.md` and `DECISIONS.md`.

Required before patch:

- READY_COUNT 1.
- READY NA-0488.
- NA-0487 DONE.
- NA-0486 DONE.
- D-0963 count 1.
- D-0964 count 1.
- D-0965 count 0.
- duplicate decision count 0.

Required after patch:

- READY_COUNT 1.
- READY NA-0488.
- D-0965 count 1.
- D-0966 count 0.
- duplicate decision count 0.

## Fuzz corpus inventory validation

Run:

```bash
rg --files qsl/qsl-client/qsc/fuzz | sort
```

Also inspect:

```bash
qsl/qsl-client/qsc/fuzz/Cargo.toml
scripts/ci/qsc_adversarial.sh
qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs
qsl/qsl-client/qsc/fuzz/corpus/
```

Required:

- existing corpus paths are only for older parser/format fuzz targets.
- `qsc_binding_semantics` exists.
- `qsc_binding_semantics` is invoked by qsc-adversarial.
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics` does not exist.
- no corpus files are added by NA-0488.

## Manifest and secret-material validation

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
```

Required:

- manifest JSON validates.
- manifest remains traceability-only for NA-0489.
- future seed strategy does not read the JSON at runtime.
- no checked-in private keys, signing keys, KEM secret keys, passphrases,
  runtime keys, backup keys, operator data, user data, live service data,
  private endpoints, or production-like identifiers are authorized.

## Scope and documentation validation

Run:

```bash
git diff --check
```

Run exact changed-path guard against the NA-0488 evidence base:

```bash
git diff --name-only origin/main...HEAD
```

Required changed paths:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0488_qsl_binding_fuzz_corpus_seed_strategy_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0488_qsl_binding_fuzz_corpus_seed_strategy_authorization_testplan.md`

Forbidden changed-path count: 0.

Run link-check, leak-scan, added-line overclaim scan, classifier, PR body
preflight, and goal-lint with the repository-supported helpers.

Required:

- link-check PASS.
- leak-scan PASS.
- added-line overclaim scan PASS.
- classifier PASS.
- PR body preflight PASS.
- goal-lint PASS.

## Inherited validation

Run:

```bash
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run inherited qsc provider-RNG, key-lifecycle, and provider-error tests as
needed by the directive.

Required:

- all commands pass unless a directive-approved local cargo-fuzz availability
  caveat is recorded.
- cargo audit output is dependency-health evidence only.

## Public-safety and PR validation

Before merge:

- PR body contains a standalone `Goals: G1, G2, G3, G4, G5` line.
- PR body states authorization-only scope.
- PR body states selected successor.
- PR body states no corpus/vector/input mutation.
- PR body states no qsc source/fuzz/Cargo/script/workflow mutation.
- PR body states no dependency/lockfile mutation.
- PR body states no public overclaim.

After merge:

- public-safety on the merge commit is green.
- qsc-adversarial-smoke is green or accepted according to docs-scope policy.
- queue remains exactly one READY item until optional closeout.
- D-0965 exists on main.

## Post-fix hardening review

Report:

1. Correctness under stress.
2. Minimality.
3. Maintainability.
4. Coverage quality.
5. Cross-lane stability.
