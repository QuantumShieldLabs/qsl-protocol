#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import zipfile
from typing import Any, Dict, Tuple

import jsonschema
from jsonschema import Draft202012Validator


def find_member(z: zipfile.ZipFile, needle: str, exts: tuple[str, ...] = (".json",)) -> str:
    cands = [
        n
        for n in z.namelist()
        if not n.endswith("/") and needle.lower() in n.lower() and n.lower().endswith(exts)
    ]
    if not cands:
        raise RuntimeError(f"Could not find member containing '{needle}' with extensions {exts}")
    return sorted(cands)[0]


def check_phase4_schemas() -> Tuple[bool, Dict[str, Any]]:
    """Validate Phase4 canonical schemas compile under Draft2020-12."""
    repo_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "../.."))
    schema_files = [
        os.path.join(repo_root, "schemas", "qshield.phase4.vector_set.schema.v1.json"),
        os.path.join(repo_root, "schemas", "qshield.phase4.interop_set.schema.v1.json"),
    ]

    details: Dict[str, Any] = {"schemas": []}
    ok = True

    for sp in schema_files:
        entry: Dict[str, Any] = {
            "path": os.path.relpath(sp, repo_root),
            "ok": True,
            "error": None,
        }
        try:
            with open(sp, "r", encoding="utf-8") as f:
                sch = json.load(f)
            Draft202012Validator.check_schema(sch)
        except Exception as e:
            entry["ok"] = False
            entry["error"] = str(e)
            ok = False
        details["schemas"].append(entry)

    return ok, details


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--phase3-zip", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    report: Dict[str, Any] = {"ok": False, "members": {}, "checks": [], "errors": []}

    # Phase4 schema sanity (repo-local)
    p4_ok, p4_details = check_phase4_schemas()
    report["checks"].append({"name": "phase4_schemas_compile", "ok": p4_ok, "details": p4_details})
    if not p4_ok:
        report["errors"].append("Phase4 schema compilation failed")

    # Phase3 P3-14 schema + reason codes shape (inputs)
    with zipfile.ZipFile(args.phase3_zip, "r") as z:
        shared_m = find_member(z, "Shared_Schemas_P3-14", (".json",))
        reason_m = find_member(z, "Reason_Codes_P3-14", (".json",))
        report["members"]["shared_schemas"] = shared_m
        report["members"]["reason_codes"] = reason_m
        shared = json.loads(z.read(shared_m).decode("utf-8"))
        reasons = json.loads(z.read(reason_m).decode("utf-8"))

    try:
        Draft202012Validator.check_schema(shared)
        report["checks"].append({"name": "shared_schema_compiles", "ok": True})
    except Exception as e:
        report["checks"].append({"name": "shared_schema_compiles", "ok": False})
        report["errors"].append(f"shared schema invalid: {e}")

    reason_ok = True
    if not isinstance(reasons, dict):
        reason_ok = False
        report["errors"].append("reason codes payload must be an object")
    else:
        allowed = {"format", "artifact_id", "version", "date", "reason_codes"}
        extra = [k for k in reasons.keys() if k not in allowed]
        if extra:
            reason_ok = False
            report["errors"].append(f"reason codes unexpected keys: {sorted(extra)}")
        if not isinstance(reasons.get("format"), str):
            reason_ok = False
            report["errors"].append("reason codes missing string 'format'")
        if not isinstance(reasons.get("artifact_id"), str):
            reason_ok = False
            report["errors"].append("reason codes missing string 'artifact_id'")
        if not isinstance(reasons.get("version"), str):
            reason_ok = False
            report["errors"].append("reason codes missing string 'version'")
        if not isinstance(reasons.get("date"), str):
            reason_ok = False
            report["errors"].append("reason codes missing string 'date'")
        codes = reasons.get("reason_codes")
        if not isinstance(codes, list) or not all(isinstance(c, str) for c in codes):
            reason_ok = False
            report["errors"].append("reason codes missing 'reason_codes' list of strings")

    report["checks"].append({"name": "reason_codes_shape", "ok": reason_ok})

    report["ok"] = (len(report["errors"]) == 0)
    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2, sort_keys=True)
    return 0 if report["ok"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
