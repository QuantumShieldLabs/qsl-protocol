Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0559 Remote Relay Targeted Non-Secret Operator Probe Capture Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0559 is authorization-only. It consumes D-1106 and D-1107, verifies fresh
qwork proof from `2026-06-28T19:38:46Z`, classifies current main required
checks, consumes the operator request for Codex-executed probing, and pivots the
next lane from an operator-run probe capture model to a Codex-executed,
non-secret probe harness under strict redaction and stop rules.

Result classification:
`REMOTE_RELAY_CODEX_EXECUTED_NON_SECRET_PROBE_AUTH_READY`.

Selected successor:
`NA-0560 -- QSL Remote Relay Codex-Executed Non-Secret Probe Harness`.

No probe was executed. No SSH, Tailscale, remote command, workflow dispatch,
rerun, local reproduction, qsc send/receive, qsc E2EE, source mutation, script
mutation, workflow mutation, dependency or lockfile mutation, qsl-server or
qsl-attachments action, public-site mutation, Cloudflare mutation, secret value
request, or private-material publication occurred.

## qwork Proof Verification

Codex copied the qwork proof files from the NA-0559 lane workspace into the
proof root and parsed the `.kv`, JSON, and cargo-target env files using a
file-backed parser. The parser verified:

- lane `NA-0559`;
- repo `qsl-protocol`;
- branch `main`;
- path `/srv/qbuild/work/NA-0559/qsl-protocol`;
- startup HEAD, origin/main, and main all at `2ae25ad37c6e`;
- clean worktree, index, and untracked proof;
- READY_COUNT 1 with queue top READY `NA-0559`;
- qwork version
  `175b0ea1d5b9abc07bdab66e9b92446e2a3d533018468e94a95c26f8698f86cf`;
- proof timestamp `2026-06-28T19:38:46Z`;
- shared Cargo target mode and expected shared target directory.

Pre-fetch live HEAD and origin/main matched the qwork proof. Root disk usage
was below the 95 percent stop threshold and `/backup/qsl` was mounted. Codex did
not run qwork, qstart, or qresume.

## D-1106 / D-1107 Inheritance

D-1106 exists once, is Accepted, and records the NA-0558 result classification
`REMOTE_RELAY_TARGETED_NON_SECRET_OPERATOR_PROBE_CAPTURE_READY`.

D-1107 exists once, is Accepted, marks NA-0558 DONE, and restores NA-0559 as
the exactly one READY successor. D-1107 records that no NA-0559 implementation
occurred during closeout.

Inherited boundaries remain binding: no secret values, route-token/capability
values, bearer values, Authorization headers, private endpoint values, private
topology, payloads, response bodies, or secret environment values may be
requested or published.

## Operator Request and Authority Pivot

The operator asked for Codex to perform the targeted non-secret probe work
because manual probing is consuming excessive time.

Operator context consumed:

- the remote test/relay/server host label is `inspiron`;
- the user label is `qslcodex`;
- the stated access form is `ssh inspiron`;
- shell command execution may not currently be configured and could require
  operator-side correction.

NA-0559 does not authorize Codex to fix shell access, mutate `qslcodex`, SSH to
`inspiron`, use Tailscale, or run probes. NA-0559 only records the authority
pivot and selects an exact NA-0560 successor that may perform bounded
Codex-executed, non-secret probes if the stop rules remain satisfied.

## Current Main Required-Check Classification

Current main was verified at `2ae25ad37c6e`, equal to origin/main.

GitHub REST metadata classified current main as healthy:

- public-safety completed success;
- advisories completed success;
- suite2-vectors completed success;
- no failed required checks;
- no required pending checks;
- branch-protection required contexts classified;
- associated PR-head proof satisfied PR-scoped `goal-lint` and `CodeQL`
  contexts;
- no `Cargo.toml`, root `Cargo.lock`, or qsc fuzz `Cargo.lock` drift;
- root disk usage below the stop threshold;
- `/backup/qsl` mounted.

## Prior Evidence and Gap Review

NA-0555 diagnostic evidence still governs this boundary:

- remote-handshake result:
  `REMOTE_HANDSHAKE_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`;
- remote-relay result: `REMOTE_RELAY_DIAGNOSTIC_NETWORK_TLS_TIMEOUT`;
- both reached endpoint label `relay_push_v1`;
- both emitted qsc error `relay_inbox_push_failed`;
- route-token header present true for both;
- bearer auth present true for both;
- HTTP status/body unknown for both.

NA-0557 operator proof was safe but insufficient:

- endpoint configuration `ENDPOINT_CONFIGURED_UNKNOWN`;
- DNS `DNS_RESOLUTION_NOT_CHECKED`;
- TCP `TCP_CONNECT_NOT_CHECKED`;
- TLS `TLS_HANDSHAKE_NOT_CHECKED`;
- service health `RELAY_SERVICE_HEALTH_NOT_CHECKED`;
- auth/route configuration `AUTH_ROUTE_CONFIGURED_UNKNOWN`;
- runner-specific proof `GITHUB_RUNNER_PROOF_NOT_PERFORMED`.

The evidence does not prove script ownership, qsc runtime ownership, relay API
shape mismatch, or environment/secret correctness. NA-0558 designed
operator-run proof commands, but the operator now requests Codex-executed
probing rather than operator-run probing.

## Codex-Executed Probe Authority Design

NA-0560 may run Codex-executed non-secret probes only from these origins:

- `LOCAL_QBUILD_HOST`;
- `INSPIRON_VIA_SSH_IF_NO_REMOTE_MUTATION_REQUIRED`;
- `GITHUB_METADATA_READ_ONLY`.

NA-0560 is not authorized to dispatch workflows, rerun historical jobs, perform
qsc send/receive, perform qsc E2EE, push POST payloads, mutate remote services,
mutate accounts or shells, mutate Tailscale configuration, or read secret files
for publication.

NA-0560 may attempt exactly one non-mutating SSH command-execution check:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 inspiron 'printf QSL_REMOTE_PROBE_READY\n'
```

If that fails because shell command execution is unavailable, NA-0560 must stop
with `REMOTE_PROBE_ACCESS_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`. NA-0560 must
not fix shell access, modify `qslcodex`, run sudo, mutate `inspiron`, or alter
remote network/service state.

If SSH command execution is available, NA-0560 may run a proof-root-generated,
non-mutating redacted probe script remotely only if the script prints JSON with
coarse classes, never prints private material, and the output passes
private-material scan before summary use.

## Exact NA-0560 Command Allowlist

Allowed future command families:

1. Startup proof and current-main checks.
2. Local qbuild host non-secret preflight, with no network probe unless a
   redacted endpoint value is available internally without being printed.
3. The exact SSH reachability command shown above.
4. Remote redacted probe script execution only after SSH reachability succeeds,
   with the exact command recorded in NA-0560 before execution.
5. GitHub secret/variable metadata names and updated-at fields only, with no
   values and no mutation.
6. Current-main workflow/read-only metadata, with no dispatch and no rerun.

Forbidden without a later directive: fixing `qslcodex` shell, sudo, account or
login-shell mutation, private-key reads, token-file publication, service
mutation, firewall/Tailscale mutation, workflow dispatch, qsc send/receive, qsc
E2EE, and qsl-server/qsl-attachments mutation.

NA-0560 result classifications must include:

- `CODEX_PROBE_DNS_TCP_TLS_CLASSIFIED`;
- `CODEX_PROBE_SERVICE_HEALTH_CLASSIFIED`;
- `CODEX_PROBE_INSPIRON_ACCESS_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`;
- `CODEX_PROBE_GITHUB_RUNNER_SPECIFIC_PROOF_REQUIRED`;
- `CODEX_PROBE_REMOTE_SERVICE_OWNER_PROOF_REQUIRED`;
- `CODEX_PROBE_ENVIRONMENT_FIX_READY`;
- `CODEX_PROBE_QSC_RUNTIME_REVIEW_READY`;
- `CODEX_PROBE_PRIVATE_MATERIAL_STOP`;
- `CODEX_PROBE_AMBIGUOUS_STOP`.

## NA-0560 Private-Material Policy

NA-0560 must scan all output before it enters repository docs or a final
response.

Forbidden in repository docs and final response:

- endpoint values;
- URLs;
- private hosts;
- private IPs;
- private topology;
- route-token/capability values;
- bearer values;
- Authorization headers;
- payloads;
- response bodies;
- secret environment values;
- private keys;
- SSH private material;
- Cloudflare tokens;
- API keys.

Allowed fields are coarse classes only: probe origin, endpoint configured
yes/no/unknown, endpoint value disclosed no, DNS/TCP/TLS/service-health class,
secret and route presence yes/no/unknown, body disclosed no, raw-output
private-material yes/no, redaction review pass/fail, no-secret assertion, and
SSH reachability class.

NA-0560 must stop if an endpoint value, token/bearer value, Authorization
header, private IP/host/topology, payload/body, or need for `qslcodex` account
mutation appears.

## Option Review

Option A, Codex-executed non-secret probe authority, is selected because exact
origins, a single SSH reachability command, remote-script constraints,
private-material policy, and stop rules are definable.

Option B, keep the operator-run NA-0559 model, is not selected for the next lane
because the operator explicitly requested Codex execution. Option C, authorize
shell/account fixing now, is rejected as out of scope and potentially
state-mutating. Option D, authorize workflow dispatch or rerun, is rejected
because the current pivot can classify host-side access first without altering
CI state. Option E, stop/ambiguous, is not selected because the NA-0560
authority can be bounded safely.

## Result Classification

`REMOTE_RELAY_CODEX_EXECUTED_NON_SECRET_PROBE_AUTH_READY`.

## Selected Successor

`NA-0560 -- QSL Remote Relay Codex-Executed Non-Secret Probe Harness`.

The successor must execute only the D-1108-authorized Codex-run non-secret
probe plan, classify the `relay_inbox_push_failed` network/TLS/timeout
boundary with coarse classes, preserve the private-material policy, and select
an exact next environment fix, service-owner proof, GitHub-runner proof, qsc
runtime review, or stop successor.

## Required-Check Boundary

Current-main required checks were classified before mutation. NA-0559 executes
no workflow dispatch and no rerun.

## Source / Script Mutation Boundary

No qsc source, qsc tests, qsc fuzz files, Cargo files, demo scripts, or other
source/script paths are mutated by NA-0559.

## Workflow Mutation Boundary

No workflow file is changed. No workflow dispatch, rerun, cancel, or delete
occurs.

## Runtime / qsc / Dependency Boundary

No local qsc send/receive, local qsc E2EE, qsc runtime reproduction, dependency
update, manifest update, or lockfile update occurs.

## qsl-server / qsl-attachments Boundary

No qsl-server or qsl-attachments command, clone, build, run, local use, or
mutation occurs.

## Remote-Action Boundary

No SSH, scp, sftp, rsync, Tailscale, ping, nc, private endpoint curl, private
endpoint TLS probe, remote command, sudo/admin action, systemctl action, backup
command, or remote-service mutation occurs in NA-0559.

## Public-Site / Cloudflare Boundary

No README public-progress content, docs/public content, website path, public
path, public-site content, deployment setting, or Cloudflare configuration is
changed.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No backup/restore-complete claim is made. No vulnerability-free claim is
made. No bug-free claim is made. No perfect-build or perfect-crypto claim is
made.

## Validation

Validation covers qwork proof, queue proof, D-1106/D-1107 inheritance,
current-main required-check classification, operator request review, prior
evidence gap review, Codex-executed authority design, exact command allowlist,
private-material policy, result classification, successor selection, marker
proof, changed Markdown link-check, private-material scan, prohibited-material
scan, overclaim scan, docs/governance-only classifier, PR body preflight,
goal-lint, cargo audits, cargo fmt, and qsc-adversarial shell syntax.

Focused qsc runtime tests are intentionally skipped because this lane is
authorization-only and makes no qsc source/runtime/dependency/workflow mutation.

## Recommendation

Merge NA-0559 only if required checks are green, then close it out to NA-0560
only if post-merge public-safety and advisories remain successful and the exact
successor block can be restored without placeholders. NA-0560 should then run
the bounded Codex-executed non-secret probe plan; NA-0559 itself must not run
probes.
