Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-04

# DOC-AUD-001 — qsc Director-Ready Crypto and Code Audit Program v0.1.0 DRAFT

## 1. Purpose and authority posture

This document provides director-ready, supporting strategic guidance for future qsc audit and remediation work.

Its job is to turn high-risk surfaces into bounded, evidence-first directives rather than generic review theater.

Authority boundary:

- the governance spine remains authoritative;
- `NEXT_ACTIONS.md` remains the execution source of truth;
- this audit program does not promote, reorder, or close queue items; and
- future directives may draw from this document only after refreshed-main proof and only when the live queue makes that truthful.

## 2. Program standard

An audit lane derived from this program should:

- reduce the highest real security or correctness risk first;
- stay seam-focused rather than broad and low-signal;
- ground every material claim in code, tests, specs, or the explicit absence of proof;
- convert every serious finding into a bounded remediation directive with required regression evidence; and
- avoid disconnected issue-counting, whole-repo review theater, or speculative queue skipping.

## 3. Current merged context

The completed `NA-0217*` modularization wave reduced qsc audit radius without reopening runtime semantics.

Current merged seams that make sharper audits possible now include:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/fs_store/mod.rs`
- `qsl/qsl-client/qsc/src/output/mod.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`

This audit program focuses qsc code and crypto review sequencing. It does not reopen qsl-server or qsl-attachments scope, and it does not override their transport-only or opaque-ciphertext-only posture.

## 4. Candidate audit and remediation areas

### 4.1 Handshake execution and direct protocol dependencies

This is the highest-yield future qsc audit target because it sits on the narrowest path to catastrophic security failure.

Primary surfaces:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/fs_store/mod.rs`
- `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
- `qsl/qsl-client/qsc/tests/handshake_security_closure.rs`
- `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`
- `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`
- `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md`
- `TRACEABILITY.md`

Questions to answer:

- is every accept path bound to the correct transcript material?
- do identity mismatches reject fail-closed without partial accept?
- can malformed, replayed, or downgraded handshake input mutate session or protocol state?
- can invalid handshake input leave disk state partially updated?
- do operator-visible markers or desktop-facing status surfaces overstate handshake success?

Preferred lane shape:

- first, a read-only seam-focused audit;
- then, one bounded remediation lane per concrete root cause when findings justify it.

### 4.2 Secret material and operator-visible surfaces

This audit focuses on how secure internals can be defeated by unsafe ingress or output surfaces.

Primary surfaces:

- `qsl/qsl-client/qsc/src/output/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs`
- `qsl/qsl-client/qsc-desktop/src-tauri/src/main.rs`
- `qsl/qsl-client/qsc/tests/vault.rs`
- `qsl/qsl-client/qsc/tests/unlock_gate.rs`
- `qsl/qsl-client/qsc/tests/lifecycle.rs`
- `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs`

Questions to answer:

- can secrets appear in argv, env, logs, markers, URLs, headers, or temporary paths?
- can qsc-desktop over-trust or overstate child-process status?
- are redaction rules and secret-safe operator surfaces actually enforced by tests?

### 4.3 Persistence, rollback, and no-mutation-on-reject

This audit verifies that bad input, partial failures, and stale-state conditions remain fail-closed at rest.

Primary surfaces:

- `qsl/qsl-client/qsc/src/fs_store/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/fs_store_contract_na0217b.rs`
- `qsl/qsl-client/qsc/tests/protocol_state_contract_na0217c.rs`
- `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`

Questions to answer:

- can invalid or partial operations mutate durable state?
- do rollback and stale-state cases reject safely?
- do locking and atomic-write assumptions match the code paths that depend on them?

### 4.4 Targeted adversarial validation expansion

This should follow the manual audits above, not replace them.

High-value follow-on work:

- parser-boundary fuzzing
- state-transition property tests
- targeted Miri coverage
- deterministic replay, tamper, and no-mutation negative tests

Relevant current suites:

- `qsl/qsl-client/qsc/tests/adversarial_properties.rs`
- `qsl/qsl-client/qsc/tests/adversarial_miri.rs`
- handshake and protocol-gate negative suites named above

### 4.5 Lower-value first moves to avoid

Avoid starting with:

- a generic whole-repo "crypto audit" detached from application invariants;
- queue-skipping audit work that ignores the live READY item;
- issue-count scanning without directive-grade remediation mapping; or
- broad UI, transport, or attachment review before the highest-risk qsc seam is justified by live queue order.

## 5. Suggested future directive sequence

When the live queue eventually promotes audit work in this area, the highest-signal sequence is:

1. qsc handshake execution security audit (read-only, evidence-first)
2. handshake findings remediation batch 1 (implementation-only, one concrete root cause at a time when feasible)
3. handshake adversarial validation expansion
4. secret material and operator surface audit
5. persistence / rollback / no-mutation audit
6. exit review / claim closure

This sequence is strategic only. `NEXT_ACTIONS.md` still decides whether any of these lanes are active, deferred, or superseded.

## 6. Mandatory finding schema

Every serious audit finding should be reported in this shape:

- Severity: `P0`, `P1`, `P2`, or `P3`
- Title: concise failure mode
- Exact surfaces:
  - file(s)
  - function(s)
  - test(s)
  - spec section(s)
- Claim violated:
  - which invariant or guarantee failed
- Why it matters:
  - exploit path or correctness-failure path
- Minimal fix direction:
  - what should change without designing the whole solution in the audit lane
- Proof gap:
  - which regression, vector, property, or fuzz check is missing
- Recommended directive shape:
  - implementation-only
  - docs/evidence-only
  - audit follow-on

Reject anti-patterns such as:

- "needs refactor"
- "crypto should be reviewed"
- "tests are insufficient" without naming the exact missing proof
- broad severity claims with no exploit or failure path

## 7. Directive guardrails derived from this program

Every future directive derived from this program should enforce:

- one goal only;
- one bounded subsystem only;
- explicit allowed write scope;
- explicit forbidden scope;
- refreshed-main proof before mutation;
- read-only audit lanes separated from fix lanes;
- exact required tests and evidence; and
- stop if a proposed fix would widen into protocol, wire, or crypto semantics outside the declared lane.

## 8. What "best possible code at this point" means

At the current stage of the repo, "best possible code" does not mean "no imperfections anywhere."

It means:

- the highest-risk live subsystem is isolated enough to audit sharply;
- no unresolved P0 or P1 findings remain in the promoted audit surface once that lane completes;
- material security claims are either established by code and tests or explicitly marked as not yet established; and
- residual risk is honest, ranked, and converted into bounded future work instead of hidden in a monolith or an audit report with no follow-through.
