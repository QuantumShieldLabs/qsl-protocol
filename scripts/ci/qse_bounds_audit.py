#!/usr/bin/env python3
from __future__ import annotations

import argparse, io, json, os, struct, zipfile

# Frozen QSE 1.8.2 bounds (do not modify without updating Phase 2 canonical docs)
MAX_ROUTE_TOKEN_LEN = 512
MAX_PAYLOAD_LEN     = 1_048_576
MAX_PAD_LEN         = 1_048_576
MAX_ENVELOPE_LEN    = 2_097_152
ENV_VERSION_1X      = 0x0100
FLAGS_REQUIRED      = 0x0000

class ParseError(Exception):
    pass

def read_u16(b: bytes, off: int) -> tuple[int, int]:
    if off + 2 > len(b):
        raise ParseError("truncated_u16")
    return (struct.unpack(">H", b[off:off+2])[0], off+2)

def read_u32(b: bytes, off: int) -> tuple[int, int]:
    if off + 4 > len(b):
        raise ParseError("truncated_u32")
    return (struct.unpack(">I", b[off:off+4])[0], off+4)

def parse_varbytes_u16(b: bytes, off: int) -> tuple[bytes, int]:
    ln, off = read_u16(b, off)
    if off + ln > len(b):
        raise ParseError("varbytes_overrun")
    return (b[off:off+ln], off+ln)

def parse_qse_envelope(b: bytes) -> dict:
    if len(b) > MAX_ENVELOPE_LEN:
        raise ParseError("envelope_oversize")
    off = 0
    env_version, off = read_u16(b, off)
    flags, off = read_u16(b, off)
    if env_version != ENV_VERSION_1X:
        raise ParseError("unknown_env_version")
    if flags != FLAGS_REQUIRED:
        raise ParseError("nonzero_flags")

    route_token, off = parse_varbytes_u16(b, off)
    if len(route_token) > MAX_ROUTE_TOKEN_LEN:
        raise ParseError("route_token_oversize")

    timestamp_bucket, off = read_u16(b, off)

    payload_len, off = read_u32(b, off)
    if payload_len > MAX_PAYLOAD_LEN:
        raise ParseError("payload_oversize")
    if off + payload_len > len(b):
        raise ParseError("payload_overrun")
    off += payload_len

    pad_len, off = read_u16(b, off)
    if pad_len > MAX_PAD_LEN:
        raise ParseError("pad_oversize")
    if off + pad_len > len(b):
        raise ParseError("pad_overrun")
    off += pad_len

    if off != len(b):
        raise ParseError("trailing_bytes")

    return {
        "env_version": env_version,
        "flags": flags,
        "route_token_len": len(route_token),
        "timestamp_bucket": timestamp_bucket,
        "payload_len": payload_len,
        "pad_len": pad_len
    }

def find_p3_23_full_zip(z: zipfile.ZipFile) -> str | None:
    cands = [n for n in z.namelist()
             if not n.endswith("/")
             and n.lower().endswith(".zip")
             and ("p3-23" in n.lower() or "p3_23" in n.lower())
             and ("full" in n.lower())]
    return sorted(cands)[0] if cands else None

def find_oversize_blob(nz: zipfile.ZipFile) -> str | None:
    exact = [n for n in nz.namelist() if os.path.basename(n) == "qse_payload_oversize_1048577.bin"]
    if exact:
        return exact[0]
    cands = [n for n in nz.namelist()
             if not n.endswith("/")
             and os.path.basename(n).startswith("qse_payload_oversize_")
             and n.lower().endswith(".bin")]
    return sorted(cands)[0] if cands else None

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--phase2-zip", required=True)
    ap.add_argument("--phase3-zip", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    report = {"ok": False, "tests": [], "errors": []}

    blob = None
    with zipfile.ZipFile(args.phase3_zip, "r") as z:
        p3_23 = find_p3_23_full_zip(z)
        if not p3_23:
            report["errors"].append("P3-23 FULL zip not found in Phase3 input; cannot run oversize blob test")
        else:
            nz = zipfile.ZipFile(io.BytesIO(z.read(p3_23)), "r")
            blob_member = find_oversize_blob(nz)
            if not blob_member:
                report["errors"].append("No qse_payload_oversize_*.bin found in P3-23 FULL zip")
            else:
                blob = nz.read(blob_member)
                report["tests"].append({"name": "loaded_oversize_blob", "ok": True, "member": blob_member, "bytes": len(blob)})

    if blob is not None:
        try:
            _ = parse_qse_envelope(blob)
            report["tests"].append({"name": "qse_oversize_blob_rejected", "ok": False})
            report["errors"].append("Oversize blob unexpectedly accepted")
        except ParseError as e:
            report["tests"].append({"name": "qse_oversize_blob_rejected", "ok": True, "error": str(e)})

    minimal = struct.pack(">H", ENV_VERSION_1X) + struct.pack(">H", FLAGS_REQUIRED)
    minimal += struct.pack(">H", 0)   # route_token len=0
    minimal += struct.pack(">H", 0)   # timestamp_bucket
    minimal += struct.pack(">I", 0)   # payload_len
    minimal += struct.pack(">H", 0)   # pad_len
    try:
        _ = parse_qse_envelope(minimal)
        report["tests"].append({"name": "qse_minimal_parses", "ok": True})
    except ParseError as e:
        report["tests"].append({"name": "qse_minimal_parses", "ok": False, "error": str(e)})
        report["errors"].append(f"Minimal envelope rejected: {e}")

    report["ok"] = (len(report["errors"]) == 0)
    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, sort_keys=True)
    return 0 if report["ok"] else 2

if __name__ == "__main__":
    raise SystemExit(main())
