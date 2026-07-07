# QuantumShield Project Charter (Canonical)

## Purpose
QuantumShield aims to deliver a communication protocol that achieves:
- **Strong post-quantum and classical confidentiality for message contents**, including post-compromise recovery.
- **Operationally robust security**, including downgrade/rollback resistance and crash-safe state management.
- **Meaningful metadata minimization**, acknowledging that cryptography alone does not eliminate network metadata.

## Non-negotiable program goals
This project is governed by GOALS.md (G1–G5). All work must advance at least one goal without regressing the others.

## Design philosophy
- **Hybrid-by-default:** combine classical and PQ assumptions rather than choosing one.
- **Fail-closed semantics:** ambiguous inputs and unexpected states must result in safe abort, not permissive behavior.
- **Specification primacy:** normative requirements live in the canonical specs; implementation guidance is subordinate.
- **Traceability:** every security-critical behavior must map to a spec section and to tests/vectors.

### Design tenets (authoritative; bind Director and executor)

These tenets govern how every lane builds. They add discipline; they never relax the
fail-closed rules above.

- **Right the first time, with plumbing for the future.** The first release must combine
  excellent architecture with **deliberate, versioned extension plumbing** that future
  builds can use or adjust. No expedient patching just to make something work now.
- **Extensibility is versioned and explicit, never permissive.** Keep strict validation
  (e.g. `deny_unknown_fields`, exact-length checks, canonical-order enforcement). Add
  forward compatibility through a documented version field and upgrade path plus
  **policy-agnostic consumers** — not by silently ignoring unknown inputs. A parser that
  quietly accepts what it does not understand is a downgrade smell.
- **Eliminate attack surfaces by construction, not by defense.** Prefer a design that
  removes a surface over one that guards it. Pre-release (no installed base) is the
  cheapest moment to bake the strongest baseline: making a protection **mandatory in the
  one-and-only format** leaves no weaker mode for an attacker to force. (Precedent:
  NA-0614 attachment padding, DOC-G5-006/007.)
- **Pre-release leverage; do not import mature-protocol assumptions prematurely.** Before
  invoking installed-base / mixed-version / inter-mode-downgrade machinery, reconfirm
  release status — that apparatus over-engineers a pre-release change.
- **Ground feasibility on the verify path.** A feasibility claim that touches the
  receive/verify path must be established by **reading that path**, not inferred from the
  send path. (Precedent: the DOC-G5-006 M1 correction in NA-0614.)

## Threat model (high-level)
- Active network attacker (replay, reorder, drop, inject, tamper).
- Endpoint compromise at arbitrary times; adversary may obtain device state.
- Long-term quantum-capable attacker (store-now-decrypt-later, plus PQ cryptanalysis).
- Operational failures (crashes, partial writes, restored backups / rollbacks).

## What “better than SPQR” means here
We will compete on explicit properties:
- **Per-message hybridization** without per-message PQ bandwidth overhead (G1).
- **Deterministic SCKA state machine** with stronger operational invariants (G2).
- **Hard downgrade resistance** with transcript-bound capability commitment (G3).
- **Verification and conformance gates** that prevent drift (G4).
- **Metadata minimization profiles** integrated with envelope semantics (G5).

## Success criteria
A change is “successful” only if:
- it is linked to Goal IDs (G1–G5),
- it is reflected in traceability,
- and it is backed by tests/vectors (and model updates where relevant).

## Operating rules
- Any change that affects protocol state machines or key schedules requires a DECISIONS.md entry.
- No feature ships without negative tests for downgrade and rollback.
- Documentation updates must precede or accompany code changes; never “later.”
