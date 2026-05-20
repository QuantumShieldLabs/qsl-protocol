Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0321 Metadata Runtime Timing and Traffic-Shape Threat Model

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0321 defines a timing and traffic-shape threat model plus future executable
evidence plan for the current metadata-runtime surfaces. It does not implement
runtime timing, jitter, batching, cover traffic, send scheduling, receive
scheduling, transport padding, service deployment behavior, or production
traffic shaping.

Current executable metadata-runtime proof is strongest for the bounded qshield
embedded relay/demo path:

- NA-0318 proves candidate fetch plus explicit ack/commit after local
  verification.
- NA-0319 proves bounded qshield candidate handles and default-padding bucket
  behavior.
- NA-0320 proves selected sanitized-error and retention/purge behavior.

That proof does not show that timing metadata is hidden. It does not show
anonymity, metadata-free behavior, untraceability, production readiness, public
internet readiness, or external review completion. qsl-server and
qsl-attachments production timing and traffic-shape behavior remain future
gates.

Selected successor:

`NA-0322 -- Metadata Runtime Timing and Traffic-Shape Measurement Harness`

## Live NA-0321 Scope

The live queue item is `NA-0321 -- Metadata Runtime Timing and Traffic-Shape
Threat Model / Executable Evidence Plan`, status `READY`, with goals G1 through
G5.

Allowed work:

- create a timing and traffic-shape threat model for bounded metadata-runtime
  surfaces;
- inventory executable evidence for timing, ordering, queue shape, send/receive
  cadence, padding, retention, and demo stress/soak;
- identify what remains unproven;
- propose future executable evidence without implementing runtime timing or
  traffic shaping;
- select the exact NA-0322 successor.

Forbidden work:

- runtime timing, jitter, batching, cover traffic, scheduling, or mitigation
  implementation;
- qshield, qsl-server, qsl-attachments, qsc, qsp, protocol, crypto, key
  schedule, dependency, workflow, public-safety, website, README, or START_HERE
  changes;
- any claim of anonymity, metadata-free behavior, untraceability, hidden timing
  metadata, production readiness, public internet readiness, or external review
  completion.

## Inherited NA-0318/NA-0319/NA-0320 Proof

NA-0318 established the bounded qshield embedded-relay candidate/ack boundary.
Candidate fetch returns queued messages without deletion. Explicit ack/commit
after local verification deletes exactly one candidate. Invalid receive reject
preserves the candidate, creates no accepted local state/output, and leaks no
configured secret/sentinel in the harnessed outputs.

NA-0319 added bounded qshield identifier/default-padding proof. The harness
proves opaque per-candidate ack handles, malformed/stale handle rejects, stale
peer/session receive reject, default padding buckets `[512, 1024, 2048, 4096,
8192]`, invalid padding config reject, malformed padded-input reject, no
accepted state/output on reject, and valid ack only after identifier/padding
verification.

NA-0320 added bounded qshield sanitized-error and retention/purge proof. The
harness exercises real qshield relay, receive, and attachment receive reject
paths, scans for forbidden secret/sentinel diagnostics, proves invalid receive
and attachment rejects retain the same candidate, proves repeated invalid
receive is bounded, proves valid ack purges exactly one candidate, and proves
stale ack fails closed.

These inherited proofs are local/demo executable evidence. They are not
qsl-server or qsl-attachments production evidence.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0321 entry.
- `tests/NA-0320_closeout_restore_na0321_testplan.md`.
- `docs/governance/evidence/NA-0320_metadata_runtime_sanitized_errors_retention_purge_harness.md`.
- `tests/NA-0320_metadata_runtime_sanitized_errors_retention_purge_harness_testplan.md`.
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`.
- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness.md`.
- `docs/governance/evidence/NA-0315_metadata_runtime_identifier_padding_executable_harness_plan.md`.
- `docs/governance/evidence/NA-0314_metadata_runtime_identifier_padding_transition_plan.md`.
- `docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md`.
- `docs/demo/DEMO-PUBLIC-001_Metadata_Visibility.md`.
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `apps/qshield-cli/src/commands/attachment.rs`.
- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/demo_adversarial_stress.sh`.
- `scripts/ci/demo_soak_repeated_run.sh`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- qsc local timing and metadata tests, including `tui_fixed_polling.rs`,
  `meta_min.rs`, `meta_phase2.rs`, `ack_camouflage.rs`, and
  `receipt_policy_mvp_na0177.rs`.
- `TRACEABILITY.md` and `DECISIONS.md`.

## Timing/Traffic-Shape Surface Inventory

| Surface | Current evidence | Classification | What remains unproven |
| --- | --- | --- | --- |
| qshield sender cadence | qshield smoke/stress/soak and send path queue messages immediately on explicit command | `PARTIAL_EXECUTABLE` | No per-event send timing distribution, burst profile, or sender schedule measurement exists. |
| qshield receiver cadence | `recv` and `attachment recv` use candidate fetch once per explicit command and ack only after local verification | `PARTIAL_EXECUTABLE` | No receive polling cadence, idle polling, or repeated receive schedule is measured. |
| qshield retry cadence | relay rejects can include retry-after metadata for rate/quota cases; repeated invalid receive is bounded by harness behavior | `PARTIAL_EXECUTABLE` | No retry/backoff timing distribution or invalid retry cadence measurement exists. |
| qshield queue shape | candidate fetch/no-delete, valid ack purge, stale ack reject, per-recipient queue cap, token quota, and total queue cap are present in code or stress proof | `PROVEN_EXECUTABLE` for selected queue semantics; `PARTIAL_EXECUTABLE` for shape under load | Queue length over time, multi-sender pressure, and queue-drain cadence are not measured. |
| qshield message ordering | candidate fetch returns front-of-queue candidates and ack can remove a matching candidate | `PARTIAL_EXECUTABLE` | Ordering correlation and observer reconstruction risk are not measured. |
| qshield size/padding | NA-0319 proves bounded default-padding bucket behavior in qshield demo | `PROVEN_EXECUTABLE` for bounded buckets | Size distribution across real traffic, attachment descriptor/ciphertext size classes, and global defaults remain unproven. |
| qshield attachment descriptor/ciphertext | demo attachment path proves descriptor/fetch/decrypt and selected integrity rejects | `PARTIAL_EXECUTABLE` | Attachment timing, descriptor/ciphertext size linkage, and production attachment service behavior remain unproven. |
| qshield stress/soak | demo smoke, adversarial stress, and repeated soak produce bounded local artifacts and no-secret/no-panic markers | `PROVEN_EXECUTABLE` for local bounded runs | Artifacts do not measure per-operation timing, traffic shape, or production deployment behavior. |
| qsl-tui metadata demo | public metadata demo says timing, channel id, and coarse size patterns remain visible; padded mode mitigates only exact size correlation | `DOCS_ONLY` plus adjacent executable demo support | Not a runtime traffic-shape mitigation proof. |
| qsc fixed polling | qsc TUI fixed polling tests prove deterministic local fixed cadence and no extra tick after receive | `PROVEN_EXECUTABLE` within qsc TUI boundary | This is not qshield timing proof and not production-service traffic shaping. |
| qsc meta plan/receive poll | qsc metadata tests prove deterministic local plan/tick markers and bounded poll validation | `PARTIAL_EXECUTABLE` | Not integrated with qshield embedded relay timing evidence and not a production measurement. |
| qsc receipt batching/jitter | receipt policy tests cover batched/immediate/off behavior and deterministic policy markers | `PARTIAL_EXECUTABLE` | Does not prove broad traffic-shape hiding or production cadence. |
| qsl-server service timing | service evidence maps queue caps, rate/global caps, TTL/retention, and local hardening lanes | `FUTURE_GATE` for this lane | Production timing, deployment logs, access-log traffic shape, and public internet behavior remain unproven. |
| qsl-attachments service timing | service evidence maps retention/recovery/quota/backup boundaries | `FUTURE_GATE` for this lane | Production upload/fetch timing, descriptor/object correlation, and long-running service timing remain unproven. |

## Threat Model

### Attacker Observations

A timing or traffic-shape observer may see:

- message size and padded bucket class;
- send timing and burst frequency;
- receive timing and queue-drain cadence;
- retry cadence and rate-limit retry-after classes;
- ack/commit timing;
- invalid-message retry timing;
- qsc fixed polling interval markers in local qsc evidence;
- attachment descriptor and ciphertext size/timing relationship when visible;
- route/contact/session/candidate handle correlation;
- ordering correlation across queued candidates;
- failed versus valid error timing;
- local demo artifact timestamps, transcripts, marker order, and total runtime;
- production service logs, metrics, proxy access logs, backup artifacts, and
  retained operator bundles if a future service lane authorizes production
  proof.

### Threat Categories

| Category | Current risk | Current evidence posture |
| --- | --- | --- |
| Passive local observer | Can observe command timing, local files, demo logs, artifacts, process starts, and marker order. | `PARTIAL_EXECUTABLE`; no-secret/no-panic scans exist, but timing data is not hidden. |
| Passive network observer | Can observe connection timing, IP/host metadata, packet sizes, bursts, and directionality. | `NOT_READY` for timing hiding; public demo docs state timing remains visible. |
| Relay observer | Can observe recipient queue keys, candidate count shape, send/fetch/ack timing, bucket metadata, and ordering. | `PARTIAL_EXECUTABLE`; qshield candidate/ack semantics are proven, but traffic shape is not mitigated. |
| Malicious queued-message injector in bounded demo context | Can attempt malformed, stale, padded, invalid, or repeated candidates and observe reject timing and queue retention. | `PARTIAL_EXECUTABLE`; selected invalid cases are secret-safe and no-delete, but timing distribution is unmeasured. |
| Log/artifact observer | Can inspect CI/demo transcripts, marker order, total runtime, and any retained stress/soak artifacts. | `PARTIAL_EXECUTABLE`; no-secret/no-panic scans exist, but timing/shape artifacts are not classified. |
| Public website/docs observer | Can see public claim boundaries and infer what is not ready. | `DOCS_ONLY`; current docs should keep timing and traffic-shape gaps explicit. |
| Production service observer | Could observe production relay/attachment timing, queue depth, logs, metrics, access logs, and support bundles. | `FUTURE_GATE`; qsl-server and qsl-attachments production timing remain unproven. |

## Existing Executable Evidence Mapping

| Evidence | What it proves | What it does not prove | Future measurement use |
| --- | --- | --- | --- |
| NA-0318 qshield ack/commit harness | Candidate fetch does not delete; ack after local verification deletes one candidate; invalid receive no-delete/no-state/no-output/no-secret. | Timing hiding, cadence, production relay support, attachment production behavior. | Reuse as a queue-state fixture for measuring fetch/ack timing and queue-drain cadence. |
| NA-0319 qshield identifier/default-padding harness | Opaque per-candidate handles, bounded padding buckets, malformed/stale reject, no-delete/no-output/no-secret. | Global default padding, all size classes, size distribution over traffic, timing hiding. | Reuse to measure bucket distributions and remaining size classes. |
| NA-0320 qshield sanitized-error/retention harness | Selected reject diagnostics are secret-safe; invalid receive/attachment rejects retain candidates; valid ack purges one; stale ack fails closed. | Reject timing equivalence, invalid retry cadence, production service retention/purge. | Reuse invalid paths to measure failed/valid timing and repeated invalid cadence. |
| NA-0291 identifier/padding fixture | Policy-fixture proof for identifier and padding expectations. | Runtime timing or runtime traffic-shape behavior. | Seed future plan assertions and marker names, not runtime measurement itself. |
| NA-0293 sanitized-error/retention fixture | Policy-fixture proof for sanitized-error and retention expectations. | Runtime timing or production retention behavior. | Seed future reject categories and artifact no-secret checks. |
| Demo CLI smoke | Bounded local positive send/receive, auth/malformed/replay/id rejects, attachment demo, and no-secret markers. | Per-operation timing distributions or traffic shaping. | Use as a baseline workload for measurement harness traces. |
| Demo adversarial stress | Bounded local abuse, queue/rate cap, restart, no-secret/no-panic, and baseline stress markers. | Timing hiding, sustained production traffic, traffic-shape mitigation. | Use as a stress workload for burst/queue pressure measurement. |
| Demo repeated soak | Repeats local smoke/stress with artifact manifest, state isolation, no-secret/no-panic markers. | Long-running production timing or public internet behavior. | Use as bounded repeated-run timing sample source. |
| Metadata conformance smoke | Local relay auth, invalid padding metadata/config, padded queue inspection, loopback boundaries. | Full traffic-shape mitigation or production timing. | Use for size/padding and queue-observation measurement seed. |
| qsc fixed polling and meta tests | Deterministic local qsc fixed cadence, poll tick, meta plan, receipt policy, ack camouflage class proof. | qshield timing proof, qsl-server production proof, timing hiding. | Compare measurement vocabulary and marker strategy; do not treat as qshield mitigation. |
| Formal/refimpl/qsc suite-id evidence | Protocol/model/vector gates for Suite-2/qsc correctness. | Metadata runtime timing or traffic-shape behavior. | Keep as health gate; not a direct timing measurement source. |
| qsl-server/qsl-attachments service maps | Local service hardening and production-gate classifications. | qshield embedded relay proof and production timing proof for this lane. | Future cross-repo authorization source for service timing lanes. |

## What Current Evidence Proves

- Bounded qshield embedded-relay candidate fetch and ack/commit behavior.
- Bounded qshield identifier/default-padding behavior for selected demo buckets.
- Bounded qshield sanitized-error and retention/purge behavior for selected
  invalid and valid paths.
- Local demo smoke/stress/soak can run bounded workloads with no-secret and
  no-panic artifact scans.
- qsc has older local deterministic cadence and metadata-plan evidence for
  specific qsc surfaces.

## What Current Evidence Does Not Prove

- It does not prove that timing metadata is hidden.
- It does not prove that padding alone hides traffic shape.
- It does not prove anonymity, metadata-free behavior, or untraceability.
- It does not prove qsl-server production relay timing behavior.
- It does not prove qsl-attachments production upload/fetch timing behavior.
- It does not prove public internet or deployment timing behavior.
- It does not prove contact graph, IP-level, route/session, or ordering
  correlation resistance.
- It does not define or implement jitter, batching, cover traffic, scheduling,
  or transport padding mitigation.

## Future Executable Evidence Plan

### Candidate Harnesses

1. qshield demo timing measurement harness.
   - Measure explicit send, candidate fetch, receive reject, receive valid,
     ack, stale ack, and attachment reject timing in a deterministic local
     harness.
   - Record bounded JSON/TSV artifacts containing event labels, monotonic
     deltas, queue counts, bucket sizes, candidate counts, and pass/fail
     markers.
   - Scan artifacts for relay token, raw handles, plaintext sentinels,
     candidate ack ids where not intentionally redacted, panic/backtrace text,
     and long secret-like dumps.
   - State that the harness measures observable timing and does not hide it.

2. Padding/size distribution harness.
   - Exercise qshield default bucket sizes and selected payload lengths.
   - Record bucket class and remaining observable size class.
   - Verify no raw plaintext/sensitive length leaks where the surface treats
     that length as sensitive.

3. Queue cadence measurement harness.
   - Queue multiple candidates, measure candidate fetch cadence, ack cadence,
     queue-depth snapshots, and drain order.
   - Distinguish queue measurement from mitigation.

4. Invalid retry cadence harness.
   - Repeat invalid receive, invalid padding, malformed decode, malformed ack,
     and attachment reject paths.
   - Measure repeated reject timing and prove the queue remains bounded and
     candidates are not deleted before valid ack.

5. Service boundary plan.
   - qsl-server and qsl-attachments timing require separate cross-repo
     authorization because production/service timing is not proven by qshield
     embedded relay evidence.

### Measurement Strategy

- Prefer local monotonic timestamps captured by the harness around external
  qshield commands and relay client requests.
- Record only bounded event categories, counts, bucket classes, and elapsed
  milliseconds.
- Do not record secrets, bearer tokens, raw route tokens, raw local paths,
  plaintext payloads, or raw candidate handles in durable artifacts.
- Keep deterministic fixture mode separate from any later mitigation design.
- Treat measurement output as evidence of observability, not evidence of
  hiding or reducing all timing metadata.

### Candidate Markers

- `NA0322_TIMING_SURFACE_INVENTORY_OK`
- `NA0322_TRAFFIC_SHAPE_THREAT_MODEL_OK`
- `NA0322_QSHIELD_DEMO_TIMING_MEASUREMENT_OK`
- `NA0322_PADDING_SIZE_DISTRIBUTION_OK`
- `NA0322_QUEUE_CADENCE_MEASUREMENT_OK`
- `NA0322_INVALID_RETRY_CADENCE_BOUNDED_OK`
- `NA0322_NO_SECRET_TIMING_ARTIFACT_OK`
- `NA0322_METADATA_TIMING_EVIDENCE_PLAN_OK`

### Instrumentation Needs

The next harness can start with external measurement around existing qshield
commands and relay requests. Runtime instrumentation is not required first if
the first successor remains bounded to local qshield/demo measurement. If
future measurement needs internal queue snapshots not safely available through
current commands, that future directive must stop and select an instrumentation
plan rather than smuggling runtime implementation into a measurement lane.

### Stop Conditions for Future Work

Future timing/traffic work must stop if:

- it needs runtime timing, jitter, batching, cover traffic, or scheduling
  implementation outside the selected scope;
- it needs qsl-server or qsl-attachments implementation changes without
  cross-repo authorization;
- it would record secrets, raw handles, plaintext, route tokens, or
  secret-like dumps in durable timing artifacts;
- it would present local/demo timing measurement as production proof;
- it would claim timing metadata is hidden without exact executable evidence;
- required CI or public-safety fails conclusively.

## qshield Embedded Relay/Demo Boundary

The current proof is bounded to the qshield embedded relay and local demo
commands. It is useful for measuring local qshield send/receive/fetch/ack
timing, queue shape, retry/reject behavior, and padding size classes. It is not
production relay evidence.

## qsl-server/qsl-attachments Production Boundary

qsl-server and qsl-attachments production timing, deployment logs, metrics,
proxy access logs, public internet exposure, long-running service timing, and
operator support bundles remain future gates. Existing service evidence maps
local hardening and production-gate gaps, but NA-0321 does not convert those
service maps into production timing proof.

## Public-Internet/Deployment Boundary

No public internet timing behavior is proven here. Cross-host/private-network
demo evidence remains non-production. Public internet exposure, production
service operation, production observability, access-log redaction, proxy
behavior, firewall/ACL posture, and abuse testing require later exact lanes.

## Claim Boundaries

NA-0321 does not claim:

- anonymity;
- metadata-free behavior;
- untraceability;
- production readiness;
- public internet readiness;
- external review completion;
- timing metadata hiding;
- traffic-shape hiding;
- padding as sufficient to hide traffic shape;
- production qsl-server or qsl-attachments timing proof.

## Selected Successor

Selected successor:

`NA-0322 -- Metadata Runtime Timing and Traffic-Shape Measurement Harness`

Rationale: existing qshield embedded-relay/demo flows can be measured by a
bounded harness without changing runtime behavior. That measurement lane should
collect observable timing, size, queue, ordering, and retry/cadence artifacts
and explicitly report what remains visible. It must not implement mitigation or
claim timing hiding.

Rejected alternatives:

- `Metadata Runtime Timing and Traffic-Shape Instrumentation Plan`: not first,
  because external qshield/demo measurement appears feasible before runtime
  instrumentation.
- `Metadata Runtime Timing and Traffic-Shape Blocker Resolution`: not first,
  because the current qshield/demo surfaces are sufficient to define a bounded
  measurement harness.
- `Metadata Runtime Service Timing Cross-Repo Authorization`: important later,
  but qshield/demo timing measurement is the smaller next evidence step before
  qsl-server/qsl-attachments production timing lanes.
- `Metadata Runtime qshield Demo Traffic-Shape Measurement Harness`: too narrow
  in title because the measurement plan should also preserve the qsc and
  service-boundary evidence relationships while implementing only bounded
  qshield/demo measurement unless future scope expands.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0321. Durable changes stay inside the
qsl-protocol worktree under `/srv/qbuild/work`, which is already inside the
qbuild backup scope. No new durable evidence location, source root, excluded
backup path, or non-rebuildable artifact outside current scope is introduced.

## Next Recommendation

After this evidence-plan PR merges and post-merge public-safety is green, close
out NA-0321 and restore:

`NA-0322 -- Metadata Runtime Timing and Traffic-Shape Measurement Harness`

The closeout must not implement NA-0322. The future NA-0322 lane should add a
bounded qshield/demo measurement harness and artifact no-secret checks, while
keeping qsl-server and qsl-attachments production timing as future gates.
