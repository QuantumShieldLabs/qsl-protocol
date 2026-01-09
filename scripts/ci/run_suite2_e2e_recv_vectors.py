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
    ap.add_argument("--actor-name", default="suite2-e2e")
    ap.add_argument("--file", default="inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/e2e_recv_vector_report.json")
    args = ap.parse_args()

    vs = jload(Path(args.file))
    vectors = [v for v in vs.get("vectors", []) if "CAT-S2-E2E-RECV-001" in (v.get("tags") or [])]
    if not vectors:
        print("No CAT-S2-E2E-RECV-001 vectors found (fail-closed).", file=sys.stderr)
        return 2

    report: Dict[str, Any] = {
        "ok": True,
        "category": "CAT-S2-E2E-RECV-001",
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
        steps = _norm(inp.get("steps"))
        if not isinstance(steps, list):
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": "steps missing or not a list"})
            continue

        negotiated = inp.get("negotiated", {})
        state = _norm(inp.get("recv_state"))
        if not isinstance(state, dict):
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": "recv_state missing or not object"})
            continue

        report["ran"] += 1
        step_ok = True
        for idx, step in enumerate(steps):
            if not isinstance(step, dict):
                step_ok = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": f"step {idx} not object"})
                break

            wire_hex = step.get("wire_hex")
            if not isinstance(wire_hex, str):
                step_ok = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": f"step {idx} missing wire_hex"})
                break

            params: Dict[str, Any] = {
                "negotiated": negotiated,
                "recv_state": state,
                "wire_hex": {"type": "hex", "data": wire_hex},
            }
            if "pq_epoch_ss" in step:
                params["pq_epoch_ss"] = {"type": "hex", "data": step["pq_epoch_ss"]}
            if "peer_adv_id" in step:
                params["peer_adv_id"] = {"type": "json", "data": step["peer_adv_id"]}

            req = {"id": f"{vid}:{idx}", "op": op, "params": params}
            resp = run_actor(args.actor, args.actor_name, req)
            expect_ok = step.get("expect_ok")
            if expect_ok is True:
                if resp.get("ok") is not True:
                    step_ok = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": resp.get("error")})
                    break
                got = resp.get("result")
                pt_hex = step.get("expect_plaintext_hex")
                if pt_hex is not None:
                    got_pt = _norm(got.get("plaintext_hex"))
                    if got_pt != pt_hex:
                        step_ok = False
                        report["failed"] += 1
                        report["failures"].append({"id": vid, "op": op, "error": f"step {idx} plaintext mismatch"})
                        break
                expect_nr = step.get("expect_nr")
                if expect_nr is not None:
                    got_state = _norm(got.get("new_state"))
                    got_nr = _norm(got_state.get("nr")) if isinstance(got_state, dict) else None
                    if not deep_equal(got_nr, expect_nr):
                        step_ok = False
                        report["failed"] += 1
                        report["failures"].append({"id": vid, "op": op, "error": f"step {idx} nr mismatch"})
                        break
                state = _norm(got.get("new_state"))
                if not isinstance(state, dict):
                    step_ok = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": f"step {idx} new_state missing"})
                    break
            else:
                if resp.get("ok") is True:
                    step_ok = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": f"step {idx} expected failure but ok"})
                    break
                reason = step.get("reason_code")
                if reason:
                    msg = ((resp.get("error") or {}).get("message") or "")
                    if reason not in msg:
                        step_ok = False
                        report["failed"] += 1
                        report["failures"].append({"id": vid, "op": op, "error": f"step {idx} missing reason_code {reason}", "got": resp.get("error")})
                        break
                break

        if step_ok:
            report["passed"] += 1
        else:
            ok_all = False

    report["ok"] = ok_all and report["failed"] == 0
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not report["ok"]:
        print(f"Suite-2 e2e recv vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')} {f.get('op')}: {f.get('error','mismatch')}", file=sys.stderr)
        return 2

    print(f"Suite-2 e2e recv vectors OK: {report['passed']} / {report['ran']}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
