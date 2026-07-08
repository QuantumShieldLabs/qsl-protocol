Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0619 — ENG-0012 Suite-2 Send-Side Ratchet Liveness Feasibility + Design

## Summary

NA-0619 produces `docs/design/DOC-G5-008` — the feasibility + staged design for making the
Suite-2 ratchet actually re-key on the real client send path (ENG-0012, the P1 that blocks the
G1/G2 release gates). Docs-only under directive QSL-DIR-2026-07-08-556 (D556): one design doc +
governance, no source, no normative spec change. The document is a PLAN; it authorizes no
implementation and makes no post-compromise / Triple-Ratchet claim.

Result classification: `RATCHET_LIVENESS_DESIGN_COMPLETE`.

## Design study (read-only, confirmed at `8d9b158e`)

- C-1 confirmed: zero X25519 in `suite2`; `rk`/`hk_s`/`hk_r` assigned once in `establish.rs`;
  `Suite2SendState`/`Suite2RecvState` carry a fixed `dh_pub` (nonce input only) and have no
  X25519 private key, no `DHr`, and no live `RK`.
- C-2 confirmed: `send_wire` rejects nonzero `flags`; no `send_boundary`/`send_pq_ctxt`; the
  receive side (`recv_boundary_in_order`, `apply_pq_reseed`) is complete but unreachable.
- Scaffolding: DOC-CAN-003 §8.5.2/§8.5.3 fully specify sender + receiver; `qsp::dh_ratchet_send`
  is a working reference.
- Parse correction: `suite2/parse.rs` permits a bare `FLAG_BOUNDARY` (DH-only boundary); only
  the PQ flags require `FLAG_BOUNDARY`. The audit's "parse requires FLAG_PQ_CTXT whenever
  FLAG_BOUNDARY is set" is imprecise.
- Static-`rk` bootstrap confirmed in `qsc/src/main.rs` (`qsp_activate_*_chain_if_needed`).

## What the design specifies (DOC-G5-008)

Ten sections: problem + code-grounded proof; feasibility; trigger policy (ratchet-on-reply +
bounded count/time fallback; sparse PQ reseed); DH-only vs co-scheduled boundaries; the sender
construction plan (`send_boundary`/`send_pq_ctxt` mirroring the receiver + qsp reference) and
the required state-struct additions (DH keypair, `DHr`, live `RK`); qsc wiring (remove the
static-`rk` bootstrap; crash-safe persistence, G2); conformance-vector requirements (both
ratchets firing mid-session through the real send path; a PCS-healing vector); the
counter-hard-stop interaction (a live ratchet resets `Ns:=0`, mooting the ENG-0013 precondition);
a staged three-lane implementation plan; and open questions for operator decision. G5
traffic-shape note: boundary cadence has a metadata signature to size into existing padding.

## Operator decision captured

The operator accepted the design; the successor NA-0620 is the ENG-0012 Stage 1 implementation
lane (classical DH ratchet on the real send path + static-`rk` removal + two-party vectors).

## Claim boundary

Research/demo; design proposal only. No public/production/security-complete/crypto-complete/
post-compromise/Triple-Ratchet/quantum-secure claim. The document explicitly binds the project
NOT to make such claims until Stages 1–2 land and vectors pass.
