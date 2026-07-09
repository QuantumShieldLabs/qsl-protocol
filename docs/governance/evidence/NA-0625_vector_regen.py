#!/usr/bin/env python3
"""NA-0625 (ENG-0023) vector regeneration — the NAMED, REVIEWED artifact.

Mutates ONLY the two named files:
  inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json
    - REPLACE expect.output of S2-SEND-PQADV-ACCEPT-0001 (body_ct +32 ADVAUTH MAC)
    - REPLACE expect.output of S2-SEND-PQRESEED-ACCEPT-0001 (hdr_ct under NHK)
    - APPEND 5 ADV-receive vectors (op suite2.recv_pq_adv)
  inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json
    - APPEND 2 CTXT-NHK receive vectors (op suite2.boundary.run)

Emits a byte-identity proof for every untouched pre-existing vector.
"""
from __future__ import annotations
import json, subprocess, sys, copy
from pathlib import Path

REPO = Path("/srv/qbuild/work/NA-0625/qsl-protocol")
ACTOR = REPO / "target/debug/refimpl_actor"
SCKA = REPO / "inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json"
PQRS = REPO / "inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json"

def run_actor(req):
    p = subprocess.run([str(ACTOR), "--name", "na0625-regen"], input=json.dumps(req) + "\n",
                       capture_output=True, text=True, timeout=30)
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
ADV_PUB = "bb" * 1184

def adv_common_state():
    return {
        "negotiated": J({"protocol_version": "0x0500", "suite_id": "0x0002"}),
        "role": J("A"),
        "session_id": H("11111111111111111111111111111111"),
        "dh_pub": H("22" * 32),
        "hk_r": H("66" * 32),
        "rk": H("77" * 32),
        "ck_ec0": H("88" * 32),
        "ck_pq_send0": H("99" * 32),
        "ck_pq_recv0": H("aa" * 32),
        "nr0": J({"u32": 0}),
        "peer_max_adv_id_seen": J({"u32": 0}),
        "peer_adv_watermark": J({"u32": 0}),
    }

def adv_vec(vid, kind, notes, message, expect, watermark=0):
    inp = adv_common_state()
    inp["peer_adv_watermark"] = J({"u32": watermark})
    inp["message"] = J(message)
    return {
        "id": vid,
        "op": "suite2.recv_pq_adv",
        "kind": kind,
        "tags": ["CAT-SCKA-LOGIC-001", "G2"],
        "spec_refs": ["DOC-CAN-004 §3.2", "DOC-CAN-003 §8.5.4"],
        "notes": notes,
        "input": inp,
        "expect": expect,
    }

def main():
    scka = json.loads(SCKA.read_text())
    pqrs = json.loads(PQRS.read_text())
    pre_scka = {v["id"]: json.dumps(v, sort_keys=True) for v in scka["vectors"]}
    pre_pqrs = {v["id"]: json.dumps(v, sort_keys=True) for v in pqrs["vectors"]}

    # --- 1. Regenerate the two SEND-ACCEPT expects (bytes change; input untouched) ---
    changed = {}
    for v in scka["vectors"]:
        if v["id"] in ("S2-SEND-PQADV-ACCEPT-0001", "S2-SEND-PQRESEED-ACCEPT-0001"):
            old = json.dumps(v["expect"]["output"], sort_keys=True)
            result = actor_ok(v["id"], v["op"], v["input"])
            v["expect"]["output"] = result
            changed[v["id"]] = (old != json.dumps(result, sort_keys=True))
    assert changed.get("S2-SEND-PQADV-ACCEPT-0001") is True
    assert changed.get("S2-SEND-PQRESEED-ACCEPT-0001") is True

    # Invariant pinned at design-lock: only wire bytes change in those two vectors —
    # new_state stays semantically identical for the ADV vector; for the reseed vector all
    # new_state keys derive from the (unchanged) new root. Verified below via the runner.

    # --- 2. Append the 5 ADV-receive vectors ---
    msg_ok = {"n": 0, "pn": 0, "pq_adv_id": 1, "pq_adv_pub_hex": ADV_PUB,
              "body_payload_hex": "00010203", "tamper": "none", "mac": "ok"}
    base = adv_common_state()

    accept_params = dict(base); accept_params["message"] = J(msg_ok)
    accept_out = actor_ok("S2-RECV-PQADV-ACCEPT-0001", "suite2.recv_pq_adv", accept_params)
    new_scka = [
        adv_vec("S2-RECV-PQADV-ACCEPT-0001", "accept",
                "NA-0625 authenticated ADV receive: header under HK_r, body under the in-order hybrid mk, ADVAUTH MAC (KMAC32(RK, 'QSP5.0/ADVAUTH', u32be(id)||pub||[0x01]), first 32 bytes of body_pt) verified under the canonical root; chain-consume advances BOTH receive chains + nr (Operator Decision 2); the peer-ADV watermark is caller-owned (the frozen CTXT-path field is untouched).",
                msg_ok, {"ok": True, "output": accept_out}),
        adv_vec("S2-RECV-PQADV-REJECT-SPOOFED-0001", "reject",
                "A planted/foreign-key ADV (header sealed under an attacker key, as a relay-inbox injector would have to) fails FIRST at the header AEAD: REJECT_S2_HDR_AUTH_FAIL, no mutation.",
                {**msg_ok, "hdr_key_hex": "ee" * 32},
                {"ok": False, "reason_code": "REJECT_S2_HDR_AUTH_FAIL"}),
        adv_vec("S2-RECV-PQADV-REJECT-BADMAC-0001", "reject",
                "Valid AEAD frame with a corrupted ADVAUTH MAC: REJECT_S2_BODY_AUTH_FAIL (reused code, no new normative reason), no mutation.",
                {**msg_ok, "mac": "corrupt"},
                {"ok": False, "reason_code": "REJECT_S2_BODY_AUTH_FAIL"}),
        adv_vec("S2-RECV-PQADV-REJECT-REPLAY-0001", "reject",
                "Replayed/non-monotonic advertisement id (id <= the caller-owned peer-ADV watermark): REJECT_SCKA_ADV_NONMONOTONIC. (A byte-replayed old frame is independently rejected earlier by the header AEAD counter mismatch — the SPOOFED vector's arm.)",
                {**msg_ok, "pq_adv_id": 1}, {"ok": False, "reason_code": "REJECT_SCKA_ADV_NONMONOTONIC"},
                watermark=1),
        adv_vec("S2-RECV-PQADV-REJECT-NOMAC-0001", "reject",
                "Pre-NA-0625-format ADV body (no leading ADVAUTH MAC — the ADV downgrade case): REJECT_S2_BODY_AUTH_FAIL, no mutation.",
                {**msg_ok, "mac": "missing", "body_payload_hex": ""},
                {"ok": False, "reason_code": "REJECT_S2_BODY_AUTH_FAIL"}),
    ]
    # Verify the rejects against the live actor before pinning.
    for v in new_scka[1:]:
        actor_reject(v["id"], v["op"], v["input"], v["expect"]["reason_code"])
    scka["vectors"].extend(new_scka)

    # --- 3. Append the 2 CTXT-NHK vectors to the pq_reseed file ---
    ctxt_state = {
        "negotiated": J({"protocol_version": "0x0500", "suite_id": "0x0002"}),
        "role": J({"role": "A"}),
        "session_id": H("000102030405060708090a0b0c0d0e0f"),
        "dh_pub": H("101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f"),
        "hk_r": H("303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f"),
        "rk": H("a5" * 32),
        "ck_ec0": H("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"),
        "ck_pq_send0": H("11" * 32),
        "ck_pq_recv0": H("22" * 32),
        "nr0": J({"u32": 0}),
        "peer_max_adv_id_seen": J({"u32": 40}),
        "known_targets": J([42]),
        "consumed_targets": J([]),
        "tombstoned_targets": J([]),
    }
    ctxt_msg = {"pn": 0, "n": 0, "flags": {"u16": 6},
                "pq_prefix_hex": "0000002a" + "aa" * 1088,
                "pq_epoch_ss": "5a" * 32, "body_pt_hex": "00010203", "tamper": "none"}
    nhk_params = dict(ctxt_state); nhk_params["message"] = J(dict(ctxt_msg))
    nhk_out = actor_ok("S2-RECV-PQRESEED-NHK-ACCEPT-0001", "suite2.boundary.run", nhk_params)
    dg_msg = dict(ctxt_msg); dg_msg["hdr_key"] = "hk"
    dg_params = dict(ctxt_state); dg_params["message"] = J(dg_msg)
    actor_reject("S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001", "suite2.boundary.run",
                 dg_params, "REJECT_S2_HDR_AUTH_FAIL")

    def ctxt_vec(vid, kind, notes, params, expect):
        return {"id": vid, "op": "suite2.boundary.run", "kind": kind,
                "tags": ["CAT-S2-PQRESEED-001", "G2"],
                "spec_refs": ["DOC-CAN-003 §8.5.1", "DOC-CAN-003 §8.5.3"],
                "notes": notes, "input": params, "expect": expect}

    pqrs["vectors"].extend([
        ctxt_vec("S2-RECV-PQRESEED-NHK-ACCEPT-0001", "positive",
                 "NA-0625 §8.5.1 alignment: the PQ-CTXT boundary header opens ONLY under the receiver's CURRENT NHK_r derived from the pre-reseed root (§8.5.3 step 1, 'Require hdr_source == CURRENT_NHK'); the frame is NHK-sealed exactly as send_pq_reseed now seals it.",
                 nhk_params, {"ok": True, "output": nhk_out}),
        ctxt_vec("S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001", "negative",
                 "Header-downgrade rejection (DoD 4): a pre-NA-0625-style boundary frame sealed under the ordinary HK_r MUST fail generically with REJECT_S2_HDR_AUTH_FAIL (NHK-only open; no HK trial, no new reason code).",
                 dg_params, {"ok": False, "reason_code": "REJECT_S2_HDR_AUTH_FAIL"}),
    ])

    # --- 4. Byte-identity proof for untouched pre-existing vectors ---
    proof = {"byte_identical": [], "changed": [], "appended": []}
    for v in scka["vectors"]:
        vid = v["id"]
        if vid in pre_scka:
            same = json.dumps(v, sort_keys=True) == pre_scka[vid]
            (proof["byte_identical"] if same else proof["changed"]).append(f"scka_logic:{vid}")
        else:
            proof["appended"].append(f"scka_logic:{vid}")
    for v in pqrs["vectors"]:
        vid = v["id"]
        if vid in pre_pqrs:
            same = json.dumps(v, sort_keys=True) == pre_pqrs[vid]
            (proof["byte_identical"] if same else proof["changed"]).append(f"pq_reseed:{vid}")
        else:
            proof["appended"].append(f"pq_reseed:{vid}")
    assert sorted(proof["changed"]) == ["scka_logic:S2-SEND-PQADV-ACCEPT-0001",
                                        "scka_logic:S2-SEND-PQRESEED-ACCEPT-0001"], proof["changed"]
    assert len(proof["byte_identical"]) == 17, proof
    assert len(proof["appended"]) == 7, proof

    SCKA.write_text(json.dumps(scka, indent=2) + "\n")
    PQRS.write_text(json.dumps(pqrs, indent=2) + "\n")
    print(json.dumps(proof, indent=1))
    print("OK: 2 replaced, 17 byte-identical, 7 appended")

if __name__ == "__main__":
    main()
