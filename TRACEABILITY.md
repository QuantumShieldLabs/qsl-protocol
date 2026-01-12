# Goal Traceability Matrix (Living)

This document maps program goals to spec sections, implementation modules, and tests/vectors.

## Instructions
- Update this matrix whenever protocol behavior changes or new invariants are added.
- Every row must be actionable: a reader should be able to find the relevant spec/code/tests quickly.

## Matrix

| Goal | Property / invariant | Spec section(s) | Implementation module(s) | Tests / vectors | Notes |
|---|---|---|---|---|---|
| G1 | Always-hybrid per-message AEAD key: mk = KDF_HYBRID(ec_mk, pq_mk) | DOC-CAN-003 §§3.3.1–3.3.5, §7.3 | tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs; tools/actors/refimpl_actor_rs (Suite-2 KDF ops + mk hybrid op) | inputs/suite2/vectors/qshield_suite2_kdf_vectors_v1.json (CAT-S2-KDF-001); inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json (CAT-S2-MK-001); suite2-ci | KDF labels and ordering are normative in DOC-CAN-003 |
| G2 | Explicit SCKA epochs; strict monotonic ADV; one-time CTXT targeting w/ tombstones; transactional commit (fail-closed); bounded skipped-key eviction | DOC-CAN-004 §§2.2, 3–6 (incl. §3.5); DOC-CAN-003 §§3.3.3, 3.3.6, 7.5.3, 8.2, 9.1.1, 9.3 | tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs; tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs; tools/refimpl/quantumshield_refimpl/src/suite2/state.rs; tools/actors/refimpl_actor_rs (SCKA logic + KEM ops + pqreseed op + ooo op + boundary op + e2e recv op + durability rollback checks) | inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json (CAT-SCKA-LOGIC-001); inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json (CAT-SCKA-KEM-001); inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json (CAT-S2-PQRESEED-001); inputs/suite2/vectors/qshield_suite2_ooo_replay_vectors_v1.json (CAT-S2-OOO-001); inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json (CAT-S2-BOUNDARY-001); inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json (CAT-S2-E2E-RECV-001); inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json (CAT-S2-CRASH-001); tests/AUDIT-20260104_critical_hardening_testplan.md; suite2-ci | Logic invariants + deterministic KEM correctness coverage |
| G3 | Transcript-/AD-bound capability commitment; fail-closed negotiation; downgrade resistance | DOC-CAN-003 §§2.2–2.3, §5.1, §6 (incl. §6.3/§6.6) | tools/refimpl/quantumshield_refimpl/src/suite2/binding.rs; tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs; tools/actors/refimpl_actor_rs (transcript check + downgrade ops + suite2.establish.run); apps/qshield-cli/src/commands/establish.rs (demo bundle.id check + replay record) | inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json (CAT-S2-TRANSCRIPT-001); inputs/suite2/vectors/qshield_suite2_downgrade_vectors_v1.json (CAT-S2-DOWNGRADE-001); inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json (CAT-S2-ESTABLISH-001); scripts/ci/metadata_conformance_smoke.sh; tests/NA-0019_identity_binding_testplan.md; tests/NA-0020_establish_replay_cache_testplan.md; tests/NA-0025_pq_binding_testplan.md; suite2-ci | No silent downgrades; reject if Suite-2 required; AD binding enforced |
| G4 | Conformance vectors + formal verification plan + CI-gated model checks | DOC-CAN-003 §10; FORMAL_VERIFICATION_PLAN.md; formal/README.md; DOC-TST-005 | .github/workflows/suite2.yml; .github/workflows/formal.yml; scripts/ci/*suite2*; formal/run_model_checks.py; tools/goal_lint.py; tools/actors/interop_actor_py/interop_actor.py | suite2-ci executes CAT-S2-KDF-001, CAT-S2-TRANSCRIPT-001, CAT-S2-MK-001, CAT-S2-PQRESEED-001, CAT-S2-OOO-001, CAT-S2-BOUNDARY-001, CAT-S2-PARSE-001, CAT-S2-ESTABLISH-001, CAT-S2-E2E-RECV-001, CAT-S2-INTEROP-001, CAT-S2-INTEROP-XIMPL-001, CAT-S2-CRASH-001, CAT-SCKA-LOGIC-001, CAT-SCKA-KEM-001, CAT-S2-DOWNGRADE-001 (fail-closed) and runs CAT-S2-KDF-001 / CAT-S2-TRANSCRIPT-001 / CAT-S2-MK-001 against the independent python actor; formal-ci executes bounded SCKA logic model; metadata-conformance-smoke enforces demo establish replay rejection; tests/NA-0024_pqxdh_scka_epoch_mapping_testplan.md | Expand model coverage incrementally; bounds changes are governance-relevant |
| G5 | Metadata minimization baseline (non-anonymity): safe-by-default demo relay/CLI transport posture | NEXT_ACTIONS.md NA-0016 (baseline); docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md; docs/privacy/DOC-G5-002_Metadata_Leakage_Inventory_v1.0.0_DRAFT.md; docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md | apps/qshield-cli/src/commands/relay.rs (/consume, /establish_record); apps/qshield-cli/src/relay_client.rs; apps/qshield-cli/src/commands/*.rs; apps/qshield-cli/src/fsutil.rs | scripts/ci/metadata_conformance_smoke.sh; tests/NA-0016_metadata_conformance_testplan.md; tests/NA-0018_prekey_lifecycle_testplan.md; tests/NA-0019_identity_binding_testplan.md; tests/NA-0020_establish_replay_cache_testplan.md; tests/NA-0021_rate_limit_testplan.md; tests/NA-0022_identifier_collision_testplan.md; tests/NA-0026_store_lifecycle_testplan.md; tests/NA-0027_identity_warning_testplan.md; tests/NA-0028_token_quota_testplan.md; .github/workflows/ci.yml (metadata-conformance-smoke) | Local relay demo only; no anonymity claims |

## Changelog
- 2025-12-28: NA-0003 — Completed DOC-CAN-003 to a self-contained, implementable Suite-2 normative spec; clarified fail-closed downgrade rules, transcript/AD binding, and Suite-2 KDF labels/ordering aligned to CI-gated KDF ops.
- 2025-12-28: NA-0004 — Completed DOC-CAN-004 to an implementable, fail-closed SCKA normative spec aligned to CAT-SCKA-LOGIC-001 and CAT-SCKA-KEM-001.
- 2025-12-28: NA-0005 — Expanded DOC-TST-005 to enumerate protocol-level composition categories (transcript binding, hybrid mk, reseed integration, OOO/replay, crash/rollback) for Suite-2.
- 2025-12-28: NA-0008 — Introduced the initial CI-gated formal model lane (`formal/`) and a bounded SCKA logic model runner executed by formal-ci.
- 2025-12-28: NA-0010 — Document rationalization: deprecation/consolidation policy recorded; legacy starters/queues redirected to START_HERE.md and NEXT_ACTIONS.md.
- 2025-12-31: NA-0006 — Governance update for baseline unblock + Suite-2 fail-closed scaffolding (x25519 static_secrets; parse_only fixture alignment to current parser semantics; Suite-2 module skeleton; actor advertises Suite-2 but rejects use with REJECT_S2_NOT_IMPLEMENTED).
- 2025-12-31: NA-0006 — Added CAT-S2-TRANSCRIPT-001 vectors + runner; Suite-2 AD binding helpers + actor transcript check op (REJECT_S2_AD_MISMATCH).
- 2025-12-31: NA-0006 — Added CAT-S2-MK-001 vectors + runner; Suite-2 per-message mk derivation helper + actor mk hybrid op (REJECT_S2_MK_MISMATCH).
- 2025-12-31: NA-0006 — Added CAT-S2-PQRESEED-001 vectors + runner; Suite-2 PQ reseed integration helper + actor pqreseed op.
- 2025-12-31: NA-0006 — Enforced ML-KEM-768 pq_ct length (1088) in PQ reseed; added reject vector for bad length.
- 2025-12-31: Hotfix — Fail-closed QSP parsing enforces hdr_ct_len=24 and body_ct_len>=16; parse_only fixtures updated.
- 2025-12-31: NA-0006 — Added CAT-S2-OOO-001 vectors + runner; Suite-2 OOO/replay receive helper + actor op.
- 2025-12-31: NA-0006 — Added CAT-S2-BOUNDARY-001 vectors + runner; Suite-2 boundary receive helper + actor op with PQ reseed integration.
- 2025-12-31: NA-0006 — Added CAT-S2-PARSE-001 vectors + runner; Suite-2 strict ratchet message decoder + parse op.
- 2025-12-31: NA-0006 — Added CAT-S2-E2E-RECV-001 vectors + runner; Suite-2 end-to-end raw wire receive op.
- 2025-12-31: NA-0006 — Added CAT-S2-INTEROP-001 vectors + runner; Suite-2 send→wire→recv interop op (flags==0 only).
- 2025-12-31: NA-0007 — Added CAT-S2-CRASH-001 vectors + runner; Suite-2 crash/restart durability and rollback detection bound to SCKA monotonic invariants.
- 2025-12-31: NA-0009 — Added independent python actor for Suite-2 KDF/transcript/mk ops; suite2-ci gates these categories against a non-Rust implementation.
- 2025-12-31: NA-0009 — Added CAT-S2-INTEROP-XIMPL-001 vectors + runner; cross-implementation wire interop between refimpl_actor and python actor.
- 2025-12-31: NA-0011 — Clarified Suite-2 establishment mapping in DOC-CAN-003 §6 (negotiation gating, base handshake contract, fail-closed rejects).
- 2025-12-31: NA-0012 — Anchored CAT-S2-ESTABLISH-001 in DOC-TST-005 and added a testplan stub (establishment mapping vectors/runner/ops).
- 2025-12-31: NA-0012 — Clarified Suite-2 base handshake contract to include DH public keys for initialization (DOC-CAN-003 §6.3/§8.2).
- 2025-12-31: NA-0012 — Implemented Suite-2 establishment op + CAT-S2-ESTABLISH-001 vectors/runner (suite2-ci gated).
- 2025-12-31: NA-0013 — Suite-2 runners prefer sessionful establish→send/recv when establishment stanzas are present; interop/ximpl/crash vectors include establishment inputs for sessionful coverage.
- 2026-01-01: NA-0015 — Added non-production demo CLI scaffold (`apps/qshield-cli/`) with local relay stub and fail-closed init/status UX (no protocol-core changes).
- 2026-01-01: NA-0015 — Added demo vertical slice: local relay queue + sessionful Suite-2 register/establish/send/recv via actor (demo-only; wire-neutral).
- 2026-01-01: NA-0015 — Demo crypto round-trip gated by CI smoke (two-party establish/send/recv via actor; local relay only).
- 2026-01-01: NA-0015 — Closeout evidence: PRs #54/#55/#56; demo-cli-build and demo-cli-smoke jobs enforced on main; Suite-2 demo only, no Suite-1/1B wire or behavior changes.
- 2026-01-02: NA-0014 — Governance hardening: AGENTS.md now specifies the exact Goals line format for goal-lint compliance.
- 2026-01-02: NA-0016 — Demo relay/CLI hardened for safe-by-default metadata posture; metadata conformance smoke added to CI.
- 2026-01-03: NA-0016 — Enforced demo store permissions (0700/0600) and expanded metadata conformance checks (token, queue caps, unauth override).
- 2026-01-03: NA-0016 — Added optional size-bucket padding envelope and CI assertions for bucket consistency (demo transport only).
- 2026-01-03: NA-0016 — Completed metadata minimization lane baseline with CI-gated conformance (PRs #61–#64).
- 2026-01-03: NA-0017 — Added clean-room comparative review scaffold (DOC-REV-001) for Signal benchmark lane.
- 2026-01-03: NA-0017 — Comparative review lane queued (clean-room Signal benchmark review; DOC-REV-001 placeholder).
- 2026-01-04: NA-0017 — Populated DOC-REV-001 with initial decision-grade matrix and Top-5 upgrade list (clean-room, spec-cited).
- 2026-01-04: NA-0017 — Closeout recorded with PR evidence and DOC-REV-001 as the comparative review artifact.
- 2026-01-04: NA-0018 — Demo relay one-time bundle consumption (/consume) + CI gate via metadata conformance smoke.
- 2026-01-04: NA-0019 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0018 — Queue follow-on from DOC-REV-001: one-time prekey lifecycle + at-most-once consumption (demo relay semantics).
- 2026-01-04: NA-0019 — Queue follow-on from DOC-REV-001: explicit identity binding for demo establish.
- 2026-01-04: NA-0019 — Demo establish identity binding (bundle.id) enforced and CI-gated.
- 2026-01-04: NA-0020 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0020 — Queue follow-on from DOC-REV-001: establish replay cache (session_id + bundle identifiers).
- 2026-01-04: NA-0020 — Demo establish replay cache enforced and CI-gated.
- 2026-01-04: NA-0021 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0021 — Queue follow-on from DOC-REV-001: relay rate limiting/backoff for register/poll.
- 2026-01-04: NA-0021 — Demo relay rate limiting enforced and CI-gated.
- 2026-01-04: NA-0022 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0022 — Queue follow-on from DOC-REV-001: relay ID format + collision handling.
- 2026-01-04: NA-0022 — Demo relay ID format validation + collision rejection enforced and CI-gated.
- 2026-01-04: NA-0023 — Queue follow-on from DOC-REV-001: eviction/deletion invariants for skipped keys.
- 2026-01-04: NA-0023 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0023 — Suite-2 skipped-key eviction/delete-on-use invariants added and CI-gated via OOO vectors.
- 2026-01-04: NA-0024 — Queue follow-on from DOC-REV-001: PQXDH-style bundle mapping to SCKA initial epoch rules.
- 2026-01-04: NA-0024 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0024 — PQXDH-style bundle mapping to SCKA initial epoch defined and CI-gated via SCKA logic vectors.
- 2026-01-04: NA-0025 — Suite-2 establish PQ KEM pub/prekey binding enforced and CI-gated.
- 2026-01-04: NA-0026 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0026 — Demo store rotation + permission checks enforced and CI-gated.
- 2026-01-04: NA-0025 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0025 — Queue follow-on from DOC-REV-001: PQ KEM public key / prekey ID binding in establishment AD.
- 2026-01-04: NA-0026 — Queue follow-on from DOC-REV-001: secure deletion/rotation policy for demo store artifacts + skipped keys.
- 2026-01-04: NA-0027 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0027 — Queue follow-on from DOC-REV-001: demo UX identity verification warnings on first establish.
- 2026-01-04: NA-0027 — Demo CLI identity verification warning enforced and CI-gated.
- 2026-01-04: NA-0028 — Promoted to READY (queue resume; derived from DOC-REV-001 follow-ons).
- 2026-01-04: NA-0028 — Queue follow-on from DOC-REV-001: per-token quotas within demo relay queue caps.
- 2026-01-04: NA-0028 — Demo relay per-token quotas enforced and CI-gated.
- 2026-01-05: Governance — Freeze NA promotion after NA-0028; public-release prep baseline.
- 2026-01-05: Audit hardening — refimpl stdcrypto Ed25519 fail-closed checks and ratchet skip-loop overflow reject (audit 2026-01-04).
- 2026-01-07: Public release runbook ordering — docs/public/PUBLIC_RELEASE_RUNBOOK.md; QSL_PUBLIC_RELEASE_PLAN.md; tests/PUBLIC_RELEASE_RUNBOOK_testplan.md.
- 2026-01-07: Public allowlist inventory — docs/public/PUBLIC_ALLOWLIST_INVENTORY.md; tests/PUBLIC_ALLOWLIST_INVENTORY_testplan.md.
- 2026-01-07: Public export manifest — docs/public/PUBLIC_EXPORT_MANIFEST.md; tests/PUBLIC_EXPORT_MANIFEST_testplan.md.
- 2026-01-07: Public repo baseline scaffolding — README.md; LICENSE; SECURITY.md; CONTRIBUTING.md; THIRD_PARTY_NOTICES.md; docs/public/PUBLIC_WORKSPACE_AND_NAMING.md; tests/PUBLIC_REPO_BASELINE_testplan.md.
- 2026-01-07: Public repo polish baseline — README.md; SECURITY.md; CONTRIBUTING.md; tests/PUBLIC_REPO_POLISH_testplan.md.
- 2026-01-07: Public export sync — community health + templates; public repo QuantumShieldLabs/qsl-protocol; tag v0.1.0-draft; public commits 72435b7 and 3f5be57; org profile merge 96be046.

- 2026-01-08: Public export allowlist synced to public repo 68b88783550f34af81319db8e3f7785405434305; includes docs/INDEX.md, .github/workflows/public-ci.yml, and .github/CODEOWNERS; high-confidence scan excludes only the workflow file; aligns with v0.1.1-draft.

- 2026-01-08: Public export allowlist synced to public repo f9aa28cb2035d067facd2f2bc7c510fbba1b68f1; includes contributor checklists and PR template links.
- 2026-01-09: Audit Issue #5 hardening — AEAD seal fail-closed (no panic) + regression guard testplan (tests/AUDIT-20260104_issue5_aead_no_panic_testplan.md) (PR #21).
- 2026-01-09: Audit Issue #4 hardening — OsRng for StdCrypto keypair/nonce generation + regression guard testplan (tests/AUDIT-20260104_issue4_rng_osrng_testplan.md) (PR #22).

- 2026-01-09: Cutover — public repo becomes primary development tree; full import from private snapshot 276c4dd (see DECISIONS.md D-0074).

- 2026-01-09: Docs cutover accuracy — README and docs/INDEX updated for v0.2.0 public development cutover (PR #14).
- 2026-01-09: Audit report import + CRITICAL regression guard tracking — docs/audit/CODE_ANALYSIS_REPORT_20260104.md; docs/audit/AUDIT_CODE_ANALYSIS_STATUS_20260104.md; tests/AUDIT-20260104_regression_guards_testplan.md (PR #18).
- 2026-01-09: Audit CRITICAL #1–#3 verified — docs/audit/AUDIT_CODE_ANALYSIS_STATUS_20260104.md; tests/AUDIT-20260104_regression_guards_testplan.md (PR #19).
- 2026-01-09: Audit Issue #8 hardening — HandshakeInit encode fail-closed on missing OPK fields; regression guard test (tests/AUDIT-20260104_issue8_opk_invariant_testplan.md) (PR #20).
- 2026-01-10: Audit Issue #7 hardening — ratchet_encrypt no-mutation on failed send + regression guard test (tests/AUDIT-20260104_issue7_send_state_no_mutation_testplan.md) (PR #23).
- 2026-01-10: Audit Issue #9 hardening — zeroize secret-bearing key material; regression guards (tests/AUDIT-20260104_issue9_zeroization_testplan.md) (PR #25).
- 2026-01-10: NA-0031 spec review — Issue #6 ck_pq_recv boundary handling analysis (tests/AUDIT-20260104_issue6_ck_pq_recv_boundary_spec_review.md).
- 2026-01-10: Audit Issue #6 hardening — ck_pq_recv boundary handling fix + regression guard tests + testplan (PR #28).
- 2026-01-10: Audit Issue #10 hardening — header-decrypt candidate trials attempt all bounded keys; deterministic reject + no-mutation guards; testplan `tests/AUDIT-20260104_issue10_header_timing_sidechannel_testplan.md` (PR #30).
- 2026-01-11: Audit Issue #12 hardening — mk_order kept consistent with mk_skipped on take; regression guard tests and testplan `tests/AUDIT-20260104_issue12_mk_order_stale_testplan.md` (PR #32).
- 2026-01-11: Audit Issue #13 hardening — SCKA monotonicity reject + regression guards; testplan `tests/AUDIT-20260104_issue13_scka_monotonicity_testplan.md` (PR #34).
- 2026-01-11: Audit Issue #14 hardening — store_mk_skipped fail-closed; regression guards; testplan `tests/AUDIT-20260104_issue14_store_mk_skipped_silent_failure_testplan.md` (PR #36).
- 2026-01-11: Audit Issue #15 hardening — DH ratchet ns overflow fail-closed; regression guards; testplan `tests/AUDIT-20260104_issue15_pn_ns_overflow_testplan.md` (PR #38).
- 2026-01-11: Audit Issue #16 hardening — bounded suite2 restore_bytes parsing; regression guards; testplan `tests/AUDIT-20260104_issue16_deser_dos_bounds_testplan.md` (PR #40).
- 2026-01-11: Audit Issue #17 hardening — header_pt unwrap removal; regression guards; testplan `tests/AUDIT-20260104_issue17_header_pt_unwraps_testplan.md` (PR #42).
- 2026-01-11: Audit Issue #18 hardening — OPK unwrap removal in initiator_start; regression guards; testplan `tests/AUDIT-20260104_issue18_opk_unwraps_testplan.md` (PR #44).
- 2026-01-11: Audit Issue #19 hardening — reduce SessionState cloning in ratchet paths; regression guards; testplan `tests/AUDIT-20260104_issue19_state_clone_key_material_testplan.md` (PR #46).
