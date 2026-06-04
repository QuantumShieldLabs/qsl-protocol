Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0412 qwork Startup Proof File Handoff Testplan

## Objective

Validate the qwork startup proof-file handoff implementation and the Packet C
governance evidence that records it.

## Scope

Allowed qsl-protocol paths for Packet C:

- `docs/governance/evidence/NA-0412_qsl_local_ops_qwork_startup_proof_file_handoff.md`
- `tests/NA-0412_qsl_local_ops_qwork_startup_proof_file_handoff_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local tool mutation already completed in Packet B:

- `/srv/qbuild/tools/qwork.sh`

`/srv/qbuild/tools/qshell.sh` must remain unchanged unless a wrapper-alignment
defect is proven. Packet B did not require qshell mutation.

## Required qwork Proof Files

After successful qwork startup, these files must exist:

- `/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.json`
- `/srv/qbuild/logs/NA-0412/startup.qsl-protocol.json`

The `.qwork` directory must be outside
`/srv/qbuild/work/NA-0412/qsl-protocol/`.

## Proof-Field Validation

Validate that the KV and JSON proofs include all required fields:

- startup result, lane, primary repo, repo result, repo name, created/existing
  state, absolute repo path, branch, upstream, HEAD/origin/main/main SHAs,
  clean-tree booleans, qsl-protocol queue fields, existing log JSON proof path,
  lane workspace proof paths, `cd`, qwork checksum/version, and UTC proof
  timestamp.

Pass criteria:

- no required KV fields are missing;
- workspace JSON parses;
- existing log JSON parses;
- workspace JSON mirrors the KV values for required fields;
- proof files are not world-writable.

## Idempotence

Run qwork repeatedly from both `/tmp` and the repo checkout:

```bash
qwork NA-0412 qsl-protocol
```

Pass criteria:

- startup succeeds from both locations;
- `.qwork` contains only `startup.qsl-protocol.kv` and
  `startup.qsl-protocol.json`;
- proof files are replaced, not accumulated as stale files.

## Future-Directive Simulation

Run a simulation that does not invoke qwork:

```bash
proof=/srv/qbuild/work/NA-0412/.qwork/startup.qsl-protocol.kv
awk -F= '$1=="startup_result" {print $2}' "$proof"
cd "$(awk -F= '$1=="cd" {print $2}' "$proof")"
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0412 --select NA-0413
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0810 --select D-0811 --select D-0812
```

Pass criteria:

- `startup_result` is `OK`;
- `cd` points to the qsl-protocol checkout;
- direct queue state matches the proof;
- D-0811 exists once after Packet C;
- D-0812 remains absent;
- the simulation completes without qwork.

## Compatibility / Safety Smokes

Run:

```bash
bash -ic 'set -e; source /srv/qbuild/tools/qshell.sh; qwork BAD/LANE qsl-protocol; echo shell-survived'
qwork NA-0411 qsl-protocol
bash -lc 'source /srv/qbuild/tools/qshell.sh; cd /tmp; qstart NA-0412 qsl-protocol; pwd'
bash -lc 'source /srv/qbuild/tools/qshell.sh; cd /tmp; qresume NA-0412 qsl-protocol; pwd'
```

Pass criteria:

- qshell invalid-lane smoke prints `shell-survived`;
- wrong-lane qwork returns nonzero with `reason=queue-lane-mismatch`;
- qstart and qresume succeed and land in the NA-0412 qsl-protocol checkout.

## Clean-State / Backup Boundary

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only || true
git ls-files --others --exclude-standard || true
sha256sum /usr/local/sbin/qsl-backup
```

Pass criteria:

- qsl-protocol remains tracked-clean and untracked-clean;
- qsl-backup checksum remains unchanged;
- no backup or restore operation is run;
- no backup source-list, backup status, or backup plan file is mutated.

## Packet C Governance Validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0412 --select NA-0413
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0810 --select D-0811 --select D-0812
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0412_qsl_local_ops_qwork_startup_proof_file_handoff.md --allowed tests/NA-0412_qsl_local_ops_qwork_startup_proof_file_handoff_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run PR-body preflight and goal-lint with a standalone near-top line:

```md
Goals: G4
```

Pass criteria:

- READY_COUNT remains `1`;
- READY remains NA-0412;
- D-0811 exists exactly once;
- D-0812 remains absent;
- scope guard reports only the five Packet C paths;
- dependency, formatting, qsc send_commit, formal, PR-body, and goal-lint
  checks pass;
- required PR checks pass and post-merge public-safety is green.
