#!/usr/bin/env python3
from __future__ import annotations

import argparse, json, sys, subprocess
from pathlib import Path
from typing import Any, Dict

def jload(p: Path) -> Any:
    return json.loads(p.read_text(encoding="utf-8"))

def _norm(v: Any) -> Any:
    if isinstance(v, dict) and "type" in v:
        t = v.get("type")
        if t == "hex":
            return v.get("data")
        if t == "utf8":
            return v.get("data")
        if t == "json":
            return _norm(v.get("data"))
    if isinstance(v, dict):
        return {k: _norm(v[k]) for k in sorted(v.keys()) if k not in ("semantic", "note")}
    if isinstance(v, list):
        return [_norm(x) for x in v]
    return v

def deep_equal(a: Any, b: Any) -> bool:
    return _norm(a) == _norm(b)

def run_actor(actor_path: str, actor_name: str, req: Dict[str, Any]) -> Dict[str, Any]:
    p = subprocess.Popen([actor_path, "--name", actor_name], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    assert p.stdin and p.stdout
    p.stdin.write(json.dumps(req) + "\n")
    p.stdin.flush()
    line = p.stdout.readline()
    p.stdin.close()
    out = p.wait(timeout=20)
    if not line:
        err = p.stderr.read() if p.stderr else ""
        raise RuntimeError(f"actor returned no output (exit={out}): {err[:400]}")
    return json.loads(line)

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--actor", required=True, help="Path to refimpl_actor binary")
    ap.add_argument("--actor-name", default="suite2-mk")
    ap.add_argument("--file", default="inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/mk_hybrid_vector_report.json")
    args = ap.parse_args()

    vs = jload(Path(args.file))
    vectors = [v for v in vs.get("vectors", []) if "CAT-S2-MK-001" in (v.get("tags") or [])]
    if not vectors:
        print("No CAT-S2-MK-001 vectors found (fail-closed).", file=sys.stderr)
        return 2

    report: Dict[str, Any] = {
        "ok": True,
        "category": "CAT-S2-MK-001",
        "vector_file": args.file,
        "ran": 0,
        "passed": 0,
        "failed": 0,
        "failures": [],
    }

    ok_all = True
    for v in vectors:
        vid = v.get("id")
        op = v.get("op")
        inp = v.get("input", {})
        exp = v.get("expect", {})
        exp_ok = exp.get("ok")
        report["ran"] += 1

        req = {"id": vid, "op": op, "params": inp}
        resp = run_actor(args.actor, args.actor_name, req)

        if resp.get("id") != vid:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": f"id mismatch: got {resp.get('id')}"})
            continue

        if exp_ok is True:
            if resp.get("ok") is not True:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": resp.get("error")})
                continue
            got = resp.get("result")
            exp_out = exp.get("output")
            if exp_out is None:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "vector missing expect.output"})
                continue
            if not deep_equal(got, exp_out):
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "mismatch": {"expect": exp_out, "got": got}})
                continue
            report["passed"] += 1
        else:
            if resp.get("ok") is True:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "vector expects failure but actor returned ok"})
                continue
            reason = exp.get("reason_code")
            if reason:
                msg = ((resp.get("error") or {}).get("message") or "")
                if reason not in msg:
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": f"missing reason_code in error message (want {reason})", "got": resp.get("error")})
                    continue
            report["passed"] += 1

    report["ok"] = ok_all and report["failed"] == 0
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not report["ok"]:
        print(f"Suite-2 mk hybrid vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')} {f.get('op')}: {f.get('error','mismatch')}", file=sys.stderr)
        return 2

    print(f"Suite-2 mk hybrid vectors OK: {report['passed']} / {report['ran']}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
