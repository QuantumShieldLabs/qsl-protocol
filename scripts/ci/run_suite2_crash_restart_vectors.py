#!/usr/bin/env python3
from __future__ import annotations

import argparse, base64, json, os, subprocess, sys, tempfile, time
from pathlib import Path
from typing import Any, Dict, Optional, Tuple


def jload(p: Path) -> Any:
    return json.loads(p.read_text(encoding="utf-8"))


def b64u(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode("ascii")


def hex_to_bytes(s: str) -> bytes:
    return bytes.fromhex(s)


def _norm(v: Any) -> Any:
    if isinstance(v, dict) and "type" in v:
        t = v.get("type")
        if t == "hex":
            return v.get("data")
        if t == "json":
            return _norm(v.get("data"))
    if isinstance(v, dict):
        return {k: _norm(v[k]) for k in sorted(v.keys()) if k not in ("semantic", "note")}
    if isinstance(v, list):
        return [_norm(x) for x in v]
    return v


class ActorProc:
    def __init__(self, name: str, proc: subprocess.Popen[str]):
        self.name = name
        self.proc = proc

    @classmethod
    def start(cls, name: str, actor_path: str, extra_env: Dict[str, str]) -> "ActorProc":
        env = os.environ.copy()
        env.update(extra_env)
        p = subprocess.Popen([actor_path, "--name", name], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)
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


def snapshot(actor: ActorProc, rid: str) -> str:
    resp = actor.request("debug_snapshot", {}, rid=rid)
    if not resp.get("ok"):
        raise RuntimeError(f"snapshot failed: {resp}")
    result = resp.get("result") or {}
    for k in ("snapshot_b64", "snapshot", "state_b64", "blob_b64"):
        v = result.get(k)
        if isinstance(v, str) and v:
            return v
    raise RuntimeError(f"snapshot missing blob: {resp}")


def restore(actor: ActorProc, snap_b64: str, rid: str) -> Dict[str, Any]:
    return actor.request("debug_restore", {"snapshot_b64": snap_b64}, rid=rid)


def run_case(actor_path: str, vec: Dict[str, Any], out: Dict[str, Any]) -> Tuple[bool, Optional[str]]:
    inp = vec.get("input", {})
    scenario = _norm(inp.get("scenario")) or {}
    case = scenario.get("case")

    establish_a = _norm(inp.get("establish_a"))
    establish_b = _norm(inp.get("establish_b"))
    use_establish = establish_a is not None or establish_b is not None

    sess_hex = _norm(inp.get("session_id"))
    if use_establish:
        if establish_a is None or establish_b is None:
            return False, "missing establish_a/establish_b"
        sess_hex = _norm(establish_a.get("session_id"))
    if not isinstance(sess_hex, str):
        return False, "missing session_id"
    session_id_b64 = b64u(hex_to_bytes(sess_hex))

    send_state = inp.get("send_state")
    recv_state = inp.get("recv_state")
    recv_state_adv = inp.get("recv_state_advanced")
    if not use_establish and (send_state is None or recv_state is None):
        return False, "missing send_state/recv_state"

    plaintexts = scenario.get("plaintexts_hex") or []
    if not isinstance(plaintexts, list) or len(plaintexts) < 2:
        return False, "plaintexts_hex must include at least 2 entries"

    dur_store = tempfile.mkdtemp(prefix=f"qsl_s2_dur_{vec.get('id','case')}_")
    extra_env = {"QSL_TEST_HOOKS": "1", "QSL_DUR_STORE_DIR": dur_store}

    a = ActorProc.start("suite2_crash_a", actor_path, extra_env)
    b = ActorProc.start("suite2_crash_b", actor_path, extra_env)

    try:
        if use_establish:
            r_est_a = a.request("suite2.establish.run", establish_a, rid=f"{vec['id']}_estA")
            if not r_est_a.get("ok"):
                return False, f"establish A failed: {r_est_a}"
            r_est_b = b.request("suite2.establish.run", establish_b, rid=f"{vec['id']}_estB")
            if not r_est_b.get("ok"):
                return False, f"establish B failed: {r_est_b}"
            sid_out_a = (r_est_a.get("result") or {}).get("session_id")
            sid_out_b = (r_est_b.get("result") or {}).get("session_id")
            if sid_out_a != session_id_b64 or sid_out_b != session_id_b64:
                return False, "session_id mismatch in establish response"

        # Message 1
        send_params1: Dict[str, Any] = {
            "negotiated": out["protocol"],
            "plaintext_hex": {"type": "hex", "data": plaintexts[0]},
        }
        if use_establish:
            send_params1["session_id"] = session_id_b64
        else:
            send_params1["send_state"] = send_state
        r1 = a.request("suite2.e2e.send", send_params1, rid=f"{vec['id']}_send1")
        if not r1.get("ok"):
            return False, f"send1 failed: {r1}"
        wire1 = _norm(r1.get("result", {}).get("wire_hex"))
        send_state = r1.get("result", {}).get("new_state")

        recv_params1: Dict[str, Any] = {
            "negotiated": out["protocol"],
            "session_id": session_id_b64,
            "wire_hex": {"type": "hex", "data": wire1},
        }
        if not use_establish:
            recv_params1["recv_state"] = recv_state
            recv_params1["send_state"] = send_state
        r1b = b.request("suite2.e2e.recv", recv_params1, rid=f"{vec['id']}_recv1")
        if not r1b.get("ok"):
            return False, f"recv1 failed: {r1b}"

        snap1 = snapshot(b, rid=f"{vec['id']}_snap1")

        # Message 2
        send_params2: Dict[str, Any] = {
            "negotiated": out["protocol"],
            "plaintext_hex": {"type": "hex", "data": plaintexts[1]},
        }
        if use_establish:
            send_params2["session_id"] = session_id_b64
        else:
            send_params2["send_state"] = send_state
        r2 = a.request("suite2.e2e.send", send_params2, rid=f"{vec['id']}_send2")
        if not r2.get("ok"):
            return False, f"send2 failed: {r2}"
        wire2 = _norm(r2.get("result", {}).get("wire_hex"))
        send_state = r2.get("result", {}).get("new_state")

        recv_state_for_step2 = recv_state_adv if recv_state_adv is not None else None
        recv_params2: Dict[str, Any] = {
            "negotiated": out["protocol"],
            "session_id": session_id_b64,
            "wire_hex": {"type": "hex", "data": wire2},
        }
        if not use_establish:
            recv_params2["recv_state"] = recv_state_for_step2 or recv_state
            recv_params2["send_state"] = send_state
        r2b = b.request("suite2.e2e.recv", recv_params2, rid=f"{vec['id']}_recv2")
        if not r2b.get("ok"):
            return False, f"recv2 failed: {r2b}"

        # Crash/restart: stop B and restore from snap1
        b.stop()
        time.sleep(0.1)
        b2 = ActorProc.start("suite2_crash_b", actor_path, extra_env)

        if case == "rollback_scka":
            rrest = restore(b2, snap1, rid=f"{vec['id']}_restore")
            if rrest.get("ok"):
                return False, "expected rollback detection on restore"
            msg = ((rrest.get("error") or {}).get("message") or "")
            if "REJECT_SCKA_ROLLBACK_DETECTED" not in msg:
                return False, f"missing rollback reason: {msg}"
            b2.stop()
            return True, None

        rrest = restore(b2, snap1, rid=f"{vec['id']}_restore")
        if not rrest.get("ok"):
            return False, f"restore failed: {rrest}"

        if case == "durable_replay":
            rrep = b2.request("suite2.e2e.recv", {
                "negotiated": out["protocol"],
                "session_id": session_id_b64,
                "wire_hex": {"type": "hex", "data": wire2},
            }, rid=f"{vec['id']}_replay")
            if rrep.get("ok"):
                return False, "expected durable replay reject"
            msg = ((rrep.get("error") or {}).get("message") or "")
            expected = scenario.get("expect_reject")
            if expected and expected not in msg:
                return False, f"missing expected reject: {expected}"
            b2.stop()
            return True, None

        # basic: send a fresh message after restore
        send_params3: Dict[str, Any] = {
            "negotiated": out["protocol"],
            "plaintext_hex": {"type": "hex", "data": plaintexts[1]},
        }
        if use_establish:
            send_params3["session_id"] = session_id_b64
        else:
            send_params3["send_state"] = send_state
        r3 = a.request("suite2.e2e.send", send_params3, rid=f"{vec['id']}_send3")
        if not r3.get("ok"):
            return False, f"send3 failed: {r3}"
        wire3 = _norm(r3.get("result", {}).get("wire_hex"))
        r3b = b2.request("suite2.e2e.recv", {
            "negotiated": out["protocol"],
            "session_id": session_id_b64,
            "wire_hex": {"type": "hex", "data": wire3},
        }, rid=f"{vec['id']}_recv3")
        if not r3b.get("ok"):
            return False, f"recv3 failed: {r3b}"
        b2.stop()
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


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--actor", required=True)
    ap.add_argument("--actor-name", default="suite2-crash")
    ap.add_argument("--file", default="inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json")
    ap.add_argument("--out", default="artifacts/suite2/crash_restart_vectors_report.json")
    args = ap.parse_args()

    vs = jload(Path(args.file))
    vectors = [v for v in vs.get("vectors", []) if "CAT-S2-CRASH-001" in (v.get("tags") or [])]
    if not vectors:
        print("No CAT-S2-CRASH-001 vectors found (fail-closed).", file=sys.stderr)
        return 2

    report: Dict[str, Any] = {
        "ok": True,
        "category": "CAT-S2-CRASH-001",
        "vector_file": args.file,
        "ran": 0,
        "passed": 0,
        "failed": 0,
        "failures": [],
    }

    ok_all = True
    for v in vectors:
        report["ran"] += 1
        ok, err = run_case(args.actor, v, vs)
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
        print(f"Suite-2 crash/restart vectors FAILED: {report['failed']} failing of {report['ran']}", file=sys.stderr)
        for f in report["failures"][:10]:
            print(f"- {f.get('id')}: {f.get('error','mismatch')}", file=sys.stderr)
        return 2

    print(f"Suite-2 crash/restart vectors OK: {report['passed']} / {report['ran']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
