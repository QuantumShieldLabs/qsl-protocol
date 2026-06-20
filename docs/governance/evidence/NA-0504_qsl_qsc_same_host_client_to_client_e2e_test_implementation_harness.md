Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0504 qsc Same-Host Client-to-Client E2E Test Implementation Harness

## Executive summary

NA-0504 consumes NA-0503/D389 inheritance and implements the selected
same-host qsc Alice/Bob integration test at
`qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`.

The test uses two independent same-host temp client roots, initializes Alice
and Bob identities, exchanges public identity fingerprints through trusted
contact/device records, completes a local relay-backed handshake, sends from
Alice to Bob, receives on Bob, sends a reply from Bob to Alice, receives on
Alice, checks captured stdout/stderr/diagnostics for secret-shaped output, and
exercises a wrong-mailbox reject path that does not mutate selected receive or
session artifacts.

This is bounded local implementation evidence only. no public-readiness claim
is made. no production-readiness claim is made. no public-internet-readiness
claim is made. no external-review-complete claim is made. no crypto-complete
claim is made. no replay-proof claim is made. no downgrade-proof claim is made.
no secret-material-complete claim is made. no zeroization-complete claim is
made. no memory-erasure-complete claim is made. no side-channel-free claim is
made. no vulnerability-free claim is made. no bug-free claim is made. no
perfect-crypto claim is made.

## Live NA-0504 scope

Allowed implementation path:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_harness.md`
- `tests/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0504 does not mutate qsc source, existing qsc tests, qsc fuzz targets,
Cargo manifests, lockfiles, dependencies, workflows, scripts, helpers, corpus
files, vector files, input files, formal models, refimpl files, services,
public docs, backup paths, qwork/qstart/qresume tooling, or qsl-backup.

## qwork proof-file verification

- Codex did not run `qwork`, `qstart`, or `qresume`.
- qwork proof files existed under `/srv/qbuild/work/NA-0504/.qwork/` and were
  copied into the directive proof root
  `/srv/qbuild/tmp/NA0504_same_host_client_to_client_e2e_impl_20260620T014854Z/`.
- `.kv` and `.json` proof agreed on `startup_result=OK`, `lane=NA-0504`,
  `repo=qsl-protocol`, path `/srv/qbuild/work/NA-0504/qsl-protocol`,
  clean worktree/index/untracked state, `head_equals_origin_main=yes`,
  `ready_count=1`, `queue_top_ready=NA-0504`, and
  `requested_lane_status=READY`.
- Proof HEAD and proof `origin_main` matched live pre-fetch refs at
  `bcc5825a661a`.
- Fetch was performed only after proof/live ref match and disk threshold proof.
- `origin/main` equals `bcc5825a661a`, which satisfies the expected ancestry.
- Disk proof before fetch: `/` usage was 73.24%, below the 95% stop threshold.
- Current main protection proof before mutation: `public-safety` completed
  success, `qsc-adversarial-smoke` completed success, `qsc-linux-full-suite`
  completed skipped by policy, `macos-qsc-full-serial` completed skipped by
  policy, and no completed red check was observed.
- qsl-backup boundary proof: installed helper SHA-256 matched
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`;
  the Codex ops source inclusion count was exactly 1. Codex did not run backup
  or restore.

Recovered proof-shape issue: the first queue/decision parser counted
historical decision references and matched a stale READY-like block. This was
classified as recoverable command/proof-shape error. Corrective action: reran
using live `### NA-...` block parsing and DECISIONS `- **ID:** D-....`
decision-record counting. Final corrected result: READY_COUNT 1, READY
NA-0504, NA-0503/NA-0502/NA-0501 DONE, D-0995 once, D-0996 once, D-0997
absent, duplicate decision record count zero.

## NA-0503 / D389 inheritance

- NA-0503 completed and was closed.
- NA-0504 was restored READY as the sole READY item.
- Selected classification:
  `SAME_HOST_CLIENT_TO_CLIENT_E2E_IMPLEMENTATION_READY`.
- Selected future path:
  `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`.
- D389 response exists at
  `/home/victor/work/qsl/codex/responses/NA0503_20260620T013404Z_D389.md`.
- Current qsc surfaces support two independent same-host qsc client roots,
  identity setup, public-record/trusted-pin exchange, send/receive, reply,
  diagnostics capture, no-secret-output assertions, and negative
  reject/no-mutation behavior through existing qsc CLI/test-visible patterns.
- Remote/LAN/two-machine setup remains deferred.
- No qsc source, dependency, workflow, corpus, vector, input, formal, refimpl,
  service, public, or backup mutation was selected.
- no public-readiness claim is inherited. no production-readiness claim is
  inherited. no crypto-complete claim is inherited. no replay-proof claim is
  inherited. no downgrade-proof claim is inherited. no secret-material-complete
  claim is inherited. no zeroization-complete claim is inherited. no
  memory-erasure-complete claim is inherited. no side-channel-free claim is
  inherited.

## Pre-mutation E2E surface selection

The selected test file was absent before mutation:
`qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`.

Read-only inspection covered:

- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/receive_e2e.rs`
- `qsl/qsl-client/qsc/tests/handshake_mvp.rs`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_ux.rs`
- `qsl/qsl-client/qsc/tests/trust_onboarding_mainstream_flow_na0187.rs`
- `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`
- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- qsc source command names and artifact names as read-only reference.

Candidate implementation classifications:

| Candidate surface | Classification | Selection note |
| --- | --- | --- |
| Two independent roots via `common::TestIsolation` | FEASIBLE_DIRECT_QSC_E2E_SURFACE | Selected; Alice and Bob each get an independent `TestIsolation` root and `QSC_CONFIG_DIR`. |
| Alice/Bob identity setup through `identity rotate` and `identity show` | FEASIBLE_DIRECT_QSC_E2E_SURFACE | Selected; uses real qsc CLI identity records. |
| Public-record/trusted-pin exchange through contacts/device trust | FEASIBLE_DIRECT_QSC_E2E_SURFACE | Selected; exchanges fingerprints and trusts device records. |
| Local inbox/mock relay delivery through `start_inbox_server` | FEASIBLE_LOCAL_INBOX_SURFACE | Selected; same-host loopback only. |
| Alice send / Bob receive | FEASIBLE_DIRECT_QSC_E2E_SURFACE | Selected. |
| Bob reply / Alice receive | FEASIBLE_DIRECT_QSC_E2E_SURFACE | Selected. |
| Wrong mailbox reject/no-mutation | FEASIBLE_DIRECT_QSC_E2E_SURFACE | Selected as negative boundary. |
| Corrupt/replay/stale delivery | SUPPORTING_ONLY_PATH | Existing tests cover related behavior; not selected for this first E2E file. |
| Remote/LAN/two-machine setup | REQUIRES_REMOTE_OR_LAN_SETUP | Deferred by scope. |
| qsc source/helper/dependency/workflow changes | REQUIRES_QSC_SOURCE_MUTATION or REQUIRES_DEPENDENCY_OR_CARGO_MUTATION | Rejected by scope. |

## qsc integration test implementation

`qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs` adds four tests:

- `same_host_alice_bob_send_receive_reply_flow`
- `same_host_e2e_negative_reject_does_not_mutate_state`
- `same_host_e2e_outputs_do_not_expose_secret_markers`
- `na0504_common_no_overclaim_markers`

The first implementation run failed because the new test passed the local self
label to `receive --from`, while qsc receive uses that argument as the
peer/channel selector for authenticated qsp unpack. Classification:
recoverable in-scope local test/command-shape failure. Corrective action:
patched only the new test file so Bob receives from `alice` and Alice receives
from `bob`. Final result: focused NA-0504 test passed.

## two independent client roots proof

The new test constructs Alice and Bob through separate `common::TestIsolation`
instances and asserts:

- Alice root differs from Bob root.
- Alice `QSC_CONFIG_DIR` differs from Bob `QSC_CONFIG_DIR`.
- Each qsc command applies its matching isolation before execution.

Marker emitted:

- `NA0504_TWO_INDEPENDENT_CLIENT_ROOTS_OK`

## identity / public-record / trust proof

The test initializes Alice and Bob with `identity rotate --confirm`, reads real
qsc identity fingerprints with `identity show`, adds each peer through
`contacts add --fp --route-token`, lists the created contact device, and trusts
that device with `contacts device trust --confirm`.

Markers emitted:

- `NA0504_ALICE_BOB_IDENTITY_SETUP_OK`
- `NA0504_PUBLIC_RECORD_TRUST_EXCHANGE_OK`

## send/receive flow proof

After local relay-backed handshake completion, Alice sends a synthetic payload
to Bob with qsc `send --transport relay`. Bob receives from Alice using qsc
`receive --transport relay`, and the test verifies the received file bytes.

Marker emitted:

- `NA0504_SEND_RECEIVE_FLOW_OK`

## reply flow proof

Bob sends a synthetic reply to Alice with qsc `send --transport relay`. Alice
receives from Bob using qsc `receive --transport relay`, and the test verifies
the received file bytes.

Marker emitted:

- `NA0504_REPLY_FLOW_OK`

## negative reject/no-mutation proof

The negative test establishes the same authenticated pair, queues an
Alice-to-Bob message, snapshots Bob's session artifact and receive-output
directory, then invokes qsc receive with an invalid mailbox. It asserts:

- the command fails closed;
- the output includes `recv_mailbox_invalid` or route-token invalid error;
- no receive artifact is created;
- Bob's selected session artifact bytes are unchanged;
- a later valid receive still commits the queued message.

Marker emitted:

- `NA0504_NEGATIVE_REJECT_NO_MUTATION_OK`

## stdout/stderr no-secret-output proof

The test captures selected qsc stdout/stderr/diagnostics from identity,
contact, handshake, send, receive, and reject commands. It scans them for
synthetic forbidden labels and high-entropy-looking output. The scanner is also
tested against synthetic forbidden markers so a superficial always-pass scanner
is rejected.

Marker emitted:

- `NA0504_STDOUT_STDERR_NO_SECRET_OUTPUT_OK`

## no remote SSH / no two-machine proof

The test uses only the existing same-host `start_inbox_server` loopback relay
and local qsc process invocations. It does not run SSH, mutate remote hosts,
create remote accounts, generate remote keys, require LAN setup, or require a
second machine.

Marker emitted:

- `NA0504_NO_REMOTE_SSH_SCOPE_OK`

## no qsc source / Cargo / dependency / workflow mutation proof

Changed qsc implementation scope is limited to the one new integration test
file. No qsc source, qsc fuzz target, qsc fuzz Cargo file, qsc fuzz lockfile,
root Cargo file, root lockfile, dependency, workflow, script, helper, validator
script, formal model, refimpl file, service file, public file, or backup file
is changed.

Markers emitted:

- `NA0504_NO_QSC_SOURCE_CHANGE_OK`
- `NA0504_NO_DEPENDENCY_CHANGE_OK`
- `NA0504_NO_WORKFLOW_CHANGE_OK`

## no corpus/vector/input mutation proof

NA-0504 does not add, edit, move, or delete corpus files, vector files, input
fixtures, internal manifests, fuzz targets, or qsc-adversarial assets. The
binding corpus and all qsc fuzz corpus validator scans passed as inherited
guardrail evidence.

## no public-readiness / production-readiness / crypto-complete / replay-proof / downgrade-proof / secret-material-complete / side-channel-free claim proof

The test emits no-overclaim markers and this evidence preserves claim
boundaries:

- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no KEM-complete claim is made.
- no signature-complete claim is made.
- no identity-complete claim is made.
- no provider-RNG-complete claim is made.
- no secret-material-complete claim is made.
- no zeroization-complete claim is made.
- no memory-erasure-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no side-channel-free claim is made.
- no vulnerability-free claim is made.
- no bug-free claim is made.
- no perfect-crypto claim is made.

## validation

Focused implementation validation passed:

```bash
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
```

Required inherited validation passed before governance authoring:

```bash
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

Full pre-PR validation remains required by the NA-0504 testplan before PR
creation and merge.

## scope guard

Expected implementation PR changed paths:

- `qsl/qsl-client/qsc/tests/same_host_client_to_client_e2e.rs`
- `docs/governance/evidence/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_harness.md`
- `tests/NA-0504_qsl_qsc_same_host_client_to_client_e2e_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## successor selection

Default successor after successful NA-0504:

`NA-0505 -- QSL Remote Client-to-Client Test Account / SSH Boundary Scope Authorization Plan`

Rationale: NA-0504 now provides bounded local same-host client-to-client
evidence. Before any remote/LAN client-to-client testing, the project needs a
least-privilege SSH/test-account boundary authorization lane. NA-0505 must be
authorization-only and must not create accounts, generate or install SSH keys,
run SSH, mutate remote hosts, or run remote tests.

## remote/LAN deferral

Remote SSH, remote account setup, host mutation, LAN testing, two-machine
testing, SSH config mutation, and remote service assumptions remain deferred to
a separate exact directive. NA-0504 does not authorize those actions.

## next recommendation

After NA-0504 implementation PR merge and green required checks, close out
NA-0504 and restore NA-0505 as the sole READY item if post-merge checks become
green inside the short attach/early-failure window. If post-merge checks remain
healthy but incomplete, hand off closeout rather than consuming a long wait.
