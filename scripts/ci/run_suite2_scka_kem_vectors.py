#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any, Dict, Tuple

CT_LEN = 1088
SS_LEN = 32


def jload(p: Path) -> Any:
    return json.loads(p.read_text(encoding="utf-8"))


def _norm(v: Any) -> Any:
    # Normalize typed objects to ignore non-semantic fields (e.g., "semantic", "note").
    if isinstance(v, dict) and "type" in v:
        t = v.get("type")
        if t in ("hex", "utf8"):
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


def hex_decode(s: str) -> bytes:
    ss = s.strip()
    if len(ss) % 2 != 0:
        raise ValueError("hex string must have even length")
    return bytes.fromhex(ss)


def get_hex_field(obj: Any, field: str) -> bytes:
    if not isinstance(obj, dict) or field not in obj:
        raise ValueError(f"missing field: {field}")
    v = obj[field]
    if isinstance(v, str):
        return hex_decode(v)
    if isinstance(v, dict) and v.get("type") == "hex":
        data = v.get("data")
        if not isinstance(data, str):
            raise ValueError(f"{field}: hex object missing data")
        return hex_decode(data)
    raise ValueError(f"{field}: expected hex string or typed hex object")


def get_bool_field(obj: Any, field: str) -> bool:
    if not isinstance(obj, dict) or field not in obj:
        raise ValueError(f"missing field: {field}")
    v = obj[field]
    # Accept direct bool.
    if isinstance(v, bool):
        return v
    # Accept typed json object wrappers.
    if isinstance(v, dict) and v.get("type") == "json":
        data = v.get("data")
        if isinstance(data, dict) and isinstance(data.get("bool"), bool):
            return bool(data["bool"])
        if isinstance(data, bool):
            return bool(data)
    # Accept {"bool": true}.
    if isinstance(v, dict) and isinstance(v.get("bool"), bool):
        return bool(v["bool"])

    raise ValueError(f"{field}: expected bool or typed json {{bool:true}}")


def run_actor(actor_path: str, actor_name: str, req: Dict[str, Any]) -> Dict[str, Any]:
    p = subprocess.Popen(
        [actor_path, "--name", actor_name],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    assert p.stdin and p.stdout
    p.stdin.write(json.dumps(req) + "\n")
    p.stdin.flush()
    line = p.stdout.readline()
    p.stdin.close()
    out = p.wait(timeout=30)
    if not line:
        err = p.stderr.read() if p.stderr else ""
        raise RuntimeError(f"actor returned no output (exit={out}): {err[:400]}")
    return json.loads(line)


def validate_positive_result(vid: str, got: Dict[str, Any], exp_ss_match: bool) -> Tuple[bool, str]:
    result = got.get("result")
    if not isinstance(result, dict):
        return False, "actor result missing or not an object"

    try:
        ct = get_hex_field(result, "pq_ct")
        ss_out = get_hex_field(result, "pq_epoch_ss_out")
        ss_in = get_hex_field(result, "pq_epoch_ss_in")
        ss_match = get_bool_field(result, "ss_match")
    except Exception as e:
        return False, f"bad result encoding: {e}"

    if len(ct) != CT_LEN:
        return False, f"pq_ct length mismatch: got {len(ct)} bytes, expected {CT_LEN}"
    if len(ss_out) != SS_LEN:
        return False, f"pq_epoch_ss_out length mismatch: got {len(ss_out)} bytes, expected {SS_LEN}"
    if len(ss_in) != SS_LEN:
        return False, f"pq_epoch_ss_in length mismatch: got {len(ss_in)} bytes, expected {SS_LEN}"

    # ss_match must be internally consistent.
    if ss_match != (ss_out == ss_in):
        return False, "ss_match inconsistent with ss_out == ss_in"

    if ss_match != exp_ss_match:
        return False, f"ss_match mismatch: expected {exp_ss_match}, got {ss_match}"

    return True, ""


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--actor", required=True, help="Path to refimpl_actor binary")
    ap.add_argument("--actor-name", default="suite2-scka-kem")
    ap.add_argument("--file", default="inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/scka_kem_vector_report.json")
    args = ap.parse_args()

    vs = jload(Path(args.file))
    vectors = [v for v in vs.get("vectors", []) if "CAT-SCKA-KEM-001" in (v.get("tags") or [])]
    if not vectors:
        print("No CAT-SCKA-KEM-001 vectors found (fail-closed).", file=sys.stderr)
        return 2

    report: Dict[str, Any] = {
        "ok": True,
        "category": "CAT-SCKA-KEM-001",
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
        resp1 = run_actor(args.actor, args.actor_name, req)

        if resp1.get("id") != vid:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": f"id mismatch: got {resp1.get('id')}"})
            continue

        if exp_ok is True:
            if resp1.get("ok") is not True:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": resp1.get("error")})
                continue

            exp_out = exp.get("output")
            if not isinstance(exp_out, dict) or "ss_match" not in exp_out:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "vector missing expect.output.ss_match"})
                continue

            try:
                exp_ss_match = get_bool_field(exp_out, "ss_match")
            except Exception as e:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": f"bad expect.output.ss_match: {e}"})
                continue

            ok, err = validate_positive_result(vid, resp1, exp_ss_match)
            if not ok:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": err, "got": resp1.get("result")})
                continue

            # Determinism check: same input must yield identical output.
            resp2 = run_actor(args.actor, args.actor_name, req)
            if resp2.get("ok") is not True or not deep_equal(resp2.get("result"), resp1.get("result")):
                ok_all = False
                report["failed"] += 1
                report["failures"].append({
                    "id": vid,
                    "op": op,
                    "error": "nondeterministic output for identical inputs",
                    "got1": resp1.get("result"),
                    "got2": resp2.get("result"),
                })
                continue

            report["passed"] += 1

        else:
            # expects failure
            if resp1.get("ok") is True:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "vector expects failure but actor returned ok"})
                continue

            reason = exp.get("reason_code")
            if reason:
                msg = ((resp1.get("error") or {}).get("message") or "")
                if reason not in msg:
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({
                        "id": vid,
                        "op": op,
                        "error": f"missing reason_code in error message (want {reason})",
                        "got": resp1.get("error"),
                    })
                    continue

            report["passed"] += 1

    report["ok"] = ok_all and report["failed"] == 0
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not report["ok"]:
        print(f"SCKA KEM vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')} {f.get('op')}: {f.get('error')}", file=sys.stderr)
        return 2

    print(f"SCKA KEM vectors OK: {report['passed']} / {report['ran']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
