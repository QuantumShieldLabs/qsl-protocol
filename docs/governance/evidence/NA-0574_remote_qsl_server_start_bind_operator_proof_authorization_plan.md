Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0574 Remote qsl-server Start / Bind Operator Proof Authorization Plan

## Executive Summary

NA-0574 consumed the NA-0573/D498 result
`QSL_SERVER_RECOVERY_EXPECTED_BIND_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`.
It does not start qsl-server, run remote commands, run SSH/scp, dispatch
workflows, run qsc send/receive, or mutate runtime/source/workflow/dependency
paths.

NA-0574 selects a non-secret operator proof package for NA-0575. The package
records only coarse yes/no/unknown/not_checked classes and safe labels needed
to classify whether a later lane may authorize a bounded qsl-server start,
operator start, service-owner action, secret/endpoint action, or a fail-closed
stop.

Result classification:
`QSL_SERVER_BIND_START_OPERATOR_PROOF_CAPTURE_READY`.

Selected successor:
`NA-0575 -- QSL Remote qsl-server Start / Bind Operator Proof Capture Harness`.

## qwork Proof Verification

Fresh qwork proof files were copied into the NA-0574 proof root and parsed from
file-backed `.kv` and JSON inputs before repository mutation.

- qwork startup result: OK
- lane/repo/path: NA-0574 / qsl-protocol / expected qbuild worktree
- branch/upstream: main / origin/main
- qwork proof timestamp: `2026-06-30T03:11:20Z` or later
- qwork HEAD and origin/main: `e6f01d620907`
- live pre-fetch HEAD and origin/main: `e6f01d620907`
- worktree, index, and untracked state: clean
- queue proof from qwork: READY_COUNT 1, READY NA-0574
- cargo target mode: shared, ready
- root disk: below the 95 percent stop threshold
- `/backup/qsl`: mounted

Codex did not run qwork, qstart, or qresume.

## D-1136 / D-1137 Inheritance

D-1136 and D-1137 were consumed as the immediate governing decisions.

- D-1136 exists once and is Accepted.
- D-1137 exists once and is Accepted.
- NA-0573 is DONE.
- NA-0574 is READY.
- D-1138 was absent before the NA-0574 implementation patch.
- D-1139 was absent before the NA-0574 implementation patch.
- Duplicate decision count was zero.

D-1136 records that qsl-server source/build/audit/test/fmt passed, qsl-server
was staged under the qslcodex test workspace, remote start was skipped, and the
result classification was
`QSL_SERVER_RECOVERY_EXPECTED_BIND_UNAVAILABLE_OPERATOR_ACTION_REQUIRED`.

D-1137 records that implementation PR #1419 and closeout PR #1420 restored
NA-0574 as the sole READY successor.

## D498 Start / Bind Blocker Review

Inherited D498 state:

- qsl-server staged binary state:
  `QSL_SERVER_STAGE_INSTALLED_OR_REPLACED`.
- remote start classification:
  `QSL_SERVER_START_EXPECTED_BIND_UNAVAILABLE`.
- remote postcheck classification:
  `QSL_SERVER_POSTCHECK_BINARY_READY_LISTENER_NOT_READY`.
- qsl-server was not started.
- no safe expected loopback bind target was available.
- no private bind target, private port, endpoint, topology, token, payload,
  response body, or process identity was guessed or published.

The remaining blocker is not qsl-server source/build/dependency state. It is
the missing non-secret proof that an operator knows the private loopback bind
target and safe start command shape, and that the start path is non-privileged
and secret-free or otherwise requires an operator/service-owner/secret action.

## Current Main Required-Check Classification

Current main was verified after fetch at `e6f01d620907`.

- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: completed success.
- no failed required checks were classified.
- no required pending checks were classified.
- root cargo audit was later validated locally.
- nested qsc fuzz cargo audit was later validated locally.
- `Cargo.toml`, root `Cargo.lock`, and qsc fuzz `Cargo.lock` drift were empty.

The D498 required-check visibility recovery model was applied for PR-scoped
contexts. `goal-lint` and `CodeQL` were not literal main-commit contexts but
were recovered as success from PR #1420 head metadata.

Recovered-failure evidence:

- failing command: main-commit required-check classification script.
- classification: recoverable required-check visibility gap for PR-scoped or
  aggregate contexts.
- corrective action: applied the D498 PR-head/aggregate recovery model.
- final result: PASS.

## Operator Proof Model

NA-0575 must collect only non-secret, coarse operator proof. It must not require
or publish private port values, endpoint URLs, topology, token values,
Authorization headers, config contents, process identities, payloads, response
bodies, authorized_keys content, public SSH key material, or private keys.

Proof classes use yes/no/unknown/not_checked where possible. More specific
safe enums are used only for operator-reviewed labels such as
`expected_loopback_target`, `binary_bind_only`, or `operator_service_required`.

## Future Proof Files

NA-0575 must require this proof package:

- `00_manifest.kv`
- `01_staged_qsl_server_presence.json`
- `02_bind_target_availability.json`
- `03_start_command_shape.json`
- `04_secret_dependency_classification.json`
- `05_operator_action_boundary.json`
- `06_private_material_scan.json`
- `07_safe_to_paste_summary.json`

## Future Proof Fields

Manifest fields:

- `proof_timestamp_utc`
- `operator`
- `host_label`
- `workspace_label`
- `lane`
- `proof_package_version`
- `values_accessed_no_secret`
- `endpoint_values_published_no`
- `private_port_values_published_no`
- `topology_published_no`
- `token_values_published_no`
- `response_bodies_published_no`
- `operator_no_secret_assertion`

Staged qsl-server presence fields:

- `staged_binary_present_class`
- `staged_binary_executable_class`
- `staged_binary_version_class`
- `staged_binary_hash_matches_expected_class`
- `workspace_writable_class`
- `rollback_manifest_present_class`
- `stage_manifest_present_class`
- `raw_output_contains_private_material`
- `redaction_review`

Bind target availability fields:

- `bind_target_known_to_operator_class`
- `bind_target_source_class`
- `bind_target_loopback_only_class`
- `bind_target_private_value_disclosed`
- `bind_target_public_safe_label`
- `bind_target_requires_root_class`
- `bind_target_requires_tailscale_or_firewall_class`
- `raw_output_contains_private_material`
- `redaction_review`

Start command shape fields:

- `start_command_known_class`
- `start_command_source_class`
- `start_command_secret_free_class`
- `start_command_non_privileged_class`
- `start_command_loopback_only_class`
- `start_command_background_policy_class`
- `start_command_value_disclosed`
- `command_args_disclosed`
- `safe_command_shape_label`
- `raw_output_contains_private_material`
- `redaction_review`

Secret dependency classification fields:

- `bearer_required_to_start_class`
- `route_token_required_to_start_class`
- `endpoint_required_to_start_class`
- `config_file_required_class`
- `config_file_contains_secret_class`
- `values_accessed`
- `values_published`
- `raw_output_contains_private_material`
- `redaction_review`

Operator action boundary fields:

- `codex_start_authorized_next_class`
- `operator_must_start_class`
- `sudo_or_systemd_required_class`
- `tailscale_or_firewall_required_class`
- `secret_or_endpoint_action_required_class`
- `recommended_next_owner`
- `raw_output_contains_private_material`
- `redaction_review`

Private-material scan fields:

- `endpoint_pattern_hits`
- `private_ip_or_host_hits`
- `private_port_value_hits`
- `bearer_token_hits`
- `route_token_capability_hits`
- `authorization_header_hits`
- `payload_or_body_hits`
- `process_identity_hits`
- `key_material_hits`
- `raw_output_private_material_class`
- `scan_pass`

Safe-to-paste summary fields:

- `proof_dir`
- `files_created`
- `scan_pass`
- `ready_for_director_review`
- `safe_to_paste_message`
- `do_not_paste_raw_files_unless_operator_reviewed`

## Future Operator Command / Collection Design

NA-0575 should be operator-run unless a later directive explicitly authorizes
Codex-executed proof collection. The future collector may run on inspiron or
consume operator-reviewed proof from the qslcodex workspace, but shared output
must remain non-secret.

The future collector must:

- produce the required proof files listed above;
- never print actual private port values;
- never print endpoint URLs;
- never print bearer or route-token values;
- never print Authorization headers;
- never print process command lines containing private arguments;
- never print payloads or response bodies;
- never print authorized_keys content or SSH key material;
- distinguish yes/no/unknown/not_checked rather than guessing;
- set `scan_pass=false` if private material appears;
- publish only the safe-to-paste summary by default.

Allowed future proof source classes:

- qslcodex workspace metadata;
- qsl-server help/version availability class after operator review;
- qsl-server staged binary manifest from NA-0573;
- non-secret operator assertion;
- redacted local check results;
- redacted proof-root files.

Forbidden future publication:

- raw process list with command-line arguments;
- raw config files;
- raw environment variables;
- raw endpoint values;
- raw port values;
- raw token, bearer, or Authorization values;
- raw logs with request or response bodies.

## Safe-to-Paste Policy

Only `07_safe_to_paste_summary.json` and a short Director-reviewed summary are
safe to paste by default. Raw proof files must not be pasted unless the
operator reviewed them first.

Repository publication requires:

- `scan_pass=true`;
- every `redaction_review` field is pass;
- every publication boundary field confirms values were not published;
- no endpoint, private port, topology, token, Authorization, payload, response
  body, process identity, authorized_keys content, public key material, private
  key, Cloudflare token, API key, or long opaque token string is present.

If private material appears, NA-0575 must classify
`QSL_SERVER_BIND_START_PROOF_PRIVATE_MATERIAL_STOP`.

## Private-Material Policy

NA-0575 must STOP before repository publication if shared material contains:

- endpoint values;
- private hosts;
- private IPs;
- private port values;
- private topology;
- process names or process identities;
- route-token/capability values;
- bearer values;
- Authorization headers;
- payloads;
- response bodies;
- raw authorized_keys content;
- public SSH key material;
- private keys;
- secret environment values;
- Cloudflare tokens;
- API keys;
- long opaque token strings.

Allowed publication classes:

- safe enum values;
- yes/no/unknown/not_checked;
- safe labels such as `expected_loopback_target`;
- public commit SHAs;
- public PR/check IDs;
- qslcodex workspace path labels;
- qsl-server binary staged/present classes;
- qsl-server version availability class, not raw full output if it contains
  environment details.

## Decision Tree

NA-0575 result classifications:

- `QSL_SERVER_BIND_START_PROOF_CODEX_START_READY`: bind target is known
  privately; loopback-only; start command shape is known; secret-free;
  non-privileged; no sudo/systemd/Tailscale/firewall required; no private
  material publication; a later lane may authorize bounded Codex start under
  an exact command allowlist.
- `QSL_SERVER_BIND_START_PROOF_OPERATOR_START_REQUIRED`: qsl-server start is
  safe but must be performed by the operator rather than Codex.
- `QSL_SERVER_BIND_START_PROOF_SERVICE_OWNER_ACTION_REQUIRED`:
  sudo/systemd/service ownership is required.
- `QSL_SERVER_BIND_START_PROOF_SECRET_OR_ENDPOINT_ACTION_REQUIRED`:
  secret/endpoint/private configuration is required before start.
- `QSL_SERVER_BIND_START_PROOF_INSUFFICIENT`: proof remains unknown or
  not_checked after the operator package.
- `QSL_SERVER_BIND_START_PROOF_PRIVATE_MATERIAL_STOP`: private material
  appears.
- `QSL_SERVER_BIND_START_PROOF_AMBIGUOUS_STOP`: proof conflicts or cannot be
  classified.

Future successor models:

- Codex bounded start ready:
  `NA-0576 -- QSL Remote qsl-server Codex-Bounded Start and Postcheck Harness`.
- Operator start required:
  `NA-0576 -- QSL Remote qsl-server Operator Start Action Authorization Plan`.
- Service-owner/root action required:
  `NA-0576 -- QSL Remote qsl-server Service Owner Action Authorization Plan`.
- Secret/endpoint action required:
  `NA-0576 -- QSL Remote qsl-server Secret / Endpoint Action Authorization Plan`.
- Insufficient, ambiguous, or private-material stop: no closeout; Director
  review required.

## Result Classification

NA-0574 result classification:
`QSL_SERVER_BIND_START_OPERATOR_PROOF_CAPTURE_READY`.

Rejected fallback classifications:

- `QSL_SERVER_BIND_START_OPERATOR_PROOF_SCHEMA_AMBIGUOUS_STOP`
- `QSL_SERVER_BIND_START_PRIVATE_MATERIAL_POLICY_STOP`
- `QSL_SERVER_BIND_START_REQUIRED_CHECK_STOP`
- `QSL_SERVER_BIND_START_AMBIGUOUS_STOP`

## Selected Successor

Selected successor:
`NA-0575 -- QSL Remote qsl-server Start / Bind Operator Proof Capture Harness`.

NA-0575 must capture and review only non-secret operator proof needed to
classify the qsl-server start/bind blocker left by NA-0573. It must not
implement a qsl-server start.

## Required-Check Boundary

NA-0574 changes only governance/testplan evidence and does not alter required
checks, workflows, branch protection, dependency files, or runtime source. The
required-check state was verified before mutation and is revalidated before PR.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, lockfile, qsc
source/test/fuzz/Cargo path, qsl-server source, qsl-server dependency, or
qsl-attachments path is changed by NA-0574.

## Workflow Mutation Boundary

No workflow files are changed. No workflow dispatch or rerun is performed.

## Runtime / qsc Boundary

No qsc command is executed. No qsc send/receive or E2EE path is exercised. No
qsc runtime behavior is changed.

## qsl-server / qsl-attachments Boundary

No qsl-server command is executed. No qsl-server start, deployment, run, source
mutation, dependency mutation, or PR occurs in NA-0574. No qsl-attachments
command, clone, build, run, mutation, or PR occurs.

## Remote-Action Boundary

No SSH, scp, Tailscale, remote command, remote probe, remote write, sudo,
systemctl, service, firewall, account, shell, authorized_keys, or root-owned
path action occurs in NA-0574.

## Public-Site / Cloudflare Boundary

No public-site path, Cloudflare configuration, Cloudflare token, API token, DNS
configuration, or public-readiness page is changed.

## Raw Output Boundary

Raw proof output remains proof-root-only. Repository evidence records only
coarse classifications, safe labels, public commit/PR/check identifiers, and
non-secret validation summaries.

## Claim Boundary

NA-0574 makes no public-readiness claim, no production-readiness claim, no
public-internet-readiness claim, no external-review-complete claim, no
vulnerability-free claim, no bug-free claim, no perfect-build claim, and no
perfect-crypto claim.

## Validation

Required validation before PR includes:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- marker proof
- link-check
- added-line/new-file private-material scan
- prohibited-material scan
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz cargo audit
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests are skipped because NA-0574 is authorization-only,
does not mutate qsc source/runtime/dependency/workflow paths, and does not
authorize qsc send/receive.

## Recommendation

Merge NA-0574 if validation and required checks stay green, then close out to
NA-0575 only after post-merge verification confirms D-1138 exists once,
required checks are healthy, and the exact successor block is available without
placeholders.
