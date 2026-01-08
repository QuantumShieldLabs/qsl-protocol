#!/usr/bin/env python3
import argparse
import base64
import json
import os
import subprocess
import sys
import time
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Optional, Tuple


def b64u_no_pad(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).decode("ascii").rstrip("=")


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


@dataclass
class ActorProc:
    name: str
    cmd: list
    env: Dict[str, str]
    proc: subprocess.Popen

    @staticmethod
    def start(name: str, actor_path: str, extra_env: Dict[str, str]) -> "ActorProc":
        env = os.environ.copy()
        env.update(extra_env)
        # Actor is a JSONL protocol on stdin/stdout.
        proc = subprocess.Popen(
            [actor_path, "--name", name, "--ci"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
            env=env,
        )
        return ActorProc(name=name, cmd=[actor_path, "--name", name, "--ci"], env=env, proc=proc)

    def stop(self) -> None:
        try:
            if self.proc.stdin:
                self.proc.stdin.close()
        except Exception:
            pass
        try:
            self.proc.terminate()
        except Exception:
            pass
        try:
            self.proc.wait(timeout=2.0)
        except Exception:
            try:
                self.proc.kill()
            except Exception:
                pass

    def request(self, op: str, params: Dict[str, Any], rid: str) -> Dict[str, Any]:
        if not self.proc.stdin or not self.proc.stdout:
            raise RuntimeError("actor process pipes not available")
        req = {"id": rid, "op": op, "params": params}
        self.proc.stdin.write(json.dumps(req) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            # Surface stderr for debugging.
            err = ""
            try:
                if self.proc.stderr:
                    err = self.proc.stderr.read()
            except Exception:
                pass
            raise RuntimeError(f"actor exited/no response (op={op}, id={rid}). stderr={err!r}")
        try:
            return json.loads(line)
        except json.JSONDecodeError as e:
            raise RuntimeError(f"invalid JSON from actor: {line!r}") from e


def _extract_snapshot(result_obj: Any) -> Tuple[str, str]:
    """Return (key, b64val) for snapshot output."""
    if not isinstance(result_obj, dict):
        raise RuntimeError("snapshot result is not an object")
    # Prefer explicit keys if present.
    for k in ("snapshot_b64", "snapshot", "state_b64", "blob_b64"):
        v = result_obj.get(k)
        if isinstance(v, str) and v:
            return k, v
    # Otherwise take the first string field.
    for k, v in result_obj.items():
        if isinstance(v, str) and v:
            return k, v
    raise RuntimeError("snapshot result did not contain a usable b64 string field")


def durability_it_dur_001(actor_path: str) -> Tuple[bool, Dict[str, Any]]:
    """IT-DUR-001: snapshot/restore across restart preserves session state."""
    suite = "Suite-1"
    case_id = "IT-DUR-001.Suite-1.AtoB"
    details: Dict[str, Any] = {"case_id": case_id, "suite": suite}

    details["stages"] = []

    def mark(stage: str, **kw: Any) -> None:
        rec: Dict[str, Any] = {"stage": stage}
        rec.update(kw)
        details["stages"].append(rec)
        details["stage"] = stage


    dur_store_dir = tempfile.mkdtemp(prefix=f"qsl_dur_{case_id.replace('.', '_')}_")
    extra_env = {"QSL_TEST_HOOKS": "1", "QSL_DUR_STORE_DIR": dur_store_dir}

    a = ActorProc.start("impl_a", actor_path, extra_env)
    b = ActorProc.start("impl_b", actor_path, extra_env)

    def fail(stage: str, **kw: Any) -> Tuple[bool, Dict[str, Any]]:
        details.update({"stage": stage, **kw})
        a.stop()
        b.stop()
        return False, details

    try:
        # Reset both.
        ra = a.request("reset", {}, rid=f"{case_id}_reset_a")
        rb = b.request("reset", {}, rid=f"{case_id}_reset_b")
        if not ra.get("ok") or not rb.get("ok"):
            return fail("reset", resp_a=ra, resp_b=rb)
        mark("reset_ok")

        # Handshake A->B.
        r1 = a.request("handshake_init", {"suite": suite, "options": {}}, rid=f"{case_id}_h1")
        if not r1.get("ok"):
            return fail("handshake_init", resp=r1)
        msg1 = (r1.get("result") or {}).get("msg1_b64")
        if not isinstance(msg1, str):
            return fail("handshake_init", error="missing_msg1_b64", resp=r1)

        r2 = b.request("handshake_respond", {"suite": suite, "msg1_b64": msg1, "options": {}}, rid=f"{case_id}_h2")
        if not r2.get("ok"):
            return fail("handshake_respond", resp=r2)
        msg2 = (r2.get("result") or {}).get("msg2_b64")
        if not isinstance(msg2, str):
            return fail("handshake_respond", error="missing_msg2_b64", resp=r2)

        r3 = a.request("handshake_finish", {"suite": suite, "msg2_b64": msg2, "options": {}}, rid=f"{case_id}_h3")
        if not r3.get("ok"):
            return fail("handshake_finish", resp=r3)
        session_id = (r3.get("result") or {}).get("session_id")
        if not isinstance(session_id, str):
            return fail("handshake_finish", error="missing_session_id", resp=r3)
        details["session_id"] = session_id
        mark("handshake_ok", session_id=session_id)

        # Send one message A->B and confirm decrypt.
        pt1_bytes = f"{case_id}:pt1".encode("utf-8")
        pt1_b64 = b64u_no_pad(pt1_bytes)
        enc1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt1_b64}, rid=f"{case_id}_enc1")
        if not enc1.get("ok"):
            return fail("encrypt1", resp=enc1)
        ct1 = (enc1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct1, str):
            return fail("encrypt1", error="missing_ciphertext_b64", resp=enc1)

        dec1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct1}, rid=f"{case_id}_dec1")
        if not dec1.get("ok"):
            return fail("decrypt1", resp=dec1)
        pt1_out = (dec1.get("result") or {}).get("plaintext_b64")
        if pt1_out != pt1_b64:
            return fail("decrypt1", error="plaintext_mismatch", expected=pt1_b64, got=pt1_out, resp=dec1)

        # Snapshot both actors.
        sa = a.request("debug_snapshot", {}, rid=f"{case_id}_snap_a")
        sb = b.request("debug_snapshot", {}, rid=f"{case_id}_snap_b")
        if not sa.get("ok") or not sb.get("ok"):
            return fail("snapshot", resp_a=sa, resp_b=sb)

        ka, sva = _extract_snapshot(sa.get("result") or {})
        kb, svb = _extract_snapshot(sb.get("result") or {})
        details["snapshot_key_a"] = ka
        details["snapshot_key_b"] = kb

        # Restart processes (simulate crash/restart).
        a.stop()
        b.stop()
        time.sleep(0.1)
        a2 = ActorProc.start("impl_a", actor_path, extra_env)
        b2 = ActorProc.start("impl_b", actor_path, extra_env)

        # Restore both (use the same key name returned by snapshot).
        ra2 = a2.request("debug_restore", {ka: sva}, rid=f"{case_id}_rest_a")
        rb2 = b2.request("debug_restore", {kb: svb}, rid=f"{case_id}_rest_b")
        if not ra2.get("ok") or not rb2.get("ok"):
            a2.stop(); b2.stop()
            details.update({"stage": "restore", "resp_a": ra2, "resp_b": rb2})
            return False, details

        # Send another message A->B after restore and confirm decrypt.
        pt2_bytes = f"{case_id}:pt2".encode("utf-8")
        pt2_b64 = b64u_no_pad(pt2_bytes)
        enc2 = a2.request("encrypt", {"session_id": session_id, "plaintext_b64": pt2_b64}, rid=f"{case_id}_enc2")
        if not enc2.get("ok"):
            a2.stop(); b2.stop()
            details.update({"stage": "encrypt2", "resp": enc2})
            return False, details
        ct2 = (enc2.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct2, str):
            a2.stop(); b2.stop()
            details.update({"stage": "encrypt2", "error": "missing_ciphertext_b64", "resp": enc2})
            return False, details

        dec2 = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_dec2")
        if not dec2.get("ok"):
            a2.stop(); b2.stop()
            details.update({"stage": "decrypt2", "resp": dec2})
            return False, details
        pt2_out = (dec2.get("result") or {}).get("plaintext_b64")
        if pt2_out != pt2_b64:
            a2.stop(); b2.stop()
            details.update({"stage": "decrypt2", "error": "plaintext_mismatch", "expected": pt2_b64, "got": pt2_out, "resp": dec2})
            return False, details

        # Replay protection: re-submit ct1; must reject.
        rep = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct1}, rid=f"{case_id}_replay")
        if rep.get("ok") is True:
            a2.stop(); b2.stop()
            details.update({"stage": "replay", "error": "expected_reject", "resp": rep})
            return False, details

        a2.stop()
        b2.stop()
        mark("ok")
        return True, details

    except Exception as e:
        return fail("exception", error=str(e))


def durability_it_dur_002(actor_path: str) -> Tuple[bool, Dict[str, Any]]:
    """IT-DUR-002: replay-after-accept survives snapshot/restore (second snapshot).

    This extends IT-DUR-001 by taking a second snapshot after accepting a second ciphertext.
    After restoring from snapshot2, replaying that second ciphertext must be rejected.
    """
    suite = "Suite-1"
    case_id = "IT-DUR-002.Suite-1.AtoB"
    details: Dict[str, Any] = {"case_id": case_id, "suite": suite}
    details["stages"] = []

    def mark(stage: str, **kw: Any) -> None:
        rec: Dict[str, Any] = {"stage": stage}
        rec.update(kw)
        details["stages"].append(rec)
        details["stage"] = stage


    dur_store_dir = tempfile.mkdtemp(prefix=f"qsl_dur_{case_id.replace('.', '_')}_")
    extra_env = {"QSL_TEST_HOOKS": "1", "QSL_DUR_STORE_DIR": dur_store_dir}

    a = ActorProc.start("impl_a", actor_path, extra_env)
    b = ActorProc.start("impl_b", actor_path, extra_env)

    def stop_all(*procs: ActorProc) -> None:
        for p in procs:
            try:
                p.stop()
            except Exception:
                pass

    def fail(stage: str, **kw: Any) -> Tuple[bool, Dict[str, Any]]:
        details.update({"stage": stage, **kw})
        stop_all(a, b)
        return False, details

    try:
        # Reset both.
        ra = a.request("reset", {}, rid=f"{case_id}_reset_a")
        rb = b.request("reset", {}, rid=f"{case_id}_reset_b")
        if not ra.get("ok") or not rb.get("ok"):
            return fail("reset", resp_a=ra, resp_b=rb)
        mark("reset_ok")

        # Handshake A->B.
        r1 = a.request("handshake_init", {"suite": suite, "options": {}}, rid=f"{case_id}_h1")
        if not r1.get("ok"):
            return fail("handshake_init", resp=r1)
        msg1 = (r1.get("result") or {}).get("msg1_b64")
        if not isinstance(msg1, str):
            return fail("handshake_init", error="missing_msg1_b64", resp=r1)

        r2 = b.request("handshake_respond", {"suite": suite, "msg1_b64": msg1, "options": {}}, rid=f"{case_id}_h2")
        if not r2.get("ok"):
            return fail("handshake_respond", resp=r2)
        msg2 = (r2.get("result") or {}).get("msg2_b64")
        if not isinstance(msg2, str):
            return fail("handshake_respond", error="missing_msg2_b64", resp=r2)

        r3 = a.request("handshake_finish", {"suite": suite, "msg2_b64": msg2, "options": {}}, rid=f"{case_id}_h3")
        if not r3.get("ok"):
            return fail("handshake_finish", resp=r3)
        session_id = (r3.get("result") or {}).get("session_id")
        if not isinstance(session_id, str):
            return fail("handshake_finish", error="missing_session_id", resp=r3)
        details["session_id"] = session_id
        mark("handshake_ok", session_id=session_id)

        # Message 1 A->B (accept).
        pt1_b64 = b64u_no_pad(f"{case_id}:pt1".encode("utf-8"))
        enc1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt1_b64}, rid=f"{case_id}_enc1")
        if not enc1.get("ok"):
            return fail("encrypt1", resp=enc1)
        ct1 = (enc1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct1, str):
            return fail("encrypt1", error="missing_ciphertext_b64", resp=enc1)

        dec1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct1}, rid=f"{case_id}_dec1")
        if not dec1.get("ok"):
            return fail("decrypt1", resp=dec1)
        if (dec1.get("result") or {}).get("plaintext_b64") != pt1_b64:
            return fail("decrypt1", error="plaintext_mismatch", expected=pt1_b64, got=(dec1.get("result") or {}).get("plaintext_b64"), resp=dec1)
        mark("m1_ok")

        # Snapshot1.
        sa1 = a.request("debug_snapshot", {}, rid=f"{case_id}_snap1_a")
        sb1 = b.request("debug_snapshot", {}, rid=f"{case_id}_snap1_b")
        if not sa1.get("ok") or not sb1.get("ok"):
            return fail("snapshot1", resp_a=sa1, resp_b=sb1)
        ka1, sva1 = _extract_snapshot(sa1.get("result") or {})
        kb1, svb1 = _extract_snapshot(sb1.get("result") or {})
        details["snapshot1_key_a"] = ka1
        details["snapshot1_key_b"] = kb1
        mark("snapshot1_ok")

        # Restart + restore snapshot1.
        a.stop(); b.stop()
        time.sleep(0.1)
        a2 = ActorProc.start("impl_a", actor_path, extra_env)
        b2 = ActorProc.start("impl_b", actor_path, extra_env)

        ra2 = a2.request("debug_restore", {ka1: sva1}, rid=f"{case_id}_rest1_a")
        rb2 = b2.request("debug_restore", {kb1: svb1}, rid=f"{case_id}_rest1_b")
        if not ra2.get("ok") or not rb2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "restore1", "resp_a": ra2, "resp_b": rb2})
            return False, details
        mark("restore1_ok")

        # Message 2 A->B (accept after restore1).
        pt2_b64 = b64u_no_pad(f"{case_id}:pt2".encode("utf-8"))
        enc2 = a2.request("encrypt", {"session_id": session_id, "plaintext_b64": pt2_b64}, rid=f"{case_id}_enc2")
        if not enc2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "encrypt2", "resp": enc2})
            return False, details
        ct2 = (enc2.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct2, str):
            stop_all(a2, b2)
            details.update({"stage": "encrypt2", "error": "missing_ciphertext_b64", "resp": enc2})
            return False, details

        dec2 = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_dec2")
        if not dec2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "decrypt2", "resp": dec2})
            return False, details
        if (dec2.get("result") or {}).get("plaintext_b64") != pt2_b64:
            stop_all(a2, b2)
            details.update({"stage": "decrypt2", "error": "plaintext_mismatch", "expected": pt2_b64, "got": (dec2.get("result") or {}).get("plaintext_b64"), "resp": dec2})
            return False, details

        # Snapshot2 (after accepting ct2).
        sa2 = a2.request("debug_snapshot", {}, rid=f"{case_id}_snap2_a")
        sb2 = b2.request("debug_snapshot", {}, rid=f"{case_id}_snap2_b")
        if not sa2.get("ok") or not sb2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "snapshot2", "resp_a": sa2, "resp_b": sb2})
            return False, details
        ka2, sva2 = _extract_snapshot(sa2.get("result") or {})
        kb2, svb2 = _extract_snapshot(sb2.get("result") or {})
        details["snapshot2_key_a"] = ka2
        details["snapshot2_key_b"] = kb2

        # Restart + restore snapshot2.
        a2.stop(); b2.stop()
        time.sleep(0.1)
        a3 = ActorProc.start("impl_a", actor_path, extra_env)
        b3 = ActorProc.start("impl_b", actor_path, extra_env)

        ra3 = a3.request("debug_restore", {ka2: sva2}, rid=f"{case_id}_rest2_a")
        rb3 = b3.request("debug_restore", {kb2: svb2}, rid=f"{case_id}_rest2_b")
        if not ra3.get("ok") or not rb3.get("ok"):
            stop_all(a3, b3)
            details.update({"stage": "restore2", "resp_a": ra3, "resp_b": rb3})
            return False, details

        # Replay ct2 after restore2: must reject.
        rep = b3.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_replay2")
        if rep.get("ok") is True:
            stop_all(a3, b3)
            details.update({"stage": "replay2", "error": "expected_reject", "resp": rep})
            return False, details

        # Sanity: fresh message still works after restore2.
        pt3_b64 = b64u_no_pad(f"{case_id}:pt3".encode("utf-8"))
        enc3 = a3.request("encrypt", {"session_id": session_id, "plaintext_b64": pt3_b64}, rid=f"{case_id}_enc3")
        if not enc3.get("ok"):
            stop_all(a3, b3)
            details.update({"stage": "encrypt3", "resp": enc3})
            return False, details
        ct3 = (enc3.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct3, str):
            stop_all(a3, b3)
            details.update({"stage": "encrypt3", "error": "missing_ciphertext_b64", "resp": enc3})
            return False, details

        dec3 = b3.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct3}, rid=f"{case_id}_dec3")
        if not dec3.get("ok"):
            stop_all(a3, b3)
            details.update({"stage": "decrypt3", "resp": dec3})
            return False, details
        if (dec3.get("result") or {}).get("plaintext_b64") != pt3_b64:
            stop_all(a3, b3)
            details.update({"stage": "decrypt3", "error": "plaintext_mismatch", "expected": pt3_b64, "got": (dec3.get("result") or {}).get("plaintext_b64"), "resp": dec3})
            return False, details

        stop_all(a3, b3)
        mark("ok")
        return True, details

    except Exception as e:
        return fail("exception", error=str(e))


def durability_it_dur_003(actor_path: str) -> Tuple[bool, Dict[str, Any]]:
    """IT-DUR-003: rollback restore must not decrypt post-snapshot ciphertext; session continues.

    Flow (A->B):
      1) Handshake A<->B
      2) A encrypt m1; B decrypt OK
      3) Snapshot S1 (both)
      4) A encrypt m2; B decrypt OK (state advances beyond S1)
      5) Restart + restore S1
      6) Attempt decrypt ct2 on restored B -> must reject
      7) Fresh message m3 after restore decrypts OK
    """
    suite = "Suite-1"
    case_id = "IT-DUR-003.Suite-1.AtoB"
    details: Dict[str, Any] = {"case_id": case_id, "suite": suite}
    details["stages"] = []

    def mark(stage: str, **kw: Any) -> None:
        rec: Dict[str, Any] = {"stage": stage}
        rec.update(kw)
        details["stages"].append(rec)
        details["stage"] = stage


    dur_store_dir = tempfile.mkdtemp(prefix=f"qsl_dur_{case_id.replace('.', '_')}_")
    extra_env = {"QSL_TEST_HOOKS": "1", "QSL_DUR_STORE_DIR": dur_store_dir}

    a = ActorProc.start("impl_a", actor_path, extra_env)
    b = ActorProc.start("impl_b", actor_path, extra_env)

    def stop_all(*procs: ActorProc) -> None:
        for p in procs:
            try:
                p.stop()
            except Exception:
                pass

    def fail(stage: str, **kw: Any) -> Tuple[bool, Dict[str, Any]]:
        details.update({"stage": stage, **kw})
        stop_all(a, b)
        return False, details

    try:
        # Reset both.
        ra = a.request("reset", {}, rid=f"{case_id}_reset_a")
        rb = b.request("reset", {}, rid=f"{case_id}_reset_b")
        if not ra.get("ok") or not rb.get("ok"):
            return fail("reset", resp_a=ra, resp_b=rb)
        mark("reset_ok")

        # Handshake A->B.
        r1 = a.request("handshake_init", {"suite": suite, "options": {}}, rid=f"{case_id}_h1")
        if not r1.get("ok"):
            return fail("handshake_init", resp=r1)
        msg1 = (r1.get("result") or {}).get("msg1_b64")
        if not isinstance(msg1, str):
            return fail("handshake_init", error="missing_msg1_b64", resp=r1)

        r2 = b.request("handshake_respond", {"suite": suite, "msg1_b64": msg1, "options": {}}, rid=f"{case_id}_h2")
        if not r2.get("ok"):
            return fail("handshake_respond", resp=r2)
        msg2 = (r2.get("result") or {}).get("msg2_b64")
        if not isinstance(msg2, str):
            return fail("handshake_respond", error="missing_msg2_b64", resp=r2)

        r3 = a.request("handshake_finish", {"suite": suite, "msg2_b64": msg2, "options": {}}, rid=f"{case_id}_h3")
        if not r3.get("ok"):
            return fail("handshake_finish", resp=r3)
        session_id = (r3.get("result") or {}).get("session_id")
        if not isinstance(session_id, str):
            return fail("handshake_finish", error="missing_session_id", resp=r3)
        details["session_id"] = session_id
        mark("handshake_ok", session_id=session_id)

        # Message 1 A->B.
        pt1_b64 = b64u_no_pad(f"{case_id}:pt1".encode("utf-8"))
        enc1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt1_b64}, rid=f"{case_id}_enc1")
        if not enc1.get("ok"):
            return fail("encrypt1", resp=enc1)
        ct1 = (enc1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct1, str):
            return fail("encrypt1", error="missing_ciphertext_b64", resp=enc1)

        dec1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct1}, rid=f"{case_id}_dec1")
        if not dec1.get("ok"):
            return fail("decrypt1", resp=dec1)
        if (dec1.get("result") or {}).get("plaintext_b64") != pt1_b64:
            return fail("decrypt1", error="plaintext_mismatch", expected=pt1_b64, got=(dec1.get("result") or {}).get("plaintext_b64"), resp=dec1)
        mark("m1_ok")

        # Snapshot S1.
        sa1 = a.request("debug_snapshot", {}, rid=f"{case_id}_snap1_a")
        sb1 = b.request("debug_snapshot", {}, rid=f"{case_id}_snap1_b")
        if not sa1.get("ok") or not sb1.get("ok"):
            return fail("snapshot1", resp_a=sa1, resp_b=sb1)
        ka1, sva1 = _extract_snapshot(sa1.get("result") or {})
        kb1, svb1 = _extract_snapshot(sb1.get("result") or {})
        details["snapshot1_key_a"] = ka1
        details["snapshot1_key_b"] = kb1
        mark("snapshot1_ok")

        # Message 2 A->B (advance beyond S1).
        pt2_b64 = b64u_no_pad(f"{case_id}:pt2".encode("utf-8"))
        enc2 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt2_b64}, rid=f"{case_id}_enc2")
        if not enc2.get("ok"):
            return fail("encrypt2", resp=enc2)
        ct2 = (enc2.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct2, str):
            return fail("encrypt2", error="missing_ciphertext_b64", resp=enc2)

        dec2 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_dec2")
        if not dec2.get("ok"):
            return fail("decrypt2", resp=dec2)
        if (dec2.get("result") or {}).get("plaintext_b64") != pt2_b64:
            return fail("decrypt2", error="plaintext_mismatch", expected=pt2_b64, got=(dec2.get("result") or {}).get("plaintext_b64"), resp=dec2)
        mark("m2_ok")

        # Restart + restore S1.
        a.stop(); b.stop()
        time.sleep(0.1)
        a2 = ActorProc.start("impl_a", actor_path, extra_env)
        b2 = ActorProc.start("impl_b", actor_path, extra_env)

        ra2 = a2.request("debug_restore", {ka1: sva1}, rid=f"{case_id}_rest1_a")
        rb2 = b2.request("debug_restore", {kb1: svb1}, rid=f"{case_id}_rest1_b")
        if not ra2.get("ok") or not rb2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "restore1", "resp_a": ra2, "resp_b": rb2})
            return False, details
        mark("restore1_ok")

        # Attempt to decrypt ct2 (post-snapshot ciphertext) on restored B: must reject.
        rej = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_dec2_after_restore")
        if rej.get("ok") is True:
            stop_all(a2, b2)
            details.update({"stage": "decrypt2_after_restore", "error": "expected_reject", "resp": rej})
            return False, details
        mark("ct2_rejected", resp=rej)

        # Fresh message after restore: must succeed.
        pt3_b64 = b64u_no_pad(f"{case_id}:pt3".encode("utf-8"))
        enc3 = a2.request("encrypt", {"session_id": session_id, "plaintext_b64": pt3_b64}, rid=f"{case_id}_enc3")
        if not enc3.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "encrypt3", "resp": enc3})
            return False, details
        ct3 = (enc3.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct3, str):
            stop_all(a2, b2)
            details.update({"stage": "encrypt3", "error": "missing_ciphertext_b64", "resp": enc3})
            return False, details

        dec3 = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct3}, rid=f"{case_id}_dec3")
        if not dec3.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "decrypt3", "resp": dec3})
            return False, details
        if (dec3.get("result") or {}).get("plaintext_b64") != pt3_b64:
            stop_all(a2, b2)
            details.update({"stage": "decrypt3", "error": "plaintext_mismatch", "expected": pt3_b64, "got": (dec3.get("result") or {}).get("plaintext_b64"), "resp": dec3})
            return False, details
        mark("m3_ok")
        mark("ok")

        stop_all(a2, b2)
        return True, details

    except Exception as e:
        return fail("exception", error=str(e))

def durability_it_dur_004(actor_path: str) -> Tuple[bool, Dict[str, Any]]:
    """IT-DUR-004: rollback at a ratchet/epoch boundary must not re-accept post-boundary ciphertext; session continues.

    Flow (A->B):
      1) Handshake A<->B
      2) Establish epoch E0 by delivering a boundary message
      3) A encrypt m1; B decrypt OK
      4) Snapshot S1 (both) -- pre-ratchet snapshot
      5) Advance epoch to E1 (B->A ping; A->B boundary), then A encrypt m2; B decrypt OK (post-boundary)
      6) Restart + restore S1
      7) Attempt decrypt ct2 on restored B -> must reject
      8) Fresh message m3 after restore decrypts OK
    """
    suite = "Suite-1"
    case_id = "IT-DUR-004.Suite-1.AtoB"
    details: Dict[str, Any] = {"case_id": case_id, "suite": suite}
    details["stages"] = []

    def mark(stage: str, **kw: Any) -> None:
        rec: Dict[str, Any] = {"stage": stage}
        rec.update(kw)
        details["stages"].append(rec)
        details["stage"] = stage


    dur_store_dir = tempfile.mkdtemp(prefix=f"qsl_dur_{case_id.replace('.', '_')}_")
    extra_env = {"QSL_TEST_HOOKS": "1", "QSL_DUR_STORE_DIR": dur_store_dir}

    a = ActorProc.start("impl_a", actor_path, extra_env)
    b = ActorProc.start("impl_b", actor_path, extra_env)

    def stop_all(*procs: ActorProc) -> None:
        for p in procs:
            try:
                p.stop()
            except Exception:
                pass

    def fail(stage: str, **kw: Any) -> Tuple[bool, Dict[str, Any]]:
        details.update({"stage": stage, **kw})
        stop_all(a, b)
        return False, details

    try:
        # Reset both.
        ra = a.request("reset", {}, rid=f"{case_id}_reset_a")
        rb = b.request("reset", {}, rid=f"{case_id}_reset_b")
        if not ra.get("ok") or not rb.get("ok"):
            return fail("reset", resp_a=ra, resp_b=rb)
        mark("reset_ok")

        # Handshake A->B.
        r1 = a.request("handshake_init", {"suite": suite, "options": {}}, rid=f"{case_id}_h1")
        if not r1.get("ok"):
            return fail("handshake_init", resp=r1)
        msg1 = (r1.get("result") or {}).get("msg1_b64")
        if not isinstance(msg1, str):
            return fail("handshake_init", error="missing_msg1_b64", resp=r1)

        r2 = b.request("handshake_respond", {"suite": suite, "msg1_b64": msg1, "options": {}}, rid=f"{case_id}_h2")
        if not r2.get("ok"):
            return fail("handshake_respond", resp=r2)
        msg2 = (r2.get("result") or {}).get("msg2_b64")
        if not isinstance(msg2, str):
            return fail("handshake_respond", error="missing_msg2_b64", resp=r2)

        r3 = a.request("handshake_finish", {"suite": suite, "msg2_b64": msg2, "options": {}}, rid=f"{case_id}_h3")
        if not r3.get("ok"):
            return fail("handshake_finish", resp=r3)
        session_id = (r3.get("result") or {}).get("session_id")
        if not isinstance(session_id, str):
            return fail("handshake_finish", error="missing_session_id", resp=r3)
        details["session_id"] = session_id
        mark("handshake_ok", session_id=session_id)

        # Establish epoch E0 (deliver a boundary message).
        pt_e0 = b64u_no_pad(f"{case_id}:e0:boundary".encode("utf-8"))
        enc_e0 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt_e0}, rid=f"{case_id}_e0_enc")
        if not enc_e0.get("ok"):
            return fail("e0_boundary_encrypt", resp=enc_e0)
        ct_e0 = (enc_e0.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct_e0, str):
            return fail("e0_boundary_encrypt", error="missing_ciphertext_b64", resp=enc_e0)

        dec_e0 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct_e0}, rid=f"{case_id}_e0_dec")
        if not dec_e0.get("ok"):
            return fail("e0_boundary_decrypt", resp=dec_e0)
        if (dec_e0.get("result") or {}).get("plaintext_b64") != pt_e0:
            return fail("e0_boundary_decrypt", error="plaintext_mismatch", expected=pt_e0, got=(dec_e0.get("result") or {}).get("plaintext_b64"))
        mark("e0_boundary_ok")

        # Message 1 A->B.
        pt1_b64 = b64u_no_pad(f"{case_id}:pt1".encode("utf-8"))
        enc1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt1_b64}, rid=f"{case_id}_enc1")
        if not enc1.get("ok"):
            return fail("encrypt1", resp=enc1)
        ct1 = (enc1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct1, str):
            return fail("encrypt1", error="missing_ciphertext_b64", resp=enc1)

        dec1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct1}, rid=f"{case_id}_dec1")
        if not dec1.get("ok"):
            return fail("decrypt1", resp=dec1)
        if (dec1.get("result") or {}).get("plaintext_b64") != pt1_b64:
            return fail("decrypt1", error="plaintext_mismatch", expected=pt1_b64, got=(dec1.get("result") or {}).get("plaintext_b64"), resp=dec1)
        mark("m1_ok")

        # Snapshot S1 (pre-ratchet snapshot).
        sa1 = a.request("debug_snapshot", {}, rid=f"{case_id}_snap1_a")
        sb1 = b.request("debug_snapshot", {}, rid=f"{case_id}_snap1_b")
        if not sa1.get("ok") or not sb1.get("ok"):
            return fail("snapshot1", resp_a=sa1, resp_b=sb1)
        ka1, sva1 = _extract_snapshot(sa1.get("result") or {})
        kb1, svb1 = _extract_snapshot(sb1.get("result") or {})
        details["snapshot1_key_a"] = ka1
        details["snapshot1_key_b"] = kb1
        mark("snapshot1_ok")

        # Advance epoch beyond S1: B->A ping, then A->B boundary.
        ping_b64 = b64u_no_pad(f"{case_id}:ping".encode("utf-8"))
        enc_ping = b.request("encrypt", {"session_id": session_id, "plaintext_b64": ping_b64}, rid=f"{case_id}_ping_enc")
        if not enc_ping.get("ok"):
            return fail("ping_encrypt", resp=enc_ping)
        ct_ping = (enc_ping.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct_ping, str):
            return fail("ping_encrypt", error="missing_ciphertext_b64", resp=enc_ping)

        dec_ping = a.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct_ping}, rid=f"{case_id}_ping_dec")
        if not dec_ping.get("ok"):
            return fail("ping_decrypt", resp=dec_ping)
        if (dec_ping.get("result") or {}).get("plaintext_b64") != ping_b64:
            return fail("ping_decrypt", error="plaintext_mismatch", expected=ping_b64, got=(dec_ping.get("result") or {}).get("plaintext_b64"), resp=dec_ping)

        pt_e1 = b64u_no_pad(f"{case_id}:e1:boundary".encode("utf-8"))
        enc_e1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt_e1}, rid=f"{case_id}_e1_enc")
        if not enc_e1.get("ok"):
            return fail("e1_boundary_encrypt", resp=enc_e1)
        ct_e1 = (enc_e1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct_e1, str):
            return fail("e1_boundary_encrypt", error="missing_ciphertext_b64", resp=enc_e1)

        dec_e1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct_e1}, rid=f"{case_id}_e1_dec")
        if not dec_e1.get("ok"):
            return fail("e1_boundary_decrypt", resp=dec_e1)
        if (dec_e1.get("result") or {}).get("plaintext_b64") != pt_e1:
            return fail("e1_boundary_decrypt", error="plaintext_mismatch", expected=pt_e1, got=(dec_e1.get("result") or {}).get("plaintext_b64"), resp=dec_e1)
        mark("ratchet_advance_ok")

        # Post-boundary message m2 A->B (accepted).
        pt2_b64 = b64u_no_pad(f"{case_id}:pt2_post_boundary".encode("utf-8"))
        enc2 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt2_b64}, rid=f"{case_id}_enc2")
        if not enc2.get("ok"):
            return fail("encrypt2", resp=enc2)
        ct2 = (enc2.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct2, str):
            return fail("encrypt2", error="missing_ciphertext_b64", resp=enc2)

        dec2 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_dec2")
        if not dec2.get("ok"):
            return fail("decrypt2", resp=dec2)
        if (dec2.get("result") or {}).get("plaintext_b64") != pt2_b64:
            return fail("decrypt2", error="plaintext_mismatch", expected=pt2_b64, got=(dec2.get("result") or {}).get("plaintext_b64"), resp=dec2)
        mark("m2_ok")

        # Restart + restore S1.
        a.stop(); b.stop()
        time.sleep(0.1)
        a2 = ActorProc.start("impl_a", actor_path, extra_env)
        b2 = ActorProc.start("impl_b", actor_path, extra_env)

        ra2 = a2.request("debug_restore", {ka1: sva1}, rid=f"{case_id}_rest1_a")
        rb2 = b2.request("debug_restore", {kb1: svb1}, rid=f"{case_id}_rest1_b")
        if not ra2.get("ok") or not rb2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "restore1", "resp_a": ra2, "resp_b": rb2})
            return False, details
        mark("restore1_ok")

        # Attempt decrypt ct2 (post-boundary ciphertext) on restored B: must reject.
        rej = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2}, rid=f"{case_id}_dec2_after_restore")
        if rej.get("ok") is True:
            stop_all(a2, b2)
            details.update({"stage": "decrypt2_after_restore", "error": "expected_reject", "resp": rej})
            return False, details
        mark("ct2_rejected", resp=rej)

        # Fresh message after restore: must succeed.
        pt3_b64 = b64u_no_pad(f"{case_id}:pt3".encode("utf-8"))
        enc3 = a2.request("encrypt", {"session_id": session_id, "plaintext_b64": pt3_b64}, rid=f"{case_id}_enc3")
        if not enc3.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "encrypt3", "resp": enc3})
            return False, details
        ct3 = (enc3.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct3, str):
            stop_all(a2, b2)
            details.update({"stage": "encrypt3", "error": "missing_ciphertext_b64", "resp": enc3})
            return False, details

        dec3 = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct3}, rid=f"{case_id}_dec3")
        if not dec3.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "decrypt3", "resp": dec3})
            return False, details
        if (dec3.get("result") or {}).get("plaintext_b64") != pt3_b64:
            stop_all(a2, b2)
            details.update({"stage": "decrypt3", "error": "plaintext_mismatch", "expected": pt3_b64, "got": (dec3.get("result") or {}).get("plaintext_b64"), "resp": dec3})
            return False, details
        mark("m3_ok")
        mark("ok")

        stop_all(a2, b2)
        return True, details

    except Exception as e:
        return fail("exception", error=str(e))


def durability_it_dur_005(actor_path: str) -> Tuple[bool, Dict[str, Any]]:
    """IT-DUR-005: rollback must reject post-boundary ciphertext that was never decrypted pre-rollback.

    This case is designed to avoid being explainable purely by the durable replay journal:
    we generate a post-boundary ciphertext (ct2b) but *do not* decrypt it before restoring
    a pre-boundary snapshot. After restore, ct2b must be rejected (epoch/ratchet mismatch),
    and the session must continue (forward progress).

    Flow (A->B):
      1) Handshake A<->B
      2) Establish epoch E0 by delivering a boundary message
      3) A encrypt m1; B decrypt OK
      4) Snapshot S1 (both) -- pre-ratchet snapshot
      5) Advance epoch to E1 (B->A ping; A->B boundary)
      6) A encrypt ct2a; B decrypt OK (prove E1 is active)
      7) A encrypt ct2b; (do NOT decrypt; not recorded in durable replay journal)
      8) Restart + restore S1
      9) Attempt decrypt ct2b on restored B -> must reject (and should NOT be durable-replay)
     10) Fresh message m3 after restore decrypts OK
    """
    suite = "Suite-1"
    case_id = "IT-DUR-005.Suite-1.AtoB"
    details: Dict[str, Any] = {"case_id": case_id, "suite": suite}
    details["stages"] = []

    def mark(stage: str, **kw: Any) -> None:
        rec: Dict[str, Any] = {"stage": stage}
        rec.update(kw)
        details["stages"].append(rec)
        details["stage"] = stage


    dur_store_dir = tempfile.mkdtemp(prefix=f"qsl_dur_{case_id.replace('.', '_')}_")
    extra_env = {"QSL_TEST_HOOKS": "1", "QSL_DUR_STORE_DIR": dur_store_dir}

    a = ActorProc.start("impl_a", actor_path, extra_env)
    b = ActorProc.start("impl_b", actor_path, extra_env)

    def stop_all(*procs: ActorProc) -> None:
        for p in procs:
            try:
                p.stop()
            except Exception:
                pass

    def fail(stage: str, **kw: Any) -> Tuple[bool, Dict[str, Any]]:
        details.update({"stage": stage, **kw})
        stop_all(a, b)
        return False, details

    def _looks_like_durable_replay(resp: Dict[str, Any]) -> bool:
        # Best-effort: if the actor reports the durable replay guard triggered, the error message
        # historically includes the substring "replay (durable)".
        try:
            blob = json.dumps(resp, sort_keys=True)
        except Exception:
            blob = str(resp)
        return "replay (durable)" in blob or "replay(durable)" in blob

    try:
        # Reset both.
        ra = a.request("reset", {}, rid=f"{case_id}_reset_a")
        rb = b.request("reset", {}, rid=f"{case_id}_reset_b")
        if not ra.get("ok") or not rb.get("ok"):
            return fail("reset", resp_a=ra, resp_b=rb)
        mark("reset_ok")

        # Handshake A->B.
        r1 = a.request("handshake_init", {"suite": suite, "options": {}}, rid=f"{case_id}_h1")
        if not r1.get("ok"):
            return fail("handshake_init", resp=r1)
        msg1 = (r1.get("result") or {}).get("msg1_b64")
        if not isinstance(msg1, str):
            return fail("handshake_init", error="missing_msg1_b64", resp=r1)

        r2 = b.request("handshake_respond", {"suite": suite, "msg1_b64": msg1, "options": {}}, rid=f"{case_id}_h2")
        if not r2.get("ok"):
            return fail("handshake_respond", resp=r2)
        msg2 = (r2.get("result") or {}).get("msg2_b64")
        if not isinstance(msg2, str):
            return fail("handshake_respond", error="missing_msg2_b64", resp=r2)

        r3 = a.request("handshake_finish", {"suite": suite, "msg2_b64": msg2, "options": {}}, rid=f"{case_id}_h3")
        if not r3.get("ok"):
            return fail("handshake_finish", resp=r3)
        session_id = (r3.get("result") or {}).get("session_id")
        if not isinstance(session_id, str):
            return fail("handshake_finish", error="missing_session_id", resp=r3)
        details["session_id"] = session_id
        mark("handshake_ok", session_id=session_id)

        # Establish epoch E0 (deliver a boundary message).
        pt_e0 = b64u_no_pad(f"{case_id}:e0:boundary".encode("utf-8"))
        enc_e0 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt_e0}, rid=f"{case_id}_e0_enc")
        if not enc_e0.get("ok"):
            return fail("e0_boundary_encrypt", resp=enc_e0)
        ct_e0 = (enc_e0.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct_e0, str):
            return fail("e0_boundary_encrypt", error="missing_ciphertext_b64", resp=enc_e0)

        dec_e0 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct_e0}, rid=f"{case_id}_e0_dec")
        if not dec_e0.get("ok"):
            return fail("e0_boundary_decrypt", resp=dec_e0)
        if (dec_e0.get("result") or {}).get("plaintext_b64") != pt_e0:
            return fail("e0_boundary_decrypt", error="plaintext_mismatch", expected=pt_e0, got=(dec_e0.get("result") or {}).get("plaintext_b64"))
        mark("e0_boundary_ok")

        # Message 1 A->B.
        pt1_b64 = b64u_no_pad(f"{case_id}:pt1".encode("utf-8"))
        enc1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt1_b64}, rid=f"{case_id}_enc1")
        if not enc1.get("ok"):
            return fail("encrypt1", resp=enc1)
        ct1 = (enc1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct1, str):
            return fail("encrypt1", error="missing_ciphertext_b64", resp=enc1)

        dec1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct1}, rid=f"{case_id}_dec1")
        if not dec1.get("ok"):
            return fail("decrypt1", resp=dec1)
        if (dec1.get("result") or {}).get("plaintext_b64") != pt1_b64:
            return fail("decrypt1", error="plaintext_mismatch", expected=pt1_b64, got=(dec1.get("result") or {}).get("plaintext_b64"), resp=dec1)
        mark("m1_ok")

        # Snapshot S1 (pre-ratchet snapshot).
        sa1 = a.request("debug_snapshot", {}, rid=f"{case_id}_snap1_a")
        sb1 = b.request("debug_snapshot", {}, rid=f"{case_id}_snap1_b")
        if not sa1.get("ok") or not sb1.get("ok"):
            return fail("snapshot1", resp_a=sa1, resp_b=sb1)
        ka1, sva1 = _extract_snapshot(sa1.get("result") or {})
        kb1, svb1 = _extract_snapshot(sb1.get("result") or {})
        details["snapshot1_key_a"] = ka1
        details["snapshot1_key_b"] = kb1
        mark("snapshot1_ok")

        # Advance epoch beyond S1: B->A ping, then A->B boundary.
        ping_b64 = b64u_no_pad(f"{case_id}:ping".encode("utf-8"))
        enc_ping = b.request("encrypt", {"session_id": session_id, "plaintext_b64": ping_b64}, rid=f"{case_id}_ping_enc")
        if not enc_ping.get("ok"):
            return fail("ping_encrypt", resp=enc_ping)
        ct_ping = (enc_ping.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct_ping, str):
            return fail("ping_encrypt", error="missing_ciphertext_b64", resp=enc_ping)

        dec_ping = a.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct_ping}, rid=f"{case_id}_ping_dec")
        if not dec_ping.get("ok"):
            return fail("ping_decrypt", resp=dec_ping)
        if (dec_ping.get("result") or {}).get("plaintext_b64") != ping_b64:
            return fail("ping_decrypt", error="plaintext_mismatch", expected=ping_b64, got=(dec_ping.get("result") or {}).get("plaintext_b64"), resp=dec_ping)

        pt_e1 = b64u_no_pad(f"{case_id}:e1:boundary".encode("utf-8"))
        enc_e1 = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt_e1}, rid=f"{case_id}_e1_enc")
        if not enc_e1.get("ok"):
            return fail("e1_boundary_encrypt", resp=enc_e1)
        ct_e1 = (enc_e1.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct_e1, str):
            return fail("e1_boundary_encrypt", error="missing_ciphertext_b64", resp=enc_e1)

        dec_e1 = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct_e1}, rid=f"{case_id}_e1_dec")
        if not dec_e1.get("ok"):
            return fail("e1_boundary_decrypt", resp=dec_e1)
        if (dec_e1.get("result") or {}).get("plaintext_b64") != pt_e1:
            return fail("e1_boundary_decrypt", error="plaintext_mismatch", expected=pt_e1, got=(dec_e1.get("result") or {}).get("plaintext_b64"), resp=dec_e1)
        mark("ratchet_advance_ok")

        # Post-boundary message ct2a A->B (accepted) - proves E1 works.
        pt2a_b64 = b64u_no_pad(f"{case_id}:pt2a_post_boundary".encode("utf-8"))
        enc2a = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt2a_b64}, rid=f"{case_id}_enc2a")
        if not enc2a.get("ok"):
            return fail("encrypt2a", resp=enc2a)
        ct2a = (enc2a.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct2a, str):
            return fail("encrypt2a", error="missing_ciphertext_b64", resp=enc2a)

        dec2a = b.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2a}, rid=f"{case_id}_dec2a")
        if not dec2a.get("ok"):
            return fail("decrypt2a", resp=dec2a)
        if (dec2a.get("result") or {}).get("plaintext_b64") != pt2a_b64:
            return fail("decrypt2a", error="plaintext_mismatch", expected=pt2a_b64, got=(dec2a.get("result") or {}).get("plaintext_b64"), resp=dec2a)
        mark("m2a_ok")

        # Post-boundary message ct2b A->B - do NOT decrypt pre-rollback.
        pt2b_b64 = b64u_no_pad(f"{case_id}:pt2b_post_boundary_nodecrypt".encode("utf-8"))
        enc2b = a.request("encrypt", {"session_id": session_id, "plaintext_b64": pt2b_b64}, rid=f"{case_id}_enc2b")
        if not enc2b.get("ok"):
            return fail("encrypt2b", resp=enc2b)
        ct2b = (enc2b.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct2b, str):
            return fail("encrypt2b", error="missing_ciphertext_b64", resp=enc2b)
        mark("m2b_created")

        # Restart + restore S1.
        a.stop(); b.stop()
        time.sleep(0.1)
        a2 = ActorProc.start("impl_a", actor_path, extra_env)
        b2 = ActorProc.start("impl_b", actor_path, extra_env)

        ra2 = a2.request("debug_restore", {ka1: sva1}, rid=f"{case_id}_rest1_a")
        rb2 = b2.request("debug_restore", {kb1: svb1}, rid=f"{case_id}_rest1_b")
        if not ra2.get("ok") or not rb2.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "restore1", "resp_a": ra2, "resp_b": rb2})
            return False, details
        mark("restore1_ok")

        # Attempt decrypt ct2b (post-boundary ciphertext) on restored B: must reject.
        rej = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct2b}, rid=f"{case_id}_dec2b_after_restore")
        if rej.get("ok") is True:
            stop_all(a2, b2)
            details.update({"stage": "decrypt2b_after_restore", "error": "expected_reject", "resp": rej})
            return False, details

        # If this is rejected as a durable replay, the case is not meeting its objective.
        if _looks_like_durable_replay(rej):
            stop_all(a2, b2)
            details.update({"stage": "decrypt2b_after_restore", "error": "unexpected_durable_replay_reject", "resp": rej})
            return False, details

        mark("ct2b_rejected", resp=rej)

        # Fresh message after restore: must succeed.
        pt3_b64 = b64u_no_pad(f"{case_id}:pt3".encode("utf-8"))
        enc3 = a2.request("encrypt", {"session_id": session_id, "plaintext_b64": pt3_b64}, rid=f"{case_id}_enc3")
        if not enc3.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "encrypt3", "resp": enc3})
            return False, details
        ct3 = (enc3.get("result") or {}).get("ciphertext_b64")
        if not isinstance(ct3, str):
            stop_all(a2, b2)
            details.update({"stage": "encrypt3", "error": "missing_ciphertext_b64", "resp": enc3})
            return False, details

        dec3 = b2.request("decrypt", {"session_id": session_id, "ciphertext_b64": ct3}, rid=f"{case_id}_dec3")
        if not dec3.get("ok"):
            stop_all(a2, b2)
            details.update({"stage": "decrypt3", "resp": dec3})
            return False, details
        if (dec3.get("result") or {}).get("plaintext_b64") != pt3_b64:
            stop_all(a2, b2)
            details.update({"stage": "decrypt3", "error": "plaintext_mismatch", "expected": pt3_b64, "got": (dec3.get("result") or {}).get("plaintext_b64"), "resp": dec3})
            return False, details
        mark("m3_ok")
        mark("ok")

        stop_all(a2, b2)
        return True, details

    except Exception as e:
        return fail("exception", error=str(e))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--evidence", required=True)
    ap.add_argument("--git-commit", required=True)
    ap.add_argument("--actor", default="tools/actors/refimpl_actor")
    args = ap.parse_args()

    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)

    report = {
        "run_id": out_dir.parent.name,  # artifacts/<run_id>/4D
        "ts": now_iso(),
        "git_commit": args.git_commit,
        "ok": False,
        "errors": [],
        "warnings": [],
        "coverage": {"profile": "durability", "required_prefixes": ["IT-DUR-001", "IT-DUR-002", "IT-DUR-003", "IT-DUR-004", "IT-DUR-005"]},
        "results": [],
    }

    ok1, det1 = durability_it_dur_001(args.actor)
    report["results"].append({
        "case_id": det1.get("case_id"),
        "p3_case_id": "IT-DUR-001",
        "suite": det1.get("suite"),
        "direction": "A->B",
        "ok": ok1,
        "stage": det1.get("stage", ""),
        "details": det1,
    })

    ok2, det2 = durability_it_dur_002(args.actor)
    report["results"].append({
        "case_id": det2.get("case_id"),
        "p3_case_id": "IT-DUR-002",
        "suite": det2.get("suite"),
        "direction": "A->B",
        "ok": ok2,
        "stage": det2.get("stage", ""),
        "details": det2,
    })

    ok3, det3 = durability_it_dur_003(args.actor)
    report["results"].append({
        "case_id": det3.get("case_id"),
        "p3_case_id": "IT-DUR-003",
        "suite": det3.get("suite"),
        "direction": "A->B",
        "ok": ok3,
        "stage": det3.get("stage", ""),
        "details": det3,
    })


    ok4, det4 = durability_it_dur_004(args.actor)
    report["results"].append({
        "case_id": det4.get("case_id"),
        "p3_case_id": "IT-DUR-004",
        "suite": det4.get("suite"),
        "direction": "A->B",
        "ok": ok4,
        "stage": det4.get("stage", ""),
        "details": det4,
    })
    

    ok5, det5 = durability_it_dur_005(args.actor)
    report["results"].append({
        "case_id": det5.get("case_id"),
        "p3_case_id": "IT-DUR-005",
        "suite": det5.get("suite"),
        "direction": "A->B",
        "ok": ok5,
        "stage": det5.get("stage", ""),
        "details": det5,
    })

    report["ok"] = bool(ok1 and ok2 and ok3 and ok4 and ok5)
    out_path = out_dir / "D2_durability.json"
    out_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(f"[4D-DUR] wrote {out_path}")
    if not report["ok"]:
        print("[4D-DUR] FAIL")
        return 1
    print("[4D-DUR] OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
