#!/usr/bin/env python3
from __future__ import annotations

import argparse, json, subprocess, sys
from pathlib import Path
from typing import Any, Dict, Tuple

def jload(p: Path) -> Any:
    return json.loads(p.read_text(encoding="utf-8"))

def _norm(v: Any) -> Any:
    # Normalize typed objects to ignore non-semantic fields (e.g., "semantic", "note").
    if isinstance(v, dict) and "type" in v:
        t = v.get("type")
        if t == "hex":
            data = v.get("data")
            if isinstance(data, str):
                return {"type": "hex", "data": data.lower()}
            return {"type": "hex", "data": data}
        if t == "json":
            return {"type": "json", "data": _norm(v.get("data"))}
        # Unknown typed wrapper: keep only type+data if present
        if "data" in v:
            return {"type": t, "data": _norm(v.get("data"))}
    if isinstance(v, dict):
        return {k: _norm(v[k]) for k in sorted(v.keys())}
    if isinstance(v, list):
        return [_norm(x) for x in v]
    return v

def deep_equal(a: Any, b: Any) -> bool:
    return _norm(a) == _norm(b)

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--actor", default="target/release/refimpl_actor")
    ap.add_argument("--actor-name", default="suite2-vectors")
    ap.add_argument("--vectors", default="inputs/suite2/vectors/qshield_suite2_kdf_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/kdf_vector_report.json")
    args = ap.parse_args()

    vec_p = Path(args.vectors)
    if not vec_p.exists():
        print(f"ERROR: vectors not found: {vec_p}", file=sys.stderr)
        return 2

    actor_p = Path(args.actor)
    if not actor_p.exists():
        print(f"ERROR: actor binary not found: {actor_p}", file=sys.stderr)
        print("Hint: build with: cargo build -p refimpl_actor --release", file=sys.stderr)
        return 2

    vs = jload(vec_p)
    vectors = vs.get("vectors", [])
    # Execute only CAT-S2-KDF-001
    cases = []
    for v in vectors:
        tags = v.get("tags") or []
        if "CAT-S2-KDF-001" in tags:
            cases.append(v)

    report: Dict[str, Any] = {
        "ok": False,
        "vectors_file": str(vec_p),
        "actor": str(actor_p),
        "ran": 0,
        "passed": 0,
        "failed": 0,
        "failures": [],
    }

    # Persistent actor process (newline-delimited JSON)
    proc = subprocess.Popen(
        [str(actor_p), "--name", str(args.actor_name)],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1,
    )

    def send(req: Dict[str, Any]) -> Dict[str, Any]:
        assert proc.stdin is not None and proc.stdout is not None
        proc.stdin.write(json.dumps(req) + "\n")
        proc.stdin.flush()
        line = proc.stdout.readline()
        if not line:
            err = (proc.stderr.read() if proc.stderr else "")
            raise RuntimeError(f"actor terminated unexpectedly; stderr={err}")
        return json.loads(line)

    ok_all = True
    for v in cases:
        vid = v.get("id", "<missing id>")
        op = v.get("op")
        inp = v.get("input") or {}
        exp = (v.get("expect") or {}).get("output")
        exp_ok = (v.get("expect") or {}).get("ok", True)

        report["ran"] += 1
        req = {"id": vid, "op": op, "params": inp}

        try:
            resp = send(req)
        except Exception as e:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": f"runner: {e}"})
            continue

        if resp.get("id") != vid:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": f"id mismatch: got {resp.get('id')}"})
            continue

        if resp.get("ok") is not True:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": resp.get("error")})
            continue

        got = resp.get("result")

        if exp_ok is not True:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": "vector expects failure but actor returned ok"})
            continue

        if exp is None:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": "vector missing expect.output"})
            continue

        if not deep_equal(got, exp):
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "mismatch": {"expect": exp, "got": got}})
            continue

        report["passed"] += 1

    # Attempt graceful shutdown
    try:
        if proc.stdin:
            proc.stdin.close()
    except Exception:
        pass
    try:
        proc.terminate()
    except Exception:
        pass

    report["ok"] = ok_all and report["failed"] == 0
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not report["ok"]:
        print(f"Suite-2 KDF vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')} {f.get('op')}: {f.get('error','mismatch')}", file=sys.stderr)
        return 2

    print(f"Suite-2 KDF vectors OK: {report['passed']} / {report['ran']}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
