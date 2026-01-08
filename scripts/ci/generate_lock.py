#!/usr/bin/env python3
from __future__ import annotations

import argparse, hashlib, io, json, os, zipfile

def sha256_bytes(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--phase3-zip", required=True)
    ap.add_argument("--out-lock", required=True)
    args = ap.parse_args()

    z = zipfile.ZipFile(args.phase3_zip, "r")
    top = [n for n in z.namelist() if not n.endswith("/")]
    entries = []

    for m in top:
        b = z.read(m)
        entries.append({
            "logical_path": m,
            "bytes": len(b),
            "sha256": sha256_bytes(b),
            "source": {"type": "top", "member": m}
        })

    for m in top:
        if not m.lower().endswith(".zip"):
            continue
        bn = os.path.basename(m)
        try:
            nz = zipfile.ZipFile(io.BytesIO(z.read(m)), "r")
        except Exception:
            continue
        for nm in nz.namelist():
            if nm.endswith("/"):
                continue
            b = nz.read(nm)
            entries.append({
                "logical_path": f"{bn}::{nm}",
                "bytes": len(b),
                "sha256": sha256_bytes(b),
                "source": {"type": "nested", "zip_basename": bn, "member": nm}
            })

    lock = {
        "format": "QSHIELD-INPUT-LOCK-1",
        "phase": 3,
        "zip_basename": os.path.basename(args.phase3_zip),
        "entries": sorted(entries, key=lambda e: e["logical_path"]),
    }

    os.makedirs(os.path.dirname(args.out_lock), exist_ok=True)
    with open(args.out_lock, "w", encoding="utf-8") as f:
        json.dump(lock, f, indent=2, sort_keys=True)
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
