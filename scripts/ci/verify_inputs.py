#!/usr/bin/env python3
from __future__ import annotations

import argparse, hashlib, io, json, os, re, zipfile
from typing import Dict, List, Tuple

HEX64 = re.compile(r"^[0-9a-f]{64}$", re.I)

def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

def read_pinned_sha256(sha_path: str) -> str:
    with open(sha_path, "r", encoding="utf-8") as f:
        line = f.readline().strip()
    tok = line.split()[0].lower()
    if not HEX64.match(tok):
        raise ValueError(f"Invalid pinned sha256 in {sha_path}: {tok}")
    return tok

def safe_open_zip(zip_path: str) -> zipfile.ZipFile:
    return zipfile.ZipFile(zip_path, "r")

def verify_phase2_manifest(z: zipfile.ZipFile) -> Dict:
    manifests = [n for n in z.namelist() if n.endswith("MANIFEST.json") and not n.endswith("/")]
    if not manifests:
        raise RuntimeError("Phase2: MANIFEST.json not found inside zip")
    manifest_path = sorted(manifests)[0]
    manifest = json.loads(z.read(manifest_path).decode("utf-8"))
    fmt = manifest.get("format")
    if fmt != "QSHIELD-BUNDLE-MANIFEST-1":
        raise RuntimeError(f"Phase2: Unexpected manifest format: {fmt}")
    files = manifest.get("files", [])
    root = os.path.dirname(manifest_path)

    results = {"manifest_path": manifest_path, "checked": [], "errors": []}
    for ent in files:
        rel = ent["path"]
        member = f"{root}/{rel}" if root else rel
        if member not in z.namelist():
            results["errors"].append({"path": rel, "error": "missing_in_zip"})
            continue
        b = z.read(member)
        got_bytes = len(b)
        got_sha = hashlib.sha256(b).hexdigest()
        exp_bytes = int(ent["bytes"])
        exp_sha = ent["sha256"].lower()
        ok = (got_bytes == exp_bytes) and (got_sha == exp_sha)
        results["checked"].append({
            "path": rel,
            "ok": ok,
            "expected_bytes": exp_bytes,
            "got_bytes": got_bytes,
            "expected_sha256": exp_sha,
            "got_sha256": got_sha
        })
        if not ok:
            results["errors"].append({"path": rel, "error": "mismatch_bytes_or_sha"})
    results["ok"] = (len(results["errors"]) == 0)
    return results

def load_phase3_lock(lock_path: str) -> Dict:
    with open(lock_path, "r", encoding="utf-8") as f:
        lock = json.load(f)
    if lock.get("format") != "QSHIELD-INPUT-LOCK-1":
        raise RuntimeError("Phase3: lock format must be QSHIELD-INPUT-LOCK-1")
    return lock

def find_members_by_basename(z: zipfile.ZipFile, basename: str) -> List[str]:
    out = []
    for n in z.namelist():
        if n.endswith("/"):
            continue
        if os.path.basename(n) == basename:
            out.append(n)
    return out

def verify_phase3_against_lock(phase3_zip_path: str, lock: Dict) -> Dict:
    z = safe_open_zip(phase3_zip_path)
    want = lock.get("entries", [])
    errors = []
    checked = []

    nested_cache: Dict[str, zipfile.ZipFile] = {}

    def open_nested_by_basename(bn: str) -> zipfile.ZipFile:
        if bn in nested_cache:
            return nested_cache[bn]
        members = find_members_by_basename(z, bn)
        if not members:
            raise RuntimeError(f"Nested zip not found in Phase3 bundle: {bn}")
        nb = z.read(members[0])
        nested_cache[bn] = zipfile.ZipFile(io.BytesIO(nb), "r")
        return nested_cache[bn]

    for ent in want:
        logical = ent["logical_path"]
        exp_bytes = int(ent["bytes"])
        exp_sha = ent["sha256"].lower()
        src = ent["source"]

        try:
            if src["type"] == "top":
                member = src["member"]
                b = z.read(member)
            elif src["type"] == "nested":
                nz = open_nested_by_basename(src["zip_basename"])
                b = nz.read(src["member"])
            else:
                raise RuntimeError(f"Unknown source type: {src['type']}")
        except Exception as e:
            errors.append({"logical_path": logical, "error": "read_failed", "detail": str(e)})
            continue

        got_bytes = len(b)
        got_sha = hashlib.sha256(b).hexdigest()
        ok = (got_bytes == exp_bytes) and (got_sha == exp_sha)
        checked.append({
            "logical_path": logical,
            "ok": ok,
            "expected_bytes": exp_bytes,
            "got_bytes": got_bytes,
            "expected_sha256": exp_sha,
            "got_sha256": got_sha
        })
        if not ok:
            errors.append({"logical_path": logical, "error": "mismatch_bytes_or_sha"})
    return {"ok": (len(errors) == 0), "checked": checked, "errors": errors}

def find_phase3_ledger_member(z: zipfile.ZipFile) -> str | None:
    candidates = [n for n in z.namelist()
                  if not n.endswith("/") and n.lower().endswith(".md")
                  and ("p3-25" in n.lower() or "p3_25" in n.lower())
                  and ("hash" in n.lower() and "ledger" in n.lower())]
    if candidates:
        return sorted(candidates)[0]
    candidates = [n for n in z.namelist()
                  if not n.endswith("/") and n.lower().endswith(".md")
                  and ("p3-25" in n.lower() or "p3_25" in n.lower())]
    return sorted(candidates)[0] if candidates else None

def crosscheck_phase3_ledger(phase3_zip_path: str) -> Dict:
    z = safe_open_zip(phase3_zip_path)
    ledger_member = find_phase3_ledger_member(z)
    if not ledger_member:
        return {"ok": False, "error": "P3-25 ledger not found in Phase3 zip"}

    ledger_txt = z.read(ledger_member).decode("utf-8", errors="replace")

    pat = re.compile(r"`([^`]+)`\s*\(bytes:\s*([0-9_]+)\s*,\s*sha256:\s*`([0-9a-f]{64})`", re.I)
    entries = [{"name": n, "bytes": int(b.replace("_","")), "sha256": s.lower()} for (n,b,s) in pat.findall(ledger_txt)]

    top_members = [n for n in z.namelist() if not n.endswith("/")]
    basemap: Dict[str, List[Tuple[str, bytes]]] = {}
    for m in top_members:
        basemap.setdefault(os.path.basename(m), []).append(("top:" + m, z.read(m)))

    nested_zip_members = [m for m in top_members if m.lower().endswith(".zip")]
    for nzm in nested_zip_members:
        bn = os.path.basename(nzm)
        try:
            nz = zipfile.ZipFile(io.BytesIO(z.read(nzm)), "r")
        except Exception:
            continue
        for nm in nz.namelist():
            if nm.endswith("/"):
                continue
            basemap.setdefault(os.path.basename(nm), []).append((f"nested:{bn}::{nm}", nz.read(nm)))

    missing = []
    present = []
    for ent in entries:
        name = ent["name"]
        if name not in basemap:
            missing.append(ent)
            continue
        found_ok = False
        for loc, data in basemap[name]:
            if len(data) == ent["bytes"] and hashlib.sha256(data).hexdigest() == ent["sha256"]:
                present.append({"name": name, "location": loc})
                found_ok = True
                break
        if not found_ok:
            missing.append({
                "name": name, "bytes": ent["bytes"], "sha256": ent["sha256"],
                "error": "present_basename_but_no_matching_digest"
            })

    return {
        "ok": True,
        "ledger_member": ledger_member,
        "ledger_entries": len(entries),
        "missing": missing,
        "present_sample": present[:25],
    }

def write_errata_md(path: str, cross: Dict) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    miss = cross.get("missing", [])
    if not miss:
        return
    with open(path, "w", encoding="utf-8") as f:
        f.write("# Phase 4 — Phase 3 Packaging Errata (Generated)\n\n")
        f.write("Phase 2 canonical specs remain frozen. No wire behavior changes are applied.\n\n")
        f.write("The following artifacts are referenced by the Phase 3 hash ledger but are missing from the Phase 3 ZIP input or do not match digests.\n\n")
        for m in miss:
            f.write(f"- `{m.get('name')}` (bytes={m.get('bytes')}, sha256={m.get('sha256')})")
            if "error" in m:
                f.write(f" — {m['error']}")
            f.write("\n")

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--phase2-zip", required=True)
    ap.add_argument("--phase2-sha", required=True)
    ap.add_argument("--phase3-zip", required=True)
    ap.add_argument("--phase3-sha", required=True)
    ap.add_argument("--phase3-lock", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--ledger-out", required=True)
    ap.add_argument("--errata-md", required=True)
    args = ap.parse_args()

    report = {"ok": False, "phase2": {}, "phase3": {}, "inputs": {}}

    p2_pin = read_pinned_sha256(args.phase2_sha)
    p3_pin = read_pinned_sha256(args.phase3_sha)
    p2_got = sha256_file(args.phase2_zip)
    p3_got = sha256_file(args.phase3_zip)
    report["inputs"] = {
        "phase2_zip_sha256": p2_got,
        "phase2_zip_pinned_sha256": p2_pin,
        "phase2_zip_sha_ok": (p2_got == p2_pin),
        "phase3_zip_sha256": p3_got,
        "phase3_zip_pinned_sha256": p3_pin,
        "phase3_zip_sha_ok": (p3_got == p3_pin),
    }

    if p2_got != p2_pin or p3_got != p3_pin:
        report["error"] = "zip_sha256_pin_mismatch"

    with safe_open_zip(args.phase2_zip) as z2:
        report["phase2"]["manifest_verification"] = verify_phase2_manifest(z2)
    if not report["phase2"]["manifest_verification"]["ok"]:
        report["error"] = report.get("error") or "phase2_manifest_verification_failed"

    lock = load_phase3_lock(args.phase3_lock)
    report["phase3"]["lock_verification"] = verify_phase3_against_lock(args.phase3_zip, lock)
    if not report["phase3"]["lock_verification"]["ok"]:
        report["error"] = report.get("error") or "phase3_lock_verification_failed"

    cross = crosscheck_phase3_ledger(args.phase3_zip)
    os.makedirs(os.path.dirname(args.ledger_out), exist_ok=True)
    with open(args.ledger_out, "w", encoding="utf-8") as f:
        json.dump(cross, f, indent=2, sort_keys=True)
    if cross.get("ok") and cross.get("missing"):
        write_errata_md(args.errata_md, cross)

    report["ok"] = (
        report["inputs"]["phase2_zip_sha_ok"]
        and report["inputs"]["phase3_zip_sha_ok"]
        and report["phase2"]["manifest_verification"]["ok"]
        and report["phase3"]["lock_verification"]["ok"]
    )

    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, sort_keys=True)

    return 0 if report["ok"] else 2

if __name__ == "__main__":
    raise SystemExit(main())
