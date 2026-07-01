Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0581 Remote qsl-server Expected Bind Failure Remediation Harness

## Executive Summary

NA-0581 consumed D-1151 and D-1152, verified fresh qwork proof from
`2026-07-01T05:12:12Z`, verified current main at `d7d8f98f336a`, and applied
bounded Codex operational authority with the D510 authorized_keys read-only
private-inspection exception.

Remote inspection classified the expected bind as Codex-start-safe from a single
loopback-only authorized_keys forwarding class. The bounded start initially
skipped because of a proof-root harness timestamp bug; failure-cause review
classified that as recoverable, patched the generated script, and retried once.
The retry started an NA-0581-owned qsl-server process. No-secret/no-body
postcheck classified relay testing ready.

Result classification: `QSL_SERVER_EXPECTED_BIND_REMEDIATION_RELAY_TESTING_READY`.

Selected successor: `NA-0582 -- QSL Remote Relay Recovered Test Verification Harness`.

## qwork Proof Verification

- qwork proof files were copied before fetch, repository mutation, SSH, remote
  script generation, GitHub metadata review, or proof publication.
- Required qwork values matched lane `NA-0581`, repo `qsl-protocol`, path
  `/srv/qbuild/work/NA-0581/qsl-protocol`, clean worktree/index/untracked state,
  READY_COUNT 1, queue top READY `NA-0581`, shared cargo target mode, and shared
  target ready.
- qwork proof timestamp was verified at or after `2026-07-01T05:12:12Z`.
- Live pre-fetch `HEAD` and `origin/main` matched qwork proof at
  `d7d8f98f336a`.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1151 / D-1152 Inheritance

- D-1151 exists once and is Accepted.
- D-1152 exists once and is Accepted.
- NA-0580 is DONE.
- NA-0581 was the sole READY item before implementation.
- D-1151 result classification was
  `QSL_SERVER_EXPECTED_BIND_ENDPOINT_VALUE_UNAVAILABLE`.
- D-1151 remote inspection classified endpoint value unavailable.
- D-1151 did not start qsl-server, run qsc, dispatch workflows, or use
  qsl-attachments.
- D-1152 restored NA-0581.
- D-1153 was absent before this implementation patch.

## Authority Model Application

- Tier 1 redacted diagnostics were limited to host label `inspiron` and
  workspace `/home/qslcodex/qsl-remote-test/`.
- Tier 2 expected-bind bounded start was conditional on no-secret, non-root,
  loopback-only, no sudo/systemd/service/firewall/Tailscale/account/shell/
  authorized_keys mutation, no qsc send/receive, no workflow dispatch/rerun, no
  qsl-attachments, no private value publication, and writes only under
  `/home/qslcodex/qsl-remote-test/`.
- Tier 3 operator/admin action remained forbidden.
- Tier 4 forbidden action remained forbidden.
- Continuous CI wait-work and read-only forward-audit policy applies to PR and
  post-merge waits.

## authorized_keys Read-Only Private-Inspection Exception

- The inspection script read `/home/qslcodex/.ssh/authorized_keys` once, read
  only, through SSH stdin.
- The parser emitted only coarse forwarding classes.
- authorized_keys content disclosed: no.
- public key material disclosed: no.
- forwarding restriction present class: yes.
- forwarding candidate count class: single.
- forwarding candidate loopback-only class: yes.
- forwarding candidate ambiguous class: no.
- No authorized_keys mutation occurred.

## Automatic Failure-Cause Investigation Policy

Automatic failure-cause investigation was applied after the first bounded start
attempt skipped unexpectedly. The generated start script parsed a UTC safe
manifest timestamp using local-time conversion, causing a fresh manifest to be
treated as stale/not safe without creating NA-0581-owned state.

Failure-cause classification:
`QSL_SERVER_EXPECTED_BIND_REMEDIATION_FAILURE_HARNESS_BUG_FIXED_AND_RETRIED`.

Corrective action: patched the proof-root generated start script to use UTC
timestamp conversion, recompiled it, preserved attempt1 evidence, and retried
the bounded start once. Final result: start succeeded and postcheck classified
relay testing ready.

## Current Main Required-Check Classification

- Current main: `d7d8f98f336a`.
- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: completed success.
- No failed visible check-run was classified.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

Recovered command-shape issue: the first check-runs API call used a POST-shaped
`gh api` invocation. It was rerun with explicit GET and completed successfully.

## qsl-server CLI / Route Review

- qsl-server source path reviewed read-only: existing qsl-server checkout at
  public source head `6bf61d439fa`.
- `--bind` is host/address only.
- `--port` is separate.
- qsl-server internally composes bind plus port before listener bind.
- Startup can use a sanitized no-bearer environment because `RELAY_TOKEN` is
  optional and absent/empty disables bearer auth.
- Canonical route shape is `POST /v1/push` and `GET /v1/pull?max=N`.
- Route usage requires `X-QSL-Route-Token`.

## qsl-protocol / qsc Relay Expectation Review

- Remote handshake and remote relay workflows both use `RELAY_URL` and
  `RELAY_TOKEN` secret-name boundaries.
- qsc push path class: `/v1/push`.
- qsc pull path class: `/v1/pull?max=N`.
- Route header class: `X-QSL-Route-Token`.
- Optional bearer class: Authorization bearer when relay token is available;
  value unavailable and not accessed.
- Expected endpoint value remains secret/private and unavailable to Codex.
- Expected bind can be tied to operator-created forwarding configuration only
  through coarse forwarding classes without publishing values.

## GitHub Metadata Review

- Workflow metadata reviewed for `remote-handshake-tests` and
  `remote-relay-tests`.
- Repository secret names observed without values: `RELAY_TOKEN`, `RELAY_URL`.
- No repository variables were reported.
- No secret or variable values were accessed or disclosed.

## Remote Script Design and Static Review

- Four proof-root-only Python stdlib scripts were generated for inspection,
  start, postcheck, and cleanup.
- Syntax review passed after the UTC timestamp fix.
- JSON-only stdout: passed.
- `shell=True`: absent.
- authorized_keys read reference exists only in the inspection script.
- No sudo/systemctl/service/journalctl/Tailscale/firewall command execution.
- No broad home-directory scan; workspace walk is limited to
  `/home/qslcodex/qsl-remote-test/`.
- Writes are limited to `/home/qslcodex/qsl-remote-test/` on the remote host.
- Start script uses `--bind <host-only> --port <port>` internally and never
  prints bind or port values.
- Cleanup only uses NA-0581-owned manifest state.

## SSH Readiness

Classification: `SSH_QSL_SERVER_EXPECTED_BIND_REMEDIATION_READY`.

Recovered classifier issue: the single authorized SSH readiness command exited
0 and emitted the directive-form sentinel with a literal trailing marker
character. The classifier accepted the sentinel without rerunning SSH.

## Remote Expected-Bind Remediation Inspection

Classification:
`QSL_SERVER_EXPECTED_BIND_REMEDIATION_CODEX_START_SAFE`.

- staged binary present class: present.
- staged binary executable class: yes.
- qsl-server version/help available class: available.
- qsl-server CLI shape class: host-only bind plus separate port.
- expected bind available class: yes.
- expected bind source class: authorized_keys forwarding restriction.
- expected bind loopback-only class: yes.
- expected bind listener already ready class: no.
- inspection safe manifest written class: yes.
- endpoint value disclosed: no.
- private topology disclosed: no.

## authorized_keys Forwarding Class Review

- authorized_keys read class: readable.
- forwarding restriction present class: yes.
- forwarding candidate count class: single.
- forwarding candidate loopback-only class: yes.
- forwarding candidate ambiguous class: no.
- authorized_keys content disclosed: no.
- public key material disclosed: no.

## Expected-Bind Bounded Start

Final classification:
`QSL_SERVER_EXPECTED_BIND_REMEDIATION_START_STARTED`.

The first attempt classified
`QSL_SERVER_EXPECTED_BIND_REMEDIATION_START_SKIPPED_NOT_SAFE` because of the
generated-script UTC timestamp bug described in the failure-cause section. It
created no NA-0581-owned state. After the proof-root harness fix and one retry,
the bounded start created NA-0581-owned state and emitted only coarse classes.

## Expected-Bind Postcheck

Classification:
`QSL_SERVER_EXPECTED_BIND_REMEDIATION_POSTCHECK_RELAY_TESTING_READY`.

- expected bind listener ready class: yes.
- push route shape class: canonical push route present with no-body rejection.
- pull route shape class: canonical pull route present.
- expected bind relay testing ready class: yes.
- endpoint alignment class: expected bind aligned.
- start manifest present class: yes.
- process owned by NA-0581 class: yes.
- response body disclosed: no.

## Cleanup / Rollback

Classification: `QSL_SERVER_EXPECTED_BIND_REMEDIATION_CLEANUP_NOT_NEEDED`.

Cleanup was not run because postcheck proved relay testing ready, the process is
NA-0581-owned, and the selected successor is recovered-test verification.

## Failure-Cause Investigation

Classification:
`QSL_SERVER_EXPECTED_BIND_REMEDIATION_FAILURE_HARNESS_BUG_FIXED_AND_RETRIED`.

The only failure was the recoverable proof-root start-harness timestamp bug.
No qsl-server source mismatch, qsc relay mismatch, permission/workspace issue,
bind/listen failure, private-material stop, service-owner action, operator
action, or ambiguous remote state was found after the retry.

## Private-Material Review

Aggregate private-material review passed after a recovered scanner false
positive for generated-script `bind_port` field names. The scanner continued to
flag actual values in raw outputs and docs.

No endpoint values, private port values, route-token/capability values, bearer
values, Authorization values, private topology, process identity, command line,
payloads, response bodies, authorized_keys content, public key material, private
key material, secret environment values, Cloudflare tokens, API keys, or private
material were published.

## Result Classification

`QSL_SERVER_EXPECTED_BIND_REMEDIATION_RELAY_TESTING_READY`.

## Selected Successor

`NA-0582 -- QSL Remote Relay Recovered Test Verification Harness`.

Successor objective:
Verify whether the recovered `inspiron` qsl-server expected-bind setup resolves
the previously failing remote-handshake and remote-relay checks. Codex may run
only exact D-1153-authorized verification actions, which may include redacted
qsl-server postcheck, read-only GitHub metadata review, and exact workflow
dispatch/rerun only if D-1153 explicitly authorizes that action. Raw logs and
artifacts must remain proof-root-only. Repository docs may publish only coarse
classifications, run IDs, check names, and redacted summaries. Codex must not
mutate remote accounts, services, Tailscale, authorized_keys, qsl-server source,
qsl-attachments, qsc runtime/source, workflows, dependencies, public-site
content, or Cloudflare configuration.

## Required-Check Boundary

NA-0581 did not weaken required checks, public-safety, advisories, goal-lint,
CodeQL, suite2-vectors, or branch-protection posture.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, or lockfile was
mutated. Only governance/evidence/testplan/traceability/journal files are
changed by this implementation patch.

## Workflow Mutation Boundary

No workflow file was mutated. No workflow dispatch or rerun occurred.

## Runtime / qsc Boundary

No qsc send/receive, E2EE, runtime mutation, source mutation, or dependency
mutation occurred.

## qsl-server / qsl-attachments Boundary

No qsl-server source mutation or qsl-server PR occurred. The bounded start used
the already staged remote qsl-server binary. No qsl-attachments command, clone,
build, run, or mutation occurred.

## Remote-Action Boundary

Remote actions were limited to the exact SSH readiness, inspection, start, and
postcheck commands. Cleanup was not run because it was not needed. No scp, sudo,
systemctl, service, journalctl, ps, ss, netstat, lsof, Tailscale, firewall,
account/shell mutation, authorized_keys mutation, root-owned path mutation, or
write outside `/home/qslcodex/qsl-remote-test/` occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Raw Output Boundary

Raw SSH stdout/stderr, generated remote scripts, raw qsl-server help/version
output, safe manifests, process manifests, and route-probe artifacts remain
proof-root-only or remote-workspace-only. Repository evidence publishes only
coarse classifications.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No vulnerability-free claim is made. No bug-free claim is made. No
perfect-build claim is made. No perfect-crypto claim is made.

## Validation

- qwork proof verification: PASS.
- Queue/decision proof before patch: PASS.
- Current main required-check classification: PASS.
- qsl-server CLI/route review: PASS.
- qsl-protocol/qsc relay expectation review: PASS.
- GitHub metadata no secret values: PASS.
- Remote script static review: PASS.
- SSH readiness: READY.
- Remote inspection: CODEX_START_SAFE.
- Bounded start: START_STARTED after one recovered harness retry.
- Postcheck: RELAY_TESTING_READY.
- Cleanup: NOT_NEEDED.
- Aggregate private-material review: PASS.

## Recommendation

Merge NA-0581 after required checks pass, then close out to the selected
NA-0582 recovered-test verification successor if post-merge gates remain green.
