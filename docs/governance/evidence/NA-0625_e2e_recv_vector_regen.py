#!/usr/bin/env python3
"""NA-0625 (ENG-0023) — bounded regeneration of the THIRD named vector file.

Authorized by Operator Decision 5 (D562 addendum, 2026-07-09), which extends the NAMED, REVIEWED
vector-file list from two files to three and BOUNDS the third file's mutation to:

  * only the vector `S2-E2E-ACCEPT-BOUNDARY-0001`;
  * within it, only `input.steps[0].wire_hex`;
  * only the 24 header-ciphertext bytes at [1136, 1160), re-sealed under the §8.5.1 NHK derived
    from that vector's own `recv_state.rk`;
  * every other wire byte, every other field of that vector, and all three sibling vectors must be
    asserted BYTE-IDENTICAL.

This script enforces all of those bounds and fails closed if any is violated.

The replacement header ciphertext is produced by the reference implementation itself, not by a
re-derivation here: we drive `suite2.send_pq_reseed` through the refimpl actor, configured as the
PEER that originally produced this frame (role B, sending to the vector's role-A receiver), with the
receiver's chains as its send chains. That sender now seals the boundary header under
`NHK_s = header_key(rk_old, "B->A", next=true)`, which is bit-for-bit the key the vector's receiver
derives as `nhk_r`. The actor returns the whole wire; we assert it differs from the pinned wire in
exactly the [1136, 1160) window before splicing.
"""
from __future__ import annotations
import json, subprocess
from pathlib import Path

REPO = Path("/srv/qbuild/work/NA-0625/qsl-protocol")
ACTOR = REPO / "target/debug/refimpl_actor"
E2E = REPO / "inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json"

TARGET_ID = "S2-E2E-ACCEPT-BOUNDARY-0001"
HDR_CT_LO, HDR_CT_HI = 1136, 1160  # envelope(10) + dh_pub(32) + flags(2) + id(4) + ct(1088)

H = lambda s: {"type": "hex", "data": s}
J = lambda v: {"type": "json", "data": v}


def run_actor(req):
    p = subprocess.run([str(ACTOR), "--name", "na0625-e2e-regen"],
                       input=json.dumps(req) + "\n", capture_output=True, text=True, timeout=30)
    resp = json.loads(p.stdout.splitlines()[0])
    assert resp.get("ok") is True, f"actor rejected: {resp.get('error')}"
    return resp["result"]


def main() -> int:
    doc = json.loads(E2E.read_text())
    before = {v["id"]: json.dumps(v, sort_keys=True) for v in doc["vectors"]}

    vec = next(v for v in doc["vectors"] if v["id"] == TARGET_ID)
    rs = vec["input"]["recv_state"]["data"]
    step = vec["input"]["steps"]["data"][0]
    old_wire = bytes.fromhex(step["wire_hex"])
    assert len(old_wire) == 1180, f"unexpected wire length {len(old_wire)}"
    assert int.from_bytes(old_wire[42:44], "big") == 0x0006, "not a PQ_CTXT|BOUNDARY frame"

    session_id = rs["session_id"]
    dh_pub = rs["dh_pub"]
    rk = rs["rk"]
    assert rs["role"] == "A", "vector receiver must be role A"

    target_id = int.from_bytes(old_wire[44:48], "big")
    pq_ct = old_wire[48:HDR_CT_LO].hex()
    assert len(old_wire[48:HDR_CT_LO]) == 1088

    # The originating PEER: role B, sending A<-B. Its send chains ARE the receiver's receive chains,
    # and its session root is the same pre-reseed RK. Everything else in the frame is fixed by the
    # vector. `hk_s` is unused by the NHK sender but must be present and non-zero.
    params = {
        "send_state": J({
            "session_id": session_id,
            "dh_pub": dh_pub,
            "hk_s": rs["hk_r"],
            "ck_ec": rs["ck_ec"],
            "ck_pq": rs["ck_pq_recv"],
            "ns": 0,
            "pn": 0,
        }),
        "recv_state": J({
            "session_id": session_id,
            "dh_pub": dh_pub,
            "hk_r": rs["hk_r"],
            "rk": rk,
            "ck_ec": rs["ck_ec"],
            "ck_pq_send": rs["ck_pq_send"],
            "ck_pq_recv": rs["ck_pq_recv"],
            "nr": 0,
            "role": "B",
            "peer_max_adv_id_seen": 0,
            "known_targets": [],
            "consumed_targets": [],
            "tombstoned_targets": [],
            "mkskipped": [],
        }),
        "pq_target_id": J({"u32": target_id}),
        "pq_ct": H(pq_ct),
        "pq_epoch_ss": H(step["pq_epoch_ss"]),
        "plaintext_hex": H(step["expect_plaintext_hex"]),
    }
    out = run_actor({"id": "regen", "op": "suite2.send_pq_reseed", "params": params})
    new_wire = bytes.fromhex(out["wire_hex"]["data"])

    # BOUND 1: the regenerated frame differs from the pinned frame in EXACTLY the hdr_ct window.
    assert len(new_wire) == len(old_wire), "wire length changed"
    diff = [i for i in range(len(old_wire)) if old_wire[i] != new_wire[i]]
    assert diff, "no change produced (was the NHK sender applied?)"
    assert min(diff) >= HDR_CT_LO and max(diff) < HDR_CT_HI, (
        f"change escaped the hdr_ct window: bytes {min(diff)}..{max(diff)}")
    assert old_wire[:HDR_CT_LO] == new_wire[:HDR_CT_LO], "prefix changed"
    assert old_wire[HDR_CT_HI:] == new_wire[HDR_CT_HI:], "body_ct changed"

    spliced = old_wire[:HDR_CT_LO] + new_wire[HDR_CT_LO:HDR_CT_HI] + old_wire[HDR_CT_HI:]
    assert spliced == new_wire

    # BOUND 2: only `wire_hex` changes on the target vector.
    other_fields_before = {k: v for k, v in step.items() if k != "wire_hex"}
    step["wire_hex"] = spliced.hex()
    other_fields_after = {k: v for k, v in step.items() if k != "wire_hex"}
    assert other_fields_before == other_fields_after

    # BOUND 3: exactly one vector changed; the other three are byte-identical.
    changed, identical = [], []
    for v in doc["vectors"]:
        (changed if json.dumps(v, sort_keys=True) != before[v["id"]] else identical).append(v["id"])
    assert changed == [TARGET_ID], f"unexpected changed set: {changed}"
    assert len(identical) == 3, f"expected 3 byte-identical siblings, got {identical}"

    E2E.write_text(json.dumps(doc, indent=2) + "\n")
    print(json.dumps({
        "file": E2E.name,
        "changed": changed,
        "changed_field": "input.steps[0].wire_hex",
        "changed_byte_range": [HDR_CT_LO, HDR_CT_HI],
        "changed_bytes": len(diff),
        "old_hdr_ct": old_wire[HDR_CT_LO:HDR_CT_HI].hex(),
        "new_hdr_ct": spliced[HDR_CT_LO:HDR_CT_HI].hex(),
        "byte_identical_siblings": identical,
        "expect_fields_unchanged": True,
    }, indent=1))
    print("OK: 1 vector, 1 field, 24 bytes; 3 siblings byte-identical")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
