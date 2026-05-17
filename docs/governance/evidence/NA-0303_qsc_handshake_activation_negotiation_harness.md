Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0303 qsc Handshake Activation Negotiation Harness

Directive: QSL-DIR-2026-05-17-111 / NA-0303

## Executive summary

NA-0303 adds a test-only qsc integration harness for the existing handshake
activation/admission CLI and relay test seam.

The harness validates:

- valid qsc handshake activation across `handshake init` and `handshake poll`;
- fail-closed reject behavior for unsupported handshake-frame version,
  downgrade-like lower handshake-frame version, malformed input, unauthorized
  identity admission, and duplicate/replayed pending input;
- no accepted qsp session mutation on rejected admission inputs;
- no `recv_commit` or qsp output artifact on rejected admission inputs;
- no panic/backtrace text; and
- no route-token, passphrase-env, or malformed-sentinel leakage.

This is bounded executable harness evidence. It is not a full cryptographic
proof and it does not change protocol, wire, crypto state-machine, handshake,
key schedule, dependency, workflow, service implementation, qsc-desktop,
website, README, START_HERE, docs/public, branch-protection, or public-safety
configuration.

## Live NA-0303 scope

Live `NEXT_ACTIONS.md` authorizes qsc handshake/admission cross-surface
hardening proof where existing public/test APIs permit it, or exact blocker
evidence when no authorized seam exists.

Protected boundaries:

- no unsupported production/public-internet/external-review/anonymity claims;
- no silent protocol or crypto semantic changes;
- executable proof or exact prerequisite stop; and
- no dependency, workflow, website, service implementation, docs/public,
  README, START_HERE, branch-protection, or public-safety configuration drift.

## Selected surface or blocker

Selected surface:

- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`

The test uses existing qsc public CLI commands:

- `handshake init`;
- `handshake poll`;
- identity and contact setup commands; and
- the existing in-process relay test helper.

The selected surface is feasible because qsc already exposes handshake
activation and admission behavior through those commands and because tests can
inject raw relay messages without changing runtime code.

Limitation recorded:

- qsc `QHSM` handshake frames carry a handshake magic, handshake version, type,
  session id, KEM/signature material, MACs, signatures, and DH public keys.
  They do not carry an explicit Suite-2 suite-id negotiation field. The
  directive marker for unsupported suite/version is therefore satisfied by the
  unsupported frame-version half of the requirement, while the absent
  suite-id handshake seam remains visible as the next recommended NA-0304
  target.

## Changed files

- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md`
- `tests/NA-0303_qsc_handshake_activation_negotiation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Harness design

The harness creates isolated qsc config directories, initializes passphrase
test vaults, rotates local identities, pins authenticated peer fingerprints,
sets relay inbox/peer route tokens, and drives the handshake through the qsc
binary.

For negative cases, it uses the existing relay test server to inject or replace
queued handshake admission frames before `handshake poll` consumes them. The
reject assertions check command output, qsp session files, relay output queues,
and the encrypted vault blob where replay rejection happens while responder
pending state exists.

## Valid qsc activation/admission proof

The valid control path:

1. Alice runs `handshake init` to emit A1.
2. Bob runs `handshake poll` and emits B1 without committing an accepted qsp
   session.
3. Alice runs `handshake poll`, commits the initiator qsp session, and emits A2.
4. Bob runs `handshake poll` and commits the responder qsp session.

The harness asserts both qsp session blobs exist only at the expected stages and
that both initiator and responder `handshake_complete` markers appear.

Marker:

- `NA0303_QSC_HANDSHAKE_CONTROL_OK`

## Unsupported suite/version admission reject proof

The qsc handshake frame has no suite-id field. The harness mutates the A1
handshake-frame version to `0xffff`, injects it into Bob's relay inbox, and
asserts deterministic `handshake_reject`, no session file, no outbound B1, no
`recv_commit`, no qsp output marker, and no leak/panic output.

Marker:

- `NA0303_UNSUPPORTED_SUITE_ADMISSION_REJECT_OK`

## Downgrade admission reject proof

The harness mutates the A1 handshake-frame version to the lower
downgrade-like value `0x0000`, injects it into Bob's relay inbox, and asserts
the same fail-closed no-mutation/no-output properties.

Marker:

- `NA0303_DOWNGRADE_ADMISSION_REJECT_OK`

## Malformed admission reject proof

The harness injects a malformed sentinel byte string instead of a `QHSM`
handshake frame. Bob's `handshake poll` rejects the input, creates no accepted
session, emits no outbound B1, emits no qsp output, and does not echo the
sentinel.

Marker:

- `NA0303_MALFORMED_ADMISSION_REJECT_OK`

## Inactive/unauthorized admission reject proof

The harness initializes qsc vaults and route settings without authenticated
identity pins, then attempts `handshake init`. qsc rejects with
`identity_unknown`, queues no A1, creates no qsp session, and emits no
commit/output markers.

Marker:

- `NA0303_INACTIVE_ADMISSION_REJECT_OK`

## Replay/duplicate admission proof

The harness lets Bob accept one valid A1 only far enough to store responder
pending state and emit one B1. It then replays the same A1 while Bob is waiting
for A2. The replay is rejected as an invalid pending-stage admission input.
The encrypted vault blob is byte-identical before and after the replay reject,
no qsp session file appears, and no second B1 is emitted.

Marker:

- `NA0303_REPLAY_ADMISSION_REJECT_OK`

## No-mutation proof

Rejected unsupported-version, downgrade-like, malformed, and unauthorized
inputs create no qsp session blob and emit no outbound handshake response. The
duplicate/replayed pending input preserves Bob's encrypted vault blob
byte-for-byte and creates no qsp session.

Marker:

- `NA0303_NO_MUTATION_ON_REJECT_OK`

## No recv_commit/output proof

Every rejected admission output is scanned to ensure it does not contain
`event=recv_commit`, `event=qsp_unpack ok=true`, or `event=handshake_complete`.

Marker:

- `NA0303_NO_RECV_COMMIT_ON_REJECT_OK`

## No panic/backtrace proof

Every valid and rejected command output is scanned for panic/backtrace markers.

Marker:

- `NA0303_NO_PANIC_OK`

## No secret/plaintext/sentinel leak proof

The harness scans output for both route tokens, the passphrase environment
variable name, and the malformed sentinel string.

Marker:

- `NA0303_NO_SECRET_LEAK_OK`

## Commands run

Startup/preflight:

```bash
date --iso-8601=seconds
date -u --iso-8601=seconds
df -BG /srv/qbuild
git status --porcelain=v1 --branch
git fetch --all --prune
git rev-parse origin/main
gh pr view 864 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 863 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 862 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 861 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 860 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 859 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 858 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 857 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 856 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 855 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 854 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 853 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 852 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 851 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 850 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 849 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 848 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 847 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 846 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 845 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 844 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 843 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 842 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 841 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 840 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 839 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 838 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 837 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 836 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 835 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 834 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 833 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 832 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 831 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 830 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 829 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 828 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 827 --json number,state,mergedAt,mergeCommit,headRefOid,headRefName,baseRefName,title,url
gh pr view 750 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
bash scripts/ci/classify_ci_scope.sh START_HERE.md
bash scripts/ci/classify_ci_scope.sh README.md START_HERE.md docs/public/INDEX.md docs/public/RELEASE_READINESS_EVIDENCE_MAP.md docs/public/EXTERNAL_REVIEW_PACKAGE.md docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md tests/NA-0294_public_evidence_navigation_refresh_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
bash scripts/ci/classify_ci_scope.sh README.md Cargo.toml
bash scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh .github/workflows/public-ci.yml
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Preflight validation:

```bash
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/metadata_conformance_smoke.sh
scripts/ci/metadata_phase2_identifier_padding_harness.sh
scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
cargo +stable test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
```

Focused NA-0303 harness:

```bash
cargo +stable fmt --check
cargo +stable fmt
cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture
```

## Artifacts

- Demo adversarial stress:
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260517T061804Z`
- Demo repeated soak:
  `/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260517T061812Z`
- Sanitized retention harness:
  `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.xNVJa9`

## Limitations

- The qsc handshake frame does not expose a suite-id negotiation field. This
  lane proves unsupported handshake-frame version and lower-version
  downgrade-like rejects at the qsc handshake/admission surface, while leaving
  a narrow seam recommendation for suite-id negotiation visibility in a future
  test-only lane.
- The harness is local bounded executable proof over deterministic fixtures and
  qsc CLI behavior. It is not external review and it is not broad deployment
  readiness.
- The harness does not change handshake implementation code, so any future
  failure that requires implementation changes must be handled in a separate
  authorized fix lane.

## No protocol/crypto implementation change proof

Changed implementation paths are limited to a qsc integration test file. No
runtime qsc source under `qsl/qsl-client/qsc/src/**`, qsp protocol-core,
crypto state-machine, key schedule, QSP wire-format, Cargo/dependency,
workflow, service, qsc-desktop, website, README, START_HERE, docs/public,
branch-protection, or public-safety configuration file is changed.

## Next recommendation

If this PR merges and post-merge public-safety is green, close out NA-0303 and
restore NA-0304 as a narrow qsc handshake negotiation seam lane. The successor
should either add test-only visibility for suite-id negotiation at handshake
activation/admission, or select the next bounded qsc core assurance proof lane
if the live queue chooses a different successor.
