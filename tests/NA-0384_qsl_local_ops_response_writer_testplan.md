Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0384 QSL Local Ops Response Writer Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the standalone qsl-protocol Codex response writer harness for
temp-output local operations without authorizing real response archive writes.

## Protected Invariants

- READY_COUNT remains `1`.
- READY remains `NA-0384` until optional closeout.
- NA-0383 remains DONE.
- D-0748 exists once.
- D-0749 exists once.
- D-0750 exists once after implementation.
- D-0751 remains absent until optional closeout.
- The helper writes no real response archive files.
- No response, directive, journal, or history indexes are created.
- No workflow, dependency, runtime, service, protocol, crypto, qshield runtime,
  qsl-server, qsl-attachments, qsc-desktop, website, docs/public, README,
  START_HERE, backup script, timer, fstab, or local backup configuration
  mutation occurs.

## Allowed Scope

- `scripts/ci/qsl_codex_response_writer.py`
- `inputs/local_ops/response_writer_fixtures/`
- `docs/governance/evidence/NA-0384_qsl_local_ops_response_writer_harness.md`
- `tests/NA-0384_qsl_local_ops_response_writer_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

Forbidden scope includes `.github/**`, workflows, Cargo/dependency files,
runtime/service/protocol/crypto/auth/state-machine files, qshield runtime,
qsl-server, qsl-attachments, qsc-desktop, website, docs/public, README,
START_HERE, backup scripts/timers/fstab/source lists/system services,
`scripts/ci/qsl_evidence_helper.py`, `scripts/ci/qsl_bounded_check_poll.py`,
`scripts/ci/qsl_directive_manifest_validate.py`,
`scripts/ci/public_safety_gate.py`, `/srv/qbuild/tools/**`, and
`/home/victor/work/qsl/codex/**` except the required final D203 response file.

## Helper CLI Requirements

Required commands:

```bash
python3 scripts/ci/qsl_codex_response_writer.py --help
python3 scripts/ci/qsl_codex_response_writer.py write --metadata PATH --body PATH --out-dir /srv/qbuild/tmp/... --json
python3 scripts/ci/qsl_codex_response_writer.py dry-run --metadata PATH --body PATH --out-dir /srv/qbuild/tmp/... --json
python3 scripts/ci/qsl_codex_response_writer.py validate --metadata PATH --body PATH --json
python3 scripts/ci/qsl_codex_response_writer.py template --target NA0384 --directive-suffix D203 --directive-id QSL-DIR-2026-05-29-203 --json
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_fixtures --tmp-dir /srv/qbuild/tmp/NA0384_response_writer_<timestamp>
```

## Metadata Schema Requirements

The helper must require schema version
`qsl.codex_response_writer.metadata.v1` and validate target NA, directive
suffix, directive ID, response/directive timestamps, timezone,
timezone_offset, output mode, required sections, real-archive flag, and
no-secret flag.

## Fixture Requirements

Fixtures must cover positive minimal/full/stop-reason/false-positive-safe
bodies and negative malformed metadata, missing target, invalid target, missing
suffix, invalid suffix, invalid timestamp, invalid timezone, missing required
section, missing wrapper-relevant timestamp, unauthorized out-dir, real archive
request, collision/no-overwrite, secret sentinel, dry-run, validate-only, and
JSON summary cases.

## Positive Validation Requirements

- valid minimal response writes under `/srv/qbuild/tmp`;
- valid full response writes under `/srv/qbuild/tmp`;
- wrapper is correct;
- filename is deterministic;
- required sections are enforced;
- stop reason section is accepted;
- `_r2` collision works;
- `_r3` collision works;
- JSON summary is valid;
- false-positive-safe text is accepted.

## Negative / Fail-Closed Requirements

Reject malformed or invalid metadata, missing sections, unauthorized output
directories, real archive output, overwrite without collision handling, and
high-confidence secret sentinel content before write.

## Temp-Output Requirements

All write-mode fixture and smoke outputs must remain under `/srv/qbuild/tmp`.
Proof logs must also remain under `/srv/qbuild/tmp`.

## Real Archive Rejection Requirements

The helper must reject `/home/victor/work/qsl/codex/responses` as an output
directory in NA-0384 and must not write any `NA0384*_D203.md` file there.

## No-Network / No-Mutation Requirements

The helper must use the Python standard library only, contain no network calls,
contain no GitHub calls, contain no branch mutation commands, perform no
deletion, and write no existing file.

## No-Secret Requirements

The helper must scan metadata and body content before write for high-confidence
private key, token, credential, recovery-envelope, raw-credential, and
`QSL_TEST_FORBIDDEN_SECRET_SENTINEL` patterns. It must fail closed without
printing matched secret content.

## Backup-Impact Requirements

The testplan expects no backup-plan update for NA-0384 because durable outputs
are tracked qsl-protocol files and temporary proof stays under `/srv/qbuild/tmp`.
Real archive writes remain future-gated.

## Public-Claim Boundary Requirements

Evidence must preserve no metadata-free, anonymity, untraceable,
production-readiness, public-internet-readiness, external-review-complete,
disaster-recovery-complete, off-host-backup-complete, qsl-server production,
qsl-attachments production, or website-readiness claim.

## Successor Selection Requirements

Expected successor:

`NA-0385 -- QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization Plan`

NA-0385 must not be implemented by NA-0384.

## Required Local Checks

Run and record:

```bash
python3 scripts/ci/qsl_codex_response_writer.py --help
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_fixtures --tmp-dir /srv/qbuild/tmp/NA0384_response_writer_<timestamp>
python3 -m py_compile scripts/ci/qsl_codex_response_writer.py
python3 scripts/ci/qsl_codex_response_writer.py template --target NA0384 --directive-suffix D203 --directive-id QSL-DIR-2026-05-29-203 --json
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

Required checks must attach and complete green before merge. public-safety must
remain required and green before merge and after merge. No admin bypass, direct
push, squash, rebase, force-push, amend, or branch deletion is authorized.

## Successor Handoff

If optional closeout runs, mark NA-0384 DONE and restore:

`NA-0385 -- QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization Plan`

Closeout must not implement NA-0385.
