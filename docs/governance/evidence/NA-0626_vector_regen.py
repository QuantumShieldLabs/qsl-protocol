#!/usr/bin/env python3
"""NA-0626 (ENG-0024 + ENG-0026) vector regeneration — the NAMED, REVIEWED artifact.

Operator Decision 5: the artifact scope is exactly what the Phase-2(a) byte-scan
(NA-0626_wf0014_pinned_frame_scan_output.txt) proved, asserted FAIL-CLOSED here:

  CHANGED (exactly ONE existing vector, ZERO wire bytes):
    inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json
      - S2-SEND-PQRESEED-ACCEPT-0001: expect.output.new_state loses the `dh_rk` member
        (the removed duplicated root slot; its pinned value was byte-equal to `rk`).
        `wire_hex` and EVERY other member must be byte-identical, asserted below.
  BYTE-IDENTICAL: every other pre-existing vector in both files, and every OTHER vector
        file in inputs/suite2/vectors/ (sha256-pinned before/after).
  APPENDED (new ids only):
    qshield_suite2_scka_logic_vectors_v1.json
      + S2-SEND-COMBINED-ACCEPT-0001 (op suite2.send_combined_boundary; pinned wire from
        the new PURE sender, caller-supplied fresh X25519 keypair)
    qshield_suite2_pq_reseed_vectors_v1.json
      + 4 constructed combined-boundary receiver vectors (op suite2.combined_boundary.run):
        accept / NOT_IN_ORDER / no-DH-capability / body-tamper (the reject=>no-mutation
        shape: the actor op asserts snapshot-byte equality internally on EVERY reject).

Any deviation from this split aborts before writing (Operator Decision 5 STOP).
"""
from __future__ import annotations
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
TARGET = Path(os.environ.get("CARGO_TARGET_DIR", REPO / "target"))
ACTOR = TARGET / "debug/refimpl_actor"
VEC_DIR = REPO / "inputs/suite2/vectors"
SCKA = VEC_DIR / "qshield_suite2_scka_logic_vectors_v1.json"
PQRS = VEC_DIR / "qshield_suite2_pq_reseed_vectors_v1.json"

# ---------- pure-Python RFC 7748 X25519 (for pinning REAL keypairs deterministically) ----------
P = 2**255 - 19
A24 = 121665


def _clamp(k: bytes) -> int:
    a = bytearray(k)
    a[0] &= 248
    a[31] &= 127
    a[31] |= 64
    return int.from_bytes(bytes(a), "little")


def x25519(k: bytes, u: bytes) -> bytes:
    x1 = int.from_bytes(u, "little") & ((1 << 255) - 1)
    k_int = _clamp(k)
    x2, z2, x3, z3 = 1, 0, x1, 1
    swap = 0
    for t in reversed(range(255)):
        k_t = (k_int >> t) & 1
        swap ^= k_t
        if swap:
            x2, x3 = x3, x2
            z2, z3 = z3, z2
        swap = k_t
        a = (x2 + z2) % P
        aa = a * a % P
        b = (x2 - z2) % P
        bb = b * b % P
        e = (aa - bb) % P
        c = (x3 + z3) % P
        d = (x3 - z3) % P
        da = d * a % P
        cb = c * b % P
        x3 = (da + cb) % P
        x3 = x3 * x3 % P
        z3 = (da - cb) % P
        z3 = z3 * z3 % P
        z3 = z3 * x1 % P
        x2 = aa * bb % P
        z2 = e * (aa + A24 * e) % P
    if swap:
        x2, x3 = x3, x2
        z2, z3 = z3, z2
    out = x2 * pow(z2, P - 2, P) % P
    return out.to_bytes(32, "little")


def x25519_pub(priv_hex: str) -> str:
    return x25519(bytes.fromhex(priv_hex), (9).to_bytes(32, "little")).hex()


# ---------- actor plumbing (the NA-0625 regenerator pattern) ----------
def run_actor(req):
    p = subprocess.run(
        [str(ACTOR), "--name", "na0626-regen"],
        input=json.dumps(req) + "\n",
        capture_output=True,
        text=True,
        timeout=60,
    )
    line = p.stdout.splitlines()[0]
    return json.loads(line)


def actor_ok(vid, op, params):
    resp = run_actor({"id": vid, "op": op, "params": params})
    assert resp.get("ok") is True, f"{vid}: actor rejected: {resp.get('error')}"
    return resp["result"]


def actor_reject(vid, op, params, want_code):
    resp = run_actor({"id": vid, "op": op, "params": params})
    assert resp.get("ok") is not True, f"{vid}: expected reject, actor accepted"
    msg = (resp.get("error") or {}).get("message") or ""
    assert want_code in msg, f"{vid}: want {want_code} in error, got: {msg}"


H = lambda s: {"type": "hex", "data": s}
J = lambda v: {"type": "json", "data": v}

# ---------- deterministic combined-boundary fixture material ----------
# Receiver B (role A in the vector states? — the RECEIVER role below is "A" to mirror the
# existing pq_reseed constructed vectors) holds a REAL ratchet keypair; the sender supplies a
# REAL fresh keypair, so the actor's construct-side dh(new_priv, dhs_pub) and receive-side
# dh(dhs_priv, new_pub) agree.
RECV_DHS_PRIV = "40" * 32
RECV_DHS_PUB = x25519_pub(RECV_DHS_PRIV)
NEW_DH_PRIV = "50" * 32
NEW_DH_PUB = x25519_pub(NEW_DH_PRIV)
OLD_PEER_PUB = "2e" * 32  # the session's CURRENT peer key (differs from NEW_DH_PUB => combined)
PQ_CT = "aa" * 1088
PQ_SS = "5a" * 32


def combined_recv_send_state():
    # The receiver's send half (role A: a live send chain), refreshed by the combined receive.
    return {
        "session_id": "000102030405060708090a0b0c0d0e0f",
        "dh_pub": "1a" * 32,
        "hk_s": "3c" * 32,
        "ck_ec": "4d" * 32,
        "ck_pq": "5e" * 32,
        "ns": 3,
        "pn": 0,
    }


def combined_recv_recv_state():
    return {
        "session_id": "000102030405060708090a0b0c0d0e0f",
        "dh_pub": OLD_PEER_PUB,
        "hk_r": "303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f",
        "rk": "a5" * 32,
        "ck_ec": "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
        "ck_pq_send": "11" * 32,
        "ck_pq_recv": "22" * 32,
        "nr": 0,
        "role": "A",
        "peer_max_adv_id_seen": 40,
        "known_targets": [42],
        "consumed_targets": [],
        "tombstoned_targets": [],
        "mkskipped": [],
    }


def combined_recv_input(message):
    return {
        "send_state": J(combined_recv_send_state()),
        "recv_state": J(combined_recv_recv_state()),
        "dh_state": J(
            {"dhs_priv": RECV_DHS_PRIV, "dhs_pub": RECV_DHS_PUB, "dhr": OLD_PEER_PUB}
        ),
        "peer_adv_id": J({"u32": 41}),
        "message": J(message),
    }


def combined_msg(**over):
    m = {
        "new_dh_priv": NEW_DH_PRIV,
        "new_dh_pub": NEW_DH_PUB,
        "n": 0,
        "pn": 3,
        "pq_target_id": 42,
        "pq_ct_hex": PQ_CT,
        "pq_epoch_ss": PQ_SS,
        "body_pt_hex": "00010203",
        "tamper": "none",
        "hdr_key": "nhk",
    }
    m.update(over)
    return m


def combined_recv_vec(vid, kind, notes, message, expect):
    return {
        "id": vid,
        "op": "suite2.combined_boundary.run",
        "kind": kind,
        "tags": ["CAT-S2-PQRESEED-001", "CAT-S2-COMBINED-001", "G2"],
        "spec_refs": [
            "DOC-CAN-003 §3.3.6",
            "DOC-CAN-003 §8.5.1",
            "DOC-CAN-003 §8.5.2",
            "DOC-CAN-003 §8.5.3",
        ],
        "notes": notes,
        "input": combined_recv_input(message),
        "expect": expect,
    }


def sha256_file(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main():
    assert ACTOR.exists(), f"actor binary missing: {ACTOR}"

    # Cross-set guard (Operator Decision 5): pin every OTHER vector file before mutation.
    other_files = sorted(
        p for p in VEC_DIR.glob("*.json") if p.name not in (SCKA.name, PQRS.name)
    )
    other_hashes_before = {p.name: sha256_file(p) for p in other_files}

    scka = json.loads(SCKA.read_text())
    pqrs = json.loads(PQRS.read_text())
    pre_scka = {v["id"]: json.dumps(v, sort_keys=True) for v in scka["vectors"]}
    pre_pqrs = {v["id"]: json.dumps(v, sort_keys=True) for v in pqrs["vectors"]}

    # --- 1. THE ONE CHANGED VECTOR: S2-SEND-PQRESEED-ACCEPT-0001 loses new_state.dh_rk ---
    reseed = next(
        v for v in scka["vectors"] if v["id"] == "S2-SEND-PQRESEED-ACCEPT-0001"
    )
    old_output = json.loads(json.dumps(reseed["expect"]["output"]))
    result = actor_ok(reseed["id"], reseed["op"], reseed["input"])
    # Fail-closed Decision-5 assertions: zero wire bytes; the ONLY delta is the dh_rk member.
    assert result["wire_hex"] == old_output["wire_hex"], "wire bytes changed — STOP"
    old_state = dict(old_output["new_state"]["data"])
    new_state = dict(result["new_state"]["data"])
    assert "dh_rk" in old_state and "dh_rk" not in new_state, "dh_rk delta missing — STOP"
    assert old_state["dh_rk"] == old_state["rk"], "pinned dh_rk was not a duplicate — STOP"
    old_state.pop("dh_rk")
    assert old_state == new_state, "unexpected new_state delta beyond dh_rk — STOP"
    reseed["expect"]["output"] = result

    # --- 2. APPEND the combined-boundary SEND vector (pinned wire from the PURE sender) ---
    send_input = {
        "send_state": J(combined_recv_send_state()),
        "recv_state": J(combined_recv_recv_state()),
        "dh_state": J(
            {"dhs_priv": RECV_DHS_PRIV, "dhs_pub": RECV_DHS_PUB, "dhr": OLD_PEER_PUB}
        ),
        "new_dh_priv": H(NEW_DH_PRIV),
        "new_dh_pub": H(NEW_DH_PUB),
        "pq_target_id": J({"u32": 42}),
        "pq_ct": H(PQ_CT),
        "pq_epoch_ss": H(PQ_SS),
        "plaintext_hex": H("00010203"),
    }
    send_out = actor_ok(
        "S2-SEND-COMBINED-ACCEPT-0001", "suite2.send_combined_boundary", send_input
    )
    scka["vectors"].append(
        {
            "id": "S2-SEND-COMBINED-ACCEPT-0001",
            "op": "suite2.send_combined_boundary",
            "kind": "accept",
            "tags": ["CAT-SCKA-LOGIC-001", "CAT-S2-COMBINED-001", "G2"],
            "spec_refs": [
                "DOC-CAN-003 §3.3.6",
                "DOC-CAN-003 §8.5.1",
                "DOC-CAN-003 §8.5.2",
                "DOC-CAN-003 §8.5.3",
            ],
            "notes": "NA-0626 (ENG-0026) combined DH+PQ boundary SEND: one FLAG_BOUNDARY|FLAG_PQ_CTXT frame carrying a FRESH caller-supplied DH_pub; DH-first-then-PQ composition (RK_final = KDF_RK_PQ(KDF_RK_DH(RK_pre, dh_out), ss)); header sealed under the pre-boundary NHK_s at n=0 of the new DH epoch; body mk from the fresh PRE-seed epoch chains. Byte-layout identical to the existing 0x0006 reseed frame (no wire FORMAT change).",
            "input": send_input,
            "expect": {"ok": True, "output": send_out},
        }
    )

    # --- 3. APPEND the 4 constructed combined-boundary RECEIVE vectors ---
    accept_msg = combined_msg()
    accept_out = actor_ok(
        "S2-RECV-COMBINED-ACCEPT-0001",
        "suite2.combined_boundary.run",
        combined_recv_input(accept_msg),
    )
    new_pqrs = [
        combined_recv_vec(
            "S2-RECV-COMBINED-ACCEPT-0001",
            "positive",
            "NA-0626 (ENG-0026) combined DH+PQ boundary RECEIVE (session-level recv_pq_reseed, fresh-DH_pub discrimination): NHK-only open at n=0 of the NEW DH epoch (nonce/AD on the fresh DH_pub), DH ratchet first from the pre-boundary root, then the frozen apply_pq_reseed from RK_dh, RK_final = KDF_RK_PQ(RK_dh, ss); the returned state carries the FULL directional schedule (root, dhr, both header keys, both PQ chains, nr=1) — the ENG-0030 structural guarantee.",
            accept_msg,
            {"ok": True, "output": accept_out},
        ),
        combined_recv_vec(
            "S2-RECV-COMBINED-REJECT-NOT-IN-ORDER-0001",
            "negative",
            "The combined frame is n=0 of the new DH epoch ONLY: a header claiming n=1 (sealed at the n=0 nonce so it opens) rejects REJECT_S2_BOUNDARY_NOT_IN_ORDER with no state mutation (asserted inside the op).",
            combined_msg(n=1),
            {"ok": False, "reason_code": "REJECT_S2_BOUNDARY_NOT_IN_ORDER"},
        ),
        combined_recv_vec(
            "S2-RECV-COMBINED-REJECT-NO-DH-CAPABILITY-0001",
            "negative",
            "A fresh DH_pub against a session with no local DH capability (dhs_priv zero — the actor plumbing-session shape) rejects REJECT_S2_LOCAL_UNSUPPORTED before any crypto, with no state mutation (asserted inside the op).",
            combined_msg(),
            {"ok": False, "reason_code": "REJECT_S2_LOCAL_UNSUPPORTED"},
        ),
        combined_recv_vec(
            "S2-RECV-COMBINED-REJECT-BODY-TAMPER-0001",
            "negative",
            "reject => no mutation: a tampered combined body fails REJECT_S2_BODY_AUTH_FAIL and the input session state is returned byte-identical (snapshot-byte equality asserted inside the op for EVERY reject shape).",
            combined_msg(tamper="body"),
            {"ok": False, "reason_code": "REJECT_S2_BODY_AUTH_FAIL"},
        ),
    ]
    # The no-DH-capability vector zeroes dhs_priv in dh_state (input-level knob).
    new_pqrs[2]["input"]["dh_state"] = J(
        {"dhs_priv": "00" * 32, "dhs_pub": RECV_DHS_PUB, "dhr": OLD_PEER_PUB}
    )
    # Verify every reject against the live actor before pinning.
    for v in new_pqrs[1:]:
        actor_reject(v["id"], v["op"], v["input"], v["expect"]["reason_code"])
    pqrs["vectors"].extend(new_pqrs)

    # --- 4. The fail-closed Decision-5 split proof ---
    proof = {
        "byte_identical": [],
        "changed": [],
        "appended": [],
        "other_files_sha256": other_hashes_before,
    }
    for v in scka["vectors"]:
        vid = v["id"]
        if vid in pre_scka:
            same = json.dumps(v, sort_keys=True) == pre_scka[vid]
            (proof["byte_identical"] if same else proof["changed"]).append(
                f"scka_logic:{vid}"
            )
        else:
            proof["appended"].append(f"scka_logic:{vid}")
    for v in pqrs["vectors"]:
        vid = v["id"]
        if vid in pre_pqrs:
            same = json.dumps(v, sort_keys=True) == pre_pqrs[vid]
            (proof["byte_identical"] if same else proof["changed"]).append(
                f"pq_reseed:{vid}"
            )
        else:
            proof["appended"].append(f"pq_reseed:{vid}")
    assert proof["changed"] == ["scka_logic:S2-SEND-PQRESEED-ACCEPT-0001"], proof["changed"]
    assert len(proof["byte_identical"]) == len(pre_scka) - 1 + len(pre_pqrs), proof
    assert sorted(proof["appended"]) == [
        "pq_reseed:S2-RECV-COMBINED-ACCEPT-0001",
        "pq_reseed:S2-RECV-COMBINED-REJECT-BODY-TAMPER-0001",
        "pq_reseed:S2-RECV-COMBINED-REJECT-NO-DH-CAPABILITY-0001",
        "pq_reseed:S2-RECV-COMBINED-REJECT-NOT-IN-ORDER-0001",
        "scka_logic:S2-SEND-COMBINED-ACCEPT-0001",
    ], proof["appended"]

    SCKA.write_text(json.dumps(scka, indent=2) + "\n")
    PQRS.write_text(json.dumps(pqrs, indent=2) + "\n")

    # Cross-set guard re-check: no OTHER vector file may have changed.
    other_hashes_after = {p.name: sha256_file(p) for p in other_files}
    assert other_hashes_after == other_hashes_before, "cross-set drift — STOP"

    print(json.dumps({k: v for k, v in proof.items() if k != "other_files_sha256"}, indent=1))
    print(
        f"OK: 1 changed (dh_rk removed, zero wire bytes), "
        f"{len(proof['byte_identical'])} byte-identical, {len(proof['appended'])} appended"
    )


if __name__ == "__main__":
    main()
