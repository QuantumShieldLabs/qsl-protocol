Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0546 Remote/Relay Non-Required CI Failure Bounded Reproduction Authorization Plan

## Executive Summary

NA-0546 authorizes the next bounded reproduction lane for the three
non-required remote/relay CI failures inherited from NA-0545:

- `remote-handshake`, historical run `28222737830`
- `remote-relay`, historical run `28221877145`
- `relay-ui-integration`, historical run `28221488004`

Result classification:

`REMOTE_RELAY_REPRODUCTION_AUTHORIZATION_PARTIAL_GITHUB_RERUN_ONLY`

Historical GitHub reruns and current-main workflow dispatch are safe and useful
as future GitHub Actions black-box reproduction paths. Local reproduction is
not authorized: the remote smoke lanes require private relay endpoint/token
boundaries, and relay UI local reproduction would require qsl-server
clone/build/run scope.

Selected successor:

`NA-0547 -- QSL Remote/Relay Non-Required CI Failure Bounded Reproduction and Log Capture Harness`

NA-0546 is authorization-only. It did not rerun historical jobs, dispatch
workflows, execute local reproduction, mutate workflows, mutate runtime/qsc
source, update dependencies or lockfiles, run qsl-server/qsl-attachments, run
remote commands, execute qwork/qstart/qresume, run qsl-backup, mutate public
site content, mutate Cloudflare, publish private material, or copy raw logs
into repository docs.

## qwork Proof Verification

Fresh qwork proof files were copied from `/srv/qbuild/work/NA-0546/.qwork/`
into the proof root and verified from `.kv`, JSON, and cargo-target env files.
Codex did not run qwork, qstart, or qresume.

Verified proof values included:

- `startup_result=OK`
- `lane=NA-0546`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0546/qsl-protocol`
- `head=0e4560febcc459add01083e43c24c19d3b19c813`
- `origin_main=0e4560febcc459add01083e43c24c19d3b19c813`
- `main=0e4560febcc459add01083e43c24c19d3b19c813`
- `ready_count=1`
- `queue_top_ready=NA-0546`
- `proof_written_at_utc=2026-06-27T03:21:51Z`
- shared Cargo target mode and expected rustc toolchain key

Proof artifacts:

- `/srv/qbuild/tmp/NA0546_remote_relay_reproduction_authorization_20260627T033115Z/qwork/qwork_proof_verification.md`
- `/srv/qbuild/tmp/NA0546_remote_relay_reproduction_authorization_20260627T033115Z/qwork/qwork_proof_verification.json`

## D-1080 / D-1081 Inheritance

D-1080 and D-1081 were consumed and verified accepted once.

D-1080 selected `REMOTE_RELAY_FORWARD_AUDIT_REPRODUCTION_AUTHORIZATION_READY`
after NA-0545 could not choose exact implementation ownership from read-only
evidence. D-1080 inherited these target classifications:

- `remote-handshake`: `REMOTE_HANDSHAKE_RUNTIME_SCOPE_LIKELY`
- `remote-relay`: `REMOTE_RELAY_RUNTIME_SCOPE_LIKELY`
- `relay-ui-integration`: `RELAY_UI_WORKFLOW_SCOPE_LIKELY`

D-1080 did not classify the failures as stale/no-fix-required. D-1081 restored
NA-0546 as authorization-only and confirmed that no NA-0546 implementation had
occurred.

## Current Main Required-Check Classification

Current main was verified at:

`0e4560febcc459add01083e43c24c19d3b19c813`

Fresh read-only GitHub API data classified branch-protection required contexts.
Direct current-main check-runs were green for required push checks including
`public-safety`, `ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`,
`demo-cli-build`, `demo-cli-smoke`, `formal-scka-model`,
`metadata-conformance-smoke`, `suite2-vectors`, and
`macos-qsc-qshield-build`.

PR #1364 read-only rollup proved PR-only required contexts `goal-lint` and
`CodeQL` completed success before merge to the current main SHA. CodeQL Analyze
check-runs were also visible on the merge commit.

`public-safety` completed success. `advisories` completed success. No failed
required checks were observed. The three target remote/relay checks were absent
on current main and classified as non-required reproduction targets, not
required-check regressions.

## Target Failure Inheritance

The inherited target failures are:

- `remote-handshake`, run `28222737830`, failed in `remote-handshake-tests`
- `remote-relay`, run `28221877145`, failed in `remote-relay-tests`
- `relay-ui-integration`, run `28221488004`, failed in `relay-ui-integration`

Historical job metadata showed failed steps:

- remote-handshake: `Run remote handshake smoke (happy-path seed=1)`
- remote-relay: `Run remote relay smoke (manual/nightly)`
- relay-ui-integration: `Start local relay and run ignored relay UI integration tests`

## Workflow Trigger Inventory

All three workflow files were identified read-only:

- `.github/workflows/remote-handshake-tests.yml`
- `.github/workflows/remote-relay-tests.yml`
- `.github/workflows/relay-ui-integration.yml`

All three workflows have `workflow_dispatch` and `schedule` triggers, no
`push`, no `pull_request`, and no `workflow_call`. All run on `ubuntu-latest`
with `contents: read`. No workflow caches or matrices were identified. The
workflows publish artifacts under their existing artifact names.

## Referenced Command Inventory

Future GitHub Actions reproduction uses the existing workflow commands only.

`remote-handshake-tests`:

- `cargo build -p qsc --locked`
- `./scripts/demo/qsc_remote_handshake_smoke.sh --scenario happy-path --seed 1 --out ./_remote_handshake_out/happy-path`
- `./scripts/demo/qsc_remote_handshake_smoke.sh --scenario drop-reorder --seed 7 --out ./_remote_handshake_out/drop-reorder`

`remote-relay-tests`:

- `cargo build -p qsc --locked`
- `./scripts/demo/qsc_remote_relay_smoke.sh --scenario "${SCENARIO}" --seed "${SEED}" --out ./_remote_relay_out`

`relay-ui-integration`:

- clone qsl-server into `$RUNNER_TEMP/qsl-server`
- build qsl-server with `cargo build --release --locked`
- start qsl-server on `127.0.0.1:${QSC_RELAY_UI_PORT}`
- probe loopback health endpoints
- `cargo test -p qsc --test relay_ui_integration -- --ignored --nocapture`

## Historical Rerun Authorization Design

Historical reruns are authorized only for a later lane and only as GitHub
Actions black-box reruns of the existing failed runs:

- `gh run rerun 28222737830 --failed`
- `gh run rerun 28221877145 --failed`
- `gh run rerun 28221488004 --failed`

Each is classified:

`HISTORICAL_RERUN_AUTHORIZED_FOR_FUTURE_LANE`

These reruns are safe because they are non-required existing workflows and do
not mutate repository state. They are useful for determining whether the
historical SHA remains failing or flaky, but they do not by themselves prove
current-main reproduction.

## Current-Main Workflow Dispatch Authorization Design

Current-main dispatch is authorized only for a later lane and only with exact
commands pinned to `main`:

- `gh workflow run remote-handshake-tests.yml --ref main`
- `gh workflow run remote-relay-tests.yml --ref main -f scenario=happy-path -f seed=1`
- `gh workflow run relay-ui-integration.yml --ref main`

Each is classified:

`CURRENT_MAIN_DISPATCH_AUTHORIZED_FOR_FUTURE_LANE`

Remote-handshake and remote-relay use repository Actions secrets
`RELAY_URL` and `RELAY_TOKEN`. relay-ui-integration generates a masked loopback
token inside GitHub Actions and uses qsl-server only through the existing
workflow.

## Local Reproduction Authorization Design

Local reproduction is not authorized by NA-0546.

- remote-handshake: `LOCAL_REPRO_NOT_ALLOWED_SECRET_OR_REMOTE_RISK`
- remote-relay: `LOCAL_REPRO_NOT_ALLOWED_SECRET_OR_REMOTE_RISK`
- relay-ui-integration: `LOCAL_REPRO_NOT_ALLOWED_QSL_SERVER_BOUNDARY`

If a future lane wants local reproduction, it must separately authorize private
relay endpoint handling or exact local qsl-server clone/build/run scope. Any
future local runtime reproduction would require isolated proof-root output and
an isolated Cargo target because it would be runtime/provenance evidence.

## qsl-server / qsl-attachments Boundary Design

remote-handshake and remote-relay have no qsl-server/qsl-attachments
involvement.

relay-ui-integration has workflow-transitive qsl-server involvement only. The
future lane may rerun or dispatch the existing GitHub Actions workflow as a
black box, but NA-0546 does not authorize local qsl-server command execution,
clone, build, mutation, or source inspection beyond the current workflow file.

qsl-attachments helpers exist in the qsc common test module, but
`relay_ui_integration.rs` does not directly invoke qsl-attachments.

## Log Capture Policy

Future raw logs may be saved only under the NA-0547 proof root, never committed
to the repository. Repository docs may contain bounded summaries only after
private-material scanning and redaction.

Every future log summary must identify whether the evidence came from
historical rerun, current-main dispatch, or local reproduction; it must include
run ID, job ID, attempt, commit SHA, workflow file, workflow name, step name,
status/conclusion, and bounded failure detail.

NA-0547 proof-root layout:

- `runs/<target>_<run_id>_run.json`
- `jobs/<target>_<run_id>_jobs.json`
- `logs/raw/<target>_<run_id>_<job_id>_attempt<attempt>.log`
- `logs/redacted/<target>_<run_id>_<job_id>_attempt<attempt>.redacted.log`
- `logs/summaries/<target>_<run_id>_<job_id>_attempt<attempt>.summary.md`
- `private_material_scan/<target>_<run_id>_<job_id>_attempt<attempt>.scan.json`

## Redaction Policy

Redact secrets, tokens, capabilities, credentials, private keys, passphrases,
route tokens, raw SSH config, `authorized_keys`, `known_hosts`, private
topology, backup material, private endpoint material, Authorization headers,
relay URLs when private, and operator-specific local paths when not needed.

Preserve workflow names, step names, run/job IDs, attempts, commit SHAs, exit
codes/conclusions, and bounded failure messages. If redaction would destroy the
evidence needed for classification, NA-0547 must stop with a private-material
limitation instead of publishing unsafe evidence.

## Private-Material Scan Policy

NA-0547 must scan all saved raw logs, redacted logs, summaries, added lines,
and new files before summarizing evidence. The scan must look for secret/token
terms, Authorization header values, private-key blocks, SSH config material,
route tokens, backup-material indicators, private endpoints, and unclassified
high-entropy token-like strings.

No raw workflow logs may be pasted into repository docs or the response.

## Future Reproduction Classification Design

NA-0547 must use these per-target classifications:

remote-handshake:

- `REMOTE_HANDSHAKE_REPRODUCED_CURRENT`
- `REMOTE_HANDSHAKE_REPRODUCED_HISTORICAL_ONLY`
- `REMOTE_HANDSHAKE_NOT_REPRODUCED`
- `REMOTE_HANDSHAKE_FLAKY_OR_INTERMITTENT`
- `REMOTE_HANDSHAKE_BLOCKED_BY_LOG_VISIBILITY`
- `REMOTE_HANDSHAKE_BLOCKED_BY_SECRET_OR_BOUNDARY`
- `REMOTE_HANDSHAKE_AMBIGUOUS_STOP`

remote-relay:

- `REMOTE_RELAY_REPRODUCED_CURRENT`
- `REMOTE_RELAY_REPRODUCED_HISTORICAL_ONLY`
- `REMOTE_RELAY_NOT_REPRODUCED`
- `REMOTE_RELAY_FLAKY_OR_INTERMITTENT`
- `REMOTE_RELAY_BLOCKED_BY_LOG_VISIBILITY`
- `REMOTE_RELAY_BLOCKED_BY_SECRET_OR_BOUNDARY`
- `REMOTE_RELAY_AMBIGUOUS_STOP`

relay-ui-integration:

- `RELAY_UI_REPRODUCED_CURRENT`
- `RELAY_UI_REPRODUCED_HISTORICAL_ONLY`
- `RELAY_UI_NOT_REPRODUCED`
- `RELAY_UI_FLAKY_OR_INTERMITTENT`
- `RELAY_UI_BLOCKED_BY_QSL_SERVER_BOUNDARY`
- `RELAY_UI_BLOCKED_BY_LOG_VISIBILITY`
- `RELAY_UI_BLOCKED_BY_SECRET_OR_BOUNDARY`
- `RELAY_UI_AMBIGUOUS_STOP`

Overall NA-0547 classifications:

- `REMOTE_RELAY_REPRODUCTION_IMPLEMENTATION_SCOPE_READY`
- `REMOTE_RELAY_REPRODUCTION_STALE_OR_FLAKY_NO_FIX_READY`
- `REMOTE_RELAY_REPRODUCTION_PARTIAL_MORE_EVIDENCE_REQUIRED`
- `REMOTE_RELAY_REPRODUCTION_QSL_SERVER_BOUNDARY_STOP`
- `REMOTE_RELAY_REPRODUCTION_PRIVATE_MATERIAL_STOP`
- `REMOTE_RELAY_REPRODUCTION_AMBIGUOUS_STOP`

## Result Classification

`REMOTE_RELAY_REPRODUCTION_AUTHORIZATION_PARTIAL_GITHUB_RERUN_ONLY`

GitHub historical rerun and current-main dispatch are exactly authorized for
the future lane. Local reproduction is not authorized.

## Selected Successor

`NA-0547 -- QSL Remote/Relay Non-Required CI Failure Bounded Reproduction and Log Capture Harness`

## Required-Check Boundary

NA-0546 verified public-safety success, advisories success, and no failed
required checks before mutation. Target remote/relay workflows are non-required
and do not block required-check health when absent or failed.

## Workflow Mutation Boundary

No workflow file was modified. NA-0547 may only run exact D-1082-authorized
GitHub Actions commands and must not mutate workflow files.

## Runtime / qsc / Dependency Boundary

No runtime, qsc source/test/fuzz/Cargo path, dependency manifest, or lockfile
was modified. Focused qsc runtime tests were skipped because NA-0546 is
authorization-only and performed no runtime/source/dependency/workflow
mutation.

## Remote-Action Boundary

No remote command, SSH, scp, sftp, rsync, qsc send/receive, E2EE reproduction,
workflow dispatch, or rerun occurred.

## Public-Site / Cloudflare Boundary

NA-0546 is not a public-site lane. It did not mutate public-site content,
`docs/public`, `public`, `website`, README public-progress content,
deployment settings, or Cloudflare configuration.

## Claim Boundary

NA-0546 makes none of these claims:

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

Validation proof is recorded under:

`/srv/qbuild/tmp/NA0546_remote_relay_reproduction_authorization_20260627T033115Z/validation/`

Required validation includes diff scope guard, queue/decision proof, marker
proof, link-check, private-material scan, overclaim scan, docs/governance-only
classifier, PR body preflight, goal-lint if available, cargo audits, cargo fmt,
and qsc-adversarial shell syntax.

## Recommendation

Proceed to NA-0547 using only the exact GitHub historical rerun and
current-main workflow-dispatch commands authorized above. Do not execute local
reproduction unless a later directive separately authorizes private relay or
qsl-server boundaries.
