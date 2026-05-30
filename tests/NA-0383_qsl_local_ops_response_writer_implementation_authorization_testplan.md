Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0383 QSL Local Ops Response Writer Implementation Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0383 produces authorization evidence for a future response
writer implementation harness without implementing the writer in NA-0383.

## Protected Invariants

- READY_COUNT remains `1`.
- READY remains `NA-0383` until optional closeout.
- NA-0382 remains DONE.
- D-0746 exists once.
- D-0747 exists once.
- D-0748 exists once after authorization.
- D-0749 remains absent until optional closeout.
- public-safety remains required and green.
- NA-0383 does not add response writer code, fixtures, generated responses,
  response indexes, history indexes, workflow changes, dependency changes,
  runtime changes, backup configuration changes, or public-claim changes.

## Allowed Scope

- `docs/governance/evidence/NA-0383_qsl_local_ops_response_writer_implementation_authorization.md`
- `tests/NA-0383_qsl_local_ops_response_writer_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `scripts/ci/qsl_evidence_helper.py`
- `scripts/ci/qsl_bounded_check_poll.py`
- `scripts/ci/qsl_directive_manifest_validate.py`
- `scripts/ci/public_safety_gate.py`
- helper implementation paths
- response writer implementation paths
- response writer fixture paths
- generated response, archive, directive, journal, or history index paths
- workflows
- Cargo/dependency files
- runtime/service/protocol/crypto/auth/state-machine paths
- qshield runtime
- qsl-server and qsl-attachments
- qsc-desktop
- website, docs/public, README, START_HERE
- backup scripts/timers/fstab/source lists/system services
- `/srv/qbuild/tools/**`
- `/home/victor/work/qsl/codex/**` except the required final D202 response file

## NA-0382 Inheritance Requirements

Confirm:

- qsl-protocol PR #1027 merged as `6efe0b6f8db5`.
- qsl-protocol PR #1028 merged as `859caa6a3e9`.
- `scripts/ci/qsl_directive_manifest_validate.py` exists and parses `--help`.
- `inputs/local_ops/directive_manifest_fixtures/` exists.
- `inputs/local_ops/scope_allow_file_fixtures/` exists.
- D-0746 and D-0747 exist once.
- D-0748 is absent at start.

## Response Archive Review Requirements

Evidence must classify:

- response archive existence;
- response archive backup coverage;
- request/directive/journal/ops history availability;
- observed response filename and wrapper consistency;
- observed collision state;
- whether future testing can use `/srv/qbuild/tmp`;
- whether real archive writes require future backup-plan review.

## Output Contract Requirements

Evidence must define:

- output directory argument;
- target NA argument;
- directive suffix argument;
- directive ID argument;
- response start timestamp argument or auto-capture mode;
- America/Chicago default and UTC timestamp mode;
- directive begin/end timestamp handling;
- standard wrapper;
- required section headings;
- collision-safe filename;
- dry-run mode;
- validate-only mode;
- no-secret scan mode;
- JSON summary mode;
- final path print;
- no deletion or modification of existing response files;
- no implicit indexing.

## No-Secret Policy Requirements

Evidence must require future default fail-closed behavior for high-confidence
private key, token, credential, passphrase, recovery-envelope, and secret marker
patterns. The plan must prohibit quoting secret content and must reject silent
redaction unless a future directive explicitly authorizes a redaction mode.

## Filename / Collision Requirements

Evidence must define:

- `NAxxxx_<YYYYMMDD>T<HHMMSS><timezone-offset>_Dnnn.md`;
- `_r2`, `_r3`, and monotonic suffixing on collision;
- extension preservation;
- no overwrite;
- no deletion;
- target NA normalization;
- directive suffix validation;
- timestamp validation.

## Lifecycle / Storage / Backup-Impact Requirements

Evidence must classify:

- qsl-protocol helper path;
- qsl-protocol fixture path;
- temp output under `/srv/qbuild/tmp`;
- real archive output under `/home/victor/work/qsl/codex/responses`;
- response index as separate future lane;
- same-host continuity versus disaster recovery;
- no NA-0383 backup-plan update;
- future real archive write backup-review gate.

## Integration Requirements

Evidence must explain integration boundaries for:

- `qsl_directive_manifest_validate.py`;
- `qsl_bounded_check_poll.py`;
- `tools/goal_lint.py`;
- public-safety;
- queue state;
- GitHub mutation boundaries.

## Fixture / Negative-Case Requirements

Future NA-0384 fixture plan must include valid minimal body, valid full body,
missing required section, invalid target, invalid suffix, invalid timestamp,
`_r2` collision, `_r3` collision chain, missing output directory creation,
unauthorized output directory rejection, no overwrite, secret-shaped rejection,
false-positive-safe acceptance, dry-run no-write, validate-only no-write, JSON
summary, wrapper correctness, and stop-reason section presence.

## Risk Matrix Requirements

Evidence must compare:

1. standalone qsl-protocol response writer;
2. extension to `qsl_directive_manifest_validate.py`;
3. extension to `qsl_evidence_helper.py`;
4. shell script;
5. local `/srv/qbuild/tools` response writer;
6. no writer / continue manual.

Each option must classify value, risk, backup impact, CI impact, security
impact, testability, dependency impact, workflow impact, local archive mutation
implications, implementation authority, and recommended status.

## Authorization Decision Requirements

Evidence must explicitly authorize or block the future first lane. Expected
authorization:

`RESPONSE_WRITER_IMPLEMENTATION_AUTHORIZATION_READY`

## Path Bundle Requirements

Evidence must list exact future NA-0384 allowed paths, `/srv/qbuild/tmp`
temporary output boundary, and forbidden paths unless separately authorized.

## Audit Carry-Forward Requirements

Evidence must carry forward NA-0380 audit findings affecting response archive
hygiene, local history backup coverage, validation cost, service-local proof,
public technical paper timing, D132 cleanup, and routine audit cadence.

## Routine Audit Cadence Recommendation Requirements

Evidence may recommend a future routine audit cadence policy lane but must not
select it ahead of NA-0384 unless response writer implementation is blocked.

## Fail-Closed Requirements

Evidence must require no overwrite, no deletion, no archive mutation, no secret
write, collision safety, deterministic timestamps, explicit output path, no
index mutation, no backup config mutation, no queue mutation, no GitHub
mutation, human and JSON summaries, and strict tests.

## Public-Claim Boundary Requirements

Evidence must state that NA-0383 and future response writer harness work do not
prove production readiness, public-internet readiness, external review
completion, metadata-free behavior, anonymity, untraceable behavior, off-host
backup completion, disaster recovery completion, qsl-server/qsl-attachments
production proof, or website/public-copy readiness.

## Successor Selection Requirements

Expected successor:

`NA-0384 -- QSL Local Ops Response Writer Implementation Harness`

NA-0384 must not be implemented by NA-0383.

## Required Local Checks

Run and record:

```bash
python3 scripts/ci/qsl_bounded_check_poll.py --help
python3 scripts/ci/qsl_bounded_check_poll.py fixture --fixture inputs/local_ops/qsl_bounded_check_poll_fixtures/pr_required_success.json --policy required
python3 scripts/ci/qsl_directive_manifest_validate.py --help
python3 scripts/ci/qsl_directive_manifest_validate.py fixture --fixture-dir inputs/local_ops/directive_manifest_fixtures --allow-fixture-dir inputs/local_ops/scope_allow_file_fixtures
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

If optional closeout runs, mark NA-0383 DONE and restore:

`NA-0384 -- QSL Local Ops Response Writer Implementation Harness`

The closeout must not implement NA-0384.
