#!/usr/bin/env python3
"""NA-0628 (ENG-0034) — ADDITIVE negative conformance vectors for REJECT_S2_DH_NONCONTRIBUTORY.

Operator Decision 4 (D565): vectors are ADDITIVE ONLY. This script CLONES an existing accepting
vector and changes EXACTLY ONE FIELD, so the negative case differs from the honest transcript only in
the peer's DH public key. It never edits an existing vector.

Why `u=1` (0x01 || 0x00*31) and not the all-zero encoding: `is_zero32(&parsed.dh_pub)`
(ratchet.rs:1420, :2317) already rejects the all-zero ENCODING with REJECT_S2_HDR_AUTH_FAIL. `u=1` is
one of the seven low-order points that PASS that ingress check and still drive the X25519 output to
all-zero (RFC 7748 §6.1). It therefore reaches — and proves — the new OUTPUT guard.

Both files round-trip byte-exactly under `json.dumps(data, indent=2) + "\n"`, so every pre-existing
line is preserved verbatim. Idempotent: re-running is a no-op.

Run from the repo root:  python3 docs/governance/evidence/NA-0628_vector_add.py
Verify with:             python3 docs/governance/evidence/NA-0628_vector_byte_scan.py
"""
from __future__ import annotations

import copy
import json
import sys
from pathlib import Path

LOW_ORDER_U1 = "01" + "00" * 31

PQ_RESEED = Path("inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json")
SCKA_LOGIC = Path("inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json")

RECV_ID = "S2-RECV-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001"
SEND_ID = "S2-SEND-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001"

RFC = "RFC 7748 §6.1"
NOTE_RECV = (
    "ENG-0034: the peer advertises a low-order DH_pub (u=1). The frame is honestly sealed, so it "
    "authenticates; the X25519 output is all-zero, contributing no entropy to the new root. The "
    "receiver MUST reject with REJECT_S2_DH_NONCONTRIBUTORY and MUST NOT mutate state. Note that "
    "u=1 is NOT the all-zero encoding, so it passes the pre-existing is_zero32(dh_pub) screen: this "
    "vector proves the DH OUTPUT check, not the encoding check."
)
NOTE_SEND = (
    "ENG-0034: the stored peer key DHr is a low-order point (u=1), so the sender's own X25519 output "
    "is all-zero. The sender MUST reject with REJECT_S2_DH_NONCONTRIBUTORY before the root advances."
)


def load(p: Path) -> dict:
    return json.loads(p.read_text(encoding="utf-8"))


def dump(p: Path, data: dict) -> None:
    p.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def find(data: dict, vid: str) -> dict:
    for v in data["vectors"]:
        if v["id"] == vid:
            return v
    raise SystemExit(f"FATAL: base vector {vid} not found (the vector set moved; STOP)")


def already(data: dict, vid: str) -> bool:
    return any(v["id"] == vid for v in data["vectors"])


def add_recv_vector() -> bool:
    data = load(PQ_RESEED)
    if already(data, RECV_ID):
        return False
    base = find(data, "S2-RECV-COMBINED-ACCEPT-0001")
    v = copy.deepcopy(base)
    v["id"] = RECV_ID
    v["kind"] = "negative"
    v["tags"] = ["CAT-S2-PQRESEED-001", "CAT-S2-COMBINED-001", "G2", "G4"]
    v["spec_refs"] = sorted(set(base.get("spec_refs", [])) | {RFC})
    v["notes"] = NOTE_RECV
    # THE ONE FIELD: the peer's fresh DH public key becomes a small-subgroup point.
    assert v["input"]["message"]["data"]["new_dh_pub"] != LOW_ORDER_U1
    v["input"]["message"]["data"]["new_dh_pub"] = LOW_ORDER_U1
    v["expect"] = {"ok": False, "reason_code": "REJECT_S2_DH_NONCONTRIBUTORY"}
    data["vectors"].append(v)
    dump(PQ_RESEED, data)
    return True


def add_send_vector() -> bool:
    data = load(SCKA_LOGIC)
    if already(data, SEND_ID):
        return False
    base = find(data, "S2-SEND-COMBINED-ACCEPT-0001")
    v = copy.deepcopy(base)
    v["id"] = SEND_ID
    v["kind"] = "reject"
    v["tags"] = ["CAT-SCKA-LOGIC-001", "CAT-S2-COMBINED-001", "G2", "G4"]
    v["spec_refs"] = sorted(set(base.get("spec_refs", [])) | {RFC})
    v["notes"] = NOTE_SEND
    # THE ONE FIELD: the stored peer key is a small-subgroup point.
    assert v["input"]["dh_state"]["data"]["dhr"] != LOW_ORDER_U1
    v["input"]["dh_state"]["data"]["dhr"] = LOW_ORDER_U1
    v["expect"] = {"ok": False, "reason_code": "REJECT_S2_DH_NONCONTRIBUTORY"}
    data["vectors"].append(v)
    dump(SCKA_LOGIC, data)
    return True


def main() -> int:
    if not PQ_RESEED.exists() or not SCKA_LOGIC.exists():
        print("FATAL: run from the repo root", file=sys.stderr)
        return 2
    added_recv = add_recv_vector()
    added_send = add_send_vector()
    print(f"{RECV_ID}: {'ADDED' if added_recv else 'already present'}")
    print(f"{SEND_ID}: {'ADDED' if added_send else 'already present'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
