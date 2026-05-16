Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0299 Core Protocol Crypto Demo Assurance Matrix

Directive: QSL-DIR-2026-05-16-106 / NA-0299

## Executive Summary

NA-0299 returns the queue from website operator-blocker work to the core QSL mission. The audit reviewed current protocol, crypto, formal/model, vector/refimpl, qsc send path, demo, metadata, service-boundary, dependency, and external-review evidence, then selected the next executable hardening lane.

The strongest current evidence is executable and repeatable for bounded Suite-2 downgrade/no-mutation models, qsc `send_commit`, refimpl Suite-2 tests, qshield demo smoke, baseline adversarial stress, repeated demo soak, metadata policy-fixture harnesses, and dependency/advisory health. The main remaining gap is not another planning document: it is a consolidated adversarial harness that expands replay, reject, and no-mutation proof across protocol-core and demo-observable boundaries without changing protocol semantics by default.

Recommended NA-0300:

**Core Protocol Replay / Reject / No-Mutation Adversarial Harness**

## Scope And Non-Goals

This audit is evidence and test-matrix work only. It does not change protocol behavior, QSP wire format, crypto state machines, key schedules, runtime demo behavior, qsl-server behavior, qsl-attachments behavior, qsc-desktop behavior, website sources, dependencies, workflows, branch protection, or public-safety configuration.

Non-goals:

- no protocol/crypto implementation changes;
- no service implementation changes;
- no demo/runtime implementation changes;
- no dependency changes;
- no website or external website mutation;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no anonymity, metadata-free, or untraceable claim;
- no claim that local evidence is a full cryptographic proof.

## Commands Run

Startup, governance, and public-safety proof:

```bash
date --iso-8601=seconds
date -u --iso-8601=seconds
df -BG /srv/qbuild
git status --porcelain=v1 --branch
git fetch --all --prune
git rev-parse origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
gh api /repos/QuantumShieldLabs/qsl-protocol/commits/882b882473c0c242dfe6167679b2e265ad0de60c/check-runs?per_page=100
```

Core and dependency checks:

```bash
python3 scripts/ci/public_safety_gate.py selftest-advisories-resilience
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Metadata and demo checks:

```bash
scripts/ci/metadata_conformance_smoke.sh
scripts/ci/metadata_phase2_identifier_padding_harness.sh
scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
```

Targeted refimpl and qshield checks:

```bash
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked --test na_0241_demo_downgrade_no_mutation -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked --test na_0243_skipped_key_decrypt_no_mutation -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked --test kt_verifier_vectors -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
```

Read-only discovery:

```bash
rg -n "send_commit|handshake|key schedule|ratchet|downgrade|replay|no mutation|reject|formal|model|vector|suite2|metadata|padding|retention|sanitized|panic|unwrap|expect|TODO|FIXME|SECURITY|external review|NOT_READY|PARTIAL|production|public internet|adversarial|stress|soak|cross-host|qshield|demo|refimpl|RustSec|webpki" <repo evidence and implementation paths>
```

The first discovery pass included absent top-level path names and exited non-zero. It was classified as recoverable command-shape/discovery error; the audit reran against the actual repository layout and recorded the zero/missing-path result without broadening scope.

## Artifact Paths

- `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.3WRGEv`
- `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260516T165257Z`
- `/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260516T165307Z`

## Evidence Classification Legend

- `PROVEN_EXECUTABLE`: deterministic local command or existing executable harness passed for the bounded claim.
- `PARTIAL_EXECUTABLE`: executable proof exists, but the claim boundary is intentionally narrower than the domain.
- `DOCS_ONLY`: current evidence is planning, mapping, or review packaging only.
- `NOT_READY`: claim or capability must not be presented as ready.
- `FUTURE_GATE`: explicit future lane or operational prerequisite.
- `OUT_OF_SCOPE`: not authorized by NA-0299.

## Protocol / Crypto Assurance Matrix

| Domain | Current evidence | Classification | Gap |
| --- | --- | --- | --- |
| Suite-2 downgrade/no-mutation model | `python3 formal/run_model_checks.py` passed SCKA bounded model and Suite-2 negotiation model: 926 SCKA states, 108 negotiation attempts, 214 rejected outcomes, 428 no-mutation assertions. | PROVEN_EXECUTABLE | Model abstracts authenticated capability evidence, transcript binding, KDFs, AEAD, secrecy, and authentication. |
| qsc send commit behavior | `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` passed 3 tests proving commit advances once and send failure does not commit. | PROVEN_EXECUTABLE | Narrowly scoped to send commit tests, not full session adversarial coverage. |
| refimpl Suite-2 / ratchet / KT tests | `cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1` passed 68 library tests plus targeted integration tests. | PROVEN_EXECUTABLE | Needs a consolidated adversarial replay/reject/no-mutation harness to reduce coverage fragmentation. |
| Key schedule and hybrid ratchet evidence | Existing canonical docs, vectors, and refimpl tests cover the research-stage Suite-2 always-hybrid design. | PARTIAL_EXECUTABLE | Not a full cryptographic proof and not external review completion. |
| Protocol implementation change | Changed paths for NA-0299 are governance docs, testplan, decisions, traceability, and journal only. | OUT_OF_SCOPE | Any code bug found later requires a separate executable directive. |

## Formal / Model Evidence

`formal/run_model_checks.py` passed locally. The Suite-2 model rejects weaker/unknown suites, capability commitment mismatch, and transcript-suite mismatch while asserting no accepted-suite or durable accept-count mutation on rejects. The SCKA model continues to prove bounded persistence/rollback/no-mutation behavior.

Classification: `PROVEN_EXECUTABLE` for the bounded model properties; `PARTIAL_EXECUTABLE` for full protocol assurance because the model intentionally abstracts cryptographic primitives and authentication.

## Vector / Refimpl Evidence

The refimpl test suite passed with targeted proof for:

- demo downgrade/no-mutation;
- skipped-key decrypt no-mutation;
- KT verifier vectors;
- Suite-2 handshake, bounded receive, bucket confidentiality, parse-only vectors, PQ KEM, and related library tests.

Classification: `PROVEN_EXECUTABLE` for covered harnesses; `PARTIAL_EXECUTABLE` for broad interoperability because NA-0299 did not add new vector differential coverage.

## send_commit / No-Mutation Evidence

The qsc `send_commit` integration test passed and proves the existing send path does not commit state on send failure and advances once on successful outbox commit. Formal and refimpl tests add additional no-mutation coverage for reject paths.

Gap: proof is distributed across multiple harnesses. NA-0300 should consolidate adversarial replay/reject/no-mutation cases into one first-class lane.

## Replay / Reject / Downgrade Evidence

Current executable proof includes:

- formal downgrade and AD mismatch rejection;
- refimpl downgrade and skipped-key no-mutation tests;
- qshield demo smoke negative auth, malformed, invalid relay ID, replay, KT reject, and attachment integrity reject markers;
- demo adversarial stress markers for auth reject no-mutation, queue/rate bounds, chaos recovery, and no panic.

Classification: `PROVEN_EXECUTABLE` for current commands; `PARTIAL_EXECUTABLE` for cross-layer coverage because the proof is not yet a single adversarial protocol-core matrix.

## Demo App Assurance

`scripts/ci/demo_cli_smoke.sh` passed with markers for local loopback default, two peer init, relay authorization, establish/send/receive/decrypt, attachment descriptor/fetch/decrypt, attachment opaque boundary, attachment integrity reject, KT reject/no-mutation, and no secret leak.

`DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh` passed and produced `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260516T165257Z`.

`DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` passed and produced `/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260516T165307Z`.

Classification: `PROVEN_EXECUTABLE` for local loopback smoke, baseline adversarial stress, and repeated soak; `PARTIAL_EXECUTABLE` for cross-host, desktop, and production service behavior.

## Metadata Assurance

`scripts/ci/metadata_conformance_smoke.sh` passed.

`scripts/ci/metadata_phase2_identifier_padding_harness.sh` passed with `NA0291_METADATA_PHASE2_HARNESS_OK`.

`scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh` passed with `NA0293_METADATA_PHASE2_SANITIZED_RETENTION_OK` and artifact directory `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.3WRGEv`.

Classification: `PROVEN_EXECUTABLE` for policy-fixture and smoke harnesses; `PARTIAL_EXECUTABLE` for broader runtime behavior. The evidence does not support anonymity, metadata-free, or untraceable claims.

## Service Boundary Assurance

Read-only sibling repo inspection found qsl-server and qsl-attachments local service hardening evidence for authorization, reject/no-mutation, bounds, retention, opaque ciphertext boundaries, and recovery-oriented tests. That evidence remains outside qsl-protocol implementation scope for NA-0299.

Classification: `PARTIAL_EXECUTABLE` for local service-boundary evidence; `NOT_READY` for production service readiness and public-internet service readiness because ingress, TLS/proxy policy, observability, long-running ops, backup/restore, deployment runbooks, and external review disposition remain future gates.

## Desktop / Sidecar Boundary

Existing qsc-desktop evidence remains a bounded prototype/sidecar boundary. NA-0299 did not run desktop UI or native package checks and did not change qsc-desktop.

Classification: `DOCS_ONLY` or `PARTIAL_EXECUTABLE` depending on prior lane evidence; `NOT_READY` for production desktop release claims.

## Dependency / Advisory Assurance

`cargo audit --deny warnings` passed with the advisory database available from cache. `cargo tree -i rustls-webpki --locked` showed `rustls-webpki v0.103.13` through `rustls v0.23.36` into qsc, qsl-tui, and qshield-cli. The advisory resilience selftest passed by simulating retryable advisory behavior without weakening the gate.

Classification: `PROVEN_EXECUTABLE` for current Rust dependency/advisory health.

## External Review Readiness

The external review package and release evidence map exist and are useful review inputs. Findings/disposition, reviewer identity, review scope, and accepted remediation evidence are not complete in this repository.

Classification: `DOCS_ONLY` for package availability; `NOT_READY` for external-review-complete claims.

## Production / Public Claim Boundaries

Allowed:

- research-stage protocol and demo evidence with explicit caveats;
- executable local proof for the bounded commands listed here;
- current release-readiness gaps and future gates.

Not ready:

- production-readiness;
- public-internet-readiness;
- external-review-complete;
- anonymity, metadata-free, or untraceable messaging;
- full cryptographic proof;
- complete formal proof;
- website implementation or deploy-readiness.

Prohibited:

- saying the audit made QSL production-ready;
- saying local demo proof equals deployment readiness;
- saying metadata fixtures prove anonymity;
- saying external review is complete;
- saying Suite-2 is quantum-proof, unbreakable, or guaranteed secure.

## Ranked Hardening Candidates

| Rank | Candidate | Risk reduced | Expected files | Tests to add/run | Implementation change may be needed? | Stop conditions |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Core Protocol Replay / Reject / No-Mutation Adversarial Harness | Fragmented reject/no-mutation proof across protocol, refimpl, and demo boundaries. | `tools/refimpl/**`, `inputs/**`, `docs/governance/evidence/**`, `tests/**`, possibly qsc tests if explicitly authorized by NA-0300. | New adversarial harness plus existing send_commit, formal, refimpl, demo smoke/stress. | Default NO; STOP if code change appears necessary. | Any semantic drift, wire change, or security bug requiring implementation edits. |
| 2 | Suite-2 Negotiation / Downgrade Expansion Harness | Downgrade coverage outside the current bounded model and selected refimpl tests. | `tools/refimpl/**`, `inputs/suite2/**`, `formal/**` only if explicitly authorized. | Additional downgrade vectors and model cases. | Default NO. | New behavior requirement or ambiguous canonical spec. |
| 3 | Refimpl / Vector Differential Consistency Harness | Vector/refimpl drift and interoperability ambiguity. | `tools/refimpl/**`, `inputs/**`, evidence/testplan docs. | Parse/derive/decrypt differential tests. | Default NO. | Need for protocol behavior change. |
| 4 | Metadata Runtime Feasibility Harness | Fixture proof not yet tied to broader runtime behavior. | Metadata inputs/harness docs and possibly runtime hooks only under future authorization. | Runtime feasibility negatives and sanitized artifact scans. | Possibly YES in future lane. | Any claim would imply anonymity or metadata-free behavior. |
| 5 | Demo End-to-End Adversarial Regression Harness | Demo assurance is strong locally but split across smoke, stress, and soak scripts. | Existing demo scripts if explicitly authorized; evidence/testplan docs. | Combined demo regression and artifact invariants. | Default NO unless script lane authorizes. | Runtime demo behavior change needed. |
| 6 | Service Production Gate Evidence Lane | qsl-server/qsl-attachments production gates are still future work. | Sibling repos plus qsl-protocol evidence. | Deployment/runbook/observability proof. | Likely YES in service repos. | Public-internet readiness overclaim. |
| 7 | Desktop Dependency / Packaging Hygiene Lane | Desktop packaging and advisory drift risk. | qsc-desktop if future lane authorizes. | Package audit/build proof. | Possibly YES. | Native host limitations or unrelated dependency churn. |

## Recommended NA-0300

**NA-0300 - Core Protocol Replay / Reject / No-Mutation Adversarial Harness**

Rationale:

- It directly advances G3 and G4 by hardening fail-closed behavior and release-gate evidence.
- It keeps the focus on core protocol assurance rather than website or public-copy work.
- It can start as an executable harness lane with no protocol/crypto implementation changes by default.
- It addresses the clearest current gap: existing replay, reject, downgrade, and no-mutation evidence is strong but spread across formal, refimpl, qsc, and demo checks.
- If the harness discovers a real protocol/crypto bug, the next directive must stop and authorize a separate implementation fix instead of silently changing semantics.

Suggested NA-0300 default flags:

- Wire/behavior change allowed? NO by default.
- Crypto/state-machine change allowed? NO by default.
- Docs-only allowed? YES for evidence, but the lane should add executable tests/harnesses if authorized.

## No Implementation Change Proof

NA-0299 changed only:

- `docs/governance/evidence/NA-0299_core_protocol_crypto_demo_assurance_matrix.md`
- `tests/NA-0299_core_protocol_crypto_demo_assurance_matrix_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

It did not change protocol-core, qsc, qsl, qshield-cli, refimpl, formal models, inputs, qsl-server, qsl-attachments, qsc-desktop, website, scripts, workflows, Cargo files, dependency locks, branch protection, or public-safety configuration.

## Stop Conditions For Next Executable Lane

NA-0300 must stop if:

- a replay/reject/no-mutation bug is proven and fixing it would require protocol/crypto implementation changes not explicitly authorized;
- any harness expectation would alter QSP wire semantics, key schedule, handshake, downgrade behavior, or cryptographic state-machine semantics;
- fixture or vector changes would hide a fail-open behavior;
- local demo evidence is used to claim production readiness or public-internet readiness;
- metadata fixture proof is used to claim anonymity, metadata-free messaging, or untraceability;
- external review package existence is treated as external review completion;
- more than one READY item would exist.
