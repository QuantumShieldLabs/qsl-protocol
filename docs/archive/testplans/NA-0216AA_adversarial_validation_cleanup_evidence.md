Status: Archive
Owner: QSL governance
Last-Updated: 2026-03-31

# NA-0216AA Adversarial Validation / Fuzz / Chaos Validation + Cleanup Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Directive lane: `NA-0216AA`
- Validation posture: qbuild-first, AWS-free, sibling repos read-only
- Base commit: `014946b78bc1`

## Authority Proof

- `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `014946b78bc1`
- `qsl-server` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `0826ffa4d6f3`
- `qsl-attachments` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `e94107ac094d`
- `qsl-protocol` `READY` count was `1`, with `NA-0216AA` as the sole `READY` item
- `qsl-server` `READY` count was `0`, and `NA-0012` remained `DONE`
- `qsl-attachments` `READY` count was `0`

## Frozen Program Proof

The merged adversarial program remained the one frozen by `DOC-G4-001` and implemented by `NA-0216A`:

- helper extraction under `qsl/qsl-client/qsc/src/adversarial/`
- property coverage in `qsl/qsl-client/qsc/tests/adversarial_properties.rs`
- nightly Miri driver in `qsl/qsl-client/qsc/tests/adversarial_miri.rs`
- bounded libFuzzer targets under `qsl/qsl-client/qsc/fuzz/`
- workflow placement in `.github/workflows/qsc-adversarial.yml`
- local smoke wiring in `scripts/ci/qsc_adversarial.sh`

## Validation Results

### Baseline package proof

- `cargo fmt --manifest-path qsl/qsl-client/qsc/Cargo.toml --all --check` passed
- `cargo build -q --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked` passed
- `cargo clippy -q --manifest-path qsl/qsl-client/qsc/Cargo.toml --all-targets -- -D warnings` passed

### Adversarial lane proof

- `sh scripts/ci/qsc_adversarial.sh` passed
- stable property test result: `adversarial_properties` -> `6 passed`
- stable smoke result for the Miri driver file: `adversarial_miri` -> `4 passed`
- bounded fuzz result:
  - `qsc_route_http` completed cleanly
  - `qsc_payload_boundaries` completed cleanly
  - `qsc_vault_envelope` completed cleanly
- nightly instrumented result:
  - `cargo +nightly miri test --manifest-path qsl/qsl-client/qsc/Cargo.toml --test adversarial_miri -- --nocapture` -> `4 passed`

### Regression proof around the adversarial helpers

- `route_header_migration_docs_na0195a` -> `2 passed`
- `attachment_streaming_na0197c` -> `17 passed`, `1 ignored`
- `vault` -> `7 passed`

## Validation / Cleanup Decision Set

### Corpus truthfulness and non-mutation handling

Classification: workflow/CI mismatch review

Result:
- no blocker found
- `scripts/ci/qsc_adversarial.sh` copies each checked-in seed corpus into a temporary run directory before invoking libFuzzer
- checked-in corpus files remained unchanged
- repo cleanliness stayed intact after the bounded fuzz run

Why this stays in `NA-0216AA` scope:
- this is direct validation of the merged adversarial program's corpus handling, not a new protocol or product lane

### Sanitizer / Miri / workflow assumptions

Classification: docs/help mismatch review

Result:
- no blocker found
- the workflow remains truthful about placement:
  - `qsc-adversarial-smoke` runs the stable smoke script plus bounded fuzzing
  - `qsc-adversarial-miri` runs the nightly Miri lane separately
- local nightly Miri execution passed, so there is no evidence-only claim gap between workflow intent and executable proof

Why this stays in `NA-0216AA` scope:
- the lane explicitly required validation of sanitizer and workflow assumptions after merge

### Deterministic tests for the new helpers and targets

Classification: test expectation mismatch review

Result:
- no blocker found
- the direct helper/property/Miri tests passed
- the adjacent route-header, attachment-streaming, and vault regression sets also passed, so the helper extraction did not show semantic drift in the nearby validated paths

Why this stays in `NA-0216AA` scope:
- confirming the merged deterministic proof is part of the post-merge validation lane

### CI placement truthfulness

Classification: workflow/CI mismatch review

Result:
- no blocker found
- CI remains bounded to the touched adversarial workflow/script set
- concurrency cancellation is already present
- rust build caching is already present in the smoke job
- no additional CI widening was required to make the lane decision-grade

Why this stays in `NA-0216AA` scope:
- the directive explicitly allowed bounded review of the touched adversarial workflow/script set

### Evidence clarity about what is implemented versus deferred

Classification: mixed issue

Result:
- fixed in this lane by adding this archive evidence artifact
- the repo now records the post-merge validation result explicitly instead of relying only on ephemeral command output
- the next blocker is documented as maintainability planning, not another adversarial implementation/finalization lane

Why this stays in `NA-0216AA` scope:
- the directive explicitly included evidence-assumption cleanup and truthful queue advancement

## Closeout Judgment

`NA-0216AA` validated cleanly enough for closeout on path `CD1`.

Why:

- no direct adversarial validation, corpus, workflow, sanitizer, or deterministic-test blocker remained after post-merge proof
- sibling repo assumptions stayed truthful with both sibling repos still at `READY=0`
- the next load-bearing blocker is the auditability and maintainability concentration in `qsl/qsl-client/qsc/src/main.rs`, not another adversarial-program finalization gap

Supporting metric:

- `qsl/qsl-client/qsc/src/main.rs` = `21,627` lines
- total `qsl/qsl-client/qsc/src/**/*.rs` = `24,790` lines
- `main.rs` share = `87.24%`

Truthful successor:

- `NA-0217 — qsc Modularization / File-Size Reduction Plan`
