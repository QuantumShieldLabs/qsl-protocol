# NEXT_ACTIONS — Authoritative Execution Queue

Goals: G4 (primary), drives G1–G3 delivery

## 0) New Chat Starter (paste this verbatim at the top of every new chat)

Read START_HERE.md first and follow it strictly.

Authoritative sources (in order):
START_HERE.md, GOALS.md, AGENTS.md, PROJECT_CHARTER.md, NEXT_ACTIONS.md, DECISIONS.md, TRACEABILITY.md.
Canonical docs: docs/canonical/DOC-CAN-003_*, docs/canonical/DOC-CAN-004_*.
Test plan/categories: docs/test/DOC-TST-005_*.
Inputs: inputs/suite2/vectors/*.json.

Constraints:
- Do not change protocol behavior, wire semantics, crypto logic, or state machines unless the selected NEXT_ACTIONS item explicitly allows it.
- Keep fail-closed everywhere.
- Follow AGENTS/goal-lint rules: any docs/ changes require proper Goals line and governance updates as needed.
- Execute the top-most READY item in NEXT_ACTIONS.md in order. Do not reorder.
- Freeze: NA promotions paused after NA-0028 until public-release prep completes.

Start by:
1) Quoting the exact NEXT_ACTIONS entry you will execute (ID + title + scope flags).
2) Implementing the smallest fail-closed change set required.
3) Providing CI evidence (checks green; artifact paths or run links if available).

If you cannot proceed, stop and state exactly what blocked you and which file/section you checked.

## 1) Status legend

- READY: Unblocked; proceed now.
- BACKLOG: Planned; not promoted; may be skipped until promoted.
- BLOCKED: Requires prerequisites before work can begin.
- IN_PROGRESS: Actively being worked.
- DONE: Completed and merged to main.

## 2) Queue (execute in order; do not reorder)

Each item includes scope flags:
- Wire/behavior change allowed? YES/NO
- Crypto/state-machine change allowed? YES/NO
- Docs-only allowed? YES/NO

---

### NA-0001 — Establish repo-local operational spine (START_HERE + NEXT_ACTIONS) and index it












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES

Objective:
- Ensure new chats/sessions cannot drift: repo contains the authoritative entrypoint + the ordered queue.

Deliverables:
- Add START_HERE.md (repo root) with authoritative sources, constraints, workflow, overlay guidance.
- Add NEXT_ACTIONS.md (repo root) with new-chat starter and ordered queue.
- Update the Document Master Index (if present) to include these files.
- Ensure new docs satisfy goal-lint rules (Goals line, classification, governance requirements).

Acceptance criteria:
- goal-lint passes.
- No protocol/behavior changes introduced.
- Reviewers can start a new chat and proceed deterministically.

Evidence:
- Green goal-lint.
- Green qshield-ci and suite2-ci (or documented rationale if CI not triggered by docs-only change).

---

### NA-0002 — Create “Document Spine Map” to reduce doc overload (no moving files yet)












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES

Objective:
- Provide a single navigational map so the project has a small authoritative core and everything else is clearly subordinate.

Deliverables:
- Add a concise table to the Document Master Index (or create docs/master/DOC-CTRL-001 update) that classifies:
  - Governance spine (authoritative)
  - Canonical specs (authoritative)
  - Test plan/categories (authoritative for testing)
  - Inputs/vectors (authoritative for test data)
  - Supporting docs (non-authoritative)
- Add “Doc Classification” headers to major documents that are ambiguous.

Acceptance criteria:
- A newcomer can identify which 8–10 documents matter without reading the entire repo.
- No semantics changed; purely navigation and classification.

Evidence:
- goal-lint green.

---

### NA-0003 — Complete DOC-CAN-003 (Suite-2 / QSP v5.0) to self-contained, implementable normative spec












Status: DONE  
Wire/behavior change allowed? NO (docs only until implementation items)  
Crypto/state-machine change allowed? NO (docs only)  
Docs-only allowed? YES

Objective:
- Make Suite-2 canonical spec complete and unambiguous, aligned with existing CI-gated categories.

Deliverables (minimum):
- Define normative:
  - version/suite namespaces for v5.0 lane
  - transcript binding and capability commitment rules (fail-closed)
  - full Suite-2 key schedule including KDF_HYBRID(ec_mk, pq_mk) per message
  - ratchet semantics and strict reject rules (bounds, OOO handling)
  - explicit error handling and security invariants
- Remove or resolve TODOs that impact interoperability or security.

Acceptance criteria:
- The spec is self-contained; no “required meaning” deferred to Phase 2/3 docs.
- All security-sensitive ambiguity resolved; fail-closed rules explicit.
- Governance updates recorded (DECISIONS/TRACEABILITY).

Evidence:
- goal-lint green.
- suite2-ci green (if docs changes trigger relevant checks).

---

### NA-0004 — Complete DOC-CAN-004 (SCKA) to implementation-grade normative spec












Status: DONE  
Wire/behavior change allowed? NO (docs only until implementation items)  
Crypto/state-machine change allowed? NO (docs only)  
Docs-only allowed? YES

Objective:
- Make SCKA spec complete and aligned with CAT-SCKA-LOGIC-001 and CAT-SCKA-KEM-001.

Deliverables (minimum):
- Define epoch rules: monotonicity, one-time consumption, persistence invariants, tombstoning.
- Define all message fields and what must be bound into transcript/AD.
- Define reject rules: epoch regression, malformed hints, unexpected transitions (fail-closed).
- Define how SCKA outputs drive PQ reseed events into Suite-2 ratchet.

Acceptance criteria:
- Implementation can be written from the spec without reading tests as “spec substitute.”
- Governance updates recorded (DECISIONS/TRACEABILITY).

Evidence:
- goal-lint green.
- suite2-ci green.

---

### NA-0005 — Expand Suite-2 test categories to protocol-level composition coverage












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES (initially), then YES/NO depending on harness changes

Objective:
- Move from “atomic building blocks” to end-to-end behavior properties.

Deliverables:
- Add categories to DOC-TST-005 for:
  - transcript binding + negotiation enforcement
  - per-message KDF_HYBRID correctness
  - PQ reseed events: epoch -> pq_chain integration
  - OOO/replay windows and deterministic rejects
  - crash/restart scenarios aligned to durability gates

Acceptance criteria:
- Each category includes clear pass/fail criteria, vector schema expectations, and CI gating intention.

Evidence:
- goal-lint green.

---

### NA-0006 — Implement Suite-2 end-to-end in refimpl (minimal surface, fail-closed)












Status: DONE  
Wire/behavior change allowed? YES (implementation-only, not changing existing wire; adds Suite-2 lane)  
Crypto/state-machine change allowed? YES (Suite-2 only, scoped)  
Docs-only allowed? NO

Objective:
- Enable real execution of protocol-level Suite-2 vectors and interop.

Deliverables:
- Minimal Suite-2 engine integration with:
  - transcript binding enforcement
  - per-message PQ chain advancement
  - SCKA epoch integration and reject rules
  - strict parsing and state transitions

Acceptance criteria:
- suite2-ci green with protocol-level vectors.
- Existing lanes remain green; no regressions.

Evidence:
- Green qshield-ci lanes (relevant).
- Green suite2-ci.
- Uploaded artifacts demonstrating vector runs.
- PRs merged: #24, #25, #26, #27, #28, #29, #30, #31, #32, #33.

---

### NA-0007 — Extend durability/rollback gates to cover Suite-2 + SCKA persistence












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? YES (persistence semantics enforcement)  
Docs-only allowed? NO

Objective:
- Ensure Suite-2 cannot bypass rollback/replay defenses.

Deliverables:
- Suite-2 persistent state format/versioning.
- Rollback detection that binds SCKA epoch state.
- Durability CI cases for crash/restart with Suite-2.

Acceptance criteria:
- Durability lane remains fail-closed.
- Explicit evidence artifacts exist for Suite-2 durability.

Evidence:
- Green qshield-ci lanes (relevant).
- Green ci-4d-dur.
- CAT-S2-CRASH-001 vectors + runner in suite2-ci.
- PRs merged: #35.

---

### NA-0008 — Formal verification skeleton (tool decision + first executable model)












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES initially

Objective:
- Convert “plan” into a checked artifact; begin G4 formalization.

Deliverables:
- Record decision: chosen tool(s) and scope for first model.
- Add `formal/` directory with:
  - minimal roles and channels
  - declared secrecy/auth goals
  - CI hook that runs fail-closed (even if initial model is narrow)

Acceptance criteria:
- Model runs in CI.
- Model scope is explicitly limited and traced.

Evidence:
- Green CI job for formal lane.

---

### NA-0009 — Interop expansion (independent actor or second implementation surface)












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO (unless implementing second actor in parallel with same semantics)  
Docs-only allowed? NO

Objective:
- Prove semantics are not “single-implementation artifacts.”

Deliverables:
- Independent interop actor or minimal second implementation of key schedule/handshake.
- Interop CI gating with evidence artifacts.

Acceptance criteria:
- Interop vectors pass between independent implementations.

Evidence:
- Green qshield-ci lanes (relevant).
- Green suite2-ci interop gating:
  - CAT-S2-KDF-001 / CAT-S2-TRANSCRIPT-001 / CAT-S2-MK-001 against python interop actor (PR #37).
  - CAT-S2-INTEROP-XIMPL-001 cross-impl wire interop (PR #38).
- PRs merged: #37, #38.

---

### NA-0010 — Doc rationalization (deprecate or fold supporting docs; keep authoritative spine small)












Status: DONE
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES

Objective:
- Reduce cognitive load without losing information.

Deliverables:
- A deprecation policy and “where to look now” notes.
- Update master index to mark:
  - authoritative vs supporting vs deprecated
- Consolidate redundant “plans” into one place; avoid multiple competing roadmaps.

Acceptance criteria:
- Authoritative spine remains ≤ 10 documents.
- Supporting docs are discoverable but not mandatory reading.

Evidence:
- goal-lint green.

---
End of NEXT_ACTIONS.md

---

### NA-0011 — Specify Suite-2 session establishment and negotiation mapping (Suite-2 only)












Status: DONE
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO
Docs-only allowed? YES

Objective:

- Remove ambiguity about how a Suite-2 session is established, including negotiation, initial key material, and what (if anything)
  is reused from Suite-1/Suite-1B handshake semantics.
- Ensure implementation work does not invent non-canonical behavior.

Deliverables:

- DOC-CAN-003 update that explicitly defines (Suite-2 only):
  - How peers negotiate Suite-2 (capability commitment and downgrade rules already exist; clarify establishment).
  - How session_id is created/validated for Suite-2 sessions.
  - What inputs produce the initial Suite-2 ratchet state (root/chain keys, header keys), including any required transcript binding.
  - Whether Suite-2 reuses an existing handshake format or defines a new establishment message type (and if reused, exactly what is
    bound/derived differently).
  - Fail-closed reject rules for malformed/ambiguous establishment messages or missing prerequisites.

Acceptance criteria:

- The Suite-2 establishment story is self-contained in DOC-CAN-003 with explicit algorithms/field bindings and reject conditions.
- No implementation changes are required to “interpret” establishment (i.e., implementers can code directly from DOC-CAN-003).

Evidence:

- PR #41 merged (DOC-CAN-003 §6 establishment mapping; DOC-SCL-002 establishment reject identifiers; governance updates).
- Green CI on PR #41 (goal-lint, suite2-vectors, qshield-ci lanes).

---

### NA-0012 — Implement Suite-2 session establishment in actors/harness (Suite-2 only; no Suite-1/1B behavior changes)












Status: DONE
Wire/behavior change allowed? NO (Suite-2 only; must not alter Suite-1/Suite-1B wire/behavior)
Crypto/state-machine change allowed? YES (Suite-2 only, scoped)
Docs-only allowed? NO

Objective:

- Enable Suite-2 sessions to be created via standard actor/harness flows without relying on ad-hoc raw-wire-only entrypoints.

Deliverables:

- Actor support for Suite-2 session establishment consistent with DOC-CAN-003 NA-0011 output (fail-closed).
- Harness integration so Suite-2 can use sessionful operations where applicable (without changing Suite-1/1B behavior).
- CI-gated vectors covering establishment success + reject cases.

Acceptance criteria:

- suite2-ci includes establishment vectors and passes.
- Existing lanes remain green; Suite-1/1B unaffected.

Evidence:

- PR #45 merged (NA-0012 establish op + vectors + runner + suite2-ci wiring).
- Merge commit: a8d647b.
- CI: suite2-vectors https://github.com/Tebbens4832/qsl-phase4_4b3/actions/runs/20633344442/job/59254831314
- CI: goal-lint https://github.com/Tebbens4832/qsl-phase4_4b3/actions/runs/20633344439/job/59254831311
- Local: `python3 scripts/ci/run_suite2_establish_vectors.py --actor target/release/refimpl_actor --actor-name suite2-establish` => 12/12 OK.

---

### NA-0013 — Suite-2 sessionful harness flow adoption (wire-neutral)












Status: DONE
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO (actors/harness only)
Docs-only allowed? NO

Objective:

- Replace any remaining ad-hoc/raw-wire Suite-2 entrypoints in harness usage with sessionful establish→send/recv flows.
- Ensure suite2-ci uses suite2.establish.run consistently for all session-based tests where applicable.

Deliverables:

- Update harness runner(s) / actor invocation patterns to create sessions via suite2.establish.run.
- Ensure suite2-ci evidence artifacts include establish report + e2e send/recv reports for sessionful lanes.
- Add/adjust minimal vectors only if required to preserve coverage (but this PR is governance-only; implementation comes later).

Acceptance criteria:

- suite2-ci remains green after follow-on implementation PR.
- No Suite-1/1B behavior change; no qsp/* changes.

Evidence:

- PR #48 merged.
- Merge commit: 2bdf5eb6776bf992cd44d78955cf86688b5824ab.
- CI: suite2-vectors https://github.com/Tebbens4832/qsl-phase4_4b3/actions/runs/20641549573/job/59273923369
- CI: goal-lint https://github.com/Tebbens4832/qsl-phase4_4b3/actions/runs/20641549575/job/59273923335
- Sessionful establish executed: crash/restart 3/3, interop 3/3, ximpl 2/2.

---

### NA-0014 — Goal-lint hardening for governance PRs (optional)












Status: DONE
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO
Docs-only allowed? YES

Objective:

- Document the exact goal-lint PR-body requirements and add a short template snippet to CHAT_STARTER.md or AGENTS.md.

Evidence:
- AGENTS.md updated with explicit Goals line format (ASCII commas; no ranges).
- DECISIONS.md entry D-0033 recorded the governance requirement.

### NA-0015 — Build polished Linux demo CLI + local relay (Suite-2 only; non-production demo)











Status: DONE
Wire/behavior change allowed? NO (protocol wire must remain unchanged)
Crypto/state-machine change allowed? NO (protocol core unchanged; demo layer only)
Docs-only allowed? NO

Objective:
- Produce a polished Linux CLI demonstrator that exercises Suite-2 session lifecycle end-to-end:
  establish → sessionful send/recv → persistence/restart → replay/rollback resistance.
- Provide a repeatable demo script suitable for expert/investor review without overstating threat model.
- Default relay is local for deterministic demos; remote deployment is optional later and not part of baseline.

Naming/UX conventions (must follow):
- Installed binary name: `qshield`
- Repo/app directory: `apps/qshield-cli/`
- Rust package/crate name: `qshield-cli` (or `qshield_cli`), but the produced/installed binary MUST be named `qshield`
- Relay is implemented as a subcommand (baseline; no separate relay binary):
  - `qshield relay serve`
- All docs/scripts MUST use `qshield ...` (not qshield-cli)

Deliverables (staged):
- Phase 1: Demo interface contract + storage/relay model (short spec + demo script plan).
- Phase 2: Working CLI vertical slice:
    init, relay serve, register, establish, send, recv, status.
- Phase 3: Demo relay robustness:
    queueing, polling, at-most-once OPK semantics, audit logging.
- Phase 4: Polish:
    consistent errors/help, deterministic scripts, packaging instructions, “demo in 5 minutes” README.
- Phase 5 (optional): demo-ci lane that runs the scripted demo on Linux.

Acceptance criteria:
- A fresh Linux host can run the demo via documented commands and obtain expected output.
- Demo uses suite2.establish.run and session_id send/recv (no state injection).
- Clear “non-production demo” disclaimers; no metadata-minimization claims beyond implemented features.
- Existing CI lanes remain green; no changes to protocol wire.

Evidence:
- Demo script path(s) and sample transcript (kept short).
- PR #54 merged — NA-0015 PR1 scaffold (apps/qshield-cli, relay stub, demo-cli-build job). Merge commit: 66c8d7b8c568348ae254c8d1baf1fd78b9421f4a.
- PR #55 merged — NA-0015 PR2 vertical slice (init/register/establish/send/recv + demo-cli-smoke). Merge commit: 0865eb1c5eb947cb650de99e9a39337a8ffcf33e.
- PR #56 merged — NA-0015 PR3 crypto-real demo (sessionful Suite-2 establish + encrypted send/recv via local relay; negotiated u16 fix). Merge commit: 31e9ac527b4f8aedbaf2452ff38b078291e7e5c5. Merged at: 2026-01-01T21:38:55Z.
- CI: demo-cli-build and demo-cli-smoke jobs present/enforced in .github/workflows/ci.yml (qshield demo lane).


### NA-0016 — Metadata minimization lane (G5): threat model + leakage analysis + envelope/transport profile (non-anonymity baseline)












Status: DONE
Wire/behavior change allowed? YES (envelope/transport profile may require wire-visible framing; keep Suite-1/1B unchanged unless explicitly authorized)
Crypto/state-machine change allowed? MAYBE (only if required by envelope; prefer no changes to Suite-2 core)
Docs-only allowed? NO (must include at least one executable check / conformance artifact)

Objective:

- Define, document, and enforce a concrete “metadata minimization” posture (G5) that is honest, measurable, and testable.
- Establish a baseline that is NOT an anonymity network, but demonstrably reduces avoidable metadata leakage compared to naive relays.

Scope (explicit):

- In-scope:
  - Threat model for metadata (who/what is being protected against; attacker capabilities).
  - Leakage analysis for the current demo relay model (what is exposed today).
  - Envelope/transport profile: identifiers, headers, error behavior, and relay API that minimize linkability where feasible.
  - Padding policy: message size buckets, optional cover padding, and observable size leakage characterization.
  - Timing policy (baseline): optional batching windows or jitter knobs; document tradeoffs.
  - Relay logging/retention posture: required/forbidden logs; default retention limits; auditability hooks.
  - Conformance checks: at least one CI-gated test that enforces envelope/profile invariants (fail-closed).

- Out-of-scope (for NA-0016):
  - Mixnets, PIR, private contact discovery, global cover traffic systems, anonymous routing guarantees (Tor/I2P may be optional integration later but not required here).
  - “Metadata eliminated” claims.

Deliverables:

1) G5 threat model document (short, authoritative)
   - Define metadata categories: contact graph, timing, size, IP/location, server-side linkability, identifiers.
   - Define attackers: relay operator, network observer, active probing adversary, compromised client.
   - Define success metrics: what is minimized vs what remains exposed.

2) Leakage analysis for current design
   - Enumerate all observable fields for:
     - client→relay requests
     - relay→client responses
     - stored relay records
   - Provide an explicit leakage table (field → observer → persistence → mitigation).

3) Envelope/transport profile specification (baseline)
   - Define:
     - stable vs rotating identifiers
     - session identifiers exposure rules
     - error normalization policy (which errors are uniform vs distinct; fail-closed preserved)
     - replay/dup handling visibility
     - relay API fields that must be opaque/constant-size where feasible
   - Include explicit non-goals and disclaimers.

4) Padding/timing profile (baseline knobs)
   - Size-bucket padding profile (e.g., 256/512/1024/2048/...); document overhead.
   - Optional batching/jitter profile (documented tradeoffs; default OFF unless justified).
   - Test vectors or fixtures demonstrating bucket behavior.

5) CI-gated conformance artifact(s)
   - Add a test/runner that fails CI if:
     - envelope fields violate the profile (e.g., variable-length identifiers where forbidden),
     - padding policy is not applied when configured,
     - relay logs/retention violate required defaults (where enforceable).
   - Evidence artifacts captured in CI.
6) Demo relay/transport safety hardening requirements (non-production but must be safe-by-default):
   - default loopback-only MUST remain; any non-loopback exposure MUST require explicit authz gating
   - unauthenticated register/send/poll MUST be prohibited (capability token or equivalent required)
   - request size limits and per-recipient/global queue caps MUST exist (fail-closed on overflow)
   - relay MUST NOT leak plaintext identifiers where avoidable; padding strategy documented (ties to G5)
   - demo clients MUST NOT assert authenticated=true unless backed by concrete evidence, or they MUST require an explicit “demo-unauthenticated override” flag that is OFF by default.
7) Documentation target placeholders (to be drafted):
   - docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md
   - docs/privacy/DOC-G5-002_Metadata_Leakage_Inventory_v1.0.0_DRAFT.md
   - docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md
   - scripts/ci/metadata_conformance_smoke.sh (executable check to be implemented)
   - CI job: metadata-conformance (or extend demo-cli-smoke) (to be implemented)

Acceptance criteria:

- G5 has a concrete, published threat model + leakage table + envelope/profile spec.
- A minimal set of invariants are enforced by CI (fail-closed); “metadata minimization” is not just a claim.
- Clear statements of residual leakage and non-goals (no overclaiming).
- Existing lanes remain green; Suite-2 cryptographic core unaffected unless explicitly justified and documented.
- CI-gated metadata conformance check MUST include assertions that:
  - relay does not accept public binding without an explicit auth token (or feature gate)
  - register/send/poll fail-closed without credentials
  - request bodies above a bound are rejected
  - queues are capped and eviction/reject behavior is deterministic

Evidence:

- PR #61 merged — NA-0016 PR1 safe-by-default relay/CLI hardening + CI metadata conformance smoke. Merge commit: 057c2a3e07bb3962480077bd65c719d8d1a1bba7.
- PR #62 merged — NA-0016 PR2 docs backbone (threat model + leakage inventory + transport profile v0.1) + traceability. Merge commit: b1fd6e1e8fbbcc21aa03b7cdbabfa3f13a225c56.
- PR #63 merged — NA-0016 PR3 strengthened conformance invariants + local store permission hardening (0700/0600). Merge commit: 8419cb929d087451dc904dcdf7de74c0a329f479.
- PR #64 merged — NA-0016 PR4 optional size-bucket padding envelope + conformance assertions. Merge commit: 9434adbe296e62ec6d792fe4f2efb4f3b2796642.
- CI: metadata-conformance-smoke job present/enforced in .github/workflows/ci.yml and passes on NA-0016 PRs.
- CI: demo-cli-smoke continues to pass (no regressions).

---

### NA-0017 — Comparative benchmark review: Signal protocol/transport hardening patterns (read-only, clean-room) (G4/G5)












Status: DONE
Wire/behavior change allowed? NO (this lane is analysis/artifacts only)
Crypto/state-machine change allowed? NO
Docs-only allowed? YES

Objective:
- Extract hardening patterns, invariants, and test strategies from Signal’s publicly available specs and repos.
- Produce a bounded “delta matrix” mapping candidate improvements into QSL Goals and future NEXT_ACTIONS items.
- Strict clean-room posture: no code copying; paraphrase only; reference Signal files by path + line ranges.

Deliverables:
- One doc: docs/review/DOC-REV-001_Signal_Comparative_Review_v1.0.0_DRAFT.md containing:
  1) Delta Matrix (≤ 25 rows):
     - Area (fail-closed, parsing, PQ transition, metadata, storage, testing)
     - Signal approach (paraphrase + file/spec cite)
     - QSL current posture (cite QSL files/docs)
     - Gap/risk (1 sentence)
     - Proposed QSL action (1 sentence)
     - Queue mapping (new NA suggestions + Goal IDs)
     - CI-gating idea (how to enforce)
  2) Top-5 actionable upgrades (ranked), each with:
     - bounded PR plan + acceptance criteria + evidence
  3) Non-goals list (≤ 5 items) to avoid scope creep

Acceptance criteria:
- DOC-REV-001 exists and meets the bounded structure above.
- All recommended follow-on actions are mapped to Goals (G1–G5) and expressed as candidate NEXT_ACTIONS entries.
- No Signal code copied into QSL (paraphrase + citations only).

Evidence:
- PR #67 merged — NA-0017 PR1 DOC-REV-001 scaffold. Merge commit: a9c30fe5b267f0cfcc87b878ced78635d51a3075.
- PR #68 merged — NA-0017 PR2 populated DOC-REV-001 + decision/trace/testplan. Merge commit: c7aab85851b97f96c20103b2a8bc544d45de92ae.
- In-tree artifact: docs/review/DOC-REV-001_Signal_Comparative_Review_v1.0.0_DRAFT.md (clean-room comparative review).

---

### NA-0018 — One-time prekey lifecycle + at-most-once consumption (demo relay semantics)












Status: DONE  
Wire/behavior change allowed? YES (demo relay semantics only)  
Crypto/state-machine change allowed? NO (no protocol-core changes)  
Docs-only allowed? NO

Objective:
- Enforce one-time consumption for demo relay prekeys/bundles with deterministic fail-closed reuse rejection.
- Define demo relay lifecycle semantics without anonymity claims or protocol-core changes.
- Add CI-gated conformance checks for single-consumption and reuse rejection.

Deliverables:
- Demo relay lifecycle specification update (prekey/bundle issuance, consumption, and reuse rejection).
- Demo relay implementation enforcing at-most-once consumption and deterministic reject behavior.
- CI: extend `scripts/ci/metadata_conformance_smoke.sh` (or add a small deterministic establish-smoke) to assert one-time consumption.
- Test plan entry: `docs/archive/testplans/NA-0018_prekey_lifecycle_testplan.md` with pass/fail criteria.

Acceptance criteria:
- A consumed prekey/bundle cannot be reused; reuse attempts reject deterministically without state mutation.
- Conformance checks fail-closed if reuse is accepted or consumption is not recorded.
- No Suite-2 protocol-core or qsp/* changes.

Evidence:
- Demo relay /consume endpoint with at-most-once bundle consumption.
- CI gate: scripts/ci/metadata_conformance_smoke.sh asserts consume + reuse rejection.
- Test plan: docs/archive/testplans/NA-0018_prekey_lifecycle_testplan.md.

---

### NA-0019 — Explicit identity binding for demo establish












Status: DONE  
Wire/behavior change allowed? YES (demo relay semantics only)  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Bind demo establish to explicit identity inputs to prevent ambiguous pairing.
- Add fail-closed reject cases for missing or mismatched identity binding.

Deliverables:
- Demo establish specification update with identity binding inputs and reject rules.
- Demo relay/CLI changes to include identity binding in establish flow.
- CI vectors/tests for missing/mismatched binding rejects.

Acceptance criteria:
- Establish fails deterministically when identity binding is absent or mismatched.
- Conformance checks gate the negative cases.

Evidence:
- Establish identity binding check in demo establish (bundle.id == peer_id).
- CI gate: metadata_conformance_smoke asserts missing/mismatch rejects without consumption.
- Test plan: docs/archive/testplans/NA-0019_identity_binding_testplan.md.

---

### NA-0020 — Establishment replay cache (session_id + bundle identifiers) for demo relay/harness












Status: DONE  
Wire/behavior change allowed? YES (demo relay semantics only)  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Add a demo relay/harness replay cache for establish operations keyed by session_id and bundle identifiers.
- Reject deterministic replays without changing protocol-core semantics.

Deliverables:
- Replay-cache specification for demo establish inputs and retention rules.
- Relay/harness implementation for replay detection and fail-closed rejection.
- CI vectors validating deterministic replay rejection.

Acceptance criteria:
- Identical establish inputs replay deterministically reject.
- Conformance checks fail if replay is accepted or cache is bypassed.

Evidence:
- Demo relay /establish_record endpoint with replay fingerprinting.
- CI gate: metadata_conformance_smoke asserts replay rejection without consumption.
- Test plan: docs/archive/testplans/NA-0020_establish_replay_cache_testplan.md.

---

### NA-0021 — Relay rate limiting/backoff (register/poll abuse resistance)












Status: DONE  
Wire/behavior change allowed? YES (demo relay semantics only)  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Add deterministic rate limits/backoff for demo relay register/poll paths.
- Ensure rate limiting is observable and fail-closed above thresholds.

Deliverables:
- Demo relay rate-limit policy and error normalization notes.
- Implementation of deterministic limits with clear reject codes (e.g., 429).
- CI conformance checks for threshold exceedance.

Acceptance criteria:
- Requests above limits reject deterministically with normalized errors.
- CI checks verify rate-limit enforcement.

Evidence:
- Demo relay rate limiting for /register and /poll (429 with retry_after_ms).
- CI gate: metadata_conformance_smoke asserts observed 429 for register/poll loops.
- Test plan: docs/archive/testplans/NA-0021_rate_limit_testplan.md.

---

### NA-0022 — Identifier collision handling + format guidance for relay IDs












Status: DONE  
Wire/behavior change allowed? YES (demo relay semantics only)  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Define allowed demo relay identifier formats and collision handling.
- Reject duplicate registrations deterministically.

Deliverables:
- Relay ID format guidance in demo docs and error normalization notes.
- Relay implementation rejecting duplicate ID registrations.
- CI checks ensuring collision rejects are enforced.

Acceptance criteria:
- Duplicate ID registration rejects deterministically.
- Conformance checks gate collision handling.

Evidence:
- Demo relay rejects invalid id format (400) and duplicate /register (409).
- CI gate: metadata_conformance_smoke asserts duplicate/invalid id rejection.
- Test plan: docs/archive/testplans/NA-0022_identifier_collision_testplan.md.

---

### NA-0023 — Explicit eviction/deletion invariants for skipped keys + bounds












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? YES (Suite-2 OOO/skip-key handling only)  
Docs-only allowed? NO

Objective:
- Define eviction/deletion rules for skipped keys with clear bounds.
- Add negative vectors to ensure eviction and non-reuse.

Deliverables:
- DOC-CAN-003 update for skipped-key eviction/deletion invariants and bounds.
- Suite-2 vector category updates for eviction/skip-key negative cases.
- CI-gated vectors ensuring evicted keys cannot be reused.

Acceptance criteria:
- Eviction rules are explicit and fail-closed.
- CI vectors reject reuse of evicted or deleted skipped keys.

Evidence:
- DOC-CAN-003 §9.1.1/§9.3 updated with MKSKIPPED eviction + delete-on-use bounds.
- Suite-2 ratchet enforces deterministic eviction and delete-on-use (`tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`).
- OOO replay vectors add reuse/eviction negatives (`inputs/suite2/vectors/qshield_suite2_ooo_replay_vectors_v1.json`) gated by suite2-vectors CI.

---

### NA-0024 — Map PQXDH-style bundle outputs to SCKA initial epoch rules (doc + vectors)












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES (docs/vectors/CI wiring only)

Objective:
- Define a doc-only mapping from PQXDH-style bundle outputs to SCKA initial epoch rules.
- Add vectors to lock down the mapping behavior.

Deliverables:
- DOC-CAN-004 mapping section for initial epoch derivation.
- CAT-SCKA or Suite-2 vector additions to enforce mapping consistency.
- CI wiring for the mapping vectors.

Acceptance criteria:
- Mapping is self-contained and implementable without external references.
- Vectors enforce the mapping; CI fails on mismatch.

Evidence:
- DOC-CAN-004 §3.5 mapping from PQXDH-style bundle outputs to SCKA epoch 0.
- SCKA logic vectors lock the mapping (`inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`).
- Test plan: docs/archive/testplans/NA-0024_pqxdh_scka_epoch_mapping_testplan.md.

---

### NA-0025 — PQ KEM public key / prekey identifier binding in establishment transcript/AD












Status: DONE  
Wire/behavior change allowed? NO (no new wire)  
Crypto/state-machine change allowed? YES (Suite-2 establish binding logic)  
Docs-only allowed? NO

Objective:
- Bind PQ KEM public key and prekey identifiers into Suite-2 establishment transcript/AD.
- Add fail-closed reject cases for missing or mismatched binding.

Deliverables:
- DOC-CAN-003 establishment updates defining the PQ binding inputs and ordering.
- Suite-2 establish vectors for missing/mismatched PQ binding.
- Actor/harness updates to enforce binding in establish checks.

Acceptance criteria:
- Establish rejects deterministically when PQ binding is missing or mismatched.
- CI vectors gate the binding behavior.

Evidence:
- DOC-CAN-003 §6.3/§6.6 adds PQ KEM pub/prekey binding requirements and reject codes.
- Suite-2 establish vectors add PQ binding negatives (`inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json`) gated by suite2-vectors CI.
- Test plan: docs/archive/testplans/NA-0025_pq_binding_testplan.md.

---

### NA-0026 — Secure deletion/rotation policy for demo store artifacts + skipped keys












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Define secure deletion/rotation policy for demo store artifacts and skipped keys.
- Add CI checks for deletion/rotation and permissions.

Deliverables:
- Demo store lifecycle doc updates (deletion/rotation policy and bounds).
- Implementation for deterministic deletion/rotation of demo store artifacts.
- CI checks for lifecycle enforcement and permissions.

Acceptance criteria:
- Deletion/rotation policy is explicit and enforced.
- CI checks fail-closed if lifecycle rules are violated.

Evidence:
- Demo store rotation command wipes config/state (`apps/qshield-cli/src/commands/rotate.rs`).
- CI gate: metadata_conformance_smoke checks perms + rotate deletion.
- Test plan: docs/archive/testplans/NA-0026_store_lifecycle_testplan.md.

---

### NA-0027 — Demo UX: surface identity verification / warnings on first establish












Status: DONE  
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Ensure demo CLI surfaces identity verification guidance on first establish.
- Require explicit override flags to suppress warnings.

Deliverables:
- Demo CLI UX update for first-establish warnings and override handling.
- Documentation updates for identity verification guidance.
- CI checks confirming warnings appear by default.

Acceptance criteria:
- CLI warns on first establish unless an explicit override is provided.
- CI checks enforce the warning behavior.

Evidence:
- Demo CLI first-establish warning (`apps/qshield-cli/src/commands/establish.rs`) + `--demo-identity-verified` override.
- CI gate: metadata_conformance_smoke asserts warning shown/suppressed.
- Test plan: docs/archive/testplans/NA-0027_identity_warning_testplan.md.

---

### NA-0028 — Per-token quotas to reduce probing within queue caps












Status: DONE  
Wire/behavior change allowed? YES (demo relay semantics only)  
Crypto/state-machine change allowed? NO  
Docs-only allowed? NO

Objective:
- Add per-token quotas to reduce probing/abuse within existing queue caps.
- Ensure deterministic backoff/reject behavior.

Deliverables:
- Demo relay quota policy (per-token limits + error normalization).
- Implementation enforcing per-token quotas.
- CI conformance checks for quota enforcement.

Acceptance criteria:
- Over-quota requests reject deterministically.
- CI checks gate per-token quota enforcement.

Evidence:
- Demo relay per-token quota enforced in /send (429 on overflow).
- CI gate: metadata_conformance_smoke asserts token quota 429 with error string.
- Test plan: tests/NA-0028_token_quota_testplan.md.


### NA-0029 — Audit closure: verify + remediate remaining findings (public primary)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by an identified audit finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Inventory remaining audit findings, confirm what is already fixed, and close the highest-priority open finding with deterministic reject behavior and CI tests that enforce the invariant.

Deliverables:

- Audit finding inventory note in PR description (with file/line anchors).
- Implementation (if needed) of the selected finding’s mitigation (fail-closed; no state mutation on reject).
- Tests proving both properties: reject is deterministic and state is unchanged on rejected inputs.

Acceptance criteria:

- All required CI checks green.
- Added/updated tests prove the new invariant.

Evidence:

- PR verification bundle with: name-only diff, scope guard, key excerpts, CI links, and post-merge anchors.

---

### NA-0030 — Audit closure: close Issue #9 (Missing key zeroization)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (fail-closed only; no wire changes)
Docs-only allowed? NO

Objective:

- Close audit Issue #9 (“Missing key zeroization”) by ensuring secret key material is zeroized on drop/overwrite, with tests proving:
  (1) deterministic reject behavior for invalid inputs if applicable, and
  (2) no state mutation on rejected inputs where reject paths exist.

Deliverables:

- Minimal mitigation implementation for Issue #9 (use existing zeroize dependency).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove the new invariant.

Evidence:

- PR verification bundle + post-merge anchors.

---

### NA-0031 — Audit triage: Issue #6 ck_pq_recv boundary handling (spec-sensitive)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO unless spec requires (must be explicitly justified)
Crypto/state-machine change allowed? YES (only if required; fail-closed)
Docs-only allowed? YES (spec check + plan allowed)

Objective:

- Resolve audit Issue #6 by checking intended spec behavior for ck_pq_recv boundary handling and then implementing the minimal
  fail-closed mitigation consistent with the spec.

---

### NA-0032 — Audit closure: close Issue #10 (Timing side-channel in header decryption)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #10 by eliminating timing side-channels in header decryption and adding regression guards.

Deliverables:

- Minimal mitigation implementation for Issue #10 (fail-closed; no timing-dependent early exit).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior without timing-dependent branches in header decryption.

Evidence:

- PR verification bundle + post-merge anchors.

---

### NA-0033 — Audit closure: close Issue #12 (take_mk_skipped leaves stale mk_order)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #12 by ensuring take_mk_skipped does not leave stale mk_order and add regression guards.

Deliverables:

- Minimal mitigation implementation for Issue #12 (fail-closed; no stale mk_order).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior where applicable and no state mutation on reject.

Evidence:

- PR verification bundle + post-merge anchors.

---

### NA-0034 — Audit closure: close Issue #13 (SCKA monotonicity check insufficient)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #13 by enforcing SCKA monotonicity checks and add regression guards.

Deliverables:

- Minimal mitigation implementation for Issue #13 (fail-closed; no monotonicity violation).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior where applicable and no state mutation on reject.

Evidence:

- PR verification bundle + post-merge anchors.

---

### NA-0035 — Audit closure: close Issue #14 (store_mk_skipped silent failure)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #14 by eliminating silent failure in store_mk_skipped and adding regression guards.

Deliverables:

- Minimal mitigation implementation for Issue #14 (fail-closed; no silent discard).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior where applicable and no state mutation on reject.

Evidence:

- PR verification bundle + post-merge anchors.

### NA-0036 — Audit closure: close Issue #15 (DH ratchet corrupts pn on ns overflow)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #15 by enforcing fail-closed behavior on pn/ns overflow in DH ratchet and adding regression guards.

Deliverables:

- Minimal mitigation implementation for Issue #15 (fail-closed; no silent corruption).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior where applicable and no state mutation on reject.

Evidence:

- PR verification bundle + post-merge anchors.

### NA-0037 — Audit closure: close Issue #16 (DoS via large collection deserialization)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #16 by enforcing bounded parsing / size limits to prevent DoS via oversized inputs, with deterministic reject
  behavior and tests proving no state mutation on reject.

Deliverables:

- Minimal mitigation implementation for Issue #16 (bounded decode / size checks; fail-closed).
- CI-exercised regression tests that prove deterministic reject and no mutation on reject.
- Audit table update + governance anchors + testplan in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove the invariant.

Evidence:

- PR verification bundle + post-merge anchors.

### NA-0038 — Audit closure: close Issue #17 (Multiple unwraps on header_pt)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #17 by removing panic paths in header processing (unwraps) and enforcing deterministic reject behavior with
  regression tests proving no state mutation on reject.

Deliverables:

- Minimal mitigation implementation for Issue #17 (no unwrap-based panics; fail-closed).
- CI-exercised tests that fail on regression.
- Audit table update + governance anchors + testplan in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior and no state mutation on reject.

Evidence:

- PR verification bundle + post-merge anchors.

### NA-0039 — Audit closure: close Issue #18 (Unsafe unwraps in OPK handling)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #18 by removing unwrap-based panic paths in OPK handling and enforcing deterministic reject behavior with CI tests
  proving no state mutation on reject.

Deliverables:

- Minimal mitigation implementation for Issue #18 (no unwrap panics; fail-closed reject).
- CI-exercised tests that fail on regression (deterministic reject + no state mutation on reject).
- Audit table update + governance anchors + testplan in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic reject behavior and no state mutation on reject.

Evidence:

- PR verification bundle + post-merge anchors.

### NA-0040 — Audit closure: close Issue #19 (State cloning proliferates key material)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? YES (only if required by the finding; must be fail-closed)
Docs-only allowed? NO

Objective:

- Close audit Issue #19 by eliminating unnecessary state cloning that proliferates key material, while preserving behavior and adding
  regression guards to prevent reintroduction.

Deliverables:

- Minimal mitigation implementation for Issue #19 (remove or confine key-material cloning).
- CI-exercised regression tests proving deterministic behavior and no state mutation on reject where applicable.
- Audit table update + governance anchors + testplan in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove the invariant.

Evidence:

- PR verification bundle + post-merge anchors.

### NA-0041 — Audit closure: close Issue #20 (Mutex::lock().unwrap() in CLI)












Status: DONE
Completed: 2026-01-11 — PR #48 (merge e0679dd)
Wire/behavior change allowed? NO (hardening + tests only unless explicitly justified)
Crypto/state-machine change allowed? NO (CLI hardening only)
Docs-only allowed? NO

Objective:

- Close audit Issue #20 by removing panic paths from poisoned mutex locking in the CLI relay command and adding regression guards for deterministic error behavior (no panic).

Deliverables:

- Minimal mitigation implementation for Issue #20 (no unwrap panics; deterministic error return).
- CI-exercised tests that fail on regression (panic-free + deterministic error behavior).
- Audit status table update + governance anchors + testplan in the fixing PR.

Acceptance criteria:

- All required CI checks green.
- Tests prove deterministic error behavior and no panic for the affected path.

Evidence:

- PR verification bundle + post-merge anchors.
### NA-0042 — Audit closure: close Issue #21 (MKSKIPPED removal without recovery in Suite-2 ratchet)












Status: DONE
Completed: 2026-01-11 — PR #50 (merge fc6c347)
Wire/behavior change allowed? NO (fail-closed rejects only; no wire changes)
Crypto/state-machine change allowed? YES (Suite-2 ratchet logic only; reject deterministically; no mutation on reject)
Docs-only allowed? NO

Objective:

- Close Audit Issue #21 by removing/mitigating MKSKIPPED state removal without recovery in Suite-2 ratchet, ensuring the implementation
  behaves deterministically and fail-closed under all invalid/edge inputs.

Deliverables:

- Minimal fix in Suite-2 ratchet code path(s) (expected: suite2/ratchet.rs) that:
    - eliminates MKSKIPPED removal without recovery (or replaces it with deterministic reject + safe recovery behavior),
    - guarantees no state mutation on reject paths,
    - preserves existing behavior for valid vectors.

- Regression tests proving:
    - deterministic reject error (stable error code/string),
    - no state mutation on reject (state snapshot equality),
    - existing green lanes remain green.

- Audit + governance updates:
    - Update the audit status row for Issue #21 (docs/audit/*) with mitigation summary + PR link.
    - Update DECISIONS.md and TRACEABILITY.md as required by project policy.
    - Add/extend a per-issue test plan under tests/ documenting vectors + invariants.

Acceptance criteria:

- Relevant CI lanes green (including any suite2/ratchet lanes).
- New tests assert deterministic reject + no mutation on reject for the Issue #21 scenario.
- Audit status table marks Issue #21 as CLOSED with PR reference.

Evidence:

- PR verification bundle: branch, commit, name-only diff + scope guard, key excerpts, CI checks (links + PASS).
- Post-merge verification: merge commit SHA and NEXT_ACTIONS READY/DONE anchors updated accordingly.
### NA-0043 — Audit closure: close Issue #22 (Boundary message window not enforced in Suite-2 ratchet)












Status: DONE
Completed: 2026-01-12 — PR #52 (merge 5d62c4e)
Wire/behavior change allowed? NO (reject-only; no wire changes)
Crypto/state-machine change allowed? YES (Suite-2 ratchet validation only; deterministic reject; no mutation on reject)
Docs-only allowed? NO

Objective:

- Close Audit Issue #22 by enforcing the Suite-2 boundary message window and rejecting out-of-window messages deterministically.

Deliverables:

- Minimal Suite-2 ratchet validation enforcing boundary window rules with deterministic reject and no state mutation on reject.
- Regression tests proving deterministic reject + no-mutation for out-of-window cases.
- Audit row update for Issue #22, plus DECISIONS/TRACEABILITY updates and a per-issue testplan.

Acceptance criteria:

- Relevant CI lanes green.
- New tests assert boundary-window reject behavior and no mutation on reject.
- Audit table marks Issue #22 CLOSED with PR reference.

Evidence:

- PR verification bundle + post-merge verification anchors.
### NA-0044 — Audit closure: close Issue #23 (ss3 entropy discarded in handshake)












Status: DONE
Completed: 2026-01-12 — PR #54 (merge a04d425)
Wire/behavior change allowed? NO (reject-only; no wire changes)
Crypto/state-machine change allowed? YES (handshake transcript/entropy binding validation only; deterministic reject; no mutation on reject)
Docs-only allowed? NO

Objective:

- Close Audit Issue #23 by ensuring ss3 entropy is not discarded and is bound into the handshake in a way that enforces reject rules.

Deliverables:

- Minimal handshake fix ensuring ss3 contributes to transcript/key schedule as specified (or deterministic reject if missing).
- Regression tests proving deterministic reject + no state mutation on reject for malformed/entropy-missing cases.
- Audit row update for Issue #23, plus DECISIONS/TRACEABILITY updates and a per-issue testplan.

Acceptance criteria:

- Relevant CI lanes green.
- New tests assert ss3 handling/binding and reject behavior.
- Audit table marks Issue #23 CLOSED with PR reference.

Evidence:

- PR verification bundle + post-merge verification anchors.
### NA-0045 — Audit closure: close Issue #24 (Hardcoded ZERO32 initialization in Suite-2 establish)












Status: DONE
Completed: 2026-01-13 — PR #57 (merge 3063676)
Wire/behavior change allowed? NO (reject-only; no wire changes)
Crypto/state-machine change allowed? YES (Suite-2 establish validation only; deterministic reject; no mutation on reject)
Docs-only allowed? NO

Objective:

- Close Audit Issue #24 by removing/guarding hardcoded ZERO32 cryptographic initialization in Suite-2 establish.

Deliverables:

- Minimal fix eliminating hardcoded ZERO32 usage for cryptographic operations (or deterministic reject if present).
- Regression tests proving deterministic reject + no state mutation on reject.
- Audit row update for Issue #24, plus DECISIONS/TRACEABILITY updates and a per-issue testplan.

Acceptance criteria:

- Relevant CI lanes green.
- New tests assert behavior and no-mutation-on-reject.
- Audit table marks Issue #24 CLOSED/FIXED with PR reference.

Evidence:

- PR verification bundle + post-merge verification anchors.
### NA-0046 — Audit closure: close Issue #25 (Inconsistent error types)












Status: DONE
Completed: 2026-01-14 — PR #60 (merge 2b2a24a)
Wire/behavior change allowed? NO (reject-only; error typing/normalization)
Crypto/state-machine change allowed? LIMITED (error plumbing only; deterministic reject strings; no mutation on reject)
Docs-only allowed? NO

Objective:

- Close Audit Issue #25 by normalizing inconsistent error types/codes in the affected module(s) to deterministic reject behavior.

Deliverables:

- Minimal error normalization fix (no protocol/wire changes).
- Regression tests proving deterministic reject strings/codes + no state mutation on reject.
- Audit row update for Issue #25, plus DECISIONS/TRACEABILITY updates and a per-issue testplan.

Acceptance criteria:

- Relevant CI lanes green.
- New tests assert the invariant(s).
- Audit table marks Issue #25 CLOSED/FIXED with PR reference.

Evidence:


### NA-0047 — Audit closure: close Issue #26 (Asymmetric initial state in Suite-2 establish)












Status: DONE
Completed: 2026-01-14 — PR #62 (merge f61fa82)
Wire/behavior change allowed? NO (guards/tests only)
Crypto/state-machine change allowed? NO (deterministic reject only)
Docs-only allowed? NO

Objective:
- Prevent asymmetric or unset initial establish state from progressing silently; fail-closed with deterministic reject.

Deliverables:
- Minimal guardrails in Suite-2 establish path for Issue #26.
- Deterministic reject string includes reason_code token.
- No state mutation on reject regression test.
- Audit status row update + DECISIONS/TRACEABILITY + per-issue testplan.

Acceptance criteria:
- Relevant CI lanes green (suite2-vectors + qshield-ci).
- Existing lanes remain green; no regressions.

Evidence:
- PR link + merge SHA recorded here on completion.


### NA-0048 — Audit closure: close Issue #27 (Signature verification order in QSP handshake)












Status: DONE
Completed: 2026-01-17 — PR #65 (merge f1c9e1b)
Wire/behavior change allowed? NO (no wire changes)
Crypto/state-machine change allowed? YES (ordering only; scoped to QSP handshake)
Docs-only allowed? NO

Objective:
- Ensure signature verification ordering cannot be abused to cause waste or side effects; fail-closed deterministically.

Deliverables:
- Minimal reorder/guardrail in QSP handshake verify path (Issue #27).
- Deterministic reject string includes reason_code token.
- No state mutation on reject regression test.
- Audit status row update + DECISIONS/TRACEABILITY + per-issue testplan.

Acceptance criteria:
- Relevant CI lanes green (qshield-ci + public-ci).
- Existing lanes remain green; no regressions.

Evidence:
- PR link + merge SHA recorded here on completion.

- PR verification bundle + post-merge verification anchors.

### NA-0049 — Audit closure: close Issue #28 (Redundant safe unwraps in refimpl)












Status: DONE
Completed: 2026-01-17 — PR #67 (merge 6bfdf7f)
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO
Docs-only allowed? NO

Objective:

- Remove remaining “safe unwrap” and panic-prone patterns in refimpl paths flagged by audit Issue #28, replacing them with deterministic reject behavior.

Deliverables:

- Replace flagged unwraps with deterministic reject/errors (no panics).
- Add regression tests proving deterministic reject and “no state mutation on reject” where state is involved.
- Update audit status row for Issue #28.
- Update DECISIONS + TRACEABILITY.
- Add a per-issue testplan under tests/.

Acceptance criteria:

- Relevant CI lanes green; no regressions.
- Tests assert deterministic reject + no mutation on reject for the touched paths.

Evidence:

- PR verification bundle + post-merge verification anchors.

### NA-0050 — Dumb Relay/Server (transport-only; no protocol changes)












Status: DONE
Completed: 2026-01-18 — PR #74 (merge b1b3e91)
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO
Docs-only allowed? NO

Objective:
- Provide a minimal, transport-only relay/server to move Suite-2/QSP messages without altering protocol semantics.

Deliverables:
- Basic relay API/CLI/service with minimal persistence.
- Deterministic error responses for invalid inputs (no panics).
- Documentation for local run + demo usage.
- Test plan documenting relay boundaries and invariants.

Acceptance criteria:
- Relay works end-to-end with existing protocol flows (no protocol-core changes).
- CI remains green; no regressions.

Invariants:
- No protocol or wire format changes.
- Transport-only; relay must not interpret or alter cryptographic content.
- Fail-closed on invalid inputs; deterministic error surface.

Evidence:
- PR verification bundle + relay test plan + CI links.

### NA-0051 — Linux TUI Reference Demo Client (uses existing protocol)












Status: DONE
Completed: 2026-01-19 — PR #80 (merge 03bf51e)
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO
Docs-only allowed? NO

Objective:
- Build a Linux TUI demo client that exercises existing protocol flows via the dumb relay without altering protocol behavior.

Deliverables:
- Minimal TUI interface (connect, establish, send/receive, status).
- Demonstration script and test plan.

Acceptance criteria:
- End-to-end demo succeeds with existing protocol semantics.
- CI remains green; no protocol-core changes introduced.

Invariants:
- UI must not drive protocol-core changes.
- Deterministic errors surfaced to users; fail-closed remains mandatory.
- No secret logging.

Evidence:
- PR verification bundle + demo test plan + CI links.


### NA-0052 — Relay interop over relay_http must pass (harness transport semantics)












Status: DONE
Completed: 2026-01-18 — PR #78 (merge 65fde9e)
Wire/behavior change allowed? NO (protocol wire); YES (harness transport behavior only)
Crypto/state-machine change allowed? NO
Docs-only allowed? NO

Objective:

- Fix relay_http harness transport semantics so Phase 4B interop passes over the HTTP relay (AWS qsl-server), proving encrypted protocol bytes can traverse remote transport unchanged.

Deliverables:

- relay_http adapter uses deterministic per-side channels aligned with harness send/receive flow (push+pull on the same side channel).
- Harness regression tests for adapter channel/direction mapping.
- Evidence capture instructions added to demo test plan.
- Local proof: interop passes 4/4 with LOCAL transport.
- Remote proof: interop passes 4/4 with QSL_TRANSPORT=relay_http against AWS relay.

Acceptance criteria:

- All CI lanes green; no regressions.
- interop over relay_http passes 4/4 with logged evidence under _forensics/ (not committed).
- NA-0052 marked DONE with completion line referencing implementation PR merge SHA.



### NA-0053 — Public metadata demo: size-padding buckets (client-layer; no protocol changes)












Status: DONE
Completed: 2026-01-19 — PR #86 (merge f10c61c)
Wire/behavior change allowed? NO (protocol unchanged; demo/client-only)
Crypto/state-machine change allowed? NO (protocol core unchanged)
Docs-only allowed? NO (client implementation + docs)

Objective:

- Provide a public, hands-on demo that (a) uses the existing QSL protocol for content encryption and (b) reduces message-size metadata
  leakage via app-layer padding buckets inside the encrypted payload.

Deliverables:

- qsl-tui implements deterministic size-padding buckets in relay demo mode (inside ciphertext).
- Headless mode prints overhead metrics (plaintext_len, padded_len, bucket).
- Demo test plan updated with “metadata reality + mitigations” and golden commands.
- Governance evidence (DECISIONS + TRACEABILITY).

Acceptance criteria:

- CI green; no regressions.
- Headless local + relay demo passes and prints overhead metrics.

Evidence:

- PR verification bundle + logs in _forensics/.

### NA-0054 — Metadata visibility demo (qsl-tui; client-only)












Status: DONE
Completed: 2026-01-20 — PR #TBD (merge TBD)
Wire/behavior change allowed? NO (protocol unchanged; demo/client-only)
Crypto/state-machine change allowed? NO (protocol core unchanged)
Docs-only allowed? NO (client implementation + docs)

Objective:

- Provide a public demo that explicitly reports plaintext_len vs ciphertext_len, padding bucket, and privacy mode.
- Make metadata tradeoffs clear in output and docs.

Deliverables:

- qsl-tui emits QSL_TUI_META + QSL_TUI_META_NOTE lines in headless mode.
- CLI adds --privacy-mode {basic,padded}.
- Demo test plan updated with commands + expected markers.
- Governance evidence (DECISIONS + TRACEABILITY).

### NA-0055 — Public demo pack for NA-0054 (metadata visibility)












Status: DONE
Wire/behavior change allowed? NO (docs/script only)
Crypto/state-machine change allowed? NO
Docs-only allowed? YES

Objective:
- Provide a public-facing demo doc + script for the metadata visibility demo.

Deliverables:
- docs/demo/DEMO-PUBLIC-001_Metadata_Visibility.md
- scripts/demo/demo_public_metadata_visibility.sh

Completed: 2026-01-20 — PR TBD

### NA-0056 — Public demo/client v1 (two-party + proxy + measurable metadata)












Status: DONE
Completed: 2026-01-21 — PR TBD (merge TBD)
Wire/behavior change allowed? NO (protocol unchanged; demo/client only)
Crypto/state-machine change allowed? NO (protocol core unchanged)
Docs-only allowed? NO (client + scripts + docs + governance)

Objective:

- Ship a public-facing two-party demo client with explicit sender/receiver roles.
- Provide optional proxy/Tor-friendly relay HTTP support.
- Emit stable, machine-readable metadata lines that expose size/timing tradeoffs clearly.

Deliverables:

- qsl-tui supports --role sender|receiver and emits QSL_TUI_META lines with role/mode/proxy/privacy/lengths/bucket.
- Two-party local and relay scripts under scripts/demo/.
- Demo test plan updated with two-party + proxy/Tor commands.
- Governance wiring (DECISIONS + TRACEABILITY).

Acceptance criteria:

- CI green (required checks pass).
- Local two-party demo runs (receiver then sender) in local and relay modes.
- Proxy/Tor mode documented and runnable.

---

### NA-0057 — Governance: Public Demo Runbook (authoritative demo/client execution discipline)












Status: DONE
Wire/behavior change allowed? NO (protocol unchanged; governance/docs only)
Crypto/state-machine change allowed? NO
Docs-only allowed? YES

Objective:
- Create a single authoritative runbook so public demo/client work proceeds deterministically each session (scope discipline, slow-machine constraints, bounded CI waits, and claims discipline), including:
  - privacy envelopes (tick schedule + size buckets + bundle packing)
  - uniform rejects + no-mutation-on-reject testing expectations
  - receipts/ACK camouflage as a roadmap item (no over-claim)
  - logging/metrics privacy budget discipline

Deliverables:
- docs/dev/DOC-DEV-004_Public_Demo_Runbook_v0.1.0_DRAFT.md
- docs/DOCS_MAP.md updated
- Minimal pointer in CHAT_STARTER.md (single bullet)
- Governance evidence: DECISIONS.md + TRACEABILITY.md
- Test plan stub: docs/archive/testplans/NA-0057_public_demo_runbook_testplan.md

Acceptance criteria:
- goal-lint green (PR body includes Goals line).
- Required CI checks attach and pass.
- Exactly one READY item exists in NEXT_ACTIONS.md (this NA-0057).

Evidence:
- PR #94 merged (https://github.com/QuantumShieldLabs/qsl-protocol/pull/94) merge=7d34360eee1e8216f3dac5a9e2aac8eab7e60018 date=2026-01-23
- DECISIONS entry (D-0007)
- TRACEABILITY entry for NA-0057
- docs/archive/testplans/NA-0057_public_demo_runbook_testplan.md


---

### NA-0058 — QSC client Phase 1: shell-first CLI + scriptable subcommands (secure-by-default)












Status: DONE
Wire/behavior change allowed? NO (protocol unchanged; client/demo layer only)
Crypto/state-machine change allowed? NO (client orchestration + storage semantics only)
Docs-only allowed? NO

Objective:
- Implement the QSC client (“qsc”) as a public-ready, shell-first CLI that is scriptable and emits stable machine-readable markers,
  while enforcing fail-closed semantics and minimizing metadata exposure.



Implementation notes (QSC):
- Spec: docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md
- Repo layout: qsl/qsl-client
- Include: Policy profiles (baseline vs strict; strict default).
- Include: qsc doctor --check-only safe diagnostics + deterministic markers.
- Include: Threat/metadata disclosure checklist for demos.
- Correctness edges: Send-state commit semantics (durably queued), recv routing bounds/oracle controls, deterministic pty + marker tests.
Authoritative design input:
- docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md

Repo layout recommendation:
- Create a separate client build directory for the new client workspace/crate(s):
  - qsl/qsl-client

Implementation-dependent correctness edges (must be explicit and tested):
- Send-state commit semantics: recommended “durably queued” outbox acceptance for .
- Recv routing: deterministic if safe hints exist; otherwise bounded try-decrypt with oracle controls.
- Secure storage: keychain-first + deterministic non-interactive fallback for CI.
- Deterministic interactive tests: pseudo-tty harness + stable marker assertions.

Required additions for public-ready defensibility (include in Phase 1):
- Policy profiles table (baseline vs strict; strict default).
-  safe diagnostics + deterministic markers.
- Threat/metadata disclosure checklist for demos (no over-claiming).

Deliverables:
-  command surface per spec (shell-first + scriptable subcommands) with stable marker output contract.
- Durable outbox semantics (or equivalent) eliminating ratchet ambiguity on transport failure.
- Recv routing policy implemented with bounded behavior and uniform rejects.
- Secure at-rest store: encrypted-by-default + atomic writes + safe permissions.
- Tests proving:
  - no-mutation-on-reject at probeable boundaries
  - no state advance on failure for send semantics
  - deterministic markers (including shell mode under pty harness)
- Demo scripts updated to use  where appropriate, without over-claiming metadata elimination.

Acceptance criteria:
- Exactly one READY item exists in NEXT_ACTIONS.md (this NA-0058).
- Existing CI lanes remain green; no regressions.
- New tests for the invariants above exist and pass.

Evidence:
- TBD (PR links + test plan + marker schema references)
- Evidence (merge): PR #95, PR #96, PR #97; main merge SHA 93d11f318e067e55e09fc02c2c725f55e6412dd2; verified 2026-01-24.


### NA-0059 — QSC client: Step 3 (command-surface expansion + security checklist alignment)












Status: DONE
Wire/behavior change allowed? NO (protocol wire unchanged; client-only)
Crypto/state-machine change allowed? NO (no new primitives; use existing engine interfaces only)
Docs-only allowed? NO (client implementation + tests + governance as required)

Objective:

- Expand QSC toward the design spec (shell-first + scriptable subcommands) while enforcing client security invariants
  (fail-closed, deterministic errors, no-mutation-on-reject) and keeping public claims honest.

Deliverables (minimum):

- Implement deterministic marker emission `QSC_MARK/1` for key events (ok/error) with a stable schema and **no secrets**.
- Add terminal output sanitization for untrusted inbound text (strip/escape control + ANSI sequences).
- Add bounded resource limits and timeouts for any I/O loop surfaced by CLI (no infinite waits).
- Add regression tests proving invariants at the client boundary:
    - deterministic error codes
    - no-mutation-on-reject for a probed boundary (tamper/replay/reorder harness placeholder acceptable)
    - sanitization prevents ANSI/control injection
- Update governance evidence as required by goal-lint for any core-path changes.

Acceptance criteria:

- Package-scoped `cargo fmt -p qsc`, `cargo test -p qsc --locked`, and `cargo build -p qsc --release --locked` pass locally (isolated env).
- CI green; no regressions.
- Exactly one READY item remains in NEXT_ACTIONS.
- Evidence (merge): PR TBD; verified 2026-01-24.

### NA-0060 — QSC store hardening: umask/permissions + atomic writes + locking + deterministic errors (client-only)












Status: DONE
Wire/behavior change allowed? YES (client local storage + CLI behavior only; no protocol wire changes)
Crypto/state-machine change allowed? NO (storage boundary only)
Docs-only allowed? NO

Objective:

- Make QSC’s on-disk store fail-closed and resilient against common local attacks (symlink/path tricks, unsafe perms, partial writes),
  while keeping behavior deterministic and testable for public demos.

Authoritative design basis:

- QSC Design Spec: client invariants for fail-closed, no-mutation-on-reject, deterministic errors, atomic writes, locking, store perms.
  (docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md)
- Client Security Checklist MUSTs: B1–B4 (perms/path/atomic/locking), C2+C5 (no-mutation + deterministic errors), E2 (timeouts),
  K1 (tests proving invariants).

What we are protecting:

- Local confidentiality/integrity of client state and keys (even before full vault encryption is expanded).
- Deterministic behavior at probed boundaries (attackers must not be able to induce partial writes, state corruption, or silent fallback).

Invariant (must never happen):

- No store mutation when a safety check fails (symlink traversal, unsafe parent perms, unsafe file perms, lock failure).
- No partial writes (config/state files must not be left truncated or half-written on crash/interrupt).

System must do instead:

- Deterministically reject with stable error code markers and perform zero mutation.
- Perform atomic write protocol (temp → fsync → rename → fsync dir) under an exclusive lock.

Deliverables:

- Runtime hardening:
  - Set umask 077 at startup (Unix) and enforce store dirs 0700 / files 0600.
  - Expand store safety checks:
      - reject symlink traversal for root and subpaths (no-follow patterns)
      - reject unsafe ownership or group/world-writable parents (policy-defined)
      - deterministic error codes for each class (no oracle strings)
- Atomic write helper used everywhere QSC mutates persisted state:
  - write temp in same dir → fsync temp → atomic rename → fsync directory
- Locking:
  - exclusive lock for mutations; shared/read lock for read-only operations (minimal dependency footprint).
- Tests (CI-enforced):
  - no-mutation-on-reject for at least two storage-probed boundaries (symlink + unsafe perms + lock failure path)
  - atomic write behavior (unit-level): temp cleanup + final file present; no truncated results
  - permission enforcement (dir/file modes) on supported platforms
  - marker determinism: stable QSC_MARK/1 error codes for each reject

Acceptance criteria:

- Local package-scoped verification: fmt/test/build for qsc passes with --locked in isolated cargo env.
- CI green; goal-lint satisfied (DECISIONS + TRACEABILITY updated in the same PR).
- NEXT_ACTIONS single-READY invariant preserved.

Evidence:

- Evidence: PR #102 merged at 2026-01-24T22:58:23Z; merge SHA b32f0d8d7c46c7d53b9ba97a9697563783b2e715; https://github.com/QuantumShieldLabs/qsl-protocol/pull/102
- PR link with:
  - scope guard (name-only diff)
  - tests proving no-mutation-on-reject and atomic write invariants
  - CI rollup green

Notes / roadmap alignment:

- Encryption-at-rest vault expansion remains mandatory, but is intentionally split into the next NA to keep this step reviewable and fail-closed.

### NA-0061 — QSC vault encrypted-at-rest default + keychain/passphrase fallback (Argon2id)












- Roadmap note: vault keyslot model MUST remain extensible for a future YubiKey hardware-backed slot (plumbing now; enforce later). See docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md (YubiKey roadmap section).

Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Encrypted-at-rest is default (no silent plaintext mode).
- Keychain preferred when available; deterministic passphrase fallback.
- Noninteractive mode never prompts; fails closed with stable marker.

Deliverables:

- Add vault module skeleton + encryption envelope for stored secrets.
- Define key derivation policy (Argon2id params) and keychain integration points.
- Tests: vault required by default; noninteractive deterministic fail; redaction guarantees.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #107 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/107) merged (merge SHA 4e0cc3af7b49224c1b3ac72224d4375219e56088).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0062 — QSC protocol boundary reject invariants (strict parse, pinned identity, blocked-no-network, replay/duplicate)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Rejects MUST NOT mutate state (receive/send boundaries).
- Pinned identity mismatch is hard fail (no silent rollover).
- Blocked contact send produces zero network traffic.
- Duplicate/replay receive returns deterministic marker and no mutation.

Deliverables:

- Introduce strict parsing limits and stable reject classes.
- Add transport mock to prove blocked=no network.
- Tests: recv_reject_does_not_advance_state; pinned_mismatch_no_mutation; blocked_send_no_network; duplicate_no_mutation.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #110 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/110) merged (merge SHA aded11b95b81fcbcc89139960a949845ad6f8c78).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0063 — QSC resource limits + bounded retries/timeouts (no infinite loops, no unbounded queues)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- No unbounded growth of queues/history/logs.
- All retries/timeouts bounded and deterministic.

Deliverables:

- Define max sizes for queues and histories; deterministic overflow reject.
- Bound connect/send/recv retry loops and jitter (bounded).
- Tests: queue_limit_enforced; retry_bound_enforced; timeout_marker_stable.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #112 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/112) merged (merge SHA 85508a2bd9f8c0567ae9856db775a838a6a1f593).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0064 — QSC diagnostics/doctor + markers schema + logging privacy budget (no secrets)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Diagnostics output never includes secrets; deterministic marker schema.
- Logging disabled by default; redaction enforced where enabled.

Deliverables:

- Define marker schema v1 and JSONL option; keep default minimal.
- Implement doctor report redacted export (check-only safe).
- Tests: diagnostics_no_secrets; markers_schema_stable; logs_off_by_default.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #114 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/114) merged (merge SHA 3cc55d3d1647b62a3aa195373519f87f66972648).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0065 — QSC output minimization posture for demos (redaction by default + claim discipline hooks)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Default output avoids endpoints/timestamps/high-cardinality identifiers.
- Sensitive output only behind explicit flag (ideally non-public build).

Deliverables:

- Introduce output policy layer: default redacted, explicit reveal.
- Add tests: default_output_no_endpoint_or_time; redact_is_enforced.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #116 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/116) merged (merge SHA 71ef24c6b92bb600c0e12eb900bedeeec573f4b6).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0066 — QSC privacy envelopes: tick schedule + size buckets + bundle packing












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Traffic shaping must be deterministic and bounded (no infinite delays).
- Padding/bucketing must be measurable and testable; no overclaims.

Deliverables:

- Define envelope contract; implement tick scheduler and bundle packing.
- Tests: tick_schedule_stable_and_bounded; bucket_sizes_match_spec; bundle_packing_rules.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #118 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/118) merged (merge SHA 6a8fcd9268dceb6b9bf9abd2f64c9e988521d6fb).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0067 — QSC receipt/ACK camouflage (avoid ACK distinguishability)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- ACK/receipt emissions must not form a distinct observable class (size/timing class bounded).

Deliverables:

- Define ACK camouflage rule and integrate with envelope policy.
- Tests: ack_size_class_matches_small_msg; ack_behavior_deterministic.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #121 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/121) merged (merge SHA aceedd34da242722f8f57844f0e3394de33b4732).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0068 — QSC supply-chain + release authenticity controls (locked deps, advisories, signed artifacts)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Dependency drift prevented by policy; releases verifiable.

Deliverables:

- Add dependency policy lane (e.g., advisory checks) when feasible.
- Document signed release verification steps; add CI check for signatures when release workflow exists.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #123 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/123) merged (merge SHA 2d21a961686060337ee78b5c4beb88c8ef7db74c).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0069 — QSC secret hygiene in memory (zeroize + crash surface minimization)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- Secret buffers not retained longer than necessary; reduce accidental leakage in crashes.

Deliverables:

- Adopt zeroize patterns on sensitive types; avoid printing secrets in panics.
- Tests: smoke tests + code review gates; ensure no secrets in stderr for known flows.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #126 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/126) merged (merge SHA 8f118163bf05b5f45944c03c91585791433ce76d).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0070 — QSC send commit semantics (outbox/prepare→send→commit to preserve no-mutation-on-failure)












Status: DONE
Wire/behavior change allowed? YES (client-only; no protocol wire changes without explicit queue approval)
Crypto/state-machine change allowed? POSSIBLY (client-local only; protocol changes require separate NA and explicit approval)
Docs-only allowed? NO

Objective:

- Harden the QSC client posture by enforcing fail-closed, deterministic behavior and CI-proven invariants.

Security invariants (must never happen):

- If transport fails, state MUST NOT advance unless explicitly committed.

Deliverables:

- Introduce durable outbox or staged commit semantics.
- Tests: send_failure_no_commit; outbox_commit_advances_once.

Acceptance criteria:

- CI lanes green (public-ci + qshield-ci) for the PR(s) that implement this NA.
- Deterministic rejects with stable marker/error codes for all reject paths introduced.
- Regression tests prove “no mutation on reject” for all state/storage boundaries touched.

Evidence:
- Evidence: PR #128 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/128) merged (merge SHA d0f3801d3d020ec2b65c73dabf95283202b1a327).
- PR link(s) in TRACEABILITY.
- Tests asserting invariants are present and green.


### NA-0071 — QSP v4.3 header key derivation correctness (KMAC-based; no placeholders)








Status: DONE
Wire/behavior change allowed? YES (protocol-core refimpl; no new wire formats)  
Crypto/state-machine change allowed? YES  
Docs-only allowed? NO

Objective:
- Scope: protocol-core (refimpl QSP v4.3 lane).
- Protect header confidentiality/integrity and correct domain separation.
- Ensure header keys are derived from RK using KMAC-based KDF (no placeholders).

Security invariants (must never happen):
- Header keys are derived using placeholders/static labels in protocol lanes.
- Rejected inputs mutate session state.

Deliverables:
- Refimpl change to enforce KMAC-based header key derivation from RK in QSP v4.3.
- Regression tests proving key dependence on RK and wrong-RK failure paths.
- CI gate evidence recorded in implementation PR(s).

Acceptance criteria:
- Tests prove header keys depend on RK; wrong RK fails.
- Negative test ensures wrong RK fails with no state mutation.
- clippy -D warnings clean for unused params in implementation PR.

Evidence:
- Evidence: PR #131 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/131) merged (merge SHA 86cae35b7864b661b09a699d294224e07a06c855).
- PR link(s) in TRACEABILITY.
- docs/archive/testplans/NA-0071_qsp_header_key_derivation_testplan.md (planned).

---

### NA-0072 — Public repo housekeeping: remove deprecated/duplicate artifacts; align doc pointers; single source of truth








Status: DONE
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES

Objective:
- Keep the public repo tidy and non-duplicative without changing protocol behavior.

Deliverables:
- Identify deprecated/duplicate artifacts and remove or deprecate them.
- Align doc pointers to committed inputs; avoid drift.
- Ensure only one authoritative source for each public-facing artifact.

Acceptance criteria:
- No protocol or behavior changes.
- goal-lint remains green for doc-only updates.

Evidence:
- Evidence: PR #135 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/135) merged (merge SHA 931cd7e9ba3e780cdc5d4ce49a4a1e8075e810e2).
- PR link(s) in TRACEABILITY.

### NA-0073 — Repo housekeeping follow-on: README alignment + harness dedupe


Status: DONE
Wire/behavior change allowed? NO  
Crypto/state-machine change allowed? NO  
Docs-only allowed? YES

Scope:
- docs/structure only: README.md + harness directories + reference integrity.

Objective:
- Remove legacy harness root and converge on one authoritative harness directory; update README references.

Invariants:
- One harness root only.
- No broken references (rg proof).
- CI required contexts remain green.

Deliverables:
- Delete or relocate legacy harness dir (likely test-harness/4b/**) or consolidate into tests/harness/4b/**.
- README.md updated to reference the authoritative path.
- Plan executed with evidence and TRACEABILITY updated.

Acceptance criteria:
- rg finds no references to removed harness path.
- Required CI contexts pass.
- TRACEABILITY updated with PR links and plan evidence.

Evidence:
- Evidence: PR #138 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/138) merged (merge SHA d81806bcb6b540cb070ee56768a756aa5b99fae0).
- PR link(s) in TRACEABILITY.


### NA-0074 — qsc Security Lens MVP (CLI + TUI) with invariant-driven observability


Status: DONE
Wire/behavior change allowed? NO
Crypto/state-machine change allowed? NO
Docs-only allowed? NO

Scope:
- qsl/qsl-client/qsc/** only (implementation later), plus docs/tests planning now.

What is being protected:
- vault secrets at rest
- session state integrity (no mutation on failure)
- metadata minimization envelope integrity
- deterministic observable outputs (markers/events)

Invariants:
1) No hidden state transitions: all state changes emit deterministic markers/events.
2) No mutation on reject/failure (persistent state) — proven by regression tests.
3) No secrets in UI/markers/logs; redaction guaranteed.
4) Fail-closed filesystem safety (unsafe parents/symlinks/perms refuse).
5) TUI must be a “lens”: no silent retries, no background recovery, no implicit sends.

Deliverables (MVP):
- CLI: existing command surface remains stable; add “observe” surfaces if needed.
- TUI: read-mostly interactive lens with:
  - contacts list
  - per-peer session panel
  - message timeline
  - status pane showing: fingerprint, epoch/ratchet counters, envelope bucket/tick, ack camouflage, send lifecycle (prepare→send→commit)
  - command bar with explicit /commands (no implicit actions)
- Charter doc: docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md
- Test plan stub: docs/archive/testplans/NA-0074_qsc_security_lens_mvp_plan.md

Acceptance criteria:
- New tests added that prove the invariants:
  1) emits marker on prepare/send/commit boundaries
  2) no mutation on reject/failure for send/receive
  3) redaction holds: no secrets in outputs
  4) fails on unsafe config parent/symlink/perms
  5) TUI “no implicit send” enforced (explicit command required)
  6) deterministic marker ordering across runs
- cargo test -p qsc --locked is green
- CI required contexts remain green; no regressions
- Charter is referenced from TRACEABILITY and enforced by tests

#### Appendix — QSC Client Suggestions Coverage (source: client_suggestions.txt)

This appendix maps additional client security suggestions into the recorded BACKLOG NAs (no READY changes).

- SUG-001 → NA-0061: QSC / QSL Client Security Requirements Checklist (Director-Ready)
- SUG-002 → NA-0068: Version: 0.1 | Date: 2026-01-22 | Scope: Client app (CLI/TUI), storage, transport boundary, and release posture.
- SUG-003 → NA-0061: GOAL
- SUG-004 → NA-0065: - Provide a demo/public client that is secure-by-default, fail-closed, and testable.
- SUG-005 → NA-0062: - No protocol wire changes implied by this checklist.
- SUG-006 → NA-0061: THREAT MODEL (MINIMUM)
- SUG-007 → NA-0064: - Local attacker: disk theft, file scraping, symlink/path trickery, log leakage.
- SUG-008 → NA-0062: - Network attacker / malicious relay: tamper, replay, reorder, DoS, metadata observation.
- SUG-009 → NA-0062: - Malicious peer: crafted payloads, parser abuse, terminal escape injection.
- SUG-010 → NA-0068: - Supply-chain: compromised dependencies, unsigned releases.
- SUG-011 → NA-0061: MUST (NON-NEGOTIABLE)
- SUG-012 → NA-0061: A) KEY MANAGEMENT + VAULT
- SUG-013 → NA-0061: A1. Encrypted-at-rest is the default; no silent plaintext mode.
- SUG-014 → NA-0061: A2. Master key:
- SUG-015 → NA-0061: - Preferred: OS keychain/credential store.
- SUG-016 → NA-0061: - Fallback: passphrase with Argon2id (salt + params stored).
- SUG-017 → NA-0061: A3. Secrets never printed:
- SUG-018 → NA-0061: - No private keys, no vault content, no session keys, no raw decrypted state in stdout/stderr.
- SUG-019 → NA-0061: A4. Vault-locking:
- SUG-020 → NA-0061: - Non-interactive mode must fail deterministically if locked (no surprise prompts unless explicitly enabled).
- SUG-021 → NA-0061: B) STORAGE + FILESYSTEM HARDENING
- SUG-022 → NA-0061: B1. Permissions enforced:
- SUG-023 → NA-0061: - dirs 0700, files 0600; process umask 077 at runtime.
- SUG-024 → NA-0061: B2. Safe path handling:
- SUG-025 → NA-0062: - Reject symlink traversal in store path and all subpaths (no-follow opens).
- SUG-026 → NA-0062: - Reject unsafe ownership or group/world-writable store parents (policy-defined).
- SUG-027 → NA-0061: B3. Atomic writes everywhere:
- SUG-028 → NA-0061: - write temp → fsync temp → atomic rename → fsync directory.
- SUG-029 → NA-0061: B4. Locking:
- SUG-030 → NA-0061: - Exclusive locks for any mutation; shared/read locks for read-only operations.
- SUG-031 → NA-0061: B5. AEAD integrity checks fail closed:
- SUG-032 → NA-0061: - Corruption/tag failure is a hard error; no best-effort parsing.
- SUG-033 → NA-0062: C) PROTOCOL/ENGINE BOUNDARY INVARIANTS
- SUG-034 → NA-0061: C1. Fail-closed parsing:
- SUG-035 → NA-0062: - Strict frame parsing with length limits; reject invalid encodings deterministically.
- SUG-036 → NA-0062: C2. No-mutation-on-reject:
- SUG-037 → NA-0062: - Any rejected inbound frame must not advance ratchets/counters/epochs in persisted state.
- SUG-038 → NA-0061: C3. Verified contact pinning:
- SUG-039 → NA-0062: - Peer identity mismatch is a hard fail (PEER_IDENTITY_MISMATCH) with no mutation.
- SUG-040 → NA-0061: - No silent key rollover.
- SUG-041 → NA-0062: C4. Blocked contact enforcement:
- SUG-042 → NA-0066: - open/send must refuse with NO NETWORK TRAFFIC.
- SUG-043 → NA-0062: - recv must drop/reject with no mutation and no storage of plaintext.
- SUG-044 → NA-0061: C5. Deterministic error classes:
- SUG-045 → NA-0061: - Small stable set of error codes; avoid detailed oracle strings.
- SUG-046 → NA-0065: D) TERMINAL/TUI OUTPUT SAFETY
- SUG-047 → NA-0061: D1. Sanitize all untrusted text before display:
- SUG-048 → NA-0061: - Strip/escape control chars and ANSI escape sequences.
- SUG-049 → NA-0061: - Collapse newlines; cap preview lengths.
- SUG-050 → NA-0061: D2. Prompt safety:
- SUG-051 → NA-0061: - No UI patterns that allow inbound text to mimic prompts/commands.
- SUG-052 → NA-0061: D3. TUI mode:
- SUG-053 → NA-0064: - No stdout logging that corrupts rendering (log to file or internal panel).
- SUG-054 → NA-0061: E) RESOURCE BOUNDS + DOS
- SUG-055 → NA-0061: E1. Strict maximums:
- SUG-056 → NA-0063: - Max frame size, max message size, max queued items, bounded history loads.
- SUG-057 → NA-0063: E2. Timeouts:
- SUG-058 → NA-0063: - Bounded connect/handshake/recv/send timeouts; no infinite loops.
- SUG-059 → NA-0068: F) RELEASE / SUPPLY CHAIN BASELINE
- SUG-060 → NA-0062: F1. Lockfile pinned deps; minimal dependency footprint.
- SUG-061 → NA-0068: F2. Signed releases (or signed tags) with verification instructions.
- SUG-062 → NA-0061: F3. Build artifacts do not embed secrets; deterministic version stamping.
- SUG-063 → NA-0061: SHOULD (HIGHLY RECOMMENDED)
- SUG-064 → NA-0069: G) MEMORY HYGIENE
- SUG-065 → NA-0061: G1. Zeroize sensitive buffers where feasible (vault plaintext, session keys).
- SUG-066 → NA-0061: G2. Avoid long-lived decrypted copies; decrypt-use-wipe.
- SUG-067 → NA-0061: H) TRANSPORT HARDENING
- SUG-068 → NA-0061: H1. TLS for relay connections (protect tokens/credentials and integrity of transport channel).
- SUG-069 → NA-0061: H2. Proxy support is connectivity-only; explicitly avoid anonymity claims.
- SUG-070 → NA-0063: H3. Backoff + jitter (bounded) for reconnect loops; rate-limit repeated failures.
- SUG-071 → NA-0064: I) DIAGNOSTICS WITHOUT LEAKAGE
- SUG-072 → NA-0064: I1. `qsc doctor` check-only command:
- SUG-073 → NA-0061: - permissions/ownership/symlinks checks
- SUG-074 → NA-0061: - vault availability checks
- SUG-075 → NA-0061: - store integrity checks (header/minimal metadata only; no plaintext exposure)
- SUG-076 → NA-0064: I2. Markers (JSONL) optional:
- SUG-077 → NA-0061: - stable schema; never includes secrets; default off.
- SUG-078 → NA-0061: J) PRIVACY POSTURE (HONEST + TESTABLE)
- SUG-079 → NA-0065: J1. Default outputs avoid metadata:
- SUG-080 → NA-0065: - endpoints hidden by default
- SUG-081 → NA-0065: - timestamps hidden by default
- SUG-082 → NA-0061: J2. Polling interval option (`recv --poll-ms`) explicitly documented as timing-variance reduction only.
- SUG-083 → NA-0066: J3. Size bucketing/padding (if implemented) explicit and auditable; no overclaims.
- SUG-084 → NA-0061: K) TESTS THAT PROVE INVARIANTS
- SUG-085 → NA-0061: K1. Regression tests for:
- SUG-086 → NA-0062: - no-mutation-on-reject (recv tamper/replay/reorder vectors)
- SUG-087 → NA-0062: - pinned identity mismatch hard-fails with no mutation
- SUG-088 → NA-0062: - blocked contact produces no network traffic (mocked transport assertion)
- SUG-089 → NA-0061: - atomic write robustness (power-loss simulation as feasible; at least unit tests)
- SUG-090 → NA-0061: - terminal escape sanitization
- SUG-091 → NA-0061: K2. Fuzz parsing and state-machine boundaries (recv pipeline).
- SUG-092 → NA-0061: NICE-TO-HAVE (PHASE 2+)
- SUG-093 → NA-0061: L) MULTI-PROFILE / MULTI-DEVICE HARDENING
- SUG-094 → NA-0061: L1. Multiple profiles with explicit selection; no accidental cross-profile mutation.
- SUG-095 → NA-0061: L2. Device enrollment UX (if supported) includes verification and attestation records.
- SUG-096 → NA-0068: M) ADVANCED EXPORT + AUDIT
- SUG-097 → NA-0065: M1. Canonical JSON export with strict redaction defaults.
- SUG-098 → NA-0064: M2. Optional transparency-friendly logs (without metadata overclaims).
- SUG-099 → NA-0061: DIRECTOR DECISIONS REQUIRED (SET EARLY)
- SUG-100 → NA-0061: D1. Default trust policy:
- SUG-101 → NA-0061: - baseline (allow unverified with warning) vs strict (refuse unverified unless allow flag).
- SUG-102 → NA-0070: D2. Unknown fingerprint sending:
- SUG-103 → NA-0065: - refuse by default vs allow only with explicit endpoint + explicit allow flag.
- SUG-104 → NA-0070: D3. Send commit semantics:
- SUG-105 → NA-0061: - state commit on durable queue/ack vs advance-on-encrypt; must be consistent and tested.
- SUG-106 → NA-0061: D4. Prompting policy:
- SUG-107 → NA-0061: - read-only commands never prompt by default (recommended for scripts).
- SUG-108 → NA-0061: DELIVERABLE EXPECTATION
- SUG-109 → NA-0061: - For each MUST item, implement:
- SUG-110 → NA-0064: (1) enforcement logic,
- SUG-111 → NA-0061: (2) deterministic error behavior,
- SUG-112 → NA-0061: (3) CI tests that prove the invariant,
- SUG-113 → NA-0061: (4) documentation of the user-visible posture (no overclaims).
Invariant: public repo must not contain duplicate/deprecated artifacts that mislead contributors; removals must be proven safe; CI lanes remain green; no orphaned references.

Deferred/Blocked:
- Deferred: harness dedupe (test-harness/4b) blocked by README.md reference; requires follow-on scope to update README.

Evidence:
- Evidence: PR #142 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/142) merged (merge SHA 8a4dbe891923f31ae6a83f8862488eaecd55ca17).

### NA-0075 — qsc Relay Demo Transport (realistic conditions, charter-enforced)
Status: DONE
Scope: qsl/qsl-client/qsc/** only (implementation later); docs/tests planning now.
What is being protected:
- send/commit semantics (no mutation on failure)
- metadata discipline (envelope bucketing/ticks; ack camouflage)
- charter guarantees (explicit-only behavior)
Invariants (non-negotiable):
1) Relay is a dumb pipe: qsc must remain secure even if relay is hostile/unreliable.
2) No implicit send: user must invoke explicit /send (TUI) or command (CLI); relay mode must not introduce implicit sends.
3) No automatic retries: failures are surfaced with deterministic markers; any retry requires explicit command.
4) No background recovery: no silent resync; explicit /recover or /resync only (if present), with markers.
5) No persistent mutation on transport failure: prepare→attempt→commit remains enforced.
6) Deterministic observability: all relay events and outcomes emit stable QSC_MARK lines.
Deliverables (MVP):
- qsc relay subcommands:
  - qsc relay serve (local relay for demos)
  - qsc relay send --to <peer> --file <msg> (CLI)
  - optional qsc tui --transport relay --relay-url ... (TUI hook, explicit)
- Hostile network knobs (configurable in relay):
  - fixed latency, jitter window, drop %, duplicate %, reorder window
  - all deterministic when seed is provided
- Tests:
  - deterministic drop/timeout test proves no mutation on failure
  - reorder test proves explicit handling (no implicit recovery)
  - duplicate delivery test proves idempotent reject (no mutation)
- Docs:
  - relay transport contract doc (DOC-QSC-002)
- CI:
  - cargo test -p qsc --locked and clippy -D warnings remain green
Acceptance criteria:
- Tests prove invariants 2–6 under at least two hostile conditions (drop + reorder)
- Markers show lifecycle: prepare/attempt/commit + relay events
- No new metadata leakage in markers (no secrets, no raw keys)
- No regressions in existing qsc tests

Evidence:
- PR #145 merged (merge SHA 7780d61d53d81dceced1c1aa9b7b09598d06e1d5).
- PR #146 merged (merge SHA 185aced78e62d65d3cbefdf30d60dc7162541714).


### NA-0076 — Workflow hardening: proactive improvements + deterministic tool/caching defaults
Status: DONE
Scope: governance + workflow policy (no code changes).
Invariants:
- Any suggested improvement must be either implemented within scope OR converted into a new NA (no drive-by).
- Codex must always report better-approach candidates when found (even if not executed).
- Deterministic builds: directives default to isolated CARGO_HOME and GH_CACHE_DIR when local caches are unsafe.
- Diagnostics-first when blocked (reaffirm existing rule).
Deliverables:
- Operating rules doc updated with a Proactive Improvement & Tooling Defaults section.
- NA-0076 plan stub defining per-PR-type checklists.
Acceptance criteria:
- New rules documented and referenced in TRACEABILITY.
- Goal-lint passes for the governance PR.

Evidence:
- Evidence: PR #148 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/148) merged (merge SHA 3c361ec1854e95c54861f5499d37328d4f2ea0ff).


### NA-0077 — Demo packaging: run-it-locally qsc + relay (shareable, deterministic)
Status: DONE
Scope:
- Packaging + docs + scripts for demo (implementation PR will likely touch docs/ + scripts/ + qsc CLI flags only if required).
- NO protocol changes; NO hidden behavior; charter enforced.
What is being protected:
- charter rules (explicit-only, no implicit retries/recovery)
- no-mutation-on-failure
- no secrets in logs/markers
- deterministic behavior with seed
Invariants:
1) Demo is reproducible: seeded hostile scenarios produce stable marker sequences.
2) One-command happy path: user can run a local demo in <= 5 minutes.
3) Demo must not require secrets or privileged ops; no sudo required.
4) Demo outputs are safe to share (redacted; no secret material).
Deliverables (packaging contract):
- Quickstart runbook doc (DOC-QSC-003) with copy/paste commands.
- Local demo topology: relay + two clients (alice/bob) with deterministic hostile knobs (drop+reorder) showcased.
- A demo script interface spec:
  - ./scripts/demo/qsc_demo_local.sh --seed <u64> --scenario <name>
  - scenarios: happy-path, drop, reorder, drop+reorder, seeded replay
  - outputs: marker log files + a short human summary
- CI evidence plan (smoke): ensure demo script at least prints help and can run a dry-run path without network.
Acceptance criteria:
- New demo runbook exists and is accurate.
- Demo script exists and can execute on Ubuntu without sudo (in follow-on implementation PR).
- Deterministic marker logs: same seed → identical output subset (defined in plan).
- CI lane added or extended to validate demo packaging doesn’t rot (implementation PR).

Evidence:
- Evidence: PR #151 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/151) merged (merge SHA 42224a2ba1c186f517749775277385df2e4270dd).

### NA-0078 — Demo packaging Phase 2: full local run (relay + two clients) + deterministic logs

Status: DONE

Scope:
- scripts/demo/** (implementation PR), docs/qsc/**, tests/** (plan)
- qsc behavior must remain charter-enforced (no implicit retry/recovery).
- No protocol-core changes.

What is being protected:
- determinism (seeded scenarios)
- no secrets in logs
- explicit-only behavior (charter)
- no mutation on failure (send lifecycle)

Invariants:
1) `qsc_demo_local.sh` full-run requires no sudo and runs on Ubuntu with only Rust + cargo installed.
2) Full-run produces shareable artifacts:
   - alice.markers, bob.markers, relay.log (markers-only)
   - summary.txt with scenario + seed + outcome
3) Deterministic subset: for same seed+scenario, the marker subset defined in plan is identical.
4) No implicit retries/recovery. Any retry is explicit and logged.
5) On drop/reorder scenarios, state never advances unless send succeeds (no-mutation proven by markers/tests).

Deliverables:
- Extend scripts/demo/qsc_demo_local.sh to support FULL RUN (not just --dry-run):
  - spawn relay (background) with knobs
  - run two clients with scripted sequences
  - capture logs under --out dir
  - teardown reliably
- Update docs (runbook addendum) with copy/paste examples and “what you should see.”
- Add CI lane or extend demo-packaging.yml to run a minimal full-run scenario in a constrained mode (or a “smoke full-run” that runs only on ubuntu-latest with short timeout).
- Update NA-0078 plan evidence.

Acceptance criteria:
- Full-run works locally: happy-path + drop-reorder.
- CI smoke proves at least one full-run completes within a bounded time (e.g., 60s) OR proves deterministic log generation in a simulated mode.
- Deterministic marker subset comparison implemented (in plan; script may implement compare).
- All existing CI contexts remain green.

Evidence:
- Evidence: PR #154 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/154) merged (merge SHA 5599ff096942782b65fe7c36bb9220ca929bb756).

### NA-0079 — qsc Security Lens: TUI + relay integration (live hostile events, charter-enforced)

Status: DONE
Scope:
- qsl/qsl-client/qsc/** (implementation later)
- docs/qsc/** + tests/** planning now
- No protocol-core changes

What is being protected:
- Charter invariants (explicit-only, no implicit retry/recovery)
- No mutation on failure (prepare→attempt→commit semantics remain)
- No secrets in UI/markers/logs
- Deterministic, safe-to-share observability

Invariants:
1) Transport selection is explicit (e.g., `qsc tui --transport relay --relay <url>`); no implicit network behavior.
2) No automatic retries or background recovery in TUI relay mode. Any retry requires an explicit command and emits markers.
3) Relay events (drop/dup/reorder/delay/deliver) are surfaced in the TUI as an “Events” pane AND emitted as deterministic QSC_MARK lines.
4) Failure never advances persistent state: send_commit remains skipped on failure; no mutation on reject (test-proven).
5) Determinism: given the same seed/scenario, the visible event stream (normalized markers) is stable across runs in headless mode.

Deliverables:
- TUI relay mode wiring (uses existing relay CLI/transport)
- TUI Events pane (last N events, filterable)
- Headless scripted TUI test harness covering:
  - drop+reorder scenario shows correct events
  - no implicit retries/recovery markers
  - no mutation on failure
- Docs spec + plan updates; TRACEABILITY evidence

Acceptance criteria:
- New tests prove invariants 1–5 under at least one hostile scenario (drop+reorder) with fixed seed.
- `cargo test -p qsc --locked` and `clippy -D warnings` remain green.
- CI contexts remain green; no regressions.

Evidence:
- Evidence: PR #157 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/157) merged (merge SHA 363194118e3ab96fa7533cb2bac492263572003f).

### NA-0080 — Remote relay testing lane (qsc) (real network conditions, charter-enforced)

Status: DONE

Scope:
- qsc remote testing only (implementation later):
  * new workflow_dispatch + nightly scheduled workflow (non-required contexts)
  * remote relay endpoint provided via GitHub secrets/vars
- No protocol-core changes
- No weakening of charter rules

What is being protected:
- explicit-only behavior (no implicit retries/recovery/sends)
- no mutation on failure (prepare→attempt→commit)
- safe-to-share outputs (no secrets in logs/markers)
- robustness under real network variance

Invariants:
1) Remote tests never become required PR checks (avoid flakiness blocking merges).
2) Remote relay endpoint is explicitly configured (RELAY_URL); no implicit network targets.
3) Logs are marker-only/redacted; no secrets emitted.
4) Failures do not mutate persistent state (no mutation on failure/reject).
5) Remote tests are reproducible in intent: same scenario inputs → same client-side normalized marker subset, even if timing differs.

Deliverables:
- DOC-QSC-006 Remote relay testing contract doc
- NA-0080 plan stub with scenario matrix + normalized marker subset definition
- Follow-on implementation PR(s) add:
  * runner command or demo-script mode targeting remote relay
  * nightly + manual workflow (non-required contexts)
  * artifact upload: markers + deterministic subset + summary

Acceptance:
- Remote lane runs successfully against a real relay (AWS) for:
  * happy-path
  * drop+reorder
- Artifacts uploaded: markers + normalized subset + summary
- No secrets in logs; no implicit retry/recovery markers
- Existing required CI contexts remain green and unchanged

Evidence:
- Impl PR #160 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/160) merged (merge SHA ca9f283d9385c0dff6ddf8b25366dd6bfb57e397).

### NA-0082 — qsc doctor clarity: checked_dir + writable semantics (test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Make `qsc doctor` output unambiguous and safe-to-share by:
  * emitting which directory was checked (checked_dir)
  * clarifying when writability is required vs advisory
  * keeping markers deterministic and secret-free

Invariants:
1) Doctor output/markers MUST include `checked_dir=<path>` (no secrets).
2) If `dir_writable=false`, doctor MUST explain whether writability is required:
   - `dir_writable_required=true|false` (or equivalent field)
3) Output must remain safe-to-share: no secrets/payloads.
4) Existing safety checks are not weakened; tests adapt instead.
5) Deterministic markers: stable field names and codes.

Deliverables:
- Add marker fields (checked_dir, dir_writable_required, reason if needed).
- Add/update tests to assert fields exist and are consistent.
- Update docs/plan evidence.

Acceptance:
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- New test proves doctor marker includes checked_dir and writable semantics.
- No secrets in doctor output (grep guard test).

Evidence:
- Evidence: PR #165 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/165) merged (merge SHA b851ffd68ca89f9abcb122171b155da80f4c07e6).

### NA-0083 — qsc XDG correctness for lock/store paths + unambiguous lock errors (test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Ensure qsc honors XDG config/state roots for lock/store files so harnesses can isolate state without HOME hacks.
- Split lock failure markers so they are unambiguous:
  * open/create failure (EACCES/EPERM/etc.)
  * lock contention (EWOULDBLOCK/EAGAIN)

Invariants:
1) If XDG_CONFIG_HOME is set, lock path MUST be under it (e.g., $XDG_CONFIG_HOME/qsc/.qsc.lock), not $HOME/.config.
2) Lock error markers MUST distinguish:
   - lock_open_failed (or equivalent) for open/create permission failures
   - lock_contended (or equivalent) for non-blocking flock contention
3) No weakening of safe-parent checks.
4) No secrets in markers/logs.
5) Tests prove behavior deterministically.

Deliverables:
- Refactor config_dir()/store root selection to honor XDG consistently.
- Update lock acquisition to map errno to distinct error codes.
- Tests:
  * XDG path respected
  * permission denial yields lock_open_failed
  * contention yields lock_contended
- Update plan evidence and TRACEABILITY.

Acceptance:
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- Tests for XDG and lock error mapping PASS
- No secrets in output

Evidence:
- Evidence: PR #168 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/168) merged (merge SHA 9bacfe0fe55c076e69cf931d00ac7a9d2bfa0109).

### NA-0084 — qsc send semantics: real sender with explicit transport (relay-backed; test-driven)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Make `qsc send` the primary “send” command with an explicit transport contract:
  * send requires explicit transport selection (no implicit network)
  * for relay transport, send delegates to existing relay send plumbing
  * send preserves prepare→attempt→commit semantics and uses outbox for durability
- Eliminate ambiguity between `qsc send` and `qsc relay send` by documenting and testing the contract.

Invariants:
1) `qsc send` MUST NOT send unless transport is explicitly specified (e.g., `--transport relay --relay <url>`), or a user explicitly set a default via config (if supported; otherwise forbid).
2) On transport failure, send MUST NOT commit/mutate send state (prepare→attempt→commit).
3) `outbox_exists` must be resolvable via `qsc send abort` (idempotent).
4) No secrets or payload contents in markers/logs.
5) Deterministic markers: lifecycle markers ordered and stable.

Deliverables:
- CLI contract updates (help text) clarifying send vs relay send.
- Implementation of `qsc send` transport flags and relay delegation.
- Tests:
  * send happy-path against local relay (serve + send)
  * send failure path against unreachable relay (no commit)
  * outbox_exists recovery via send abort
  * no-secrets grep guard
- Update plan evidence and TRACEABILITY.

Acceptance:
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- Tests prove send can succeed end-to-end with explicit relay, and failure does not commit.
- Documentation/help output no longer ambiguous.

Evidence:
- Evidence: PR #171 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/171) merged (merge SHA 6964408bf486af2bef1c5b45e7697fa59fa33589).

### NA-0085 — TUI help rendering: /help renders deterministic command list (test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Make `/help` in the TUI actually render a command list in the UI (not just emit a marker),
  and enforce this with a headless deterministic test.

Invariants:
1) `/help` must render a deterministic command list to a visible pane (Events or Help panel).
2) The command list must be derived from the same command registry as the parser (no drift).
3) Markers remain deterministic and safe-to-share (no secrets).
4) No marker text is written into the input line (future NA covers this; here we focus on help rendering).

Deliverables:
- Add a help render path (pane buffer or overlay) with deterministic content.
- Add headless test: QSC_TUI_HEADLESS=1 + QSC_TUI_SCRIPT="/help;/exit" must assert help list appears.
- Update NA-0085 plan evidence.

Acceptance:
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- New test proves help list is rendered (not just cmd marker).
- No secrets in output.

Evidence:
- PR #174 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/174) merged (merge SHA 85aff62321b8c818fbaa143d5a71f1bbdbf07e32).

### NA-0086 — TUI marker routing: no stdout markers in interactive TUI (headless unchanged)

Status: DONE

Evidence:
- PR #177 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/177) merged (merge SHA 7816293cbd238f8a782d2fa99244dd4cf9ba7522).


Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Prevent TUI rendering corruption by ensuring QSC_MARK lines are not written to terminal stdout/stderr during interactive TUI runs.
- Preserve headless mode behavior: markers remain emitted to stdout for deterministic tests and CI evidence.

Invariants:
1) Interactive TUI MUST NOT emit QSC_MARK lines to stdout/stderr (prevents framebuffer corruption).
2) Interactive TUI MUST route markers into the in-app Events pane buffer deterministically.
3) Headless mode MUST continue to emit QSC_MARK lines to stdout exactly as before (no regression).
4) No secrets in markers or help output.
5) Deterministic behavior with seed/scenario preserved.

Deliverables:
- Marker emission gate: stdout markers only when QSC_TUI_HEADLESS=1 (or equivalent).
- Interactive mode: markers appended to Events pane only.
- Tests:
  * headless run still shows QSC_MARK markers
  * interactive-run capture shows no QSC_MARK on stdout (or equivalent deterministic proof)
- Update plan evidence and traceability.

Acceptance:
- Manual: run TUI, type /help → layout remains stable; events pane shows help items; no marker text splats UI.
- Automated: cargo test -p qsc --locked PASS with new tests.
- clippy -D warnings PASS.


### NA-0087 — TUI /help full-screen mode (list + details; deterministic; test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- `/help` switches the TUI into a full-screen Help mode (no background context),
  with a scrollable command list and a details pane.
- Help content is deterministic and derived from the command registry/parser to prevent drift.

Invariants:
1) `/help` must switch to a full-screen Help mode (not events-pane text).
2) Help view must be deterministic: stable ordering and stable strings.
3) Help content must be derived from the command registry/parser (single source of truth).
4) Interactive mode must not print QSC_MARK to terminal (already enforced by NA-0086).
5) No secrets in help.

Deliverables:
- Full-screen help renderer (list + details pane).
- Navigation keys: ↑↓, PgUp/PgDn (optional), Enter for details, Esc to exit help.
- Optional search box (can be Phase 2; if included, deterministic).
- Headless test proves help mode renders (not just marker).
- Update plan evidence.

Acceptance:
- Manual: In TUI, `/help` replaces main layout with help view; Esc returns to normal layout.
- Headless: script "/help;/exithelp;/exit" (or equivalent) deterministically proves help mode displayed.
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS

Evidence:
- PR #180 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/180)
- Merge commit: a1a74d795f1b81263feaa83967bacfe75cff3b8c

### NA-0088 — TUI Focus Modes: full-screen Events/Status/Session/Contacts (scrollable; deterministic; test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Add full-screen Focus modes for each major pane:
  * Focus Events (deep event history, scroll, filters later)
  * Focus Status (more status history visible, scroll)
  * Focus Session (peer verification state and counters, scroll)
  * Focus Contacts (searchable list, scroll)
- Keep the dashboard minimal; Focus modes provide deep inspection.
- Ensure keybindings are practical in common terminals:
  * /help enters help mode; Esc exits
  * '?' toggles help mode (since F1 is intercepted by GNOME Terminal)
  * Focus toggles via F2-F5 (or other non-intercepted keys)

Invariants:
1) Focus mode is explicit-only: entering/exiting focus emits deterministic in-app events (no stdout markers in interactive mode).
2) Focus views are full-screen (no background context) and scrollable.
3) Deterministic rendering: stable ordering and stable strings for focus headers and lists.
4) No secrets in UI or markers.
5) Keymap must work in GNOME Terminal: do not depend on F1.

Deliverables:
- TuiMode extended with FocusEvents/FocusStatus/FocusSession/FocusContacts.
- Keybindings:
  * '?' and /help for help mode; Esc exits help.
  * F2 focus Events, F3 focus Status, F4 focus Session, F5 focus Contacts; Esc returns to Dashboard.
- Headless tests proving focus mode switches deterministically (markers like tui_focus pane=events on=true).
- Plan evidence updated.

Acceptance:
- Manual: toggle each focus mode; it takes over screen and Esc returns.
- Headless: scripted run proves focus mode entered and rendered deterministically.
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- No QSC_MARK printed to stdout in interactive mode (NA-0086 remains enforced).

Evidence:
- PR #182 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/182)
- PR #183 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/183) — merge SHA daf6bab657f75874d73d1106ac7d99c3780d98db

### NA-0089 — Evidence readability polish: deterministic counts in demo artifacts (drop/reorder/deliver)

Status: DONE

Scope:
- scripts/demo/** and demo artifact formats (implementation PR)
- docs/qsc/** runbooks may be updated to explain new fields
- No protocol-core changes

Objective:
- Make demo artifacts self-explanatory at a glance by adding deterministic counts to:
  * summary.txt
  * normalized_subset.txt (or an additional normalized_counts.txt)
  for both:
  * local demo scripts (qsc_demo_local.sh)
  * remote relay smoke (qsc_remote_relay_smoke.sh + workflow artifacts)

Invariants:
1) Counts must be derived from deterministic markers/events (not wall clock).
2) No secrets/payloads in artifacts (safe-to-share).
3) Same seed+scenario → same counts and same normalized subset (within defined subset rules).
4) Do not add new required PR checks; any new job must be non-blocking unless explicitly approved.

Deliverables:
- Artifact format update:
  - summary.txt includes: deliver_count, drop_count, reorder_count (and optionally dup_count)
  - normalized subset includes the same counts (or a separate normalized_counts file)
- Plan + executed evidence
- Docs/runbook note describing the new fields (if needed)

Acceptance:
- Local and remote scripts produce the new fields.
- Headless/CI artifacts include the new fields.
- Existing scripts still run without secrets and remain deterministic.

Evidence:
- PR #186 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/186)
- Merge commit: e62faf76d8f9f5608f07714e8a5c02d1a4b0a964

### NA-0090 — Remote scenario enforcement: client-side fault injection yields hostile markers

Status: DONE

Scope:
- qsl/qsl-client/qsc/** (implementation PR)
- scripts/demo/qsc_remote_relay_smoke.sh may be adjusted to validate markers/counts
- No server changes required

Objective:
- Ensure remote demo scenarios are meaningful:
  * happy-path has deliver_count>0 and drop/reorder counts 0
  * drop-reorder produces deterministic relay_event markers (drop/reorder/deliver) and non-zero counts when run against remote relay
- Achieve this via client-side deterministic fault injection keyed by seed+scenario (not server-side).

Invariants:
1) Fault injection is explicit-only: enabled only when scenario indicates it (drop-reorder) and seed is supplied.
2) Deterministic: same seed+scenario → same injected actions and same counts.
3) Safe-to-share: no payloads/secrets in markers or artifacts.
4) Does not weaken fail-closed behavior or no-mutation invariants.
5) Remote smoke continues to be manual/nightly, not required PR checks (existing policy).

Deliverables:
- qsc transport wrapper applies deterministic drop/reorder/dup actions locally (client side) and emits markers:
  event=relay_event action=<deliver|drop|reorder|dup>
- scripts/demo/qsc_remote_relay_smoke.sh updates: verify that in drop-reorder scenario, counts are non-zero and relay_event markers present.
- Tests proving marker generation under headless/CLI run without requiring real remote relay:
  * use local relay serve and run remote-smoke logic against it, or unit-test the injector.
- Plan evidence updated.

Acceptance:
- qsc tests/clippy pass
- Local test proves drop-reorder yields relay_event markers and non-zero counts
- Manual remote run against https://qsl.ddnsfree.com shows non-zero drop/reorder counts in summary.txt/normalized_counts.txt

Evidence:
- PR #188 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/188)
- PR #189 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/189) — merge SHA 2dff509b9e832ab986e1eb73e7098dec9d2976a7

### NA-0091 — Receive-path E2E: two-way exchange (CLI + TUI) over relay (explicit-only; test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR)
- scripts/demo/** only if needed for deterministic demo harness
- No protocol-core changes, no server changes

Objective:
- Enable explicit receive for qsc so two clients can exchange messages and files over relay.
- Integrate receive into the TUI as an explicit command (/receive), updating timeline/events deterministically.

Invariants:
1) Explicit-only: receive occurs only on explicit CLI/TUI command; no background polling.
2) Deterministic markers for receive lifecycle:
   - recv_start / recv_item / recv_commit / recv_none / recv_error (stable schema)
3) No mutation on failure/reject (receive does not mutate persistent state unless commit succeeds).
4) No secrets/payload contents in markers, UI, or artifacts.
5) Safe-parent checks and XDG correctness remain enforced.
6) TUI interactive mode prints no QSC_MARK to stdout (NA-0086 invariant).

Deliverables:
- `qsc receive --transport relay --relay <URL> --from <peer> --max <N>` (or equivalent explicit args)
- Two-way E2E tests (local relay):
  - alice sends → bob receives
  - bob sends → alice receives
  - assert markers and state transitions
- TUI:
  - `/receive` (and/or `/receive <peer>`) triggers explicit receive and appends received items to timeline (or a receive pane)
  - Focus views remain functional

Acceptance:
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- New tests prove two-way exchange over local relay with explicit receive.
- Headless TUI test proves /receive triggers receive markers and updates deterministic output (no stdout spam).

Evidence:
- PR #192 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/192) — merge SHA 533321405659e58b945701cc7dcec61ef3a26aa7

### NA-0092 — QSP/QSE on-wire enforcement: pack/unpack + truthy ACTIVE/INACTIVE status

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Enforce QSP/QSE on-wire usage:
  * pack/encrypt before push
  * verify/decrypt/unpack after pull
- Derive a truthy “QSP/QSE: ACTIVE|INACTIVE (reason=...)” status from real runtime behavior (not config).

Invariants:
1) Outbound on-wire bytes are QSP/QSE envelope ciphertext (not raw file bytes).
2) Inbound bytes are verified+decrypted+unpacked before write; rejects are deterministic and do not advance state.
3) Truthy status: “QSP/QSE: ACTIVE|INACTIVE (reason=...)” derived from actual pack/unpack success.
4) No payload/secrets in markers/UI/artifacts.
5) Server remains blind (ciphertext-only).

Deliverables:
- Add qsc dependency/wiring to QSP/QSE implementation (likely refimpl crate or minimal client-side library).
- Add tests that prove:
  * send path invokes pack/encrypt and on-wire bytes contain envelope header/magic/version (no raw payload).
  * receive path verifies/unpacks; rejects do not mutate.
  * status indicator flips truthfully.
- Update runbooks as needed.

Acceptance:
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- E2E test against embedded inbox proves pack+unpack roundtrip and on-wire is not raw.

Evidence:
- PR #195 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/195) — merge SHA 4b98291187a1bb64a8992ecfd787f1392f223c20

### NA-0093 — Truthful protocol status: QSP/QSE ACTIVE/INACTIVE (reason=...) in CLI + TUI

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR), plus tests planning now.
- No protocol-core changes.

Objective:
- Make protocol status truthy and deterministic in CLI + TUI:
  * ACTIVE/INACTIVE derived from local pack+unpack self-check (no network, no disk writes)
  * explicit reason for INACTIVE (no “reason=none”)

Invariants:
1) Status output is truthy and deterministic.
2) INACTIVE always includes explicit reason (no “reason=none”).
3) ACTIVE only if local pack+unpack self-check succeeds.
4) No secrets in status output.
5) TUI Status pane shows same ACTIVE/INACTIVE and reason (no stdout marker spam in interactive).

Deliverables:
- CLI status marker fields: qsp_status status=... reason=... version=...
- TUI Status focus pane line: “QSP/QSE: ACTIVE” or “INACTIVE (reason=...)”.
- Tests proving: seeded => ACTIVE; missing seed => INACTIVE reason=missing_seed; unsafe parent => INACTIVE reason=unsafe_parent; deterministic marker string; no-secrets guard.

Acceptance:
- cargo test -p qsc --locked PASS
- cargo clippy -p qsc --all-targets -- -D warnings PASS
- status_truthy_active_inactive test updated to match real behavior

Evidence:
- PR #198 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/198) — merge SHA `65bda575276a605a0bc9d8b10064d02fe74ecc45`

### NA-0094 — Fail-closed: refuse send/receive unless ACTIVE

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (implementation PR).

Objective:
- Hard gate send/receive unless protocol status is ACTIVE.

Invariants:
1) send/receive must refuse unless qsp_status is ACTIVE.
2) deterministic error marker: code=protocol_inactive reason=<explicit>.
3) no mutation on reject.
4) no payload/secrets in markers or UI.

Deliverables:
- send/receive checks against protocol status.
- Tests for refusal when INACTIVE.

Evidence:
- PR #201 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/201) — merge SHA `1d6aa6d78618dbb9d8dcc0bebd13550221e00cad`

### NA-0095 — Interactive handshake MVP (QSP/QSE): session establish over relay inbox; deterministic transcript tests; ratchet interfaces designed (not activated)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** (client-only)
- tools/refimpl/quantumshield_refimpl/src/crypto/** (PqKem768 implementation; minimal paths only)
- Uses qsl-server inbox contract (PUSH/PULL); server remains blind.

Objective:
- Establish fresh session keys via an interactive handshake over the inbox transport.
- Enforce transcript integrity and deterministic rejection on tamper/replay/out-of-order.
- Define ratchet interfaces (types + boundaries) but do not activate ratchet advancement.

Invariants:
1) No seed-derived session required: handshake establishes fresh session keys over the network (inbox).
2) Deterministic transcript verification: message order/version/domain separation enforced; tamper/replay rejected deterministically.
3) No mutation on reject: rejected handshake messages do not advance persistent session state.
4) No secrets in markers/UI/logs/artifacts.
5) Handshake MUST be PQ or PQ-primary hybrid; X25519-only is forbidden.
6) Ratchet interfaces are defined (types + state machine boundaries) but ratchet advancement is NOT activated.

Deliverables:
- CLI handshake commands (proposed), TUI lens markers, deterministic headless tests.
- Implement PqKem768 (ML-KEM-768) in refimpl; StdCrypto implements the trait.
- ACTIVE status becomes based on handshake completion (not just seed).
- Ratchet interface spec recorded in plan (types only; no activation).

Acceptance:
- Handshake completes in deterministic test harness.
- Tamper and out-of-order tests reject with deterministic markers.
- No-mutation tests cover reject cases.
- Tests prove PQ KEM is used (marker/length-only evidence); X25519-only handshake is not allowed.
- qsc fmt/test/clippy gates PASS.

Evidence:
- PR #205 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/205) — merge SHA `4145ea1`.

### NA-0096 — First ratchet step: send/recv chain advancement + skipped handling + PCS/FS test vectors

Status: DONE
Evidence: PR #211 merged (merge SHA d718a66e2b2bd4e2d42b36a2ca8cd59a936a73c0)

Scope:
- qsl/qsl-client/qsc/** (implementation PR)
- Uses existing refimpl ratchet APIs (no refimpl code changes in this governance PR).

Objective:
- Activate first ratchet step for send/receive with bounded skipped handling and PCS/FS test vectors.

Invariants:
1) Message key reuse forbidden (send chain advances per message).
2) Receive chain advances per message; replay detected deterministically.
3) Skipped-message keys stored bounded (cap) with deterministic eviction.
4) Rejects (tamper/out-of-order/replay) must not mutate persistent state.
5) No secrets in markers/UI/logs/artifacts.

Deliverables:
- Ratchet advancement implementation.
- Deterministic markers and tests for send/recv advance, skipped handling, replay/tamper rejects.

### NA-0099 — Handshake v1.1: A2 confirm (3-message) + deterministic transcript confirmation (PQ; fail-closed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (client). No refimpl/server/workflow changes.

Objective:
- Add A2 transcript-confirm message so responder commits session only after confirmation; eliminate half-established ambiguity.

What is protected:
- Session establishment correctness and explicit confirmation; fail-closed on tamper/replay/out-of-order.

Invariants:
1) PQ handshake remains PQ-primary (ML-KEM-768); no classical-only fallback.
2) Responder commits session only after valid A2.
3) Replays/out-of-order/tamper rejected deterministically with no mutation on reject.
4) No secrets in markers/UI/logs.

Deliverables:
- A2 message type + MAC over transcript using derived K_confirm.
- CLI + TUI /handshake command supports init/poll and confirm send (explicit-only).
- Deterministic tests proving:
  * B does not become ACTIVE until A2 arrives
  * A2 tamper/replay/out-of-order reject with no mutation

Acceptance:
- cargo fmt/test/clippy for qsc green; CI green.
- tests prove commit gating on B.

### NA-0100 — Identity binding MVP: TOFU + pin peer PQ identity fingerprint (client-only; test-backed)

Status: DONE
Evidence: PR #217 merged (merge SHA 8c0a472feb6ad4825d2212a5d244d7791f34a31e)

Scope:
- qsl/qsl-client/qsc/** only (client). No server/workflow changes.

Goal:
- Bind a stable peer identity to handshake/session to detect and refuse ongoing MITM.

MVP identity definition (PQ-aligned, no new crypto):
- Use peer’s PQ KEM public key fingerprint (or a stable fingerprint of it) as “identity key”.
- Display fingerprint in CLI/TUI.
- Store pinned fingerprint on first successful handshake (TOFU).

Invariants:
1) On first successful handshake with a peer, store pinned identity fingerprint (TOFU).
2) On subsequent handshakes, if fingerprint differs => deterministic reject (no session mutation).
3) No secrets in UI/markers/logs; only fingerprint/lengths.
4) Fail-closed on identity mismatch; send/receive remain blocked (protocol_inactive) if no valid session.

Deliverables:
- CLI: show peer fp (qsc status / handshake status) and mismatch marker.
- TUI Status pane: “Peer FP: … (pinned)” and mismatch warning.
- Tests: first pin, mismatch reject no mutation, determinism, no-secrets.

Acceptance:
- qsc fmt/test/clippy pass; CI green; deterministic markers.

### NA-0101 — PQ signature identity: ML-DSA identity signing + signed handshake binding

Status: DONE

Scope:
- qsl/qsl-client/qsc/** (client) + refimpl PQ signature primitives (ML-DSA).

Goal:
- Implement true PQ authentication using ML-DSA once available in refimpl:
  signed handshake transcript binding, pinned identity keys, revocation/rotation policy.
Evidence:
- PR #237 merged (merge SHA 03cc989a57d996a47e4a667e404c11b157843594).

### NA-0102 — Identity UX: qsc identity show/rotate + peers list (pinned fingerprints) + deterministic markers (test-backed)

Status: DONE
Evidence: PR #220 merged (SHA: 77613619296d31fdc2d213016c47c321bc3d12a0)

Scope:
- qsl/qsl-client/qsc/** only.

Deliverables:
1) `qsc identity show` — prints own fp + pinned status marker (no secrets).
2) `qsc identity rotate` — explicit rotation with confirmation flag; emits marker; rotates keypair; invalidates prior peer pins only if explicitly requested (default: keep peer pins).
3) `qsc peers list` — lists pinned peers with fp and status (pinned/mismatch/unknown).
4) TUI Status pane shows own fp + current peer fp (if session selected).

Invariants:
- No silent identity changes; rotate requires explicit `--confirm`.
- No secrets in output; only fingerprints.
- Deterministic markers for show/rotate/list.
- No mutation on rejected rotate (missing confirm).

Acceptance:
- Tests for show, rotate (confirm/no-confirm), peers list determinism, and no-secrets guard.
- qsc fmt/test/clippy pass; CI green.

### NA-0103 — Metadata minimization lane (qsc): fixed-interval polling + padding/bucketing + bounded batching (deterministic; test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (client). No server/workflow changes.

What Is Protected:
- Reduce observable metadata without weakening fail-closed invariants.

Invariants:
1) All metadata-min features are explicit-only (no hidden background behavior).
2) Deterministic mode exists for CI/demo (seeded; no wall-clock leakage in artifacts unless explicitly enabled).
3) Padding/bucketing bounded and documented; no unbounded queues.
4) Fixed-interval polling option bounded and has resource caps; no implicit retries/recovery.
5) No secrets in markers/logs; safe-to-share outputs.

Deliverables (Phase 1):
- CLI flags / commands to enable: fixed-interval polling, padding bucket selection, batch caps.
- Deterministic markers showing chosen cadence/bucket/batch.
- Tests proving: bounds, determinism, no mutation on reject.

Acceptance:
- qsc fmt/test/clippy pass; CI green.

Evidence:
- PR #223 merged (merge SHA 6e8d5dcda90fe73ba7fd9769b978c99d9b87f4d5).

---

### NA-0104 — TUI Layout v2: Inspector Drawer (H3) (Status/Events/Session/Contacts), responsive rules (test-backed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only (no server/workflow changes).

Deliverables:
1) Default screen uses H3 layout:
   - Left: optional Contacts list (collapsible).
   - Center: Timeline/Chat (only scroll region).
   - Right: Inspector pane (single pane, switchable): Status, Events, Session, Contacts.
2) Keybindings for inspector switching:
   - F2 Events, F3 Status, F4 Session, F5 Contacts.
   - Enter: focus current inspector into full-screen mode (scroll/search).
   - Esc: back.
3) Responsive rules:
   - If width < breakpoint, auto-hide Contacts; keep Inspector available.
   - If height < breakpoint, compress header/footer hints.
4) Deterministic headless tests:
   - render each inspector mode headless.
   - ensure no overflow/panic and deterministic output subset.

Invariants:
- Home screen must not show multiple tiny scrolling boxes.
- Only chat/timeline scrolls on home.
- Interactive mode emits no QSC_MARK to stdout.
- No secrets in UI output/markers.

Evidence:
- PR #227 merged (merge SHA 34c15522da4dfb271138662959006625f7a327f6).

### NA-0105 — Truthful ACTIVE requires validated session; remove seed fallback (client-only; fail-closed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only.

Objective:
- Ensure protocol status truthfulness and fail-closed behavior by removing production synthetic/session fallback paths.

Invariants:
1) Protocol status MUST NOT become ACTIVE via seed/synthetic session in production.
2) ACTIVE requires a validated stored session (handshake-established and parse/validate succeeds).
3) If session is invalid or missing, status is deterministic INACTIVE with explicit reason.
4) Fail-closed: send/receive refuse when INACTIVE.

Deliverables:
- Remove or lock down `QSC_QSP_SEED` synthetic session fallback to test-only override.
- Replace any-file ACTIVE heuristic with validated session load checks.
- Add deterministic tests for ACTIVE/INACTIVE truth table and fail-closed send/receive gating.

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- Tests prove no production path to ACTIVE without validated session state.

Evidence:
- PR #231 merged (merge SHA a8cc0f85559c73f203bc96ea10fc5fd26406f3cf).

### NA-0106 — Identity secret at rest: encrypt/private-key storage + legacy migration (client-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only.

Objective:
- Ensure identity secret material is not stored plaintext on disk and provide deterministic migration for legacy files.

Deliverables:
- Encrypted-at-rest or keyring/vault-backed identity secret storage.
- Migration path from legacy plaintext identity files.
- Deterministic tests for migration safety and no secret leakage in outputs.

Evidence:
- PR #234 merged (merge SHA 9f8ac906707bf261331dbb5cada61d3a9636da29).

### NA-0107 — Remote relay auth regression: Bearer header support for inbox push/pull (client-only; fail-closed)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** only.

Objective:
- Restore remote relay compatibility for auth-gated deployments by sending Authorization headers for relay inbox push/pull when token env is present.

Invariants:
1) If `QSC_RELAY_TOKEN` or `RELAY_TOKEN` is set (non-empty), qsc relay inbox push/pull sends `Authorization: Bearer <token>`.
2) Token is never printed in markers, logs, UI, or artifacts.
3) Auth remains optional: when token env is unset, behavior is unchanged for open/local relays.
4) Unauthorized responses map to deterministic error code (`relay_unauthorized`) and keep no-mutation guarantees.

Deliverables:
- Add optional bearer-header injection in qsc relay inbox push/pull client.
- Add deterministic unauthorized mapping and marker coverage.
- Add tests covering no-token unauthorized failure and token-present success.
- Prove nightly/manual remote workflow lane passes for `happy-path seed=1` and `drop-reorder seed=7`.

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- `remote-relay-tests` workflow PASS for both dispatch scenarios above.

Evidence:
- PR #243 merged (merge SHA b74e21a22ebc7f287e19c8459ac21ec9996c617f).
- `remote-relay-tests` PASS: happy-path seed=1 (https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/21792900305).
- `remote-relay-tests` PASS: drop-reorder seed=7 (https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/21792900550).

### NA-0108 — Remote handshake tests lane: ACTIVE(reason=handshake) + bidirectional send/receive (fail-closed)

Status: DONE

Scope:
- `.github/workflows/remote-handshake-tests.yml` (new)
- `scripts/demo/qsc_remote_handshake_smoke.sh` (new)
- `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md` (Handshake lane section)
- `docs/archive/testplans/NA-0108_remote_handshake_lane_plan.md` (new plan stub)
- Governance tracking updates only (`NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`)
- `qsl/qsl-client/qsc/**` allowed for minimal receive mailbox/peer-label separation required to keep remote handshake lane fail-closed without seed fallback.

Objective:
- Add a remote workflow lane that proves real handshake-established sessions without seed fallback, then proves bidirectional `send`/`receive` with strict marker assertions.

Invariants:
1) No `QSC_ALLOW_SEED_FALLBACK` usage in the lane.
2) Workflow trigger policy is `workflow_dispatch` + `schedule` only (no `pull_request`, no required PR check wiring).
3) Fail closed if any `protocol_inactive` or `relay_unauthorized` marker appears.
4) Fail closed if any required marker is missing:
   - `qsp_pack ok=true` for both directions
   - `qsp_unpack ok=true` for both directions
   - `recv_commit count>=1` for both directions
5) Artifacts are safe-to-share with URL/token redaction and deterministic normalized subset.

Deliverables:
- New workflow `remote-handshake-tests` runs two fixed scenarios:
  - `happy-path` with `seed=1`
  - `drop-reorder` with `seed=7`
- New smoke script performs:
  - explicit four-step handshake (`init/poll/poll/poll`)
  - handshake-established assertions
  - bidirectional relay send/receive assertions
  - artifact generation: `alice.log`, `bob.log`, `alice_recv.log`, `bob_recv.log`, `summary.txt`, `normalized_subset.txt`, `normalized_counts.txt`, `markers`

Acceptance:
- Workflow YAML contains no `pull_request` trigger.
- Script fails closed on all required lane invariants.
- Artifacts include summary/count evidence for both directions.

Evidence:
- Plan stub: `docs/archive/testplans/NA-0108_remote_handshake_lane_plan.md`.
- Governance scope expansion: PR #247 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/247) — merge SHA `98bc981624503f7067490cd3d4f8c5f0d6a3184f`.
- Implementation (mailbox/peer separation + lane hardening): PR #248 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/248) — merge SHA `ee7e789587c1a792ebf8e8398ed0ca84f9387b80`.
- Implementation follow-ups:
  - PR #249 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/249) — merge SHA `897ba4d257682924718e43223e188c7653c8dd1a`.
  - PR #250 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/250) — merge SHA `d9de78ef639390b2e37daed36a4a8c1b7c8dbb98`.
  - PR #252 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/252) — merge SHA `1afa7a3070701d6704646ca61ee5f9c89ce3b7fd`.
- `remote-handshake-tests` PASS:
  - happy-path + drop-reorder run A: https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/21794286407
  - happy-path + drop-reorder run B: https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/21794286815
- Artifact proof (run `21794286407` happy-path): `status=pass`, `handshake=ACTIVE(reason=handshake) both_peers`, `qsp_pack_ok=true both_directions`, `qsp_unpack_ok=true both_directions`, `recv_commit_bob=1`, `recv_commit_alice=1`, `protocol_inactive_count=0`, `relay_unauthorized_count=0`.

### NA-0109 — Session/ratchet state at rest: encrypt + integrity-check + legacy migration (client-only; test-backed)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**` only.
- No server, workflow, or refimpl changes.

Objective:
- Protect session/ratchet keys and counters at rest against disk disclosure, backup/snapshot leaks, and tamper replay by requiring encrypted + integrity-checked storage with deterministic fail-closed behavior and safe legacy migration.

Invariants:
1) No plaintext session/ratchet key material on disk.
2) Session state must be encrypted + integrity-checked before load (tamper => deterministic reject; no mutation).
3) Fail-closed: if vault/secret unavailable, protocol cannot become `ACTIVE(reason=handshake)` and send/receive refuse deterministically.
4) Migration is safe + idempotent:
   - legacy plaintext session file is migrated only when vault is available; otherwise it remains unchanged and client emits deterministic `migration_blocked` marker.
5) No secrets in output/markers.

Deliverables:
- New encrypted session store format (vault-backed), plus legacy migration.
- Tests proving the invariants, including tamper reject + no mutation.
- CI gates green.

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- Tests explicitly prove:
  - no plaintext session/ratchet key material on disk
  - tamper reject with no mutation
  - deterministic refuse + no mutation when vault unavailable
  - migration idempotent

Evidence:
- Plan stub: `docs/archive/testplans/NA-0109_session_state_at_rest_plan.md`.
- Implementation: PR #255 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/255) — merge SHA `943e9a7964d5a908112386da3833bb1eb032c0ab`.

### NA-0110 — Provenance light touch: NOTICE + PROVENANCE + signed-release runbook

Status: DONE

Scope:
- Governance files only:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
  - `docs/archive/testplans/NA-0110_provenance_lighttouch_plan.md`
- Repo-root docs only:
  - `NOTICE`
  - `PROVENANCE.md`
  - `SIGNED_RELEASES_RUNBOOK.md`
- No code, CI workflow, or protocol behavior changes.

Objective:
- Add a lightweight, fail-closed provenance baseline for public consumers:
  - repository notice and licensing pointer
  - provenance guidance tied to authoritative CI proof lanes
  - signed-tags and checksum verification runbook instructions (no in-repo key generation)

Invariants:
1) No protocol/client/server/runtime behavior changes.
2) No `.github/workflows/**` edits.
3) Official proof references distinguish:
   - `remote-handshake-tests` = handshake proof lane
   - `remote-relay-tests` = `seed_fallback_test` transport health check lane
4) Guidance never asks users to trust unaudited binaries.

Deliverables:
- Add `NOTICE` at repo root with AGPL reference and canonical repository references.
- Add `PROVENANCE.md` at repo root with verification and trust-model guidance.
- Add `SIGNED_RELEASES_RUNBOOK.md` at repo root with signed-tag and checksum verification instructions.
- Implementation complete: PR #258 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/258), merge SHA `0c15b124cec15744c5e9b7d375fb5f545f06249b`.

Acceptance:
- Scope guard for each PR contains only allowed files.
- Checks green on governance, implementation, and close-out PRs.
- NA-0110 closed to `DONE` with evidence and `READY=0`.

### NA-0111 — Client lifecycle hardening (qsc): startup->runtime->shutdown security

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**` only (implementation PR); no server/refimpl/workflow changes.

What is being protected:
- Secrets in memory (keys, passphrases, session state, tokens).
- Secrets at rest (vault/session blobs remain protected; no new plaintext artifacts).
- Output channels (stdout/stderr/log files/markers).
- Crash surfaces (panic paths, backtraces, core dumps).

Non-negotiable invariants:
1) No secrets in stdout/stderr/markers/logs, including error paths.
2) Deterministic failure markers; fail-closed everywhere.
3) Panic safety:
   - no secret-bearing panic messages
   - backtraces off by default in release runs (documented)
   - recommend disabling core dumps in runbook guidance
4) Safe directory posture:
   - all state/config roots safe-parent verified
   - no writes into CWD and no repo-root artifacts (vault.qsv-class bug remains impossible)
5) Secret lifetime minimization:
   - zeroize secrets as soon as practical
   - minimize secret-bearing copies
6) Shutdown hygiene:
   - temporary files removed
   - no lingering plaintext buffers on disk

Deliverables:
- Tighten lifecycle guards in qsc (implementation PR):
  - centralized redaction guard
  - explicit env handling for tokens
  - panic hook with redaction + deterministic marker
  - deterministic debug-mode gating (explicit)
- Add regression tests proving invariants:
  - no-secrets scan across outputs
  - panic path does not leak
  - CWD write regression tests (vault/session/temp)
  - fail-closed + no-mutation on lifecycle rejects
- CI gates green.

Acceptance:
- New tests added and passing; tests prove:
  - no secret substrings across outputs
  - panic path safe
  - no CWD artifacts created
  - deterministic markers on lifecycle failures
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS

Evidence:
- Plan stub: `docs/archive/testplans/NA-0111_client_lifecycle_hardening_plan.md`.
- Implementation PR complete: #261 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/261), merge SHA `fefcaae8d56c9606fce7010b6d0179a24923f768`.

### NA-0112 — Metadata minimization Phase 2 (qsc): fixed-interval poll + padding/bucketing + bounded batching + cover traffic knobs (deterministic; test-backed)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**` only.
- No server changes.
- No workflow changes.

What is being protected:
- Timing metadata (when user sends/receives).
- Size metadata (payload sizes).
- Batch/queue metadata (burst patterns).
- Receipt-class distinguishability (ACK camouflage).

Non-negotiable invariants:
1) Deterministic scheduling mode exists:
   - fixed-interval poll tick (no wall-clock drift in deterministic mode)
2) Size buckets enforced:
   - outbound payloads padded to explicit, bounded buckets
3) Bounded batching:
   - max batch size and max latency (no unbounded queueing)
4) Explicit-only cover traffic:
   - if enabled, cover traffic is deterministic + bounded + visibly marked (no silent background)
5) No hidden retries/recovery:
   - all behavior emits deterministic markers; no implicit behavior in TUI

Deliverables:
- CLI knobs (explicit):
  - `qsc meta plan` dry-run showing selected tick/bucket/batch parameters
  - `qsc send`/`qsc receive` honor bucketing/batching in deterministic mode
- Deterministic markers:
  - `meta_tick tick=n interval_ms=...`
  - `meta_bucket bucket=...`
  - `meta_batch count=... bytes=...`
  - `meta_cover enabled=true` (if used)
- Regression tests:
  - determinism across runs
  - bounds enforced (batch size/latency)
  - no secrets in output/markers
  - no mutation on reject

Acceptance:
- New tests are added and pass for deterministic replay, bounds, reject/no-mutation, and no-secret output checks.
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS

Evidence:
- Plan stub: `docs/archive/testplans/NA-0112_metadata_minimization_phase2_plan.md`.
- Implementation PR complete: #264 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/264), merge SHA `79e7c779ab26d187395335ead65114c76e922a8b`.

### NA-0113 — Delivered receipts (client ACK): explicit-only, camouflaged, bounded; deterministic markers; test-backed

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**` only.
- No server changes; receipts travel via the same relay inbox path as normal messages.

What is being protected:
- Prevent false delivery claims (`delivered` means receiver decrypted/unpacked).
- Minimize metadata leakage (no server-generated receipts; avoid presence oracle).
- Maintain explicit-only behavior (no silent background receipts).

Non-negotiable invariants:
1) Two distinct meanings:
   - `delivered_to_relay`: relay accepted push.
   - `delivered_to_peer`: sender received peer client ACK.
2) Receipt generation:
   - Receiver emits ACK only after `qsp_unpack ok=true` for that message.
   - ACK is encrypted as a normal message (relay cannot infer plaintext type).
3) Camouflage:
   - ACK fits same size bucket class as the small-message class.
   - No distinct receipt-only observable class in plaintext transport.
4) Explicit-only:
   - Receipts are OFF by default.
   - Explicit CLI/TUI opt-in required.
5) Deterministic markers:
   - `receipt_send kind=delivered msg_id=<redacted> bucket=...`
   - `receipt_recv kind=delivered msg_id=<redacted>`
   - `receipt_disabled` when off.
6) No presence:
   - Server does not generate delivered receipts.
   - Client ACKs are batchable/deferrable; no immediate online ping requirement.
7) Fail-closed + no-mutation on reject:
   - Receipt logic failures do not silently mutate message state.

Deliverables:
- CLI/TUI surface (implementation PR):
  - `qsc send --receipt delivered` (explicit)
  - `qsc receive --emit-receipts delivered` (explicit)
  - TUI toggle/indicator for receipts enabled
- Tests:
  - local two-client inbox flow: send -> receiver unpack -> ack -> sender receives ack marker
  - ACK bucket camouflage checks
  - receipts off => no ACK sent
  - no secrets in outputs/markers
  - no mutation on reject/tamper
- CI gates green.

Acceptance:
- Added tests prove on/off behavior, tamper/replay safety, delayed ACK handling, camouflage bounds, and no-secret outputs.
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS

Evidence:
- Plan stub: `docs/archive/testplans/NA-0113_delivered_receipts_plan.md`.
- Implementation PR complete: #267 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/267), merge SHA `7aef7330696f4a31e21d44b432a7b0ea0c37a310`.

### NA-0114 — TUI Phase: readability + information density (H3 inspector + focus panes) — timestamps, scroll/search, hints, responsive rules (test-backed)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**` only.
- No server/refimpl/workflow changes.

What is being protected:
- Operator clarity (no misleading UI).
- Deterministic observability for the security lens.
- Metadata discipline (no presence signal and no extra network chatter).
- Accessibility and ergonomics at common terminal sizes.

Non-negotiable invariants:
1) Home screen remains uncluttered:
   - H3 inspector is single-pane summary; no tiny scroll boxes.
2) Home scroll behavior:
   - Only Timeline scrolls on home; inspector is summary-only.
3) Focus panes are full-screen, scrollable, and optionally searchable:
   - Focus Events: scroll + timestamps; optional filter/search.
   - Focus Status: expanded history + key epochs/ratchet counters + protocol mode.
   - Focus Session: per-peer details and recent handshake/ratchet markers.
   - Focus Contacts: pinned peers + fingerprint + mismatch status.
4) Timestamp behavior:
   - Local display timestamps only; no wall-clock dependency in deterministic/headless mode.
   - Deterministic tests use tick counters or deterministic time source.
5) Stable and visible keybindings:
   - `F2`–`F5` inspector
   - `Ctrl+F2`–`Ctrl+F5` jump to focus
   - `Enter` focus current pane
   - `Esc` back
   - `/help` opens full-screen help
6) Interactive mode emits no `QSC_MARK` to stdout.

Deliverables:
- Improve inspector summaries for small terminals (truncate/ellipsis rules).
- Add timestamp rendering in focus panes with deterministic headless behavior.
- Add search/filter in at least Focus Events if feasible without excessive complexity.
- Add tests:
  - headless render for all focus panes with timestamps present
  - determinism of marker subset
  - no overflow/panic at small width/height breakpoints
  - no secrets in output

Acceptance:
- New or updated headless tests prove:
  - timestamps present in deterministic form
  - focus panes use full-height scroll region
  - keybindings remain consistent
  - interactive stdout has no `QSC_MARK`
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS

Evidence:
- Plan stub: `docs/archive/testplans/NA-0114_tui_readability_h3_plan.md`.
- Implementation PR complete: #270 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/270), merge SHA `7ff06a282802b17735538d56ddb44b6adfac8d96`.

### NA-0115 — Local unlock gate (vault/session/identity) (client-only; fail-closed)

Status: DONE

Invariants:
- locked-by-default; explicit unlock required for send/receive/handshake/rotate
- deterministic marker: `event=error code=vault_locked`
- no mutation on reject; no secrets in output

Deliverables:
- CLI + TUI unlock surface (local only; no server presence)
- tests: locked refuses all sensitive ops; unlock enables; no-secrets; deterministic

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- CI green

Evidence:
- Plan stub: `docs/archive/testplans/NA-0115_local_unlock_gate_plan.md`.
- Implementation PR complete: #274 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/274), merge SHA `6c56a1eb0ddd3514453001284d039d79ebd9b2cc`.

### NA-0116 — Contacts + verify + block (pinned fingerprints; mismatch UX) (client-only)

Status: DONE

Invariants:
- no silent trust; mismatch is explicit error state; block is deterministic

Deliverables:
- add/remove/list/verify/block; mismatch workflow; TUI status shows peer pin state
- tests: pinning, mismatch reject/no mutation, determinism, no-secrets

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- CI green

Evidence:
- Plan stub: `docs/archive/testplans/NA-0116_contacts_verify_block_plan.md`.
- Implementation PR complete: #277 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/277), merge SHA `ed03ad8806b712d4de3d9c75d69b4c6ebb5edca3`.

### NA-0117 — Encrypted conversation timeline store (client-only; at-rest protected)

Status: DONE

Invariants:
- no plaintext messages on disk; tamper reject; no mutation on reject

Deliverables:
- store/list/view timeline entries; dedupe; bounded retention knobs
- tests: encrypted-at-rest, tamper reject/no mutation, deterministic ordering

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- CI green

Evidence:
- Plan stub: `docs/archive/testplans/NA-0117_encrypted_timeline_store_plan.md`.
- Implementation PR complete: #280 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/280), merge SHA `e0db6eef10f6df3df88fc6c634e5d25f94e351b8`.

### NA-0118 — Message state model (honest delivery states) (client-only)

Status: DONE

Invariants:
- never claim `delivered_to_peer` without `receipt_recv`

Deliverables:
- per-message state transitions + deterministic markers
- tests: no false delivered, deterministic transitions, no mutation on reject

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- CI green

Evidence:
- Plan stub: `docs/archive/testplans/NA-0118_message_state_model_plan.md`.
- Implementation PR complete: #283 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/283), merge SHA `141c929c50f0611840c9ba0725452c4cf1c5cd27`.

### NA-0119 — File transfer MVP (bounded, integrity checked) (client-only)

Status: DONE

Invariants:
- bounded memory; integrity verified; fail-closed oversize/tamper

Deliverables:
- send/receive file payload with limits + hash/MAC
- tests: tamper reject/no mutation, oversize reject, deterministic markers

Acceptance:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS
- CI green

Evidence:
- Plan stub: `docs/archive/testplans/NA-0119_file_transfer_mvp_plan.md`.
- Implementation PR complete: #286 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/286), merge SHA `8d03a6fbd80b2307c7e09e4c9acfbda55d0f6404`.

### NA-0120 — QSC TUI Spec & Invariants (unified left nav + main + command bar) (docs-only)

Status: DONE

Scope:
- docs-only

Deliverables:
- `docs/qsc/QSC_TUI_SPEC.md` (or existing canonical qsc docs location) defining:
  - unified left nav pane rules (one domain expanded; headers visible; nav-only)
  - main panel semantics per domain
  - command bar explicit-intent grammar
  - focus model + keybindings
  - bounded auto-update rules (no focus stealing; unread counters)
  - status containment rule
  - multi-select allowed only Files/Logs; never Messages/Keys
- `docs/qsc/QSC_TUI_INVARIANTS.md`

Acceptance:
- Spec is sufficiently precise for implementation without guesswork.
- Spec explicitly references NA-0118/NA-0119 state semantics (message/file states) as UI truth sources.

Evidence:
- Implementation PR complete: #290 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/290), merge SHA `5ef289bbb10cc81d01181d00d230eadaed758407`.

### NA-0121 — QSC TUI Implementation: unify layout per NA-0120 (client-only)

Status: DONE

Scope:
- qsc client only

Acceptance:
- TUI matches NA-0120 structure.
- No inline actions; command bar only for intent.
- Auto-update bounded and visible; never steals focus.
- Deterministic headless markers/tests remain unchanged and green.
- Add minimal TUI invariant tests if feasible, or add an explicit test plan file if not.

Evidence:
- Implementation PR complete: #293 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/293), merge SHA `833c8d59f29c18eff143ebdbaf3c8392cd64a69d`.

### NA-0122 — Tooling hardening: preflight + post-merge verifier + goal-lint robustness (governance/dev tooling)

Status: DONE

Scope:
- repo tooling only (scripts/docs/workflows as needed)

Acceptance:
- Preflight scripts for governance and qsc implementation.
- Post-merge verifier script for close-outs.
- Goal-lint robustness improvement plan (document close/reopen remediation; optionally add edited trigger).

Evidence:
- Implementation PR complete: #296 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/296), merge SHA `5a34880d036680aaf5897baecb17197978a1102b`.

### NA-0123 — TUI Messages + Contacts feature-complete (truthful states) + invariant tests (client-only)

Status: DONE

Scope:
- client-only (qsc)
- normative references: `docs/qsc/QSC_TUI_SPEC.md`, `docs/qsc/QSC_TUI_INVARIANTS.md`

Requirements:
- bounded auto-update (no focus steal)
- status containment
- explicit intent via command bar only (no inline actions)
- deterministic headless markers unchanged
- add/extend `tui_*` invariant tests where feasible

Messages specifics:
- left nav shows conversations and unread counts
- main panel shows conversation stream
- reflect NA-0118 message states truthfully (no over-claiming)
- commands mapped to command bar: send, verify, export

Contacts specifics:
- list and verification/pinning view
- no content leakage via previews

Acceptance:
- render invariants for Messages/Contacts views pass
- at least one focus + auto-update counter behavior test passes

Evidence:
- Implementation PR complete: #300 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/300), merge SHA `c495b8d08ba6bf194a67254365462330b21befa6`.

### NA-0124 — TUI Files feature-complete (multi-select allowed) + invariant tests (client-only)

Status: DONE

Scope:
- client-only (qsc)
- normative references: `docs/qsc/QSC_TUI_SPEC.md`, `docs/qsc/QSC_TUI_INVARIANTS.md`

Requirements:
- bounded auto-update (no focus steal)
- status containment
- explicit intent via command bar only (no inline actions)
- deterministic headless markers unchanged
- add/extend `tui_*` invariant tests where feasible

Files specifics:
- list and inspection views
- multi-select allowed only in Files domain (and Logs if present)
- reflect NA-0119 file states truthfully (never claim verified before manifest verified)

Acceptance:
- multi-select rendering invariant passes
- command bar presence invariant passes

Evidence:
- Implementation PR complete: #303 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/303), merge SHA `4b0376e0596c5c4acc61ae0e12ebc13f56622da9`.

### NA-0125 — TUI Keys + Activity + Status feature-complete + invariant tests (client-only)

Status: DONE

Scope:
- client-only (qsc)
- normative references: `docs/qsc/QSC_TUI_SPEC.md`, `docs/qsc/QSC_TUI_INVARIANTS.md`

Requirements:
- bounded auto-update (no focus steal)
- status containment
- explicit intent via command bar only (no inline actions)
- deterministic headless markers unchanged
- add/extend `tui_*` invariant tests where feasible

Domain specifics:
- Keys: inspection-first; dangerous operations command-bar only; never multi-select
- Activity: ledger view
- Status: snapshot view

Acceptance:
- domains render correctly and do not duplicate status everywhere
- corresponding `tui_*` domain invariants pass

Evidence:
- Implementation PR complete: #306 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/306), merge SHA `22047cbfda857caba4d8ae034056aa4d73066c7d`.

### NA-0126 — TUI Settings + Lock feature-complete + leakage audit checklist + invariant tests (client-only)

Status: DONE

Scope:
- client-only (qsc)
- normative references: `docs/qsc/QSC_TUI_SPEC.md`, `docs/qsc/QSC_TUI_INVARIANTS.md`

Requirements:
- bounded auto-update (no focus steal)
- status containment
- explicit intent via command bar only (no inline actions)
- deterministic headless markers unchanged
- add/extend `tui_*` invariant tests where feasible

Settings/Lock specifics:
- Settings domain: read-only policy view plus explicit maintenance commands
- Lock domain: explicit lock/unlock UX with correct locked-state redaction

Leakage audit checklist acceptance items:
- no sensitive previews in nav
- locked-state redaction verified in focused and unfocused states
- no status spam outside Activity/Status containment
- no inline dangerous actions outside command bar

Acceptance:
- locked-state redaction render invariant passes
- no-preview-in-nav invariant passes

Evidence:
- Implementation PR complete: #309 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/309), merge SHA `5533d3f982a3e0ef28ddaee51ae3651a41e730fb`.

### NA-0127 — Relay-backed UI integration lane (non-required initially)

Status: DONE

Scope:
- client-only + CI workflow lane (non-required initially)

Goal:
- Add 1–3 relay-backed integration scenarios that exercise:
  1) inbound message while unfocused increments counter only (no auto-append)
  2) inbound message while main focused appends to stream
  3) file transfer inbound reflects NA-0119 truth states in Files view
- Scenarios must run against qsl-server relay in a controlled way.

Constraints:
- Must be non-required initially (nightly/manual lane) to avoid PR flakiness.
- Must not leak secrets (use GitHub Actions secrets as needed).
- Must produce deterministic-ish artifacts/log markers sufficient for auditing.

Acceptance:
- Workflow exists and runs successfully on demand (`workflow_dispatch`) and/or scheduled.
- Clear documentation in workflow comments for secrets needed (e.g., `RELAY_TOKEN`).
- One proof run link recorded in TRACEABILITY once implemented.

Evidence:
- Implementation PR complete: #313 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/313), merge SHA `9ecf8b4174c9c9a81344a78a85c883f6e79fc9e3`.
- Workflow fix PR complete: #314 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/314), merge SHA `2748e7a764489954257d4592e2d7fe8f674a845a`.
- Successful relay UI integration proof run: https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/21888936094

### NA-0128 — TUI Locked-first startup + zero-leak pre-unlock shell + init/unlock UX (client-only)

Status: DONE

Scope:
- qsc client-only

Invariants:
- Pre-unlock displays nothing sensitive (no alias, no IDs, no counts, no protocol state, no files/contacts/messages).
- Locked nav shows only `Unlock` and `Exit`, with `Unlock` selected by default.
- Locked main shows only `Locked — unlock required`, or first-run `No vault found — run /init`.
- `/help` is disabled while locked.
- While locked, only `/unlock` (or `/init` if no vault) and `/exit` are accepted; all other commands return deterministic `locked: unlock required`.
- Passphrase creation enforces strong passphrase with explicit no-recovery warning and typed acknowledgement.
- Alias is required, local-only, stored encrypted, and never shown pre-unlock.

Acceptance:
- Render tests prove locked mode has zero-leak output and restricted nav/commands.
- Deterministic markers prove command rejection while locked.

Evidence:
- Implementation PR complete: #319 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/319), merge SHA `847a3b83ce7059a61581c807042013f09c878ced`.

### NA-0129 — TUI chrome simplification + Help/About/Legal (post-unlock only) + remove debug noise (client-only)

Status: DONE

Scope:
- qsc client-only

Requirements:
- Remove `Nav [focus]` and `Main: ...` labels; nav top-left shows only `QSC`.
- Command bar is minimal: `Cmd: /help` post-unlock and `Cmd: /unlock` while locked.
- `Help`, `About`, and `Legal` nav items exist post-unlock only; main panel shows corresponding content when selected.
- Remove/disable internal marker/debug lines from normal main views.

Acceptance:
- Render tests assert headers are removed, `QSC` branding is present, command bar is minimal, and `Help`/`About`/`Legal` appear post-unlock only.
- Deterministic render output remains stable in headless mode.

Evidence:
- Implementation PR complete: #322 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/322), merge SHA `a61a8ed78881e9d0dcedd71154843d6431bd26af`.

### NA-0130 — Auto-lock (inactivity) enabled by default + adjustable timeout + clear UI buffers on lock (client-only)

Status: DONE

Scope:
- qsc client-only

Requirements:
- Auto-lock is enabled by default with adjustable timeout (default target: 10 minutes).
- Any keypress counts as activity.
- On auto-lock: lock state engages, UI buffers are cleared to reduce terminal scrollback leakage, and UI returns to locked shell.
- Exit remains available while locked.

Acceptance:
- Tests prove auto-lock triggers after simulated inactivity and clears/redacts display deterministically.
- Tests prove activity input resets inactivity timer.

Evidence:
- Implementation PR complete: #325 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/325), merge SHA `3a5c893fc672d64a9a5e27f09487d568f3f595e3`.

### NA-0131 — Locked/Cmd/Init UX hardening + cmd cursor (steady block) (client-only)

Status: DONE

Scope:
- qsc client-only

Goals:
1. Locked mode key gating:
   - While locked, ONLY allow: Up/Down, Enter, Tab, Esc, `/`, and typing in Cmd when focused.
   - Disable Ctrl+Fx focus shortcuts and any other hotkeys while locked.
2. Cmd focus + echo:
   - `/` always focuses Cmd and inserts `/`.
   - Cmd input is visible when focused (echo), with a steady non-blinking BLOCK cursor.
   - Esc returns focus to Nav and clears partial input.
   - Tab toggles focus Nav <-> Cmd (locked mode).
3. Cmd placeholder behavior:
   - When Cmd NOT focused: show `Cmd:` only (no sticky `Cmd: /init` text).
   - When Cmd focused: show `Cmd: <user input>`.
   - After executing a command: clear input and revert to `Cmd:`.
4. Enter activation while locked:
   - Enter on selected `Exit` exits.
   - Enter on `Unlock` starts unlock flow (or focuses Cmd with `/unlock`).
   - First-run `No vault found` allows `/init` and must not appear frozen.
5. `/init` wizard visibility:
   - `/init` drives visible step prompts (Alias -> no-recovery ack -> passphrase -> confirm).
   - No silent waiting states.

Acceptance:
- Render tests prove Cmd echo + steady block cursor when focused.
- Render tests prove exactly one selection marker in nav.
- Render tests prove Enter activates Exit/Unlock.
- Render tests prove locked mode disables Ctrl+Fx focus shortcuts.
- Render tests prove `/init` does not freeze (visible prompt state).

Evidence:
- Implementation PR complete: #329 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/329), merge SHA `6077cfb8e078e602f728c3f409b52b5ab560b5c1`.
- Follow-up implementation PR complete: #330 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/330), merge SHA `23ce4dcc34419a19332b9e128901e2aebadb8151`.
- Lock/unlock UX polish PR complete: #331 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/331), merge SHA `c45defd5d5f939ce1ac4a13a1ad651123cc916ac`.
- UX cleanup PR complete: #332 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/332), merge SHA `f32b2a0b645af19147c4ed678727ae5846671fb4`.

### NA-0132 — Audit Charter + Threat Model (Protocol + Metadata focus) (docs-only)

Status: DONE

Scope:
- docs-only governance/audit artifacts

Protect/Never-Happen Invariants:
- Never normalize protocol or metadata risks without explicit threat-model coverage.
- Never treat untrusted transport or endpoint metadata as inherently safe.
- Never convert audit uncertainty into implementation assumptions.

Deliverables:
- Audit charter document defining scope, assumptions, exclusions, severity rubric, and evidence standards.
- Threat model covering protocol state transitions, key lifecycle, relay assumptions, and metadata exposure surfaces.
- Asset/trust-boundary map for client, relay, storage, and operator environments.

Acceptance:
- Charter + threat model are complete, reviewable, and traceable to protocol/metadata concerns.
- No fixes during audit; implementation changes are explicitly deferred.
- Findings are converted into follow-on NAs with clear ownership/scope.

Evidence:
- Implementation PR complete: #336 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/336), merge SHA `1daceebb3d50c1fc9461c9c4b2c892ac83d74c96`.

### NA-0133 — Protocol Security Audit + SPQR/Triple Ratchet gap analysis (docs-only)

Status: DONE

Scope:
- docs-only protocol audit outputs

Protect/Never-Happen Invariants:
- Never claim protocol security properties that are not evidenced by implementation and tests.
- Never accept downgrade/replay/order ambiguity without explicit fail-closed treatment.
- Never merge protocol-hardening claims without documented gap analysis.

Deliverables:
- Protocol security audit report with evidence-backed findings and severity ratings.
- SPQR/Triple Ratchet gap analysis against current QSC/qsl-protocol behavior.
- Prioritized remediation matrix mapping each gap to follow-on NA candidates.

Acceptance:
- Report distinguishes confirmed findings, assumptions, and unknowns.
- No fixes during audit; report-only output in this NA.
- Findings are translated into follow-on NAs (one or more per material gap).

Evidence:
- Implementation PR complete: #339 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/339), merge SHA `5fe0cb5855f4137862313bdabc0b3fa3224626e2`.

### NA-0134 — Metadata Leakage Audit + mitigation matrix (docs-only)

Status: DONE

Scope:
- docs-only metadata/privacy audit outputs

Protect/Never-Happen Invariants:
- Never leak sensitive metadata by default in UI, logs, telemetry, or transport headers.
- Never classify “helpful diagnostics” as acceptable if they increase deanonymization risk.
- Never ship mitigations without explicit threat/impact mapping.

Deliverables:
- Metadata leakage audit report covering at-rest, in-transit, and on-screen/operational leakage vectors.
- Mitigation matrix with controls, residual risk, validation approach, and rollout priority.
- Evidence table linking each leakage class to source artifacts and detection method.

Acceptance:
- Leakage classes and mitigations are explicitly mapped and prioritized.
- No fixes during audit; this NA is analysis/matrix only.
- Findings are converted into follow-on NAs with bounded remediation scope.

Evidence:
- Implementation PR complete: #342 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/342), merge SHA `e459b90c0f0d634ff72875000c91633aa0fba7c8`.

### NA-0135 — Protocol roadmap decision: Ongoing PQ ratchet (SPQR/Triple Ratchet vs alternatives) (docs-only)

Status: DONE

Scope:
- docs-only decision artifact

Protect/Never-Happen Invariants:
- Never claim PQ-resilient ongoing FS/PCS without explicit mechanism and evidence.
- Never proceed to implementation without a recorded decision and trade-off table.
- Never mix threat-model assumptions with design facts without labeling uncertainty.

Deliverables:
- Decision document: what we have today vs what SPQR/Triple Ratchet provides.
- Explicit target properties: FS/PCS (classical + PQ-resilient), replay/downgrade expectations.
- Candidate design set: SPQR-like sparse PQ ratchet, periodic PQ rekey, and hybrid alternatives.
- Trade-offs table: bandwidth, latency, complexity, operational risk.
- Decision entry requirement in DECISIONS with follow-on implementation acceptance criteria.

Acceptance:
- Decision is explicit, evidence-backed, and cites NA-0133 findings.
- No fixes in this NA; findings/decision only.
- Follow-on NAs are created for chosen path and deferred alternatives.

Evidence:
- Implementation PR complete: #346 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/346), merge SHA `f6ac72d39280668d421da275f232494c8ed3ba72`.

### NA-0136 — Protocol implementation plan: Ongoing PQ ratchet MVP (design-to-tests plan; no code yet) (docs-only)

Status: DONE

Scope:
- docs-only implementation plan

Protect/Never-Happen Invariants:
- Never start protocol coding without a test-first plan for claimed security properties.
- Never allow ambiguous downgrade/version behavior in rollout planning.
- Never accept reject-path mutations in protocol state transitions.

Deliverables:
- Concrete protocol/state-machine plan, including message-format expectations where applicable.
- Test-first plan with vectors proving FS/PCS claims and no-mutation on reject.
- Rollout strategy: feature flag/versioning and downgrade-prevention approach.
- Explicit stop conditions for implementation phase.

Acceptance:
- Plan is specific enough to implement without guesswork.
- No fixes in this NA; plan-only output.
- Follow-on implementation NA(s) include bounded scope and fail-closed checks.

Evidence:
- Implementation PR complete: #349 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/349), merge SHA `5ea2f43ee550e9dc599af1823b6eba76c0b31c27`.

### NA-0137 — Metadata mitigations roadmap: padding/batching/fixed-interval modes (defaults + cost table) (docs-only)

Status: DONE

Scope:
- docs-only metadata mitigation roadmap

Protect/Never-Happen Invariants:
- Never enable metadata mitigations by default without quantified cost/UX impact.
- Never present mitigations without stating residual leakage.
- Never introduce mitigation claims that are not testable.

Deliverables:
- Prioritized mitigation list derived from NA-0134 findings.
- Default vs optional stance for each mitigation.
- Quantified cost table: bandwidth, latency, battery/CPU, complexity.
- Acceptance criteria for selecting one mitigation MVP.

Acceptance:
- Roadmap ties each mitigation to a specific leakage class and expected effect.
- No fixes in this NA; roadmap-only output.
- Follow-on implementation NA(s) are evidence-driven and bounded.

Evidence:
- Implementation PR complete: #355 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/355), merge SHA `507f7c0c078d291793ab2be590b9b79f1e260505`.

### NA-0138 — Metadata mitigation MVP: one optional mode (pick one: fixed-interval polling OR size bucketing) (client-only, test-backed)

Status: DONE

Scope:
- qsc client-only implementation

Protect/Never-Happen Invariants:
- Never regress baseline protocol truth semantics while adding metadata mitigation.
- Never mutate persisted state on reject/error paths introduced by mitigation logic.
- Never ship non-deterministic mitigation behavior without clear bounds.

Deliverables:
- Implement exactly one optional mitigation mode first: fixed-interval polling OR size bucketing.
- Deterministic behavior specification and markers (if applicable) aligned with existing conventions.
- Tests proving no leakage regressions beyond intended mitigation scope.
- Tests proving no-mutation on reject and baseline behavior parity when mitigation is disabled.
- Basic before/after performance impact measurements.

Acceptance:
- MVP is optional and bounded; defaults remain explicit and justified.
- Test evidence covers correctness, reject behavior, and performance delta.
- Follow-on NAs capture expansion/rollout decisions.

Evidence:
- Implementation PR complete: #358 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/358), merge SHA `7df70c7e44bc09f889de9537b2e15eb20a3f387e`.
- Implementation follow-up complete: #359 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/359), merge SHA `93438992be2491177437f0e977a0db4601c353fe`.

### NA-0139 — Hygiene hardcode: detached-worktree verification + prune sentinel in scripts (tooling-only)

Status: DONE

Scope:
- tooling-only (scripts/ci and governance hygiene automation)

Protect/Never-Happen Invariants:
- Never run verification in ambiguous/dirty worktrees.
- Never leave stale/prunable worktrees unreported in hygiene output.
- Never allow branch/worktree collisions to silently block governance flow.

Deliverables:
- Implement DD-28 in tooling:
  - detached worktree only for verification worktrees,
  - automatic `git worktree prune` reporting,
  - hygiene sentinel warnings in scripts/ci,
  - safeguards to prevent “main pinned to another worktree” recurrences.

Acceptance:
- Tooling emits deterministic hygiene diagnostics and clear fail-closed messages.
- No protocol/client behavior changes in this NA.
- Evidence demonstrates the recurrence class is explicitly guarded.

Evidence:
- Implementation PR complete: #352 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/352), merge SHA `065c72021c0ad71a1aab428bfc7cc34b94042fd5`.

### NA-0140 — Command output routing policy + Settings UX cleanup (client-only)

Status: DONE

Scope:
- qsc client-only

Protect/Never-Happen Invariants:
- Never route show-command output into an unrelated view (for example, Settings) by accident.
- Never steal focus as a side effect of command result routing.
- Never regress deterministic command behavior while cleaning Settings UX text/layout.

Deliverables:
- Define and implement deterministic command-result routing policy:
  - `/status` navigates to Status view (deterministic, no focus steal).
  - `/poll show` and `/autolock show` render in one consistent location (Status view or dedicated Command Results area).
  - Show-command output must not dump into Settings unless explicitly part of the chosen policy.
- Settings cleanup:
  - reduce debug-dump feel,
  - group Lock, Auto-lock, and Polling with clean spacing,
  - remove internal-only fields unless truly user-meaningful.
- Tests:
  - prove `/status` changes main view to Status,
  - prove `/poll show` and `/autolock show` output appear in the chosen consistent place,
  - prove Settings text is stable and excludes removed internal fields.

Acceptance:
- Routing policy is explicit, deterministic, and test-backed.
- Settings UX becomes cleaner without removing essential user controls.
- No focus-steal regressions introduced.

Evidence:
- Implementation PR complete: #363 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/363), merge SHA `00cebbee1c3fadf954614de1b37727a522b97c2b`.
- Implementation follow-up complete: #365 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/365), merge SHA `2233b3ba0649b512d7e0fa2482f64e729f1c8671`.

### NA-0141 — TUI Information Architecture redesign (System hub + Contacts/Messages subnav + Cmd Results + remove counters)

Status: DONE

Scope:
- qsc client-only (TUI navigation + rendering + command routing as needed)

Protect/Never-Happen Invariants:
- Never show per-item `(<n>)` counters in nav.
- Never allow more than one expanded nav domain at once.
- Never regress locked-first zero-leak or command non-wedge invariants while redesigning IA.

Deliverables:
- Implement nav IA with this required structure:
  - `System` (overview in main)
    - `Settings`
    - `Cmd Results`
  - `Contacts` (overview)
    - `<contact items>` (Alice, Bob, ...)
  - `Messages` (overview)
    - `<thread items>` (Alice, Bob, ...)
  - `Activity`
  - `Keys`
  - `Lock`
  - `Help`
  - `About`
  - `Legal`
- Navigation behavior:
  - only one expanded domain at a time,
  - Up/Down selection updates main immediately,
  - no nav counters; optional subtle dot `•` may be added later.
- Command routing behavior:
  - `/status` routes to System overview and focuses Nav on `System`,
  - `/poll show` and `/autolock show` route consistently to either System overview or Cmd Results.

Acceptance:
- Render tests prove nav hierarchy, no counters, focus behavior, and command-routing targets.
- No regressions to locked-first zero-leak and command non-wedge invariants.

Evidence:
- Implementation PR complete: #370 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/370), merge SHA `f6ce6d576db343f1b7ef12344966fb13f73b4a4c`.
- Follow-up PR complete: #371 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/371), merge SHA `5b18f69f0f6a5874f8b9e3f3088c82f715ca1479`.
- Follow-up PR complete: #372 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/372), merge SHA `a5e90244a0fa7da5a68201d28abaeda3674fc3c0`.
- Follow-up PR complete: #373 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/373), merge SHA `8f528b8410df6827caf636435e4ace3c462dc76b`.
- Follow-up PR complete: #374 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/374), merge SHA `927788a043251f1a98137ad80c49436253f2034e`.
- Follow-up PR complete: #375 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/375), merge SHA `5c7cc58d7013b207aec2642c9263d3faf5daca99`.

### NA-0142 — System→Account + Results + label cleanup + remove Submit/Cancel footers + /account destroy

Status: DONE

Scope:
- qsc client-only

Protect/Never-Happen Invariants:
- Never expose account/vault-sensitive details while locked; preserve locked-first zero-leak behavior.
- Never permit destructive vault actions without explicit passphrase + exact phrase confirmation.
- Never reintroduce wedge/relock regressions while routing command results and cleanup labels.

Deliverables:
- System subnav restructure:
  - `Account` (first)
  - `Settings`
  - `Results` (rename from `Cmd Results`)
- Account page (post-unlock only) shows:
  - alias (local-only),
  - verification code (4x4 Crockford + checksum) display-only,
  - vault state,
  - vault location hidden (`/vault where` optional),
  - storage safety status,
  - commands list: `/account passwd` (future), `/account destroy`, `/vault where` (optional).
- Remove `Submit: Enter | Cancel: Esc` footer lines across UX surfaces.
- UI label cleanup: drop underscores in user-facing labels (use spaces).
- Add dangerous `/account destroy` flow:
  - requires current passphrase (masked),
  - requires exact phrase `DESTROY MY VAULT`,
  - performs cryptographic erase + best-effort file removal,
  - post-condition returns locked shell with `No vault found — run /init`.

Acceptance:
- System nav includes `Account` and `Results` (exact label), with deterministic behavior.
- No `Submit: Enter | Cancel: Esc` line appears in rendered views.
- `/account destroy` requires passphrase + confirmation phrase and is fail-closed on mismatch.
- After destroy: vault absent, locked shell shows init-required prompt, and `/init` path works.

Evidence:
- Implementation PR complete: #379 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/379), merge SHA `2087b2091f687e9159877afe2b796c1ec49aa2a5`.
- Follow-up PR complete: #380 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/380), merge SHA `c23144fb0d271b42b8379a64bd9d7112d8ed41d1`.
- Follow-up PR complete: #381 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/381), merge SHA `a97d2d2ab6bf72e6981d997bac33916c0247c72d`.
- Follow-up PR complete: #382 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/382), merge SHA `ac28089cc80350fbc818b9bce77abb45e6fb863c`.

### NA-0143 — Main focus + scrolling (Tab cycles Nav→Main→Cmd)

Status: DONE

Scope:
- qsc client-only

Protect/Never-Happen Invariants:
- Never trigger vault reads, network operations, or extra polling due to focus changes or scrolling.
- Never mutate nav selection while Main is focused and scroll/navigation keys are used for main-content traversal.
- Never make Cmd unreachable while adding Main focus semantics.

Deliverables:
- Focus model updates:
  - `Tab` cycles `Nav -> Main -> Cmd -> Nav`.
  - `Shift+Tab` cycles backward when terminal/key handling supports it.
  - `Esc` returns focus to `Nav`.
- Main focus scrolling:
  - `Up/Down` scroll main content line-by-line.
  - `PgUp/PgDn` page scroll.
  - `Home/End` jump top/bottom.
- Invariants:
  - no vault reads, no network work, no extra polling due to scrolling,
  - nav selection remains unchanged while Main is focused,
  - Cmd remains reachable via `/` and via `Tab` cycling.
- Focus cue (no border colors):
  - add a tiny non-color Main-focus indicator (for example `•` in a corner).
- Tests:
  - deterministic `Tab` focus cycle behavior,
  - scrolling updates visible window only while Main focused,
  - nav selection unchanged while Main focused and scrolling.

Acceptance:
- Focus transitions are deterministic and test-backed.
- Main scrolling works with bounded behavior and no side effects on nav selection.
- No regressions to performance/safety invariants established under NA-0142.

Evidence:
- Implementation PR complete: #386 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/386), merge SHA `5d3416dc1c23435ea2a52b4abf1c1acfc214c75e`.
- Follow-up PR complete: #387 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/387), merge SHA `3a45ef142ab08180da7cf0ee30b091abbe69e339`.
- Follow-up PR complete: #389 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/389), merge SHA `d9f66f3ae4cd8f977c2b844a40b0fcc43f6469ee`.

### NA-0144 — Performance sensors & regression guards (deterministic, non-flaky)

Status: DONE

Scope:
- qsc client-only

Protect/Never-Happen Invariants:
- Never reintroduce vault/KDF work in the TUI render loop, nav movement, or idle ticks.
- Never add flaky performance checks that depend on wall-clock timing variance or sleeps.
- Never require protocol/server behavior changes to verify client-side performance invariants.

Deliverables:
- Add deterministic performance sensors for qsc client runtime hotspots (for example: KDF, vault reads/decrypts, render-trigger counters) suitable for headless tests.
- Add regression guards that fail when nav/focus/scroll/idle paths trigger forbidden heavy work.
- Add bounded mutation-path assertions to allow expected write-side work while preventing hidden background churn.
- Keep signals additive and test-oriented (no debug-noise regressions in user-facing TUI output).

Acceptance:
- Deterministic tests prove no forbidden heavy work occurs on nav/focus/scroll/idle paths.
- Mutation flows have explicit, bounded sensor deltas.
- Performance guard tests are non-flaky and pass consistently in CI.

Evidence:
- Implementation PR complete: #403 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/403), merge SHA `f98e22a3fe3ee520fa66b5878799db09a24137e1`.

### NA-0145 — Contacts UX Option 1 (overview table + contact card detail) + /contacts add + /verify

Status: DONE

Scope:
- qsc client-only

Protect/Never-Happen Invariants:
- Never show message previews in Contacts overview.
- Never leak contact/account details while locked.
- Never route command errors silently; mismatch/errors must be explicit and deterministic.
- Never store account-scoped contacts state outside encrypted vault storage.

Deliverables:
- Contacts UX Option 1
  - Contacts header/domain overview renders:
    - `You: <alias>`
    - table columns: `Alias | Trust | Blocked | Last seen` (last seen optional/coarse)
    - no message previews.
  - Contacts child (`Contacts -> <alias>`) renders a contact card with:
    - Trust section (state, last verify, mismatch indicator)
    - Identity section (verification code; fingerprint hidden behind explicit command if introduced later)
    - Policy section (blocked)
    - Notes (optional, local-only)
    - Commands list.
- Commands
  - `/contacts add <alias> <verification code>`
  - `/verify <alias> <verification code>`
  - `/contacts block <alias>`
  - `/contacts unblock <alias>`
  - optional later: rename/delete/notes.
- Behavior policy
  - Success: stay on current view, command bar shows `ok:`, Results updated.
  - Error: route to `System -> Results` and focus Nav.
  - Mismatch: logged as `err` and routed to Results.
- Storage
  - Contacts stored encrypted in vault (account-scoped).
  - Contacts data fully wiped by `/account destroy`.

Acceptance:
- Contacts overview renders table with expected headers.
- Contact detail renders required card sections.
- `/contacts add` creates entry; `/verify` updates trust state deterministically.
- Mismatch case routes to Results with deterministic `err` entry.
- Nav children remain alias-only.
- Locked mode shows no contact leakage.

Evidence:
- Implementation PR complete: #393 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/393), merge SHA `6c6829f6846b3ec59e20bbeb82868bbf06078f15`.

### NA-0146 — TUI chrome redesign: single-panel layout (one outer border; internal dividers; same IA/behavior)

Status: DONE

Scope:
- qsc client-only, layout/render only

Protect/Never-Happen Invariants:
- Never change IA/behavior while implementing chrome-only refactor.
- Never regress locked-first behavior or command-routing semantics.
- Never reintroduce timers/animations.
- Never reintroduce perf regressions (no vault/KDF work on nav/idle paths).

Deliverables:
- Stage 1 (chrome-only refactor)
  - Replace current 3-panel boxed chrome with a single outer border.
  - Add internal dividers:
    - vertical divider between nav column and main area
    - horizontal divider above cmd line
  - Preserve all current behavior and IA:
    - System/Contacts/Messages hierarchy
    - Results routing & command policy
    - Locked-first behavior
    - Tab focus cycling + Main scrolling
    - Perf invariants (no vault/KDF on nav/idle)
  - No timers/animations.
  - Update render tests to match new chrome without changing semantics.
- Stage 2 (optional polish, chrome-only)
  - spacing/indent alignment refinements that do not change behavior.

Acceptance:
- Single-panel chrome renders with required internal dividers.
- Existing IA/behavior remains unchanged and test-backed.
- Render tests updated for chrome-only diffs; semantics preserved.
- No timers/animations introduced.

Evidence:
- Implementation PR complete: #397 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/397), merge SHA `29cdaa66b041b909fa09338bef8587483ae1280a`.
- Implementation PR complete: #398 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/398), merge SHA `0f9108fe3b04aa86a75101a4dfb6fc5240b276e8`.
- Implementation PR complete: #399 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/399), merge SHA `5b04702b41056d85481cfdd331e95eb38a0ed3ed`.
- Implementation PR complete: #400 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/400), merge SHA `74a1fe87fdb379119c55874743c1cf8ecbe213a5`.

### NA-0147 — UX Wave: Contacts table alignment + Note to Self + Messages thread filtering + "You:" cleanup + cmd-bar focus label

Status: DONE

Scope:
- qsc client-only

Protect/Never-Happen Invariants:
- Never regress locked-first zero-leak behavior while introducing UX refinements.
- Never couple focus-label UX to timers, blinking, or color-only cues.
- Never reintroduce semantic drift in command routing or thread/contact boundaries.

Deliverables:
- Contacts overview table alignment (Option A):
  - fixed-width columns with no pipes,
  - header and rows rendered via the same formatter helper,
  - `Alias` width 12 (truncate with `…`),
  - `Trust` width 11,
  - `Blocked` width 7,
  - `Last seen` uses remaining width.
- Add pinned Messages thread labeled exactly `Note to Self`:
  - always present even with zero messages,
  - treated as a pinned local/self thread, not a peer contact.
- Messages subnav shows threads only:
  - no contacts with zero message/file history,
  - thread appears on first message/file event,
  - exception: `Note to Self` always present.
- `You:` copy cleanup:
  - show `You: <alias>` only once in the UI (preferred: Contacts overview OR System -> Account),
  - remove `You: <alias>` from individual contact detail view.
- Focus indicator in cmd bar (no colors, no timers):
  - explicitly render `Focus: NAV|MAIN|CMD`,
  - update deterministically on Tab/Shift+Tab/Esc and in locked mode,
  - no border color changes or blinking.

Acceptance:
- Contacts overview table alignment is visually correct in monospace using fixed-width formatter output.
- `Note to Self` is always present under Messages.
- Messages subnav filters out non-thread contacts with no history (except `Note to Self`).
- `You:` copy rule enforced exactly once and absent from contact detail view.
- Focus label updates deterministically with focus transitions.
- Tests cover table alignment, Note-to-Self presence, messages thread filtering, You-copy rule, and focus-label determinism.

Evidence:
- Implementation PR complete: #407 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/407), merge SHA `c136abe4444c54001ccb7951cc8baeb1d6cae5ce`.
- Follow-up implementation PR complete: #408 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/408), merge SHA `a91aa5ac00fd24521fa1a20eeda488a16de91b68`.

### NA-0148 — System -> Relay/Server config (vault-backed, redacted) + remote testing ergonomics (client-only)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`
- `TRACEABILITY.md` (+ `DECISIONS.md` only if goal-lint requires)

Must protect:
- relay URL/token/mailbox identifiers (secrets) and prevent leakage.

Invariants:
- Never display relay token plaintext in UI/logs/markers.
- Locked: relay commands reject deterministically, no state mutation.
- Vault-only persistence: relay config stored encrypted in vault.

Deliverables:
- Nav: System -> Relay (or `Server`) page showing:
  - relay url (ok to show)
  - auth token: set/unset (never show value)
  - mailbox/device label: redacted/minimal
- Commands:
  - `/relay show`
  - `/relay set url <https://...>`
  - `/relay set token <token>` (store only; never echo)
  - `/relay clear token`
- Deterministic tests:
  - token never appears in rendered output or result strings
  - set/show/clear behaviors
  - locked reject no-mutation

Acceptance:
- Can configure relay for real-world tests safely without leaking secrets.

Evidence:
- Implementation PR complete: #412 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/412), merge SHA `e60e65f8ddec140b7d8a4150c3964b2ed7e3b9e3`.

### NA-0149 — Messages UX MVP (send/compose/thread view/scroll; deterministic) (client-only)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`

Must protect:
- no secrets or internal markers leaked into UI.

Invariants:
- commands never wedge UI; failures route to Results.

Deliverables:
- Thread view: clear transcript rendering, scrollable.
- Compose/send:
  - `/msg "<text>"` sends to selected thread
  - Note to Self supported
- Deterministic tests:
  - send appends outgoing message deterministically
  - thread appears only after history exists (except Note to Self)
  - scroll changes visible content in small viewport

Acceptance:
- Usable messaging in terminal against configured relay.

Evidence:
- Implementation PR complete: #415 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/415), merge SHA `3e2d40b67bdcdb0020a458b9ade679d9fab4222d`.

### NA-0150 — Messages UX polish (draft editor/actions/selection caret; optional) (client-only)

Status: BACKLOG

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`

Deliverables:
- Optional main draft editor.
- Optional selection caret + minimal actions (reply/quote), no timers.
- Tests for any new behavior.

### NA-0151 — Remote relay transport hardening: HTTPS-only off-loopback + deterministic rejects (client-only)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`

Must protect:
- confidentiality/integrity against on-path attackers for any non-local relay usage; token secrecy.

Invariants (never happen):
- Never permit plaintext `http://` relay endpoints for non-loopback hosts.
- Never silently downgrade `https://` -> `http://`.
- Reject invalid/unsafe relay URLs deterministically with NO persistence mutation.
- Relay auth token must never be printed (UI/log/markers).

Deliverables:
- Implement a strict Relay URL policy:
  - Allow `http://` ONLY for loopback (`localhost`, `127.0.0.1`, `::1`) for dev/demo.
  - Require `https://` for all other hosts.
- Ensure TLS verification remains default (no `danger_accept_invalid_certs`).
- Add deterministic error code(s) surfaced in CLI/TUI (e.g., `QSC_ERR_RELAY_TLS_REQUIRED`).
- Tests:
  - Unit tests for loopback vs non-loopback allow/deny.
  - Integration test proving reject paths do not mutate vault-stored relay config.

Acceptance:
- Policy enforced in all code paths that contact a relay.
- Tests deterministic and green in CI.

Evidence:
- PR #418 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/418) merged (merge SHA `3ed3ff27fa92303cabd16a1295ed85475db3fb37`).

### NA-0152 — Relay metadata minimization: route-token addressing + label removal in transport (client + local relay)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`, `apps/qshield-cli/**` (only if needed)

Must protect:
- contact graph + label correlation via URLs/paths.

Invariants:
- Network-visible identifiers MUST be opaque tokens, not human labels.
- Tokens must be vault-backed and never printed.

Deliverables:
- Define route-token scheme (generation, rotation, storage).
- Replace any label-derived mailbox/channel naming with route tokens.
- Add tests that scan constructed URLs/paths and assert absence of labels.

Acceptance:
- Remote relay requests contain no human labels; only opaque tokens.

Evidence:
- PR #420 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/420) merged (merge SHA `ff5ce1e11b397c4275f843efa510f19c4716de92`).

### NA-0153 — QSE envelope length confidentiality: ensure padding/buckets are not defeated by cleartext length fields (protocol + tooling)

Status: DONE

Scope:
- `tools/refimpl/quantumshield_refimpl/src/qse/**`, `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`, `docs/canonical/**` (QSE), `inputs/**` (if vectors added)

Must protect:
- payload length leakage beyond declared bucket/profile.

Invariants:
- Observers must not learn exact plaintext length when a bucketed padding mode is selected.

Deliverables:
- Audit QSE header fields for length leakage.
- If leakage exists: redesign so only bucket ID (or encrypted length) is visible; update spec + refimpl + CI vectors.
- Update QSC QSE bucket-mode decode to recover payload boundary without relying on cleartext payload_len/pad_len.

Acceptance:
- CI-gated test proves envelope header does not reveal exact payload length under bucketed padding.
- qsc file_transfer_mvp and QSE-related tests pass in CI.

Evidence:
- PR #422 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/422) merged (merge SHA `c09645298e0f2de37e60a57b90d7144a729576c2`, Decision D-0260).

### NA-0154 — Handshake security closure: verify FS/identity binding properties are evidenced; add CI proofs; fix gaps (Suite-2 + QSC)

Status: DONE

Scope:
- `tools/refimpl/quantumshield_refimpl/src/suite2/**`, `qsl/qsl-client/qsc/**`, `inputs/suite2/vectors/**` (if needed)

Must protect:
- forward secrecy claims, identity binding, downgrade resistance.

Deliverables:
- Validate current establish flow against FS/identity-binding expectations from canonical docs/tests.
- Add vectors/tests that would fail if identity binding or transcript checks are bypassed.
- If any gap is found: implement smallest fail-closed fix + vectors.

Acceptance:
- Evidence exists (tests/vectors) for the claimed handshake properties; no “paper-only” security.

Evidence:
- PR #425 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/425) merged (merge SHA `928fc3826034a60b4e9edf5bd947a3a0d423567a`, Decision D-0261).

### NA-0155 — Ratchet durability + nonce reuse regression: crash/retry/send-commit atomicity proofs (client + suite2)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**`, `tools/refimpl/quantumshield_refimpl/src/suite2/**`, relevant tests/vectors

Must protect:
- nonce reuse, rollback-induced state divergence.

Deliverables:
- Add deterministic crash/retry tests covering send prepare->send->commit boundaries.
- Prove no nonce reuse and no state-advance on failure.

Acceptance:
- CI fails if nonce reuse/rollback regression reappears.

Evidence:
- PR #427 merged at 2026-02-22T17:10:28Z; merge SHA a3d5615d629afdd3cdeb0cca2526489d4750f93e; https://github.com/QuantumShieldLabs/qsl-protocol/pull/427

### NA-0156 — Bounded receive work: header decrypt attempt caps + reject normalization (DoS/oracle resistance)

Status: DONE

Scope:
- `tools/refimpl/quantumshield_refimpl/src/suite2/**`, `qsl/qsl-client/qsc/**`

Must protect:
- CPU/DoS resistance and side-channel/oracle minimization.

Deliverables:
- Enforce bounded header-decrypt attempts.
- Normalize rejects where appropriate (fail-closed, deterministic).
- Tests asserting bound is honored.

Acceptance:
- Worst-case work is bounded and test-proven.

Evidence:
- PR #429 merged at 2026-02-22T22:57:33Z; merge SHA 63d245ebaf593f7eda34a3c642bd0377eabb4d71; https://github.com/QuantumShieldLabs/qsl-protocol/pull/429

### NA-0157 — Vault hardening follow-ons: attempt-limit option, KDF parameter review, memory lifetime/zeroization discipline (client-only)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/**`

Deliverables:
- Add opt-in attempt-limit/wipe-on-failure with strong warnings + tests.
- Review KDF params (time/memory) and record decision when changed.
- Reduce decrypted key material lifetime where feasible; add tests where possible.

Acceptance:
- Security hardening is test-backed and deterministic.

Evidence:
- PR #431 merged at 2026-02-23T00:12:58Z; merge SHA c1f8c8dd116bf87fd29ee20a688e75fc725a033f; https://github.com/QuantumShieldLabs/qsl-protocol/pull/431

### NA-0158 — Modularize qsc/src/main.rs for auditability (no behavior change)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**`

Invariants:
- No behavior change; tests must remain green.

Deliverables:
- Split into modules (`tui/`, `cmd/`, `vault/`, `relay/`, `model/`, `store/`).
- No refactors beyond file moves + minimal wiring.

Acceptance:
- Name-only diff shows only qsc paths; tests green.

Evidence:
- PR #433 merged at 2026-02-23T01:26:27Z; merge SHA a0bfe8f18b0e1cb25f5bbb7ec899706d5314983f; https://github.com/QuantumShieldLabs/qsl-protocol/pull/433

### NA-0159 — Legal/compliance: short /init acceptance + System->Legal + System->About polish (client-only)

Status: DONE

Scope:
- `qsl/qsl-client/qsc/src/**`, `qsl/qsl-client/qsc/tests/**` (and docs if needed)

Invariants:
- No secrets; no over-claiming (“metadata eliminated” etc).

Deliverables:
- Add short-form legal acceptance in /init flow (with full Legal page text).
- About page includes proof links to governance/docs/tests.

Acceptance:
- Deterministic rendering/tests; no token leakage.

Evidence:
- PR #435 merged at 2026-02-23T02:39:08Z; merge SHA 05353d756e94fe20dc4902bae45afb26b03375da; https://github.com/QuantumShieldLabs/qsl-protocol/pull/435

### NA-0160 — Cross-platform confidence: add macOS CI build lane for qsc + qshield (no behavior claims)

Status: DONE

Scope:
- `.github/workflows/**`, plus any minimal build fixes if REQUIRED (else separate NA)

Deliverables:
- CI job that builds/tests on macOS for qsc (and qshield-cli if feasible).

Acceptance:
- CI lane enforced; green.

Evidence:
- PR #437 merged at 2026-02-23T03:45:29Z; merge SHA b4cab022227038c80b68eb93472f3f8f90c91f44; https://github.com/QuantumShieldLabs/qsl-protocol/pull/437

### NA-0161 — Cross-repo security review: qsl-server transport/auth/metadata alignment (BLOCKED: requires qsl-server repo access)

Status: BLOCKED

Prereq:
- Confirm qsl-server repo access + intended integration scope.

Deliverables (once unblocked):
- Server-side review: TLS termination posture, token auth semantics, logging/retention, request size caps, route-token support.

Evidence:
- TBD

### NA-0162 — Relay/server hardening: bind-to-loopback default + capability-safe logging (relay-only)

Status: DONE

Scope:
- `/home/victor/work/qsl/qsl-server/**` (qsl-server repo)
- `qsl/qsl-client/qsc/**` only if needed for compatibility/tests (prefer none)

Must protect:
- route tokens / channel identifiers (capability-like), contact-graph correlation, accidental public exposure of relay port.

Invariants:
- Relay MUST NOT log raw channel identifiers/tokens (only hash-prefix if needed).
- Relay MUST bind to loopback by default; public bind requires explicit opt-in.
- Reject paths are deterministic and do not enqueue/mutate state.

Deliverables:
- Add bind address configuration (env/config) with safe default (127.0.0.1).
- Sanitize push/pull logs to avoid printing channel tokens.
- Add CI-gated tests proving:
  - default bind is loopback-only
  - logs do not contain raw channel values

Acceptance:
- Default deployment is not publicly reachable on 8080.
- No capability leakage via relay logs; tests enforce.

Evidence:
- PR #18 (https://github.com/QuantumShieldLabs/qsl-server/pull/18) merged at 2026-02-24T03:24:13Z; merge SHA 4a40d3881d982ff7b62cdd480d460d9675a24c80.

### NA-0163 — Relay deployment smoothness: production runbook + Caddy log hygiene + token rotation checklist (server ops + packaging)

Status: DONE

Scope:
- qsl-server repo: packaging/**, scripts/**, README.md (docs/runbook)
- (No qsl-protocol code changes unless explicitly required later)

Must protect:
- capability-like route tokens/channels, relay bearer token, accidental public exposure of relay port.

Invariants:
- No secrets in repo artifacts (examples use placeholders only).
- Production reverse proxy MUST NOT log /v1/* request URIs (avoid leaking tokens in paths).
- Deployment steps are deterministic and verifiable (auditable checks).

Deliverables:
- Add a production-ready runbook (README section or packaging/runbook.md) describing:
  - install (systemd + env + perms)
  - update (binary swap / script)
  - rollback (if supported)
  - token rotation steps (server + client)
  - firewall/Security Group guidance: only 443/80 public; 8080 closed
- Provide a Caddy example that disables/sanitizes access logs for /v1/* while keeping safe logs elsewhere.
- Add an audit script (no secrets) that prints:
  - bind address/port (must show loopback by default)
  - service status
  - env file perms
  - caddy active config path + whether /v1 logging is disabled
- CI-gated check that example env files contain no token-like values.

Acceptance:
- A fresh Ubuntu instance can be installed/updated with copy-paste steps + scripts.
- No route-token leakage via proxy logs, and CI prevents regression.

Evidence:
- PR #19 (https://github.com/QuantumShieldLabs/qsl-server/pull/19) merged at 2026-02-24T04:09:09Z; merge SHA a69a942c7a9fd333001083657ab571ecf157981d.

### NA-0164 — Relay release/update reliability: versioned artifacts + checksum-verified update path (server-only)

Status: DONE

Scope:
- QuantumShieldLabs/qsl-server:
  - .github/workflows/**
  - scripts/**
  - packaging/**
  - README.md (if needed)

Must protect:
- supply-chain integrity for relay updates; prevent “silent” or tampered upgrades.

Invariants:
- Update path MUST verify checksums before installing a new binary.
- No secrets committed; examples remain placeholder-only.
- Failure must be deterministic and must not leave partial installs.

Deliverables:
- Add a CI workflow that builds Linux release artifacts for qsl-server and emits SHA256 checksums.
- Extend update_ubuntu.sh (or add update_from_release.sh) to:
  - accept a version/tag or artifact URL
  - download artifact + checksum
  - verify checksum (fail closed on mismatch)
  - install atomically (tmp + move/symlink swap)
- Add a deterministic test or script-check in CI that proves checksum verification is enforced.

Acceptance:
- A user can upgrade relay with a single command and the script refuses tampered/mismatched artifacts.

Evidence:
- qsl-server PR #20 (https://github.com/QuantumShieldLabs/qsl-server/pull/20) merged at 2026-02-25T01:57:01Z; merge SHA 4610c8202f3800eedeb55ed896b096029322a071.
- Release tag v0.0.2 published: https://github.com/QuantumShieldLabs/qsl-server/releases/tag/v0.0.2

### NA-0165 — Remote relay real-world stability (100-client): integration soak harness + failure-mode playbook (client+server)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** (tests and tooling permitted)
- QuantumShieldLabs/qsl-server (scripts/**, packaging/**; server code only if required later)
- docs/runbooks (if needed, minimal)

Must protect:
- end-to-end reliability under remote relay conditions (latency, drops, retries) without leaking capabilities.

Invariants:
- No raw route tokens / channel identifiers in logs (proxy or server).
- Failures are deterministic and recoverable (no state corruption; retries do not cause nonce reuse).
- Load is bounded: relay must not OOM; rejects must be explicit under backpressure.

Deliverables:
- A reproducible “100-client soak” harness (script/tool) that can run against an HTTPS relay and reports:
  - success rate, latency summary, rejection counts by code, and any client-side crypto errors.
- A failure-mode playbook: what to check when pushes/pulls stall (audit script, logs, queue depth, token mismatch, TLS issues).
- A minimal CI smoke test (small N) that ensures the harness wiring doesn’t rot.

Acceptance:
- Operator can run one command to execute the soak test against AWS and get a clear PASS/FAIL report and diagnostics.

Evidence:
- PR #444 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/444) merged at 2026-02-25T13:36:43Z; merge SHA b7357b31e3765a3b0ccf49f0b65c59fae3994b41.
- Key artifacts: `qsl/qsl-client/qsc/scripts/remote_soak.py`, `qsl/qsl-client/qsc/REMOTE_SOAK_PLAYBOOK.md`, `qsl/qsl-client/qsc/tests/remote_soak_smoke_na0165.rs`.

### NA-0166 — Relay update UX: fix release checksum filename field; ensure update_from_release.sh --release works; cut v0.0.3 (server-only)

Status: DONE

Scope:
- QuantumShieldLabs/qsl-server (.github/workflows/**, scripts/**) + governance updates

Must protect:
- checksum-verified updates remain fail-closed; atomic installs; no secrets

Deliverables:
- release workflow emits checksum referencing bare filename (no dist/ prefix); CI test proves updater refuses tampering; publish v0.0.3

Acceptance:
- one-command upgrade works without manual checksum rewrite

Evidence:
- qsl-server PR #22 (https://github.com/QuantumShieldLabs/qsl-server/pull/22) merged at 2026-02-26T01:43:00Z; merge SHA 720b2db92e45778deb6f18fa4c6273bb54652312.
- Release tag v0.0.3 published: https://github.com/QuantumShieldLabs/qsl-server/releases/tag/v0.0.3
- Artifacts: `scripts/update_from_release.sh`, `.github/workflows/release-linux.yml`, `scripts/ci/test_update_checksum.sh`.

### NA-0167 — Relay 100-client capacity baseline: run remote_soak.py against AWS + backpressure observability (client+server)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** (tests and tooling permitted)
- QuantumShieldLabs/qsl-server (scripts/**, packaging/**; server code only if required later)
- docs/runbooks (if needed, minimal)

Must protect:
- end-to-end reliability under remote relay conditions (latency, drops, retries) without leaking capabilities.

Invariants:
- No raw route tokens / channel identifiers in logs (proxy or server).
- Failures are deterministic and recoverable (no state corruption; retries do not cause nonce reuse).
- Load is bounded: relay must not OOM; rejects must be explicit under backpressure.

Deliverables:
- A reproducible 100-client soak run against AWS relay with summarized outcomes:
  - success rate, latency summary, rejection counts by code, and any client-side crypto errors.
- Backpressure observability evidence:
  - queue-depth/reject visibility and operator checks for saturation handling.
- A concise operator runbook update for baseline capacity execution and diagnostics.

Acceptance:
- Operator can run one command to execute the baseline soak against AWS and get a clear PASS/FAIL report plus diagnostics.

Evidence:
- PR #447 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/447) merged at 2026-02-26T03:45:53Z; merge SHA 8caa283f0eb184dfc5a7195e40f57c43c2c82dda.
- Baseline artifact: `qsl/qsl-client/qsc/REMOTE_SOAK_BASELINE_AWS_2026-02-26.md`.

### NA-0168 — Relay 100-client stabilization: explicit backpressure codes + queue depth observability + retry tuning (client+server)

Status: DONE

Scope:
- qsl/qsl-client/qsc/**
- QuantumShieldLabs/qsl-server (server + ops scripts as needed)

Must protect:
- deterministic overload handling without capability leakage under sustained relay pressure.

Invariants:
- Backpressure rejects use explicit, stable codes surfaced in client and server diagnostics.
- Queue depth/pressure observability is available to operators without exposing route tokens/channels.
- Retry behavior remains nonce-safe and bounded (no replay amplification, no state corruption).

Deliverables:
- Define and enforce explicit backpressure/reject codes end-to-end.
- Add queue-depth/pressure observability hooks and operator-facing checks.
- Tune retry behavior for 100-client remote stability with deterministic reject handling.

Acceptance:
- Under controlled 100-client load, overload scenarios produce explicit codes and actionable diagnostics; no token leakage in logs.

Evidence:
- PR #450 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/450) merged at 2026-02-28T14:03:24Z; merge SHA 34972db07f546e87340d733f8298c3f522e7edf1.
- Validation evidence: `suite2-vectors` green on PR #450 and AWS relay soak progression completed (2-client diag PASS, 10-client warm-up PASS, 100-client baseline PASS) with leak scans `/v1/=0` and `hex>=32=0`.

### NA-0169 — CI determinism + flake elimination: stabilize macOS lane failures in ratchet/handshake tests (client-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/**
- qsl/qsl-client/qsc/tests/**
- (CI wiring only if needed) .github/workflows/** for deterministic qsc test execution on macOS

Must protect:
- deterministic, reproducible CI signal for qsc ratchet/handshake behavior without reducing security coverage.

Invariants:
- Required macOS lane must fail only on real regressions, not nondeterministic harness/state collisions.
- No test skip/disable as a primary fix; preserve security coverage for ratchet/handshake paths.
- Marker outputs remain deterministic and secret-safe.

Deliverables:
- Isolate and remove macOS-specific flake sources in qsc ratchet/handshake tests.
- Add deterministic regression guard(s) proving fixed behavior under CI conditions.
- Ensure required macOS lane is stable across repeated reruns.

Acceptance:
- macOS required lane passes consistently on repeated CI reruns with unchanged code and no skipped security tests.

Evidence:
- PR #452 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/452) merged at 2026-02-28T22:32:37Z; merge SHA a84e720728ba4d76d246dd3dad7b4e15e23adc66.
- Determinism proof: `macos-qsc-qshield-build` passed three consecutive reruns on the same SHA 83878719ead0ddaabd281fe9a6ff7be8d2beef41 (job URLs recorded in PR #452 evidence).

### NA-0170 — Linux operator UX: qsc TUI MVP + safe config handling (client-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/** (implementation)
- qsl/qsl-client/qsc/tests/** (minimal smoke/regression)
- docs/runbooks/** (minimal, if needed)

Must protect:
- No secret leakage (no tokens, no /v1/<token> URIs, no auth headers).
- Protocol behavior unchanged (UI/wrapper only unless explicitly required and separately approved).
- Cross-platform compile: Linux primary, but do not break macOS/CI builds.

Deliverables:
- Add a Linux-first `qsc tui` command that provides:
  - relay URL + token-file configuration (token never echoed; permission checks enforced)
  - contact list + add contact + handshake flow
  - send/receive message flow
  - file send/receive basic progress UI
- Config persistence:
  - default config path with strict perms (0600) or refuse with deterministic marker.
- Minimal smoke test in qsc tests ensuring tui module/command wiring compiles and runs a non-network selftest mode.

Acceptance:
- On Linux, an operator can run `qsc tui ...` and complete handshake + message + file transfer against a relay with no secret leakage.
- CI remains green; no skip/ignore tactics.

Evidence:
- qsl-server PR #24 (https://github.com/QuantumShieldLabs/qsl-server/pull/24) merged at 2026-03-01T00:01:22Z; merge SHA eafb7880eb86c38d0f52f5301ee991724a891f98.
- AWS dev relay wrapper validation via `ssh qsl` returned `QSL_AWS_UPDATE_RESULT PASS code=ok` with sanitized leak counts `/v1/=0` and `hex>=32=0`.

### NA-0171 — Remaining macOS CI flake elimination: contacts_verify_block pinned_mismatch_refuses_no_mutation / relay_inbox_push_failed (client-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/tests/** only
- qsl/qsl-client/qsc/tests/common/** only if required for deterministic test harness isolation

Must protect:
- No skip/ignore as the primary fix.
- Preserve handshake/contacts security coverage.
- Deterministic marker outputs remain secret-safe (no tokens, no `/v1/<token>` URIs).

Invariants:
- macOS lane failures must reflect real regressions, not shared-state/test-order collisions.
- Flake fix must not lower test coverage.

Deliverables:
- Isolate failing contacts verify paths from relay inbox/test-state collisions.
- Add deterministic harness/test guard(s) proving pinned mismatch path no longer flakes.
- Document precise flake signature eliminated (`pinned_mismatch_refuses_no_mutation` / `relay_inbox_push_failed`).

Acceptance:
- `macos-qsc-qshield-build` passes 3 consecutive reruns on the same SHA with no code changes between reruns.
- All other required checks remain green.

Evidence:
- qsl-protocol PR #455 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/455) merged at 2026-03-01T04:07:23Z; merge SHA `2366c8d20c68f5039cab44b61486ee59042d6617`.
- `macos-qsc-qshield-build` passed 3 consecutive reruns on the same SHA `a487b42f46ab8f537059cbadb7edcc63e6b381ca` (see CI links in directive evidence / PR discussion).

### NA-0172 — Process guardrails: queue successor requirement + CI dependency policy (docs-only)
Status: DONE
Scope:
- qsl-protocol/AGENTS.md (process rules)
- qsl-server/scripts/ci/** docs/README if present (dependency policy note)
- workspace CODEX_RUNBOOK.md (template guardrail wording only)
Must protect:
- Exactly-one-READY queue discipline; no “terminal READY” dead-ends.
- CI determinism; no adding tool dependencies in CI scripts without installing them.
Invariants:
- No skip/ignore as primary fix.
- Governance PRs stay minimal.
Deliverables:
- Add a queue-succession guardrail to directive/governance templates (“if READY is terminal, append an approved next NA in same directive or STOP”).
- Add a short CI dependency policy note for scripts/ci (“POSIX shell + coreutils + grep/awk only unless workflow installs extras”).
Acceptance:
- Governance closes without getting stuck due to missing successor NA.
- CI scripts do not fail due to missing nonstandard tools.
Evidence:
- qsl-protocol PR #458 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/458), merge SHA `0d44348f26ce7eaf53a7b69d4758645d9f1dfea4`.
- qsl-server PR #25 (https://github.com/QuantumShieldLabs/qsl-server/pull/25), merge SHA `2c96793bb1e2897fcd2c1a5632376350df60de2a`.
- workspace `CODEX_RUNBOOK.md` updated locally (non-git).

### NA-0173 — Test-harness transport stability: harden local mock relay (start_inbox_server) for macOS determinism (tests-only)
Status: DONE
Scope:
- qsl/qsl-client/qsc/tests/**
Must protect:
- No reduced security coverage; no skip/ignore as a primary fix.
Invariants:
- Any macOS flake-oriented PR must show 3 consecutive macOS passes on the same SHA before merge.
Deliverables:
- Identify and fix transport-facing flake sources in the local mock relay used by tests (start_inbox_server / handle_conn / request parsing / connection lifecycle), without touching product code.
- Add at least one deterministic regression guard (tests-only) that would have caught the relay_inbox_push_failed / relay_inbox_bad_request class.
- Keep secret-safe deterministic marker outputs (no tokens/URIs).
Acceptance:
- PR green; macos-qsc-qshield-build passes 3 consecutive reruns on same SHA.
Evidence:
- qsl-protocol PR #460 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/460), merge SHA `9fd9ae65c8608f80a0c4a471dc52740666cb10e7`.
- macOS 3 consecutive passes on the same SHA `a270b9001fbd7b20393fad7b26048f9980a58fa5`:
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22548687121/job/65315141911
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22548687121/job/65315575180
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22548687121/job/65316192580
- Key artifacts:
  - `qsl/qsl-client/qsc/tests/common/mod.rs`
  - `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
  - `qsl/qsl-client/qsc/tests/outbox_abort.rs`

### NA-0174 — Mock relay regression guards: Content-Length conflict + truncated body handling (tests-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/tests/**

Must protect:
- deterministic CI signal (macOS/Linux) without weakening protocol/security assertions.
- no skip/ignore as primary mechanism.

Invariants:
- mock relay must not hang CI on malformed/partial input; bounded timeouts required.
- transport-facing failures must be classified deterministically (no generic flake loops).

Deliverables:
- Add deterministic regression test(s) that exercise:
  1) conflicting Content-Length headers → deterministic 400 (no hang)
  2) truncated body (Content-Length > bytes sent) → deterministic failure within bounded deadline (no hang)
- Add a short comment block documenting mock relay contract:
  - single-request-per-connection (or explicitly safe multi-request behavior)
  - timeout behavior
  - readiness predicate semantics

Acceptance:
- macos-qsc-qshield-build passes 3 consecutive times on same SHA for the implementation PR (same proof pattern as NA-0171/0173).
- No reduced security coverage; no skip/ignore.

Evidence:
- qsl-protocol PR #462 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/462), merge SHA `434c243fc14f747aa0cb3aeee9b4a2ede7dbe2e4`, merged at 2026-03-02T00:59:08Z.
- macOS 3 consecutive passes on the same SHA `00201f3e2e5f6c11c7bd111b5abfe659acabcf31`:
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22556351027/job/65334321571
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22556351027/job/65335076388
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22556351027/job/65335910300
- Key artifacts:
  - `qsl/qsl-client/qsc/tests/common/mod.rs`
  - `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`

### NA-0175 — Mock relay transport contract: additional deterministic negative cases (tests-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/tests/** only

Must protect:
- Deterministic CI signal across macOS/Linux for transport-facing integration tests.
- No hangs: all mock relay paths must be bounded by deadlines/timeouts.
- No coverage reduction: no skip/ignore, no assertion weakening.

Deliverables:
1) Add deterministic regression test: reject Transfer-Encoding: chunked on /v1/push (HTTP 400 preferred), and prove not enqueued (pull 204).
2) Add deterministic regression test: second request on same TCP connection is handled per contract (either rejected deterministically or closed),
   with bounded completion and no hang.
3) Keep all tests raw-socket style (TcpStream), with explicit timeouts and deterministic response parsing.

Acceptance:
- All required checks pass.
- macos-qsc-qshield-build achieves 3 consecutive PASS on the same SHA for the implementation PR.

Evidence:
- qsl-protocol PR #464 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/464), merge SHA `4c52ad65f652f8efbb0f739c1ca339c6a2116d6d`, mergedAt `2026-03-02T03:05:06Z`.
- macOS 3 consecutive passes on the same SHA `3ca3eb6cf70c7fbba73f9884d4e03f574f7bfd34`:
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22558864532/job/65341270301
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22558864532/job/65341852322
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22558864532/job/65342640667
- Key artifacts:
  - `qsl/qsl-client/qsc/tests/common/mod.rs`
  - `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`

### NA-0176 — Mock relay transport: Transfer-Encoding normalization + combined-header edge cases (tests-only)

Status: DONE

Scope:

- qsl/qsl-client/qsc/tests/**
  - Specifically mock relay transport tests + shared harness helpers, if needed.

Invariants:

- Tests-only changes. No qsc/src/**, no refimpl, no workflows.
- No skip/ignore-based “fixes”; preserve negative-case security assertions.
- Deterministic raw-socket tests with explicit timeouts; no sleep-as-solution.

Deliverables:

1) Add deterministic regression tests for:
   - Transfer-Encoding header case/whitespace normalization (e.g., "tRaNsFeR-EnCoDiNg:  chunked").
   - Transfer-Encoding list values that include "chunked" (e.g., "gzip, chunked") rejected deterministically.
   - Transfer-Encoding: chunked combined with Content-Length present rejected deterministically.
2) Ensure all tests prove "not enqueued" via /v1/pull 204 after rejection.
3) macOS stability acceptance: 3 consecutive macos-qsc-qshield-build PASS on the same SHA for the implementation PR.

Acceptance:

- Required CI lanes pass (including suite2-vectors).
- macOS 3-pass same-SHA proof captured.
- Strict scope: tests-only for implementation PR.

Evidence:

- qsl-protocol PR #466 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/466), merge SHA `5fcaedf6980715556755b0a7ced0974c9c689f94`, mergedAt `2026-03-02T13:28:42Z`.
- macOS 3 consecutive passes on same SHA `d9028181beb8dacd3cbfd0f53cdeb45d89dc90bf`:
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22576878139/job/65398329931
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22576878139/job/65399962315
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22576878139/job/65401327924
- suite2-vectors pass URL:
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22576878148/job/65398330111
- Key artifacts:
  - `qsl/qsl-client/qsc/tests/common/mod.rs`
  - `qsl/qsl-client/qsc/tests/mock_relay_transport_na0173.rs`
- tests-only; harness contract enforcement + regressions; no product code changes.

### NA-0177 — TUI UX hardening + polish: make the qsc demo feel production-grade (client-only)

Status: DONE

Scope:
- qsl/qsl-client/qsc/**            (TUI + CLI UX implementation)
- qsl/qsl-client/qsc/tests/**      (unit/snapshot/non-interactive UX regression guards)
- Optional docs (only if required by goal-lint): TRACEABILITY/DECISIONS updates as needed

Must protect:
- No secret leakage (no tokens, no Authorization values, no /v1/<token> URIs).
- Preserve security coverage; no skip/ignore as a primary fix.
- Deterministic markers/log output for automation and CI.

Deliverables (ordered):
1) Onboarding / first-run UX
   - Clear setup path: relay URL + token file path validation + perms checks.
   - Clear “connected/authenticated/ready” status surface.
2) Trust/identity UX
   - Always-visible identity fingerprint surface + verified/unverified status.
   - Safe copy/confirm flows (no secrets).
3) Messaging UX polish
   - Clear send state (queued/sent/acked/failed) with user-meaningful reasons.
   - Deterministic error mapping for common failures (401, network unreachable, overloaded/429).
4) File transfer UX polish
   - Professional progress presentation and deterministic failure recovery semantics.
5) Diagnostics UX (safe-by-default)
   - A “debug view” that is redacted and deterministic (counts/hashes only).

Acceptance (must be explicit in PR evidence):
- A new user can complete: handshake → chat → file transfer from the TUI without reading source code.
- TUI surfaces verification/trust state clearly and consistently.
- At least one non-interactive regression guard exists for a key UX invariant (e.g., render snapshot, deterministic status string, or redaction invariant).
- macos-qsc-qshield-build remains stable (3 consecutive passes on same SHA for the PR before merge).

Evidence:
- PR link(s), merge SHA(s), and CI proof including macOS 3-pass same-SHA links.
- Post-fix hardening review (5 points) included in completion response.

Signal Comparative Plan of Record (Snapshot: 2026-03-06)
- Provenance:
  - Signal sources: official repositories under `https://github.com/signalapp`, inspected via local snapshots in `~/work/qsl/signal/{libsignal-main,Signal-Android-main,Signal-iOS-main,Signal-Desktop-main,Signal-Server-main}`.
  - Snapshot caveat: point-in-time local snapshots; no reliable snapshot commit SHAs recorded in this governance note.
- Top deltas (copy/adapt/reject):
  - Per-device session model (COPY/ADAPT): treat recipient+device as a first-class routing/session primitive.
  - Honest delivery ladder (COPY): keep `accepted_by_relay` and `peer_confirmed` explicit and non-conflated.
  - Typed receipts + batching discipline (ADAPT): keep privacy-aware policy modes `off|batched|immediate`.
  - Identity-change remediation flow (ADAPT): `CHANGED`/`REVOKED` remain fail-closed with clear operator actions.
  - Attachment idempotency/dedupe patterns (ADAPT): reduce duplicate work and ambiguous retries.
  - Logging posture (REJECT): do not adopt broad request URL/header logging patterns.
  - PQ posture clarity (ADAPT): avoid over-claiming PQ guarantees in UX text and status labels.
- Locked decisions (current plan):
  - Multi-device routing default: `primary_only` (fanout scaffold exists; fanout not active by default).
  - `peer_confirmed` meaning in multi-device mode (until fanout): `primary_device_only`.
  - Receipt policy default posture: `batched` (privacy-balanced), with `immediate` as explicit opt-in and `off` as privacy-high.
  - Fail-closed trust policy remains non-negotiable (no UX bypass).
- Evidence pointer (Phase C completion):
  - `qsl-protocol` PR #477: https://github.com/QuantumShieldLabs/qsl-protocol/pull/477
  - Head SHA: `247c34916975bcde2aae7e8d728a2fc4f3ec4604`
  - Merge SHA: `4b313969c2d3776c737a027d0f84baee54d3d3e0`
  - mergedAt: `2026-03-06T03:42:49Z`
- Next ordered work items (post-Phase C):
  - 1) Define and implement multi-device `peer_confirmed` policy wiring; tests must prove no false `peer_confirmed`.
  - 2) Trust UX consistency pass: clarify `VERIFIED` vs `TRUSTED` and remediation copy for `CHANGED`/`REVOKED`.
  - 3) File pipeline idempotency and dedupe-safe retry semantics (adapt Signal-style dependency graph patterns).
  - 4) Operator guidance hardening for rate-limit/backoff with bounded, deterministic, leak-safe behavior.

Evidence / Merged PRs
- PR #477 — Phase C per-device primary-only routing policy (merged 2026-03-06)
- PR #478 — governance record: Signal comparative Plan of Record for NA-0177 (merged 2026-03-06)
- PR #479 — primary-only `peer_confirmed` target-device binding + wrong-device ignore/no-mutation guards (merged 2026-03-07)

### NA-0178 — Trust Remediation UX Hardening (Fail-Closed, Operator-Clarity)

Status: DONE

Problem statement:
- `CHANGED` / `REVOKED` / `no_trusted_device` paths are security-correct, but operator guidance and wording need deterministic clarity.
- Terminology must remain explicit: `verified != trusted`.

Scope:
- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- Optional governance docs only if goal-lint requires.

Acceptance:
1) No trust bypasses; fail-closed behavior preserved.
2) Deterministic CLI/TUI wording and markers for:
   - `no_trusted_device`
   - `device_changed_reapproval_required`
   - `device_revoked`
3) Non-interactive deterministic tests assert wording and no-mutation on blocked paths.
4) Leak invariants in deterministic outputs:
   - `/v1/` count = 0
   - long-hex (`[0-9a-f]{32,}`) count = 0
5) Local gates pass: `fmt`, `test`, `clippy`, `build`.
6) Required CI green with `suite2-vectors` and macOS proof policy per directive class.

Evidence:
- PR: https://github.com/QuantumShieldLabs/qsl-protocol/pull/481
- Head SHA: `679a0359648dac0d62f16c9961ef17db6e3e55da`
- Merge SHA: `0feeee2ad8e576f658d775532b57fdc06bbef4c6`
- mergedAt: `2026-03-07T15:23:13Z`
- suite2-vectors PASS: https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22800719665/job/66141977676
- macOS 3-pass same-SHA:
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22800719677/job/66141977716
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22800719677/job/66143107965
  - https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/22800719677/job/66143699539
- Leak counts: `/v1/ = 0`, `long-hex>=32 = 0`

### NA-0179 — Repo Documentation Cleanup Program (Docs-only)

Status: DONE

Problem statement:
- Documentation sprawl is reducing discoverability and increasing risk of stale or duplicate entry guidance.
- The repo needs a safe, phased docs-only cleanup program with strict scope discipline.

Hard constraints:
- Docs-only: no changes to any code (`src/**`, `tests/**/*.rs`, scripts that change behavior).
- No changes to workflows (`.github/workflows/**`) or CI definitions.
- No changes to governance queue files except when explicitly directed by a governance directive.
- No secrets in docs: no tokens/auth headers, no `/v1/<token>`, no raw route tokens.
- Maintain truthful semantics language everywhere: `accepted_by_relay != peer_confirmed`.
- Maintain trust terminology consistency: `VERIFIED` vs `TRUSTED` vs `CHANGED` vs `REVOKED`.
- Do not introduce trust-bypass instructions in docs.

Phase 1 — Taxonomy + Deprecation of Superseded Entry Docs (docs-only)
Allowed files ONLY for Phase 1 implementation PR:
- `docs/INDEX.md`
- `docs/README.md`
- `docs/DOCS_MAP.md`
- `CHAT_STARTER.md`
- `README_PHASE4.md`
- `QSL_PUBLIC_RELEASE_PLAN.md`
- `docs/archive/START_HERE_2.md`

Forbidden (Phase 1):
- Any `src/**`, `tests/**` code files, `.github/**`, `tools/**`, `refimpl/**`
- `NEXT_ACTIONS.md`, `STATUS.md`, `TRACEABILITY.md`, `DECISIONS.md` (not part of Phase 1)

Phase 1 deliverables:
- `docs/INDEX.md` becomes the single docs front door and includes taxonomy labels: Authoritative / Supporting / Archive.
- `docs/README.md` becomes a short pointer to `docs/INDEX.md` (no parallel front door).
- `docs/DOCS_MAP.md` is aligned with taxonomy and clearly marks deprecated entry docs.
- Each superseded entry doc (`CHAT_STARTER.md`, `README_PHASE4.md`, `QSL_PUBLIC_RELEASE_PLAN.md`, `START_HERE_2.md`) becomes a DEPRECATED stub with replacement pointers and no operational instructions.
- No deletions in Phase 1 unless a file is proven unreferenced and replacement pointers exist.

Phase 2 — Historical test-plan markdown archive migration (docs-only; separate directive/PR)
- Move historical `tests/*.md` plans to `docs/archive/testplans/`.
- Add mapping index (`old name -> new location`).
- Leave a thin index in `tests/` (if anything remains) pointing to archive.

Phase 3 — Navigation consolidation (docs-only; separate directive/PR)
- Enforce single onboarding path: `START_HERE.md` (root) + `docs/INDEX.md` (docs).
- Reduce duplicate entry docs to short pointers.

Acceptance criteria (for NA-0179 Phase 1 PR):
1) A reader can find the authoritative start paths quickly:
   - root: `START_HERE.md`
   - docs: `docs/INDEX.md`
2) Deprecated entry docs contain no active instructions and clearly point to replacements.
3) No code/test/workflow behavior changes.
4) No secret-like material appears in docs.
5) Scope proof shows only allowed files were modified.

Evidence:
- Phase 1 docs taxonomy + deprecations — PR #483: https://github.com/QuantumShieldLabs/qsl-protocol/pull/483
  - Merge SHA (short): `0114658d58a9`
  - mergedAt: `2026-03-07T23:04:27Z`
- Phase 2 archive historical testplans — PR #484: https://github.com/QuantumShieldLabs/qsl-protocol/pull/484
  - Merge SHA (short): `d6221b422c19`
  - mergedAt: `2026-03-07T23:49:00Z`
- Historical tests markdown clutter moved to `docs/archive/testplans/` with redirect index at `docs/archive/testplans/INDEX.md`.
- `tests/` markdown surface reduced to `tests/README.md` plus harness contract markdown.
- Queue invariants preserved: `NA-0161` remains `BLOCKED`.
- Safe scan summary:
  - v1-path pattern count: 0
  - hex32plus pattern count: 0

### NA-0180 — Docs Hygiene Guardrails (Prevent Doc Sprawl)

Status: DONE

Problem:
- Docs cleanup gains can regress unless placement and classification guardrails are explicit and enforced.

Scope:
- `AGENTS.md`
- `docs/INDEX.md`
- Documentation files only

Deliverables:
1) Add a “Where docs go” guardrail: testplan markdown must live under `docs/archive/testplans/`.
2) Require a short classification header for new docs: `authoritative`, `supporting`, or `archive`.
3) Add a lightweight doc link sanity-check procedure (command-based), with no CI/workflow changes.

Acceptance:
1) One clear docs front door remains enforced.
2) No reintroduction of testplan markdown into `tests/` root.
3) No secret-like output patterns are introduced in docs guidance.

Evidence:
- Phase A — PR #486: https://github.com/QuantumShieldLabs/qsl-protocol/pull/486
  - Merge SHA (short): `a4d827d37ec3`
  - mergedAt: `2026-03-08T01:41:22Z`
- Phase B — PR #487: https://github.com/QuantumShieldLabs/qsl-protocol/pull/487
  - Merge SHA (short): `382ef0adbdbf`
  - mergedAt: `2026-03-08T02:12:07Z`
- Phase C — PR #488: https://github.com/QuantumShieldLabs/qsl-protocol/pull/488
  - Merge SHA (short): `b0b47df1e8b6`
  - mergedAt: `2026-03-08T14:15:01Z`
- Phase D — PR #489: https://github.com/QuantumShieldLabs/qsl-protocol/pull/489
  - Merge SHA (short): `561fcae9a632`
  - mergedAt: `2026-03-08T14:45:41Z`
- Phase E — PR #490: https://github.com/QuantumShieldLabs/qsl-protocol/pull/490
  - Merge SHA (short): `2a4a341ce58c`
  - mergedAt: `2026-03-08T15:41:08Z`
- Guardrails now in place:
  - Placement rules and doc classification header template.
  - Manual docs link-integrity runbook.
  - Docs PR checklist snippet.
  - Monthly audit cadence plus audit evidence template.
  - Docs move protocol example with explicit root-vs-recursive count pitfall coverage.
- Safe scan summary:
  - v1-path pattern count: 0
  - hex32plus pattern count: 0

### NA-0181 — Docs Cleanup Program Phase 3: Public/Release Docs Consistency (Docs-only, No Code Changes)

Status: DONE

Problem:
- Remaining docs risk is drift and duplication across public/release-facing guidance after cleanup and guardrail rollout.
- Goal: consolidate public release posture into one canonical path while preserving docs-only safety constraints.

Scope:
- Docs-only.
- No `src/**`, no `tests/*.rs`, no workflows.
- Expected touched areas (final allowlist set by implementation directive): `README.md` and `docs/public/**`.

Deliverables:
1) Identify duplicate or conflicting public/release guidance.
2) Consolidate guidance into canonical doc(s).
3) Convert superseded pages to DEPRECATED stubs with replacement pointers (no deletion unless unreferenced).
4) Update internal markdown links accordingly.
5) Use AGENTS docs PR checklist plus manual link-integrity runbook as required evidence.

Acceptance:
1) Docs-only PR(s) with strict name-only scope proof each time.
2) No broken links (runbook PASS).
3) No new docs front doors created.
4) Queue invariant preserved (sole READY remains `NA-0181` until close-out).

Evidence:
- PR: #492 https://github.com/QuantumShieldLabs/qsl-protocol/pull/492
- Merge SHA (short): `4558b747ffba`
- mergedAt: `2026-03-08T22:01:49Z`
- Outcomes:
  - Public/release documentation was consolidated around `docs/public/INDEX.md` as the canonical path.
  - Superseded pointers were normalized without expanding scope beyond docs-only cleanup.

### NA-0189 — AWS Round-2 Fix-or-File (Small-File Integrity + TUI /relay Diagnostic)

Status: DONE

Problem:
- Credentialed AWS Round-2 validation exposed two client-side issues:
  - small-file transfer could fail integrity verification with `manifest_mismatch`
  - headless TUI `/relay test` could return a misleading generic error even when later real traffic succeeded

Scope:
- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`

Deliverables:
1) Reproduce AWS-R2-001 and AWS-R2-002 on fresh credentialed two-client AWS runs.
2) Fix client-side root causes and add deterministic regression tests.
3) Update the AWS runbook and issue ledger with durable operator-safe evidence.

Acceptance:
1) AWS-R2-001 fixed or filed with conclusive evidence.
2) AWS-R2-002 fixed or filed with conclusive evidence.
3) Deterministic tests added for any fixed defect.
4) Secret-safe evidence only; honest delivery semantics preserved.

Evidence:
- PR: #499 https://github.com/QuantumShieldLabs/qsl-protocol/pull/499
- Merge SHA (short): `dbad4f31b23d`
- mergedAt: `2026-03-11T11:20:36Z`
- Outcomes:
  - AWS-R2-001 and AWS-R2-002 were fixed with deterministic regression coverage.
  - AWS runbook and issue ledger were updated, and CI completed green including macOS validation.

### NA-0190 — AWS TUI Command-Surface Audit (Two-Client, Real Relay) + UX Issue Ledger (Fix-or-File)

Status: DONE

Problem:
- The TUI command contract and operator flow need end-to-end validation on the real AWS relay using two isolated clients.
- Operator friction and misleading UX paths must be captured, then fixed and locked deterministically where possible.

Scope:
- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- No server code edits; secret-safe evidence only.

Deliverables:
1) AWS runbook subsection covering the TUI command audit and operator-safe execution steps.
2) PASS/FAIL matrix for command-surface coverage across setup, onboarding, trust, messaging, and file flows.
3) AWS issue ledger entries for every failure or meaningful friction point.
4) Client fixes with deterministic tests where conclusive, or filed follow-ons where not.

Acceptance:
1) At least 25 TUI commands exercised across setup, onboarding, trust, messaging, and file flows.
2) At least one negative fail-closed trust scenario proven.
3) Honest delivery semantics preserved (`accepted_by_relay` remains distinct from `peer_confirmed`).
4) No secret leakage in evidence or operator documentation.
5) Deterministic tests added for any fixed defect.

Evidence:
- PR: #501 https://github.com/QuantumShieldLabs/qsl-protocol/pull/501
- Merge SHA (short): `96ca54fc62a9`
- mergedAt: `2026-03-11T17:56:01Z`
- Outcomes:
  - AWS two-client TUI command-surface audit completed.
  - AWS-TUI-001 was fixed and locked by deterministic tests.
  - AWS-TUI-002 was filed for follow-on NA-0191.
  - AWS-FILE-007 was filed for follow-on NA-0192.
- Evidence hygiene:
  - v1-path pattern count: 0
  - hex32plus pattern count: 0

### NA-0191 — AWS TUI Handshake Decode-Failure Root Cause (Two-Client, External Relay) — Fix-or-File + Deterministic Lock

Status: DONE

Problem:
- AWS-TUI-002: clean AWS TUI handshake rerun fails with `decode_failed` after A1/B1 exchange.

Scope:
- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- Goal-lint-required linkage only if explicitly forced in the implementation directive.

Non-negotiables:
- Maintain fail-closed trust gates.
- Maintain honest delivery semantics (`accepted_by_relay` remains distinct from `peer_confirmed`).
- No secret leakage in markers, logs, tests, or response output.

Deliverables:
1) Reproduce the clean AWS TUI handshake failure on fresh isolated two-client state against the external relay.
2) Root-cause analysis with concrete code anchors.
3) Fix-or-file outcome:
   - If client-fixable: implement fix and add deterministic tests.
   - If not client-fixable: file with crisp repro, suspected server/protocol anchor, and mitigation.
4) Update the AWS runbook and issue ledger with secret-safe PASS/FAIL and issue status.

Acceptance:
1) Repro is confirmed or conclusively ruled out with evidence.
2) Deterministic tests are added for the root-cause path if a fix is made.
3) CI is green; macOS same-SHA 3-pass proof is present if production code changes.

Evidence:
- PR: #503 https://github.com/QuantumShieldLabs/qsl-protocol/pull/503
- Merge SHA (short): `1bd4c51042c8`
- mergedAt: `2026-03-12T01:41:35Z`
- Outcomes:
  - AWS-TUI-002 was reproduced on fresh isolated AWS clients and confirmed client-side.
  - Canonical TUI handshake commands now reuse the CLI handshake helpers and no longer lose `hs_pending` between `B1` and `A2`.
  - Deterministic restart/rerun regression coverage locks the clean A1/B1/A2 handshake contract.
  - The AWS runbook and issue ledger were updated; AWS-FILE-007 remains OPEN for follow-on NA-0192.
- Evidence hygiene:
  - reportable AWS evidence excerpts: `/v1/` count 0, `hex32plus` count 0, `Authorization/Bearer` hits 0, token literal hits 0.

### NA-0192 — AWS Medium-File Integrity Failure (qsp_verify_failed) Root Cause (Two-Client, External Relay)

Status: DONE

Problem:
- Tracks AWS-FILE-007 from the AWS ledger: medium-file receive can fail integrity verification with `qsp_verify_failed` during external-relay two-client validation.

Scope:
- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- Goal-lint-required linkage only if explicitly forced in the implementation directive.

Non-negotiables:
- Maintain fail-closed trust gates and integrity checks.
- Maintain honest delivery semantics (`accepted_by_relay` remains distinct from `peer_confirmed`).
- No secret leakage in markers, logs, tests, or response output.
- Do not widen into server/protocol-core changes unless a client-only fix is disproven with evidence.

Deliverables:
1) Reproduce AWS-FILE-007 on fresh isolated two-client state against the external relay, or conclusively rule it out with evidence.
2) Root-cause analysis with concrete code anchors and clear ownership (`client` vs `relay` vs `protocol`).
3) Fix-or-file outcome:
   - If client-fixable: implement the minimal fix and add deterministic regression tests.
   - If not client-fixable: file a crisp follow-on with secret-safe repro, suspected ownership, and mitigation.
4) Update the AWS runbook and issue ledger with secret-safe PASS/FAIL and issue status.

Acceptance:
1) Repro is confirmed or conclusively ruled out with evidence.
2) Deterministic tests are added for the root-cause path if a fix is made.
3) CI is green; macOS same-SHA 3-pass proof is present if production code changes.

Evidence:
- PR: #505 https://github.com/QuantumShieldLabs/qsl-protocol/pull/505
- Merge SHA (short): `9a0b7daedd71`
- mergedAt: `2026-03-12T23:20:54Z`
- Outcomes:
  - Clean AWS reproduction proved the small-file Bob -> Alice control passes on fresh mailbox state while the 1.2MB medium-file baseline still fails with `qsp_verify_failed`.
  - The merged implementation updated the AWS runbook and issue ledger with credential-pack bootstrap, fresh mailbox-token requirements, and the higher-fidelity `AWS-FILE-007` classification.
  - No client-side fix was asserted without proof; the follow-on remains a relay/protocol-boundary investigation.
- Evidence hygiene:
  - reportable AWS evidence excerpts: `/v1/` count 0, `hex32plus` count 0, `Authorization/Bearer` hits 0, token literal hits 0.

### NA-0192A — AWS Medium-File Integrity Relay-Boundary Investigation (Two-Client, External Relay)

Status: DONE

Problem:
- Direct continuation of AWS-FILE-007 after PR #505: clean AWS runs now prove a small-file PASS plus 1.2MB medium-file FAIL pairing, with the receiver failing on the first pulled medium envelope before file-specific client logic runs.

Scope:
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- read-only inspection of `qsl-server/**` and current protocol/spec text as needed to classify relay/protocol ownership
- minimal `qsl/qsl-client/qsc/src/**` / `qsl/qsl-client/qsc/tests/**` changes only if the relay-boundary investigation produces a conclusive client-side fix

Must protect:
- Maintain fail-closed trust and integrity behavior.
- Maintain honest delivery semantics (`accepted_by_relay` remains distinct from `peer_confirmed`).
- No secret leakage in markers, logs, docs, or test-visible output.
- Do not mutate the live relay during classification without a separate operational directive.

Deliverables:
1) Reproduce the clean small-file PASS plus 1.2MB medium-file FAIL pairing on fresh mailbox tokens.
2) Determine whether the first failing pulled envelope is corrupted, truncated, reordered, or mismatched across the relay/protocol boundary.
3) Prove whether bounded chunk-size variation changes the failure.
4) Fix client-side only if conclusive evidence isolates a client defect; otherwise file the relay/protocol remediation with directive-ready acceptance criteria.

Acceptance:
1) Ownership is narrowed to `client`, `relay`, `protocol`, or an explicit mixed boundary with evidence.
2) Any client-side fix is locked with deterministic tests and live revalidation.
3) If no client fix is justified, the follow-on record includes precise server/protocol mitigation scope without creating queue ambiguity.

Evidence:
- PR: #507 https://github.com/QuantumShieldLabs/qsl-protocol/pull/507
- Merge SHA: `70f12324d516`
- mergedAt: `2026-03-13T04:46:30Z`
- Outcomes:
  - Byte-identity/order proof and local replay established the original 32768-byte medium-file failure as a qsc client boundary bug; qsc now rejects that chunk size fail-closed before relay send and the clean 16384-byte 1.2MB receive path succeeds with explicit receive bounds.
  - The live AWS revalidation surfaced a separate follow-on (`AWS-FILE-008`): after the successful 16384-byte medium-file receive and completion send, the sender confirmation pull fails with `qsp_replay_reject`, so the direct continuation remains a mixed server/protocol boundary remediation item.
- Evidence hygiene:
  - reportable AWS evidence excerpts: `/v1/` count 0, `Authorization/Bearer` hits 0, token literal hits 0, capability-bearing URL hits 0.

### NA-0192B — AWS Medium-File Integrity Server/Protocol Boundary Remediation

Status: DONE

Problem:
- Direct continuation of AWS-FILE-008 after PR #507: the clean 16384-byte 1.2MB medium-file path now receives and completes on Alice, but Bob's follow-up confirmation pull fails with `qsp_replay_reject`, leaving the ownership narrowed to a mixed server/protocol boundary rather than a resolved client-only path.

Scope:
- `QuantumShieldLabs/qsl-server/**` read/write as needed for relay batching/pull semantics and remediation.
- `qsl/qsl-client/qsc/REMOTE_TWO_CLIENT_AWS_RUNBOOK.md`
- `qsl/qsl-client/qsc/REMOTE_AWS_ISSUE_LEDGER.md`
- Minimal `qsl/qsl-client/qsc/src/**` / `qsl/qsl-client/qsc/tests/**` changes only if the final evidence proves a client/protocol-side correction is required.

Must protect:
- Maintain fail-closed trust and integrity behavior.
- Maintain honest delivery semantics (`accepted_by_relay` remains distinct from `peer_confirmed`).
- Preserve the fixed 32768-byte chunk-bound reject and the clean 16384-byte receive path from PR #507.
- No secret leakage in relay evidence, server logs, docs, or test-visible output.

Deliverables:
1) Reproduce the post-#507 16384-byte medium-file confirmation failure on clean AWS mailbox state and determine whether relay pull semantics, protocol replay framing, or a mixed boundary interaction is responsible.
2) Prove whether relay/server batching or server/protocol confirmation ordering mutates or misclassifies the completion-ack path.
3) Fix in the correct repo(s) only after ownership is conclusive, with deterministic locks wherever feasible.
4) Update runbooks/ledgers and queue evidence truthfully without reopening the already-fixed 32768-byte client chunk-bound bug.

Acceptance:
1) Ownership is narrowed to `relay`, `protocol`, or an explicit mixed boundary with evidence grounded in the post-#507 confirmation path.
2) Any fix preserves fail-closed file/integrity behavior and honest delivery semantics.
3) Queue continuity remains single-threaded with `NA-0192B` as the direct continuation until the confirmation-path issue is closed.

Evidence:
- PR: #509 https://github.com/QuantumShieldLabs/qsl-protocol/pull/509
- Merge SHA: `e0a5f351e1d2`
- mergedAt: `2026-03-14T01:04:15Z`
- Outcomes:
  - Fresh current-mainline AWS evidence reproduced the `M1` baseline: Alice completed the clean 16384-byte 1.2MB medium-file receive and sent the coarse-complete confirmation, while Bob replay-rejected that fresh confirmation on a single-item pull.
  - The merged qsl-protocol fix now commits the receive unpack state before sending the file-complete receipt, so the send-side ratchet advance is not clobbered by the older receive snapshot.
  - Two fresh candidate AWS reruns removed the confirmation replay failure while preserving the protected 16384-byte receive path and the fixed 32768-byte fail-closed reject.
- Evidence hygiene:
  - reportable AWS evidence excerpts: `/v1/` count 0, `Authorization/Bearer` hits 0, token literal hits 0, capability-bearing URL hits 0.

### NA-0193 — qsl-server Deployment/Layout Cleanup + Canonical Packaging Alignment (Server/Ops)

Status: DONE

Problem:
- qsl-server still has duplicated/conflicting service definitions and route-token-in-URL handling documented in ways that indicate unresolved deployment/layout drift on the server boundary.

Scope:
- `QuantumShieldLabs/qsl-server/**`
- Packaging/runbook/systemd/docs files as needed.
- No `qsl/**` protocol-core changes.

Must protect:
- Relay remains transport-only and opaque to QSL/QSP payload semantics.
- Operator artifacts remain secret-safe; no token leakage in docs, logs, or examples.

Deliverables:
1) Audit the in-tree deployment/layout drift and identify the canonical install/update shape.
2) Resolve or explicitly deprecate duplicate/conflicting packaging/service definitions.
3) Update the server runbook so a clean host install and update path are deterministic.
4) File any host-only cleanup steps separately if they do not belong in git.

Acceptance:
1) One canonical systemd/env/layout path is represented in-tree.
2) Duplicate or conflicting service definitions are resolved or clearly deprecated.
3) No relay semantics, protocol parsing, or cryptographic behavior changes occur in this item.

Evidence:
- qsl-server promotion PR: #26 https://github.com/QuantumShieldLabs/qsl-server/pull/26
- qsl-server implementation PR: #27 https://github.com/QuantumShieldLabs/qsl-server/pull/27
- qsl-server closeout PR: #28 https://github.com/QuantumShieldLabs/qsl-server/pull/28
- Merge SHAs:
  - qsl-server PR #26: `834a3cb9df88`
  - qsl-server PR #27: `94da6e22eac7`
  - qsl-server PR #28: `7707ff929dd0`
- mergedAt:
  - qsl-server PR #26: `2026-03-14T02:07:56Z`
  - qsl-server PR #27: `2026-03-14T02:16:44Z`
  - qsl-server PR #28: `2026-03-14T02:19:46Z`
- Outcomes:
  - qsl-server governance was repaired first: `NA-0006` was promoted cleanly as the sole READY item and stale READY traceability drift for `NA-0003/NA-0004/NA-0005` was removed.
  - The qsl-server repo now has one canonical packaging-based install/update story centered on `packaging/systemd/qsl-server.service`, `packaging/systemd/relay.env.example`, `/etc/qsl-server/relay.env`, and the aligned install/update/verify scripts.
  - The conflicting root `systemd/qsl-server.service` was removed, `scripts/install_ubuntu_24_04_systemd.sh` was retained only as a deprecated wrapper, and `scripts/verify_remote.sh` no longer assumes `/opt/qsl-server/repo`.
  - qsl-server READY count returned to 0 after closeout.
- Evidence hygiene:
  - qsl-server changes stayed in packaging/docs/scripts/governance scope only; no relay semantics, auth posture, API shape, or protocol behavior changed.

### NA-0194 — GitHub Actions Runtime Maintenance + CI Risk-Tiering (Workflow/Policy Only)

Status: DONE

Problem:
- Current qsl-protocol workflows still depend on `actions/checkout@v4`, and docs/governance-only pull requests still run broad CI lanes because workflow triggers and required checks are not risk-tiered.

Scope:
- `.github/workflows/**` in the affected public repositories.
- Supporting policy/docs updates only as needed.
- No product/runtime code changes.

Must protect:
- Do not weaken required gates for runtime-critical client/server/protocol paths.
- Preserve CodeQL, vectors, and other security-critical checks where applicable.

Deliverables:
1) Remove current GitHub Actions runtime deprecation exposure from maintained workflows.
2) Encode and/or document a risk-tiered CI policy so docs/governance-only changes do not pay unnecessary heavy-lane costs.
3) Align GitHub-enforced required checks with the intended release/risk policy.

Acceptance:
1) Maintained workflows no longer emit the current JavaScript-action runtime deprecation warning.
2) Docs/governance-only changes no longer require unnecessary heavy-lane reruns.
3) Runtime-critical paths still prove stability appropriately.

Evidence:
- qsl-protocol implementation PR: #512 https://github.com/QuantumShieldLabs/qsl-protocol/pull/512
- qsl-protocol merge SHA: `da1c976d9522`
- qsl-protocol mergedAt: `2026-03-14T04:25:46Z`
- qsl-server PR set:
  - promotion PR #29 https://github.com/QuantumShieldLabs/qsl-server/pull/29 (merge SHA `61ac7ab3f3ee`, mergedAt `2026-03-14T03:29:46Z`)
  - implementation PR #30 https://github.com/QuantumShieldLabs/qsl-server/pull/30 (merge SHA `e61239ff84b2`, mergedAt `2026-03-14T03:42:29Z`)
  - closeout PR #31 https://github.com/QuantumShieldLabs/qsl-server/pull/31 (merge SHA `729c04442b48`, mergedAt `2026-03-14T03:48:55Z`)
- Outcomes:
  - Maintained qsl-protocol workflows now use safe maintained action majors where available (`actions/checkout@v5`, `actions/setup-python@v6`, `actions/upload-artifact@v6`, `actions/attest-build-provenance@v3`), while `dtolnay/rust-toolchain@stable` remains in place because it is a composite action rather than a deprecation-exposed JavaScript action.
  - qsl-protocol now classifies PR scope deterministically and resolves stable required contexts cheaply for docs/governance-only changes without weakening runtime/security gates for workflow-security or runtime-critical changes.
  - qsl-protocol `main` branch protection now matches the implemented risk policy and requires the stable contexts `public-safety`, `ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`, `demo-cli-build`, `demo-cli-smoke`, `formal-scka-model`, `goal-lint`, `metadata-conformance-smoke`, `suite2-vectors`, `CodeQL`, and `macos-qsc-qshield-build`.
  - qsl-server maintained workflows now use maintained action majors where applicable, and qsl-server `main` now has a minimal truthful protection baseline that requires only `rust` for ordinary pull requests.
- Evidence hygiene:
  - Workflow/policy/settings scope only; no qsl-protocol product/runtime files or qsl-server runtime/API/auth/relay semantics changed, and no secrets or bearer/token values were committed.

### NA-0195 — Route-Token-in-URL API Shape Review + Migration Decision (Docs/Design)

Status: DONE

Problem:
- The relay API still uses route tokens in `/v1/push/{channel}` and `/v1/pull/{channel}` URL paths, which remains a security/operability concern because URLs propagate into logs, proxies, browser history, and tooling.

Scope:
- Relevant qsl-server API docs/spec text and `DECISIONS.md` / `TRACEABILITY.md` linkage as needed.
- Docs/design analysis only unless a separate implementation item is later promoted.

Must protect:
- No accidental token disclosure in docs, examples, or evidence.
- No silent compatibility break without an explicit migration plan.

Deliverables:
1) Threat-model the current route-token placement and enumerate concrete leakage surfaces.
2) Decide whether to keep or migrate the API shape, with rationale.
3) If change is justified, produce a follow-on implementation item with migration and compatibility criteria.

Acceptance:
1) Decision is recorded with rationale.
2) Compatibility and operator impacts are understood.
3) No protocol-core or relay behavior changes occur in this item.

Evidence:
- qsl-server promotion PR: #32 https://github.com/QuantumShieldLabs/qsl-server/pull/32
- qsl-server implementation PR: #33 https://github.com/QuantumShieldLabs/qsl-server/pull/33
- qsl-server closeout PR: #34 https://github.com/QuantumShieldLabs/qsl-server/pull/34
- Merge SHAs:
  - qsl-server PR #32: `c42d63438d61`
  - qsl-server PR #33: `893144a5a5e9`
  - qsl-server PR #34: `de2e5d98a94a`
- mergedAt:
  - qsl-server PR #32: `2026-03-14T12:11:19Z`
  - qsl-server PR #33: `2026-03-14T12:16:35Z`
  - qsl-server PR #34: `2026-03-14T12:19:02Z`
- Outcome:
  - qsl-server docs/design review chose MIGRATE away from URL-embedded route tokens.
  - Grounded leakage surfaces include reverse-proxy/access logs, shell history and copied command lines, support bundles/screenshots/tutorials, and observability traces/metrics.
  - README and qsl-server API docs now treat `/v1/push/:channel` and `/v1/pull/:channel` as the current compatibility shape only, with direct follow-on requirements recorded for a compatibility-preserving migration.
- Evidence hygiene:
  - Docs/design/governance scope only; no qsl-protocol product/runtime files or qsl-server runtime/API/auth/relay semantics changed, and no raw route tokens were committed.

### NA-0195A — Route-Token API Migration + Compatibility Rollout

Status: DONE

Problem:
- qsl-server Decision D-0008 concluded that route tokens should migrate out of URL paths because the current `/v1/push/{channel}` and `/v1/pull/{channel}` shape leaks capability-like identifiers across operator-visible surfaces and leaves too much safety burden on compensating controls.

Scope:
- `QuantumShieldLabs/qsl-server/**` runtime/API surfaces and docs needed to support a compatibility-preserving migration away from URL-embedded route tokens.
- Relevant `qsl-protocol` relay client/docs/test surfaces needed to preserve interoperability during rollout.
- No attachment-architecture work in this item.

Must protect:
- No silent compatibility break for existing clients/operators.
- No raw route-token leakage in logs, traces, examples, screenshots, or support artifacts.
- Fail-closed auth/relay behavior remains intact throughout rollout.
- Current route-token capability semantics remain explicit and deterministic until migration completes.

Deliverables:
1) Define and implement the replacement route-token carriage mechanism and compatibility window.
2) Update server/client handling, docs/runbooks, and verification flows for the migrated shape.
3) Define rollout, deprecation, and removal criteria for the URL-embedded compatibility path.
4) Add deterministic compatibility and log-safety validation proving the migration does not regress delivery semantics.

Acceptance:
1) Migrated route-token carriage works with an explicit compatibility strategy and no silent break.
2) Supported operator and client flows no longer require raw route tokens in URL paths.
3) Log-safety and compatibility validation are recorded with deterministic proof.

Evidence:
- qsl-server promotion PR: #35 https://github.com/QuantumShieldLabs/qsl-server/pull/35
- qsl-server implementation PR: #36 https://github.com/QuantumShieldLabs/qsl-server/pull/36
- qsl-server closeout PR: #37 https://github.com/QuantumShieldLabs/qsl-server/pull/37
- qsl-protocol implementation PR: #515 https://github.com/QuantumShieldLabs/qsl-protocol/pull/515
- Merge SHAs:
  - qsl-server PR #35: `ddaf4da68868`
  - qsl-server PR #36: `1bbf0207ec3e`
  - qsl-server PR #37: `41b51998bbb6`
  - qsl-protocol PR #515: `74f00ac8f71d`
- mergedAt:
  - qsl-server PR #35: `2026-03-14T13:25:39Z`
  - qsl-server PR #36: `2026-03-14T13:48:15Z`
  - qsl-server PR #37: `2026-03-14T20:19:46Z`
  - qsl-protocol PR #515: `2026-03-14T21:27:57Z`
- Outcome:
  - qsl-server and qsc now use token-free canonical relay endpoints (`POST /v1/push`, `GET /v1/pull?max=N`) with `X-QSL-Route-Token` as the route-token carriage header.
  - qsl-server keeps the legacy `/v1/push/:channel` and `/v1/pull/:channel` paths as a compatibility-only surface during the rollout window, with deterministic mismatch rejection and no mutation on mismatch or missing canonical headers.
  - Supported qsc operator/client flows no longer require raw route tokens in URL paths, and deterministic tests lock canonical request shape, compatibility behavior, and docs/script log safety.
  - Advisory classification for PR #515 was `A1` (baseline-only, non-required), and the implementation merged via path `M1` after proving the failing `advisories` lane reproduces unchanged on current `main` and all required checks were green.
- Evidence hygiene:
  - No qsl-server changes were made in this directive; qsl-server PRs #35/#36/#37 were referenced as prior merged evidence only.
  - No workflow, website, `.github`, or attachment-program changes were made, and no raw route tokens or bearer secrets were committed.

### NA-0195B — qsl-protocol Dependency Advisory Baseline Remediation

Status: DONE

Problem:
- qsl-protocol still carries a material dependency advisory baseline that either blocks or materially undermines confidence in ordinary runtime PRs.

Scope:
- qsl-protocol dependency graph, Cargo manifests, lockfiles, and the minimal code adjustments strictly required for safe dependency upgrades.
- No qsl-server work and no attachment-program work.

Must protect:
- No wire, protocol, or crypto semantic changes without explicit proof and decision.
- No hidden algorithm substitutions.
- Fail-closed behavior and the merged route-token migration behavior remain intact.

Deliverables:
1) Classify each remaining advisory by direct/transitive and runtime/dev-only surface.
2) Remediate safe upgrades or explicit removals where possible.
3) Record any justified residual risk explicitly.
4) Restore green ordinary runtime-PR confidence.

Acceptance:
1) No material unresolved advisory baseline remains on ordinary qsl-protocol runtime PRs, or explicit governance records any unavoidable residual risk.
2) Protocol, wire, and crypto semantics remain unchanged unless separately authorized.
3) Queue and evidence are updated truthfully.

Evidence:
- qsl-protocol implementation PR: #517 https://github.com/QuantumShieldLabs/qsl-protocol/pull/517
- Merge SHA: `191d2426b68a`
- mergedAt: `2026-03-14T23:31:19Z`
- Advisories classification summary:
  - `R1` safe direct remediation: `bytes` `1.11.1`, `quinn-proto` `0.11.14`, `keccak` `0.1.6`
  - `R2` governed residuals via `.cargo/audit.toml`: `RUSTSEC-2024-0388` (`derivative`), `RUSTSEC-2024-0384` (`instant`), `RUSTSEC-2024-0436` (`paste`), `RUSTSEC-2025-0144` (`ml-dsa`), `RUSTSEC-2024-0380` (`pqcrypto-dilithium`), `RUSTSEC-2024-0381` (`pqcrypto-kyber`), `RUSTSEC-2026-0002` (`lru`)
  - `R3` out-of-scope semantic-risk findings: none after the safe `keccak` patch update
- Remediation summary:
  - The exact current `public-ci` advisories command now passes on `main` under the workflow-pinned Rust `1.84.0` toolchain.
  - Route-token migration behavior from `NA-0195A` remained intact through the targeted qsc regression checks.
  - No qsl-server, workflow, website, `.github`, or attachment-program files changed in this item.
- Residual-risk / ignore summary:
  - The repo-local cargo-audit config at `.cargo/audit.toml` is advisory-specific and narrow.
  - Every ignored advisory reproduced on pre-remediation `main` and was not introduced or worsened by PR #517.
  - The remaining residuals still matter because they include direct/runtime crypto-adjacent packages and supported-surface UI/keychain transitive packages whose safe removal or replacement is out of scope for this item.
- Closeout path:
  - `G2` — the advisories lane is now truthful and green, but material residual advisory risk still requires direct continuation ahead of `NA-0196`.

### NA-0195C — qsl-protocol Residual Advisory Risk Resolution

Status: DONE

Problem:
- qsl-protocol still carries documented residual advisory risk that could not be safely eliminated within NA-0195B without out-of-scope semantic change.

Scope:
- qsl-protocol dependency graph / manifests / lockfiles / minimal code adjustments and, if needed, explicit design decisions for any remaining crypto-adjacent replacements
- no qsl-server work
- no attachment-program work

Must protect:
- no wire/protocol/crypto semantic changes without explicit proof and decision
- no hidden algorithm substitutions
- merged route-token migration behavior remains intact

Deliverables:
1) eliminate or explicitly redesign around the remaining residual advisories
2) record any unavoidable residual risk with exact scope
3) restore full ordinary runtime-PR confidence

Acceptance:
1) no material residual advisory baseline remains, or explicit governance decision records the only unavoidable residuals with clear rationale
2) protocol/wire/crypto semantics remain unchanged unless separately authorized
3) queue/evidence are updated truthfully

Evidence:
- qsl-protocol implementation PR: #519 https://github.com/QuantumShieldLabs/qsl-protocol/pull/519
- Merge SHA: `1167dea08c7b`
- mergedAt: `2026-03-15T01:35:22Z`
- Residual matrix summary:
  - `S1` safe remediation now: upgraded the supported qsc/qsl-tui TUI surface onto `ratatui` `0.30.0` / `crossterm` `0.29.0`, upgraded the optional qsc keychain surface onto `keyring` `3.6.3`, and refreshed `Cargo.lock` so the prior `derivative`, `instant`, `paste`, and old `lru` residuals no longer appear in the raw audit output.
  - `S2` safe supported/optional surface reduction now: none beyond tightening the optional keychain surface behind its existing non-default feature gate and preserving the demo-only qshield-cli HTTP stack as an explicit temporary exception.
  - `S3` residual design-needed but governable: none after the safe supported-surface cleanup.
  - `S4` out-of-scope semantic-risk residuals: `RUSTSEC-2025-0144` (`ml-dsa` via `refimpl_actor`), `RUSTSEC-2024-0380` (`pqcrypto-dilithium` on supported/runtime and shared internal PQ paths), `RUSTSEC-2024-0381` (`pqcrypto-kyber` on supported/runtime and shared internal PQ paths).
- Remediation summary:
  - The exact `public-ci` advisories command is now green on the merged implementation path and remains truthful because `.cargo/audit.toml` contains only the three residual crypto-adjacent advisories above.
  - `tests/NA-0195C_dependency_architecture_evidence.md` records the canonical commodity stacks, supported-surface decisions, security-sensitive chain owners, and the residual audit map needed to evaluate future dependency PRs.
  - The merged `NA-0195A` route-token migration behavior remained intact through targeted qsc regression coverage.
- Retained ignore summary:
  - `.cargo/audit.toml` now retains only `RUSTSEC-2025-0144`, `RUSTSEC-2024-0380`, and `RUSTSEC-2024-0381`.
  - Each retained ignore is advisory-specific, reproduced in the raw scratch-clone audit without config, and could not be safely removed in this item without forbidden crypto/API redesign.
- Closeout path:
  - `J2` — the advisories signal is now truthful and green, but material residual advisory risk still remains concentrated in security-sensitive crypto/PQ boundaries, so the queue advances to a narrower direct continuation ahead of `NA-0196`.
- Dependency-policy / canonical-stack summary:
  - Canonical workspace commodity stacks are `reqwest` + `rustls`, `clap`, `serde` / `serde_json`, `tracing`, and `tokio` only where async behavior is actually needed.
  - `qsc` keychain remains a supported optional surface; qsc/qsl-tui TUI remains a supported deliberate product surface; qshield-cli keeps `ureq` + `tiny_http` only as a temporary demo-surface exception.

### NA-0195D — qsl-protocol Security-Sensitive Dependency Boundary Consolidation

Status: DONE

Problem:
- qsl-protocol still carries material residual advisory risk concentrated in crypto-adjacent and supported runtime dependency chains that could not be safely eliminated within NA-0195C.

Scope:
- qsl-protocol dependency graph / manifests / lockfiles / minimal code adjustments and, if needed, explicit design decisions for remaining crypto-adjacent or supported-surface boundary replacements
- no qsl-server work
- no attachment-program work

Must protect:
- no wire/protocol/crypto semantic changes without explicit proof and decision
- no hidden algorithm substitutions
- merged route-token migration behavior remains intact

Deliverables:
1) eliminate or explicitly redesign around the remaining material residuals
2) record any unavoidable residual risk with exact supported-surface scope
3) restore full ordinary runtime-PR confidence

Acceptance:
1) no material residual advisory baseline remains, or explicit governance decision records the only unavoidable residuals with clear rationale
2) protocol/wire/crypto semantics remain unchanged unless separately authorized
3) queue/evidence are updated truthfully

Evidence:
- Implementation PR: #521 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/521)
- Merge commit: `d02001fa4c0ba4bfb18c402a5da4ec058b6cceee`
- Merged at: `2026-03-15T03:19:39Z`
- Boundary consolidation summary:
  - `qsc` no longer directly names `pqcrypto-kyber`, `pqcrypto-dilithium`, or `pqcrypto-traits` in its manifest.
  - supported qsc source/tests now consume runtime PQ keypair/length helpers from `quantumshield_refimpl::crypto::stdcrypto` instead of naming provider crates directly.
  - `qsl-tui` remains boundary-only for runtime PQ ownership, while `refimpl_actor` remains tooling-only for `ml-dsa` / `ml-kem`.
  - `quantumshield_refimpl` is now the sole owned supported-runtime PQ/provider boundary, recorded in Decision `D-0303` and `tests/NA-0195D_dependency_boundary_evidence.md`.
- Residual matrix summary after consolidation:
  - `RUSTSEC-2025-0144` (`ml-dsa`) remains tooling-only via `refimpl_actor`.
  - `RUSTSEC-2024-0380` (`pqcrypto-dilithium`) remains only inside the owned `quantumshield_refimpl` supported-runtime boundary.
  - `RUSTSEC-2024-0381` (`pqcrypto-kyber`) remains only inside the owned `quantumshield_refimpl` supported-runtime boundary.
- Retained ignore summary:
  - `.cargo/audit.toml` retains only `RUSTSEC-2025-0144`, `RUSTSEC-2024-0380`, and `RUSTSEC-2024-0381`, each classified explicitly as tooling-only or boundary-internal residuals.
- Closeout path:
  - `K2` — supported runtime no longer owns provider churn directly, but material PQ/provider residual advisory risk still remains inside the newly consolidated internal boundary, so the queue advances to a narrower provider-migration follow-on ahead of `NA-0196`.
- Dependency-policy / boundary-owner summary:
  - `quantumshield_refimpl` is the owned supported-runtime PQ/provider boundary.
  - supported app/runtime crates must not directly own third-party PQ/provider churn.
  - route-token migration behavior from `NA-0195A` remained intact throughout this item.

### NA-0195E — qsl-protocol PQ Provider Migration / Boundary-Internal Crypto Replacement

Status: DONE

Problem:
- qsl-protocol still carries material residual advisory risk inside the now-owned internal PQ/provider boundary, and eliminating it requires explicit provider replacement or crypto-boundary redesign that was out of scope for `NA-0195D`.

Scope:
- qsl-protocol internal PQ/provider boundary crate(s), manifests, lockfiles, minimal code adjustments, and explicit design decisions for provider replacement
- no qsl-server work
- no attachment-program work

Must protect:
- no wire/protocol/crypto semantic changes without explicit proof and decision
- no hidden algorithm substitutions
- app/runtime boundary consolidation from `NA-0195D` remains intact
- merged route-token migration behavior remains intact

Deliverables:
1) choose the internal PQ/provider replacement plan
2) prove wire/protocol behavior equivalence or explicitly isolate any authorized change
3) eliminate the remaining material residual advisories inside the boundary
4) restore full ordinary runtime-PR confidence

Acceptance:
1) no material residual advisory baseline remains on supported runtime surfaces, or any remaining residual is explicitly governed and non-material
2) protocol/wire/crypto semantics remain unchanged unless separately authorized
3) queue/evidence are updated truthfully

Evidence:
- Implementation PR: #523 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/523)
- Merge commit: `05e4227a0b0deb89778d4a821ab0e9d2fd7c60d7`
- Merged at: `2026-03-15T05:24:34Z`
- Provider-migration summary:
  - `quantumshield_refimpl` remained the sole supported-runtime PQ/provider boundary owner.
  - ML-KEM-768 moved from `pqcrypto-kyber` to `pqcrypto-mlkem`.
  - ML-DSA-65 moved from `pqcrypto-dilithium` to maintained RustCrypto `ml-dsa`.
  - supported app/runtime crates did not regain direct third-party PQ/provider ownership.
- Residual matrix summary:
  - supported-runtime PQ residuals removed from the raw no-config audit path
  - pre-edit supported-runtime residuals `RUSTSEC-2024-0380` and `RUSTSEC-2024-0381` no longer appear on merged `main`
  - remaining raw residual is tooling-only `RUSTSEC-2025-0144` via `refimpl_actor -> ml-dsa`
- Retained ignore summary:
  - `.cargo/audit.toml` now retains only `RUSTSEC-2025-0144` because it is tooling-only, outside supported-runtime risk accounting, and not a safe drop-in upgrade inside this item
- Closeout path: `L1`
- Boundary-owner / equivalence-proof summary:
  - boundary owner remained `quantumshield_refimpl`
  - existing on-wire, handshake, route-token, and fail-closed regressions stayed green
  - no public length/serialization contract drift was introduced into supported runtime

### NA-0196 — Cross-Repo Public/License Posture Alignment (Docs/Legal Hygiene)

Status: DONE

Problem:
- Public posture is still drifting across qsl-protocol, qsl-server, the QuantumShield website, and the QuantumShieldLabs org profile: notice/license text, repo descriptions, and website legal copy are not fully aligned with current public repo reality.

Scope:
- `README.md`, `NOTICE`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, org profile docs, and website legal/proof/docs pages.
- Docs/legal copy only; no protocol, relay, or website product-behavior changes.

Must protect:
- No marketing or security claims beyond current evidence.
- No contradictory public licensing posture across repos.
- Escalate for counsel review if the intended public/commercial split is unclear.

Deliverables:
1) Ensure every public repo with notice/license language has an explicit in-tree `LICENSE` and aligned notices.
2) Align website legal/product positioning with the actual public-source/licensing posture, or explicitly distinguish open-source and commercial offerings.
3) Refresh org-profile and public proof links so repo descriptions and evidence pointers match current public reality.

Acceptance:
1) No public repo references a missing `LICENSE`.
2) No public page contradicts the actual distribution posture.
3) Current proof links resolve to current or canonical evidence.

Evidence:
- qsl-server PR set:
  - promotion PR #38 https://github.com/QuantumShieldLabs/qsl-server/pull/38 (merge SHA `488abb95703d`, mergedAt `2026-03-15T14:43:09Z`)
  - implementation PR #39 https://github.com/QuantumShieldLabs/qsl-server/pull/39 (merge SHA `3ed59379bdbc`, mergedAt `2026-03-15T14:50:15Z`)
  - closeout PR #40 https://github.com/QuantumShieldLabs/qsl-server/pull/40 (merge SHA `b1926ea69a62`, mergedAt `2026-03-15T14:54:55Z`)
- website PR set:
  - promotion PR #13 https://github.com/Tebbens4832/QuantumShield/pull/13 (merge SHA `d0585cd2ac85`, mergedAt `2026-03-15T14:43:09Z`)
  - implementation PR #14 https://github.com/Tebbens4832/QuantumShield/pull/14 (merge SHA `2137678fe58f`, mergedAt `2026-03-15T14:53:28Z`)
  - closeout PR #15 https://github.com/Tebbens4832/QuantumShield/pull/15 (merge SHA `65ec2f338969`, mergedAt `2026-03-15T14:56:07Z`)
- `.github` implementation PR:
  - PR #4 https://github.com/QuantumShieldLabs/.github/pull/4 (merge SHA `65a7541d6480`, mergedAt `2026-03-15T14:50:55Z`)
- qsl-protocol implementation PR:
  - PR #525 https://github.com/QuantumShieldLabs/qsl-protocol/pull/525 (merge SHA `03c5027d5df6`, mergedAt `2026-03-15T14:54:55Z`)
- Cross-repo posture outcomes:
  - Every touched public repo that references a `LICENSE` now ships an explicit in-tree `LICENSE`.
  - qsl-protocol repo-local posture now states that the public repo includes specifications, conformance vectors, and research-stage reference implementations under `AGPL-3.0-only`.
  - qsl-server now ships an explicit `LICENSE` and a README posture section that keeps the transport-only relay repo distinct from any separate commercial services/support.
  - The website legal/high-visibility copy now distinguishes the public AGPL repositories from any future separate commercial offerings/support and no longer states that the public protocol libraries require a separate license.
  - The QuantumShieldLabs org profile no longer describes qsl-protocol as docs/vectors only.
- Metadata alignment:
  - qsl-protocol repo description before: `QSL Protocol: public specifications and conformance vectors (Suite-2 / SCKA).`
  - qsl-protocol repo description after: `QSL Protocol: public specifications, conformance vectors, and research-stage reference implementations.`
  - qsl-server and website descriptions were reviewed and left unchanged because they did not materially contradict the merged file posture.
- Public/commercial distinction:
  - Public qsl-protocol, qsl-server, and website repositories are governed by the licenses shipped in those repositories.
  - Any future commercial SDK, hosted service, consulting, or support offering is separate from those public repository licenses and is not a prerequisite to read, evaluate, or use the public source under its published terms.

### NA-0196A — Website Technical Claims / Proof Integrity Alignment

Status: DONE

Problem:
- The public QuantumShield website still contains materially misleading technical/security/status claims despite the NA-0196 legal/posture alignment:
  - hardcoded "latest proof" / "recent PASS" style claims,
  - production-ready / audited / benchmark / support-posture claims without current public repo evidence,
  - outdated protocol architecture framing,
  - residual "request access" language for public AGPL repositories,
  - and privacy/metadata claims stronger than the currently documented bounded guarantees.

Scope:
- Website repo content/legal/proof/docs and qsl-protocol governance linkage only.
- No website product/UI behavior changes.
- No qsl-server work.
- No attachment-program work.

Must protect:
- Public technical/security/status claims must track current canonical qsl-protocol docs and current live proof sources.
- No hardcoded "latest" claims unless clearly dated historical examples.
- No product/runtime/workflow changes.
- Public AGPL repositories must not be described as gated-access resources.

Deliverables:
1) Remove or rewrite materially unsupported website technical/security/status claims using current public evidence only.
2) Align website architecture wording to the current canonical qsl-protocol docs.
3) Add a minimal website-only guardrail so unsupported technical/security/status claims do not drift back in.

Acceptance:
1) No materially unsupported website technical/security/status claims remain in the corrected surfaces.
2) Any retained proof/status wording is source-backed and not stale.
3) No protocol, relay, workflow, or website product behavior changes occur.
4) Queue/evidence are updated truthfully and NA-0197 is restored only after this urgent correction closes.

Evidence:
- qsl-protocol governance repair PR: #527 https://github.com/QuantumShieldLabs/qsl-protocol/pull/527
- Website promotion PR: #16 https://github.com/Tebbens4832/QuantumShield/pull/16
- Website implementation PR: #17 https://github.com/Tebbens4832/QuantumShield/pull/17
- Website closeout PR: #18 https://github.com/Tebbens4832/QuantumShield/pull/18
- Merge SHAs:
  - qsl-protocol repair: `43f51b59848f`
  - website promotion: `570146b67d59`
  - website implementation: `247918de92ed`
  - website closeout: `d5e91acc7d63`
- mergedAt:
  - qsl-protocol repair: `2026-03-15T20:18:50Z`
  - website promotion: `2026-03-15T20:21:02Z`
  - website implementation: `2026-03-15T20:51:08Z`
  - website closeout: `2026-03-15T20:53:10Z`
- Outcomes:
  - The website no longer hardcodes `latest proof` or `recent PASS` claims; it now links readers to the live qsl-protocol workflow pages for current status.
  - Unsupported production/audit/benchmark/support posture claims were removed or rewritten as roadmap/future-boundary language.
  - Website architecture wording now tracks the current qsl-protocol public docs: True Triple Ratchet, SCKA, bounded privacy claims, and the current runtime crypto boundary.
  - Public AGPL repositories are no longer described as gated resources, and the website now ships a permanent static guardrail plus source map for technical/security/status claims.
  - NA-0197 was temporarily demoted only to allow this urgent public-truth correction to land before attachment-architecture work begins.
- Evidence hygiene:
  - qsl-protocol governance plus website content/docs/governance only; no qsc/runtime files, no qsl-server changes, no workflows, and no website product/UI behavior changes.

### NA-0197 — Signal-Class Attachment Architecture Program (100 MiB target, design only)

Status: DONE

Problem:
- Current qsc file-transfer limits and architecture are not competitive with Signal-class attachment sizes, and reaching about `100 MiB` is an architecture/design problem rather than a constant bump.

Scope:
- Design/program definition only across qsl-protocol, qsl-server or a dedicated attachment surface, and the qsc client state/persistence model.
- No implementation in this item.

Must protect:
- Fail-closed integrity semantics.
- Honest delivery semantics.
- No "just raise the limits" shortcut before architecture is defined.
- No secret leakage in artifact or operator flows.

Deliverables:
1) Choose the attachment service boundary and ownership model.
2) Define attachment descriptor, integrity, resume, retention, quota, and abuse-control semantics.
3) Define the client streaming and persistence model.
4) Define the validation plan for meaningful large attachment sizes.

Acceptance:
1) Design scope is explicit enough to spawn implementation items.
2) No constant-bump implementation is authorized before the design item is complete.
3) Cross-repo scope and sequencing are clear.

Evidence:
- Implementation PR: #529 https://github.com/QuantumShieldLabs/qsl-protocol/pull/529
- Merge SHA: `aebfdb04bb60`
- mergedAt: `2026-03-15T22:07:50Z`
- Architecture outcomes:
  - The current `<= 4 MiB` file-transfer path is explicitly classified as a bounded legacy message-plane path and not a viable route to the `100 MiB` target.
  - Constant-bump enlargement and relay-inbox-as-blob-plane reuse are explicitly rejected.
  - QSL attachments now have a chosen control-plane / data-plane split: message plane carries an authenticated attachment descriptor; a separate opaque attachment service boundary carries encrypted blob parts with resume/quota/retention semantics.
  - qsl-server remains transport-only and unchanged; the chosen future blob plane is a separate service boundary, even if later co-located operationally.
  - The legacy `<= 4 MiB` path remains temporarily compatible until later validation proves the new path and authorizes a transition decision.
- Design artifacts:
  - `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
  - `tests/NA-0197_attachment_validation_and_rollout_plan.md`
  - Decision `D-0306`
- Evidence hygiene:
  - Docs/governance/evidence only; no `qsl/qsl-client/qsc/src/**`, no `qsl/qsl-client/qsc/tests/**` product/runtime changes, no qsl-server files, no website files, no `.github` files, and no workflows changed.

### NA-0197A — Attachment Descriptor + Control-Plane Contract

Status: DONE

Problem:
- `NA-0197` chose a separate opaque attachment plane, but the message-plane attachment descriptor, transcript binding, reject rules, confirmation handle, and legacy coexistence rules are not yet implementation-grade.
- Both the future attachment service contract and qsc streaming client work depend on this control-plane contract.

Scope:
- qsl-protocol docs/normative contract only.
- No implementation yet.

Must protect:
- No runtime/workflow changes.
- `accepted_by_relay` remains distinct from attachment-plane commit and from `peer_confirmed`.
- No capability-like secrets in canonical URLs.
- Current `<= 4 MiB` legacy path remains unchanged until separately authorized.

Deliverables:
1) define the implementation-grade attachment descriptor and field meanings
2) define transcript binding, reject rules, and peer-confirm linkage
3) define legacy coexistence rules and source-of-truth mappings for later service/client work

Acceptance:
1) descriptor/control-plane contract is explicit enough for service and client implementation items
2) no runtime/workflow/server/website changes occur
3) queue/evidence are updated truthfully

Evidence:
- Implementation PR: #531 https://github.com/QuantumShieldLabs/qsl-protocol/pull/531
- Merge SHA: `5703fea53e9d`
- mergedAt: `2026-03-15T23:02:37Z`
- Closeout path: `M1`
- Canonical doc created:
  - `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- Descriptor/control-plane freeze summary:
  - froze the canonical payload identity as `t = "attachment_descriptor"` and `v = 1`
  - froze the exact transmitted peer-visible field set, local-only/service-only split, transcript-bound compare set, confirmation-handle derivation, and no-capability-in-canonical-URL rule
  - froze the legacy coexistence rules so the current `file_chunk` / `file_manifest` path remains unchanged and distinct from the new attachment descriptor path
- Reject/coexistence summary:
  - `DOC-CAN-005` defines deterministic reject classes for unknown version, missing/invalid fields, inconsistent size/count/commitment shape, mixed legacy/attachment mode, malformed locator/capability placement, expiry/policy violations, and confirmation mismatch
  - reject behavior is fail-closed with no durable completion-state mutation and no retroactive rewrite of `accepted_by_relay`
  - the coexistence matrix is explicit enough that `NA-0197B` can now define the service contract without guessing the control-plane meaning
- Evidence artifacts:
  - `tests/NA-0197A_descriptor_contract_evidence.md`
  - Decision `D-0307`

### NA-0197B — Attachment Service Contract + Governance Promotion

Status: DONE

Problem:
- The chosen attachment architecture requires a separate opaque attachment plane, but the service/session/storage contract and repo-local governance lane for that surface do not yet exist.

Scope:
- qsl-protocol plus repo-local governance preparation for the chosen attachment-surface repo.
- No runtime implementation yet unless a separately authorized smaller step proves safe.

Must protect:
- No plaintext attachments on server surfaces.
- No capability-like secrets in canonical URLs.
- qsl-server remains transport-only.
- `accepted_by_relay` / `peer_confirmed` semantics remain unchanged.

Deliverables:
1) define the attachment-plane API/session/commit/resume/quota/retention contract
2) define operator/logging/metadata invariants for the attachment surface
3) prepare the next repo-local governance promotion for service implementation

Acceptance:
1) service contract is explicit enough to spawn service implementation
2) repo-local governance promotion path is clear
3) no runtime changes occur unless separately authorized by a smaller follow-on

Evidence:
- qsl-protocol implementation PR: #533 https://github.com/QuantumShieldLabs/qsl-protocol/pull/533
- qsl-protocol merge SHA: `a8204c83f151`
- qsl-protocol mergedAt: `2026-03-16T00:05:23Z`
- qsl-attachments bootstrap PR: #1 https://github.com/QuantumShieldLabs/qsl-attachments/pull/1
- qsl-attachments merge SHA: `e8290755c194`
- qsl-attachments mergedAt: `2026-03-16T00:06:48Z`
- Closeout path: `N1`
- Canonical service contract created:
  - `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- Service-contract summary:
  - froze the canonical endpoint family for create/upload/status/commit/abort/retrieval
  - froze the no-secret-in-canonical-URL rule and secret carriage via dedicated request headers plus secret-bearing response bodies
  - froze the session/object state machine, commit/retrieval preconditions, service-side reject taxonomy, and operator/logging/metadata invariants
- Repo bootstrap summary:
  - no pre-existing canonical attachment-surface repo existed under QuantumShieldLabs
  - created public repo `QuantumShieldLabs/qsl-attachments` with AGPL-3.0-only posture via a minimal seed step using GitHub repo creation with initial README and LICENSE
  - repo-local governance PR #1 established `NA-0001 — Attachment Service Runtime Implementation` as the sole READY item in `qsl-attachments`
- Evidence hygiene:
  - docs/governance/bootstrap only; no qsl-protocol runtime files, no qsl-server files, no website files, no `.github` files, and no workflows changed

### NA-0197BA — qsl-attachments Service Runtime Implementation

Status: DONE

Problem:
- The attachment service contract is now frozen, but the opaque encrypted attachment plane does not yet exist as a runtime implementation.

Scope:
- `QuantumShieldLabs/qsl-attachments/**` runtime/service implementation only
- no qsl-protocol runtime changes
- no qsl-server changes

Must protect:
- no plaintext attachments on service surfaces
- no capability-like secrets in canonical URLs
- deterministic session/commit/resume/retrieval rejects
- qsl-server remains transport-only
- control-plane contract from `DOC-CAN-005` remains authoritative

Deliverables:
1) implement the canonical service/session/object lifecycle
2) implement opaque encrypted upload/download/commit/resume
3) implement quota/retention/expiry/abuse controls and deterministic errors
4) add runtime tests proving contract faithfulness

Acceptance:
1) runtime faithfully implements the canonical service contract
2) no secret-bearing URL or plaintext-service leakage occurs
3) queue/evidence are updated truthfully

Evidence:
- qsl-attachments implementation PR: https://github.com/QuantumShieldLabs/qsl-attachments/pull/2
- qsl-attachments closeout PR: https://github.com/QuantumShieldLabs/qsl-attachments/pull/3
- merge SHAs:
  - implementation: `da7400119b2af7a96e635aa8ce6becb1d9931dc4`
  - closeout: `59f632fbdced58c256c543c5739d77d1093d4071`
- mergedAt:
  - implementation: `2026-03-16T01:18:39Z`
  - closeout: `2026-03-16T01:21:09Z`
- runtime/contract-faithfulness outcomes:
  - `QuantumShieldLabs/qsl-attachments` now implements the canonical `DOC-CAN-006` lifecycle: session create/upload/status/commit/abort, committed-object retrieval, valid single-range fetch, deterministic reject codes, expiry/quota/abuse enforcement, opaque ciphertext-only local-disk storage, JSON journal persistence, and secret-bearing header carriage through `X-QATT-Resume-Token` / `X-QATT-Fetch-Capability`
  - deterministic tests cover the commit boundary, no-mutation reject paths, expiry behavior, quota policy, range behavior, audit-log redaction, and canonical-URL query rejection
  - qsl-attachments `main` now has a minimal truthful baseline: `rust` CI plus `main` branch protection requiring the `rust` check
- qsl-server remained untouched and transport-only throughout this item
- Evidence hygiene:
  - qsl-protocol runtime/workflow files were untouched
  - qsl-server files were untouched
  - website and `.github` repos were untouched

### NA-0197CA — Attachment Encryption Context Contract Clarification

Status: DONE

Problem:
- `DOC-ATT-001` requires qsc to derive/load an attachment encryption context for upload and to obtain/derive the corresponding decryption context for download, but the current canonical attachment docs do not yet freeze those semantics to implementation-grade precision.
- Starting `NA-0197C` before that clarification would force qsc to invent protocol behavior rather than implement it.

Scope:
- qsl-protocol canonical docs/governance only
- no qsc runtime implementation
- no qsl-attachments runtime changes
- no qsl-server work

Must protect:
- no runtime/service changes
- no capability-like secrets in canonical URLs
- honest-delivery semantics remain explicit
- qsl-attachments remains opaque and plaintext-free

Deliverables:
1) freeze sender-generated attachment encryption context and receiver acquisition/derivation rules
2) freeze the peer-visible, local-only, and service-only split for encryption-context material
3) freeze decrypt-order and reject/no-mutation rules so `NA-0197C` can proceed without semantic guesswork

Acceptance:
1) qsc/client implementation can proceed without guessing attachment encryption/decryption semantics
2) no runtime/service changes occur
3) queue/evidence are updated truthfully

Evidence:
- governance repair PR: #536 https://github.com/QuantumShieldLabs/qsl-protocol/pull/536
- governance repair merge SHA: `7c0cd66be64f`
- governance repair mergedAt: `2026-03-16T02:39:23Z`
- implementation PR: #537 https://github.com/QuantumShieldLabs/qsl-protocol/pull/537
- implementation merge SHA: `8abb7eed6990`
- implementation mergedAt: `2026-03-16T03:10:55Z`
- Closeout path: `P1`
- Chosen encryption-context strategy:
  - `C1` sender-generated per-attachment encryption context carried only inside the authenticated descriptor as transcript-bound `enc_ctx_alg` plus `enc_ctx_b64u`
- Contract clarification summary:
  - created `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
  - amended `DOC-CAN-005` to freeze the new `enc_ctx_*` descriptor fields, transcript-bound compare set, field split, and decrypt-path reject matrix
  - amended `DOC-CAN-006` to keep the attachment service blind to decrypt-context material and to point ciphertext-shape semantics back to the canonical control-plane docs
  - no new confirmation payload field was added because the transcript-bound descriptor plus `integrity_root` already bind the decrypt context to the existing confirmation linkage
- Decrypt-order / reject-matrix summary:
  - sender now canonically generates fresh attachment context, stages ciphertext locally while computing `integrity_root`, commits the blob, and only then emits the descriptor
  - receiver now canonically authenticates the descriptor, validates and decodes `enc_ctx_*`, fetches and verifies ciphertext shape plus `integrity_root`, decrypts to temp staging, promotes only after exact plaintext-length validation, and only then may emit completion confirmation
  - early confirmation, malformed or unsupported `enc_ctx_*`, ciphertext precheck failure, per-part decrypt authentication failure, and post-decrypt plaintext-shape failure all reject fail-closed with no durable completion mutation
- Evidence hygiene:
  - qsl-protocol runtime/workflow files were untouched
  - qsl-attachments files were untouched
  - qsl-server files were untouched
  - website and `.github` repos were untouched

### NA-0197C — qsc Streaming Attachment Client Implementation

Status: DONE

Problem:
- Current qsc file transfer still assumes whole-file reads, timeline-embedded partial persistence, and in-memory reconstruction, so client behavior must move to streaming/resumable attachment handling after the control-plane and service contracts are fixed.

Scope:
- qsc/client-side streaming, persistence, resume, and UI/logging behavior.

Must protect:
- Fail-closed integrity semantics.
- Honest delivery semantics (`accepted_by_relay` != `peer_confirmed`).
- No secret leakage in logs/markers.
- Legacy path treatment follows the approved transition rules.

Deliverables:
1) implement streaming upload/download and local attachment persistence
2) implement restart-safe resume state and deterministic cleanup/reject behavior
3) preserve explicit delivery-state semantics and UI/logging boundaries

Acceptance:
1) qsc no longer assumes whole-file memory reconstruction for attachment-plane transfers
2) restart/resume/integrity/metadata-log checks pass at the approved size ladder
3) current semantics remain truthful

Evidence:
- qsl-protocol implementation PR: #539 https://github.com/QuantumShieldLabs/qsl-protocol/pull/539
- qsl-protocol merge SHA: `3778f7442d2a`
- qsl-protocol mergedAt: `2026-03-16T14:14:15Z`
- qsl-attachments correction PR set: none
- Closeout path: `O1`
- Client/service integration outcomes:
  - qsc now integrates against the live `qsl-attachments` runtime for sender-side session create/upload/commit, authenticated descriptor send, receiver-side fetch/verify/decrypt/store, restart-safe attachment journal resume, and attachment completion confirmation only after verified local persistence
  - descriptor `enc_ctx_*` handling from `DOC-CAN-007` is implemented on both sender and receiver paths, and attachment/service acceptance remains distinct from relay acceptance and from `peer_confirmed`
  - route-token migration, handshake/on-wire, and identity-at-rest regressions stayed green on the updated branch
- Coexistence rule used:
  - keep the legacy in-message path for `<= 4 MiB`
  - use the attachment service path only above `<= 4 MiB` and only when `--attachment-service` is supplied
- Large-file proof summary:
  - `64 MiB` local end-to-end attachment roundtrip passed in `77.30s`
  - `100 MiB` local end-to-end attachment roundtrip passed in `117.02s`
  - `timeout 300s cargo test -p qsc --locked` was attempted and not counted as a pass because it stalled in `tests/aws_file_medium_boundary_na0192a.rs`
- Evidence hygiene:
  - qsl-attachments remained unchanged and no repo-local correction item was needed
  - qsl-server remained untouched and transport-only
  - no website, `.github`, or workflow files changed

### NA-0198 — Suite-2 / True Triple Ratchet Hardening + Equivalence

Status: DONE

Problem:
- The attachment program and its immediate implementation follow-ons are complete enough that the next load-bearing work returns to protocol/runtime hardening and equivalence.

Scope:
- qsl-protocol runtime/integration/hardening only
- no qsl-server work
- no website/.github work
- no attachment architecture redesign

Must protect:
- no wire/protocol/crypto semantic drift without proof
- no regression to attachment, route-token, or current fail-closed behavior

Deliverables:
1) harden Suite-2 / True Triple Ratchet runtime/integration behavior
2) add/refresh equivalence and regression evidence
3) close any remaining protocol-runtime confidence gaps

Acceptance:
1) protocol/runtime hardening work is explicit enough to execute next
2) attachment and existing transport behaviors remain intact
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #541 https://github.com/QuantumShieldLabs/qsl-protocol/pull/541
- qsl-protocol merge SHA: `e2da830cc6e4`
- qsl-protocol mergedAt: `2026-03-17T01:41:18Z`
- qsl-attachments correction PR set: none
- Closeout path: `Q1`
- Hardening/equivalence outcomes:
  - qsc now has a direct deterministic Suite-2 runtime equivalence test that compares real qsc send/receive behavior against canonical refimpl wire/state expectations on both sides of a live roundtrip
  - shared-temp-root test hardening removed a full-suite parallel flake from `ratchet_durability_na0155`, `outbox_abort`, `relay_drop_no_mutation`, and `relay_dup_no_mutation` without changing runtime semantics
  - ordinary runtime confidence is now grounded by a successful full local `cargo test -p qsc --locked` pass in addition to the targeted route-token, attachment, fail-closed, and identity-at-rest regressions
- Coexistence rule used:
  - keep the legacy in-message path for `<= 4 MiB`
  - use the attachment service path only above `<= 4 MiB` and only when `--attachment-service` is supplied
- Large-file/coexistence proof summary:
  - `64 MiB` local service-backed end-to-end attachment roundtrip passed in `80.22s`
  - `100 MiB` local service-backed end-to-end attachment roundtrip passed in `116.63s`
  - legacy-path coexistence regression remained green in `attachment_streaming_na0197c`
  - full local `cargo test -p qsc --locked` passed in `1164.16s`
- Evidence hygiene:
  - qsl-attachments remained unchanged and no repo-local correction item was needed
  - qsl-server remained untouched and transport-only
  - no website, `.github`, or workflow files changed

### NA-0199 — Legacy Attachment Path Transition + Validation

Status: DONE

Problem:
- The attachment path and legacy <=4 MiB path now coexist truthfully, but the project still needs an explicit transition/validation decision for the long-term fate of the legacy path.

Scope:
- qsl-protocol + qsl-attachments only as needed for transition validation and cutover decision
- no qsl-server work
- no website/.github work

Must protect:
- no silent break of <=4 MiB legacy flows
- no false peer_confirmed
- no secret-bearing URLs/logs
- qsl-server remains transport-only

Deliverables:
1) validate the current coexistence rule against the integrated runtime
2) define the long-term legacy-path transition/cutover decision with proof
3) identify any final attachment-lane correctness fixes needed for that transition

Acceptance:
1) legacy transition is explicit and truthful
2) queue/evidence are updated truthfully
3) attachment and current transport behaviors remain intact

Evidence:
- qsl-protocol implementation PR: #543 https://github.com/QuantumShieldLabs/qsl-protocol/pull/543
- qsl-protocol merge SHA: `9480d5132ac3`
- qsl-protocol mergedAt: `2026-03-17T03:30:21Z`
- qsl-attachments correction PR set: none
- Transition decision: `R1`
- Coexistence-validation summary:
  - direct qsc validation now proves that `<= 4 MiB` sends stay on the legacy in-message path even when `--attachment-service` is configured, and that an exact-threshold `4 MiB` send also stays on the legacy path
  - above-threshold sends now have direct proof that they reject with `attachment_service_required` when `--attachment-service` is absent and use the service-backed attachment path when it is supplied
  - legacy-path reject handling now has direct proof that no durable receive mutation occurs on reject and that no false `peer_confirmed` can be emitted before a later successful receive and confirmation
  - attachment-path success, route-token migration integrity, and secret-safe output/URL behavior remained green in the targeted regression set
- Large-file / threshold / legacy proof summary:
  - `24 KiB` legacy-path roundtrip success, reject/no-mutation, and later truthful `peer_confirmed` were proved directly
  - exact `4 MiB` threshold send stayed on the legacy path
  - `4 MiB + 1` send rejected cleanly without `--attachment-service` and succeeded on the attachment path when configured
  - full local `cargo test -p qsc --locked` passed in `1539.76s`
- Evidence hygiene:
  - qsl-attachments remained unchanged and no repo-local correction item was needed
  - qsl-server remained untouched and transport-only
  - no website, `.github`, or workflow files changed

### NA-0200 — qsl-attachments Deployment / Operational Hardening Contract

Status: DONE

Problem:
- Current coexistence is now validated truthfully, but the next blocker to any default-path promotion or legacy deprecation is that `qsl-attachments` still only has a single-node local-disk runtime plus minimal CI/protection posture and no explicit deployment/operational hardening contract.

Scope:
- qsl-protocol + qsl-attachments only as needed for deployment/operational hardening contract and readiness-ladder definition
- no qsl-server work
- no website/.github work

Must protect:
- no silent break of `<= 4 MiB` legacy flows
- no false peer_confirmed
- no secret-bearing URLs/logs
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) define the deployment / operational hardening contract and readiness ladder for qsl-attachments
2) specify the storage, retention, observability, recovery, and rollout expectations required before default-path promotion
3) identify any final attachment-lane operational or correctness fixes required for safe promotion

Acceptance:
1) the operational blocker is explicit and truthful
2) queue/evidence are updated truthfully
3) coexistence and current transport behaviors remain intact

Evidence:
- qsl-protocol implementation PR: #545 https://github.com/QuantumShieldLabs/qsl-protocol/pull/545
- qsl-protocol merge SHA: `91d8678b0bcb56361881da45c7b1ff1bc7dca758`
- qsl-protocol mergedAt: `2026-03-17T12:25:19Z`
- qsl-attachments PR set:
  - promotion PR: #4 https://github.com/QuantumShieldLabs/qsl-attachments/pull/4
  - implementation PR: #5 https://github.com/QuantumShieldLabs/qsl-attachments/pull/5
  - closeout PR: #6 https://github.com/QuantumShieldLabs/qsl-attachments/pull/6
  - promotion merge SHA: `0c6fa760364dda6b57c33eafed3bb3aee0e23c55`
  - implementation merge SHA: `45550eb325e96803ef6041a81f89098346757c93`
  - closeout merge SHA: `8eadc9652d2c5a8081f1335c96b0ae83fd17ed08`
  - promotion mergedAt: `2026-03-17T12:02:38Z`
  - implementation mergedAt: `2026-03-17T12:08:33Z`
  - closeout mergedAt: `2026-03-17T12:11:23Z`
- operational-contract/readiness-ladder summary:
  - `DOC-ATT-002` now freezes three deployment profiles, readiness categories, a constrained-host validation ladder, and explicit saturation-vs-correctness interpretation rules for the attachment lane
  - qsl-attachments now carries aligned repo-local docs that state the current single-node local-disk runtime truthfully, keep the repo docs/governance-only in this item, and define `NA-0003` as the next repo-local execution lane
  - default-path promotion and legacy `<= 4 MiB` deprecation remain explicitly blocked until the constrained-host ladder is executed truthfully against deployed `qsl-attachments` plus the real relay and separated from stronger reference-profile evidence
- constrained-host validation requirements:
  - real-world validation must treat weak service hosts and weak relays, especially the AWS relay, as first-class inputs
  - the ladder must capture CPU, memory, disk, latency, throughput, retries, and backpressure
  - bounded degradation under weak hardware is acceptable only when it is classified honestly as saturation rather than protocol correctness failure
- closeout path: `S1`

### NA-0200AA — Real Relay Deployment Compatibility Restoration

Status: DONE

Problem:
- The live relay deployment at `https://qsl.ddnsfree.com` is stale relative to current qsc/qsl-server canonical relay semantics: the deployed host still serves the legacy path-token pull shape while current qsc uses only the canonical header-based `/v1/push` and `/v1/pull?max=N` endpoints, so NA-0200A cannot execute truthfully until the real relay is restored.

Scope:
- qsl-server deployment drift restoration + qsl-protocol governance only
- no qsl-protocol runtime/client changes
- no qsl-attachments runtime changes
- no qsl-server semantic redesign

Must protect:
- no qsl-server semantic drift
- no qsl-protocol runtime/client drift
- no qsl-attachments runtime changes
- no secrets in probes, evidence, logs, or queue text
- qsl-server remains transport-only

Deliverables:
1) repair the qsl-protocol queue so the external blocker is explicit
2) restore the live relay to the current canonical header-based API already implemented on qsl-server main
3) hard-code a fail-fast relay compatibility preflight/guard into qsl-server project docs/scripts so future real-world validation cannot start against a stale deployment unnoticed
4) restore NA-0200A as the sole READY item once the live relay matches current canonical expectations again

Acceptance:
1) qsl-protocol queue truthfully reflects the external blocker during repair and returns to exactly one READY item after closeout
2) the real relay serves the canonical header-based API expected by qsc
3) qsl-server project files encode a fail-fast compatibility preflight
4) qsl-attachments remains unchanged

Evidence:
- qsl-protocol governance-repair PR: #547 https://github.com/QuantumShieldLabs/qsl-protocol/pull/547
- qsl-server PR set:
  - promotion PR: #41 https://github.com/QuantumShieldLabs/qsl-server/pull/41
  - implementation PR: #42 https://github.com/QuantumShieldLabs/qsl-server/pull/42
  - closeout PR: #43 https://github.com/QuantumShieldLabs/qsl-server/pull/43
- qsl-protocol repair merge SHA: `61619b6593dd58e18db0ad34a2f680ecd1474ea4`
- qsl-server promotion merge SHA: `d567215a960eceb08aab3a276b9195f222ebc76c`
- qsl-server implementation merge SHA: `d66ea363bf6a45f550114c52b98b3f750b166c6c`
- qsl-server closeout merge SHA: `60ea496c8149697f9ed1d214bbd5f6676ab1d430`
- qsl-protocol repair mergedAt: `2026-03-18T02:06:34Z`
- qsl-server promotion mergedAt: `2026-03-18T02:10:13Z`
- qsl-server implementation mergedAt: `2026-03-18T02:26:41Z`
- qsl-server closeout mergedAt: `2026-03-18T02:28:47Z`
- Relay deployment restoration outcomes:
  - qsl-protocol queue repair made the external blocker explicit first by promoting `NA-0200AA` and marking `NA-0200A` blocked pending relay restoration
  - qsl-server `main` already implemented the canonical header-based relay API, so the blocker was deployment drift only
  - the live relay at `https://qsl.ddnsfree.com` now serves the canonical `/v1/push` and `/v1/pull?max=N` API expected by qsc while preserving legacy compatibility truthfully during the migration window
  - a fresh headless `qsc` relay-test now authenticates successfully against the restored real relay
- Hard-coded compatibility-guard summary:
  - qsl-server now carries `scripts/check_relay_compatibility.sh` as a fail-fast canonical/legacy probe
  - `scripts/verify_remote.sh` and `scripts/aws_update_and_verify.sh` now run the compatibility preflight before real-world validation can proceed
  - `scripts/ci/test_relay_deploy_compatibility_guard.sh` now proves the exact `legacy_only_deploy` failure code against a mocked stale relay
- qsl-attachments state:
  - unchanged in this item; `NA-0003` remains the repo-local operational lane

### NA-0200A — qsl-attachments Operational Hardening Implementation + Constrained-Host Real-World Validation

Status: DONE

Problem:
- The operational hardening/readiness contract is now frozen, but the qsl-attachments runtime has not yet been exercised and hardened against the constrained-host real-world validation ladder needed before any default-path promotion or legacy deprecation.

Scope:
- qsl-attachments runtime/ops/docs as needed for deployment/operational hardening and real-world validation
- qsl-protocol evidence/governance only as needed
- no qsl-server work

Must protect:
- no plaintext on service surfaces
- no capability-like secrets in canonical URLs
- no silent break of <=4 MiB legacy flows
- saturation vs correctness must be classified honestly
- qsl-server remains transport-only

Deliverables:
1) implement the operational hardening controls from the contract
2) execute the constrained-host real-world validation ladder over deployed qsl-attachments + real relay
3) record CPU/memory/disk/retry/latency evidence and classify failures correctly
4) determine whether default-path promotion / legacy deprecation is justified or still blocked

Acceptance:
1) readiness ladder is executed truthfully
2) operational evidence is recorded
3) queue/evidence updated truthfully

Evidence:
- qsl-attachments implementation PR: #7 https://github.com/QuantumShieldLabs/qsl-attachments/pull/7
- qsl-attachments closeout PR: #8 https://github.com/QuantumShieldLabs/qsl-attachments/pull/8
- qsl-attachments implementation merge SHA: `2d69abd084dd8918a0092385a92fcf56a8a6748b`
- qsl-attachments closeout merge SHA: `61d576ee56c05de2c89c0fddf357139c6540d2dd`
- qsl-attachments implementation mergedAt: `2026-03-18T22:58:25Z`
- qsl-attachments closeout mergedAt: `2026-03-18T23:01:00Z`
- operational-hardening summary: qsl-attachments now enforces storage-headroom reserve rejects before disk exhaustion on weak hosts, emits operator-safe startup configuration logs, and carries a `101 MiB` ciphertext ceiling so the `100 MiB` target class remains truthful after part-cipher overhead.
- constrained-host ladder summary: direct evidence now covers `< 4 MiB` legacy-path success, exact `4 MiB` legacy-path weak-relay saturation with fail-closed bounded retries, `> 4 MiB` missing-service reject, `> 4 MiB` service-backed success, `16 MiB` / `64 MiB` / `100 MiB` service-backed success, upload-resume success, direct service-restart success, direct API quota/session/object-expiry rejects, and limited concurrency success over the restored real relay plus deployed single-node service on `qsl`.
- saturation-vs-correctness summary: no qsl-attachments correctness failure was proven in the required ladder; the exact `4 MiB` queue-full result was bounded weak-relay saturation with the service path idle, while a stricter exploratory receive-abort composite exposed a client-side confirm issue outside the minimum required service ladder.
- closeout path: `U1`

### NA-0201 — Reference Deployment Validation + Default Attachment Path Promotion Decision

Status: DONE

Problem:
- Constrained-host validation is now directly grounded, but the project still lacks broader reference-deployment evidence and a promotion-gate decision strong enough to justify any default attachment-path promotion or legacy `<= 4 MiB` deprecation.

Scope:
- qsl-attachments runtime/ops/docs as needed for stronger reference-deployment validation and promotion-gate evidence
- qsl-protocol evidence/governance only as needed
- no qsl-server source changes

Must protect:
- no plaintext on service surfaces
- no capability-like secrets in canonical URLs
- no silent break of `<= 4 MiB` legacy flows
- saturation vs correctness must be classified honestly across constrained and reference profiles
- qsl-server remains transport-only

Deliverables:
1) execute the validation ladder against a stronger reference deployment profile while comparing results against the constrained-host evidence from `NA-0200A`
2) record promotion-gate evidence for relay/service behavior, resource ceilings, and operational maturity across deployment profiles
3) determine whether default attachment-path promotion above threshold is justified or still blocked
4) determine whether legacy `<= 4 MiB` deprecation is justified or still blocked

Acceptance:
1) reference-deployment evidence is recorded truthfully
2) promotion/deprecation decision is explicit and evidence-backed
3) queue/evidence updated truthfully

Evidence:
- qsl-attachments promotion PR: #9 https://github.com/QuantumShieldLabs/qsl-attachments/pull/9
- qsl-attachments implementation PR: #10 https://github.com/QuantumShieldLabs/qsl-attachments/pull/10
- qsl-attachments closeout PR: #11 https://github.com/QuantumShieldLabs/qsl-attachments/pull/11
- qsl-attachments promotion merge SHA: `71e0d3be68389227aab9abd12e96b42762d7f0d7`
- qsl-attachments implementation merge SHA: `3d3f1b6591180763cda020a35b684713bc58cc2b`
- qsl-attachments closeout merge SHA: `e12478d3c8486ee3ae6712cf90d4112eb47008de`
- qsl-attachments promotion mergedAt: `2026-03-19T01:21:10Z`
- qsl-attachments implementation mergedAt: `2026-03-19T03:04:24Z`
- qsl-attachments closeout mergedAt: `2026-03-19T03:06:47Z`
- stronger reference deployment summary: `qatt` provided a materially stronger reference profile than constrained-host `qsl` (`4` vCPU, `~16 GiB` RAM, `~100 GiB` root volume observed) while preserving the single-node local-disk `qsl-attachments` runtime on loopback behind Caddy TLS at `https://qatt.ddnsfree.com`; qsl-server source stayed untouched, and the hard-coded relay compatibility guard was rerun successfully before validation began.
- mixed message + attachment validation summary: direct evidence now covers message-only relay traffic, service-backed `5 MiB` / `16 MiB` / `64 MiB` / `100 MiB` attachment runs, mixed `16 MiB` message + attachment traffic, upload interruption-resume, direct `qsl-attachments` restart, bounded concurrency with two parallel mixed peers, and a five-iteration mixed short soak over the real relay plus the stronger `qatt` deployment.
- saturation-vs-correctness summary: no qsl-attachments correctness failure was proven on the stronger reference deployment; the only degraded threshold stages remained on the weak relay / legacy path, where corrected `< 4 MiB` timed out while still making forward progress and exact `4 MiB` failed closed with bounded `relay_inbox_queue_full` retries while `qatt` remained effectively idle. This is enough to rule out reference-host immaturity as the next blocker, but not enough to justify default-path promotion or legacy `<= 4 MiB` deprecation without a broader kitchen-sink stress/soak/chaos lane.
- qsl-protocol runtime correction summary: no qsl-protocol runtime/client correction PR was needed for `NA-0201`.
- closeout path: `X1`

### NA-0201A — Message + Attachment Stress / Soak / Chaos Validation

Status: DONE

Problem:
- The stronger reference deployment now works well enough that the honest next blocker is no longer basic operational readiness on `qsl-attachments`, but a broader mixed message + attachment stress/soak/chaos campaign needed before any default attachment-path promotion or legacy `<= 4 MiB` deprecation decision can be made honestly.

Scope:
- qsl-attachments runtime/ops/docs as needed for broader mixed message + attachment stress/soak/chaos validation
- qsl-protocol evidence/governance only as needed
- no qsl-server source changes

Must protect:
- no plaintext on service surfaces
- no capability-like secrets in canonical URLs
- no silent break of `<= 4 MiB` legacy flows
- saturation vs correctness must be classified honestly under mixed stress/soak/chaos load
- qsl-server remains transport-only

Deliverables:
1) execute a broader mixed message + attachment stress/soak/chaos campaign across constrained and reference deployment profiles
2) capture CPU/memory/disk/retry/backpressure/latency/recovery evidence under longer and more varied load
3) identify any correctness or operational blockers that still preclude default attachment-path promotion or legacy `<= 4 MiB` deprecation
4) determine whether the default-path / legacy decision is justified after kitchen-sink evidence or still blocked

Acceptance:
1) mixed stress/soak/chaos evidence is recorded truthfully
2) saturation/correctness/deployment-immaturity classification is explicit and honest
3) queue/evidence updated truthfully

Evidence:
- qsl-attachments promotion PR: https://github.com/QuantumShieldLabs/qsl-attachments/pull/12
- qsl-attachments implementation PR: https://github.com/QuantumShieldLabs/qsl-attachments/pull/13
- qsl-attachments closeout PR: https://github.com/QuantumShieldLabs/qsl-attachments/pull/14
- qsl-protocol runtime correction PR: none
- qsl-attachments promotion merge SHA: `c929aac10015888a3ce969066c9434ba4a5a8eca`
- qsl-attachments implementation merge SHA: `72fd5441262626ef644b6877ad8a0e29bf275583`
- qsl-attachments closeout merge SHA: `aa5d59d86ecd08501d62766e6a017008e9adf4ea`
- qsl-attachments promotion mergedAt: `2026-03-20T02:55:18Z`
- qsl-attachments implementation mergedAt: `2026-03-20T11:33:47Z`
- qsl-attachments closeout mergedAt: `2026-03-20T11:36:56Z`
- mixed message + attachment validation summary: the bounded kitchen-sink lane now includes message-only relay traffic, mixed `5 MiB` exchanges on weak-host `qsl` and stronger reference-host `qatt`, service-backed `16 MiB` / `64 MiB` / `100 MiB` reference-host transfers, restart and resumed-upload recovery on `qatt`, mixed concurrency ramps on `qsl` and `qatt`, and a `30` minute mixed soak on `qatt`.
- weak-host versus reference-host summary: `qsl` remained the weak-host / weak-relay baseline and continued to expose bounded threshold-path degradation, while `qatt` remained the stronger reference deployment and stayed bounded through large files, restart/recovery, concurrency up to `8`, and the `30` minute mixed soak window.
- saturation-vs-correctness summary: no qsl-attachments correctness failure or load-bearing deployment immaturity was proven; the only degraded required stages stayed on the weak-host / weak-relay legacy threshold path (`< 4 MiB` and exact `4 MiB`) as explicit bounded `relay_inbox_queue_full` saturation that failed closed without dishonest delivery state, so the next blocker is the actual default attachment-path / legacy decision rather than more kitchen-sink validation.
- closeout path: `Z1`

### NA-0202 — Default Attachment Path Promotion + Legacy In-Message Deprecation Decision

Status: DONE

Problem:
- The bounded kitchen-sink lane is now grounded across weak-host `qsl` and stronger reference-host `qatt`, so the honest next blocker is no longer operational validation but the actual decision about whether to promote the attachment service path by default and whether legacy in-message file carriage at `<= 4 MiB` should remain, narrow, or be deprecated.

Scope:
- qsl-protocol and qsl-attachments evidence/governance/docs as needed for the promotion / deprecation decision
- minimal runtime/client/service changes only if the decision itself requires a narrowly scoped implementation follow-on
- no qsl-server source changes

Must protect:
- no plaintext on service surfaces
- no capability-like secrets in canonical URLs
- no silent break of `<= 4 MiB` legacy flows
- weak-host bounded saturation must not be misclassified as a protocol or attachment-service correctness defect
- qsl-server remains transport-only

Deliverables:
1) decide whether attachment service should become the default path above threshold based on the combined coexistence, constrained-host, reference-host, and kitchen-sink evidence
2) decide whether legacy in-message file carriage at `<= 4 MiB` should remain as-is, narrow, or move toward deprecation
3) identify the smallest truthful implementation/governance follow-on implied by that decision
4) record the decision and queue/evidence updates truthfully

Acceptance:
1) the default attachment-path decision is explicit and evidence-backed
2) the legacy in-message decision is explicit and evidence-backed
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation/design PR: #552 https://github.com/QuantumShieldLabs/qsl-protocol/pull/552
- qsl-protocol implementation/design merge SHA: `62557eb70e16c63c5a01938e9b33425f33f111a2`
- qsl-protocol implementation/design mergedAt: `2026-03-21T00:22:27Z`
- decision summary:
  - `DOC-ATT-003` now freezes that the accumulated coexistence, constrained-host, stronger reference-host, and bounded kitchen-sink evidence is strong enough to justify default attachment-path promotion only above the `4 MiB` threshold in validated deployments
  - the same evidence does not justify replacing or deprecating the validated legacy `<= 4 MiB` in-message path
  - no silent fallback from an above-threshold attachment attempt to the legacy path is authorized
- exact chosen policy result: `D1` — promote attachment path by default above threshold in validated deployments; legacy `<= 4 MiB` remains
- exact weak-host versus reference-host evidence summary:
  - weak-host `qsl` remained the weak-host / weak-relay baseline and continued to expose only bounded legacy-threshold saturation at `< 4 MiB` and exact `4 MiB`
  - stronger reference-host `qatt` stayed bounded through service-backed `5 MiB` / `16 MiB` / `64 MiB` / `100 MiB`, mixed traffic, restart/recovery, concurrency up to `8`, and a `30` minute soak
  - no qsl-attachments correctness failure or load-bearing deployment immaturity was proven on the stronger reference profile
- exact reason legacy deprecation remains blocked:
  - the `DOC-ATT-002` deprecation gates are still unmet because there is not yet an explicit migration plan, explicit rollback plan, proof of no silent break for legacy-sized flows during the migration window, or explicit operator-visible fallback behavior
- closeout path: `AB1`

### NA-0202A — qsc Default Attachment Path Promotion Above Threshold

Status: DONE

Problem:
- `DOC-ATT-003` now freezes that validated deployments have enough evidence to make the attachment path the default only above the `4 MiB` threshold, but current qsc still requires explicit `--attachment-service` on those sends instead of treating validated deployment configuration as the default selection rule.

Scope:
- qsc runtime/tests/docs as needed to promote the attachment path by default above `4 MiB` in validated deployments
- no qsl-attachments runtime changes
- no qsl-server changes
- no legacy `<= 4 MiB` deprecation implementation

Must protect:
- no silent break of `<= 4 MiB` legacy flows
- no silent fallback from above-threshold attachment sends to the legacy path
- no capability-like secrets in canonical URLs
- honest delivery semantics remain distinct and truthful
- qsl-server remains transport-only

Deliverables:
1) implement default attachment-path selection for `> 4 MiB` sends in validated deployments
2) make per-send `--attachment-service` no longer mandatory in validated deployments while preserving it as an explicit override/diagnostic surface if needed
3) keep the `<= 4 MiB` legacy path explicitly unchanged
4) prove fail-closed behavior when validated attachment-service configuration is absent or the attachment path rejects

Acceptance:
1) above-threshold sends use the attachment path by default in validated deployments
2) `<= 4 MiB` legacy behavior remains explicit and unchanged
3) no silent fallback or honest-delivery drift is introduced

Evidence:
- qsl-protocol implementation PR: #554 https://github.com/QuantumShieldLabs/qsl-protocol/pull/554
- qsl-protocol implementation head SHA: `aca22a7a05b9bf79f8080d5b2f73ee56f34fcf73`
- qsl-protocol implementation merge SHA: `922bb45056177e92ba40bc0e42a28959d71289de`
- qsl-protocol implementation mergedAt: `2026-03-21T23:47:09Z`
- exact validated-deployment config decision:
  - explicit `--attachment-service` remains the highest-precedence override and diagnostic surface
  - `QSC_ATTACHMENT_SERVICE` is the operator-controlled validated deployment configuration
  - if neither is present for a `> 4 MiB` send, qsc now fails closed explicitly with `attachment_service_required`
- exact selection/no-silent-fallback summary:
  - qsc now selects the attachment path before send start for files strictly larger than `4 MiB` when either the explicit override or validated deployment configuration is present
  - exact `4 MiB` and smaller files remain on the legacy in-message path unchanged
  - attachment-path start or reject failures do not silently retry the legacy path
  - honest delivery semantics and route-token migration remained intact under the refreshed deterministic regression set
- exact narrow advisory remediation summary:
  - the workflow-pinned advisories lane was reproduced exactly on both current `main` and the PR #554 branch with Rust `1.84.0` and `cargo audit --deny warnings`
  - the failure was identical on both branches: `RUSTSEC-2026-0049` on `rustls-webpki 0.103.9`
  - the minimal safe remediation was lockfile-only: `Cargo.lock` now resolves `rustls-webpki 0.103.10`
  - no advisory ignore was added and no manifest churn was introduced
- closeout path: `AC1`

### NA-0202B — Legacy In-Message Deprecation Readiness

Status: DONE

Problem:
- Default promotion above threshold is now justified as the next blocker, but legacy `<= 4 MiB` deprecation still lacks the migration/rollback/no-silent-break readiness evidence required by `DOC-ATT-002` and `DOC-ATT-003`.

Scope:
- qsl-protocol, qsc, and qsl-attachments docs/evidence/governance as needed to prove legacy-deprecation readiness
- no immediate legacy-removal implementation
- no qsl-server changes

Must protect:
- no silent break of `<= 4 MiB` legacy flows
- explicit operator-visible fallback behavior during any migration window
- no capability-like secrets in canonical URLs
- qsl-server remains transport-only

Deliverables:
1) define the migration plan for legacy `<= 4 MiB` deprecation
2) define the rollback plan if promotion/deprecation rollout misbehaves
3) prove no silent break for legacy-sized flows during the migration window
4) define any remaining fallback behavior so it is explicit and operator-visible

Acceptance:
1) legacy-deprecation gates are explicit and evidence-backed
2) migration/rollback/fallback behavior is explicit
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation/design PR: #556 https://github.com/QuantumShieldLabs/qsl-protocol/pull/556
- qsl-protocol implementation/design merge SHA: `62134df3e6dd7541919b39c52784d79c98af7dbb`
- qsl-protocol implementation/design mergedAt: `2026-03-22T15:30:44Z`
- chosen readiness result: `R1`
- migration / rollback / fallback summary:
  - `DOC-ATT-004` now freezes a staged migration window with `W0` current coexistence, `W1` attachment-first canary for legacy-sized sends, and `W2` legacy send-path deprecation completion
  - rollback is frozen as a configuration-only return to `W0` for new legacy-sized sends without requiring qsl-server or qsl-attachments rollback
  - automatic fallback from an attachment attempt to the legacy send path remains forbidden; any temporary legacy send use during migration must be explicit and operator-selected
- exact no-silent-break summary:
  - current evidence already proves deterministic path selection, truthful delivery milestones, and explicit fail-closed behavior on the current legacy and attachment path families
  - `DOC-ATT-004` now freezes the additional implementation-time proof obligations for legacy-sized attachment-first sends, receive compatibility during migration, and rollback restoration
- exact reason implementation is now justified:
  - the remaining blocker is no longer a smaller governance or evidence-finalization gap; it is now the actual qsc implementation of the staged migration/rollback surface and its deterministic proof obligations
- no qsl-attachments repo-local follow-on required for `NA-0202B`
- closeout path: `AD1`

### NA-0203 — Legacy In-Message Deprecation Implementation

Status: DONE

Problem:
- `NA-0202B` froze the staged readiness boundary for legacy `<= 4 MiB` deprecation, so the next blocker is now the actual qsc implementation of the attachment-first migration window with explicit rollback and no-silent-break proof.

Scope:
- `qsl/qsl-client/qsc/**` runtime/tests/docs as needed to implement the staged legacy-sized migration window frozen by `DOC-ATT-004`
- qsl-protocol docs/governance/evidence as needed
- no qsl-attachments runtime changes unless a concrete new correctness defect is proven
- no qsl-server changes

Must protect:
- no silent break of `<= 4 MiB` legacy flows
- no silent fallback from an attachment attempt to the legacy send path
- explicit operator-visible migration and rollback behavior
- no capability-like secrets in canonical URLs
- qsl-server remains transport-only

Deliverables:
1) add the explicit operator-controlled migration and rollback surface frozen by `DOC-ATT-004`
2) implement `W1` attachment-first send selection for legacy-sized files in validated deployments without removing receive compatibility for either path family
3) add deterministic tests proving exact-threshold behavior, no silent fallback, truthful delivery milestones, receive compatibility during migration, and rollback restoration to `W0`
4) update operator-facing qsc documentation/runbook surfaces and queue/evidence truthfully

Acceptance:
1) legacy-sized attachment-first sends work only under explicit migration mode in validated deployments
2) rollback restores `W0` for new legacy-sized sends without requiring qsl-server or qsl-attachments rollback
3) no silent fallback or dishonest delivery behavior is introduced
4) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #558 https://github.com/QuantumShieldLabs/qsl-protocol/pull/558
- qsl-protocol implementation merge SHA: `5fd59985bd1af27bc53ae79355f56947eb1b3da9`
- qsl-protocol implementation mergedAt: `2026-03-23T03:46:53Z`
- exact migration-window summary:
  - `w0` remains the current coexistence baseline and the configuration-only rollback target for new legacy-sized sends
  - `w1` moves new legacy-sized sends to the attachment path only when the operator explicitly selects the migration stage
  - `> 4 MiB` validated-deployment attachment-first selection from `NA-0202A` remains unchanged
- exact override / fallback / rollback summary:
  - `QSC_LEGACY_IN_MESSAGE_STAGE` and `--legacy-in-message-stage` are the operator-visible migration controls for legacy-sized sends
  - validated deployment attachment configuration continues through `QSC_ATTACHMENT_SERVICE` and `--attachment-service`
  - if attachment selection is active without validated attachment-service configuration, qsc fails closed with `attachment_service_required`
  - attachment-path failures do not retry the legacy path silently, and operators roll back by returning the stage to `w0`
- closeout path: `AE1`

### NA-0203A — Legacy Deprecation Validation + Cleanup

Status: DONE

Problem:
- `NA-0203` implemented the staged `W0`/`W1` migration-window controls in qsc, so the next blocker is validating the merged migration lane and cleaning up any remaining repo-local test/runbook/evidence gaps before any future `W2` legacy-send removal planning.

Scope:
- `qsl/qsl-client/qsc/**` tests/docs/evidence as needed to validate merged `W0`/`W1` behavior and clean up migration-window artifacts
- qsl-protocol governance/evidence as needed
- no qsl-attachments runtime changes unless a concrete new correctness defect is proven
- no qsl-server changes

Must protect:
- no semantic drift from the frozen `W0`/`W1` migration policy
- no silent fallback and no silent break for legacy-sized sends
- honest delivery semantics and route-token header carriage remain unchanged
- qsl-server remains transport-only

Deliverables:
1) run and record post-merge validation for `W0` rollback, `W1` canary sends, and mixed receive compatibility on the supported lanes that matter for the migration window
2) close any remaining deterministic qsc test/runbook cleanup discovered during the implementation and CI stabilization passes
3) prove the merged migration-window lane is clean enough that the next blocker after this item is not another direct `W1` implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms `W0` rollback, `W1` canary behavior, and no-silent-fallback behavior on the required supported lanes
2) migration-window-local deterministic test and operator-runbook cleanup is complete
3) no protocol, relay, or attachment-service semantic change is introduced
4) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #560 https://github.com/QuantumShieldLabs/qsl-protocol/pull/560
- qsl-protocol implementation merge SHA: `f0d9b9e8675f`
- qsl-protocol implementation mergedAt: `2026-03-24T04:09:14Z`
- exact cleanup / validation summary:
  - qsc runtime semantics remained unchanged from `NA-0203`; PR #560 touched only qsc runbooks and deterministic tests
  - the required qsc local validation matrix stayed green for `cargo fmt -p qsc -- --check`, `cargo clippy -p qsc --all-targets -- -D warnings`, `cargo build -p qsc --release --locked`, the targeted migration-window/auth/config/on-wire suites, and full `cargo test -p qsc --locked`
  - stale legacy-only wording was removed so the local and remote operator runbooks now state truthfully that `w0` / `w1` only affect new legacy-sized sends while validated `> 4 MiB` attachment-first behavior from `NA-0202A` remains unchanged
- exact operator-visible migration / fallback / rollback summary:
  - `QSC_LEGACY_IN_MESSAGE_STAGE` and `--legacy-in-message-stage` remain the explicit `w0` / `w1` controls for legacy-sized sends
  - rollback remains a configuration-only return to `w0` for new legacy-sized sends
  - validated deployment attachment configuration remains explicit through `QSC_ATTACHMENT_SERVICE` and `--attachment-service`, and missing validated configuration still fails closed with `attachment_service_required`
  - attachment-path failures do not silently retry the legacy path, and no false `peer_confirmed` or route-token regression was introduced under the refreshed deterministic test set
- closeout path: `AF1`

### NA-0204 — Legacy Deprecation Final Removal Decision

Status: DONE

Problem:
- `NA-0203A` finished validation and cleanup of the merged `W0` / `W1` migration window, so the next blocker is no longer another migration-window cleanup pass but the actual decision about whether the accumulated evidence is now strong enough to authorize `W2` legacy send-path removal planning.

Scope:
- qsl-protocol governance/docs/evidence as needed to decide whether final legacy send-path removal is justified
- read-only qsl-attachments and qsl-server evidence/queue posture as needed for the decision
- minimal successor-lane definition only; no legacy-removal runtime implementation in this item
- no qsl-server changes

Must protect:
- no semantic drift from the frozen `W0` / `W1` migration policy before any explicit final-removal decision
- no silent fallback and no silent break for legacy-sized sends
- honest delivery semantics and route-token header carriage remain unchanged
- qsl-server remains transport-only

Deliverables:
1) decide whether the validated `W0` / `W1` migration-window evidence is sufficient to authorize final legacy send-path removal planning
2) if authorized, define the smallest truthful `W2` / removal implementation lane; otherwise define the smallest remaining blocker truthfully
3) record the decision and queue/evidence updates truthfully

Acceptance:
1) the final legacy-removal decision is explicit and evidence-backed
2) the smallest truthful successor lane after the decision is explicit
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation/design PR: #562 https://github.com/QuantumShieldLabs/qsl-protocol/pull/562
- qsl-protocol implementation/design head SHA: `5cea7185abd7423058a9d5c529f788f2dfe30924`
- qsl-protocol implementation/design merge SHA: `87fe6297e4329b47940bc5de3094a7a18b032ef7`
- qsl-protocol implementation/design mergedAt: `2026-03-25T00:22:01Z`
- exact chosen final-removal result: `R1`
- exact reason final-removal implementation is now justified: the merged `NA-0199`, `NA-0200A`, `NA-0201`, `NA-0201A`, `NA-0202B`, `NA-0203`, and `NA-0203A` evidence now satisfies the explicit `DOC-ATT-004` `W1` proof obligations strongly enough that the next blocker is `W2` implementation work, not another gate-finalization item
- explicit closeout path: `AG1`

### NA-0205 — Legacy In-Message Final Removal Implementation

Status: DONE

Problem:
- `NA-0204` concluded that the accumulated coexistence, constrained-host, reference-deployment, stress/soak, migration-window, and cleanup evidence is strong enough to justify the final W2 implementation lane.
- The next blocker is now the actual qsc implementation of the already-frozen final-removal behavior for legacy in-message send carriage in validated deployments.

Scope:
- `qsl/qsl-client/qsc/**` runtime/tests/docs as needed to implement the final-removal behavior already frozen by `DOC-ATT-004` and `DOC-ATT-005`.
- qsl-protocol governance/evidence updates as needed.
- no qsl-attachments runtime changes
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no silent fallback from attachment to legacy
- no dishonest delivery-state behavior
- no capability-like secrets in canonical URLs
- any remaining receive-side compatibility or operator-visible controls must behave exactly as frozen by current policy
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the final W2 legacy-removal behavior exactly as frozen by `DOC-ATT-004` and `DOC-ATT-005`
2) update qsc operator/help/runbook surfaces to match the final-removal behavior truthfully
3) add deterministic tests proving no-silent-fallback, honest delivery semantics, and policy-faithful final-removal behavior
4) record evidence and queue updates truthfully

Acceptance:
1) the final-removal behavior is implemented exactly as frozen by current policy
2) no silent fallback, no dishonest delivery behavior, and no secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #564 https://github.com/QuantumShieldLabs/qsl-protocol/pull/564
- qsl-protocol implementation merge SHA: `33352450d934`
- qsl-protocol implementation mergedAt: `2026-03-25T01:45:14Z`
- exact W2 behavior summary:
  - validated deployments now default new legacy-sized sends, including exact `4 MiB`, to the attachment path when `QSC_ATTACHMENT_SERVICE` is present and no explicit legacy stage override is set
  - explicit `w0` remains the rollback/coexistence control for new legacy-sized sends, `w1` is retained only as a deprecated compatibility alias to `w2`, and receive compatibility remains intact for both already-supported path families
  - selecting `w2` without validated attachment-service configuration still fails closed with `attachment_service_required`, and attachment-path failures do not silently retry the legacy path
- exact operator-visible control / fallback / rollback summary:
  - operators continue to control rollback/coexistence explicitly with `QSC_LEGACY_IN_MESSAGE_STAGE=w0` or `--legacy-in-message-stage w0`
  - validated deployment configuration remains explicit through `QSC_ATTACHMENT_SERVICE` and `--attachment-service`; qsc help and runbook surfaces now document `w0` / `w2` truthfully
  - no rollback or fallback path is silent: missing validated configuration fails closed, and send failures preserve honest delivery semantics without false `peer_confirmed`
- explicit closeout path: `AH1`

### NA-0205A — Final Removal Validation + Cleanup

Status: DONE

Problem:
- `NA-0205` implements the frozen `W2` final-removal behavior, so the next blocker is validating the merged lane end-to-end and cleaning up any remaining deterministic test, runbook, or evidence assumptions before the project treats the legacy send path as fully retired for validated deployments.

Scope:
- `qsl/qsl-client/qsc/**` tests/docs/evidence as needed to validate merged `W2` behavior and clean up final-removal artifacts
- qsl-protocol governance/evidence as needed
- no qsl-attachments runtime changes
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no silent fallback from attachment to legacy
- honest delivery semantics remain truthful
- no capability-like secrets in canonical URLs
- any remaining receive-side compatibility or operator-visible controls must behave exactly as frozen by current policy
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) run and record post-merge validation for the `W2` final-removal lane
2) close any remaining deterministic test/runbook/evidence cleanup discovered during implementation and CI stabilization
3) prove the merged final-removal lane is clean enough that the next blocker is not another direct `W2` implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms final-removal behavior, no-silent-fallback, and truthful delivery semantics on the required supported lanes
2) final-removal-local deterministic test and operator-runbook cleanup is complete
3) no protocol, relay, or attachment-service semantic change is introduced
4) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #566 https://github.com/QuantumShieldLabs/qsl-protocol/pull/566
- qsl-protocol implementation merge SHA: `1dc4d9b655b4`
- qsl-protocol implementation mergedAt: `2026-03-26T02:11:46Z`
- exact validation/cleanup summary:
  - post-merge validation reran the frozen `W2` lane green across `cargo fmt -p qsc -- --check`, `cargo clippy -p qsc --all-targets -- -D warnings`, `cargo build -p qsc --release --locked`, the required targeted qsc tests, and full `cargo test -p qsc --locked`
  - the only merged cleanup was deterministic and scope-minimal: stale `NA-0203` / `W1` test evidence identifiers were refreshed to `NA-0205A` / `W2`, and explicit regression coverage now proves the deprecated `w1` env/CLI selector resolves to `w2` semantics without changing runtime behavior
  - implementation scope stayed inside `qsl/qsl-client/qsc/tests/**`, so no qsc runtime, qsl-attachments, or qsl-server semantic change occurred
- exact operator-visible W2 behavior summary:
  - validated deployments keep attachment-first default selection for new legacy-sized sends, including exact `4 MiB`, when `QSC_ATTACHMENT_SERVICE` is present and no explicit `w0` rollback/coexistence override is selected
  - explicit `w0` remains the rollback/coexistence control, `w1` remains only a deprecated compatibility alias to `w2`, and missing validated attachment configuration still fails closed with `attachment_service_required`
  - no silent fallback, no false `peer_confirmed`, route-token/header carriage, and receive compatibility remained intact under the refreshed regression set
- explicit closeout path: `AI1`

### NA-0206 — Legacy Receive Compatibility Retirement Decision

Status: DONE

Problem:
- `NA-0205` implemented the final `W2` removal of legacy in-message send carriage for validated deployments, and `NA-0205A` validates that merged lane.
- The next blocker is deciding whether the remaining receive-side legacy compatibility should stay as-is, narrow, or move toward retirement, and under what evidence-backed conditions.

Scope:
- qsl-protocol docs/evidence/governance only as needed for the receive-compatibility retirement decision
- qsl-attachments and qsl-server read-only as needed for evidence
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) decide whether and when remaining legacy receive-side compatibility can be narrowed or retired
2) define the smallest truthful successor lane implied by that decision
3) record the decision and evidence truthfully

Acceptance:
1) the receive-compatibility decision is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation/design PR: #568 https://github.com/QuantumShieldLabs/qsl-protocol/pull/568
- qsl-protocol implementation/design merge SHA: `c0d7c62c3ebf`
- qsl-protocol implementation/design mergedAt: `2026-03-26T02:57:29Z`
- exact chosen receive-compatibility result: `D0`
- exact reason receive-retirement implementation is not justified now:
  - merged `W2` policy and tests still keep `w0` as the explicit rollback/coexistence control for new legacy-sized sends, so legacy receive remains load-bearing on current `main`
  - current policy still defers replacement of the receiver-side contract for already-supported legacy payloads to a later explicit item, so direct retirement would invent semantics rather than implement a frozen boundary
- explicit closeout path: `AJ2`

### NA-0206A — Receive Compatibility Retirement Gate Finalization

Status: DONE

Problem:
- `NA-0206` materially advanced the receive-compatibility retirement decision, but at least one explicit gate still blocks a truthful implementation lane.

Scope:
- qsl-protocol docs/evidence/governance only as needed to finalize the remaining receive-retirement gate(s)
- qsl-attachments and qsl-server read-only as needed for evidence
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) identify and freeze the remaining gate(s) that still block receive-compatibility retirement implementation
2) define the smallest truthful successor lane implied by that result
3) record the decision and evidence truthfully

Acceptance:
1) the remaining receive-retirement gate(s) are explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation/design PR: #570 https://github.com/QuantumShieldLabs/qsl-protocol/pull/570
- qsl-protocol implementation/design merge SHA: `7710951e5e00`
- qsl-protocol implementation/design mergedAt: `2026-03-26T11:49:51Z`
- exact chosen receive-compatibility result: `GR1`
- exact reason receive-retirement implementation is not justified now:
  - merged `DOC-ATT-007` freezes that current `main` still keeps explicit `w0` rollback/coexistence live, so already-supported legacy `file_chunk` / `file_manifest` payloads remain inside the supported receiver contract while `w0` stays available
  - merged evidence still does not freeze a truthful post-`w0` runtime-removal contract, so one smaller final gate must come before receive-retirement implementation
- explicit closeout path: `AK2`

### NA-0206C — Receive Compatibility Retirement Final Gate

Status: DONE

Problem:
- `NA-0206A` materially advanced the receive-side retirement gates, but one final explicit gate still blocks a truthful implementation lane.

Scope:
- qsl-protocol docs/evidence/governance only as needed to finalize the last blocking receive-retirement gate
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) identify and freeze the exact remaining gate blocking receive-side retirement implementation
2) define the smallest truthful successor lane implied by that gate
3) record the decision and evidence truthfully

Acceptance:
1) the final blocking gate is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation/design PR: #573 https://github.com/QuantumShieldLabs/qsl-protocol/pull/573
- qsl-protocol implementation/design merge SHA: `edb3e2ae96aa`
- qsl-protocol implementation/design mergedAt: `2026-03-27T00:33:06Z`
- explicit closeout path: `AL1`
- exact summary of the approved post-`w0` receiver-side policy:
  - while `w0` remains live, current mixed legacy receive compatibility remains exactly as already frozen and implemented
  - once `w0` is no longer live, legacy receive-side compatibility for legacy `file_chunk` / `file_manifest` payloads is retired on validated deployments with no implicit drain window, continued-support posture, fallback, or rollback
  - post-`w0` legacy payload receipt must fail closed explicitly and operator-visibly without reconstructing legacy files, persisting or promoting durable completion state, advancing `peer_confirmed`, or creating dishonest delivery outcomes
  - route-token/header-carriage behavior remains unchanged, qsl-server remains transport-only, and qsl-attachments remains opaque ciphertext-only
- exact reason `NA-0207` is now justified:
  - `DOC-ATT-008` and Decision `D-0322` now freeze the previously missing post-`w0` receiver-side boundary, so the remaining blocker is implementation detail rather than policy selection
  - current `main` keeps live `w0` behavior unchanged until that later implementation lands, preserving truthful rollback/coexistence on validated deployments

### NA-0207 — Legacy Receive Compatibility Retirement Implementation

Status: DONE

Problem:
- `NA-0206C` froze the post-`w0` receiver-side retirement boundary, so the next blocker is now the actual qsc implementation of legacy receive compatibility retirement.

Scope:
- `qsl/qsl-client/qsc/**` runtime/tests/docs as needed to implement the receive-side retirement behavior already frozen by current policy and the `NA-0206` / `NA-0206A` / `NA-0206C` decisions
- qsl-protocol governance/evidence as needed
- no qsl-attachments runtime changes
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the receive-side retirement behavior exactly as frozen by current policy
2) update operator/help/runbook surfaces truthfully
3) add deterministic tests proving no-silent-break, honest delivery semantics, and policy-faithful receive-side retirement behavior
4) record evidence and queue updates truthfully

Acceptance:
1) the receive-side retirement behavior is implemented exactly as frozen by current policy
2) no dishonest delivery behavior or secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #575 https://github.com/QuantumShieldLabs/qsl-protocol/pull/575
- qsl-protocol implementation merge SHA: `75a311b4cc17`
- qsl-protocol implementation mergedAt: `2026-03-27T01:47:57Z`
- explicit closeout path: `AM1`
- exact post-`w0` behavior summary:
  - `qsc receive` now defaults `--legacy-receive-mode` to `coexistence`, preserving current mixed legacy receive compatibility while `w0` remains live
  - validated post-`w0` lanes can switch explicitly to `--legacy-receive-mode retired`
  - in `retired` mode legacy `file_chunk` / `file_manifest` payloads fail closed before legacy reconstruction or completion handling, while attachment-descriptor receive behavior remains unchanged
  - rejected legacy payloads do not reconstruct legacy files, mutate durable receive/timeline state, emit file completion receipts, or advance `peer_confirmed`
- exact operator-visible reject/failure summary:
  - retired-mode legacy receive rejects emit `event=legacy_receive_reject code=legacy_receive_retired_post_w0`
  - retired-mode legacy receive rejects emit `event=file_xfer_reject code=legacy_receive_retired_post_w0`
  - retired-mode legacy receive rejects emit `event=error code=legacy_receive_retired_post_w0`
- exact reason `NA-0207A` is now justified:
  - merged PR #575 implements the frozen post-`w0` receive-retirement behavior in qsc, updates operator/help/runbook surfaces truthfully, and adds deterministic no-mutation / no-false-`peer_confirmed` proof for the retired receive path
  - local validation and required CI are green, so the next blocker is post-merge validation/cleanup of the merged lane rather than another direct implementation gap

### NA-0207A — Receive Retirement Validation + Cleanup

Status: DONE

Problem:
- `NA-0207` implements the already-frozen receive-side retirement behavior, so the next blocker is validating the merged lane end-to-end and cleaning up any remaining deterministic tests, runbooks, or evidence assumptions before the project treats legacy receive compatibility as fully retired for validated deployments.

Scope:
- `qsl/qsl-client/qsc/**` tests/docs/evidence as needed to validate merged receive-retirement behavior and clean up final-removal artifacts
- qsl-protocol governance/evidence as needed
- no qsl-attachments runtime changes
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) run and record post-merge validation for the receive-retirement lane
2) close any remaining deterministic test/runbook/evidence cleanup discovered during implementation and CI stabilization
3) prove the merged receive-retirement lane is clean enough that the next blocker is not another direct implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms receive-retirement behavior, no-silent-break, and truthful delivery semantics on the required supported lanes
2) receive-retirement-local deterministic test and operator-runbook cleanup is complete
3) no protocol, relay, or attachment-service semantic change is introduced
4) queue/evidence updated truthfully

Evidence:
- qsl-protocol implementation PR: #577 https://github.com/QuantumShieldLabs/qsl-protocol/pull/577
- qsl-protocol implementation merge SHA: `872335bfe45c`
- qsl-protocol implementation mergedAt: `2026-03-27T02:56:14Z`
- explicit closeout path: `AN1`
- exact validation/cleanup summary:
  - merged PR #577 removed the remaining receive-retirement-local cleanup gap by aligning the local qsc runbook with the full retired-mode reject marker set and tightening deterministic regressions to prove `event=file_xfer_reject code=legacy_receive_retired_post_w0` alongside the existing reject and error markers
  - required local validation was rerun green on the merged lane: `cargo fmt -p qsc -- --check`, `cargo clippy -p qsc --all-targets -- -D warnings`, `cargo build -p qsc --release --locked`, the directive-required focused qsc tests, and full `cargo test -p qsc --locked`
  - required GitHub checks for PR #577 finished green before merge, including the long-running `ci-4a` and `macos-qsc-qshield-build` lanes
- exact operator-visible receive-retirement behavior summary:
  - `qsc receive` still defaults `--legacy-receive-mode` to `coexistence`, preserving mixed legacy receive compatibility while `w0` remains live
  - validated post-`w0` lanes can switch explicitly to `--legacy-receive-mode retired`
  - in `retired` mode legacy `file_chunk` / `file_manifest` payloads fail closed with `event=legacy_receive_reject code=legacy_receive_retired_post_w0`, `event=file_xfer_reject code=legacy_receive_retired_post_w0`, and `event=error code=legacy_receive_retired_post_w0`
  - rejected legacy payloads do not reconstruct legacy files, mutate durable receive/timeline state, emit file completion receipts, or advance `peer_confirmed`; attachment-descriptor receive, route-token/header carriage, qsl-server transport-only posture, and qsl-attachments opaque ciphertext-only posture remain unchanged

### NA-0208 — Post-W0 Activation / Legacy Mode Retirement Decision

Status: DONE

Problem:
- `NA-0207` implemented receive-side retirement and `NA-0207A` validated the merged lane, but the project still needs an explicit evidence-backed decision for when validated deployments stop operating with live `w0` coexistence and move to the post-`w0` retired posture by default, including what operator-visible controls remain.

Scope:
- qsl-protocol docs/evidence/governance only as needed for the post-`w0` activation / legacy-mode retirement decision
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) decide when and under what conditions validated deployments leave live `w0` coexistence and adopt the post-`w0` retired posture by default
2) define what operator-visible controls remain, narrow, or disappear at that cutover point
3) define the smallest truthful successor lane implied by that decision
4) record the decision and evidence truthfully

Acceptance:
1) the post-`w0` activation / legacy-mode retirement decision is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Evidence:
- implementation PR: #579 https://github.com/QuantumShieldLabs/qsl-protocol/pull/579
- implementation merge SHA: `72c355373f57`
- implementation mergedAt: `2026-03-28T00:45:16Z`
- exact chosen activation result: `PA1` / `AO1` — post-`w0` activation / legacy-mode retirement implementation is now the next truthful lane
- exact reason implementation is now justified: qsl-attachments remains `READY=0` with the deployment/readiness lane complete, qsl-server remains `READY=0` with the canonical header-based compatibility guard in place, `W2` send-path removal is already implemented and validated, and the post-`w0` receive boundary is already frozen plus implemented/validated; the only remaining work is to retire the still-live `w0` / deprecated `w1` / receive `coexistence` operator posture itself
- exact control-surface freeze: current `main` stays unchanged until the activation implementation lands, but that lane must retire live `w0` rollback/coexistence, retire deprecated `w1`, and retire receive `coexistence` as supported validated-deployment controls while keeping fail-closed attachment-service configuration and explicit retired-mode reject surfaces
- explicit closeout path: `AO1`

### NA-0209 — Post-W0 Activation / Legacy Mode Retirement Implementation

Status: DONE

Problem:
- `NA-0208` concluded that validated deployments have enough evidence to stop operating with live `w0` coexistence and adopt the post-`w0` retired posture by default.
- The next blocker is now the actual qsc implementation of that activation/cutover behavior and its operator-visible controls.

Scope:
- `qsl/qsl-client/qsc/**` runtime/tests/docs as needed to implement the post-`w0` activation / legacy-mode retirement behavior already frozen by current policy and the `NA-0208` decision
- qsl-protocol governance/evidence as needed
- no qsl-attachments runtime changes
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the post-`w0` activation / legacy-mode retirement behavior exactly as frozen by current policy and the `NA-0208` decision
2) update operator/help/runbook surfaces truthfully
3) add deterministic tests proving no-silent-break, honest delivery semantics, and policy-faithful activation/cutover behavior
4) record evidence and queue updates truthfully

Acceptance:
1) the post-`w0` activation / legacy-mode retirement behavior is implemented exactly as frozen by current policy
2) no dishonest delivery behavior or secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Evidence:
- implementation PR: #581 https://github.com/QuantumShieldLabs/qsl-protocol/pull/581
- implementation merge SHA: `5df9fef387ee`
- implementation mergedAt: `2026-03-28T01:55:55Z`
- exact activation/cutover behavior summary:
  - validated send lanes now treat `QSC_ATTACHMENT_SERVICE` as the post-`w0` cutover trigger: default `<= 4 MiB` sends use `w2`, and explicit `w0` / deprecated `w1` restoration attempts fail closed with `legacy_in_message_stage_retired_post_w0`
  - validated receive lanes now treat `--attachment-service` as the post-`w0` cutover trigger: receive defaults to retired, explicit `--legacy-receive-mode coexistence` fails closed with `legacy_receive_mode_retired_post_w0`, and residual legacy payloads still reject with `legacy_receive_retired_post_w0` without reconstructing files or advancing `peer_confirmed`
  - route-token/header carriage remains canonical and unchanged, qsl-server remains transport-only, and qsl-attachments remains opaque ciphertext-only
- exact control-surface/removal summary:
  - live `w0` and deprecated `w1` no longer remain usable validated-deployment send controls once the attachment-service trigger is present
  - validated receive coexistence no longer remains a usable control once `--attachment-service` is present
  - there is no rollback path back to live coexistence on the validated post-`w0` lane; the remaining AWS coexistence runbook text is explicitly non-validated compatibility coverage only
- explicit closeout path: `AP1`

### NA-0209A — Post-W0 Activation Validation + Cleanup

Status: DONE

Problem:
- `NA-0209` implements the already-frozen post-`w0` activation / legacy-mode retirement behavior, so the next blocker is validating the merged lane end-to-end and cleaning up any remaining deterministic tests, runbooks, or evidence assumptions before the project treats the post-`w0` retired posture as the normal validated-deployment baseline.

Scope:
- `qsl/qsl-client/qsc/**` tests/docs/evidence as needed to validate the merged post-`w0` activation lane and clean up activation/cutover artifacts
- qsl-protocol governance/evidence as needed
- no qsl-attachments runtime changes
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) run and record post-merge validation for the activation/cutover lane
2) close any remaining deterministic test/runbook/evidence cleanup discovered during implementation and CI stabilization
3) prove the merged activation/cutover lane is clean enough that the next blocker is not another direct implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms activation/cutover behavior, no-silent-break, and truthful delivery semantics on the required supported lanes
2) activation-local deterministic test and operator-runbook cleanup is complete
3) no protocol, relay, or attachment-service semantic change is introduced
4) queue/evidence updated truthfully

Evidence:
- implementation PR: #583 https://github.com/QuantumShieldLabs/qsl-protocol/pull/583
- merge SHA: `75ad9859725a`
- mergedAt: `2026-03-28T02:57:08Z`
- exact validation/cleanup summary:
  - post-merge validation reran the required qsc local matrix green: `cargo fmt -p qsc -- --check`, `cargo clippy -p qsc --all-targets -- -D warnings`, `cargo build -p qsc --release --locked`, the required activation/route-token regression suites, and `cargo test -p qsc --locked`
  - the remaining direct cleanup gap was operator-surface clarity rather than a runtime bug: qsc help text, the local validated-deployment runbook, and deterministic regressions now state and prove that `file send --attachment-service` is diagnostic-only while `QSC_ATTACHMENT_SERVICE` remains the validated send-side activation trigger
  - no protocol, relay, attachment-service, route-token/header, or honest-delivery semantics changed during validation/cleanup
- exact operator-visible post-`w0` behavior summary:
  - validated send lanes remain keyed to `QSC_ATTACHMENT_SERVICE`: legacy-sized sends default to `w2`, and explicit `w0` / `w1` restoration attempts fail closed with `legacy_in_message_stage_retired_post_w0`
  - validated receive lanes remain keyed to `--attachment-service`: legacy receive defaults to `retired`, explicit `coexistence` fails closed with `legacy_receive_mode_retired_post_w0`, and residual legacy payloads still reject with `legacy_receive_retired_post_w0` without false `peer_confirmed`
  - route-token/header carriage remains canonical and unchanged, qsl-server remains transport-only, and qsl-attachments remains opaque ciphertext-only
- exact chosen outcome: `AQ1` — no additional transport-transition blocker remains after activation validation/cleanup; the next truthful blocker is metadata leakage/logging contract review

### NA-0211 — Metadata Leakage Surface Review + Logging Contract

Status: DONE

Problem:
- The transport-lifecycle transition is complete enough that the next load-bearing blocker is no longer legacy/attachment coexistence, but the project’s metadata leakage, log-hygiene, and secret-handling surface, especially around descriptors, journals, support artifacts, and operator-visible tooling.

Scope:
- qsl-protocol docs/evidence/governance and runtime review as needed for metadata/logging contract definition
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work in this item

Must protect:
- no silent break of current validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) inventory unavoidable versus avoidable metadata leakage surfaces across message plane, attachment plane, logs, docs, and evidence artifacts
2) define the logging/secret-hygiene contract the project must enforce
3) identify the smallest truthful follow-on implementation/enforcement lanes implied by that contract
4) record the decision and evidence truthfully

Acceptance:
1) metadata/log-hygiene posture is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Evidence:
- implementation PR: #585 https://github.com/QuantumShieldLabs/qsl-protocol/pull/585
- merge SHA: `b3ca301cdff0`
- mergedAt: `2026-03-28T21:39:38Z`
- exact chosen metadata/logging result: `MLR0` — direct metadata / secret-hygiene enforcement is the next blocker
- exact reason the chosen next blocker is truthful:
  - `DOC-G5-004`, `DECISIONS.md`, and `TRACEABILITY.md` now freeze the current attachment-plane-era metadata inventory, distinguish unavoidable versus avoidable leakage, and state the non-negotiable logging / secret-hygiene rules without inventing new protocol, relay, or attachment-service semantics
  - the remaining direct gap is enforcement of that frozen contract across passive logs, operator-visible markers, docs/runbooks, and evidence artifacts; the review found no smaller missing review artifact and no prerequisite qsl-attachments authn/authz / policy-subject contract blocker that must be resolved first
- explicit closeout path: `AR1`

### NA-0211A — Metadata / Secret-Hygiene Enforcement

Status: DONE

Problem:
- `NA-0211` completed the evidence-backed metadata leakage surface review and froze the logging / secret-hygiene contract.
- The next blocker is now enforcing that contract in runtime behavior, tests, logs, docs, and evidence generation surfaces.

Scope:
- qsl-protocol, qsl-attachments, and qsl-server runtime/tests/docs as needed to enforce the frozen metadata / secret-hygiene contract
- governance/evidence updates as needed
- no website/.github work unless later separately authorized

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the agreed metadata / secret-hygiene contract on the relevant surfaces
2) add deterministic proof for secret-safe logging, markers, evidence artifacts, and operator-visible outputs
3) identify any remaining metadata leakage that is unavoidable versus avoidable after enforcement
4) update queue/evidence truthfully

Acceptance:
1) the agreed metadata / secret-hygiene contract is enforced truthfully
2) no dishonest delivery behavior or secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AS1`
- qsl-protocol implementation PR: #587 https://github.com/QuantumShieldLabs/qsl-protocol/pull/587
- qsl-protocol merge SHA: `3b70ab832919`
- qsl-protocol mergedAt: `2026-03-28T22:47:54Z`
- qsl-attachments implementation PR: #16 https://github.com/QuantumShieldLabs/qsl-attachments/pull/16
- qsl-attachments merge SHA: `5e5849055118`
- qsl-attachments mergedAt: `2026-03-28T22:27:19Z`
- qsl-attachments repo-local closeout PR: #17 https://github.com/QuantumShieldLabs/qsl-attachments/pull/17
- exact enforcement summary by repo:
  - `qsl-protocol/qsc`: deterministic attachment-streaming coverage now proves operator-visible send/receive/reject output does not leak full `attachment_id`, attachment-service base URLs, `fetch_capability`, `locator_ref`, or malformed `enc_ctx_b64u` values.
  - `qsl-attachments`: passive audit-log events now emit only short deterministic `session_handle`, `locator_handle`, and `attachment_handle` values, and the contract test proves audit snapshots exclude raw stable identifiers, capabilities, and plaintext markers.
  - `qsl-server`: current repo state already complied with the frozen contract on the in-scope operator surfaces, so no repo-local mutation was required.
- exact reason the chosen next blocker is truthful:
  - direct metadata / secret-hygiene enforcement is complete enough across the currently load-bearing qsc, qsl-attachments, and qsl-server surfaces to remove passive identifier leakage and stale unsafe assumptions without changing protocol, relay, or attachment-service semantics
  - the next unresolved blocker is the missing qsl-attachments authn/authz / policy-subject contract, which determines what operator-visible identities, quotas, policy subjects, and metadata exposures are legitimate beyond the now-enforced secret-hygiene baseline

### NA-0212 — qsl-attachments Authn/Authz / Policy Subject Contract

Status: DONE

Problem:
- `NA-0211A` completed or materially exhausted the direct metadata / secret-hygiene enforcement lane, and the next load-bearing blocker is now the missing authn/authz / policy-subject contract for qsl-attachments, which determines what identities, quotas, policy subjects, and metadata exposures are legitimate.

Scope:
- qsl-attachments docs/evidence/governance and qsl-protocol linkage as needed to define the authn/authz / policy-subject contract
- qsl-server read-only as needed for proof
- no runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) define the qsl-attachments authn/authz / policy-subject contract
2) define what operator-visible identities, quotas, and policy subjects exist and what metadata they legitimately expose
3) identify the smallest truthful implementation lane implied by that contract
4) record the decision and evidence truthfully

Acceptance:
1) the authn/authz / policy-subject contract is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AT1`
- qsl-protocol implementation/design PR: #589 https://github.com/QuantumShieldLabs/qsl-protocol/pull/589
- qsl-protocol merge SHA: `cabaaeedcbfc`
- qsl-protocol mergedAt: `2026-03-28T23:37:34Z`
- qsl-attachments promotion PR: #18 https://github.com/QuantumShieldLabs/qsl-attachments/pull/18
- qsl-attachments promotion merge SHA: `737b081133a9`
- qsl-attachments promotion mergedAt: `2026-03-28T23:26:29Z`
- qsl-attachments implementation/design PR: #19 https://github.com/QuantumShieldLabs/qsl-attachments/pull/19
- qsl-attachments implementation/design merge SHA: `bdcb68803e24`
- qsl-attachments implementation/design mergedAt: `2026-03-28T23:32:51Z`
- qsl-attachments closeout PR: #20 https://github.com/QuantumShieldLabs/qsl-attachments/pull/20
- qsl-attachments closeout merge SHA: `bdd606c201e7`
- qsl-attachments closeout mergedAt: `2026-03-28T23:39:59Z`
- exact chosen auth/policy result: `ASC0`
- exact frozen contract summary:
  - the sole current qsl-attachments service policy subject is the operator-scoped deployment
  - `resume_token` and `fetch_capability` remain per-resource authorizers rather than service-account identities
  - deployment-global quota and abuse ceilings remain owned by the operator-scoped deployment subject
  - legitimate operator-visible metadata remains limited to non-secret refs, short deterministic handles, state, and reason codes
- exact reason the chosen next blocker is truthful:
  - current runtime and evidence already distinguish the real policy subject, capability authorizers, and quota ownership without semantic invention
  - qsl-attachments `main` now truthfully shows `NA-0008` as the sole READY repo-local successor, so the remaining blocker is explicit implementation of the frozen contract rather than more finalization or continued-support governance
- explicit closeout path statement: `AT1`

### NA-0212A — qsl-attachments Authn/Authz / Policy Subject Implementation

Status: DONE

Problem:
- `NA-0212` froze the qsl-attachments authn/authz / policy-subject contract clearly enough that the next blocker is now implementing it.

Scope:
- `qsl-attachments/**` runtime/tests/docs as needed to implement the frozen authn/authz / policy-subject contract
- qsl-protocol linkage/evidence as needed
- no qsl-server changes
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the frozen authn/authz / policy-subject contract in qsl-attachments
2) update operator/help/runbook surfaces truthfully
3) add deterministic tests proving policy-subject handling, quota ownership, and secret-hygiene behavior
4) record evidence and queue updates truthfully

Acceptance:
1) the authn/authz / policy-subject contract is implemented exactly as frozen
2) no dishonest delivery behavior or secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AU1`
- qsl-attachments implementation PR: #21 https://github.com/QuantumShieldLabs/qsl-attachments/pull/21
- qsl-attachments implementation merge SHA: `d93eb729642a`
- qsl-attachments implementation mergedAt: `2026-03-29T00:32:55Z`
- qsl-attachments closeout PR: #22 https://github.com/QuantumShieldLabs/qsl-attachments/pull/22
- qsl-attachments closeout merge SHA: `cecf5d1025ce`
- qsl-attachments closeout mergedAt: `2026-03-29T00:35:10Z`
- qsl-protocol implementation/linkage PR: #591 https://github.com/QuantumShieldLabs/qsl-protocol/pull/591
- qsl-protocol merge SHA: `6f1419aba595`
- qsl-protocol mergedAt: `2026-03-29T00:40:04Z`
- exact implementation summary:
  - qsl-attachments now makes the operator-scoped deployment subject explicit on runtime/operator surfaces through `Config::operator_policy_surface()` / `AppState::operator_policy_surface()` and startup markers, without adding a new service auth layer
  - deterministic tests now prove deployment-global quota ownership, many-transfer behavior under one deployment policy/quota, and one-capability-per-resource semantics for both `resume_token` and `fetch_capability`
  - qsl-attachments repo-local governance now truthfully records `NA-0008` as DONE and returns the sibling repo to `READY=0`
- exact reason the chosen next blocker is truthful:
  - no direct authn/authz / policy-subject implementation gap remains in the current qsl-attachments runtime/tests/docs/operator surfaces
  - the next unresolved load-bearing issue is the durability / recovery contract across crash consistency, backup/restore, and restart behavior rather than more identity/policy implementation work
- explicit closeout path statement: `AU1`

### NA-0213 — qsl-attachments Durability / Recovery Contract

Status: DONE

Problem:
- `NA-0212A` implemented the frozen operator-scoped authn/authz / policy-subject contract, so the next load-bearing blocker is no longer identity/policy semantics but the durability / recovery guarantees of qsl-attachments across crash consistency, backup/restore, and restart behavior.

Scope:
- qsl-attachments docs/evidence/governance and qsl-protocol linkage as needed to define the durability / recovery contract
- qsl-server read-only as needed for proof
- no runtime changes yet
- no website/.github work

Must protect:
- no plaintext attachment handling on service surfaces
- no capability-like secrets in canonical URLs
- qsl-attachments remains opaque ciphertext-only
- qsl-server remains transport-only

Deliverables:
1) define the durability / recovery contract for qsl-attachments
2) define crash consistency, backup/restore, and restart expectations truthfully
3) identify the smallest truthful implementation lane implied by that contract
4) update queue/evidence truthfully

Acceptance:
1) the durability / recovery contract is explicit and evidence-backed
2) no attachment-service semantic redesign occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AV1`
- qsl-protocol implementation/design PR: #593 https://github.com/QuantumShieldLabs/qsl-protocol/pull/593
- qsl-protocol merge SHA: `14cfe34ff45e`
- qsl-protocol mergedAt: `2026-03-29T01:30:00Z`
- qsl-attachments promotion PR: #23 https://github.com/QuantumShieldLabs/qsl-attachments/pull/23
- qsl-attachments promotion merge SHA: `8a53b5a549db`
- qsl-attachments promotion mergedAt: `2026-03-29T01:17:19Z`
- qsl-attachments implementation/design PR: #24 https://github.com/QuantumShieldLabs/qsl-attachments/pull/24
- qsl-attachments implementation/design merge SHA: `6fedcdd1ea27`
- qsl-attachments implementation/design mergedAt: `2026-03-29T01:21:53Z`
- qsl-attachments closeout PR: #25 https://github.com/QuantumShieldLabs/qsl-attachments/pull/25
- qsl-attachments closeout merge SHA: `0d994f90bda3`
- qsl-attachments closeout mergedAt: `2026-03-29T01:32:21Z`
- exact chosen durability/recovery result: `DRC0`
- exact frozen contract summary:
  - one operator-managed local storage root on one node is the entire durability boundary
  - graceful same-root restart is in scope, and committed-object recovery requires both `object.json` and `ciphertext.bin`
  - cold whole-root backup/restore plus matching service configuration is the only supported backup shape
  - abrupt-crash/open-session survival and cross-file transactional durability are not promised; bounded operator cleanup remains explicit instead
- exact reason the chosen next blocker is truthful:
  - the merged code/evidence already freeze the real local-disk durability boundary, restart scope, and unsupported hot-backup / abrupt-crash claims without another design pass
  - the remaining blocker is deterministic implementation and test coverage for crash windows, startup recovery handling, and cold whole-root backup/restore under that frozen contract
- explicit closeout path statement: `AV1`

### NA-0213A — qsl-attachments Durability / Recovery Implementation

Status: DONE

Problem:
- `NA-0213` froze the qsl-attachments durability / recovery contract clearly enough that the next blocker is now implementing it.

Scope:
- `qsl-attachments/**` runtime/tests/docs as needed to implement the frozen durability / recovery contract
- qsl-protocol linkage/evidence as needed
- no qsl-server changes
- no website/.github work

Must protect:
- no plaintext attachment handling on service surfaces
- no capability-like secrets in canonical URLs
- qsl-attachments remains opaque ciphertext-only
- qsl-server remains transport-only

Deliverables:
1) implement the frozen durability / recovery contract in qsl-attachments
2) update operator/help/runbook surfaces truthfully
3) add deterministic tests proving crash consistency, recovery, and backup/restore expectations that are in scope
4) record evidence and queue updates truthfully

Acceptance:
1) the durability / recovery contract is implemented exactly as frozen
2) no attachment-service semantic redesign or secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AW1`
- qsl-attachments implementation PR: #26 https://github.com/QuantumShieldLabs/qsl-attachments/pull/26
- qsl-attachments implementation merge SHA: `4a6b2c4eee7a40b64b050859a760e2e72b858911`
- qsl-attachments implementation mergedAt: `2026-03-29T02:19:04Z`
- qsl-attachments closeout PR: #27 https://github.com/QuantumShieldLabs/qsl-attachments/pull/27
- qsl-attachments closeout merge SHA: `d0fbfa7c1edd92fdef14be1e1f3e6afd4f2990ae`
- qsl-attachments closeout mergedAt: `2026-03-29T02:22:21Z`
- qsl-protocol linkage PR: #595 https://github.com/QuantumShieldLabs/qsl-protocol/pull/595
- qsl-protocol linkage merge SHA: `8f29ec32a5bbd1a39dc18c949c1fb845016e297e`
- qsl-protocol linkage mergedAt: `2026-03-29T02:28:10Z`
- exact implementation summary:
  - qsl-attachments now performs bounded startup reconciliation on the same storage root so only coherent open sessions and committed objects are re-exposed after graceful same-root restart
  - committed-object recovery is now enforced as a paired-file rule (`object.json` + `ciphertext.bin`), incoherent recovery artifacts are discarded fail-closed, and operator-visible startup markers/docs now state the cold full-root backup/restore boundary truthfully
  - deterministic repo-local tests now prove graceful same-root restart, paired-file committed-object recovery, fail-closed discard of incoherent/orphan recovery artifacts, and docs truthfulness without any qsl-protocol runtime/client change
- exact reason the chosen next blocker is truthful:
  - the frozen durability / recovery contract is now implemented explicitly enough that the next honest blocker is validating the merged lane and cleaning up any remaining deterministic tests/runbooks/evidence assumptions
  - qsl-attachments `main` now truthfully shows `NA-0010A` as the sole READY repo-local successor, so there is no remaining direct durability / recovery implementation gap to finalize first
- explicit closeout path statement: `AW1`

### NA-0213B — qsl-attachments Durability / Recovery Validation + Cleanup

Status: DONE

Problem:
- `NA-0213A` implemented the frozen durability / recovery contract, so the next blocker is validating the merged lane end-to-end and cleaning up any remaining deterministic tests, runbooks, or evidence assumptions.

Scope:
- `qsl-attachments/**` tests/docs/evidence as needed to validate the merged durability / recovery lane
- qsl-protocol governance/evidence as needed
- no qsl-server changes
- no website/.github work

Must protect:
- no plaintext attachment handling on service surfaces
- no capability-like secrets in canonical URLs
- qsl-attachments remains opaque ciphertext-only
- qsl-server remains transport-only

Deliverables:
1) run and record post-merge validation for the durability / recovery lane
2) close any remaining deterministic test/runbook/evidence cleanup discovered during implementation and CI stabilization
3) prove the merged durability / recovery lane is clean enough that the next blocker is not another direct implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms durability / recovery behavior on the required supported lanes
2) durability-local deterministic test and operator-runbook cleanup is complete
3) no attachment-service semantic redesign is introduced
4) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AX1`
- qsl-attachments implementation PR: #28 https://github.com/QuantumShieldLabs/qsl-attachments/pull/28
- qsl-attachments implementation merge SHA: `f486747c11e7efe5a1b0d35df9b95eabbd28cd70`
- qsl-attachments implementation mergedAt: `2026-03-29T03:08:13Z`
- qsl-attachments closeout PR: #29 https://github.com/QuantumShieldLabs/qsl-attachments/pull/29
- qsl-attachments closeout merge SHA: `e94107ac094d2be938098b43017b2019f2e894a3`
- qsl-attachments closeout mergedAt: `2026-03-29T03:10:41Z`
- qsl-protocol linkage PR: #597 https://github.com/QuantumShieldLabs/qsl-protocol/pull/597
- qsl-protocol linkage merge SHA: `e3b5540711348c5e6035b949efced2caed0422ea`
- qsl-protocol linkage mergedAt: `2026-03-29T03:17:46Z`
- exact validation/cleanup summary:
  - qsl-attachments top-level operator surfaces now state explicitly that hot/live backup and partial restore remain unsupported while preserving the frozen same-root graceful restart and cold full-root backup/restore boundary
  - qsl-attachments `tests/NA-0010A_durability_recovery_validation_evidence.md` now records the green local validation bundle, and deterministic tests still prove same-root restart, paired-file committed-object recovery, fail-closed discard of incoherent/orphaned recovery artifacts, docs/evidence truthfulness, and secret-safe audit-handle posture
  - qsl-attachments now truthfully returns to `READY=0`, so the merged durability / recovery lane no longer has a direct repo-local validation/finalization blocker
- exact reason the chosen next blocker is truthful:
  - no direct durability validation/finalization gap remains in qsl-attachments or qsl-protocol after the merged cleanup
  - the next remaining load-bearing blocker is the operator/user-facing quality, predictability, and packaging/error-state surface of qsc rather than another attachment durability lane
- explicit statement whether closeout path was AX1 or AX2: `AX1`

### NA-0214 — qsc TUI UX / Error-State / Packaging Audit

Status: DONE

Problem:
- The transport transition and the first post-transition hardening wave are now settled enough that the next load-bearing blocker is no longer attachment/relay lifecycle semantics but the quality, predictability, and operator/user-facing behavior of the TUI client and its packaging/error-state surface.

Scope:
- `qsl/qsl-client/qsc/**` runtime/tests/docs/packaging surfaces as needed for a TUI UX / error-state / packaging audit
- qsl-protocol governance/evidence as needed
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) audit the TUI’s UX, focus/error-state behavior, operator-visible messaging, and packaging surface
2) identify the smallest truthful implementation lane implied by that audit
3) record the decision and evidence truthfully

Acceptance:
1) the TUI audit result is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AY1`
- qsl-protocol implementation/design PR: #599 https://github.com/QuantumShieldLabs/qsl-protocol/pull/599
- qsl-protocol implementation/design merge SHA: `2b884c4343c9`
- qsl-protocol implementation/design mergedAt: `2026-03-29T04:26:43Z`
- audit artifact: `docs/design/DOC-QSC-007_qsc_TUI_UX_Error_State_and_Packaging_Audit_v0.1.0_DRAFT.md`
- exact audit summary:
  - current qsc evidence already proves the focus model, locked-shell behavior, runbook/help parity, and secret-hygiene posture strongly enough to preserve them without another review-only lane
  - the remaining qsc blocker is product polish: operator-visible error/remediation wording, qbuild-first packaging/release front-door clarity, and migration/retired-mode UX polish
  - option `TU0` was chosen and `TU1` rejected because no smaller audit/finalization gap remains after the current audit artifact and test-backed review
- tests/evidence summary:
  - focused qsc audit bundle remained green: `cargo test -p qsc --locked --test cli --test diagnostics --test route_header_migration_docs_na0195a --test tui_nav_selection --test tui_focus_modes --test tui_locked_first --test tui_lock_unlock_polish --test tui_command_output_routing --test tui_startup_hardening_na0177 --test session_state_at_rest --test identity_secret_at_rest --test two_client_local_runbook_na0182 --test vault_attempt_limit`
- exact reason the chosen next blocker is truthful:
  - qsl-attachments now truthfully returns to `READY=0` and qsl-server remains `READY=0`, so no sibling durability or relay prerequisite blocks qsc product work first
  - the current qsc docs/tests/runbooks are already decision-grade enough that the next blocker is direct product polish and implementation rather than another audit-only or finalization-only lane
- explicit statement whether closeout path was AY1 or AY2: `AY1`

### NA-0214A — qsc TUI Product Polish Implementation

Status: DONE

Problem:
- `NA-0214` completed the evidence-backed TUI UX / error-state / packaging audit clearly enough that the next blocker is now direct qsc TUI polish and implementation work.

Scope:
- `qsl/qsl-client/qsc/**` runtime/tests/docs/packaging surfaces as needed to implement the TUI product-polish changes implied by the audit
- qsl-protocol governance/evidence as needed
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the TUI polish changes justified by the audit
2) improve operator-visible messaging, focus/navigation, and packaging/release surfaces truthfully
3) add deterministic proof for the corrected TUI behavior
4) record evidence and queue updates truthfully

Acceptance:
1) the audited TUI/product issues are implemented truthfully
2) no protocol, relay, or attachment-service semantic change is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `AZ1`
- qsl-protocol implementation PR: #601 https://github.com/QuantumShieldLabs/qsl-protocol/pull/601
- qsl-protocol implementation merge SHA: `46c0cb286c41`
- qsl-protocol implementation mergedAt: `2026-03-29T15:02:59Z`
- exact implementation summary:
  - qsc relay/status/lock/help surfaces now replace placeholder copy with truthful operator wording, actionable vault/session/migration guidance, and deterministic product-polish markers
  - `qsl/qsl-client/qsc/README.md` plus the local/remote qsc runbooks now make the qbuild/local lane the operator front door and classify remote/AWS artifacts as compatibility-only evidence
  - deterministic coverage now includes `qsl/qsl-client/qsc/tests/tui_product_polish_na0214a.rs`
- tests/evidence summary:
  - local validation bundle green: `cargo fmt -p qsc -- --check`; `cargo clippy -p qsc --all-targets -- -D warnings`; `cargo build -p qsc --release --locked`; `cargo test -p qsc --locked --test cli -- --nocapture`; `cargo test -p qsc --locked --test route_header_migration_docs_na0195a -- --nocapture`; `cargo test -p qsc --locked --test tui_charter -- --nocapture`; `cargo test -p qsc --locked --test tui_main_scroll_focus -- --nocapture`; `cargo test -p qsc --locked --test identity_secret_at_rest -- --nocapture`; `cargo test -p qsc --locked --test tui_product_polish_na0214a -- --nocapture`; `cargo test -p qsc --locked`
  - implementation PR #601 merged after bounded check polling reached `TOTAL=30 SUCCESS=30 INPROG=0 FAILS=0`
- exact reason the chosen next blocker is truthful:
  - the direct qsc TUI product-polish implementation gap is closed: local validation passed, required GitHub checks passed, and the merged lane preserves transport/auth/attachment semantics
  - the next truthful blocker is validating the merged lane end-to-end and cleaning up any residual deterministic test/runbook/packaging assumptions rather than finishing another direct implementation gap
- explicit statement whether closeout path was AZ1 or AZ2: `AZ1`

### NA-0214C — qsc TUI Product Polish Validation + Cleanup

Status: DONE

Problem:
- `NA-0214A` implemented the TUI product-polish changes justified by the audit, so the next blocker is validating the merged lane end-to-end and cleaning up any remaining deterministic tests, runbooks, or packaging/evidence assumptions.

Scope:
- `qsl/qsl-client/qsc/**` tests/docs/evidence/packaging surfaces as needed to validate the merged TUI product-polish lane
- qsl-protocol governance/evidence as needed
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) run and record post-merge validation for the TUI product-polish lane
2) close any remaining deterministic test/runbook/packaging cleanup discovered during implementation and CI stabilization
3) prove the merged lane is clean enough that the next blocker is not another direct implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms TUI/product-polish behavior truthfully
2) TUI-local deterministic test and operator-runbook cleanup is complete
3) no protocol, relay, or attachment-service semantic change is introduced
4) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BA1`
- qsl-protocol implementation PR: #603 https://github.com/QuantumShieldLabs/qsl-protocol/pull/603
- qsl-protocol implementation merge SHA: `aafc9c948bdb`
- qsl-protocol implementation mergedAt: `2026-03-29T16:12:48Z`
- exact validation/cleanup summary:
  - `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md` now classifies remote relay testing as compatibility-only evidence behind the qbuild/local front door and removes stale placeholder/workflow-era wording
  - deterministic coverage now includes `qsl/qsl-client/qsc/tests/tui_product_polish_validation_na0214c.rs`
  - the merged qsc TUI product-polish lane revalidated cleanly across the required local bundle, targeted state/vault proof, and full `cargo test -p qsc --locked`
- tests/evidence summary:
  - local validation bundle green: `cargo fmt -p qsc -- --check`; `cargo clippy -p qsc --all-targets -- -D warnings`; `cargo build -p qsc --release --locked`; `cargo test -p qsc --locked --test cli -- --nocapture`; `cargo test -p qsc --locked --test route_header_migration_docs_na0195a -- --nocapture`; `cargo test -p qsc --locked --test tui_charter -- --nocapture`; `cargo test -p qsc --locked --test tui_main_scroll_focus -- --nocapture`; `cargo test -p qsc --locked --test identity_secret_at_rest -- --nocapture`; `cargo test -p qsc --locked --test tui_product_polish_na0214a -- --nocapture`; `cargo test -p qsc --locked --test tui_product_polish_validation_na0214c -- --nocapture`; `cargo test -p qsc --locked --test session_state_at_rest -- --nocapture`; `cargo test -p qsc --locked --test vault_attempt_limit -- --nocapture`; `cargo test -p qsc --locked`
  - implementation PR #603 merged after bounded check polling reached `TOTAL=30 SUCCESS=30 INPROG=0 FAILS=0`
- exact reason the chosen next blocker is truthful:
  - no additional direct qsc TUI validation/finalization gap remains: packaging/help/operator-front-door wording is aligned, deterministic doc/test cleanup is merged, required checks are green, and the state/vault plus secret-hygiene proofs remain green
  - the remaining blocker is deciding whether and how to add a desktop GUI layer for Linux and macOS without changing the current qsc / qsl-attachments / qsl-server boundary
- explicit statement whether closeout path was BA1 or BA2: `BA1`

### NA-0215 — Desktop GUI Architecture Decision (Linux/macOS)

Status: DONE

Problem:
- The TUI product-quality pass is complete enough that the next blocker is no longer direct TUI polish, but deciding whether and how to add a desktop GUI layer for Linux and macOS without compromising the current qsc / qsl-attachments / qsl-server boundaries.

Scope:
- qsl-protocol docs/evidence/governance only as needed for the desktop GUI architecture decision
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) decide whether a desktop GUI is warranted now and what architectural boundary it must respect
2) evaluate the likely implementation shape (for example Tauri-first vs alternatives) without implementing it
3) define the smallest truthful successor lane implied by that decision
4) record the decision and evidence truthfully

Acceptance:
1) the GUI architecture decision is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BB1`
- qsl-protocol implementation/design PR: #605 https://github.com/QuantumShieldLabs/qsl-protocol/pull/605
- qsl-protocol implementation/design merge SHA: `7e9108d122c7`
- qsl-protocol implementation/design mergedAt: `2026-03-29T16:59:10Z`
- exact decision summary:
  - `docs/design/DOC-QSC-008_Desktop_GUI_Architecture_Decision_v0.1.0_DRAFT.md` freezes the desktop GUI as a Linux/macOS-only, Tauri-first, qsc-centered shell boundary and rejects both continued TUI-only deferral and an extra architecture-finalization lane as the truthful next posture.
  - qsc remains the owner of vault/session/relay/attachment truth; a future GUI may wrap qsc-owned behavior but must not scrape `qsc tui`, move client-core logic into frontend JavaScript, or introduce a second persistent secret store.
  - current evidence therefore makes the next blocker the bounded prototype definition itself, so `NA-0215A — Desktop GUI Prototype Boundary` becomes the sole READY item.

### NA-0215A — Desktop GUI Prototype Boundary

Status: DONE

Problem:
- `NA-0215` concluded that a desktop GUI is warranted now and froze the architectural boundary clearly enough that the next blocker is defining and proving a minimal prototype boundary rather than re-litigating whether a GUI should exist.

Scope:
- qsl-protocol docs/evidence/governance only as needed to define the GUI prototype boundary
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) define the smallest truthful desktop GUI prototype boundary
2) define what qsc functionality, packaging, and platform support are in and out of scope
3) define the security boundary between GUI shell and existing qsc/qsl-attachments/qsl-server components
4) record the decision and evidence truthfully

Acceptance:
1) the GUI prototype boundary is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BB1`
- qsl-protocol implementation/design PR: #607 https://github.com/QuantumShieldLabs/qsl-protocol/pull/607
- qsl-protocol implementation/design merge SHA: `2f5bdc978631`
- qsl-protocol implementation/design mergedAt: `2026-03-29T18:45:04Z`
- exact prototype-boundary summary:
  - `docs/design/DOC-QSC-009_Desktop_GUI_Prototype_Boundary_v0.1.0_DRAFT.md` freezes the first desktop GUI prototype as a Linux/macOS-only, message-first, Tauri-sidecar shell over one bundled `qsc` binary rather than a broader attachments/full-transcript desktop client.
  - the exact sidecar contract is now explicit: allowlisted `qsc` commands and marker/text parses only, no `qsc tui` scraping, no arbitrary shell execution, no GUI parsing of qsc private store files, and no second persistent secret store.
  - current evidence therefore makes direct prototype implementation the next blocker, so `NA-0215B — Desktop GUI Prototype Implementation` becomes the sole READY item.

### NA-0215B — Desktop GUI Prototype Implementation

Status: DONE

Problem:
- `NA-0215A` froze the Linux/macOS desktop GUI prototype boundary clearly enough that the next blocker is now implementing the minimal prototype rather than re-litigating the boundary.

Scope:
- `qsl-protocol/**` runtime/tests/docs/packaging surfaces as needed to implement the frozen GUI prototype boundary
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only
- no second persistent secret store
- no movement of client-core logic into frontend JavaScript

Deliverables:
1) implement the minimal frozen GUI prototype boundary
2) preserve the qsc-centered sidecar-shell model exactly as frozen
3) add deterministic proof for the prototype behavior
4) record evidence and queue updates truthfully

Acceptance:
1) the GUI prototype is implemented exactly as frozen
2) no protocol, relay, attachment-service, or secret-storage semantic change is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BD2`
- qsl-protocol implementation PR: #609 https://github.com/QuantumShieldLabs/qsl-protocol/pull/609
- qsl-protocol implementation merge SHA: `23310c680a9a`
- qsl-protocol implementation mergedAt: `2026-03-29T20:57:30Z`
- exact implementation outcome:
  - `qsl/qsl-client/qsc-desktop/` now provides the bounded Linux/macOS Tauri shell frozen by `NA-0215A`: one bundled `qsc` sidecar, single-profile message-first UI, Rust-only sidecar bridge, explicit out-of-scope messaging for attachments/history/multiprofile, and fail-closed operator errors for locked or unsupported states.
  - deterministic proof landed in `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`, and backend parser/unit coverage landed in `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`; local validation also produced a truthful Linux AppImage artifact without widening into qsl-attachments, qsl-server, website, or workflow changes.
  - merged evidence still leaves at least one direct implementation blocker before a truthful validation/cleanup lane: keychain-backed active operations remain deferred under the current `qsc` shell contract, and GUI send/receive still requires pre-existing protocol state so the shell must surface `protocol_inactive` rather than invent handshake/session-establish behavior.
  - current evidence therefore makes implementation finalization, not merged-lane validation/cleanup, the next blocker, so `NA-0215BB — Desktop GUI Prototype Implementation Finalization` becomes the sole READY item.

### NA-0215BB — Desktop GUI Prototype Implementation Finalization

Status: DONE

Problem:
- `NA-0215B` materially advanced the bounded GUI prototype, but at least one direct implementation gap still blocks a truthful validation/cleanup lane.

Scope:
- `qsl-protocol/**` runtime/tests/docs/packaging surfaces as needed to finish the already-frozen GUI prototype implementation
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only
- no second persistent secret store
- no movement of client-core logic into frontend JavaScript

Deliverables:
1) close the remaining direct implementation gap(s) without semantic redesign
2) add or refresh deterministic proof for the corrected behavior
3) keep operator-visible GUI prototype behavior truthful
4) update queue/evidence truthfully

Acceptance:
1) the remaining direct implementation blocker is removed truthfully
2) no dishonest delivery behavior or secret-bearing URL regression is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BE2`
- qsl-protocol implementation PR: #611 https://github.com/QuantumShieldLabs/qsl-protocol/pull/611
- qsl-protocol implementation merge SHA: `cf599f8fa0db`
- qsl-protocol implementation mergedAt: `2026-03-29T22:17:08Z`
- exact implementation outcome:
  - the merged desktop GUI now surfaces peer-specific `qsc handshake status` truth in the Rust bridge/UI, blocks send/receive fail-closed until protocol state is actually ready, and removes the prior seeded-fallback shortcut from desktop contract coverage.
  - the merged packaging surface is narrowed to the frozen truthful lane: Linux AppImage on Linux hosts and macOS `.app` on macOS hosts.
  - current evidence removed the direct protocol-activation-readiness blocker inside the frozen prototype boundary.
  - the remaining blocker is no longer direct implementation inside this lane: keychain-backed active operations still sit outside the current passphrase-centered sidecar contract, so the next truthful item is explicit active-ops boundary finalization rather than validation/cleanup.
  - current evidence therefore makes `NA-0215BC — Desktop GUI Prototype Active-Ops Boundary Finalization` the sole READY item.

### NA-0215BC — Desktop GUI Prototype Active-Ops Boundary Finalization

Status: DONE

Problem:
- `NA-0215BB` materially advanced the bounded desktop GUI prototype, but the remaining blocker is no longer direct implementation. It is the explicit finalization of the active-ops boundary, especially keychain-backed active operations and protocol-activation readiness, before the prototype can move into validation/cleanup truthfully.

Scope:
- qsl-protocol docs/evidence/governance only as needed to finalize the remaining active-ops boundary
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only
- no second persistent secret store
- no movement of client-core logic into frontend JavaScript

Deliverables:
1) identify and freeze the exact remaining active-ops boundary gap blocking validation/cleanup
2) define the smallest truthful successor lane implied by that gap
3) record the decision and evidence truthfully

Acceptance:
1) the remaining active-ops boundary gap is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BF1`
- qsl-protocol implementation/design PR: #613 https://github.com/QuantumShieldLabs/qsl-protocol/pull/613
- qsl-protocol implementation/design merge SHA: `02ebc1d56be8`
- qsl-protocol implementation/design mergedAt: `2026-03-29T23:17:27Z`
- exact finalization outcome:
  - `NA-0215BC` froze active ops as passphrase-backed sidecar actions only: passphrase init/unlock, relay/contact/device mutations, and send/receive only when peer-specific handshake status proves readiness.
  - keychain-backed active operations remain deferred, handshake/session-establish UI remains out of scope, and operator-visible `keychain_deferred` / `protocol_inactive` messaging stays explicit and fail-closed.
  - current merged desktop prototype already fits that frozen boundary, so no smaller direct implementation follow-on remains before truthful merged-lane validation/cleanup.
  - current evidence therefore makes `NA-0215BA — Desktop GUI Prototype Validation + Cleanup` the sole READY item.

### NA-0215BA — Desktop GUI Prototype Validation + Cleanup

Status: DONE

Problem:
- `NA-0215BC` froze the remaining active-ops/keychain boundary clearly enough that the current bounded desktop GUI prototype can now move into merged-lane validation and cleanup.

Scope:
- `qsl-protocol/**` tests/docs/evidence/packaging surfaces as needed to validate the merged desktop GUI prototype lane
- qsl-attachments and qsl-server read-only as needed for proof
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only
- no second persistent secret store
- no movement of client-core logic into frontend JavaScript

Deliverables:
1) run and record post-merge validation for the GUI prototype lane
2) close any remaining deterministic test/runbook/packaging cleanup discovered during implementation and CI stabilization
3) prove the merged lane is clean enough that the next blocker is not another direct implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms GUI prototype behavior truthfully
2) GUI-local deterministic test and operator-runbook cleanup is complete
3) no protocol, relay, or attachment-service semantic change is introduced
4) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `BG1`
- qsl-protocol implementation PR: #615 https://github.com/QuantumShieldLabs/qsl-protocol/pull/615
- qsl-protocol implementation merge SHA: `65884b3ed81c`
- qsl-protocol implementation mergedAt: `2026-03-30T00:29:30Z`
- exact validation/cleanup outcome:
  - current qbuild/local proof now covers `cargo fmt`, `cargo clippy`, `cargo build`, targeted `qsc` / `qsc-desktop` tests, and an isolated-target `npm run tauri:build -- --debug` AppImage build for the bounded desktop prototype.
  - remaining packaging/help drift was cleaned up by making sidecar prep honor `CARGO_TARGET_DIR`, marking the desktop README qbuild/local-first and AWS-free, and clarifying bundle-output paths when isolated targets or debug profiles are used.
  - no additional direct GUI validation/finalization blocker remains: keychain-backed active operations stay explicitly deferred by design, `protocol_inactive` messaging remains truthful, and the merged prototype validated cleanly without semantic redesign.
  - current evidence therefore makes `NA-0216 — Adversarial Validation / Fuzz / Chaos Program Definition` the sole READY item.

### NA-0216 — Adversarial Validation / Fuzz / Chaos Program Definition

Status: DONE

Problem:
- The bounded desktop GUI prototype is implemented and validated cleanly enough that the next load-bearing blocker is no longer direct GUI polish, but defining the adversarial validation / fuzz / chaos program that should harden the full system under malformed inputs, restart/disruption, and high-variance traffic.

Scope:
- qsl-protocol docs/evidence/governance only as needed to define the adversarial validation / fuzz / chaos program
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) define the adversarial validation / fuzz / chaos program scope and goals
2) explain why it truthfully outranks more GUI work now
3) define the smallest truthful implementation lane implied by that program
4) record the decision and evidence truthfully

Acceptance:
1) the adversarial validation / fuzz / chaos program definition is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `CA2`
- qsl-protocol implementation/design PR: #617 https://github.com/QuantumShieldLabs/qsl-protocol/pull/617
- qsl-protocol implementation/design merge SHA: `480407b209a9`
- qsl-protocol implementation/design mergedAt: `2026-03-30T01:42:11Z`
- exact post-audit decision outcome:
  - `DOC-G4-001` and Decision `D-0337` now freeze the adversarial validation / fuzz / chaos program strongly enough that no smaller finalization gap remains.
  - The audit set and current repo proof still show higher-severity live secret-ingress debt: qsc keeps passphrase argv/env/direct-ingress and `QSC_PASSPHRASE` reuse paths, qsl-server keeps compatibility-only URI-carried route-token paths, and qsl-protocol public/demo guidance still normalizes weaker env-secret handling in places.
  - Current evidence therefore makes `NA-0216B — Client / Relay Secret-Ingress Remediation` the sole READY item, not direct adversarial-program implementation.

### NA-0216B — Client / Relay Secret-Ingress Remediation

Status: DONE

Problem:
- `NA-0216` showed that the most urgent next blocker is not adversarial-program implementation but remediation of the highest-severity client/relay secret-ingress risks already identified by the audit set.

Scope:
- qsl-protocol/qsc runtime/tests/docs as needed to remove passphrase-via-argv/env/echo flows and normalize safer ingress
- qsl-server runtime/tests/docs as needed to retire legacy URI-carried route-token compatibility paths
- qsl-protocol public/demo docs as needed to stop teaching weak secret-handling patterns
- qsl-attachments read-only unless a directly related surface is proven load-bearing
- no website/.github work in this item

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) remove or retire the highest-severity secret-ingress patterns identified by the audits
2) add deterministic proof that the corrected behavior is truthful and fail-closed
3) keep public/demo operator guidance aligned with the safer ingress posture where in scope
4) update queue/evidence truthfully

Acceptance:
1) the identified high-severity secret-ingress problems are remediated truthfully
2) no protocol, relay, or attachment-service semantic redesign is introduced
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `CB1`
- qsl-protocol implementation PR: #619 https://github.com/QuantumShieldLabs/qsl-protocol/pull/619
- qsl-protocol implementation merge SHA: `b0d1d8be767f`
- qsl-protocol implementation mergedAt: `2026-03-30T03:49:06Z`
- exact remediation outcome:
  - qsc now retires general passphrase-via-argv/env ingress and ambient `QSC_PASSPHRASE` reuse; explicit passphrase-file/stdin ingress plus process-local reuse remain, and the hidden reserved desktop-sidecar env handoff stays narrowly scoped for compatibility only
  - qsl-server PR #45 retires URI-carried route-token compatibility outright, qsl-server PR #46 returns the sibling repo to `READY=0`, and canonical header-carried routing remains authoritative
  - in-scope public/demo guidance now uses explicit 0600 passphrase-file handling, and deterministic tests prove that retired argv/env/URI-ingress patterns no longer remain on the remediated paths
  - no direct client / relay secret-ingress blocker remains before the already-frozen adversarial validation / fuzz / chaos implementation lane, so the next truthful blocker is `NA-0216A`

### NA-0216A — Adversarial Validation / Fuzz / Chaos Implementation

Status: DONE

Problem:
- `NA-0216B` remediated the highest-severity client / relay secret-ingress issues, so the next load-bearing blocker is now implementing the already-frozen adversarial validation / fuzz / chaos program.

Scope:
- qsl-protocol runtime/tests/docs and qsl-attachments / qsl-server targeted test surfaces as needed to implement the frozen adversarial validation / fuzz / chaos program
- governance/evidence updates as needed
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) implement the agreed adversarial validation / fuzz / chaos program
2) add or wire the selected fuzz/property/sanitizer/program surfaces
3) define corpora/targets/invariants and CI placement truthfully
4) update queue/evidence truthfully

Acceptance:
1) the agreed program is implemented truthfully
2) no protocol, relay, or attachment-service semantic change is introduced beyond the agreed test/instrumentation surfaces
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `CC1`
- qsl-protocol implementation PR: #621 https://github.com/QuantumShieldLabs/qsl-protocol/pull/621
- qsl-protocol implementation merge SHA: `ba314936c379`
- qsl-protocol implementation mergedAt: `2026-03-30T23:56:27Z`
- exact implementation outcome:
  - qsc now exposes bounded adversarial helper modules for route parsing, payload parsing, and vault-envelope parsing, with coverage-guided fuzz targets, curated seed corpora, property-based invariants, and a nightly Miri lane wired through the qbuild-first workflow/script.
  - local and CI proof remain within the frozen semantics: canonical header-carried route-token handling stays authoritative, reject paths remain fail-closed and parse-only, and no protocol / relay / attachment-service redesign was introduced.
  - qsl-server and qsl-attachments did not require repo-local promotion in this lane because existing restore / stress / chaos evidence already satisfied the frozen sibling-repo assumptions and both repos remain at `READY=0`.
  - current evidence removes the direct implementation blocker for the frozen adversarial program, so the next truthful blocker is merged-lane validation/cleanup rather than another implementation-finalization lane.
  - current evidence therefore makes `NA-0216AA — Adversarial Validation / Fuzz / Chaos Validation + Cleanup` the sole READY item.

### NA-0216AA — Adversarial Validation / Fuzz / Chaos Validation + Cleanup

Status: DONE

Problem:
- `NA-0216A` implemented the frozen adversarial validation / fuzz / chaos program, so the next blocker is validating the merged lane end-to-end and cleaning up any remaining deterministic tests, corpora, sanitizer placement, or evidence assumptions.

Scope:
- qsl-protocol runtime/tests/docs and any already-touched sibling-repo test surfaces as needed to validate the merged adversarial program
- governance/evidence updates as needed
- no website/.github work beyond what is already merged into this program lane

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) run and record post-merge validation for the adversarial program
2) close any remaining deterministic test/corpus/sanitizer cleanup discovered during implementation and CI stabilization
3) prove the merged lane is clean enough that the next blocker is not another direct implementation gap
4) update queue/evidence truthfully

Acceptance:
1) post-merge evidence confirms the adversarial program truthfully
2) deterministic test/corpus/sanitizer cleanup is complete enough for the lane
3) no protocol, relay, or attachment-service semantic redesign is introduced
4) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `CD1`
- qsl-protocol implementation PR: #623 https://github.com/QuantumShieldLabs/qsl-protocol/pull/623
- qsl-protocol implementation merge SHA: `136c236c3a3d`
- qsl-protocol implementation mergedAt: `2026-03-31T01:20:02Z`
- exact validation/cleanup outcome:
  - qsl-protocol now carries repo-local post-merge evidence in `docs/archive/testplans/NA-0216AA_adversarial_validation_cleanup_evidence.md`, so the merged adversarial lane no longer relies only on ephemeral command output for its validation truth.
  - local qbuild proof stayed green across qsc fmt/build/clippy, the bounded adversarial smoke script, the nightly Miri lane, and the nearby route-header, attachment-streaming, and vault regression sets.
  - bounded fuzzing ran from temporary copies of the checked-in corpora, the checked-in corpora stayed unchanged, and repo-local artifact paths remained ignored/clean, so no corpus-handling blocker remains.
  - qsl-server remained truthfully at `READY=0` with `NA-0012` still `DONE`, and qsl-attachments remained truthfully at `READY=0`, so no sibling-repo validation gap forced scope widening.
  - no direct adversarial validation / fuzz / chaos finalization blocker remains; the next truthful blocker is qsc modularization / file-size reduction planning because `qsl/qsl-client/qsc/src/main.rs` still holds `21,627` of `24,790` qsc source lines (`87.24%`).

### NA-0217 — qsc Modularization / File-Size Reduction Plan

Status: DONE

Problem:
- The adversarial validation / fuzz / chaos lane is settled enough that the next load-bearing blocker is no longer missing test seriousness, but the concentrated maintainability and auditability risk in `qsc`, especially the extreme size and responsibility concentration in `qsl/qsl-client/qsc/src/main.rs`.

Scope:
- qsl-protocol docs/evidence/governance only as needed to define the modularization / file-size reduction plan
- qsl-attachments and qsl-server read-only as needed for proof
- no qsl-protocol runtime changes yet
- no website/.github work

Must protect:
- no silent break of validated deployment flows
- no dishonest delivery semantics
- no capability-like secrets in canonical URLs
- no regression to route-token/header-carriage behavior
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) define the highest-value `qsc` modularization seams and file-size reduction order
2) define the smallest truthful implementation lane implied by that plan
3) record the decision and evidence truthfully

Acceptance:
1) the modularization / file-size reduction plan is explicit and evidence-backed
2) no protocol, relay, or attachment-service semantic change occurs in this item
3) queue/evidence updated truthfully

Closeout evidence:
- closeout path: `CE1`
- qsl-protocol implementation PR: #625 https://github.com/QuantumShieldLabs/qsl-protocol/pull/625
- qsl-protocol implementation merge SHA: `5e16b60929ed`
- qsl-protocol implementation mergedAt: `2026-03-31T03:49:24Z`
- exact planning outcome:
  - qsl-protocol now carries `docs/design/DOC-QSC-011_qsc_Modularization_and_File_Size_Reduction_Plan_v0.1.0_DRAFT.md`, `docs/archive/testplans/NA-0217_qsc_modularization_file_size_reduction_plan_evidence.md`, `DECISIONS.md` D-0343, and matching `TRACEABILITY.md` links, so the planning lane no longer relies on ephemeral shell output for its concentration proof, seam order, or successor definition.
  - the planning artifact freezes the live `qsc` concentration map (`21,627` of `24,790` qsc source lines in `qsl/qsl-client/qsc/src/main.rs`), the fragile-zone regression inventory, and the no-drift modularization invariants strongly enough that the next blocker is direct implementation rather than more planning or audit finalization.
  - qsl-server remained truthfully at `READY=0`, qsl-attachments remained truthfully at `READY=0`, and no runtime, workflow, protocol, relay, or attachment-service semantic change was introduced by this lane.
  - the smallest truthful direct successor is `NA-0217A — qsc Marker / Output Foundation Extraction`, because it removes one coherent high-fan-out foundation seam from `main.rs` without widening into TUI, handshake, transport, or attachment redesign.

### NA-0217A — qsc Marker / Output Foundation Extraction

Status: DONE

Problem:
- `NA-0217` froze the modularization plan strongly enough that the next truthful blocker is no longer choosing seams, but extracting the smallest coherent foundation seam from `qsl/qsl-client/qsc/src/main.rs` without behavioral drift. The marker / output cluster is the first move because it is high-fan-out, contract-heavy, and lower-semantic-risk than TUI, transport, handshake, or attachment work.

Scope:
- `qsl/qsl-client/qsc/src/main.rs`
- new `qsl/qsl-client/qsc/src/output/**`
- `qsl/qsl-client/qsc/src/lib.rs` only if a narrow re-export or test-visibility bridge is mechanically required
- `qsl/qsl-client/qsc/tests/**` and qsl-protocol docs/evidence/governance only as needed for no-drift proof
- no qsc-desktop, qsl-server, or qsl-attachments runtime changes
- no `.github`, website, `Cargo.toml`, or `Cargo.lock` changes

Must protect:
- one `qsc` binary and the current CLI contract
- current marker names, marker shapes, marker routing, and redaction behavior
- current qsc-desktop sidecar contract, including child-scoped passphrase handling and peer-specific readiness parsing
- current route-token/header discipline and secret-free canonical URLs
- current honest-delivery semantics, especially `accepted_by_relay` versus `peer_confirmed`
- current `protocol_inactive` / `ACTIVE` / `INACTIVE` fail-closed posture
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) extract marker formatting, routing, redaction, log-writing, and terminal-sanitizer helpers into a dedicated output foundation module
2) keep existing call sites behavior-identical while reducing `main.rs` responsibility concentration
3) prove no drift across the representative marker-sensitive regression suites
4) update queue/evidence truthfully

Acceptance:
1) `QSC_MARK/1` plain-marker and JSONL output remain unchanged for the same inputs
2) the representative marker-sensitive suites remain green: `tui_charter`, `tui_product_polish_na0214a`, `desktop_gui_contract_na0215b`, `route_header_migration_docs_na0195a`, and `remote_soak_diag_mapping_na0168`
3) `qsl/qsl-client/qsc/src/main.rs` loses one coherent marker/output foundation cluster without widening into subsystem redesign
4) no protocol, relay, attachment, vault, identity, or qsc-desktop semantic drift is introduced

Closeout evidence:
- closeout path: `CF1`
- qsl-protocol implementation PR: #627 https://github.com/QuantumShieldLabs/qsl-protocol/pull/627
- qsl-protocol implementation merge SHA: `3ecf3a4c44c8`
- qsl-protocol implementation mergedAt: `2026-03-31T05:41:18Z`
- archive evidence: `docs/archive/testplans/NA-0217A_marker_output_foundation_extraction_evidence.md`
- exact implementation/evidence outcome:
  - merged main now carries `qsl/qsl-client/qsc/src/output/mod.rs`, `qsl/qsl-client/qsc/tests/output_marker_contract_na0217a.rs`, `DECISIONS.md` D-0345, and matching `TRACEABILITY.md` implementation/evidence links, so the marker/output extraction no longer relies only on ephemeral shell output for its no-drift truth.
  - the merged seam shrinks `qsl/qsl-client/qsc/src/main.rs` from `21,627` to `21,338` LOC while moving `318` LOC of marker/output foundation code into `qsl/qsl-client/qsc/src/output/mod.rs`.
  - representative no-drift proof stayed explicit across the plain-marker contract, JSONL marker contract, secret-like redaction behavior, stdout-versus-in-app routing behavior, and the desktop sidecar-sensitive contract suite.
  - this closeout PR is governance-only and introduces no runtime-path changes.
  - no direct marker/output blocker remains; the next truthful blocker is `NA-0217B` because `DOC-QSC-011` freezes filesystem/config/locking as the next foundation seam and that fail-closed storage-safety logic still lives in `main.rs`.

### NA-0217B — qsc Filesystem / Config / Locking Foundation Extraction

Status: READY

Problem:
- `NA-0217A` removed the marker/output foundation, but `qsl/qsl-client/qsc/src/main.rs` still mixes generic filesystem/config/locking/path-safety rules with higher-level feature logic. The next truthful blocker is extracting that fail-closed storage-safety foundation before touching session, identity, transport, attachment, or TUI subsystem seams.

Scope:
- `qsl/qsl-client/qsc/src/main.rs`
- new `qsl/qsl-client/qsc/src/fs_store/**`
- `qsl/qsl-client/qsc/tests/**` and qsl-protocol docs/evidence/governance only as needed for no-drift proof
- no qsc-desktop, qsl-server, or qsl-attachments runtime changes
- no `.github`, website, `Cargo.toml`, or `Cargo.lock` changes

Must protect:
- one `qsc` binary and the current CLI contract
- symlink-safe paths
- `0700` / `0600` enforcement
- atomic writes
- lock behavior
- current qbuild/local-first operator posture
- current qsc-desktop sidecar contract
- current route-token/header discipline and secret-free canonical URLs
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only

Deliverables:
1) extract filesystem/config/locking/path-safety helpers into a dedicated fs-store foundation module
2) keep existing call sites behavior-identical while reducing `main.rs` responsibility concentration
3) prove no drift across the representative at-rest/store-safety regression suites
4) update queue/evidence truthfully

Acceptance:
1) symlink-safe paths, `0700` / `0600` enforcement, atomic writes, and lock behavior remain unchanged for the same inputs
2) the representative suites remain green: `desktop_gui_contract_na0215b`, `identity_secret_at_rest`, `session_state_at_rest`, and `timeline_store`
3) `qsl/qsl-client/qsc/src/main.rs` loses one coherent filesystem/config/locking foundation cluster without widening into session, transport, attachment, or TUI redesign
4) no protocol, relay, attachment, vault, identity, or qsc-desktop semantic drift is introduced
