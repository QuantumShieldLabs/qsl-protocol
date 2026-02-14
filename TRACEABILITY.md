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

- 2026-02-11: TUI nav selection UX hardening — single selected nav marker (`>`), arrow-key nav selection, Enter activation, and deterministic nav invariants in `qsl/qsl-client/qsc/tests/tui_nav_selection.rs` (PR #316).
- 2026-01-26: NA-0071 READY — https://github.com/QuantumShieldLabs/qsl-protocol/pull/130 — QSP v4.3 header key derivation correctness; target files: tools/refimpl/quantumshield_refimpl/src/qsp/state.rs, handshake.rs, ratchet.rs; test plan: tests/NA-0071_qsp_header_key_derivation_testplan.md.
- 2026-01-26: NA-0071 IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/131 — KMAC-only header key derivation at session init; files: tools/refimpl/quantumshield_refimpl/src/qsp/state.rs, handshake.rs; tests: tools/refimpl/quantumshield_refimpl/tests/na_0071_header_key_derivation.rs; plan: tests/NA-0071_qsp_header_key_derivation_testplan.md.
- 2026-01-26: NA-0072 BACKLOG — https://github.com/QuantumShieldLabs/qsl-protocol/pull/130 — Public repo housekeeping (deprecated/duplicate artifacts cleanup; doc pointer alignment; single source of truth).
- 2026-01-25: QSC — Added staged YubiKey roadmap for vault keyslots (“plumbing now, enforce later”) in design spec and NEXT_ACTIONS (NA-0062 BACKLOG).
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
- 2026-01-11: Audit Issue #20 hardening — CLI relay lock poison handling; regression guards; testplan `tests/AUDIT-20260104_issue20_cli_mutex_poison_testplan.md` (PR #69).
- 2026-01-11: Audit Issue #21 hardening — MKSKIPPED no-mutation-on-reject; regression guard; testplan `tests/AUDIT-20260104_issue21_mkskipped_removal_testplan.md` (PR #50).
- 2026-01-12: Audit Issue #22 hardening — boundary header single-attempt; deterministic reject + no-mutation guard; testplan `tests/AUDIT-20260104_issue22_boundary_window_testplan.md` (PR #52).
- 2026-01-12: Audit Issue #23 hardening — ss3 mixed into handshake key schedule; regression guards; testplan `tests/AUDIT-20260104_issue23_ss3_entropy_testplan.md` (PR #54).
- 2026-01-12: Audit Issue #24 hardening — guard ZERO32 unset chain keys; regression guard; testplan `tests/AUDIT-20260104_issue24_zero32_testplan.md` (PR #57).

- 2026-01-14: Audit Issue #25 hardening — canonical refimpl error surface; regression guards; testplan `tests/AUDIT-20260104_issue25_error_types_testplan.md` (PR #69).

- 2026-01-14: Audit Issue #26 hardening — asymmetric establish ZERO32 guard tests; testplan `tests/AUDIT-20260104_issue26_asymmetric_initial_state_testplan.md` (PR #69).

- Hotfix: Suite-2 CHAINKEY_UNSET reject includes reason_code token (PR #64) — files: tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs — proof: unit test suite2::ratchet::tests::asymmetric_recv_unset_chainkey_rejects_deterministically_and_no_mutation

- 2026-01-17: Audit Issue #27 hardening — QSP handshake fail-fast guards before signature verification; testplan `tests/AUDIT-20260104_issue27_sig_verify_order_testplan.md` (PR #65).
- 2026-01-17: Audit Issue #28 hardening — QSP ProtocolMessage encode fail-closed on missing PQ fields; testplan `tests/AUDIT-20260104_issue28_safe_unwraps_testplan.md` (PR #67).
- 2026-01-18: CodeQL baseline hygiene — eliminate rust/hard-coded-cryptographic-value findings; Decision D-0102; artifacts: tests/CODEQL_hardcoded_crypto_value_cleanup_testplan.md; files: tools/actors/refimpl_actor_rs/src/main.rs, tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs, tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs, tools/refimpl/quantumshield_refimpl/src/qsp/state.rs, tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs (PR #69).

- 2026-01-18: CodeQL operating procedure — START_HERE.md + DOC-DEV-002; Decision D-0103 (PR #70).

- 2026-01-18: Roadmap execution sequence — START_HERE “Execution Roadmap: Suite-2 → Relay → Linux TUI Demo”; NA-0050/NA-0051 BACKLOG; Decision D-0104; testplan docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md (PR #72).

- 2026-01-18: NA-0050 relay harness adapter — tests/harness/4b/lib/relay_http.py + tests/harness/4b/runner.py; test plan docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md; Decision D-0105; PR #74.

CodeQL ops: START_HERE.md pointer → docs/dev/DOC-DEV-002_CodeQL_Operating_Procedure_v1.0.0_DRAFT.md → DECISIONS D-0106.

- NA-0052 relay interop: tests/harness/4b/lib/relay_http.py + tests/harness/4b/runner.py + tests/harness/4b/tests/test_relay_http_adapter.py → docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md → DECISIONS D-0107 → PR #78.
- NA-0051 Linux TUI demo: apps/qsl-tui/** → docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md → DECISIONS D-0108 → https://github.com/QuantumShieldLabs/qsl-protocol/pull/104.
- NA-0051 headless demo mode: apps/qsl-tui/** → docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md → DECISIONS D-0109 → PR #83.

qsl-tui headless demo tests: tests/QSL_TUI_HEADLESS_demo_testplan.md → PR #83.

DEMO-0001 headless evidence: DOC-TST-RELAY-TUI (DEMO-0001 Evidence) → DECISIONS D-0001.


NA-0053 (public demo + metadata reality + padding mitigation): NEXT_ACTIONS.md → apps/qsl-tui → docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md → DECISIONS D-0002.

## Governance hygiene
- GOV docs organization: D-0003 -> docs/DOCS_MAP.md

NA-0054 metadata visibility demo: NEXT_ACTIONS.md -> apps/qsl-tui -> docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md -> DECISIONS D-0004.
- DEMO-PUBLIC-001 metadata visibility demo: docs/demo/DEMO-PUBLIC-001_Metadata_Visibility.md -> scripts/demo/demo_public_metadata_visibility.sh -> DECISIONS D-0005

NA-0056 (public demo/client v1): apps/qsl-tui + scripts/demo + DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md -> DECISIONS D-0006

- 2026-01-22: NA-0057 Public Demo Runbook — docs/dev/DOC-DEV-004_Public_Demo_Runbook_v0.1.0_DRAFT.md; docs/DOCS_MAP.md; DECISIONS D-0007; tests/NA-0057_public_demo_runbook_testplan.md.
- 2026-01-23: NA-0057 DONE — PR #94 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/94) merge=7d34360eee1e8216f3dac5a9e2aac8eab7e60018.
- 2026-01-23: NA-0058 DONE — QSC client Phase 1 (secure-by-default CLI; durable send semantics; bounded recv routing; deterministic markers/tests) — Spec: docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md. (evidence: PR #95, PR #96, PR #97; merge 93d11f318e067e55e09fc02c2c725f55e6412dd2; verified 2026-01-24)
- 2026-01-23: NA-0058 (Step 1) — QSC scaffold (qsl/qsl-client/qsc) + deterministic marker tests + workspace wiring — PR #96 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/96) — Spec: docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md.
- 2026-01-24 — NA-0058 (QSC client) Step-2 hardening + deterministic tests — PR #97: anchored QSC_CONFIG_DIR safety checks; doctor check-only markers; non-HOME test root; locked tests/build.
- 2026-01-24 — NA-0059 DONE — QSC Step 3: command-surface expansion + security checklist alignment (https://github.com/QuantumShieldLabs/qsl-protocol/pull/104).
- 2026-01-24 — NA-0059 (Step 3) DONE — terminal sanitization + marker discipline + bounded waits — https://github.com/QuantumShieldLabs/qsl-protocol/pull/104.
- 2026-01-24 — NA-0060 READY — QSC store hardening (umask/perms, symlink-safe paths, atomic writes, locking, deterministic errors). Design basis: docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md. (https://github.com/QuantumShieldLabs/qsl-protocol/pull/102 (merge b32f0d8d7c46c7d53b9ba97a9697563783b2e715))
- 2026-01-24 — QSC backlog expansion: appended NA-0061..NA-0070 as BACKLOG items (governance PR: https://github.com/QuantumShieldLabs/qsl-protocol/pull/101).
- 2026-01-24 — PR #101 also incorporates client_suggestions.txt coverage mapping into NEXT_ACTIONS appendix.
- 2026-01-24 — NA-0060 IN-PROGRESS — store hardening (locking, atomic writes, perms, keyslot-ready metadata) — https://github.com/QuantumShieldLabs/qsl-protocol/pull/102 (merge b32f0d8d7c46c7d53b9ba97a9697563783b2e715).
- NA-0061 DONE — PR #107 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/107) merged (merge SHA 4e0cc3af7b49224c1b3ac72224d4375219e56088).
- NA-0061 Phase 2 — encrypted-at-rest vault default + keychain-preferred fallback + deterministic noninteractive (https://github.com/QuantumShieldLabs/qsl-protocol/pull/130)
- NA-0062 DONE — PR #110 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/110) merged (merge SHA aded11b95b81fcbcc89139960a949845ad6f8c78).
- 2026-01-25: GOV — Codified quoting-safe directive template rules (Decision D-0118).
- NA-0062 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/110 — vault keyslot providers (yubikey stub + mock tests)
- NA-0063 DONE — PR #112 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/112) merged (merge SHA 85508a2bd9f8c0567ae9856db775a838a6a1f593).
- NA-0063 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/112 — qsc resource limits + bounded retries/timeouts
- NA-0064 DONE — PR #114 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/114) merged (merge SHA 3cc55d3d1647b62a3aa195373519f87f66972648).
- NA-0064 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/114
- NA-0065 READY — promoted after NA-0064 close-out.
- NA-0065 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/116 (merge 71ef24c6b92bb600c0e12eb900bedeeec573f4b6) — output minimization posture (redaction by default, reveal flag)
- NA-0066 DONE — PR #118 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/118) merged (merge SHA 6a8fcd9268dceb6b9bf9abd2f64c9e988521d6fb).
- NA-0067 DONE — PR #121 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/121) merged (merge SHA aceedd34da242722f8f57844f0e3394de33b4732).
- 2026-01-25: GOV — Added mandatory State Ledger + state reset requirement (Decision D-0124).
- NA-0067 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/121 — receipt/ACK camouflage (avoid ACK distinguishability)
- NA-0068 DONE — PR #123 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/123) merged (merge SHA 2d21a961686060337ee78b5c4beb88c8ef7db74c).
- NA-0068 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/123 — supply-chain + release authenticity controls
- NA-0069 DONE — PR #126 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/126) merged (merge SHA 8f118163bf05b5f45944c03c91585791433ce76d).
- 2026-01-26: GOV — Added Codex diagnosis requirement for blocked/unclear issues (Decision D-0127).
- NA-0069 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/126 — qsc secret hygiene (zeroize + crash surface minimization)
- NA-0070 DONE — PR #128 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/128) merged (merge SHA d0f3801d3d020ec2b65c73dabf95283202b1a327).
- NA-0070 — IN PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/128 — send commit semantics (prepare→send→commit)
- NA-0071 DONE — PR #131 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/131) merged (merge SHA 86cae35b7864b661b09a699d294224e07a06c855).
- NA-0072 READY — repo housekeeping plan — tests/NA-0072_repo_housekeeping_plan.md — PR #133 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/133).
- NA-0072 implementation — PR #135 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/135) — removed duplicate PR template; archived START_HERE_2.md; plan: tests/NA-0072_repo_housekeeping_plan.md.
- NA-0072 DONE — PR #135 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/135) merged (merge SHA 931cd7e9ba3e780cdc5d4ce49a4a1e8075e810e2); deferred harness dedupe due to README.md reference (follow-on required).
- NA-0073 READY — harness dedupe + README alignment — tests/NA-0073_harness_dedupe_readme_plan.md — PR #137 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/137).
- NA-0073 implementation — PR #138 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/138) — removed legacy test-harness/; README aligned to tests/harness/.
- NA-0073 DONE — PR #138 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/138) merged (merge SHA d81806bcb6b540cb070ee56768a756aa5b99fae0).
- NA-0074 READY — qsc Security Lens MVP (CLI + TUI) — docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md — tests/NA-0074_qsc_security_lens_mvp_plan.md — PR #140 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/140).
- NA-0074 IMPLEMENTATION — TUI skeleton + charter tests + clippy fixes — qsl/qsl-client/qsc/** — PR #141 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/141).
- NA-0074 IMPLEMENTATION — phase 1 completion (status pane + session panel + receive no-mutation test) — qsl/qsl-client/qsc/** — PR #142 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/142).
- NA-0074 DONE — PR #142 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/142) merged (merge SHA 8a4dbe891923f31ae6a83f8862488eaecd55ca17).
- NA-0075 READY — qsc relay demo transport — docs/qsc/DOC-QSC-002_Relay_Demo_Transport_Contract_v1.0.0_DRAFT.md — tests/NA-0075_qsc_relay_demo_transport_plan.md — PR #144 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/144).
- NA-0075 implementation — PR #145 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/145) — qsc relay serve/send + deterministic fault injection tests (drop/dup) — tests/NA-0075_qsc_relay_demo_transport_plan.md.
- NA-0075 implementation — PR #146 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/146) — deterministic relay reorder tests (reorder, drop+reorder, seeded replay) — tests/NA-0075_qsc_relay_demo_transport_plan.md.
- NA-0075 DONE — PR #145 merge SHA 7780d61d53d81dceced1c1aa9b7b09598d06e1d5; PR #146 merge SHA 185aced78e62d65d3cbefdf30d60dc7162541714.
- NA-0076 READY — workflow hardening defaults — docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md — tests/NA-0076_quality_workflow_hardening_plan.md — PR #148 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/148).
- NA-0076 DONE — PR #148 merge SHA 3c361ec1854e95c54861f5499d37328d4f2ea0ff.
- NA-0077 DONE — PR #151 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/151) merged (merge SHA 42224a2ba1c186f517749775277385df2e4270dd).
- NA-0077 implementation — PR #151 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/151) — demo script + runbook wiring + CI smoke workflow.
- NA-0078 DONE — PR #154 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/154) merged (merge SHA 5599ff096942782b65fe7c36bb9220ca929bb756).
- NA-0078 implementation — PR #154 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/154) — full-run demo script + CI full-run smoke + deterministic artifacts.
- NA-0079 DONE — PR #157 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/157) merged (merge SHA 363194118e3ab96fa7533cb2bac492263572003f).
- NA-0079 implementation — PR #157 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/157) — qsc TUI relay wiring + headless tests; qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs.
- NA-0080 DONE — Impl PR #160 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/160) merged (merge SHA ca9f283d9385c0dff6ddf8b25366dd6bfb57e397).
- NA-0080 — Impl PR — https://github.com/QuantumShieldLabs/qsl-protocol/pull/162 — remote relay workflow inputs
- qsc outbox recovery — PR #163 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/163) — adds `qsc send abort` + qsl/qsl-client/qsc/tests/outbox_abort.rs.
- NA-0082 READY — tests/NA-0082_qsc_doctor_clarity_plan.md — PR #164 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/164).
- NA-0082 implementation — PR #165 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/165) — qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/cli.rs.
- NA-0082 DONE — PR #165 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/165) merged (merge SHA b851ffd68ca89f9abcb122171b155da80f4c07e6).
- NA-0083 READY — tests/NA-0083_qsc_xdg_lock_plan.md — PR #167 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/167).
- NA-0083 implementation — PR #168 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/168) — qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/cli.rs.
- NA-0083 DONE — PR #168 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/168) merged (merge SHA 9bacfe0fe55c076e69cf931d00ac7a9d2bfa0109).
- NA-0084 READY — tests/NA-0084_qsc_send_semantics_plan.md — PR #170 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/170).
- NA-0084 implementation — PR #171 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/171) — qsc send explicit relay transport + tests.
- NA-0084 DONE — PR #171 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/171) — merge SHA 6964408bf486af2bef1c5b45e7697fa59fa33589.
- NA-0085 READY — tests/NA-0085_qsc_tui_help_render_plan.md — PR #173 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/173).
- NA-0085 implementation — PR #174 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/174) — TUI /help renders deterministic command list; headless test in qsl/qsl-client/qsc/tests/tui_help_render.rs.
- NA-0085 DONE — PR #174 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/174) merged (merge SHA 85aff62321b8c818fbaa143d5a71f1bbdbf07e32).
- NA-0086 READY — tests/NA-0086_qsc_tui_marker_routing_plan.md — PR #176 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/176).
- NA-0086 implementation — PR #177 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/177) — interactive TUI routes markers in-app; headless stdout preserved; tests in qsl/qsl-client/qsc/tests/tui_marker_routing.rs.
- NA-0086 DONE — PR #177 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/177) merged (merge SHA 7816293cbd238f8a782d2fa99244dd4cf9ba7522).
- NA-0087 READY — tests/NA-0087_qsc_tui_help_fullscreen_plan.md — PR #179 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/179).
- NA-0087 implementation — PR #180 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/180) — full-screen help mode + headless tests.
- NA-0087 DONE — PR #180 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/180) — merge: a1a74d795f1b81263feaa83967bacfe75cff3b8c.
- NA-0088 READY — tests/NA-0088_qsc_tui_focus_modes_plan.md — PR #182 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/182).
- NA-0088 implementation — PR #183 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/183) — full-screen focus modes + headless tests.
- NA-0088 DONE — PR #183 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/183) — merge SHA daf6bab657f75874d73d1106ac7d99c3780d98db.
- NA-0089 READY — tests/NA-0089_demo_evidence_counts_plan.md — PR #185 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/185).
- NA-0089 implementation — PR #186 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/186) — demo artifacts now include summary.txt + normalized_counts.txt.
- NA-0089 DONE — PR #186 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/186) merged (merge SHA e62faf76d8f9f5608f07714e8a5c02d1a4b0a964).
- NA-0090 READY — tests/NA-0090_remote_scenario_fault_injection_plan.md — PR #188 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/188).
- NA-0090 implementation — PR #189 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/189) — client-side relay fault injection markers + tests/remote_fault_injection.rs.
- NA-0090 DONE — PR #189 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/189) — merge SHA 2dff509b9e832ab986e1eb73e7098dec9d2976a7.
- NA-0091 READY — tests/NA-0091_qsc_receive_e2e_plan.md — PR #191 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/191).
- NA-0091 implementation — PR #192 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/192) — relay-backed receive (CLI+TUI) + tests/receive_e2e.rs.
- NA-0091 DONE — PR #192 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/192) — merge SHA 533321405659e58b945701cc7dcec61ef3a26aa7.
- NA-0092 READY — tests/NA-0092_qsp_qse_onwire_plan.md — PR #194 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/194).
- NA-0092 implementation — PR #195 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/195).
- NA-0092 DONE — PR #195 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/195) — merge SHA 4b98291187a1bb64a8992ecfd787f1392f223c20.
- NA-0093 READY — tests/NA-0093_qsc_truthy_protocol_status_plan.md — PR #197 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/197).
- NA-0093 implementation — PR #198 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/198) — truthy QSP/QSE status + tests/qsp_status_truthy.rs.
- NA-0093 DONE — PR #198 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/198) — merge SHA `65bda575276a605a0bc9d8b10064d02fe74ecc45`.
- NA-0094 READY — tests/NA-0094_qsc_protocol_gate_plan.md — PR #200.
- NA-0094 implementation — PR #201 — qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs.
- NA-0094 DONE — PR #201 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/201) — merge SHA `1d6aa6d78618dbb9d8dcc0bebd13550221e00cad`.
- NA-0095 READY — tests/NA-0095_qsp_qse_handshake_mvp_plan.md (incl. refimpl PQ KEM prerequisite) — PR #206 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/206).
- NA-0095 implementation — PR #207 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/207) — refimpl ML-KEM-768 PqKem768 for StdCrypto + tests.
- NA-0095 handshake MVP — PR #205 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/205) — PQ KEM handshake frames + transcript checks + tests.
- NA-0095 DONE — PR #205 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/205) — merge SHA 4145ea1.
- NA-0096 READY — tests/NA-0096_first_ratchet_step_plan.md — PR #210 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/210).
- NA-0096 implementation — PR #211 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/211) — ratchet send/recv advance + skipped handling + tests/ratchet_step.rs.
- NA-0096 DONE — PR #211 merged (merge SHA d718a66e2b2bd4e2d42b36a2ca8cd59a936a73c0).
- NA-0099 READY — tests/NA-0099_handshake_a2_confirm_plan.md — PR #213 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/213).
- NA-0099 implementation — PR #214 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/214) — A2 confirm and responder commit gating + tests.
- NA-0099 DONE — PR #214 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/214) — merge SHA 9b10828d522824a65704a58ac5f4828555e1cb8c.
- NA-0100 READY — tests/NA-0100_identity_binding_tofu_plan.md — PR #216 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/216).
- NA-0100 implementation — PR #217 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/217) — TOFU identity pinning + mismatch reject.
- NA-0100 DONE — PR #217 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/217) — merge SHA `8c0a472feb6ad4825d2212a5d244d7791f34a31e`.
- NA-0101 READY — tests/NA-0101_pq_signature_identity_plan.md — PR #236 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/236).
- NA-0101 implementation — qsl/qsl-client/qsc/src/main.rs, tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs, tests/NA-0101_pq_signature_identity_plan.md — PR #237 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/237).
- NA-0101 DONE — PR #237 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/237) — merge SHA 03cc989a57d996a47e4a667e404c11b157843594.
- NA-0102 DONE — PR #220 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/220) — merge SHA 77613619296d31fdc2d213016c47c321bc3d12a0.
- NA-0103 READY — tests/NA-0103_metadata_minimization_plan.md — PR #222 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/222).
- NA-0103 implementation — PR #223 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/223).
- NA-0103 DONE — PR #223 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/223) — merge SHA 6e8d5dcda90fe73ba7fd9769b978c99d9b87f4d5.
- NA-0104 READY — tests/NA-0104_tui_layout_h3_plan.md — PR #226 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/226).
- NA-0104 implementation — PR #227 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/227) — H3 home layout with switchable inspector drawer, responsive breakpoints, and headless render tests.
- NA-0104 DONE — PR #227 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/227) merged (merge SHA 34c15522da4dfb271138662959006625f7a327f6).
- NA-0104 follow-up implementation — PR #229 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/229) — home F2-F5 inspector switching, Ctrl+F2-F5 focus jump, and `/ins` alias with deterministic headless tests.
- NA-0105 READY — tests/NA-0105_truthful_active_session_only_plan.md — PR #230 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/230)
- NA-0106 implementation — tests/NA-0106_identity_secret_at_rest_plan.md — PR #234 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/234)
- NA-0106 DONE — PR #234 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/234) — merge SHA 9f8ac906707bf261331dbb5cada61d3a9636da29.
- NA-0105 implementation — PR #231 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/231) — ACTIVE requires validated peer session; seed fallback restricted to explicit test-only override.
- NA-0105 DONE — PR #231 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/231) — merge SHA a8cc0f85559c73f203bc96ea10fc5fd26406f3cf.
- NA-0107 READY — tests/NA-0107_remote_relay_auth_header_plan.md — PR #242 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/242).
- NA-0107 implementation — qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/relay_auth_header.rs; tests/NA-0107_remote_relay_auth_header_plan.md — PR #243 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/243).
- NA-0107 DONE — PR #243 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/243) — merge SHA b74e21a22ebc7f287e19c8459ac21ec9996c617f; workflow runs 21792900305 (happy-path seed=1) and 21792900550 (drop-reorder seed=7) PASS.
- NA-0108 READY — tests/NA-0108_remote_handshake_lane_plan.md — governance promotion recorded in local directive execution (2026-02-08).
- NA-0108 implementation — .github/workflows/remote-handshake-tests.yml; scripts/demo/qsc_remote_handshake_smoke.sh; docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md; tests/NA-0108_remote_handshake_lane_plan.md.
- NA-0108 governance scope-expansion — PR #247 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/247) — allow minimal `qsl/qsl-client/qsc/**` receive mailbox/peer separation for fail-closed remote handshake lane completion (directive 0265).
- NA-0108 implementation update (directive 0265) — PR #248 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/248) — receive mailbox/peer separation in `qsl/qsl-client/qsc/src/main.rs`; tests in `qsl/qsl-client/qsc/tests/receive_e2e.rs`; remote lane script hardening in `scripts/demo/qsc_remote_handshake_smoke.sh`; contract note update in `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md`.
- NA-0108 implementation follow-up — PR #249 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/249) — relay env normalization for remote handshake script.
- NA-0108 implementation follow-up — PR #250 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/250) — per-run mailbox/peer isolation for relay traffic.
- NA-0108 implementation follow-up — PR #252 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/252) — handshake status assert compatibility for redacted peer labels.
- NA-0108 DONE — merge SHAs: #247=`98bc981624503f7067490cd3d4f8c5f0d6a3184f`, #248=`ee7e789587c1a792ebf8e8398ed0ca84f9387b80`, #249=`897ba4d257682924718e43223e188c7653c8dd1a`, #250=`d9de78ef639390b2e37daed36a4a8c1b7c8dbb98`, #252=`1afa7a3070701d6704646ca61ee5f9c89ce3b7fd`; workflow PASS runs: 21794286407 and 21794286815.
- NA-0109 READY — tests/NA-0109_session_state_at_rest_plan.md — PR #254 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/254).
- NA-0109 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/vault.rs`; `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`; `qsl/qsl-client/qsc/tests/vault.rs`; `tests/NA-0109_session_state_at_rest_plan.md` — PR #255 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/255).
- NA-0109 DONE — PR #255 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/255) — merge SHA `943e9a7964d5a908112386da3833bb1eb032c0ab`.
- NA-0110 READY — `tests/NA-0110_provenance_lighttouch_plan.md` — PR #257 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/257).
- NA-0110 implementation — `NOTICE`; `PROVENANCE.md`; `SIGNED_RELEASES_RUNBOOK.md`; `tests/NA-0110_provenance_lighttouch_plan.md` — PR #258 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/258).
- NA-0110 DONE — PR #258 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/258) — merge SHA `0c15b124cec15744c5e9b7d375fb5f545f06249b`.
- NA-0111 READY — `tests/NA-0111_client_lifecycle_hardening_plan.md` — PR #260 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/260).
- NA-0111 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/lifecycle.rs`; `tests/NA-0111_client_lifecycle_hardening_plan.md` — PR #261 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/261).
- NA-0111 DONE — PR #261 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/261) — merge SHA `fefcaae8d56c9606fce7010b6d0179a24923f768`.
- NA-0112 READY — `tests/NA-0112_metadata_minimization_phase2_plan.md` — PR #263 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/263).
- NA-0112 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/meta_phase2.rs`; `tests/NA-0112_metadata_minimization_phase2_plan.md` — PR #264 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/264).
- NA-0112 DONE — PR #264 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/264) — merge SHA `79e7c779ab26d187395335ead65114c76e922a8b`.
- NA-0113 READY — `tests/NA-0113_delivered_receipts_plan.md` — PR #266 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/266).
- NA-0113 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/receipts_delivered.rs`; `tests/NA-0113_delivered_receipts_plan.md` — PR #267 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/267).
- NA-0113 DONE — PR #267 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/267) — merge SHA `7aef7330696f4a31e21d44b432a7b0ea0c37a310`.
- NA-0114 READY — `tests/NA-0114_tui_readability_h3_plan.md` — PR #269 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/269).
- NA-0114 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_readability.rs`; `tests/NA-0114_tui_readability_h3_plan.md` — PR #270 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/270).
- NA-0114 DONE — PR #270 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/270) — merge SHA `7ff06a282802b17735538d56ddb44b6adfac8d96`.
- NA-0115 BACKLOG — `tests/NA-0115_local_unlock_gate_plan.md` — PR #272 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/272).
- NA-0115 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/vault.rs`; `qsl/qsl-client/qsc/tests/unlock_gate.rs`; `tests/NA-0115_local_unlock_gate_plan.md` — PR #274 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/274).
- NA-0115 DONE — PR #274 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/274) — merge SHA `6c56a1eb0ddd3514453001284d039d79ebd9b2cc`.
- NA-0116 BACKLOG — `tests/NA-0116_contacts_verify_block_plan.md` — PR #272 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/272).
- NA-0116 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/contacts_verify_block.rs`; `qsl/qsl-client/qsc/tests/identity_binding.rs`; `qsl/qsl-client/qsc/tests/identity_ux.rs`; `qsl/qsl-client/qsc/tests/tui_relay_drop_reorder.rs`; `tests/NA-0116_contacts_verify_block_plan.md` — PR #277 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/277).
- NA-0116 DONE — PR #277 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/277) — merge SHA `ed03ad8806b712d4de3d9c75d69b4c6ebb5edca3`.
- NA-0117 BACKLOG — `tests/NA-0117_encrypted_timeline_store_plan.md` — PR #272 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/272).
- NA-0117 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/timeline_store.rs`; `tests/NA-0117_encrypted_timeline_store_plan.md` — PR #280 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/280).
- NA-0117 DONE — PR #280 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/280) — merge SHA `e0db6eef10f6df3df88fc6c634e5d25f94e351b8`.
- NA-0118 BACKLOG — `tests/NA-0118_message_state_model_plan.md` — PR #272 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/272).
- NA-0118 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/message_state_model.rs`; `qsl/qsl-client/qsc/tests/receipts_delivered.rs`; `tests/NA-0118_message_state_model_plan.md`; `DECISIONS.md` — PR #283 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/283), merge SHA `141c929c50f0611840c9ba0725452c4cf1c5cd27`.
- NA-0119 BACKLOG — `tests/NA-0119_file_transfer_mvp_plan.md` — PR #272 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/272).
- NA-0119 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`; `tests/NA-0119_file_transfer_mvp_plan.md`; `DECISIONS.md` — PR #286 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/286), merge SHA `8d03a6fbd80b2307c7e09e4c9acfbda55d0f6404`.
- NA-0120 implementation (docs-only) — `docs/qsc/QSC_TUI_SPEC.md`; `docs/qsc/QSC_TUI_INVARIANTS.md`; `docs/qsc/QSC_TUI_IMPLEMENTATION_CHECKLIST.md` — PR #290 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/290).
- NA-0121 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_unified_layout.rs` — PR #293 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/293), merge SHA `833c8d59f29c18eff143ebdbaf3c8392cd64a69d`.
- NA-0122 DONE — tooling hardening (preflight + post-merge verifier + goal-lint robustness) — PR #296 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/296), merge SHA `5a34880d036680aaf5897baecb17197978a1102b`.
- NA-0123 DONE — TUI Messages + Contacts feature-complete (truthful states) + invariant tests (client-only) — PR #300 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/300) — merge SHA `c495b8d08ba6bf194a67254365462330b21befa6`.
- NA-0124 DONE — TUI Files feature-complete (multi-select allowed) + invariant tests (client-only) — PR #303 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/303) — merge SHA `4b0376e0596c5c4acc61ae0e12ebc13f56622da9`.
- NA-0125 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_keys_activity_status.rs` — PR #306 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/306).
- NA-0125 DONE — TUI Keys + Activity + Status feature-complete + invariant tests (client-only) — PR #306 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/306) — merge SHA `22047cbfda857caba4d8ae034056aa4d73066c7d`.
- NA-0126 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_settings_lock.rs` — PR #309 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/309), merge SHA `5533d3f982a3e0ef28ddaee51ae3651a41e730fb`.
- NA-0127 DONE — relay-backed UI integration lane — PR #313 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/313), merge SHA `9ecf8b4174c9c9a81344a78a85c883f6e79fc9e3`; PR #314 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/314), merge SHA `2748e7a764489954257d4592e2d7fe8f674a845a`; proof run https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/21888936094.
- NA-0128 DONE — TUI locked-first startup + zero-leak pre-unlock shell + init/unlock UX — PR #319 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/319), merge SHA `847a3b83ce7059a61581c807042013f09c878ced`.
- NA-0129 DONE — TUI chrome simplification + Help/About/Legal post-unlock + debug-noise removal — PR #322 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/322), merge SHA `a61a8ed78881e9d0dcedd71154843d6431bd26af`.
- NA-0130 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_autolock.rs` — PR #325 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/325), merge SHA `3a5c893fc672d64a9a5e27f09487d568f3f595e3`.
- NA-0131 implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs` — PR #329 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/329).
- NA-0131 follow-up implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs` — PR #330 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/330).
- NA-0131 lock/unlock UX polish implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs` — PR TBD.
- NA-0131 UX cleanup implementation — `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs` — PR TBD.
