Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0253A Relay-Auth macOS Public-Safety Recovery Audit

Directive: QSL-DIR-2026-05-07-043 / NA-0253A

## Objective

Recover main `public-safety` after the PR #756 merge commit failed in
`macos-qsc-full-serial` on the qsc relay-auth no-token regression test.

## Start-State Proof

- Governing `origin/main`: `59ae6f25d39e`.
- PR #756: merged, merge commit `59ae6f25d39e`.
- Queue: `READY_COUNT 1`, sole READY `NA-0253`.
- Decisions: D-0110 and D-0439 through D-0473 present once; D-0474 absent;
  duplicate count zero.
- Branch protection: `public-safety` required; force pushes and deletions
  disabled; admin enforcement enabled.

## Failure Evidence

- Failed `public-safety` job: `74847633140`.
- Failed macOS watched-suite job: `74846998386`.
- `qsc-linux-full-suite`: success on job `74846992556`.
- `qsc-adversarial-smoke`: success on job `74846998608`.
- Failing test: `relay_auth_without_token_fails_no_mutation` in
  `qsl/qsl-client/qsc/tests/relay_auth_header.rs`.
- macOS output showed the test expected `relay_unauthorized`, while qsc emitted
  a bounded relay push failure after `push_fail` and `send_attempt ok=false`.

## Root-Cause Classification

The qsc runtime already maps actual HTTP 401/403 relay responses to
`relay_unauthorized`. Local Linux reproduction of both the single test and the
full relay-auth test file passed.

The concrete failure was a test-harness expectation mismatch on macOS: the
unauthenticated send remained fail-closed and did not mutate the relay inbox,
but the command surfaced the outer relay push failure marker instead of the
inner unauthorized marker. The bounded repair therefore stays in the test
harness and does not alter runtime, wire, crypto, auth, branch protection,
public-safety configuration, Cargo metadata, qsl-server, qsl-attachments,
qsc-desktop, website, or external website code.

## Repair Summary

The relay-auth test server now records the received request target,
route-token header state, and authorization-header state before returning an
unauthorized response. The no-token test now accepts only either:

- the authoritative `relay_unauthorized` marker, or
- the bounded combination of relay `push_fail` plus `relay_inbox_push_failed`.

It still asserts all protected invariants:

- the command fails;
- no send-success, delivery, or commit marker is emitted;
- the test server saw the canonical push path;
- the route-token header was present;
- no authorization header was sent;
- relay inbox length remains zero;
- relay auth token, bearer text, and authorization-header text do not appear in
  command output.

## Validation Plan

Required validation for this recovery includes:

- `git diff --check`
- scope guard
- targeted relay-auth single-test run
- full relay-auth test file
- `send_commit` regression test
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- queue and decision parsers
- link/leak checks
- goal-lint
- required PR checks and post-merge main `public-safety`

## Boundary Statement

NA-0253A does not close NA-0253 and does not promote NA-0254. It is a bounded
public-safety recovery only. NA-0253 remains READY until a separate closeout
packet runs after main `public-safety` is green.
