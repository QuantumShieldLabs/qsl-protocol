# DEPRECATED — STATUS.md

This file is deprecated and retained only as a compatibility stub.

It previously tracked a per-close-out queue snapshot, but that duplicated the live
governance spine and drifted (its last real update was 2026-03-02 / NA-0176, while
the queue has since advanced past NA-0628). Live queue and decision truth are:

- `NEXT_ACTIONS.md` — the live queue (the sole `^Status: READY` lane, ON DECK, and the `STATE:` header)
- `DECISIONS.md` — the canonical decision log
- `TRACEABILITY.md` — goal → spec → implementation → test/vector traceability
- `docs/ops/IMPROVEMENT_LEDGER.md` — the prioritized backlog

Historical note:
- This file previously summarized NA-lane status through NA-0176.
- That content is historical and non-operative; do not treat it as current state.
