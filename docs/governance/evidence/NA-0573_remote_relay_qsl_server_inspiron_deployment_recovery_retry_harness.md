Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0573 Remote Relay qsl-server Inspiron Deployment Recovery Retry Harness

## Executive Summary

NA-0573 consumed the D497 required-check visibility stop, applied the D498
PR-head/rollup recovery model, acquired and validated qsl-server from the
qsl-server repository, staged the qsl-server binary under the qslcodex remote
test workspace, and stopped before remote start because no authorized proof
source provided the expected internal loopback bind target.

Result classification:
`QSL_SERVER_RECOVERY_EXPECTED_BIND_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`.

Selected successor:
`NA-0574 -- QSL Remote qsl-server Start / Bind Operator Proof Authorization Plan`.

## qwork Proof Verification

Fresh qwork proof files were copied from the NA-0573 lane workspace and verified
before qsl-server source acquisition, SSH, scp, remote script execution, or
repository mutation.

- startup result: OK
- lane/repo/path: NA-0573 / qsl-protocol / expected qbuild worktree
- branch/upstream: main / origin/main
- HEAD and origin/main before fetch: `c89c758d4174`
- worktree, index, and untracked state: clean
- queue proof: READY_COUNT 1, READY NA-0573
- cargo target mode: shared, ready
- root disk: below the 95 percent stop threshold
- `/backup/qsl`: mounted

Codex did not run qwork, qstart, or qresume.

## D497 / D-1134 / D-1135 Inheritance

D497 was consumed as a pre-implementation stop. It stopped because `goal-lint`
was required by branch protection but was not visible as a literal main-commit
check-run/status context under that directive.

Inheritance proof:

- D497 did not run qsl-server source acquisition.
- D497 did not run SSH, scp, or remote scripts.
- D497 did not mutate repository files.
- D497 did not create D-1136.
- D-1134 exists once and is Accepted.
- D-1135 exists once and is Accepted.
- NA-0572 is DONE.
- NA-0573 is READY.
- qsl-server PR #57 merged at `6bf61d439fa2`.
- qsl-server now locks `quinn-proto 0.11.15`.
- No qsl-server deployment occurred before this directive.

## Required-Check Visibility Recovery

The D498 recovery model was applied to qsl-protocol main at `c89c758d4174`.
The governing PR was #1418, merged with head `f3b5e9a31326`.

Recovered context:

- `goal-lint`: absent as a literal governing-main check-run/status context,
  recovered from merged PR #1418 head evidence, completed success.

Aggregate context:

- `CodeQL`: satisfied through merged PR-head aggregate success and successful
  current-main language analysis check-runs.

A malformed GitHub API endpoint used during proof collection was classified as
a recoverable command-shape issue and corrected once with the proper statuses
endpoint. Final required-check classification passed.

## Current Main Required-Check Classification

Required contexts were classified as success or conclusively satisfied:

- public-safety: completed success
- advisories: completed success
- suite2-vectors: completed success
- ci-4a, ci-4b, ci-4c, ci-4d, ci-4d-dur: completed success
- demo-cli-build and demo-cli-smoke: completed success
- formal-scka-model: completed success
- metadata-conformance-smoke: completed success
- macos-qsc-qshield-build: completed success
- goal-lint: PR-head/rollup success
- CodeQL: aggregate/PR-head success

Failed required contexts: zero. Pending or ambiguous required contexts after the
recovery model: zero. Non-required failed check-runs on the governing main
commit: zero.

## qsl-server Source Acquisition

qsl-server was cloned under the proof root with:
`gh repo clone QuantumShieldLabs/qsl-server`.

Source proof:

- qsl-server commit: `6bf61d439fa2`
- source branch: main
- source status: clean
- qsl-server main equals the PR #57 merge
- Cargo.toml present
- Cargo.lock present
- Cargo.lock `quinn-proto`: `0.11.15`
- package/binary: qsl-server / qsl-server
- canonical route shape: canonical push/pull route shape present
- loopback start controls: bind and port CLI available, loopback default present

## qsl-server Audit / Build / Test / Binary Manifest

qsl-server validation completed under proof-root-only source/build paths.

- `cargo metadata --locked --format-version=1`: pass
- `cargo audit --deny warnings`: pass
- `cargo build --release --locked --target-dir <proof-root target>`: pass
- `cargo test --locked`: pass
- `cargo fmt --check`: pass
- expected binary path: proof-root release qsl-server
- binary size: 3490752 bytes
- binary SHA-256: full value recorded proof-root-only; prefix `a0de07ec8ae4`
- `qsl-server --version`: available
- `qsl-server --help`: available
- `qsl-server help`: non-fatal metadata-shape result

## Remote Script Design and Static Review

Generated proof-root-only scripts:

- `qsl_server_inventory.py`
- `qsl_server_stage.py`
- `qsl_server_start.py`
- `qsl_server_postcheck.py`

Static review passed:

- syntax check passed for all scripts
- stdout is JSON-only by construction
- no `shell=True`
- no sudo, systemctl, service, journalctl, Tailscale, firewall, or
  authorized_keys access
- no qsc send/receive/E2EE
- no qsl-attachments command
- remote writes are rooted under `/home/qslcodex/qsl-remote-test/`
- rollback manifest is created before replacing qsl-server
- private bind target is never printed
- response bodies are never read or printed

One AST-summary command had a heredoc redirection shape error. It was classified
as recoverable proof-tooling shape, corrected once, and final static review
passed.

## SSH Readiness

The exact authorized SSH readiness command was run once. It returned exit 0, no
stderr, and the expected sentinel with the literal trailing `n` produced by the
directive command form.

Classification:
`SSH_QSL_SERVER_RECOVERY_READY`.

Private-material scan for readiness output passed.

## Remote Inventory

Inventory ran through SSH stdin and emitted JSON only.

Classification:
`QSL_SERVER_INVENTORY_STAGE_REQUIRED`.

Coarse classes:

- workspace exists and is writable
- qsc binary exists and is executable
- qsl-server binary was missing before staging
- expected bind target unavailable from authorized sources
- staging needed
- start not attempted by inventory
- repairable within qslcodex workspace
- no sudo/service/secret/endpoint action requested by inventory

## Remote Transfer and Stage

The proof-root qsl-server release binary was transferred to the authorized
remote tmp path and staged through the reviewed stage script.

Stage classification:
`QSL_SERVER_STAGE_INSTALLED_OR_REPLACED`.

Stage proof:

- staged binary present
- qsl-server staged executable
- rollback/stage manifest created before replacement
- previous binary backup class: none previous
- remote write scope: qslcodex workspace only
- Codex mutation scope respected

## Remote Start

Remote start was not executed.

Classification:
`QSL_SERVER_START_EXPECTED_BIND_UNAVAILABLE`.

Reason: no expected internal loopback bind target was safely available from the
authorized proof sources. Codex did not guess a private port, did not use a
public bind, and did not run the remote start command.

## Remote Postcheck

Postcheck ran through SSH stdin after staging.

Classification:
`QSL_SERVER_POSTCHECK_BINARY_READY_LISTENER_NOT_READY`.

Coarse proof:

- qsl-server binary ready
- expected listener not checked because expected bind was unavailable
- route probes not run
- relay testing readiness: not ready
- rollback manifest present
- stage manifest present
- start manifest absent
- no response body read or published

## Rollback and Cleanup Proof

The stage script created the recovery/stage manifest before installing the
qsl-server binary. Because there was no previous qsl-server binary in the remote
workspace, no previous-binary backup was needed.

Remote writes were limited to `/home/qslcodex/qsl-remote-test/`.

## Private-Material Review

Private-material review passed after bounded scan recovery and manual
adjudication of public metadata URL false positives.

No endpoint values, private port values, route-token/capability values, bearer
values, Authorization headers, private topology, process identity, payloads,
response bodies, authorized_keys content, public key material, private key
material, secret environment values, Cloudflare tokens, or API keys were
published.

Raw qsl-server help, build/test logs, SSH/scp output, remote JSON output, and
full binary hash remain proof-root-only.

## Result Classification

Selected result:
`QSL_SERVER_RECOVERY_EXPECTED_BIND_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`.

Reason: qsl-server source/build/audit/test/fmt passed and the binary was staged
successfully, but no safe expected internal loopback bind target was available,
so qsl-server was not started and relay testing is not ready.

## Selected Successor

Selected successor model: Option C.

Exact successor:
`NA-0574 -- QSL Remote qsl-server Start / Bind Operator Proof Authorization Plan`.

The successor must authorize exact operator-owned or service-owner proof/action
requirements to identify the private loopback bind target or safe qsl-server
start command. Codex remains read-only until exact non-secret proof is available.

## Required-Check Boundary

The required-check recovery model did not ignore failed checks. It recovered
only missing visibility for PR-scoped or aggregate-only contexts and stopped
would have occurred if any required context failed, remained pending, or stayed
ambiguous.

## Source / Script Mutation Boundary

No qsl-protocol source, script, workflow, dependency, or lockfile mutation was
introduced. qsl-server source was not mutated and no qsl-server PR was opened.

## Workflow Mutation Boundary

No workflow files were changed. No workflow dispatch occurred. No workflow rerun
occurred.

## Runtime / qsc Boundary

No qsc send/receive command ran. No qsc E2EE command ran. qsc was inspected only
as a coarse remote binary existence/executable class.

## qsl-server / qsl-attachments Boundary

qsl-server source was acquired read-only under the proof root, validated, built,
and staged remotely under the qslcodex test workspace. qsl-server was not
started because the expected bind target was unavailable.

No qsl-attachments command, clone, build, run, or mutation occurred.

## Remote-Action Boundary

Remote action was limited to the exact authorized SSH readiness command, remote
inventory via SSH stdin, binary scp to the authorized tmp path, remote stage via
SSH stdin, and remote postcheck via SSH stdin.

No sudo, systemctl, service, journalctl, Tailscale, firewall, account, shell, or
authorized_keys mutation occurred. No writes occurred outside the qslcodex test
workspace.

## Public-Site / Cloudflare Boundary

No README public progress, docs/public, public, website, Cloudflare, or public
deployment mutation occurred.

## Raw Output Boundary

Raw outputs and manifests remain proof-root-only. Repository docs contain only
coarse classes, short commit identifiers, and redacted summaries.

## Claim Boundary

No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No vulnerability-free claim is made.
No bug-free claim is made.
No perfect-build claim is made.

## Validation

Pre-PR validation is expected to include scope guard, marker proof, link-check,
private-material scan, overclaim scan, docs/governance classifier, PR body
preflight, goal-lint if available, cargo audits, cargo metadata, cargo fmt, and
qsc adversarial shell syntax checks.

Focused qsc runtime tests are skipped because NA-0573 made no qsc source,
runtime, dependency, workflow, or send/receive mutation.

## Recommendation

Proceed to an operator-proof authorization lane for the private loopback bind
target or safe start command. Do not start qsl-server, run qsc send/receive,
dispatch workflows, or claim relay recovery until that proof exists and a later
directive authorizes the exact action.
