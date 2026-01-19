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
- Test plan entry: `tests/NA-0018_prekey_lifecycle_testplan.md` with pass/fail criteria.

Acceptance criteria:
- A consumed prekey/bundle cannot be reused; reuse attempts reject deterministically without state mutation.
- Conformance checks fail-closed if reuse is accepted or consumption is not recorded.
- No Suite-2 protocol-core or qsp/* changes.

Evidence:
- Demo relay /consume endpoint with at-most-once bundle consumption.
- CI gate: scripts/ci/metadata_conformance_smoke.sh asserts consume + reuse rejection.
- Test plan: tests/NA-0018_prekey_lifecycle_testplan.md.

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
- Test plan: tests/NA-0019_identity_binding_testplan.md.

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
- Test plan: tests/NA-0020_establish_replay_cache_testplan.md.

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
- Test plan: tests/NA-0021_rate_limit_testplan.md.

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
- Test plan: tests/NA-0022_identifier_collision_testplan.md.

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
- Test plan: tests/NA-0024_pqxdh_scka_epoch_mapping_testplan.md.

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
- Test plan: tests/NA-0025_pq_binding_testplan.md.

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
- Test plan: tests/NA-0026_store_lifecycle_testplan.md.

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
- Test plan: tests/NA-0027_identity_warning_testplan.md.

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

Status: READY
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
