Goals: G1, G3, G4, G5

# NA-0279 qsl-server Rate-Limit / Global Route-Cap Design Testplan

## Objective

Verify that NA-0279 records an evidence-bound qsl-server rate-limit and global
route-cap design plus NA-0280 executable harness plan without implementing
qsl-server behavior, changing qsl-server tests, or changing qsl-protocol
runtime/protocol behavior.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0279.
- D-0528 exists once after the evidence patch.
- D-0529 remains absent in the evidence PR.
- qsl-server remains read-only.
- qsl-server implementation files remain untouched.
- qsl-server tests/harness files remain untouched.
- qsl-server `Cargo.toml`, `Cargo.lock`, and workflows remain untouched.
- qsl-protocol implementation paths remain untouched.
- qsl-attachments implementation paths remain untouched.
- qsc-desktop, website, external website, workflows, scripts, Cargo files,
  branch protection, and public-safety configuration remain untouched.
- Current gaps remain explicit: no in-app rate limiting and no global
  route-count cap are implemented yet.
- No production-readiness or deployment-approval claim is introduced.
- NA-0280 remains future work.

## Allowed / Forbidden Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0279_qsl_server_rate_global_cap_design.md`
- `tests/NA-0279_qsl_server_rate_global_cap_design_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` only for
  handoff references
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope:

- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration
- branch deletion

## Read-Only qsl-server Baseline Proof

The design evidence must record:

- qsl-server repo path and audited SHA;
- clean qsl-server worktree proof;
- qsl-server `cargo audit --deny warnings` result, if run;
- qsl-server `cargo test --locked` result, if run;
- current per-route queue cap semantics;
- current body cap semantics;
- current overload behavior;
- current route-token isolation behavior;
- current auth reject and body reject no-mutation behavior;
- current no-secret logging behavior under pressure;
- current absence of in-app rate limiting;
- current absence of global route-count caps;
- current route lifecycle/TTL gap.

## Rate / Global-Cap Design Requirements

The design must answer:

- rate-limit dimensions: per route, process/global, auth-mode, and source-IP
  boundary;
- rate-limit algorithm and accounting bounds;
- reject status and error codes;
- no-mutation behavior for rate and route-cap rejects;
- global route-count cap semantics;
- route creation rules;
- route expiration or cleanup rules;
- interaction with per-route queue depth;
- memory-bound formula and accounting caps;
- auth ordering and wrong-bearer behavior;
- logging/no-secret rules;
- deployment boundary between in-app controls and upstream controls.

## Future NA-0280 Executable Harness Requirements

NA-0280 should include deterministic local qsl-server tests for:

- new-route reject at global route cap;
- unknown pulls not consuming route slots;
- rate-limited pushes returning the chosen 429 error code without enqueue;
- wrong bearer auth not creating route or rate state;
- oversized bodies not creating route or rate state;
- drain or TTL cleanup releasing route slots;
- log capture proving rate/global-cap rejects do not leak route tokens, bearer
  auth, auth headers, or payloads;
- fail-closed configuration parsing for new limits.

The harness must use loopback/local execution only, bounded loops, small limits,
and a fake or controllable clock if time refill/TTL behavior is claimed.

## No Implementation Proof

NA-0279 must prove:

- qsl-server tracked files did not change;
- qsl-server tests/harness files did not change;
- qsl-protocol implementation files did not change;
- no Cargo, workflow, dependency, branch-protection, or public-safety file
  changed;
- any rate/global-cap wording remains design/future-harness wording, not an
  implementation claim.

## No Production-Readiness Claim

NA-0279 may claim only design and harness-planning evidence. It must not claim
public exposure approval, production deployment approval, completed external
service review, implemented in-app rate limiting, or implemented global
route-count caps.

## Link / Leak / Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth header values, raw route
  tokens, payload sentinels, sensitive endpoints, or long secret-like hex
  dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.
- Known gaps must remain visible.

## CI Expectations

Local validation should include:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- direct overclaim phrase scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- repo-local goal-lint or helper PR-body preflight with the exact Goals line.

Required CI must pass normally before merge. The expected qsl-protocol PR scope
is docs, governance, evidence, and testplan only, so docs-only cost control may
skip full suites where policy allows.

## Successor Handoff

After the NA-0279 qsl-protocol evidence PR merges and post-merge
public-safety is green, NA-0279 may close out under a separate packet that
promotes exactly one READY successor: NA-0280, the executable qsl-server
rate-limit/global route-cap harness lane. The closeout must not implement
NA-0280.
