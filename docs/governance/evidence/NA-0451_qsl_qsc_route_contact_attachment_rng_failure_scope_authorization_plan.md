Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0451 QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0451 consumes the NA-0450 residual RNG triage and authorizes the next exact
implementation successor for selected qsc route, contact, and attachment RNG
failure evidence.

Selected primary classification:

`ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTATION_READY`

Selected successor:

`NA-0452 -- QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Harness`

The route/contact/attachment implementation scope is small enough to combine if
it stays limited to:

- default route token failure during vault init;
- generated contact route-token failure in CLI and TUI contact-add flows;
- generated TUI relay inbox route-token failure during local bootstrap, scoped
  to the relay token write only;
- attachment ID, CEK, and nonce-prefix failures before staging/journal/send
  mutation.

The TUI account verification seed RNG call is not selected for the combined
implementation successor because it occurs after earlier account-default writes.
It remains an account-bootstrap residual unless a later directive authorizes
transactional bootstrap behavior or a narrower invariant.

NA-0451 performs no implementation mutation. It adds no tests, no test seams,
no runtime behavior, no crypto behavior, no dependency changes, no Cargo
manifest changes, no lockfile changes, no workflow changes, no fuzz target
changes, no vector changes, no formal model changes, no qsl-server changes, no
qsl-attachments changes, no qshield runtime changes, no qshield-cli changes, no
website changes, no README changes, and no START_HERE changes.

No RNG-failure-complete claim is made. Cargo audit green is dependency-health
evidence only.

## Live NA-0451 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

Status: READY.

Allowed mutation paths for this evidence PR:

- `docs/governance/evidence/NA-0451_qsl_qsc_route_contact_attachment_rng_failure_scope_authorization_plan.md`
- `tests/NA-0451_qsl_qsc_route_contact_attachment_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection included qsc source/tests/fuzz, refimpl source, qshield-cli
demo boundary files, formal models, inputs, governance evidence, scripts,
workflows, and the qwork proof files.

Forbidden mutation scope:

- runtime source;
- crypto source;
- dependency metadata;
- Cargo manifests;
- lockfiles;
- workflows;
- executable tests;
- fuzz targets;
- vectors;
- formal models;
- qsl-server;
- qsl-attachments;
- qshield runtime;
- qshield-cli;
- website;
- public docs;
- README;
- START_HERE;
- qwork/qstart/qresume/qshell;
- qsl-backup;
- backup status or plan files;
- rollback subtree or `/backup/qsl`.

Acceptance criteria:

- NA-0450 inheritance is consumed;
- route/contact/attachment RNG surfaces are classified;
- exact future implementation paths are selected;
- no implementation mutation occurs;
- root cargo audit remains green;
- nested qsc fuzz lock audit remains green;
- inherited qsc RNG, zeroization, provider-error, send_commit, refimpl, and
  formal checks remain green or are recorded with bounded local-tool caveats;
- public-safety is green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- missing, malformed, stale, or inconsistent qwork proof;
- PR #1170 not merged at the expected merge commit;
- queue not READY NA-0451 at start;
- D-0888 absent or D-0889 present at start;
- root or nested cargo audit failure;
- route/contact/attachment RNG scope cannot be safely classified;
- successor cannot be selected safely;
- public-safety red or missing;
- scope drift into forbidden paths;
- backup, restore, qwork, qstart, or qresume execution by Codex;
- public-claim expansion.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read proof files:

- `/srv/qbuild/work/NA-0451/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0451/.qwork/startup.qsl-protocol.json`

Proof values verified:

- `startup_result=OK`
- `lane=NA-0451`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0451/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0451`
- `requested_lane_status=READY`

Freshness proof:

- qwork proof head matched live `HEAD` before fetch.
- qwork proof `origin/main` matched live `origin/main` before fetch.
- Fetched `origin/main` remained `5b15748c0aec` and did not advance beyond
  the qwork proof.
- `origin/main` equals and descends from PR #1170 merge commit
  `5b15748c0aec`.
- PR #1170 was verified MERGED with merge commit `5b15748c0aec`.

Proof root:

`/srv/qbuild/tmp/NA0451_qsc_route_contact_attachment_rng_scope_20260609T162307Z`

## NA-0450 inheritance

NA-0450 selected:

`RNG_RESIDUAL_TRIAGE_ROUTE_CONTACT_ATTACHMENT_NEXT`

Selected successor inherited by this lane:

`NA-0451 -- QSL qsc Route / Contact / Attachment RNG Failure Scope Authorization Plan`

NA-0450 residual surfaces consumed here:

- route/default-route/relay token RNG;
- contact route-token RNG;
- attachment ID RNG;
- attachment CEK RNG;
- attachment nonce-prefix RNG.

Important inherited boundary:

- Provider-dependent qsc RNG remains separate.
- refimpl/provider RNG remains separate.
- qshield-cli demo RNG remains demo-local backlog.
- formal/model RNG remains supporting/backlog.
- fuzz/vector RNG remains supporting/backlog.
- The TUI account verification seed is an account-bootstrap residual unless a
  later exact directive scopes its partial-write semantics.

NA-0449 inheritance remains bounded:

- the cfg seam pattern `qsc_rng_failure_test_seam` exists;
- the selector `QSC_RNG_FAILURE_TEST_SEAM` is read only under the cfg;
- normal builds without the cfg keep production `OsRng` behavior;
- default route token generation in `vault/mod.rs` already has a cfg label, but
  NA-0449 did not add an executable test for that label.

## Applicable Stewardship Review

Level 1 stewardship review is active for this evidence lane.

Level 2 and Level 3 stewardship rollout remain future-gated.

No separate Directors are created. No steward has independent READY promotion
authority. No steward has independent merge authority. Lead Director final
authority is preserved.

### Crypto / Protocol Steward

Route/contact/attachment RNG failure scope remains bounded to qsc-owned
test-only seam evidence. A combined implementation lane is safe only if it
uses exact labels, preserves production semantics without the cfg, and proves
failure before selected route/contact/attachment state or output mutation.

The account verification seed is not selected for the combined implementation
because its failure point follows earlier account-default writes. No RNG-failure-complete claim is made.

### CI / Dependency / Release Health Steward

Root `cargo audit --deny warnings` is green.

Nested qsc fuzz lock `cargo audit --deny warnings --file
qsl/qsl-client/qsc/fuzz/Cargo.lock` is green.

Cfg RNG failure tests are green. Normal no-cfg RNG failure tests are green. qsc
key lifecycle and provider-error tests are green. qsc adversarial smoke is
green on current main. Public-safety is green on current main.

Cargo audit green is dependency-health evidence only.

No cargo audit output is used as public-readiness proof.
No cargo audit output is used as production-readiness proof.
No cargo audit output is used as public-internet readiness proof.
No cargo audit output is used as external-review-complete proof.
No cargo audit output is used as crypto-complete proof.
No cargo audit output is used as RNG-failure-complete proof.
No cargo audit output is used as vulnerability-free proof.
No cargo audit output is used as bug-free proof.
No cargo audit output is used as perfect-crypto proof.
No cargo audit output is used as side-channel-free proof.

### Public Claims / External Review Steward

No RNG-failure-complete claim is made.

No crypto-complete claim is made.

No side-channel-free claim is made.

No vulnerability-free claim is made.

No bug-free claim is made.

No perfect-crypto claim is made.

No public-readiness claim is made.

No production-readiness claim is made.

No external-review-complete claim is made.

Route/contact/attachment scope authorization is internal governance evidence
only.

### Product / Demo / Service Boundary Steward

qshield-cli remains demo-local and out of scope.

qsl-server remains a service boundary and out of scope.

qsl-attachments remains a service boundary and out of scope.

No qshield, website, qsl-server, or qsl-attachments public-service readiness
claim is made.

### Local Ops / Backup / Restore Steward

No backup, restore, or local-ops mutation is authorized or performed.

The qsl-backup proof is boundary evidence only. The installed qsl-backup SHA
matched the directive value `e9ecff3d22ed`, and the Codex ops source-list
inclusion count was exactly one.

## Route RNG failure scope review

Read-only source evidence:

- `qsl/qsl-client/qsc/src/vault/mod.rs` generates the default route token during
  vault init. The cfg label `QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN` already exists
  and the call occurs before payload serialization and before vault file or temp
  file creation.
- `qsl/qsl-client/qsc/src/contacts/mod.rs` contains
  `generate_route_token()`, which fills 32 bytes from `OsRng` and hex-encodes
  them.
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs` calls
  `generate_route_token()` when a TUI contact is added without an explicit
  route token.
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` calls
  `generate_route_token()` for the TUI relay inbox route token during local
  bootstrap.

Route/default-route/relay-token RNG calls remaining after NA-0449:

- default route token during vault init;
- CLI contact-add route token;
- TUI contact-add route token;
- TUI relay inbox route token during account bootstrap.

NA-0449 coverage:

- The default vault route-token source label exists.
- NA-0449 did not prove the default route-token failure label in an executable
  test.
- Contacts and TUI route-token calls are not labeled today.

Future label plan:

- Reuse `QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN`.
- Add `QSC.CONTACT.ROUTE_TOKEN`.
- Add `QSC.TUI.CONTACT.ROUTE_TOKEN`.
- Add `QSC.TUI.RELAY_INBOX_ROUTE_TOKEN`.

Future no-partial-state proof:

- default route token: prove forced failure leaves no vault file and no temp
  vault file;
- CLI contact route token: prove forced failure occurs before
  `contacts_entry_upsert`;
- TUI contact route token: prove forced failure occurs before
  `state.contacts_records` insertion and before `persist_contacts_cache`;
- TUI relay inbox route token: prove forced failure occurs before the relay
  inbox token secret write. This is not a full account-bootstrap no-mutation
  proof because earlier account defaults may already be written.

Classification:

`ROUTE_RNG_IMPLEMENTATION_READY`

The implementation-ready classification is scoped to route-token labels only.
The TUI account verification seed remains deferred.

## Contact RNG failure scope review

Read-only source evidence:

- `qsl/qsl-client/qsc/src/contacts/mod.rs` generates a contact route token when
  `contacts_add` receives no explicit route token.
- The generated token is assigned into the contact record and primary device
  record before `contacts_entry_upsert`.
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs` generates a
  contact route token for TUI contact-add when no explicit route token is
  provided, before inserting into the in-memory contact cache and before
  persistence.

Contact RNG calls:

- CLI contact-add generated route token.
- TUI contact-add generated route token.

The calls are related to contact route-token material, not invitations or
provider-generated keys.

Future no-partial-contact-state proof is clear because randomness is drawn
before contact upsert or TUI contact-cache insertion.

The existing cfg seam pattern is reusable with contact-specific labels. The
future implementation should avoid deterministic fallback bytes and should fail
closed only under the custom cfg.

Classification:

`CONTACT_RNG_IMPLEMENTATION_READY`

## Attachment RNG failure scope review

Read-only source evidence:

- `qsl/qsl-client/qsc/src/attachments/mod.rs` has
  `attachment_generate_id()`, which fills 32 bytes from `OsRng` and hex-encodes
  the attachment ID.
- `attachment_build_enc_ctx()` fills a 32-byte CEK and 8-byte nonce prefix from
  `OsRng`.
- `attachment_build_outbound_record()` calls `attachment_generate_id()` and
  `attachment_build_enc_ctx()` before staging directory selection, ciphertext
  file creation, journal insertion, service session creation, upload, commit,
  or descriptor send.

Attachment RNG calls:

- attachment ID;
- attachment CEK;
- attachment nonce prefix.

Future labels:

- `QSC.ATTACHMENT.ID`
- `QSC.ATTACHMENT.CEK`
- `QSC.ATTACHMENT.NONCE_PREFIX`

Future no-partial-state proof:

- force ID failure and prove no staged ciphertext file, no journal record, and
  no send/service mutation;
- force CEK failure and prove no staged ciphertext file, no journal record, and
  no send/service mutation;
- force nonce-prefix failure and prove no staged ciphertext file, no journal
  record, and no send/service mutation.

The existing cfg seam pattern is reusable. Attachment-only implementation would
also be safe, but combined route/contact/attachment remains small enough after
deferring account verification seed semantics.

Classification:

`ATTACHMENT_RNG_IMPLEMENTATION_READY`

## Combined vs split scope decision

### Option 1 - combined route/contact/attachment implementation

Recommendation: select.

Evidence:

- All selected calls are qsc-owned `OsRng.fill_bytes` route/contact/attachment
  material.
- Exact source paths are clear.
- Existing `qsc_rng_failure_test_seam` pattern is reusable.
- Future tests can prove no selected route/contact/attachment write/output
  after forced failure.
- Production semantics can be proven unchanged by running the future test
  without `--cfg qsc_rng_failure_test_seam`.

Future paths are listed in the exact future scope candidate below.

Public-claim caveat:

- bounded internal evidence only;
- no RNG-failure-complete claim;
- no public readiness claim.

### Option 2 - attachment-only implementation first

Recommendation: reject as primary.

Evidence:

- Attachment ID/CEK/prefix is the cleanest individual sub-scope.
- However, route and contact token paths are also clear after deferring the
  account verification seed. Splitting would add queue churn without reducing
  enough risk.

### Option 3 - route-only implementation first

Recommendation: reject as primary.

Evidence:

- Route-token labels are clear, but contact and attachment labels are equally
  ready.

### Option 4 - contact-only implementation first

Recommendation: reject as primary.

Evidence:

- Contact route-token labels are clear, but route and attachment labels are
  equally ready.

### Option 5 - split-scope triage

Recommendation: reject as primary, preserve one deferred sub-scope.

Evidence:

- Split-scope triage is not needed for the selected route/contact/attachment
  labels.
- The account verification seed is explicitly deferred as an account-bootstrap
  residual, so it does not block the combined implementation.

### Option 6 - provider-boundary or other RNG residual takes priority

Recommendation: reject.

Evidence:

- Provider-dependent RNG remains important but crosses qsc/refimpl provider
  contracts and is less cohesive than the qsc-owned route/contact/attachment
  labels.

## Exact future scope candidate

Selected future implementation paths:

- `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`

Selected future governance paths:

- `docs/governance/evidence/NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

For each selected source path:

| Path | RNG call(s) to label | Label naming | Future invariant | Future marker | Production semantics proof |
|---|---|---|---|---|---|
| `qsl/qsl-client/qsc/src/vault/mod.rs` | default route token during vault init | reuse `QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN` | forced failure occurs before vault payload/file/temp-file commit | `NA0452_ROUTE_DEFAULT_TOKEN_RNG_FAILURE_NO_VAULT_WRITE_OK` | normal test without cfg proves selector inert |
| `qsl/qsl-client/qsc/src/contacts/mod.rs` | CLI contact-add route-token generation | `QSC.CONTACT.ROUTE_TOKEN` | forced failure occurs before `contacts_entry_upsert` | `NA0452_CONTACT_ROUTE_TOKEN_RNG_FAILURE_NO_CONTACT_WRITE_OK` | normal test without cfg proves selector inert |
| `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs` | TUI contact-add route-token generation | `QSC.TUI.CONTACT.ROUTE_TOKEN` | forced failure occurs before TUI contact-cache insertion/persistence | `NA0452_TUI_CONTACT_ROUTE_TOKEN_RNG_FAILURE_NO_CONTACT_WRITE_OK` | normal test without cfg proves selector inert |
| `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | TUI relay inbox route-token generation | `QSC.TUI.RELAY_INBOX_ROUTE_TOKEN` | forced failure occurs before relay inbox token secret write; not a full account-bootstrap no-mutation proof | `NA0452_TUI_RELAY_INBOX_ROUTE_TOKEN_RNG_FAILURE_NO_ROUTE_SECRET_WRITE_OK` | normal test without cfg proves selector inert |
| `qsl/qsl-client/qsc/src/attachments/mod.rs` | attachment ID, CEK, nonce prefix | `QSC.ATTACHMENT.ID`; `QSC.ATTACHMENT.CEK`; `QSC.ATTACHMENT.NONCE_PREFIX` | forced failure occurs before staged ciphertext, journal record, service session, upload, commit, or descriptor send | `NA0452_ATTACHMENT_RNG_FAILURE_NO_STAGE_OR_JOURNAL_WRITE_OK` | normal test without cfg proves selector inert |

The future implementation must not select the TUI account verification seed
unless a later exact directive expands scope. If a future attempt cannot keep
the account seed out of the combined path, it must stop.

## Authorization matrix

| Surface | Path(s) | RNG role | Existing NA-0449 seam coverage? | New seam label needed? | Existing API enough? | Future test path | Future source path(s) | Implementation readiness | Risk | Priority | Selected for successor? | Public-claim implication | Goals |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| route/default-route/relay RNG | `qsl/qsl-client/qsc/src/vault/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs` | default route token and TUI relay inbox route token | partial for vault source label only; no executable proof | yes for TUI relay label; reuse vault label | yes with cfg seam and exact tests | `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs` | `vault/mod.rs`; `tui/controller/commands/locked.rs` | `ROUTE_RNG_IMPLEMENTATION_READY` for route-token labels only | medium | high | yes | no RNG-failure-complete claim | G2, G4, G5 |
| contact RNG | `qsl/qsl-client/qsc/src/contacts/mod.rs`; `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs` | generated contact route tokens | no | yes | yes with cfg seam and exact tests | `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs` | `contacts/mod.rs`; `tui/controller/commands/contacts.rs` | `CONTACT_RNG_IMPLEMENTATION_READY` | medium | high | yes | no public-readiness claim | G2, G4, G5 |
| attachment ID RNG | `qsl/qsl-client/qsc/src/attachments/mod.rs` | outbound attachment ID | no | yes | yes with cfg seam and exact tests | `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs` | `attachments/mod.rs` | `ATTACHMENT_RNG_IMPLEMENTATION_READY` | medium/high | high | yes | no production-readiness claim | G1, G2, G4, G5 |
| attachment CEK RNG | `qsl/qsl-client/qsc/src/attachments/mod.rs` | content encryption key | no | yes | yes with cfg seam and exact tests | `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs` | `attachments/mod.rs` | `ATTACHMENT_RNG_IMPLEMENTATION_READY` | medium/high | high | yes | no crypto-complete claim | G1, G2, G4 |
| attachment nonce-prefix RNG | `qsl/qsl-client/qsc/src/attachments/mod.rs` | per-part nonce prefix | no | yes | yes with cfg seam and exact tests | `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs` | `attachments/mod.rs` | `ATTACHMENT_RNG_IMPLEMENTATION_READY` | medium/high | high | yes | no side-channel-free claim | G1, G2, G4, G5 |
| provider-dependent RNG residual | `qsl/qsl-client/qsc/src/identity/mod.rs`; `qsl/qsl-client/qsc/src/handshake/mod.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` | provider keypair/encapsulation randomness | no | not selected | no, provider boundary needed | future provider-boundary path only if later selected | future provider-boundary paths only | backlog/separate | medium/high | medium | no | no vulnerability-free claim | G1, G2, G4 |
| refimpl provider RNG residual | `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`; `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs` | provider randomness and `Rng12` behavior | no | not selected | no, trait/provider boundary needed | future provider-boundary path only if later selected | future refimpl provider paths only | backlog/separate | medium/high | medium | no | no perfect-crypto claim | G1, G2, G4 |
| qshield-cli demo RNG residual | `apps/qshield-cli/` | demo-local randomness boundary | no | not selected | no immediate qsc seam relevance | none selected | none selected | demo-local backlog | low/medium | low | no | no public-readiness claim | G4, G5 |
| formal/fuzz/vector residual | `formal/`; `qsl/qsl-client/qsc/fuzz/`; `inputs/` | supporting model/fuzz/vector RNG evidence | no | not selected | no, supporting after executable scope | none selected | none selected | supporting/backlog | medium | medium | no | no RNG-failure-complete claim | G4 |

## Authorization decision

Primary classification:

`ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTATION_READY`

Selected highest-priority successor:

`NA-0452 -- QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Harness`

This directive authorizes no implementation mutation. It does not mutate
runtime source, crypto source, dependencies, Cargo manifests, lockfiles,
workflows, executable tests, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, or
START_HERE.

Future implementation must be exact and separately authorized by NA-0452.

No public claim expansion is authorized.

Exactly one READY successor remains mandatory.

## Successor selection

Selected NA-0452:

`NA-0452 -- QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Harness`

Rationale:

- combined selected labels are exact;
- selected source paths are clear;
- cfg seam reuse is safe if normal no-cfg behavior remains unchanged;
- no dependency, Cargo, lockfile, workflow, service, backup, website, or public
  claim mutation is needed;
- the one ambiguous sub-surface, TUI account verification seed, is explicitly
  deferred rather than forced into the implementation lane.

Do not implement NA-0452 in this directive.

## Future path/scope bundle

Future NA-0452 allowed implementation paths:

- `qsl/qsl-client/qsc/tests/rng_failure_residual_surfaces.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/src/contacts/mod.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`

Future NA-0452 allowed governance paths:

- `docs/governance/evidence/NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0452_qsl_qsc_route_contact_attachment_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden unless exact scope authorizes:

- runtime/crypto implementation changes outside exact selected paths;
- dependency changes;
- Cargo/lockfile changes;
- workflow changes;
- test source changes outside exact selected paths;
- fuzz target source changes;
- vector changes;
- formal model changes;
- public docs/website;
- qsl-server/qsl-attachments changes;
- backup/restore/qsl-backup changes;
- public claims.

## Future validation/marker plan

Common NA-0452 markers:

- `NA0452_RNG_RESIDUAL_AUTHORIZATION_CONSUMED_OK`
- `NA0452_NEXT_SCOPE_SELECTED_OK`
- `NA0452_NO_DEPENDENCY_CHANGE_OK`
- `NA0452_NO_WORKFLOW_CHANGE_OK`
- `NA0452_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0452_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0452_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0452_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0452_ONE_READY_INVARIANT_OK`

Combined implementation markers:

- `NA0452_ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTED_OK`
- `NA0452_ROUTE_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_CONTACT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_ATTACHMENT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0452_ROUTE_DEFAULT_TOKEN_RNG_FAILURE_NO_VAULT_WRITE_OK`
- `NA0452_CONTACT_ROUTE_TOKEN_RNG_FAILURE_NO_CONTACT_WRITE_OK`
- `NA0452_TUI_CONTACT_ROUTE_TOKEN_RNG_FAILURE_NO_CONTACT_WRITE_OK`
- `NA0452_TUI_RELAY_INBOX_ROUTE_TOKEN_RNG_FAILURE_NO_ROUTE_SECRET_WRITE_OK`
- `NA0452_ATTACHMENT_RNG_FAILURE_NO_STAGE_OR_JOURNAL_WRITE_OK`

Future required validation:

- `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture`
- inherited `rng_failure_behavior` cfg and normal tests;
- inherited key lifecycle and provider-error tests;
- qsc `send_commit`;
- refimpl `pqkem768`;
- root and nested cargo audit;
- qsc adversarial syntax and CI smoke;
- formal model checks;
- public-safety before and after merge.

## Public claim/external review/website boundary

Route/contact/attachment RNG scope authorization is internal governance evidence
only.

Route/contact/attachment RNG scope authorization is not production readiness.

Route/contact/attachment RNG scope authorization is not public-internet
readiness.

Route/contact/attachment RNG scope authorization is not crypto-complete proof.

Route/contact/attachment RNG scope authorization is not side-channel-free
proof.

Route/contact/attachment RNG scope authorization is not RNG-failure-complete
proof.

Route/contact/attachment RNG scope authorization is not bug-free proof.

Route/contact/attachment RNG scope authorization is not vulnerability-free
proof.

Route/contact/attachment RNG scope authorization is not perfect-crypto proof.

Route/contact/attachment RNG scope authorization is not public technical paper
content.

No README, START_HERE, docs-public, or website update is made.

No public-readiness or public-security claim is made.

Cargo audit green is dependency-health evidence, not vulnerability-free proof.

Future tests, if authorized, must be described as bounded evidence only.

## Rejected alternatives

`ATTACHMENT_RNG_IMPLEMENTATION_READY`

Rejected as primary because attachment is ready but route/contact token labels
are also exact after deferring account verification seed semantics.

`ROUTE_RNG_IMPLEMENTATION_READY`

Rejected as primary because route-only work would leave equally clear contact
and attachment labels for another lane.

`CONTACT_RNG_IMPLEMENTATION_READY`

Rejected as primary because contact-only work would leave equally clear route
and attachment labels for another lane.

`ROUTE_CONTACT_ATTACHMENT_RNG_SPLIT_SCOPE_NEEDED`

Rejected as primary because split scope is not needed for selected labels. The
one ambiguous account-bootstrap seed is deferred explicitly.

`PROVIDER_RNG_BOUNDARY_NEXT`

Rejected as immediate successor because provider-dependent RNG crosses
qsc/refimpl provider contracts and may require a separate boundary lane.

`ROUTE_CONTACT_ATTACHMENT_RNG_DOCUMENTATION_ONLY`

Rejected because executable qsc RNG failure evidence remains a meaningful
bounded verification gap.

`ROUTE_CONTACT_ATTACHMENT_RNG_BACKLOG`

Rejected because the selected paths are exact enough for a future
implementation lane.

`ROUTE_CONTACT_ATTACHMENT_RNG_AMBIGUOUS`

Rejected because ambiguity is isolated to the deferred account verification
seed, not the selected route/contact/attachment labels.

## Backup-impact statement

No backup was run.

No restore was run.

No sudo was run.

No qsl-backup mutation occurred.

No backup status file or backup plan file mutation occurred.

No rollback subtree or `/backup/qsl` mutation occurred.

The qsl-backup SHA/source-list proof is boundary evidence only.

## Next recommendation

Proceed to closeout only after the NA-0451 evidence PR merges and post-merge
public-safety is green.

Restore exactly one READY successor:

`NA-0452 -- QSL qsc Route / Contact / Attachment RNG Failure Test Seam Implementation Harness`

NA-0452 should implement only the exact paths and markers listed here. It
should stop if implementation requires the deferred account verification seed,
provider/refimpl RNG, qshield-cli, formal/fuzz/vector, dependency, Cargo,
lockfile, workflow, qsl-server, qsl-attachments, website, public-doc, backup,
restore, or qsl-backup scope.
