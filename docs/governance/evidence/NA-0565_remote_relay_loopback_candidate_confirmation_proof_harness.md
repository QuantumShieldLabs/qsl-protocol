Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0565 Remote Relay Loopback Candidate Confirmation Proof Harness

## Executive Summary

NA-0565 executed the D-1118/D-1119-selected candidate confirmation proof for
the coarse loopback listener class found by NA-0564. The lane verified fresh
qwork proof, current main health, queue state, and inherited NA-0564 evidence
before generating a proof-root-only candidate probe.

Selected result classification:
`LOOPBACK_CANDIDATE_MISSING_SERVICE_DEPLOYMENT_PROOF_REQUIRED`.

Selected successor:
`NA-0566 -- QSL Remote Relay Listener Deployment Proof Authorization Plan`.

The exact SSH readiness command succeeded. The exact remote candidate probe ran
once through SSH stdin and returned valid coarse JSON. The strict candidate
confirmation result was `CANDIDATE_CONFIRMATION_NO_CANDIDATE`: no usable safe
loopback candidate was visible to this confirmation probe. NA-0565 therefore
does not authorize the D-1118 operator alignment bundle and instead selects an
authorization-only deployment proof lane.

## qwork Proof Verification

Fresh NA-0565 qwork proof files were copied into the proof root and parsed with
a file-backed parser before fetch, probe execution, or repository mutation.

- qwork proof timestamp: `2026-06-29T02:58:54Z`
- lane: `NA-0565`
- repo: `qsl-protocol`
- qwork HEAD: `5cefcca84667`
- qwork origin/main: `5cefcca84667`
- READY_COUNT: `1`
- READY: `NA-0565`
- worktree/index/untracked in qwork proof: clean
- cargo target mode: shared
- shared target ready: yes

The live pre-fetch checkout matched the qwork proof. Root disk usage was below
the stop threshold and `/backup/qsl` was mounted. Codex did not run qwork,
qstart, or qresume.

## D-1118 / D-1119 Inheritance

D-1118 exists once and is Accepted. It selected NA-0565 as a bounded
candidate-confirmation proof lane after NA-0564 found one coarse candidate
listener class but left expected-target alignment unknown.

D-1119 exists once and is Accepted. It marked NA-0564 DONE and restored NA-0565
as the exactly one READY successor.

Inherited NA-0564 facts consumed:

- result classification: `LOOPBACK_ALIGNMENT_CANDIDATE_PROOF_READY`
- expected target listener present: `unknown`
- other loopback listener presence: `yes`
- loopback listener count class: `one`
- candidate listener class: `present`
- expected target alignment class: `unknown`
- action owner: `unknown`
- operator action required: `unknown`
- operator action bundle status: review-only; no action authorized by NA-0564

NA-0564 generated a placeholder-only action bundle beginning with the Director
review warning. NA-0565 did not execute that bundle.

## Current Main Required-Check Classification

Current main was verified at `5cefcca84667`.

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success
- failed required checks: none
- pending required checks: none
- branch-protection required contexts: classified green or conclusively
  satisfied
- Cargo.toml drift: none
- Cargo.lock drift: none
- nested qsc fuzz Cargo.lock drift: none

Recovered proof-tooling note: the first current-main classifier treated
required contexts that were not all visible as merge-commit check runs as
missing. This was a recoverable metadata classifier issue before repository
mutation. The corrected classifier consumed current main check-runs/statuses,
current main workflow metadata, merged PR #1402 head checks, and successful
analysis jobs for aggregate required context satisfaction.

## Candidate Confirmation Probe Script Design and Static Review

The proof-root-only script
`probe_scripts/candidate_confirmation_probe.py` was generated and statically
reviewed before remote execution.

The script:

- outputs exactly one JSON object;
- imports no subprocess, os, shutil, tempfile, or pathlib modules;
- performs no file writes;
- calls no external commands;
- reads only proc-net TCP listener tables;
- inspects only LISTEN state and loopback address classes;
- does not inspect service configuration;
- does not inspect authorized key material;
- does not run qsc;
- attempts TCP and HEAD-only path checks only when exactly one candidate exists;
- sends no auth, route, bearer, payload, or body material;
- reads only the status line for HEAD classification;
- prints no raw address table, private port value, endpoint value, topology,
  process identity, token, bearer value, Authorization header, payload, or
  response body.

Static review passed.

## SSH Candidate Readiness

Codex ran the exact NA-0565 SSH readiness command once.

- classification: `SSH_CANDIDATE_CONFIRMATION_READY`
- exit code: `0`
- readiness marker present: yes
- private-material scan: pass
- command rerun: no

The raw stdout and stderr captures remain proof-root-only.

## Remote Candidate Confirmation Proof

Codex ran the exact NA-0565 remote candidate-confirmation probe through SSH
stdin once after readiness succeeded.

- remote probe classification: `CANDIDATE_CONFIRMATION_NO_CANDIDATE`
- JSON parse: pass
- private-material scan: pass
- candidate listener count class: `none`
- candidate listener class: `absent`
- candidate TCP connect class: `not_checked`
- candidate v1 push HEAD class: `not_checked`
- candidate v1 pull HEAD class: `not_checked`
- candidate relay shape class: `unknown`
- candidate confirmation class: `unknown`
- expected target alignment class: `unknown`
- action owner: `unknown`
- operator action required: `unknown`
- codex mutation authorized: `no`

The raw SSH/probe stdout and stderr captures remain proof-root-only. Repository
docs publish only the coarse classes above.

## GitHub Metadata Review

Read-only GitHub metadata reviewed:

- repository identity;
- current main check-runs and commit status;
- current main workflow runs and jobs;
- branch-protection required status checks;
- merged PR #1402 metadata and head checks.

No secret values were accessed. Repository variable values were not printed.
No GitHub mutation was performed during metadata review.

## Private-Material Review

Private-material scans passed for:

- SSH readiness raw stdout/stderr;
- remote candidate probe raw stdout/stderr;
- parsed remote candidate JSON;
- proof summaries used for repository docs.

No endpoint values, private hosts/IPs/topology, private port values beyond
approved labels, route-token/capability values, bearer values, Authorization
headers, payloads, response bodies, process identities, raw authorized key
material, public SSH key material, private keys, Cloudflare tokens, API keys,
long opaque token strings, raw logs, raw artifacts, or private material were
published.

Recovered scan note: the first aggregate private-material scan flagged the
approved long NA-0566 successor path names as opaque strings. This was a local
scanner-context false positive. The scanner was rerun with only those exact
approved governance path identifiers allowed, and the final aggregate scan
passed.

## Root-Cause / Candidate Classification

Selected result classification:
`LOOPBACK_CANDIDATE_MISSING_SERVICE_DEPLOYMENT_PROOF_REQUIRED`.

Rationale: the strict NA-0565 candidate confirmation probe did not find any
usable safe loopback candidate listener class. That means NA-0565 cannot make
the D-1118 review-only operator action bundle actionable. The next safe step is
authorization-only proof design for whether the remote relay listener is
stopped, uninstalled, bound elsewhere, or intentionally absent.

## Selected Successor

Selected successor:
`NA-0566 -- QSL Remote Relay Listener Deployment Proof Authorization Plan`.

The selected successor is authorization-only and must define exact non-secret
service-owner or operator proof for the absence of any usable loopback relay
candidate. It must not publish private ports, endpoint values, topology,
process identity, token values, bearer values, Authorization headers, payloads,
response bodies, or secret environment values.

## Required-Check Boundary

public-safety and advisories were green before this patch. Required contexts
were classified green or conclusively satisfied, with no failed or pending
required check.

## Source / Script Mutation Boundary

No source path or repository script path was mutated. The generated candidate
probe script remained proof-root-only and was not committed.

## Workflow Mutation Boundary

No workflow file was mutated. No workflow dispatch, rerun, cancel, or workflow
run mutation occurred.

## Runtime / qsc / Dependency Boundary

No qsc send/receive, qsc E2EE, qsc runtime reproduction, dependency edit,
Cargo.toml edit, Cargo.lock edit, or nested qsc fuzz lockfile edit occurred.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, or mutation
occurred.

## Remote-Action Boundary

Remote action was limited to the two exact NA-0565-authorized SSH commands:

- the SSH candidate readiness command;
- the remote candidate-confirmation probe through SSH stdin.

No account, shell, authorized key material, Tailscale, firewall, service,
remote file, backup, qsl-server, or qsl-attachments mutation occurred.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, public path, website
path, deployment setting, public-site content, or Cloudflare configuration was
mutated.

## Raw Output Boundary

Raw SSH and remote probe outputs remain proof-root-only. Repository docs include
only coarse classifications and scan outcomes. Raw logs and raw artifacts were
not committed.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No
public-internet-readiness claim was made. No external-review-complete claim was
made. No vulnerability-free claim was made. No bug-free claim was made. No
perfect-build claim was made. No perfect-crypto claim was made.

## Validation

Validation is recorded in proof-root-only artifacts and summarized by
`tests/NA-0565_remote_relay_loopback_candidate_confirmation_proof_testplan.md`.

Focused qsc runtime tests are skipped for this governance-only change because no
qsc source/runtime/dependency/workflow mutation occurred and qsc local
send/receive was not authorized.

## Recommendation

Proceed to the selected NA-0566 listener deployment proof authorization lane
after implementation merge and closeout. The next lane should design exact
non-secret proof requirements without authorizing Codex service mutation,
account mutation, SSH/Tailscale mutation, workflow dispatch/rerun, qsc
send/receive, qsl-server/qsl-attachments mutation, public-site mutation, or
private-material publication.
