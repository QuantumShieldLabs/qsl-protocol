Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0313 qsc Handshake Suite-ID Parameter-Block Implementation Harness Testplan

Directive: QSL-DIR-2026-05-18-121 / NA-0313

## Objective

Validate that NA-0313 implements and proves the bounded qsc `QHSM` v2
suite-id parameter-block admission surface authorized by D-0603, or stops with
an exact blocker if the surface cannot be implemented in the authorized files.

## Protected invariants

- Exactly one READY item remains NA-0313 until closeout.
- D-0605 exists exactly once after the implementation evidence patch.
- D-0606 is absent during the implementation patch.
- qsc changes stay inside the exact authorized qsc file set.
- qsp implementation, dependencies, Cargo files, workflows, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, public docs,
  branch-protection, and public-safety configuration remain untouched.
- Compatibility behavior is separated from explicit v2 suite-context
  admission.
- Suite-required mode rejects legacy and malformed suite-context inputs before
  accepted qsp session state is written.
- Reject paths do not create `recv_commit`, qsp output, accepted session state,
  surviving partial state, panic/backtrace output, or fixture secret/sentinel
  output.
- Metadata runtime reduction remains visible and is not implemented here.

## Allowed scope

Implementation/test scope:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs`

Governance/evidence scope:

- `docs/governance/evidence/NA-0313_qsc_handshake_suite_id_parameter_block_implementation_harness.md`
- `tests/NA-0313_qsc_handshake_suite_id_parameter_block_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsp implementation files.
- qsc changes outside the authorized list.
- qsl-server implementation files.
- qsl-attachments implementation files.
- qsc-desktop files.
- website or external website files.
- README or START_HERE.
- docs/public files unless a later directive explicitly requires a conservative
  reference update.
- `.github` workflows.
- `Cargo.toml` or `Cargo.lock`.
- dependency updates.
- branch-protection or public-safety configuration.
- metadata runtime implementation.

## Implementation boundary requirements

The qsc implementation must provide:

- `QHSM` v1 legacy compatibility path;
- `QHSM` v2 frame header with bounded parameter-block length;
- canonical critical Suite-2 context parameter carrying `protocol_version` and
  `suite_id`;
- deterministic parser/admission rejects for unsupported, downgrade-like,
  stripped, duplicate, unknown critical, unknown noncritical, noncanonical,
  malformed, and inconsistent tuples;
- byte-exact A1/B1/A2 context matching;
- transcript and qsc handshake key-context binding for explicit v2 frames;
- accepted-state write only after successful admission and authentication;
- deterministic internal reason labels without secret-bearing diagnostics.

## Harness marker requirements

The qsc harness must emit these markers only after all associated assertions
pass:

- `NA0313_QHSM_V2_PARAMETER_BLOCK_PARSE_OK`
- `NA0313_VALID_SUITE2_ACCEPT_OK`
- `NA0313_LEGACY_COMPAT_ACCEPT_OK`
- `NA0313_REQUIRED_MODE_LEGACY_REJECT_OK`
- `NA0313_UNSUPPORTED_SUITE_REJECT_OK`
- `NA0313_DOWNGRADE_SUITE_REJECT_OK`
- `NA0313_STRIPPED_SUITE_REJECT_OK`
- `NA0313_MISMATCH_SUITE_REJECT_OK`
- `NA0313_DUPLICATE_SUITE_REJECT_OK`
- `NA0313_UNKNOWN_CRITICAL_REJECT_OK`
- `NA0313_NONCANONICAL_REJECT_OK`
- `NA0313_MALFORMED_REJECT_OK`
- `NA0313_TRANSCRIPT_BINDING_OK`
- `NA0313_KEY_CONTEXT_BINDING_OK`
- `NA0313_NO_MUTATION_ON_REJECT_OK`
- `NA0313_NO_OUTPUT_ON_REJECT_OK`
- `NA0313_NO_SECRET_LEAK_OK`
- `NA0313_QSC_SUITE_ID_PARAMETER_BLOCK_OK`

## Valid path requirements

The harness must prove:

- Alice A1, Bob B1, and Alice A2 are `QHSM` v2 frames in suite-required mode.
- All three frames carry the canonical Suite-2 parameter block.
- Both peers store accepted qsp session state only after successful A1/B1/A2.
- Stored send and receive session tuples remain Suite-2.
- Compatibility mode can still complete a v1 handshake and is not counted as
  explicit v2 admission.

## Reject path requirements

The harness must cover:

- v1 legacy frame rejected in suite-required mode;
- unsupported suite id;
- downgrade-like tuple;
- stripped suite id;
- mismatched A1/B1 suite context;
- mismatched B1/A2 suite context;
- duplicate suite-id parameter;
- unknown critical parameter;
- noncanonical parameter order;
- malformed parameter length;
- transcript-context mismatch;
- key-context mismatch or missing required context;
- replayed v2 A1 and replayed v2 A2 where representable.

## No-mutation/no-output/no-leak requirements

Reject tests must assert:

- no accepted qsp session file is created for rejected peers;
- pending state is cleared for ambiguous or malicious v2 rejects;
- replayed A2 does not mutate the already accepted session snapshot;
- no `handshake_complete`, `recv_commit`, or successful qsp output marker is
  emitted by reject paths;
- reject output does not include fixture passphrase, route-token shape,
  plaintext/sentinel, panic text, thread panic text, or stack-backtrace text.

## Model/vector/refimpl cross-check requirements

Validation must run:

- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `cargo test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`
- full refimpl tests where feasible.

The qsc harness must parse the NA-0310 vector file and require all 20 vector
categories to remain present.

## Metadata successor requirements

If NA-0313 succeeds, the selected successor must be:

NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan

If NA-0313 stops on a qsc prerequisite blocker, the selected successor must be
an exact qsc blocker-resolution lane. NA-0313 must not implement metadata
runtime behavior.

## Claim-boundary requirements

The evidence and PR body must preserve:

- no production readiness claim;
- no public internet service readiness claim;
- no external review completion claim;
- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no claim that metadata runtime reduction is implemented;
- no claim that compatibility-mode v1 acceptance is explicit v2 admission.

## Backup-impact requirements

Record whether the lane creates durable evidence outside the current
`/srv/qbuild/work` qsl-protocol scope. Expected result: no backup-plan update
is required if changes stay in tracked qsl-protocol paths.

## Required local checks

Before PR:

- `git status --porcelain=v1 --branch`
- `git diff --name-only`
- `git diff --check`
- `cargo fmt -p qsc --check`
- `cargo +stable check -p qsc --locked`
- `cargo clippy -p qsc --locked -- -D warnings`
- `cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- qsc NA-0302, NA-0303, and NA-0304 harnesses where directly runnable;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for NA-0310 vectors;
- targeted NA-0310 refimpl oracle;
- full refimpl tests where feasible;
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- metadata conformance and metadata phase-2 harnesses;
- demo smoke/stress and repeated soak where feasible;
- qshield-cli build/test where feasible;
- queue and decision helpers;
- scope guard with exact allowed paths;
- link-check and added-line leak scan;
- overclaim phrase scan;
- goal-lint on PR body;
- classifier proof for changed path set.

## CI expectations

- Required checks must attach and complete green before merge.
- `public-safety` must remain a required status check and complete green on the
  PR head.
- The PR must merge normally with `--merge` and `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, delete-branch flag, branch
  deletion command, or branch-protection mutation is permitted.
- Post-merge `public-safety` must complete green on `origin/main`.

## Successor handoff

After successful PR merge and green post-merge public-safety, NA-0313 closeout
should mark NA-0313 DONE and restore exactly one READY item:

NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan
