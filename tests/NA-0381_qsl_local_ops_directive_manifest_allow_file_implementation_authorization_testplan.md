Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0381 QSL Local Ops Directive Manifest and Allow-File Implementation Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0381 produces authorization evidence for a future directive manifest and scope allow-file implementation harness without implementing that harness in NA-0381.

## Protected Invariants

- READY_COUNT remains `1`.
- READY remains `NA-0381` until closeout.
- NA-0380 remains DONE.
- D-0742 exists once.
- D-0743 exists once.
- D-0744 exists once after NA-0381 authorization.
- D-0745 is absent until closeout.
- Public-safety remains required and green.
- NA-0381 does not add parser code, helper code, schema files, generated manifests, generated allow-files, workflow changes, dependency changes, runtime changes, backup configuration changes, or public-claim changes.

## Allowed Scope

- `docs/governance/evidence/NA-0381_qsl_local_ops_directive_manifest_allow_file_implementation_authorization.md`
- `tests/NA-0381_qsl_local_ops_directive_manifest_allow_file_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `scripts/ci/qsl_evidence_helper.py`
- `scripts/ci/public_safety_gate.py`
- `scripts/ci/qsl_bounded_check_poll.py`
- helper implementation paths
- schema/parser/generated manifest/generated allow-file paths
- workflows
- Cargo/dependency files
- runtime/service/protocol/crypto/auth/state-machine paths
- qshield runtime
- qsl-server and qsl-attachments
- qsc-desktop
- website, docs/public, README, START_HERE
- backup scripts/timers/fstab/source lists/system services
- `/srv/qbuild/tools/**`
- `/home/victor/work/qsl/codex/**`

## NA-0380 Inheritance Requirements

Confirm:

- qsl-protocol PR #1023 merged as `678995bac98e`.
- qsl-protocol PR #1024 merged as `2503a46a2be5`.
- `scripts/ci/qsl_bounded_check_poll.py` exists and parses `--help`.
- `inputs/local_ops/qsl_bounded_check_poll_fixtures/` exists.
- D-0742 and D-0743 exist once.
- D-0744 is absent at start.

## Audit Report Intake Requirements

Confirm both audit reports exist or use the prior response summary if absent. If present, SHA-256 must match:

- overall audit `66dd26c0b35b97113f160e4dd67fdc9992bd3be91c72452359fbef74dcef0913`
- code/crypto audit `70c21179e7a57dd168dff77e2d5bb18ac2ad1c7c285b216da7875ca712d1c099`

Checksum mismatch is a stop condition.

## Manifest Schema Requirements

Evidence must define:

- JSON format and `schema_version`;
- directive identity fields;
- target NA;
- expected origin/main;
- prior response path;
- mutable and read-only repos;
- allowed and forbidden paths;
- local path boundaries;
- required checks and evidence files;
- forbidden operations;
- public-claim boundaries;
- backup-impact expectation;
- operator-input requirement;
- packet plan;
- implementation paths;
- temporary artifact paths;
- closeout successor;
- stop conditions;
- response file expectation;
- history read-only paths;
- strict unknown-key policy;
- no-secret policy;
- validation order and failure behavior.

## Allow-File Schema Requirements

Evidence must define:

- one path per line;
- comments with `#`;
- blank lines ignored;
- exact repo-relative paths by default;
- explicit `glob:` support only when marked;
- explicit `local:` support only when future local scope authorizes;
- default rejection of absolute paths;
- parent traversal rejection;
- broad glob rejection;
- forbidden overlay precedence;
- machine-readable summary output;
- fail-closed scope-guard compatibility.

## Lifecycle / Storage / Backup-Impact Requirements

Evidence must decide:

- NA-0381 creates no manifest/allow-file tooling or generated files.
- Future NA-0382 should use tracked fixtures under `inputs/local_ops/`.
- Operator-supplied live manifests outside repo need explicit future authorization.
- Durable local manifests/history artifacts outside repo require backup-plan review.
- NA-0381 requires no backup-plan update.

## Integration Requirements

Evidence must explain integration with:

- `scripts/ci/qsl_bounded_check_poll.py`;
- `scripts/ci/qsl_evidence_helper.py scope-guard`;
- `tools/goal_lint.py`;
- public-safety branch protection.

Manifest conflicts with `NEXT_ACTIONS.md` must stop. The plan must not weaken public-safety or branch protection.

## Fixture / Negative-Case Requirements

Future NA-0382 fixture plan must include valid, missing-field, wrong-target, wrong-main, forbidden-path, exact-allow, unlisted-path, broad-glob, parent-traversal, local-absolute, missing-required-check, successor-mismatch, public-claim-boundary, unknown-key, malformed-JSON, comments/blank-lines, forbidden-overlay, and no-secret cases.

## Risk Matrix Requirements

Evidence must compare:

1. standalone qsl-protocol validator;
2. `qsl_evidence_helper.py` extension;
3. shell validator;
4. local `/srv/qbuild/tools` validator;
5. no validator.

Each option must classify value, risk, backup impact, CI/security impact, testability, and recommendation.

## Authorization Decision Requirements

Evidence must explicitly authorize or block the future first lane. Expected authorization:

`DIRECTIVE_MANIFEST_ALLOW_FILE_IMPLEMENTATION_AUTHORIZATION_READY`

## Path Bundle Requirements

Evidence must list exact future NA-0382 allowed paths and forbidden future paths. `scripts/ci/qsl_evidence_helper.py` may appear only as a future alternative if a later live directive authorizes it.

## Audit-Finding Carry-Forward Requirements

Evidence must carry forward GOV-001, GOV-002, GOV-003, GOV-004, GOV-006, CC-001, CC-002, CC-003, CC-004, CC-005, and CC-007 without creating multiple READY items.

## Fail-Closed Requirements

Evidence must require:

- live repo state beats manifest;
- manifest mismatch stops;
- one READY item;
- exact path matching by default;
- forbidden overlay wins;
- no broad globs by default;
- no parent traversal;
- no absolute local paths unless scoped;
- no secret material;
- deterministic human and JSON summaries;
- bounded checks.

## Public-Claim Boundary Requirements

Evidence must state that NA-0381 and future manifest/allow-file harness work do not prove production readiness, public-internet readiness, external review completion, metadata-free behavior, anonymity, untraceable behavior, off-host backup completion, disaster recovery completion, qsl-server/qsl-attachments production proof, or website/public-copy readiness.

## Successor Selection Requirements

Expected successor:

`NA-0382 -- QSL Local Ops Directive Manifest and Allow-File Implementation Harness`

NA-0382 must not be implemented by NA-0381.

## Required Local Checks

Run and record:

```bash
python3 scripts/ci/qsl_bounded_check_poll.py --help
python3 -m py_compile scripts/ci/qsl_bounded_check_poll.py
python3 scripts/ci/qsl_bounded_check_poll.py fixture --fixture inputs/local_ops/qsl_bounded_check_poll_fixtures/pr_required_success.json --policy required
python3 - <<'PY'
import json, pathlib
for p in pathlib.Path("inputs/local_ops/qsl_bounded_check_poll_fixtures").glob("*.json"):
    if p.name != "malformed_json.json":
        json.loads(p.read_text())
PY
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed <allowed paths>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI Expectations

The PR body must include `Goals: G1, G2, G3, G4, G5`, impact, no-regression, and tests/vectors. Required checks must attach and pass normally. public-safety must remain required and green before merge and after merge.

## Successor Handoff

After Packet T merge and green post-merge public-safety, an optional closeout may mark NA-0381 DONE and restore NA-0382 READY without implementing NA-0382.
