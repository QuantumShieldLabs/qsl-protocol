Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0545 Remote/Relay Non-Required CI Failure Forward-Audit Authorization Plan

## Executive Summary

NA-0545 audited the three failed non-required remote/relay check-runs forwarded
from NA-0543:

- `remote-handshake`, run `28222737830`
- `remote-relay`, run `28221877145`
- `relay-ui-integration`, run `28221488004`

Current main at `594704571c36` has public-safety completed success,
advisories completed success, and no failed required checks. The three target
checks are not attached to the current main SHA, but the relevant workflow,
script, and test surfaces are unchanged since the failing `717b38ac7d3d`
merge. The failures are therefore not safe to dismiss as fixed from read-only
evidence alone.

Overall result:

`REMOTE_RELAY_FORWARD_AUDIT_REPRODUCTION_AUTHORIZATION_READY`

Selected successor:

`NA-0546 -- QSL Remote/Relay Non-Required CI Failure Bounded Reproduction Authorization Plan`

NA-0545 performed authorization and triage only. No implementation mutation,
workflow mutation, runtime mutation, dependency or lockfile mutation, qsc source
mutation, qsl-server/qsl-attachments use, remote action, qwork/qstart/qresume
execution, qsl-backup execution, public-site mutation, or Cloudflare mutation
occurred.

## qwork Proof Verification

Fresh qwork proof files were copied from `/srv/qbuild/work/NA-0545/.qwork/` and
verified from file-backed `.kv`, JSON, and cargo-target env parsing.

Verified fields included:

- `startup_result=OK`
- `lane=NA-0545`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0545/qsl-protocol`
- branch `main`
- upstream `origin/main`
- HEAD, `origin/main`, and `main` at `594704571c36`
- clean worktree, index, and untracked state
- READY_COUNT 1
- queue top READY NA-0545
- requested lane status READY
- qwork proof timestamp `2026-06-27T01:53:59Z`
- shared Cargo target mode and target directory as expected
- `explicit_target_preserved=no`
- `shared_target_ready=yes`

Codex did not run qwork, qstart, or qresume.

## D-1078 / D-1079 Inheritance

D-1078 exists once, is Accepted, and accepted the NA-0544 operator action proof
review. D-1079 exists once, is Accepted, marked NA-0544 DONE, and restored
NA-0545 as the sole READY item.

D-1079 restored NA-0545 as authorization and triage only for the non-required
remote/relay CI failures. It did not perform NA-0545 implementation. The
operator-supplied public-site context is acknowledged as future-route context
only; it is not part of NA-0545 mutation scope.

## Current Main Required-Check Classification

Fresh GitHub REST evidence was collected for current main `594704571c36`:

- current main check-runs
- branch-protection required status checks for `main`
- combined status API
- current main workflow runs
- PR #1362 status rollup for PR-only required contexts

Result:

- public-safety: completed success
- advisories: completed success
- no failed required checks
- branch-protection required contexts classified
- `goal-lint` satisfied by PR #1362 rollup
- `CodeQL` satisfied by PR #1362 rollup and current CodeQL analysis jobs
- `qsc-adversarial-smoke` visible on current main as non-required success

The combined status API returned no legacy statuses for the current SHA; check
runs and PR rollup data were the authoritative classification inputs.

## Prior Failed Run Inventory

Historical run metadata was visible for all three target runs:

| Target | Run ID | Workflow | Head | Event | Conclusion | Required now |
|---|---:|---|---|---|---|---|
| remote-handshake | 28222737830 | remote-handshake-tests | `717b38ac7d3d` | schedule | failure | no |
| remote-relay | 28221877145 | remote-relay-tests | `717b38ac7d3d` | schedule | failure | no |
| relay-ui-integration | 28221488004 | relay-ui-integration | `717b38ac7d3d` | schedule | failure | no |

Check-run lists for later merge commits `b4a64f78efe7`, `fec5a099e0ef`, and
`594704571c36` do not include these target check-runs. Relevant target workflow,
script, and qsc test paths did not change from `717b38ac7d3d` to current main.

## remote-handshake Forward Audit

Classification:

`REMOTE_HANDSHAKE_RUNTIME_SCOPE_LIKELY`

The historical run failed in job `remote-handshake`, step `Run remote handshake
smoke (happy-path seed=1)`. Bounded failed-log summaries show that the smoke
harness reached qsc vault initialization and failed while initializing the
Alice actor before handshake assertions could run.

The current workflow definition and referenced script still exist:

- `.github/workflows/remote-handshake-tests.yml`
- `scripts/demo/qsc_remote_handshake_smoke.sh`

The workflow uses GitHub Actions Linux runners, stable Rust, `cargo build -p qsc
--locked`, and secret-supplied relay URL/token values. Exact implementation
ownership cannot be selected from read-only evidence alone because the failure
could involve qsc runtime behavior, harness setup, or environment interaction.

Recommended successor action: bounded reproduction/rerun/log-capture before any
implementation lane.

## remote-relay Forward Audit

Classification:

`REMOTE_RELAY_RUNTIME_SCOPE_LIKELY`

The historical run failed in job `remote-relay`, step `Run remote relay smoke
(manual/nightly)`. Bounded failed-log summaries show qsc relay send reached a
contact-store validation error and the harness failed happy-path delivery-count
expectations.

The current workflow definition and referenced script still exist:

- `.github/workflows/remote-relay-tests.yml`
- `scripts/demo/qsc_remote_relay_smoke.sh`

The workflow uses GitHub Actions Linux runners, stable Rust, `cargo build -p qsc
--locked`, optional workflow-dispatch scenario inputs, and secret-supplied relay
values. Exact implementation ownership cannot be selected from read-only
evidence alone because the failure could involve qsc contact-store/runtime
behavior or remote-relay harness setup.

Recommended successor action: bounded reproduction/rerun/log-capture before any
implementation lane.

## relay-ui-integration Forward Audit

Classification:

`RELAY_UI_WORKFLOW_SCOPE_LIKELY`

The historical run failed in job `relay-ui-integration`, step `Start local relay
and run ignored relay UI integration tests`. Bounded failed-log summaries show
the workflow built and started a local relay, then a loopback relay health HTTP
request returned 404 before ignored relay UI tests ran.

The current workflow definition and referenced test still exist:

- `.github/workflows/relay-ui-integration.yml`
- `qsl/qsl-client/qsc/tests/relay_ui_integration.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`

The workflow clones `qsl-server` outside the workspace, builds it, starts a
local relay on Linux, generates a masked token, probes relay health, and runs
ignored qsc relay UI integration tests. Exact implementation ownership cannot
be selected from read-only evidence alone because a future fix might require
workflow/test adjustment or qsl-server contract review, and qsl-server mutation
is outside NA-0545.

Recommended successor action: bounded reproduction/rerun/log-capture before any
implementation lane.

## Current Workflow Inventory

Current workflow files:

- `.github/workflows/remote-handshake-tests.yml`
- `.github/workflows/remote-relay-tests.yml`
- `.github/workflows/relay-ui-integration.yml`

All three workflows are non-required scheduled/manual lanes on Linux. They are
not branch-protection required contexts. The remote-handshake and remote-relay
workflows consume GitHub secrets for relay access. The relay-ui-integration
workflow generates its own local token and depends on a fresh qsl-server clone
and build outside the qsl-protocol workspace.

## Referenced Script Inventory

Referenced in-repo paths:

- `scripts/demo/qsc_remote_handshake_smoke.sh`
- `scripts/demo/qsc_remote_relay_smoke.sh`
- `qsl/qsl-client/qsc/tests/relay_ui_integration.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`

The relay UI workflow also uses the external `QuantumShieldLabs/qsl-server`
repository read-only during CI. NA-0545 did not clone or run qsl-server.

## Log Visibility and Redaction Review

Failed-step logs were visible through read-only GitHub CLI log access. NA-0545
saved bounded proof-root summaries only. Full raw logs were not copied into
repository docs.

Saved proof-root log summaries were scanned for private material. The scan found
no unmasked private keys, GitHub tokens, bearer tokens, relay tokens, or
authorization header values. Masked secret placeholders were present, as
expected. Endpoint strings in proof-root logs are not copied into repository
docs.

## Per-Check Classification

- remote-handshake: `REMOTE_HANDSHAKE_RUNTIME_SCOPE_LIKELY`
- remote-relay: `REMOTE_RELAY_RUNTIME_SCOPE_LIKELY`
- relay-ui-integration: `RELAY_UI_WORKFLOW_SCOPE_LIKELY`

## Overall Result Classification

`REMOTE_RELAY_FORWARD_AUDIT_REPRODUCTION_AUTHORIZATION_READY`

Rationale: logs are visible and current workflow/script/test ownership is
identified, but the target checks have not rerun on the latest main SHA and the
exact future implementation path bundle is not safe to select from read-only
evidence alone.

## Future Scope Options

Option A, exact implementation successor, was rejected because exact mutation
ownership cannot be selected safely from read-only evidence.

Option B, bounded reproduction authorization successor, was selected because a
future lane can safely authorize exact rerun/log-capture/redaction boundaries
before implementation.

Option C, public-site Cloudflare verification successor, was rejected because
the target failures are not proven stale/no-fix-required.

## Selected Successor

Selected successor:

`NA-0546 -- QSL Remote/Relay Non-Required CI Failure Bounded Reproduction Authorization Plan`

The future lane should authorize bounded reproduction/rerun/log-capture only.
It must not mutate workflows, runtime, qsc, dependencies, qsl-server,
qsl-attachments, public content, or Cloudflare configuration unless a later
exact implementation lane is selected.

## Required-Check Boundary

public-safety completed success. advisories completed success. No failed
required check was observed. Branch-protection required contexts were
classified, including PR-only `goal-lint` and `CodeQL` satisfaction through PR
#1362 rollup.

## Workflow Mutation Boundary

No workflow mutation occurred. Workflow files were inspected read-only only.

## Remote / Relay Runtime Boundary

No remote action occurred. No remote command, SSH, scp, sftp, rsync, qsc
send/receive, qsc E2EE, GitHub workflow dispatch, rerun, cancellation, or branch
protection mutation occurred.

## qsc / Dependency / Lockfile Boundary

No qsc source/test/fuzz/Cargo mutation occurred. No dependency or lockfile
mutation occurred. The root `Cargo.lock`, nested qsc fuzz lockfile, and
`Cargo.toml` files were not changed by NA-0545.

## qsl-server / qsl-attachments Boundary

No qsl-server command was run by Codex. No qsl-attachments command was run by
Codex. No qsl-server or qsl-attachments path was mutated. The relay UI workflow
dependency on qsl-server is recorded as future-scope evidence only.

## Public-Site / Cloudflare Boundary

The operator-supplied context that qtsl.org and quantumshieldlabs.org exist as
Cloudflare-hosted surfaces was acknowledged. NA-0545 did not fetch, deploy,
change, or configure either site. No public-site mutation occurred. No
Cloudflare configuration mutation occurred.

## Private-Material Review

No private material was published. No raw logs were copied into repository docs.
Saved proof-root log summaries were scanned and passed. Repository evidence uses
summaries and avoids endpoint strings or raw masked log excerpts.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No reproducibility-complete claim is made. No backup/restore-complete
claim is made. No vulnerability-free claim is made. No bug-free claim is made.
No perfect-build claim is made. No perfect-crypto claim is made.

## Validation

Required local validation for this governance-only authorization lane includes:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- required marker proof
- changed Markdown link check
- added-line/new-file private-material scan
- saved-log-extract private-material scan
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint when available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0545 is authorization and
triage only and does not mutate qsc source/runtime/dependency/workflow or
protocol/wire/security implementation paths.

## Recommendation

Proceed to NA-0546 as bounded reproduction authorization. The future lane should
authorize exact rerun/log-capture/redaction boundaries, then decide whether a
later implementation lane needs workflow, harness, runtime, qsl-server, or other
exact paths.
