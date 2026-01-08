#!/usr/bin/env python3
from __future__ import annotations

import argparse, base64, json, sys, subprocess
from pathlib import Path
from typing import Any, Dict, Optional

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

def b64u(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode("ascii")

def hex_to_bytes(s: str) -> bytes:
    return bytes.fromhex(s)

class ActorProc:
    def __init__(self, name: str, proc: subprocess.Popen[str]):
        self.name = name
        self.proc = proc

    @classmethod
    def start(cls, name: str, actor_path: str) -> "ActorProc":
        p = subprocess.Popen([actor_path, "--name", name], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        return cls(name, p)

    def request(self, op: str, params: Dict[str, Any], rid: str) -> Dict[str, Any]:
        req = {"id": rid, "op": op, "params": params}
        if self.proc.stdin is None or self.proc.stdout is None:
            raise RuntimeError("actor missing stdin/stdout")
        self.proc.stdin.write(json.dumps(req) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            err = self.proc.stderr.read() if self.proc.stderr else ""
            raise RuntimeError(f"actor returned no output (op={op}, id={rid}): {err[:400]}")
        return json.loads(line)

    def stop(self) -> None:
        if self.proc.stdin:
            try:
                self.proc.stdin.close()
            except Exception:
                pass
        try:
            self.proc.terminate()
            self.proc.wait(timeout=1.0)
        except Exception:
            try:
                self.proc.kill()
            except Exception:
                pass

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--actor", required=True, help="Path to refimpl_actor binary")
    ap.add_argument("--actor-name", default="suite2-interop")
    ap.add_argument("--file", default="inputs/suite2/vectors/qshield_suite2_interop_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/interop_vector_report.json")
    args = ap.parse_args()

    vs = jload(Path(args.file))
    vectors = [v for v in vs.get("vectors", []) if "CAT-S2-INTEROP-001" in (v.get("tags") or [])]
    if not vectors:
        print("No CAT-S2-INTEROP-001 vectors found (fail-closed).", file=sys.stderr)
        return 2

    report: Dict[str, Any] = {
        "ok": True,
        "category": "CAT-S2-INTEROP-001",
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
        report["ran"] += 1

        establish_a = _norm(inp.get("establish_a"))
        establish_b = _norm(inp.get("establish_b"))
        use_establish = establish_a is not None or establish_b is not None

        send_state = inp.get("send_state")
        recv_state = inp.get("recv_state")
        plaintext = inp.get("plaintext_hex")
        flags = inp.get("flags")

        if plaintext is None:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": vid, "op": op, "error": "missing plaintext_hex"})
            continue

        a = ActorProc.start(args.actor_name + "-A", args.actor)
        b = ActorProc.start(args.actor_name + "-B", args.actor)
        session_id_b64: Optional[str] = None
        try:
            if use_establish:
                if establish_a is None or establish_b is None:
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": "missing establish_a/establish_b"})
                    continue
                sess_hex = _norm(establish_a.get("session_id"))
                if not isinstance(sess_hex, str):
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": "missing session_id in establish_a"})
                    continue
                session_id_b64 = b64u(hex_to_bytes(sess_hex))
                ra = a.request("suite2.establish.run", establish_a, rid=f"{vid}:estA")
                if not ra.get("ok"):
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": f"establish A failed: {ra}"})
                    continue
                rb = b.request("suite2.establish.run", establish_b, rid=f"{vid}:estB")
                if not rb.get("ok"):
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": f"establish B failed: {rb}"})
                    continue
                sid_out_a = (ra.get("result") or {}).get("session_id")
                sid_out_b = (rb.get("result") or {}).get("session_id")
                if sid_out_a != session_id_b64 or sid_out_b != session_id_b64:
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": "session_id mismatch in establish response"})
                    continue
            else:
                if send_state is None or recv_state is None:
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": "missing send_state/recv_state"})
                    continue

            send_params: Dict[str, Any] = {
                "negotiated": vs.get("protocol", {}),
                "plaintext_hex": plaintext,
            }
            if use_establish and session_id_b64 is not None:
                send_params["session_id"] = session_id_b64
            else:
                send_params["send_state"] = send_state
            if flags is not None:
                send_params["flags"] = flags

            send_resp = a.request("suite2.e2e.send", send_params, rid=f"{vid}:send")

            if exp.get("ok") is False:
                if send_resp.get("ok") is True:
                    ok_all = False
                    report["failed"] += 1
                    report["failures"].append({"id": vid, "op": op, "error": "expected send failure but ok"})
                    continue
                reason = exp.get("reason_code")
                if reason:
                    msg = ((send_resp.get("error") or {}).get("message") or "")
                    if reason not in msg:
                        ok_all = False
                        report["failed"] += 1
                        report["failures"].append({"id": vid, "op": op, "error": f"missing reason_code {reason}", "got": send_resp.get("error")})
                        continue
                report["passed"] += 1
                continue

            if send_resp.get("ok") is not True:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": send_resp.get("error")})
                continue

            wire_hex = _norm(send_resp.get("result", {}).get("wire_hex"))
            send_state_norm = _norm(send_resp.get("result", {}).get("new_state"))
            sender_ns = send_state_norm.get("ns") if isinstance(send_state_norm, dict) else None

            recv_params: Dict[str, Any] = {
                "negotiated": vs.get("protocol", {}),
                "wire_hex": {"type": "hex", "data": wire_hex},
            }
            if use_establish and session_id_b64 is not None:
                recv_params["session_id"] = session_id_b64
            else:
                recv_params["recv_state"] = recv_state
            recv_resp = b.request("suite2.e2e.recv", recv_params, rid=f"{vid}:recv")

            if recv_resp.get("ok") is not True:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": recv_resp.get("error")})
                continue

            recv_plaintext = _norm(recv_resp.get("result", {}).get("plaintext_hex"))
            recv_state_norm = _norm(recv_resp.get("result", {}).get("new_state"))
            receiver_nr = recv_state_norm.get("nr") if isinstance(recv_state_norm, dict) else None

            exp_out = exp.get("output", {})
            if _norm(exp_out.get("plaintext_hex")) != recv_plaintext:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "plaintext mismatch"})
                continue
            if _norm(exp_out.get("sender_ns")) != sender_ns:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "sender ns mismatch"})
                continue
            if _norm(exp_out.get("receiver_nr")) != receiver_nr:
                ok_all = False
                report["failed"] += 1
                report["failures"].append({"id": vid, "op": op, "error": "receiver nr mismatch"})
                continue

            report["passed"] += 1
        finally:
            try:
                a.stop()
            except Exception:
                pass
            try:
                b.stop()
            except Exception:
                pass

    report["ok"] = ok_all and report["failed"] == 0
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not report["ok"]:
        print(f"Suite-2 interop vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')} {f.get('op')}: {f.get('error','mismatch')}", file=sys.stderr)
        return 2

    print(f"Suite-2 interop vectors OK: {report['passed']} / {report['ran']}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
