Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0268 Closeout and NA-0269 Restoration Test Plan

## Scope

This test plan covers the NA-0268 governance closeout after PR #789 merged the
artifact-safe cross-host/private-network soak evidence.

In scope:

- mark NA-0268 DONE;
- record PR #789 head, merge, proof mode, artifact directory, runtime cleanup,
  and post-merge public-safety evidence;
- add D-0507;
- restore NA-0269 as the sole READY successor;
- update TRACEABILITY;
- preserve docs/governance-only scope.

Out of scope:

- NA-0269 implementation;
- qsl-server implementation changes;
- qsl-attachments implementation changes;
- protocol, wire, crypto, auth, or state-machine changes;
- qsc, qsc-desktop, website, workflow, script, Cargo, branch-protection, or
  public-safety configuration changes.

## Expected Queue State

Before closeout:

```text
READY_COUNT 1
READY NA-0268 Cross-Host / Private-Network Soak Expansion
D-0506 exists once
D-0507 absent
```

After closeout:

```text
READY_COUNT 1
READY NA-0269 qsl-server / qsl-attachments Production-Boundary Hardening Plan
NA-0268 DONE
D-0506 exists once
D-0507 exists once
```

## Required Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Required result:

- scope is limited to `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan;
- no forbidden path is touched;
- no duplicate decision IDs;
- link-check has zero missing links;
- added-line leak-scan has zero findings;
- dependency and send_commit health checks remain green;
- public-safety remains required and green before PR creation and after merge.

## CI Cost-Control Expectation

This closeout is docs/governance/testplan only. Under NA-0262A, the PR and
post-merge main push may skip `qsc-linux-full-suite` and
`macos-qsc-full-serial`, while `public-safety`, goal-lint, and other required
docs/governance contexts must still complete green.
