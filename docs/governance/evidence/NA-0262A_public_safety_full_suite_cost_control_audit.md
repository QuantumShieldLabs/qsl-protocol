Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

# NA-0262A Public-Safety Full-Suite Cost-Control Audit

## Objective

Document the root cause and implementation evidence for skipping Linux/macOS full-suite work only on proven docs/governance-only main pushes while preserving fail-closed public-safety behavior for runtime and security-relevant changes.

## Root Cause

Before this lane, docs/governance-only main pushes still waited on full suites for two independent reasons:

- `public-ci.yml` ran `wait-commit-checks` on every push and always required `qsc-linux-full-suite` and `macos-qsc-full-serial`.
- `ci.yml` and `macos-build.yml` ran the full-suite jobs on every non-PR event. Push classification invoked `scripts/ci/classify_ci_scope.sh` without changed paths, and that helper correctly failed closed to `runtime_critical=true`.

The fail-closed default was safe for ambiguous pushes, but too expensive for normal merge commits whose changed paths are proven docs/governance-only.

## Timing Evidence

Packet 0 merged as `5e6ba71a98e1`.

- `macos-qsc-full-serial`: started `2026-05-10T17:39:51Z`, completed success `2026-05-10T18:48:12Z`.
- `qsc-linux-full-suite`: started `2026-05-10T17:41:09Z`, completed success `2026-05-10T19:07:34Z`.
- `public-safety`: started `2026-05-10T17:42:40Z`, completed success `2026-05-10T19:07:42Z`.

The post-merge public-safety result waited until Linux completed even though the Packet 0 merge changed only governance/testplan files.

## Design Choice

Chosen design: prevent heavy Linux/macOS full-suite jobs from running on docs/governance-only main pushes, and make public-safety skip waiting for them only for the same docs-only proof.

Why this is safest:

- Runtime, workflow, Cargo, scripts/ci, app, service, unknown, empty, and mixed scopes still fail closed into full-suite coverage.
- The existing classifier remains the source of truth for path classes.
- Branch protection is unchanged.
- Required PR contexts remain stable; the heavy full-suite jobs are push-only contexts, not protected PR contexts.

## Behavior

Docs/governance-only main push:

- push changed paths are computed from the push before SHA to `GITHUB_SHA`;
- classifier returns `docs_only=true`;
- `qsc-linux-full-suite` and `macos-qsc-full-serial` are skipped;
- public-safety still runs fast push safety checks and skips only the full-suite wait.

Runtime/security/Cargo/workflow/code main push:

- classifier returns `docs_only=false`;
- Linux and macOS full suites run;
- public-safety waits for both full suites to complete successfully.

Ambiguous or mixed main push:

- missing/empty path sets use the existing fail-closed runtime classification;
- mixed docs plus runtime paths classify as runtime critical;
- full suites run and public-safety waits.

## Negative Case Coverage

`python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control` covers:

- docs/governance-only closeout paths skip full-suite wait and jobs;
- qsc runtime path requires full-suite wait and jobs;
- `apps/qshield-cli` path requires full-suite wait and jobs;
- `scripts/ci` path requires full-suite wait and jobs;
- `.github/workflows` path requires full-suite wait and jobs;
- `Cargo.toml` and `Cargo.lock` require full-suite wait and jobs;
- `qsl-server`, `qsl-attachments`, and `qsc-desktop` paths require full-suite wait and jobs;
- mixed docs plus runtime requires full-suite wait and jobs;
- unknown paths require full-suite wait and jobs;
- empty/ambiguous push scope requires full-suite wait and jobs.

## Boundary

This lane does not change protocol, runtime, crypto, demo, qsl-server, qsl-attachments, qsc-desktop, website, external website, Cargo dependency, branch-protection, or public-safety required-check semantics.
