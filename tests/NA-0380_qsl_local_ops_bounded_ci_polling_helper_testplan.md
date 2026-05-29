Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0380 QSL Local Ops Bounded CI Polling Helper Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0380 implements a standalone bounded CI/public-safety polling
helper with deterministic fixture coverage and without workflow, dependency,
runtime, public-safety gate, qsl_evidence_helper, backup, secret, sibling-repo,
or public-claim mutation.

## Protected Invariants

- READY_COUNT remains `1`.
- READY remains `NA-0380` until closeout.
- D-0742 exists once after implementation.
- D-0743 is absent until closeout.
- public-safety remains required and green.
- Red required checks fail closed.
- Timeouts fail closed.
- Public-safety missing/red fails closed.
- PR-head and push/merge SHA contexts are distinct.

## Allowed Scope

- `scripts/ci/qsl_bounded_check_poll.py`
- `inputs/local_ops/qsl_bounded_check_poll_fixtures/*.json`
- `docs/governance/evidence/NA-0380_qsl_local_ops_bounded_ci_polling_helper_harness.md`
- `tests/NA-0380_qsl_local_ops_bounded_ci_polling_helper_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `scripts/ci/public_safety_gate.py`
- `scripts/ci/qsl_evidence_helper.py`
- Cargo files and dependency changes
- runtime, service, protocol, crypto, auth, state-machine, qsc, qsp, qsl,
  qshield runtime, qsc-desktop, apps runtime, and tools/refimpl paths
- qsl-server and qsl-attachments
- website, external website, README, START_HERE, docs/public
- `/srv/qbuild/tools/**`
- backup scripts, timers, fstab, system services, source lists, keys,
  credentials, restore, deploy, rollback, off-host targets, and branch deletion

## Helper CLI Requirements

Run:

```bash
python3 scripts/ci/qsl_bounded_check_poll.py --help
python3 scripts/ci/qsl_bounded_check_poll.py pr --repo OWNER/REPO --pr N --required --interval 10 --max-iters 180
python3 scripts/ci/qsl_bounded_check_poll.py pr --repo OWNER/REPO --pr N --all --interval 10 --max-iters 180
python3 scripts/ci/qsl_bounded_check_poll.py public-safety --repo OWNER/REPO --sha SHA --interval 10 --max-iters 180
python3 scripts/ci/qsl_bounded_check_poll.py sha-summary --repo OWNER/REPO --sha SHA --report-only
python3 scripts/ci/qsl_bounded_check_poll.py fixture --fixture PATH --policy POLICY_NAME
```

Expected: commands parse; live commands use `gh api`; fixture commands use no
network; all polling is bounded.

## Fixture Requirements

Fixtures must cover:

- all required green;
- public-safety in progress then success;
- red required check;
- red public-safety;
- pending timeout;
- docs-only skipped accepted;
- skipped rejected without policy;
- CodeQL neutral accepted with policy;
- CodeQL red rejected;
- missing public-safety rejected;
- push SHA report-only with PR-only context absence tolerated;
- stale failed rerun with latest success;
- transient API 404 then success;
- persistent API 404 failure;
- malformed JSON failure.

## Red-Check Fail-Closed Requirements

Red required check and red public-safety fixtures must exit nonzero and print the
check name, status, conclusion, and URL.

## Timeout Fail-Closed Requirements

Pending timeout fixtures must exit nonzero at the configured bounded iteration
cap and print remaining pending/missing checks.

## API Failure Requirements

Transient API errors must be reported and recover if a later bounded iteration
satisfies the requested condition. Persistent API errors must exit nonzero with
endpoint metadata.

## PR-vs-Push Requirements

`pr` mode gates PR-head checks. `sha-summary --report-only` summarizes push or
merge SHAs and must not turn missing PR-only contexts into merge-gate failures.

## Docs-Only Skip Requirements

Skipped full-suite checks are accepted only under explicit docs-only policy.
The same skipped checks must fail without that policy.

## CodeQL Neutral Requirements

CodeQL neutral is accepted only under explicit CodeQL policy. CodeQL red remains
rejected under that policy.

## No-Watch / No-Mutation Requirements

Scan the helper for forbidden commands and strings:

- no `--watch`;
- no `gh pr merge`;
- no `gh run rerun`;
- no `git push`;
- no branch mutation, branch deletion, force-push, amend, rebase, squash, or
  admin-bypass command.

## No-Secret Requirements

Fixtures and changed lines must contain no tokens, private keys, credential
paths, passphrases, auth headers, route tokens, or secret-bearing URLs.

## Backup-Impact Requirements

Helper and fixtures are tracked in qsl-protocol. Proof logs remain temporary
under `/srv/qbuild/tmp`. No backup-plan update is required unless future durable
outputs are created outside authorized repo paths or `/srv/qbuild/tmp`.

## Public-Claim Boundary Requirements

The implementation must not introduce production-readiness, public-internet,
external-review-complete, metadata-free, anonymity, untraceable, hidden-size,
hidden-timing, hidden-traffic-shape, off-host-backup-complete, disaster-recovery,
target-configured, host-identity-verified, key-custody, or key-recovery claims.

## Successor Selection Requirements

Expected successor after NA-0380:

`NA-0381 -- QSL Local Ops Directive Manifest and Allow-File Implementation Authorization Plan`

NA-0381 must not be implemented by NA-0380.

## Required Local Checks

Run and record:

```bash
python3 scripts/ci/qsl_bounded_check_poll.py --help
python3 -m py_compile scripts/ci/qsl_bounded_check_poll.py
python3 scripts/ci/qsl_bounded_check_poll.py fixture --fixture <case> --policy <policy>
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allow <allowed paths>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI Expectations

The PR must include `Goals: G1, G2, G3, G4, G5`, pass required checks normally,
keep public-safety required and green, and merge only with normal merge commit
and `--match-head-commit`.

## Successor Handoff

After merge and post-merge public-safety green, closeout may mark NA-0380 DONE
and restore NA-0381 as READY without implementing NA-0381.
