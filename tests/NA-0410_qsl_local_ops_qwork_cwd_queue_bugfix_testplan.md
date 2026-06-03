Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0410 qwork CWD Queue Bugfix Testplan

## Purpose

Validate the local qwork cwd-independent queue verification bugfix and the
governance evidence for D-0806.

## Required Local Tool Checks

Run from qsl-protocol unless a command sets its own cwd.

```bash
bash -n /srv/qbuild/tools/qwork.sh /srv/qbuild/tools/qshell.sh
cd ~ && qwork NA-0410 qsl-protocol
cd /tmp && qwork NA-0410 qsl-protocol
cd /srv/qbuild/tmp/NA0410_qwork_cwd_queue_bugfix_20260603T151931-0500/tests/unrelated-cwd && qwork NA-0410 qsl-protocol
cd /srv/qbuild/work/NA-0410/qsl-protocol && qwork NA-0410 qsl-protocol
```

Expected result: each READY-lane run reports `startup_result=OK`,
`ready_count=1`, `queue_top_ready=NA-0410`, and
`requested_lane_status=READY`.

## Classification Checks

```bash
cd /tmp && qwork NA-0411 qsl-protocol
env CARGO_HOME=<fake-helper-fail-root> /srv/qbuild/tools/qwork.sh NA-0410 qsl-protocol
env CARGO_HOME=<fake-read-fail-root> /srv/qbuild/tools/qwork.sh NA-0410 qsl-protocol
env CARGO_HOME=<fake-multiple-ready-root> /srv/qbuild/tools/qwork.sh NA-0410 qsl-protocol
```

Expected results:

- Wrong-lane returns nonzero with `reason=queue-lane-mismatch`.
- Helper execution failure returns nonzero with `reason=queue-helper-failed`.
- Queue read/parse failure returns nonzero with `reason=queue-read-failed`.
- Multiple READY fixture returns nonzero with `reason=multiple-ready`.

## Shell Compatibility Checks

```bash
bash -ic 'set -e; source /srv/qbuild/tools/qshell.sh; qwork bad/lane qsl-protocol; echo shell-survived'
/srv/qbuild/tools/qwork.sh bad/lane qsl-protocol
bash -ic 'source /srv/qbuild/tools/qshell.sh; qstart NA-0410 qsl-protocol; pwd; qresume NA-0410 qsl-protocol; pwd'
```

Expected results:

- Interactive set-e shell prints fail-closed proof and `shell-survived`.
- Direct automation failure returns nonzero.
- qstart/qresume both report already-current clean qsl-protocol state.

## Governance Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0410 --select NA-0411
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0805 --select D-0806 --select D-0807
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body-file> --scan-overclaims
```

Expected results:

- READY_COUNT is `1`.
- READY remains `NA-0410 -- QSL Local Ops qwork CWD-Independent Queue Verification Bugfix`.
- NA-0411 exists and is not READY.
- D-0805 and D-0806 each exist once.
- D-0807 is absent.
- Changed qsl-protocol paths are limited to the Packet C allowed evidence paths.
- Link, leak, and PR-body checks pass.

## Dependency, Formatting, Formal, and Client Checks

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
```

Expected result: all feasible commands pass, and `rustls-webpki` remains
`v0.103.13` or newer safe version.

## Boundary Checks

```bash
git status --porcelain=v1 --branch
sha256sum /usr/local/sbin/qsl-backup
rg -n --fixed-strings '/home/victor/work/qsl/codex/ops' /usr/local/sbin/qsl-backup | wc -l
rg -n "reset --hard|git reset|git stash|git clean|force-push|push --force|checkout -f|rm -rf|git push" \
  /srv/qbuild/tools/qwork /srv/qbuild/tools/qwork.sh /srv/qbuild/tools/qshell.sh || true
```

Expected results:

- qsl-protocol worktree is clean before governance edits and after local qwork
  implementation.
- qsl-backup checksum remains unchanged from required preflight state.
- Codex ops source inclusion count remains `1`.
- Forbidden command scan reports zero matches.
- No backup or restore command is run.
- No qsl-server or qsl-attachments mutation occurs.
- No NA-0411 implementation occurs.
