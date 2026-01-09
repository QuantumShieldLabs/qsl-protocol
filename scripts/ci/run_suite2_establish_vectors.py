#!/usr/bin/env python3
from __future__ import annotations

import argparse, base64, json, sys, subprocess
from pathlib import Path
from typing import Any, Dict, Optional, Tuple


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


def run_accept_case(actor_path: str, actor_name: str, vec: Dict[str, Any], protocol: Dict[str, Any]) -> Tuple[bool, Optional[str]]:
    inp = vec.get("input", {})
    est_a = _norm(inp.get("establish_a"))
    est_b = _norm(inp.get("establish_b"))
    msg = _norm(inp.get("message"))
    if est_a is None or est_b is None or msg is None:
        return False, "missing establish_a/establish_b/message"

    sess_hex = _norm(est_a.get("session_id"))
    if not isinstance(sess_hex, str):
        return False, "missing session_id"
    session_id_b64 = b64u(hex_to_bytes(sess_hex))

    a = ActorProc.start(actor_name + "-A", actor_path)
    b = ActorProc.start(actor_name + "-B", actor_path)
    try:
        ra = a.request("suite2.establish.run", est_a, rid=f"{vec['id']}:estA")
        if not ra.get("ok"):
            return False, f"establish A failed: {ra}"
        rb = b.request("suite2.establish.run", est_b, rid=f"{vec['id']}:estB")
        if not rb.get("ok"):
            return False, f"establish B failed: {rb}"

        sid_out_a = (ra.get("result") or {}).get("session_id")
        sid_out_b = (rb.get("result") or {}).get("session_id")
        if sid_out_a != session_id_b64 or sid_out_b != session_id_b64:
            return False, "session_id mismatch in establish response"

        plaintext_hex = _norm(msg.get("plaintext_hex"))
        flags = msg.get("flags")
        if not isinstance(plaintext_hex, str):
            return False, "missing plaintext_hex"

        send_params: Dict[str, Any] = {
            "negotiated": protocol,
            "session_id": session_id_b64,
            "plaintext_hex": {"type": "hex", "data": plaintext_hex},
        }
        if flags is not None:
            send_params["flags"] = flags

        send_resp = a.request("suite2.e2e.send", send_params, rid=f"{vec['id']}:send")
        if not send_resp.get("ok"):
            return False, f"send failed: {send_resp}"

        wire_hex = _norm(send_resp.get("result", {}).get("wire_hex"))
        send_state_norm = _norm(send_resp.get("result", {}).get("new_state"))
        sender_ns = send_state_norm.get("ns") if isinstance(send_state_norm, dict) else None

        recv_resp = b.request("suite2.e2e.recv", {
            "negotiated": protocol,
            "session_id": session_id_b64,
            "wire_hex": {"type": "hex", "data": wire_hex},
        }, rid=f"{vec['id']}:recv")
        if not recv_resp.get("ok"):
            return False, f"recv failed: {recv_resp}"

        recv_plaintext = _norm(recv_resp.get("result", {}).get("plaintext_hex"))
        recv_state_norm = _norm(recv_resp.get("result", {}).get("new_state"))
        receiver_nr = recv_state_norm.get("nr") if isinstance(recv_state_norm, dict) else None

        exp = vec.get("expect", {})
        exp_out = exp.get("output", {})
        if _norm(exp_out.get("plaintext_hex")) != recv_plaintext:
            return False, "plaintext mismatch"
        if _norm(exp_out.get("sender_ns")) != sender_ns:
            return False, "sender ns mismatch"
        if _norm(exp_out.get("receiver_nr")) != receiver_nr:
            return False, "receiver nr mismatch"

        return True, None
    finally:
        try:
            a.stop()
        except Exception:
            pass
        try:
            b.stop()
        except Exception:
            pass


def run_reject_case(actor_path: str, actor_name: str, vec: Dict[str, Any]) -> Tuple[bool, Optional[str]]:
    inp = vec.get("input", {})
    est = _norm(inp.get("establish"))
    if est is None:
        return False, "missing establish"

    a = ActorProc.start(actor_name + "-A", actor_path)
    try:
        resp = a.request("suite2.establish.run", est, rid=f"{vec['id']}:est")
        exp = vec.get("expect", {})
        if exp.get("ok") is True:
            if not resp.get("ok"):
                return False, f"expected ok but got error: {resp}"
            return True, None
        if resp.get("ok") is True:
            return False, "expected failure but actor returned ok"
        reason = exp.get("reason_code")
        if reason:
            msg = ((resp.get("error") or {}).get("message") or "")
            if reason not in msg:
                return False, f"missing reason_code {reason}"
        return True, None
    finally:
        try:
            a.stop()
        except Exception:
            pass


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--actor", required=True)
    ap.add_argument("--actor-name", default="suite2-establish")
    ap.add_argument("--file", default="inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/establish_vector_report.json")
    args = ap.parse_args()

    vs = jload(Path(args.file))
    vectors = [v for v in vs.get("vectors", []) if "CAT-S2-ESTABLISH-001" in (v.get("tags") or [])]
    if not vectors:
        print("No CAT-S2-ESTABLISH-001 vectors found (fail-closed).", file=sys.stderr)
        sys.exit(2)
        return 2

    report: Dict[str, Any] = {
        "ok": True,
        "category": "CAT-S2-ESTABLISH-001",
        "vector_file": args.file,
        "ran": 0,
        "passed": 0,
        "failed": 0,
        "failures": [],
    }

    ok_all = True
    for v in vectors:
        report["ran"] += 1
        kind = v.get("kind")
        if kind == "positive":
            ok, err = run_accept_case(args.actor, args.actor_name, v, vs.get("protocol", {}))
        else:
            ok, err = run_reject_case(args.actor, args.actor_name, v)
        if ok:
            report["passed"] += 1
        else:
            ok_all = False
            report["failed"] += 1
            report["failures"].append({"id": v.get("id"), "error": err})

    report["ok"] = ok_all and report["failed"] == 0
    outp = Path(args.out)
    outp.parent.mkdir(parents=True, exist_ok=True)
    outp.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if not report["ok"]:
        print(f"Suite-2 establish vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')}: {f.get('error')}", file=sys.stderr)
        return 2

    print(f"Suite-2 establish vectors OK: {report['passed']} / {report['ran']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
