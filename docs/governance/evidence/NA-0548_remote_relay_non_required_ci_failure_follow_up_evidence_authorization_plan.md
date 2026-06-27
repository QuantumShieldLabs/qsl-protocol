Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0548 Remote/Relay Non-Required CI Failure Follow-Up Evidence Authorization Plan

## Executive Summary

NA-0548 consumes D-1084 and D-1085 and authorizes a bounded follow-up evidence
lane for the three non-required remote/relay failures reproduced by NA-0547.

Result classification:

`REMOTE_RELAY_FOLLOW_UP_EVIDENCE_TARGETED_GITHUB_CAPTURE_READY`

Selected successor:

`NA-0549 -- QSL Remote/Relay Non-Required CI Failure Targeted Follow-Up Evidence Harness`

NA-0548 is authorization-only. It did not execute reruns, workflow dispatches,
local reproduction, qsl-server/qsl-attachments commands, runtime tests, or
fixes. It did not mutate workflow, runtime/source, dependency, qsc,
qsl-server, qsl-attachments, public-site, Cloudflare, backup, or
operator-local paths.

## qwork Proof Verification

Fresh qwork proof files from `2026-06-27T19:36:42Z` were copied from the
NA-0548 lane `.qwork/` directory into the proof root and verified from
file-backed `.kv`, JSON, and cargo-target env parsing.

Verified fields included lane `NA-0548`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0548/qsl-protocol`, clean worktree/index/untracked state,
HEAD/origin/main `17737472b438`, READY_COUNT 1, READY NA-0548, shared Cargo
target mode, toolchain key `rustc-1.95.0-x86_64-unknown-linux-gnu`,
`explicit_target_preserved=no`, and `shared_target_ready=yes`.

Codex did not run qwork, qstart, or qresume.

## D-1084 / D-1085 Inheritance

D-1084 and D-1085 were consumed and verified Accepted once.

D-1084 accepted NA-0547 bounded reproduction/log-capture evidence. NA-0547
reproduced all three target failures on then-current main:

- remote-handshake: `REMOTE_HANDSHAKE_REPRODUCED_CURRENT`
- remote-relay: `REMOTE_RELAY_REPRODUCED_CURRENT`
- relay-ui-integration: `RELAY_UI_REPRODUCED_CURRENT`

D-1084 selected `REMOTE_RELAY_REPRODUCTION_PARTIAL_MORE_EVIDENCE_REQUIRED`.
D-1085 accepted NA-0547 closeout and restored NA-0548 as the sole READY
follow-up authorization lane. No NA-0548 implementation occurred before this
directive. Raw NA-0547 logs were not committed to repository docs and private
material was not published.

## Current Main Required-Check Classification

Fresh GitHub REST data was collected for current main `17737472b438`:

- current main check-runs
- branch-protection required status checks
- combined status
- workflow runs for the current head
- PR #1368 status rollup for PR-only required context proof

Classification result:

- public-safety: `REQUIRED_GREEN`
- advisories: `NON_REQUIRED_SUCCESS`
- goal-lint: `REQUIRED_GREEN` through PR #1368 rollup proof
- CodeQL: `REQUIRED_GREEN` through current CodeQL Analyze check-runs
- qsc-adversarial-smoke: `NON_REQUIRED_SUCCESS`
- remote-handshake: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- remote-relay: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`
- relay-ui-integration: `NON_REQUIRED_ABSENT_ON_CURRENT_MAIN`

The absent target checks are not treated as stale or harmless. NA-0547 already
reproduced them on current main at the time of its reproduction lane, and the
later `17737472b438` merge is governance closeout-only.

## NA-0547 Evidence Review

Repository evidence docs, redacted extracts, job metadata, and response
summaries were sufficient for bounded failure signatures. Raw logs were not
needed for repository summaries in NA-0548.

NA-0547 evidence reviewed:

- historical runs: `28222737830`, `28221877145`, `28221488004`
- current-main dispatch runs: `28298341119`, `28298371731`, `28298405239`
- proof root:
  `/srv/qbuild/tmp/NA0547_remote_relay_reproduction_log_capture_20260627T183232Z`

## remote-handshake Failure Signature

- Historical run/job/attempt: `28222737830` / `83841772195` / `2`
- Current run/job/attempt: `28298341119` / `83842138852` / `1`
- Workflow: `.github/workflows/remote-handshake-tests.yml`
- Failed job: `remote-handshake`
- Failed step: `Run remote handshake smoke (happy-path seed=1)`
- Exit code: `1`
- Bounded failure message: `vault init failed for alice`
- Stability: historical rerun and current-main dispatch failed in the same
  step with the same bounded message and exit code.
- Likely ownership class: ambiguous between qsc vault/runtime state and smoke
  script harness; secret-backed relay environment is present, but failure
  occurs before handshake assertions.

## remote-relay Failure Signature

- Historical run/job/attempt: `28221877145` / `83841839651` / `2`
- Current run/job/attempt: `28298371731` / `83842233657` / `1`
- Workflow: `.github/workflows/remote-relay-tests.yml`
- Failed job: `remote-relay`
- Failed step: `Run remote relay smoke (manual/nightly)`
- Exit code: `1`
- Bounded failure message: qsc marker error `contacts_store_invalid`, followed
  by failed happy-path count expectations.
- Stability: historical rerun and current-main dispatch failed in the same
  step with the same marker/error class and exit code.
- Likely ownership class: ambiguous between qsc contact-store/runtime state and
  smoke script harness; secret-backed relay environment is present.

## relay-ui-integration Failure Signature

- Historical run/job/attempt: `28221488004` / `83841918788` / `2`
- Current run/job/attempt: `28298405239` / `83842330793` / `1`
- Workflow: `.github/workflows/relay-ui-integration.yml`
- Failed job: `relay-ui-integration`
- Failed step: `Start local relay and run ignored relay UI integration tests`
- Exit code: `22`
- Bounded failure message: loopback relay health request returned HTTP 404
  before the ignored relay UI tests ran.
- Stability: historical rerun and current-main dispatch failed in the same
  step with the same HTTP error class and exit code.
- qsl-server appears only as existing GitHub Actions black-box behavior.

## Current Workflow and Referenced Surface Review

remote-handshake:

- Workflow file: `.github/workflows/remote-handshake-tests.yml`
- Job: `remote-handshake`
- Commands: `cargo build -p qsc --locked`, then
  `scripts/demo/qsc_remote_handshake_smoke.sh`
- Environment: `RELAY_URL` and `RELAY_TOKEN` from GitHub Actions secrets
- Persistent state: runner-local output and artifact directories only
- qsl-server/qsl-attachments: not involved
- Network: secret-backed external relay
- More evidence without mutation: existing run logs and uploaded artifacts

remote-relay:

- Workflow file: `.github/workflows/remote-relay-tests.yml`
- Job: `remote-relay`
- Commands: `cargo build -p qsc --locked`, then
  `scripts/demo/qsc_remote_relay_smoke.sh`
- Environment: relay URL/token secrets plus scenario/seed inputs
- Persistent state: runner-local qsc state and artifact directories only
- qsl-server/qsl-attachments: not involved
- Network: secret-backed external relay
- More evidence without mutation: existing run logs, marker files, counts, and
  uploaded artifacts

relay-ui-integration:

- Workflow file: `.github/workflows/relay-ui-integration.yml`
- Job: `relay-ui-integration`
- Commands: clone qsl-server outside workspace, build qsl-server, start it on
  loopback, probe health, then run ignored qsc relay UI tests
- Environment: loopback port and generated masked token
- Persistent state: runner-temp qsl-server clone/build and workspace artifacts
- qsl-server: GitHub Actions black-box clone/build/run
- qsl-attachments: shared helper module references exist, but the relay UI test
  does not directly invoke qsl-attachments helpers
- More evidence without mutation: existing run logs and relay-server artifact
  logs

## Per-Target Evidence Gap Analysis

- remote-handshake:
  `REMOTE_HANDSHAKE_GAP_ADDITIONAL_GITHUB_LOGS_REQUIRED`. The step failure is
  stable, but exact remediation cannot be chosen without the underlying
  artifact logs for the Alice/Bob vault steps.
- remote-relay:
  `REMOTE_RELAY_GAP_ADDITIONAL_GITHUB_LOGS_REQUIRED`. The marker error is
  stable, but exact remediation cannot be chosen without artifact marker,
  count, and send-log detail.
- relay-ui-integration:
  `RELAY_UI_GAP_ADDITIONAL_GITHUB_LOGS_REQUIRED`. The health 404 is stable, but
  exact ownership cannot be chosen without relay-server artifact logs and
  bounded workflow log review.

## Implementation Readiness Review

All three targets are:

`IMPLEMENTATION_NOT_READY_MORE_EVIDENCE_REQUIRED`

No exact remediation mutation paths are selected. Dependency/lockfile,
workflow, qsc source, and qsl-server/qsl-attachments mutation needs are unknown
until the additional evidence is captured and scanned.

## qsl-server / qsl-attachments Boundary Review

Classification:

`QSL_SERVER_BOUNDARY_GITHUB_ACTIONS_BLACK_BOX_ONLY`

qsl-server appears only in the relay-ui-integration workflow, where GitHub
Actions clones it outside the qsl-protocol workspace, builds it, starts it on
loopback, and probes health before running qsc tests. qsl-attachments helpers
appear in the shared qsc test helper module, but relay_ui_integration.rs does
not directly invoke them.

Future evidence can remain GitHub Actions black-box by reading existing run
logs and artifacts. Local qsl-server clone/build/run is not authorized by
NA-0548. If future artifact evidence proves qsl-server contract/source
ownership is required, a later boundary authorization lane must be selected.

## Follow-Up Evidence Permission Design

D-1086 authorizes a future NA-0549 targeted follow-up evidence lane to perform
read-only GitHub metadata/log/artifact capture for the already-created NA-0547
runs only.

Authorized future actions:

- `gh run view <known-run-id> --json databaseId,headSha,headBranch,event,status,conclusion,attempt,workflowName,jobs,url,createdAt,updatedAt`
- `gh api /repos/QuantumShieldLabs/qsl-protocol/actions/runs/<known-run-id>/jobs?per_page=100`
- `gh run view <known-run-id> --log`
- `gh api /repos/QuantumShieldLabs/qsl-protocol/actions/runs/<known-run-id>/artifacts?per_page=100`
- `gh run download <known-run-id> --name <known-artifact-name> --dir "$PROOF_DIR/artifacts/<target>/<known-run-id>"`

Known artifact names:

- `remote-handshake-artifacts`
- `remote-relay-artifacts`
- `relay-ui-integration-artifacts`

Known run IDs:

- remote-handshake: `28222737830`, `28298341119`
- remote-relay: `28221877145`, `28298371731`
- relay-ui-integration: `28221488004`, `28298405239`

No rerun, workflow dispatch, local reproduction, workflow/script/runtime
mutation, qsl-server/qsl-attachments local use, or public-site/Cloudflare work
is authorized by this design.

## Log and Redaction Policy

Raw logs and artifact contents must remain proof-root-only. Repository docs may
contain bounded summaries only after private-material scanning and redaction.

Every summary must preserve run ID, job ID, attempt, commit SHA, workflow file,
evidence type, step name, exit code, and error class. Stop if redaction would
remove the evidence needed for truthful classification.

## Private-Material Policy

Successor lanes must stop before repository summary if private material cannot
be safely redacted. Do not publish raw Authorization headers, tokens, secrets,
passphrases, private keys, SSH material, route tokens, capability values,
backup material, private endpoints/topology, Cloudflare tokens, or environment
secret values.

## Result Classification

`REMOTE_RELAY_FOLLOW_UP_EVIDENCE_TARGETED_GITHUB_CAPTURE_READY`

Exact remediation ownership is not safe yet, but bounded read-only GitHub
metadata/log/artifact capture from the already-created NA-0547 runs is exact
and safe.

## Selected Successor

`NA-0549 -- QSL Remote/Relay Non-Required CI Failure Targeted Follow-Up Evidence Harness`

The successor must execute only exact D-1086-authorized GitHub Actions
metadata/log/artifact actions and select a later exact remediation,
qsl-server-boundary, no-fix, or stop successor.

## Required-Check Boundary

public-safety completed success. advisories completed success. Branch-protection
required contexts were classified and no failed required check was observed.

## Workflow Mutation Boundary

No workflow mutation occurred in NA-0548. No future workflow mutation is
authorized by the selected successor.

## Runtime / qsc / Dependency Boundary

No runtime/source, qsc source/test/fuzz/Cargo, dependency manifest, or lockfile
mutation occurred in NA-0548. No future runtime/qsc/dependency mutation is
authorized by the selected successor.

## qsl-server / qsl-attachments Boundary

No qsl-server/qsl-attachments command, clone, build, local use, or source
mutation occurred in NA-0548. Future NA-0549 may only read existing GitHub
Actions black-box evidence unless D-1086 exact commands say otherwise.

## Remote-Action Boundary

NA-0548 executed only read-only GitHub API/metadata collection. It did not
execute reruns, workflow dispatches, cancellations, deletions, SSH, scp, sftp,
rsync, qsc send/receive, E2EE reproduction, or branch-protection mutation.

## Public-Site / Cloudflare Boundary

NA-0548 is not a public-site lane. It did not mutate README public-progress
content, `docs/public`, `public`, `website`, public-site content, deployment
settings, or Cloudflare configuration.

## qwork Operator Ergonomics Note

The operator reported that piping `qwork NA-0548 qsl-protocol` through `tee`
left the shell at the home directory. NA-0548 records this as operator
ergonomics/process friction only. It is not a qwork proof failure and no qwork
mutation is authorized or performed in this lane.

## Claim Boundary

NA-0548 makes none of these claims:

- no public-readiness claim
- no production-readiness claim
- no public-internet-readiness claim
- no external-review-complete claim
- no reproducibility-complete claim
- no backup/restore-complete claim
- no vulnerability-free claim
- no bug-free claim
- no perfect-build claim
- no perfect-crypto claim

## Validation

Validation must prove qwork proof, current main required-check classification,
all NA-0548 markers, exact five-path scope, changed Markdown links, added-line
private-material scan, added-line overclaim scan, docs/governance-only
classifier, PR body preflight, cargo audits, cargo fmt, and qsc-adversarial
shell syntax.

Focused qsc runtime tests may be skipped because NA-0548 is
authorization-only, performs no local reproduction, and mutates no runtime,
qsc, dependency, or workflow path.

## Recommendation

Proceed with the selected NA-0549 targeted follow-up evidence successor after
NA-0548 closeout. Do not select an implementation lane until the exact
mutation-path bundle is supported by the additional GitHub evidence.
