Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0313 qsc Handshake Suite-ID Parameter-Block Implementation Harness

Directive: QSL-DIR-2026-05-18-121 / NA-0313

## Executive summary

NA-0313 implements the bounded qsc `QHSM` v2 suite-context parameter-block
surface authorized by D-0603 and proves it with an executable qsc CLI/relay
harness.

The implementation is intentionally narrow:

- `QHSM` v1 remains the compatibility path.
- `QHSM` v2 carries one canonical critical parameter: the Suite-2
  `protocol_version` / `suite_id` tuple.
- `suite-required` mode rejects legacy, stripped, unsupported, downgrade-like,
  mismatched, duplicate, unknown, noncanonical, malformed, transcript-context,
  and key-context inputs before accepted qsp session state is written.
- Reject paths emit deterministic suite-admission reason labels without
  creating `recv_commit`, qsp output, accepted session state, surviving pending
  state, panic/backtrace text, or fixture secret/sentinel output.
- Valid `QHSM` v2 Suite-2 A1/B1/A2 preserves the existing Suite-2 session tuple.

This lane does not change qsp implementation, dependencies, Cargo files,
workflows, services, website sources, public docs, README, START_HERE,
branch-protection, public-safety configuration, or metadata runtime behavior.

Selected successor after successful closeout:

NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan

## Live NA-0313 scope

Live `NEXT_ACTIONS.md` records NA-0313 as the sole READY item with these
boundaries:

- implement and prove only the bounded qsc handshake suite-id parameter-block
  harness selected by NA-0312;
- use the file, marker, vector, model, refimpl, compatibility,
  transcript/key-context, and stop-condition boundaries frozen by D-0603;
- do not change qsc outside the exact files authorized by NA-0312;
- do not change qsp implementation, dependencies, workflow, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, public docs,
  branch-protection, or public-safety configuration;
- keep metadata runtime reduction visible and restore it as the next lane after
  NA-0313 unless NA-0313 stops on a qsc prerequisite blocker.

The live scope matched the directive and permitted the implementation below.

## Inherited D-0603 authorization

D-0603 authorized a future bounded implementation/harness lane limited to:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs`
- named governance, evidence, testplan, and traceability files.

D-0603 required compatibility-mode separation, suite-required fail-closed
behavior, transcript binding, qsc handshake key-context binding, vector/refimpl
and formal/model cross-checks, reject no-mutation/no-output/no-leak evidence,
and exact stop if broader files or semantics were needed.

## Sources inspected

- `GOALS.md`
- `PROJECT_CHARTER.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0312_closeout_restore_na0313_testplan.md`
- `docs/governance/evidence/NA-0312_qsc_handshake_suite_id_parameter_block_implementation_authorization.md`
- `tests/NA-0312_qsc_handshake_suite_id_parameter_block_authorization_testplan.md`
- `docs/governance/evidence/NA-0311_qsc_handshake_suite_id_qsc_harness_requirements.md`
- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md`
- `docs/governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md`
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`
- `docs/governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md`
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` read-only
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md` read-only
- `README.md` read-only
- `START_HERE.md` read-only

Search terms included `QHSM`, `Handshake`, `A1`, `B1`, `A2`, `HS_VERSION`,
`session`, `frame`, `encode`, `decode`, `parse`, `protocol_version`,
`suite_id`, `parameter`, `compatibility`, `legacy`, `recv_commit`, `output`,
`passphrase`, `route`, `qse`, `error`, `reject`, and `malformed`.

## Implementation summary

Changed qsc files:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs`

`qsl/qsl-client/qsc/tests/common/mod.rs` was inspected but not changed.

`handshake/mod.rs` now has:

- `QHSM` v1 legacy compatibility and `QHSM` v2 explicit suite-context frames;
- bounded parameter-block parsing with a 64-byte cap;
- canonical Suite-2 tuple admission for protocol version `0x0500` and suite id
  `0x0002`;
- duplicate, unknown critical, unknown noncritical, noncanonical, malformed,
  stripped, unsupported, downgrade-like, and inconsistent tuple rejection;
- v2 context in A1/B1/A2 transcript bytes;
- v2 context in the qsc handshake KDF/context labels;
- byte-exact pending-context comparison before accepted-state mutation;
- replay rejection for v2 A1/A2 shapes that would otherwise reprocess accepted
  or pending state;
- deterministic suite-admission markers for accept and reject outcomes.

`cmd/mod.rs` adds a bounded `HandshakeSuiteMode` enum and `--suite-mode` option
for `handshake init` and `handshake poll`. `main.rs` passes the selected mode to
new suite-mode entry points. Existing wrapper entry points remain so existing
TUI/internal callers continue through the named compatibility path.

## QHSM v2 parameter-block behavior

The v2 frame header is:

- `QHSM` magic;
- version `2`;
- frame type `1`, `2`, or `3` for A1/B1/A2;
- a two-byte parameter-block length;
- the parameter block;
- the existing frame payload.

The canonical Suite-2 parameter block contains exactly one critical parameter:

- parameter id `0x0001`;
- critical flag set;
- four-byte value `protocol_version || suite_id`;
- canonical Suite-2 value `0x0500 || 0x0002`.

The parser is fail-closed. It rejects overlong blocks, incomplete headers,
malformed lengths, reserved flags, missing suite context, duplicate suite
context, out-of-order parameters, unknown critical parameters, unknown
noncritical parameters, unsupported Suite-2 suite ids, legacy tuple downgrade
attempts, and inconsistent tuples.

## Compatibility mode behavior

The named compatibility mode is `legacy-compat`. It preserves existing qsc
`QHSM` v1 behavior and emits `ACCEPT_QSC_HS_LEGACY_COMPATIBILITY` when the
legacy path completes. This path is not represented as explicit v2 admission.

The CLI default remains the named compatibility mode to preserve existing
operator and TUI behavior. The NA-0313 harness explicitly selects
`--suite-mode suite-required` for v2 Suite-2 admission tests and
`--suite-mode legacy-compat` only for compatibility tests.

## Suite-required mode behavior

The `suite-required` mode emits v2 frames for initiated handshakes and rejects
v1 frames with `REJECT_QSC_HS_LEGACY_REQUIRED`. It also rejects missing,
stripped, unsupported, downgraded, mismatched, duplicate, unknown,
noncanonical, malformed, transcript-context, key-context, and replay cases.

Suite-required reject paths clear pending state where the rejected item could
otherwise leave an ambiguous partial handshake. They do not store accepted qsp
session state and do not emit qsp receive/output markers.

## Valid path proof

The qsc NA-0313 harness executes a full local CLI/relay handshake:

1. Alice sends a v2 A1 in `suite-required` mode.
2. Bob validates the v2 A1 and sends a v2 B1.
3. Alice validates the v2 B1 and sends a v2 A2.
4. Bob validates the v2 A2 and both peers store Suite-2 session state.

The harness inspects relay-carried A1/B1/A2 bytes and requires version `2` plus
the canonical Suite-2 parameter block on all three frames. It decrypts stored
qsp session state and verifies both send and receive tuples retain the existing
Suite-2 protocol version and suite id.

## Reject path proof

The harness covers these reject classes:

- legacy v1 rejected in suite-required mode;
- unsupported suite id;
- downgrade-like legacy tuple in v2;
- stripped suite id;
- duplicate suite id parameter;
- unknown critical parameter;
- unknown noncritical parameter;
- noncanonical order;
- malformed length;
- inconsistent protocol/suite tuple;
- mismatched A1/B1 context;
- mismatched B1/A2 context;
- transcript-context mismatch;
- missing key context in required mode;
- replayed A1 with suite context;
- replayed A2 with suite context.

Every reject asserts a deterministic `REJECT_QSC_HS_*` reason where the
runtime surface can safely expose one.

## No-mutation proof

Reject tests assert:

- no accepted qsp session file is created for rejected peers;
- pending handshake state is absent after reject paths that can observe
  ambiguous or malicious v2 state;
- replayed A2 does not change the already accepted session snapshot.

The harness marker `NA0313_NO_MUTATION_ON_REJECT_OK` is emitted only after those
checks pass.

## No-output proof

Reject tests assert:

- no `event=handshake_complete` marker appears on reject output;
- no `event=recv_commit` marker appears on reject output;
- no `event=qsp_unpack ok=true` marker appears on reject output;
- the local relay has no B1/A2 output after parser/admission rejects that
  should stop before response.

The harness marker `NA0313_NO_OUTPUT_ON_REJECT_OK` is emitted only after those
checks pass.

## No-secret-leak proof

The harness checks command output for fixture passphrase, route-token shapes,
desktop passphrase env name, a sentinel value embedded in negative fixtures,
panic text, stack-backtrace text, and Rust thread-panic text. The secret-scan
assertions run on accept and reject output before emitting
`NA0313_NO_SECRET_LEAK_OK`.

## NA-0309 model cross-check

NA-0309 model properties map directly to this harness:

- valid v2 Suite-2 accept -> v2 A1/B1/A2 accept and stored Suite-2 tuple;
- legacy compatibility -> explicit `legacy-compat` accept;
- required-mode legacy reject -> suite-required v1 reject;
- unsupported/downgrade/stripped/duplicate/unknown/noncanonical/malformed
  rejects -> parser/admission negative fixtures;
- mismatch A1/B1 and B1/A2 -> byte-exact pending-context comparisons;
- transcript binding -> B1 transcript-field mutation reject;
- key-context binding -> missing-context pending reject;
- no accepted-state mutation, no output, and no secret leak -> harness
  assertions before final markers.

`formal/model_qsc_handshake_suite_id_bounded.py` and
`formal/run_model_checks.py` remain green with the implementation.

## NA-0310 vector/refimpl cross-check

The harness parses `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
and asserts all 20 NA-0310 vector categories are present. The implemented
runtime cases cover the vector categories in qsc where they are representable.

The NA-0310 refimpl oracle
`tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
remains green and continues to validate vector metadata, categories,
deterministic reasons, no-mutation/no-output/no-leak expectations, transcript
and key-context labels, and future-harness expectations.

## NA-0311 harness requirement cross-check

NA-0311 required a real qsc CLI/relay harness rather than synthetic parse-only
bytes. NA-0313 uses the qsc binary, isolated config roots, the existing local
inbox server, real identities, contact pins, route-token configuration, vault
state, relay-carried A1/B1/A2 bytes, and stored qsp session-state inspection.

The harness therefore proves the shared parser/encoder/admission path used by
the CLI transport surface, not only a detached fixture parser.

Existing NA-0302, NA-0303, and NA-0304 qsc harnesses remain green as regression
checks. NA-0304 is treated as a historical/default-legacy seam regression: its
pre-NA-0313 marker names are not used as v2 suite-context evidence, and the
NA-0313 harness is the evidence for the new explicit v2 path.

## Limitations

- Only the Suite-2 tuple `0x0500 / 0x0002` is admitted in `QHSM` v2.
- The parameter-block surface is intentionally bounded and is not a general
  extensible negotiation registry.
- Unknown noncritical parameters are rejected by default so acceptance cannot
  silently ignore unmodeled semantics.
- The CLI default remains the named compatibility mode for existing behavior;
  suite-required evidence requires explicit `--suite-mode suite-required`.
- This lane does not implement metadata runtime identifier rotation, default
  padding runtime behavior, broader sanitized-error runtime expansion,
  retention/purge runtime behavior, traffic shaping, batching, or cover traffic.
- This lane does not change dependencies, QSP core implementation, qsl-server,
  qsl-attachments, qsc-desktop, website sources, public docs, README, or
  START_HERE.

## Metadata successor decision

The metadata evidence reviewed for NA-0313 records that runtime identifier
rotation and runtime default padding remain open. NA-0291 and NA-0293 prove
policy fixtures only; they do not implement runtime metadata reduction.

Because NA-0313 succeeded without a qsc prerequisite blocker, the selected
successor is:

NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan

The next lane should begin as a design/transition plan unless later live scope
explicitly authorizes runtime implementation.

## Backup-plan impact statement

All durable artifacts changed by this lane are tracked qsl-protocol files under
`/srv/qbuild/work`. No new durable evidence location outside the current qbuild
worktree scope is introduced. No backup-plan update is required.

## No broader runtime/protocol/crypto/dependency/website change proof

The implementation touches only the authorized qsc handshake/CLI/test paths and
named governance files. It does not touch:

- qsp implementation files;
- qsl-server or qsl-attachments implementation files;
- qsc-desktop files;
- website or external website files;
- README or START_HERE;
- public docs;
- `.github` workflows;
- `Cargo.toml` or `Cargo.lock`;
- dependency configuration;
- branch-protection or public-safety configuration.

The qsc handshake change is bounded to D-0603: a `QHSM` v2 suite-context
admission surface with transcript/context binding. No unbounded key schedule
work or broader crypto state-machine change is introduced.

## Next recommendation

After PR merge and green post-merge public-safety, close out NA-0313 and
restore exactly one READY successor:

NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan
