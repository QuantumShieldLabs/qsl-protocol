Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0549 Remote/Relay Non-Required CI Failure Targeted Follow-Up Evidence Harness

## Executive Summary

NA-0549 consumes D-1086 and D-1087, verifies fresh qwork proof from
`2026-06-27T20:45:18Z`, and captures targeted read-only GitHub evidence for the
six already-created NA-0547 remote/relay runs. The lane executed no rerun,
workflow dispatch, local reproduction, workflow mutation, runtime mutation, qsc
source/test/fuzz/Cargo mutation, dependency/lockfile mutation,
qsl-server/qsl-attachments command or mutation, public-site mutation, or
Cloudflare mutation.

Result classification:

`REMOTE_RELAY_TARGETED_EVIDENCE_QSL_SERVER_BOUNDARY_READY`

Selected successor:

`NA-0550 -- QSL Relay UI qsl-server Boundary Authorization Plan`

## qwork Proof Verification

The qwork proof files were copied from `/srv/qbuild/work/NA-0549/.qwork/` into
the NA-0549 proof root and parsed from file-backed `.kv`, JSON, and
cargo-target env files.

Verified fields included lane `NA-0549`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0549/qsl-protocol`, clean worktree/index/untracked state,
HEAD/origin/main `e2e4b159ebfd`, READY_COUNT 1, READY NA-0549, shared Cargo
target mode, toolchain key `rustc-1.95.0-x86_64-unknown-linux-gnu`,
`explicit_target_preserved=no`, and `shared_target_ready=yes`.

Codex did not run qwork, qstart, or qresume.

## D-1086 / D-1087 Inheritance

D-1086 and D-1087 were consumed and verified Accepted once.

D-1086 selected
`REMOTE_RELAY_FOLLOW_UP_EVIDENCE_TARGETED_GITHUB_CAPTURE_READY` and authorized
read-only GitHub metadata/log/artifact capture for already-created NA-0547 runs
only. D-1086 did not authorize rerun, workflow dispatch, local reproduction, or
implementation mutation.

D-1087 accepted NA-0548 closeout and restored NA-0549 as the sole READY
successor. No NA-0549 implementation occurred before this directive.

Inherited NA-0547 current-main reproduction results:

- remote-handshake: `REMOTE_HANDSHAKE_REPRODUCED_CURRENT`
- remote-relay: `REMOTE_RELAY_REPRODUCED_CURRENT`
- relay-ui-integration: `RELAY_UI_REPRODUCED_CURRENT`

## Current Main Required-Check Classification

Fresh GitHub REST data was captured for current main `e2e4b159ebfd`.

Classification result:

- public-safety: `REQUIRED_GREEN`
- advisories: `NON_REQUIRED_SUCCESS`
- goal-lint: `REQUIRED_GREEN` through PR #1370 head proof
- CodeQL: `REQUIRED_GREEN` through PR #1370 aggregate proof plus current-main
  Analyze checks
- qsc-adversarial-smoke: `NON_REQUIRED_SUCCESS`
- remote-handshake: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- remote-relay: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- relay-ui-integration: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`

All branch-protection required contexts were classified as green or
conclusively satisfied. No failed required check was observed. Root Cargo.lock,
nested qsc fuzz lockfile, and Cargo.toml files had no drift from origin/main.

## Target Run Set

The exact D-1086 target run set was verified:

| Target | Historical run | Current-main run |
|---|---:|---:|
| remote-handshake | `28222737830` | `28298341119` |
| remote-relay | `28221877145` | `28298371731` |
| relay-ui-integration | `28221488004` | `28298405239` |

Each run belongs to `QuantumShieldLabs/qsl-protocol`, matches the expected
workflow, and is one of the authorized run IDs.

## Run / Job / Check Metadata Capture

For each target run, NA-0549 captured proof-root-only run JSON, job JSON,
check-run metadata, and `gh run view --json` metadata. Failed job and failed
step names were resolved from the captured metadata:

- remote-handshake failed in `Run remote handshake smoke (happy-path seed=1)`.
- remote-relay failed in `Run remote relay smoke (manual/nightly)`.
- relay-ui-integration failed in `Start local relay and run ignored relay UI
  integration tests`.

## Artifact Metadata and Capture Review

Artifact metadata was captured for all six runs. Artifacts were downloaded
proof-root-only because every artifact was small, target-named, and
diagnostically relevant.

Raw artifacts remain proof-root-only. Repository docs include only redacted
summary statements. Artifact scanning found private endpoint material in raw
artifact content and handled it by proof-root retention plus redacted extracts;
no private endpoint material is copied into repository docs.

## Log Capture Inventory

`gh run view --log` output was captured proof-root-only for all six authorized
run IDs. Raw logs remain under the NA-0549 proof root only and are not committed
to repository docs.

Bounded redacted extracts and per-run summaries were created under the proof
root for all six runs.

## Redaction and Private-Material Review

Private-material scans covered raw logs, redacted extracts, artifact metadata,
downloaded artifacts, and candidate summaries.

Scan result:

- raw logs: `PASS_HANDLED`
- redacted extracts: `PASS`
- artifacts: `PASS_HANDLED`
- candidate summaries: `PASS`

Redacted extracts contain no unredacted token, private endpoint, authorization
header value, private-key material, or long unclassified token-like material
needed for repository summaries.

## remote-handshake Failure Correlation

Historical and current-main signatures match.

- Historical run: `28222737830`
- Current-main run: `28298341119`
- Failed step: `Run remote handshake smoke (happy-path seed=1)`
- Exit code: 1
- Bounded message: `vault init failed for alice`

The artifact marker narrows the failure to retired passphrase-env use at
`vault_init`, before external relay handshake/send/receive interaction. The
evidence supports script/harness ownership in
`scripts/demo/qsc_remote_handshake_smoke.sh`.

Updated evidence gap:

`REMOTE_HANDSHAKE_GAP_EXACT_IMPLEMENTATION_READY`

## remote-relay Failure Correlation

Historical and current-main signatures match.

- Historical run: `28221877145`
- Current-main run: `28298371731`
- Failed step: `Run remote relay smoke (manual/nightly)`
- Exit code: 1
- Bounded marker: `contacts_store_invalid`

The artifacts show repeated contact-store invalid markers, a failed remote
complete marker, and zero delivery/drop/reorder/dup counts. The failure occurs
before successful relay delivery markers and supports script/harness ownership
in `scripts/demo/qsc_remote_relay_smoke.sh`.

Updated evidence gap:

`REMOTE_RELAY_GAP_EXACT_IMPLEMENTATION_READY`

## relay-ui-integration Failure Correlation

Historical and current-main signatures match.

- Historical run: `28221488004`
- Current-main run: `28298405239`
- Failed step: `Start local relay and run ignored relay UI integration tests`
- Exit code: 22
- Bounded message: loopback health request HTTP 404 before ignored tests ran

The failure occurs after TCP bind proof for the qsl-server process and before
`cargo test -p qsc --test relay_ui_integration` starts. The health probe uses a
v1-path pattern; exact literal path evidence remains proof-root-only. The
evidence narrows the failure to a qsl-server route/path or workflow wiring
boundary, but qsl-server source/local use was not authorized in NA-0549.

Updated evidence gap:

`RELAY_UI_GAP_QSL_SERVER_BOUNDARY_REQUIRED`

## Per-Target Evidence-Gap Update

| Target | Updated classification |
|---|---|
| remote-handshake | `REMOTE_HANDSHAKE_GAP_EXACT_IMPLEMENTATION_READY` |
| remote-relay | `REMOTE_RELAY_GAP_EXACT_IMPLEMENTATION_READY` |
| relay-ui-integration | `RELAY_UI_GAP_QSL_SERVER_BOUNDARY_REQUIRED` |

## Implementation Readiness Review

| Target | Readiness classification | Exact future path status |
|---|---|---|
| remote-handshake | `IMPLEMENTATION_READY_EXACT_PATHS` | Future exact path: `scripts/demo/qsc_remote_handshake_smoke.sh` |
| remote-relay | `IMPLEMENTATION_READY_EXACT_PATHS` | Future exact path: `scripts/demo/qsc_remote_relay_smoke.sh` |
| relay-ui-integration | `IMPLEMENTATION_NOT_READY_QSL_SERVER_BOUNDARY_REQUIRED` | No exact remediation path selected until qsl-server boundary authorization |

The remote-handshake remediation is expected to replace retired passphrase-env
vault initialization with file-based passphrase ingress inside the script. The
remote-relay remediation is expected to add exact trusted contact/route setup or
equivalent harness state before send/count assertions. No qsc runtime/source,
workflow, dependency, qsl-server, qsl-attachments, public-site, or Cloudflare
mutation is authorized by NA-0549.

## qsl-server / qsl-attachments Boundary Review

Classification:

`QSL_SERVER_BOUNDARY_SEPARATE_AUTHORIZATION_REQUIRED`

qsl-server appears only through existing GitHub Actions behavior in NA-0549
evidence. qsl-attachments does not appear in the target failure evidence. The
relay UI failure is narrowed to qsl-server route/path or workflow wiring, but
the next lane must authorize the boundary before any local qsl-server clone,
build, run, source mutation, workflow mutation, or qsl-attachments action.

## Result Classification

`REMOTE_RELAY_TARGETED_EVIDENCE_QSL_SERVER_BOUNDARY_READY`

## Selected Successor

Selected successor:

`NA-0550 -- QSL Relay UI qsl-server Boundary Authorization Plan`

Successor block:

```md
### NA-0550 — QSL Relay UI qsl-server Boundary Authorization Plan
Status: READY
Goals: G1, G2, G3, G4, G5

Objective:
Authorize a bounded lane to define whether and how relay UI reproduction or
remediation may use qsl-server/qsl-attachments context. Identify exact
qsl-server/qsl-attachments paths, commands, logs, private-material risks, proof
boundaries, and successor rules before any local qsl-server or relay UI
reproduction is attempted.

Allowed scope:
- docs/governance/evidence/NA-0550_relay_ui_qsl_server_boundary_authorization_plan.md
- tests/NA-0550_relay_ui_qsl_server_boundary_authorization_testplan.md
- DECISIONS.md
- TRACEABILITY.md
- docs/ops/ROLLING_OPERATIONS_JOURNAL.md
- read-only qsl-protocol workflow/test references
- read-only GitHub qsl-server/qsl-attachments workflow metadata if visible
- exact future boundary design
- successor selection

Forbidden scope:
- qsl-server/qsl-attachments clone/build/run/mutation;
- workflow mutation;
- runtime mutation;
- dependency/lockfile mutation;
- qsc source/test/fuzz/Cargo mutation;
- remote command;
- qwork/qstart/qresume execution by Codex;
- qsl-backup execution;
- backup mutation;
- public-site mutation;
- Cloudflare mutation;
- public-readiness, production-readiness, vulnerability-free, bug-free, or
  perfect-build claim.
```

## Required-Check Boundary

public-safety completed success. advisories completed success. No failed
required check was observed. Required contexts were classified using
current-main check runs plus PR #1370 rollup proof for aggregate/PR-only
contexts.

## Workflow Mutation Boundary

No workflow file was mutated. NA-0549 reviewed workflow metadata and logs
read-only.

## Runtime / qsc / Dependency Boundary

No runtime/source path, qsc source/test/fuzz/Cargo path, Cargo.toml, or
lockfile was mutated. Root Cargo.lock and nested qsc fuzz lockfile remained
unchanged from origin/main.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, local use, source
mutation, or artifact publication occurred. qsl-server evidence remains GitHub
Actions black-box evidence.

## Remote-Action Boundary

No remote command outside read-only GitHub API/log/artifact access was executed.
No SSH, scp, sftp, rsync, qsl-backup, backup mutation, qwork, qstart, or qresume
execution occurred.

## Public-Site / Cloudflare Boundary

No README public-progress, docs/public, public, website, deployment,
Cloudflare, or public-site path was mutated.

## Raw-Log / Artifact Boundary

Raw logs and raw artifacts remain proof-root-only. Repository docs contain only
bounded redacted summaries and classifications. No raw logs or raw artifacts are
committed.

## Claim Boundary

NA-0549 makes no public-readiness claim, production-readiness claim,
public-internet-readiness claim, external-review-complete claim,
reproducibility-complete claim, backup/restore-complete claim,
vulnerability-free claim, bug-free claim, perfect-build claim, or perfect-crypto
claim.

## Validation

Validation for the governance patch includes exact five-path implementation
scope guard, marker proof, queue/decision proof, link-check, private-material
scan, overclaim scan, docs/governance-only classifier, PR body preflight,
goal-lint when available, root cargo audit, nested qsc fuzz lock cargo audit,
`cargo fmt --check`, and qsc-adversarial shell syntax checks.

Focused qsc runtime tests are skipped because NA-0549 uses read-only GitHub
metadata/log/artifact evidence only; no local qsc runtime reproduction was
authorized; and no qsc source/runtime/dependency/workflow mutation occurred.

## Recommendation

Proceed with NA-0550 as the qsl-server boundary authorization lane. Preserve the
remote-handshake and remote-relay exact script-remediation findings for a later
exact remediation lane after the relay UI boundary is authorized or explicitly
deferred.
