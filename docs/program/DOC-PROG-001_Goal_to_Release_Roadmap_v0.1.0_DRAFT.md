Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-03

# DOC-PROG-001 — Goal-to-Release Roadmap v0.1.0 DRAFT

## 1. Mission target

QuantumShield reaches a truthful release point only when the merged program simultaneously satisfies `GOALS.md`: always-hybrid per-message protection (`G1`), explicit fail-closed PQ state-machine behavior (`G2`), transcript-bound downgrade resistance (`G3`), CI-backed verification and formal evidence (`G4`), and metadata-minimizing secret-safe operator surfaces (`G5`). This roadmap is strategic only. `NEXT_ACTIONS.md` remains the execution source of truth for ordering, promotion, and closeout.

## 2. Current merged workstreams

The merged program already has these major workstreams in place:

- canonical Suite-2 and SCKA specs plus reference implementation and vector coverage;
- qsc client hardening, desktop boundary decisions, and the completed `NA-0217*` modularization wave;
- qsl-server governance that keeps the relay transport-only;
- qsl-attachments governance that keeps the service opaque-ciphertext-only; and
- adversarial validation, fuzz, chaos, and formal-verification lanes that now exist as checked-in program work rather than informal intent.

## 3. Release-readiness gates by goal

| Goal | Release gate | Merged work now | Still required before any release claim |
| --- | --- | --- | --- |
| `G1` | Per-message hybrid derivation stays normative, implemented, and regression-tested across supported surfaces | `DOC-CAN-003`, Suite-2 refimpl, `CAT-S2-KDF-001`, `CAT-S2-MK-001`, and related CI lanes are merged | Continue carrying the merged hybrid contract through future queue work and integrated release evidence; no release claim is justified by isolated vector success alone |
| `G2` | Explicit SCKA state-machine, persistence, rollback detection, and deterministic reject behavior remain specified and tested | `DOC-CAN-004`, durability vectors, bounded model checks, and qsc fail-closed state foundations are merged | Preserve those guarantees across future promoted work and prove the full release surface still respects them under current main truth |
| `G3` | Downgrade resistance and transcript binding remain fail-closed and operator-visible where required | downgrade/transcript vectors, establish coverage, and qsc handshake-related hardening are merged | Maintain the downgrade contract through future queue work and integrated product evidence; no silent fallback or queue shortcut is acceptable |
| `G4` | Verification remains a release gate: vectors, formal model checks, adversarial lanes, and truthful continuity evidence stay current | `DOC-TST-005`, `formal/README.md`, `DOC-G4-001`, qsc-adversarial lanes, and the `NA-0218` continuity canon are merged | Continue executing live queue items with green evidence and maintain off-host continuity discipline; docs alone do not satisfy the full release gate |
| `G5` | Operator surfaces, continuity artifacts, relay posture, and attachment posture minimize metadata and secret leakage | qsc product-surface audits, desktop boundary docs, qsl-server route-token/header governance, qsl-attachments canonical docs, and `NA-0218` secret-safe continuity rules are merged | Keep runtime/output/ops artifacts aligned with the merged posture; do not make production-readiness claims beyond current evidence |

## 4. What the completed `NA-0217*` wave accomplished

The completed `NA-0217A` through `NA-0217J` wave finished the qsc modularization plan frozen in `DOC-QSC-011` without reopening semantics. Across that wave:

- marker/output behavior moved into `output`;
- filesystem/config/locking behavior moved into `fs_store`;
- protocol-state and session-at-rest logic moved into `protocol_state`;
- identity helpers moved into `identity`;
- contacts/trust/routing moved into `contacts`;
- timeline/delivery logic moved into `timeline`;
- relay transport moved into `transport`;
- attachment/file-transfer logic moved into `attachments`;
- handshake execution moved into `handshake`; and
- the final TUI controller/headless/render helpers moved into `tui/**`.

The wave reduced audit radius and review concentration while preserving the existing qsc-desktop sidecar contract, route-token/header discipline, honest-delivery semantics, and fail-closed state handling already merged on `main`.

## 5. Remaining blockers after modularization

The modularization wave removed a maintainability blocker, but it did not by itself make the program release-ready. The remaining strategic blockers are:

- every live and future promoted queue item still has to land in `NEXT_ACTIONS.md` order with green evidence;
- continuity and off-host snapshot practice must remain current after governance and queue changes, not just be documented once;
- cross-repo release evidence for `qsl-protocol`, `qsl-server`, and `qsl-attachments` is still narrower than a truthful public production-readiness claim; and
- historical status summaries remain subordinate to live repo truth, so release decisions must keep using refreshed refs plus the governance spine.

## 6. Strategic sequencing rule

Use this roadmap to judge whether merged work is moving the program toward the release gates above. Do not use it to skip, reorder, or implicitly promote queue items. If the roadmap and `NEXT_ACTIONS.md` ever appear to disagree, `NEXT_ACTIONS.md` wins and the roadmap must be updated later.
