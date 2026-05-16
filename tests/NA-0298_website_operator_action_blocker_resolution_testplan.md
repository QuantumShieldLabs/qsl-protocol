Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0298 Website Operator Action Blocker Resolution Testplan

## Objective

Validate that NA-0298 records the website source/deploy/authority operator
bundle state without mutating any website, external website repository,
deployment surface, or qsl-protocol implementation path.

## Protected Invariants

- Source discovery does not equal authority to edit.
- Historical deploy evidence does not equal current deploy authority.
- Operator action capture does not equal website implementation.
- Missing bundle fields remain visible.
- Exactly one READY item remains NA-0298.
- NA-0297 remains DONE.
- D-0572 exists once and D-0573 remains absent.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  website-updated, source-verified, deploy-ready, or implementation-ready
  claim is introduced.
- Required public-safety stays green.

## Allowed Scope

- `docs/governance/evidence/NA-0298_website_operator_action_blocker_resolution.md`
- `tests/NA-0298_website_operator_action_blocker_resolution_testplan.md`
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- website or external website repository files;
- website PR creation, merge, deployment, DNS, hosting, settings, forms,
  comments, public posts, social actions, or media generation;
- README, START_HERE, `.github/**`, `scripts/**`, Cargo/dependency, workflow,
  branch-protection, or public-safety configuration changes;
- qsl-protocol runtime, protocol, crypto, state-machine, demo, service,
  qsc-desktop, qsl-server, qsl-attachments, qsp, qsc, qsl-client, apps, tools,
  inputs, formal, or production service implementation paths.

## Operator Bundle Discovery Requirements

The evidence document must classify each required operator bundle field as one
of:

- `PROVIDED_VERIFIED`
- `PROVIDED_UNVERIFIED`
- `MISSING`
- `CONTRADICTORY`
- `NOT_APPLICABLE`

Required fields:

1. exact target surface;
2. exact website source repository URL;
3. exact branch and base SHA;
4. package manager;
5. build command;
6. preview/staging command or process;
7. deployment target/provider/path;
8. rollback process;
9. hosting/deployment authority;
10. future website source PR permission;
11. future read-only local build permission;
12. future edit scope permission;
13. future merge/deploy approval requirement;
14. environment variable names/status or statement that none are needed;
15. live deployment forbidden unless separately authorized.

## External Recheck Requirements

Read-only inspection must cover:

- `https://quantumshieldlabs.org/`
- `https://quantumshieldlabs.dev/`
- `https://github.com/QuantumShieldLabs`
- `https://github.com/QuantumShieldLabs/qsl-protocol`
- official public repositories under `QuantumShieldLabs` that appear relevant;
- public candidate `mbennett-labs/qsl` only as unconfirmed unless officially
  linked and authorized;
- any URL or repository found by local search.

For each inspected target, record:

- URL;
- inspected status;
- official/unofficial classification;
- source/deploy clues;
- claim-risk clues;
- whether any missing operator bundle field was resolved;
- no-mutation proof.

## Bundle Classification Requirements

The evidence document must classify the bundle as exactly one of:

- `OPERATOR_BUNDLE_COMPLETE`
- `OPERATOR_BUNDLE_INCOMPLETE`
- `OPERATOR_BUNDLE_CONTRADICTORY`

If fields are missing, implementation must remain blocked. If fields conflict,
the next action must resolve authority conflicts before implementation.

## Operator Action Request Requirements

The operator request packet must include:

- why implementation is blocked;
- why Codex cannot infer source/deploy authority;
- why website mutation remains forbidden;
- all required bundle fields;
- a copy-paste response template;
- an example safe response format;
- how a future NA-0299 or successor may use the bundle;
- what remains forbidden even after the bundle is supplied.

The template must not ask for secret values to be pasted into public docs.
Environment variables may be listed by name/status only, with a secure
delivery process for any secret value.

## Link / Leak / Overclaim Expectations

Required checks:

- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- direct changed-line scan for high-risk claim phrases
- `git diff --check`

Expected result:

- relative links resolve;
- added-line leak scan reports zero findings;
- high-risk phrases appear only in prohibited wording, negated claims,
  explicit NOT_READY/future-gated contexts, or examples of what not to say;
- no public template asks for secret values.

## CI Expectations

Run the local validation bundle:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed paths
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` if present
- goal-lint using a synthetic PR event if needed locally
- changed-path classifier proof

PR merge is allowed only after required protected checks complete normally and
`public-safety` is green. No admin bypass, direct push, squash, rebase, or
branch deletion is allowed.

## Future Implementation Gate

A future implementation directive may proceed only after:

- a complete operator bundle is supplied and verified;
- the future directive explicitly authorizes the exact source repo and action;
- the source worktree is clean;
- branch/base SHA, package manager, build, preview/staging, deploy target,
  rollback, authority, and environment status are verified;
- PR/build/deploy permissions are explicit;
- source-level link and overclaim scans run;
- deployment remains forbidden unless separately approved.
