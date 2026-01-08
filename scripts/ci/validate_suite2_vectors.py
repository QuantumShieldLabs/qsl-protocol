#!/usr/bin/env python3
from __future__ import annotations

import argparse, json, sys
from pathlib import Path

import jsonschema
from jsonschema import Draft202012Validator

def load_json(p: Path):
    return json.loads(p.read_text(encoding="utf-8"))

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--schema", default="schemas/qshield.phase4.vector_set.schema.v1.json")
    ap.add_argument("--glob", default="inputs/suite2/vectors/*.json")
    ap.add_argument("--out", default="artifacts/suite2/vector_schema_report.json")
    args = ap.parse_args()

    schema_p = Path(args.schema)
    if not schema_p.exists():
        print(f"ERROR: schema not found: {schema_p}", file=sys.stderr)
        return 2

    schema = load_json(schema_p)
    try:
        Draft202012Validator.check_schema(schema)
    except Exception as e:
        print(f"ERROR: schema does not compile: {e}", file=sys.stderr)
        return 2

    report = {"ok": False, "schema": str(schema_p), "files": [], "errors": []}
    ok = True
    for p in sorted(Path().glob(args.glob)):
        entry = {"file": str(p), "ok": False, "errors": []}
        try:
            data = load_json(p)
        except Exception as e:
            ok = False
            entry["errors"].append(f"json_parse: {e}")
            report["files"].append(entry)
            continue

        v = Draft202012Validator(schema)
        errs = sorted(v.iter_errors(data), key=lambda e: e.path)
        if errs:
            ok = False
            for e in errs[:50]:
                loc = "/".join([str(x) for x in e.path]) if e.path else "(root)"
                entry["errors"].append(f"{loc}: {e.message}")
        else:
            entry["ok"] = True
        report["files"].append(entry)

    report["ok"] = ok
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not ok:
        print("Suite-2 vector schema validation FAILED", file=sys.stderr)
        for f in report["files"]:
            if not f["ok"]:
                print(f"- {f['file']}", file=sys.stderr)
                for er in f["errors"][:10]:
                    print(f"    {er}", file=sys.stderr)
        return 2

    print("Suite-2 vector schema validation OK")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
