Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0409 qwork Director-Facing Startup Hardening Testplan

## Purpose

Validate the local qwork availability and qshell tmux/set-e safety hardening
evidence. This testplan proves bare qwork works in a fresh shell, the qshell
wrapper preserves an interactive set-e shell on fail-closed qwork results,
automation fail-closed behavior remains nonzero, qstart/qresume compatibility
is preserved, and qsl-protocol governance evidence records the change without
runtime, dependency, workflow, backup, public, or sibling-repo drift.

## Required Local Tool Checks

Run from the qsl-protocol repo root unless a command has an absolute path.

```bash
bash -n /srv/qbuild/tools/qwork /srv/qbuild/tools/qwork.sh /srv/qbuild/tools/qshell.sh /home/victor/.bashrc
bash -lc 'command -v qwork && qwork NA-0409 qsl-protocol'
bash -ic 'source /srv/qbuild/tools/qshell.sh; qwork NA-0409 qsl-protocol; pwd'
bash -ic 'set -e; source /srv/qbuild/tools/qshell.sh; qwork bad/lane qsl-protocol; echo shell-survived'
bash -lc '/srv/qbuild/tools/qwork.sh bad/lane qsl-protocol; status=$?; printf "status=%s\n" "$status"; test "$status" -ne 0'
bash -ic 'source /srv/qbuild/tools/qshell.sh; qstart NA-0409 qsl-protocol; pwd; qresume NA-0409 qsl-protocol; pwd'
```

Expected results:

- Bash syntax checks exit zero.
- Bare `qwork` resolves through `/home/victor/.local/bin/qwork`.
- Bare qwork reports `startup_result=OK`, `queue_top_ready=NA-0409`, and
  `requested_lane_status=READY`.
- Interactive qshell success ends in `/srv/qbuild/work/NA-0409/qsl-protocol`.
- Interactive set-e failure prints qwork failure proof,
  `qshell_qwork_wrapper=fail-closed-shell-preserved`, and `shell-survived`.
- `/srv/qbuild/tools/qwork.sh bad/lane qsl-protocol` reports nonzero status.
- qstart/qresume both report already-current clean qsl-protocol state.

## Install and Idempotence Checks

```bash
stat -c '%a %U %G %s %y %n' /srv/qbuild/tools/qwork /srv/qbuild/tools/qwork.sh /srv/qbuild/tools/qshell.sh /home/victor/.bashrc
readlink /home/victor/.local/bin/qwork
bash -ic 'source /home/victor/.bashrc; source /home/victor/.bashrc; python3 - <<PY
import os
parts=os.environ.get("PATH","").split(":")
print("local_bin_path_count", parts.count(os.path.expanduser("~/.local/bin")))
PY'
```

Expected results:

- `/srv/qbuild/tools/qwork` exists and is executable.
- `/home/victor/.local/bin/qwork` symlinks to `/srv/qbuild/tools/qwork`.
- repeated `.bashrc` sourcing keeps exactly one `$HOME/.local/bin` PATH entry.
- rollback files exist under the Packet B proof root.

## Forbidden Command Scan

```bash
rg -n "reset --hard|git reset|git stash|git clean|force-push|push --force|checkout -f|rm -rf|git push" \
  /srv/qbuild/tools/qwork /srv/qbuild/tools/qwork.sh /srv/qbuild/tools/qshell.sh || true
```

Expected result:

- zero matches.

## Governance Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0409 --select NA-0410
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0802 --select D-0803 --select D-0804
git diff --name-only origin/main...HEAD
git diff --check
```

Expected results:

- `READY_COUNT 1`.
- `READY NA-0409 QSL Local Ops qwork Director-Facing Startup Availability / tmux Safety Hardening`.
- NA-0410 exists and is not READY.
- D-0802 exists once.
- D-0803 exists once.
- D-0804 is absent.
- Changed paths are limited to the Packet C allowed qsl-protocol evidence paths.
- `git diff --check` exits zero.

## Link, Leak, and Overclaim Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main --paths \
  docs/governance/evidence/NA-0409_qsl_local_ops_qwork_director_facing_startup_hardening.md \
  tests/NA-0409_qsl_local_ops_qwork_director_facing_startup_hardening_testplan.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body-file> --scan-overclaims
```

Expected results:

- `TOTAL_MISSING 0`.
- `SECRET_FINDING_COUNT 0`.
- PR-body preflight reports no missing required fields and no prohibited phrases.

## Dependency, Formal, and Client Checks

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

Expected results:

- `cargo audit --deny warnings` exits zero.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13`
  or a newer safe version.
- Formatting, qsc send_commit, formal model checks, and feasible qshield-cli
  build/test commands pass.

## Boundary Checks

```bash
git status --porcelain=v1 --branch
sha256sum /usr/local/sbin/qsl-backup
```

Expected results:

- qsl-protocol worktree is clean before governance edits and clean after local
  qwork implementation.
- `/usr/local/sbin/qsl-backup` checksum remains unchanged from the NA-0407
  source-list state.
- No backup or restore command is run.
- No qsl-server or qsl-attachments mutation occurs.
- No NA-0410 implementation occurs.

## Acceptance

The implementation evidence passes only if bare qwork availability, interactive
set-e shell preservation, automation nonzero fail-closed behavior, qstart/qresume
compatibility, D-0803 governance evidence, and all boundary checks pass.
