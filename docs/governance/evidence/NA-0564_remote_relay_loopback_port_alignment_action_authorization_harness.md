Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0564 Remote Relay Loopback Port Alignment Action Authorization Harness

## Executive Summary

NA-0564 executed the D-1116/D-1117-authorized non-mutating loopback-alignment
proof. Codex verified fresh qwork proof, current main health, queue state, and
the exact D-1116/D-1117 inheritance before generating a proof-root-only probe.

Selected result classification:
`LOOPBACK_ALIGNMENT_CANDIDATE_PROOF_READY`.

Selected successor:
`NA-0565 -- QSL Remote Relay Loopback Candidate Confirmation Proof Harness`.

The remote probe found a coarse loopback candidate listener class present with
count class `one`, but the D-1116 expected target value was not disclosed or
available to the proof script. Therefore NA-0564 does not authorize an operator
alignment action. It records a review-only action-bundle shell that requires
candidate confirmation before any future action.

## qwork Proof Verification

Fresh NA-0564 qwork proof files were copied into the proof root and parsed with
a file-backed parser before fetch, probe execution, or repository mutation.

- qwork proof timestamp: `2026-06-29T01:58:42Z`
- lane: `NA-0564`
- repo: `qsl-protocol`
- qwork HEAD: `c21c107cc7fe`
- qwork origin/main: `c21c107cc7fe`
- READY_COUNT: `1`
- READY: `NA-0564`
- worktree/index/untracked in qwork proof: clean
- cargo target mode: shared
- shared target ready: yes

The live pre-fetch checkout matched the qwork proof. Root disk usage was below
the stop threshold and `/backup/qsl` was mounted. Codex did not run qwork,
qstart, or qresume.

## D-1116 / D-1117 Inheritance

D-1116 exists once and is Accepted. It selected NA-0564 as a bounded
Codex-executed non-mutating loopback-alignment proof plus operator-owned action
bundle lane.

D-1117 exists once and is Accepted. It marked NA-0563 DONE and restored NA-0564
as the exactly one READY successor.

Inherited facts consumed:

- NA-0563 result classification:
  `REMOTE_RELAY_LOOPBACK_ALIGNMENT_ACTION_AUTH_READY`
- NA-0562 expected listener class: absent
- NA-0562 other loopback listener presence class: yes
- NA-0562 TCP class: refused
- NA-0562 v1 push HEAD class: not checked
- D-1116 Codex mutation authority for remote/service/account/tunnel state: no

No NA-0564 implementation existed before this directive, and inherited
repository evidence did not publish private material.

## Current Main Required-Check Classification

Current main was verified at `c21c107cc7fe`.

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
aggregate `goal-lint` and `CodeQL` required contexts as missing. This was a
recoverable proof-classifier issue because no required check failed or was
pending. The corrected classifier used PR #1400 status rollup for `goal-lint`
and successful attached analysis/code-scanning metadata for `CodeQL`.

## Loopback Alignment Probe Script Design and Static Review

The proof-root-only script
`probe_scripts/loopback_alignment_probe.py` was generated and statically
reviewed before remote execution.

The script:

- outputs exactly one JSON object;
- imports no subprocess, os, shutil, or tempfile modules;
- performs no file writes;
- calls no external commands;
- reads only `/proc/net/tcp` and `/proc/net/tcp6`;
- inspects only LISTEN state and loopback address classes;
- does not inspect service configuration;
- does not inspect authorized key material;
- does not run qsc;
- prints no raw address table, private port value, endpoint value, topology,
  process identity, token, bearer value, Authorization header, payload, or
  response body.

Because no non-secret expected target value was authorized into the script, the
script was redacted-by-construction and classified expected-target presence as
`unknown`.

## SSH Alignment Readiness

Codex ran the exact D-1116-authorized SSH readiness command once.

- classification: `SSH_ALIGNMENT_PROBE_READY`
- exit code: `0`
- private-material scan: pass
- command rerun: no

Local classifier recovery: the exact command's unquoted format string produced
the deterministic marker with a trailing `n` rather than a newline. The
classifier was corrected from the raw captured bytes without rerunning SSH.

## Remote Loopback Alignment Proof

Codex ran the exact D-1116-authorized remote loopback-alignment probe through
SSH stdin once after readiness succeeded.

- remote probe classification:
  `LOOPBACK_ALIGNMENT_UNKNOWN_ACCESS_LIMITED`
- JSON parse: pass
- private-material scan: pass
- expected target listener present: `unknown`
- other loopback listener presence: `yes`
- loopback listener count class: `one`
- candidate listener class: `present`
- expected target alignment class: `unknown`
- action owner: `unknown`
- operator action required: `unknown`
- codex mutation authorized: `no`

The raw SSH/probe stdout and stderr captures remain proof-root-only.

## Operator Action Bundle

The operator action bundle status is
`candidate-proof-only; no operator action authorized by NA-0564`.

The bundle is non-secret, placeholder-only, and begins with
`DO NOT RUN UNTIL DIRECTOR REVIEW`. It records that candidate confirmation is
required before any operator alignment action can be reviewed or executed.

No runnable privileged command, service command, SSH mutation command, tunnel
mutation command, firewall command, Tailscale command, qsc command, payload
push, workflow dispatch, or rerun command is included.

## GitHub Metadata Review

Read-only GitHub metadata reviewed:

- repository identity;
- current main check-runs and commit status;
- branch-protection required status checks;
- PR #1400 status rollup;
- main code-scanning analysis metadata.

No secret values were accessed. Repository variable values were not printed.

## Private-Material Review

Private-material scans passed for:

- SSH readiness raw stdout/stderr;
- remote alignment probe raw stdout/stderr;
- parsed remote probe JSON;
- proof-root operator manifests;
- added repository docs;
- final response draft before publication.

No endpoint values, private hosts/IPs/topology, private port values beyond
approved labels, route-token/capability values, bearer values, Authorization
headers, payloads, response bodies, process identities, raw authorized key
material, public SSH key material, private keys, Cloudflare tokens, API keys,
long opaque token strings, raw logs, raw artifacts, or private material were
published.

## Root-Cause / Action Classification

Selected result classification:
`LOOPBACK_ALIGNMENT_CANDIDATE_PROOF_READY`.

Rationale: the non-secret probe identified a coarse candidate listener class
with count class `one`, but expected-target alignment stayed `unknown` because
the expected target value was not disclosed to the probe. That is enough to
authorize a candidate confirmation successor, not an operator alignment action.

## Selected Successor

Selected successor:
`NA-0565 -- QSL Remote Relay Loopback Candidate Confirmation Proof Harness`.

The successor must run or review exact non-secret proof to confirm the safe
loopback listener candidate class identified by NA-0564 before any operator
action bundle can be authorized.

## Required-Check Boundary

public-safety and advisories were green before this patch. Required contexts
were classified green or conclusively satisfied, with no failed or pending
required check.

## Source / Script Mutation Boundary

No source path or repository script path was mutated. The generated probe script
remained proof-root-only and was not committed.

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

Remote action was limited to the two exact D-1116-authorized SSH commands:

- the SSH alignment readiness command;
- the remote loopback-alignment probe through SSH stdin.

No account, shell, authorized key material, Tailscale, firewall, service,
remote file, backup, qsl-server, or qsl-attachments mutation occurred.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, public path, website
path, deployment setting, public-site content, or Cloudflare configuration was
mutated.

## Raw Output Boundary

Raw SSH and remote probe outputs remain proof-root-only. Repository docs include
only coarse classifications and scan outcomes.

## Claim Boundary

No public-readiness claim was made. No production-readiness claim was made. No public-internet-readiness claim was made. No external-review-complete claim was made. No vulnerability-free claim was made. No bug-free claim was made. No perfect-build or perfect-crypto claim was made.

## Validation

Validation recorded:

- qwork/live repository proof: pass
- queue/decision proof: pass
- current main check classification: pass after recorded classifier recovery
- probe script syntax/static review: pass after recorded scanner recovery
- SSH readiness classification: pass after recorded local classifier recovery
- remote alignment probe JSON parse and private-material scan: pass
- selected result classification: pass
- exact successor selection: pass

Focused qsc runtime tests may be skipped because this patch is evidence,
governance, and operator-bundle documentation only, with no qsc source,
runtime, dependency, workflow, or repository-script mutation.

## Recommendation

Proceed to implementation PR review for D-1118. After merge and healthy
post-merge checks, close out NA-0564 by restoring the D-1118-selected
candidate confirmation proof successor as NA-0565. Do not run operator action
or mutate remote/service/account/tunnel state during closeout.
