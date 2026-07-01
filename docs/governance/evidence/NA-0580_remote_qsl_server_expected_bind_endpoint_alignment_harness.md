Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0580 Remote qsl-server Expected Bind / Endpoint Alignment Harness

## Executive Summary

NA-0580 consumed D-1149 and D-1150, verified fresh qwork proof from `2026-07-01T03:33:39Z`, verified current main at `4ba0bb4988c3`, and applied the D-1142/D-1143 bounded Codex operational authority model.

qsl-server source/help and qsc relay expectation review reconfirmed the compatible route boundary: qsl-server uses host-only `--bind` plus separate `--port`, internally composes bind and port, and serves canonical `/v1/push` and `/v1/pull` routes using the `X-QSL-Route-Token` header. qsc expects `/v1/push` and `/v1/pull?max=N`, with the same route header and optional bearer auth if a relay token is available.

Remote inspection reached the staged qsl-server binary and confirmed help/version/CLI shape classes, but could not derive the expected bind or endpoint alignment from non-secret qslcodex workspace manifests or labels. GitHub metadata review exposed only secret names for `RELAY_URL` and `RELAY_TOKEN`; no secret values were accessed.

Result classification: `QSL_SERVER_EXPECTED_BIND_ENDPOINT_VALUE_UNAVAILABLE`.

Selected successor: `NA-0581 -- QSL Remote qsl-server Expected Bind Failure Remediation Harness`.

## qwork Proof Verification

- qwork proof files were copied before fetch, repository mutation, SSH, remote script generation, GitHub metadata review, or proof publication.
- Required qwork values matched lane `NA-0580`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0580/qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1, queue top READY `NA-0580`, shared cargo target mode, and shared target ready.
- qwork proof timestamp was verified at or after `2026-07-01T03:33:39Z`.
- Live pre-fetch `HEAD` and `origin/main` matched qwork proof at `4ba0bb4988c3`.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1149 / D-1150 Inheritance

- D-1149 exists once and is Accepted.
- D-1150 exists once and is Accepted.
- NA-0579 is DONE.
- NA-0580 was the sole READY item before implementation.
- D-1149 result classification was `QSL_SERVER_PROOF_COMPLETION_TEMP_LOOPBACK_ROUTE_SHAPE_PASS_EXPECTED_BIND_REQUIRED`.
- D-1149 proved qsl-server can start and answer route-shape probes on a corrected temporary loopback bind.
- D-1149 did not prove expected bind or endpoint alignment.
- D-1150 restored NA-0580.
- D-1151 was absent before this implementation patch.

## Authority Model Application

- Tier 1 redacted diagnostics were limited to host label `inspiron` and workspace `/home/qslcodex/qsl-remote-test/`.
- Tier 2 expected-bind bounded start was conditional on no-secret, non-root, loopback-only, no sudo/systemd/service/firewall/Tailscale/account/shell/authorized_keys mutation, no qsc send/receive, no workflow dispatch/rerun, no qsl-attachments, no private value publication, and writes only under `/home/qslcodex/qsl-remote-test/`.
- Tier 2 start was not authorized because inspection returned `expected_bind_codex_start_safe=no`.
- Tier 3 operator/admin action remained forbidden.
- Tier 4 forbidden action remained forbidden.
- Continuous CI wait-work and read-only forward-audit policy applies to PR and post-merge waits.

## Automatic Failure-Cause Investigation Policy

The automatic failure-cause policy was applied after inspection classified endpoint value unavailable and start was skipped. The generated scripts, static review, qsl-server source/help assumptions, and qsc workflow/transport expectations were compared. No harness defect or source mismatch was found. No retry was allowed because further progress would require private endpoint/secret values or an operator-provided non-secret expected-bind manifest.

Failure-cause classification: `QSL_SERVER_EXPECTED_BIND_FAILURE_ENDPOINT_VALUE_UNAVAILABLE`.

## Current Main Required-Check Classification

- Current main: `4ba0bb4988c3`.
- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: completed success.
- No failed visible check-run was classified.
- D498 visibility recovery was applied only for `goal-lint` and `CodeQL`, both proven success on the PR-head check-runs for PR #1433.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success.
- `cargo metadata --locked --format-version=1`: success.
- `cargo fmt --check`: success.
- `sh -n scripts/ci/qsc_adversarial.sh`: success.
- `bash -n scripts/ci/qsc_adversarial.sh`: success.
- Cargo manifest/lock drift: absent.

## qsl-server CLI / Route Review

- qsl-server source path reviewed read-only: existing qsl-server checkout at public source head `6bf61d439fa`.
- `--bind` is host/address only.
- `--port` is separate.
- qsl-server internally composes bind plus port before listener bind.
- Startup can use a sanitized no-bearer environment because `RELAY_TOKEN` is optional.
- Canonical route shape is `POST /v1/push` and `GET /v1/pull?max=N`.
- Route usage requires `X-QSL-Route-Token`.
- Local qsl-server binary was not present, so no qsl-server build was performed; staged remote binary help/version was captured by inspection proof-root-only.

## qsl-protocol / qsc Relay Expectation Review

- qsc relay endpoint source class: `qsl_protocol_workflow_metadata`.
- Workflow secret name source class: `github_secret_metadata_names_only`.
- Remote handshake and remote relay workflows both use `RELAY_URL` and `RELAY_TOKEN`.
- Endpoint value is secret/private and unavailable to Codex.
- qsc push path class: `/v1/push`.
- qsc pull path class: `/v1/pull?max=N`.
- Route header class: `X-QSL-Route-Token`.
- Optional bearer class: Authorization bearer, value unavailable and not accessed.
- Expected bind could not be inferred safely without private endpoint/secret values or a non-secret workspace manifest.

## GitHub Metadata Review

- Workflow metadata reviewed for `remote-handshake-tests` and `remote-relay-tests`.
- Repository secret names observed without values: `RELAY_TOKEN`, `RELAY_URL`.
- Repository variables list was empty.
- No secret or variable values were accessed.
- No workflow dispatch or rerun occurred.

## Remote Script Design and Static Review

Four proof-root-only Python stdlib scripts were generated and static-reviewed:

- `qsl_server_expected_bind_alignment_inspect.py`
- `qsl_server_expected_bind_bounded_start.py`
- `qsl_server_expected_bind_postcheck.py`
- `qsl_server_expected_bind_cleanup.py`

Static review passed for syntax, JSON-only stdout, no `shell=True`, no forbidden subprocess targets, no forbidden path literals, workspace-bounded writes, no qsc send/receive, no qsl-attachments action, no raw endpoint/bind/process/command publication, deterministic manifest gates, start argv shape `--bind <host-only> --port <port>`, and cleanup limited to NA-0580-owned state.

Recovered proof issue: the first static reviewer flagged directive-required JSON field names containing words such as service/sudo/Tailscale. This was a recoverable scanner false positive. Corrective action: reran AST-based review with required field-name allowlisting and actual subprocess/path checks. Final result: PASS.

Recovered proof issue: validation overclaim scanning matched required negative-boundary sentence fragments embedded in long table rows. This was a recoverable scanner false positive. Corrective action: reran overclaim scan with sentence-fragment negative-boundary handling. Final result: PASS.

Recovered proof issue: a goal-lint discovery `rg` command included optional package files that do not exist. This was a recoverable command-shape/discovery issue. Corrective action: reran targeted discovery against existing scripts/workflow paths and found the local goal-lint path. Final result: PASS.

## SSH Readiness

Classification: `SSH_QSL_SERVER_EXPECTED_BIND_ALIGNMENT_READY`.

The authorized SSH readiness command ran exactly once. A strict newline classifier mismatch was recovered without rerunning SSH by checking return code, expected sentinel prefix, empty stderr, and private-material scan pass.

## Remote Expected-Bind / Endpoint Alignment Inspection

Classification: `QSL_SERVER_EXPECTED_BIND_ALIGNMENT_ENDPOINT_VALUE_UNAVAILABLE`.

Remote inspection confirmed:

- staged qsl-server binary present/executable;
- qsl-server version available;
- qsl-server help available;
- qsl-server CLI shape `bind_host_only_port_separate`;
- expected bind source unavailable from non-secret workspace manifests/labels;
- endpoint alignment source class `github_secret_metadata_names_only`;
- endpoint alignment known class `unknown`;
- endpoint alignment Codex-safe class `no`;
- `expected_bind_codex_start_safe=no`;
- `secret_or_endpoint_action_required_class=yes`;
- no expected bind, endpoint, private topology, token, Authorization, payload/body, process identity, command line, or key material was disclosed.

## Expected-Bind Bounded Start

Classification: `QSL_SERVER_EXPECTED_BIND_BOUNDED_START_SKIPPED_NOT_SAFE`.

The bounded start SSH command was not run because inspection did not write an expected-bind-safe manifest and returned `expected_bind_codex_start_safe=no`.

## Expected-Bind Postcheck

Classification: `QSL_SERVER_EXPECTED_BIND_POSTCHECK_ENDPOINT_ALIGNMENT_UNPROVEN`.

The postcheck SSH command was not run because start was skipped and inspection did not classify an expected listener already ready.

## Cleanup / Rollback

Classification: `QSL_SERVER_EXPECTED_BIND_CLEANUP_NOT_NEEDED`.

The cleanup SSH command was not run because no NA-0580-owned expected-bind process or state was started.

## Failure-Cause Investigation

Classification: `QSL_SERVER_EXPECTED_BIND_FAILURE_ENDPOINT_VALUE_UNAVAILABLE`.

The failure-cause review found:

- no generated harness defect after static review;
- no qsl-server source/help mismatch;
- no qsc relay expectation mismatch;
- no permission/workspace or bind/listen failure;
- no private-material stop;
- endpoint alignment remains unavailable from secret metadata names only.

No retry was allowed because retrying would require private endpoint/secret values, operator/service action, qsc send/receive, workflow dispatch, or out-of-scope mutation.

## Private-Material Review

Aggregate private-material review passed for raw SSH outputs, parsed JSON, generated scripts, source-review artifacts, workflow metadata artifacts, added docs, and response draft inputs available at implementation time.

No endpoint value was published. No private port value was published. No route-token/capability value was published. No bearer or Authorization value was published. No private topology was published. No process identity was published. No command line was published. No payload or response body was published. No authorized_keys content or key material was published.

## Result Classification

`QSL_SERVER_EXPECTED_BIND_ENDPOINT_VALUE_UNAVAILABLE`

The staged qsl-server binary and CLI shape are present, but expected bind / endpoint alignment could not be derived without private endpoint/secret values or an operator-provided non-secret expected-bind source.

## Selected Successor

`NA-0581 -- QSL Remote qsl-server Expected Bind Failure Remediation Harness`

## Required-Check Boundary

Required-check handling was read-only. No workflow dispatch or rerun occurred. No failed required check was classified.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, lockfile, qsc runtime, qsl-server source, qsl-server PR, or qsl-attachments source mutation occurred.

## Workflow Mutation Boundary

No workflow file changed. No workflow dispatch or rerun occurred.

## Runtime / qsc Boundary

No qsc command was run. No qsc send/receive or E2EE action occurred.

## qsl-server / qsl-attachments Boundary

qsl-server source was reviewed read-only and was not mutated. qsl-server was not deployed. qsl-attachments was not run, cloned, built, or mutated.

## Remote-Action Boundary

Remote action was limited to the authorized SSH readiness command and authorized SSH stdin inspection command. No bounded start, postcheck, or cleanup SSH command was run. No scp, sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys, qsl-backup, or root-owned path action occurred. No writes outside `/home/qslcodex/qsl-remote-test/` were reported by the scripts.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare mutation occurred.

## Raw Output Boundary

Raw outputs, generated scripts, parsed remote JSON, qsl-server help/version raw output, metadata review artifacts, and private scans remain proof-root-only. Tracked repository evidence contains only coarse classifications and safe boundary statements.

## Claim Boundary

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-build claim is made. No perfect-crypto claim is made.

## Validation

Validation includes qwork proof, queue/decision proof, current-main checks, D-1149/D-1150 inheritance, authority confirmation, qsl-server CLI/route review, qsc relay expectation review, GitHub metadata review without values, remote script static review, SSH readiness scan, remote inspection scan, start/postcheck/cleanup skip scans, failure-cause review, aggregate private-material review, result/successor classification, scope guard, marker proof, link-check, private-material scan, overclaim scan, PR body preflight, goal-lint, cargo audits, locked metadata, formatting, and shell syntax checks.

Recovered validation issue: nested qsc fuzz cargo audit initially used a non-existent `qsc/fuzz` path. This was a recoverable command-shape/path issue. Corrective action: reran root audit with deny warnings and reran nested audit against `qsl/qsl-client/qsc/fuzz/Cargo.lock`. Final result: PASS.

Recovered staging issue: a normal `git add` refused the in-scope evidence file because `docs/governance/evidence/**` is ignored. This was a recoverable staging command-shape issue. Corrective action: force-added only the allowed NA-0580 evidence file and staged other tracked files normally. Final result: PASS.

Recovered scan issue: final added-line private-material scan matched the safe negative-boundary phrase `bearer Authorization values were not accessed` as if it were a bearer token. This was a recoverable scanner false positive. Corrective action: preserved token scans but required concrete token-like bearer material instead of adjacent descriptive words. Final result: PASS.

## Recommendation

Proceed to closeout only after this implementation PR merges and post-merge public-safety/advisories/required-check gates are green. Closeout should mark NA-0580 DONE, record D-1152, and restore exactly one READY successor: `NA-0581 -- QSL Remote qsl-server Expected Bind Failure Remediation Harness`. Do not implement NA-0581 during closeout.
