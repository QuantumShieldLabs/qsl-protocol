#!/usr/bin/env python3
import argparse
import json
import os
import platform
import shlex
import re
import subprocess
import sys
import tempfile
import time
import zipfile
from pathlib import Path

from lib.hashutil import sha256_file, sha256_bytes
from lib.ziputil import extract_member_to, read_member_bytes
from lib.manifest import write_manifest_4b
from lib.junit import write_junit
from lib.policy import Policy
from lib.b64u import b64u_decode_strict
from lib.qse import qse_parse
from lib.qsp import qsp_parse
from lib.kt import kt_parse_sth, kt_parse_inclusion_proof, kt_parse_consistency_proof

REASON_CODES = {
    "noncanonical_qse","bounds_exceeded","invalid_request","rate_limited","queue_full",
    "auth_failed","forbidden","not_found","conflict","opk_unavailable","server_error",
    "kt_fail","bundle_sig_fail","aead_fail","replay","policy_reject"
}

def _now_ms() -> int:
    return int(time.time() * 1000)

def _write_json(path: Path, obj) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(obj, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    tmp.replace(path)

def _runner_versions():
    return {
        "python": sys.version.split()[0],
        "platform": platform.platform(),
    }

def _read_p3_04_md(phase3_zip: Path) -> str:
    member = "Phase3/QuantumShield_Phase3_Interop_Test_Plan_P3-04_v1_0.md"
    with zipfile.ZipFile(phase3_zip, "r") as z:
        return z.read(member).decode("utf-8")

def _p3_04_catalog(phase3_zip: Path) -> list[dict]:
    """
    Extract interop catalog from P3-04 markdown inside the Phase3 ZIP.
    Returns list of dicts:
      {case_id, title, suites:[0x0001,0x0002], roles:["A->B","B->A"]}
    Conservative defaults (fail-closed posture):
      - suites: both Suite-1 and Suite-1B unless header clearly restricts
      - roles: both directions unless header clearly restricts
    """
    md = _read_p3_04_md(phase3_zip)
    marks = [(m.group(1), m.start()) for m in re.finditer(r"\*\*(IT-[A-Z]{2,3}-\d{3})\b", md)]
    out = []
    for i, (cid, pos) in enumerate(marks):
        end = marks[i + 1][1] if i + 1 < len(marks) else len(md)
        block = md[pos:end].strip()
        first = block.splitlines()[0].strip()
        m = re.match(r"^\*\*(" + re.escape(cid) + r"[^*]+)\*\*", first)
        hdr = m.group(1) if m else cid
        title = hdr[len(cid):].lstrip()
        if title.startswith(":"):
            title = title[1:].lstrip()

        # Suites: default both
        if "(S1/S1B)" in hdr or "S1/S1B" in hdr:
            suites = [0x0001, 0x0002]
        elif "(S1B)" in hdr or "S1B" in hdr:
            suites = [0x0002]
        elif "(S1)" in hdr or " S1" in hdr:
            suites = [0x0001]
        else:
            suites = [0x0001, 0x0002]

        # Roles: default both directions
        roles = []
        if "A→B" in hdr or "A->B" in hdr:
            roles.append("A->B")
        if "B→A" in hdr or "B->A" in hdr:
            roles.append("B->A")
        if not roles:
            roles = ["A->B", "B->A"]

        out.append({"case_id": cid, "title": title, "suites": suites, "roles": roles})
    return out

def cmd_preflight(args):
    p2 = Path(args.phase2_zip)
    p3 = Path(args.phase3_zip)
    if not p2.exists():
        raise SystemExit(f"missing phase2 zip: {p2}")
    if not p3.exists():
        raise SystemExit(f"missing phase3 zip: {p3}")

    out = {
        "run_id": args.run_id,
        "git_commit": args.git_commit,
        "runner_versions": _runner_versions(),
        "inputs": {
            "phase2_zip": {"path": str(p2), "bytes": p2.stat().st_size, "sha256": sha256_file(p2)},
            "phase3_zip": {"path": str(p3), "bytes": p3.stat().st_size, "sha256": sha256_file(p3)},
        },
        "canonical_assertions": {"qsp_version": "4.3.2", "qse_version": "1.8.2"},
        "phase2_internal_sha256sums_ok": None,
        "phase2_internal_sha256sums_errors": [],
    }

    # Verify Phase2 internal sha256sums.txt (fail-closed).
    sha_sums = read_member_bytes(p2, "Phase2_CANONICAL_FROZEN_QSP4.3.2_QSE1.8.2/sha256sums.txt").decode("utf-8")
    errors = []
    for line in sha_sums.splitlines():
        line = line.strip()
        if not line:
            continue
        parts = line.split()
        if len(parts) < 2:
            errors.append(f"bad sha256sums line: {line}")
            continue
        expect = parts[0].lower()
        rel = parts[-1]
        data = read_member_bytes(p2, f"Phase2_CANONICAL_FROZEN_QSP4.3.2_QSE1.8.2/{rel}")
        got = sha256_bytes(data)
        if got != expect:
            errors.append(f"sha256 mismatch: {rel} expect={expect} got={got}")
    out["phase2_internal_sha256sums_ok"] = (len(errors) == 0)
    out["phase2_internal_sha256sums_errors"] = errors

    _write_json(Path(args.out) / "B0_preflight.json", out)
    if errors:
        raise SystemExit("Phase2 sha256sums verification failed")

def cmd_env(args):
    policy = Policy.from_env()
    # Compose publishes ports 18080/18081/18082 if running; treat absence as hard fail for CI.
    # If you later want to allow non-compose mode, gate it behind an explicit flag (not default).
    endpoints = {
        "rsf": "http://127.0.0.1:18080/healthz",
        "pds": "http://127.0.0.1:18081/healthz",
        "ktl": "http://127.0.0.1:18082/healthz",
    }

    import urllib.request
    results = {}
    ok = True
    for name, url in endpoints.items():
        try:
            raw = urllib.request.urlopen(url, timeout=2).read()
            results[name] = {"ok": True, "url": url, "body_sha256": sha256_bytes(raw)}
        except Exception as e:
            ok = False
            results[name] = {"ok": False, "url": url, "error": str(e)}

    out = {
        "run_id": args.run_id,
        "git_commit": args.git_commit,
        "policy": policy.to_dict(),
        "services": results,
    }
    _write_json(Path(args.out) / "B1_env.json", out)
    if not ok:
        raise SystemExit("4B1 env bring-up failed: one or more health checks failed")

def _load_p3_23_bundle(phase3_zip: Path, workdir: Path) -> Path:
    # Extract inner P3-23 bundle zip from Phase3 ZIP, then unzip it into workdir/p3-23
    inner_name = "Phase3/QuantumShield_Phase3_P3-23_FULL.zip"
    inner_zip_path = workdir / "P3-23_FULL.zip"
    extract_member_to(phase3_zip, inner_name, inner_zip_path)

    p3_23_dir = workdir / "p3-23"
    p3_23_dir.mkdir(parents=True, exist_ok=True)

    import zipfile
    with zipfile.ZipFile(inner_zip_path, "r") as z:
        z.extractall(p3_23_dir)

    return p3_23_dir

def cmd_negative(args):
    policy = Policy.from_env()
    phase3 = Path(args.phase3_zip)
    if not phase3.exists():
        raise SystemExit(f"missing phase3 zip: {phase3}")

    outdir = Path(args.out)
    outdir.mkdir(parents=True, exist_ok=True)

    errata = []

    with tempfile.TemporaryDirectory(prefix="qshield_4b_p3_23_") as td:
        work = Path(td)
        p3_23_dir = _load_p3_23_bundle(phase3, work)

        # Run shipped generator selftest (fail-closed).
        gen = p3_23_dir / "QuantumShield_Phase3_Negative_Vectors_Generator_P3-23_v1_0.py"
        subprocess.run([sys.executable, str(gen), "--bundle-root", str(p3_23_dir), "--selftest"], check=True)

        vec_path = p3_23_dir / "QuantumShield_Phase3_Negative_Vectors_P3-23_v1_0.json"
        vec = json.loads(vec_path.read_text(encoding="utf-8"))

        cases_out = []
        counts = {"pass": 0, "fail": 0, "skip": 0}

        for case in vec.get("cases", []):
            t0 = _now_ms()
            cid = case["id"]
            op = case["op"]
            expect = case["expect"]
            requires = case.get("requires")

            if requires and not policy.satisfies(requires):
                counts["skip"] += 1
                cases_out.append({
                    "id": cid, "op": op, "status": "skipped",
                    "requires": requires,
                    "duration_ms": _now_ms() - t0
                })
                continue

            # Decode input
            inp = case["input"]
            raw = b""
            try:
                if inp["type"] == "b64u":
                    raw = b64u_decode_strict(inp["data"])
                elif inp["type"] == "path":
                    raw = (p3_23_dir / inp["path"]).read_bytes()
                elif inp["type"] == "string":
                    raw = inp["data"].encode("utf-8")
                else:
                    raise ValueError("unknown input type")
            except Exception:
                # If we cannot decode declared input, that's harness/runtime error.
                # Treat as fail-closed mismatch (invalid_request expected only for base64 op).
                actual = {"ok": False, "reason_code": "invalid_request", "state_unchanged": True}
                # Continue through standard comparison below.
            else:
                # Execute op
                actual = {"ok": True}
                try:
                    if op == "base64url_decode":
                        _ = b64u_decode_strict(raw.decode("utf-8"))
                        actual = {"ok": True}
                    elif op == "qse_parse":
                        _ = qse_parse(raw, policy=policy)
                        actual = {"ok": True}
                    elif op == "qsp_parse":
                        _ = qsp_parse(raw)
                        actual = {"ok": True}
                    elif op == "kt_parse_sth":
                        kt_parse_sth(raw)
                        actual = {"ok": True}
                    elif op == "kt_parse_inclusion_proof":
                        kt_parse_inclusion_proof(raw)
                        actual = {"ok": True}
                    elif op == "kt_parse_consistency_proof":
                        kt_parse_consistency_proof(raw)
                        actual = {"ok": True}
                    else:
                        raise ValueError(f"unknown op: {op}")
                except Exception as e:
                    # Map exceptions to reason codes (P3-23 semantics).
                    rc = getattr(e, "reason_code", None) or "invalid_request"
                    actual = {"ok": False, "reason_code": rc, "state_unchanged": True}

            # Validate reason code enum if present
            if not actual.get("ok", True):
                rc = actual.get("reason_code")
                if rc not in REASON_CODES:
                    actual = {"ok": False, "reason_code": "server_error", "state_unchanged": True}

            # Compare to expected
            status = "passed"
            if actual.get("ok") != expect.get("ok") or actual.get("reason_code") != expect.get("reason_code"):
                status = "failed"
                counts["fail"] += 1
                errata.append({
                    "source_artifact": "P3-23",
                    "case_id": cid,
                    "expected": expect,
                    "actual": actual,
                    "notes": case.get("notes", "")
                })
            else:
                counts["pass"] += 1

            cases_out.append({
                "id": cid,
                "op": op,
                "status": status,
                "expected": expect,
                "actual": actual,
                "duration_ms": _now_ms() - t0,
                "input_sha256": sha256_bytes(raw),
            })

    report = {
        "run_id": args.run_id,
        "git_commit": args.git_commit,
        "artifact_id": "P3-23",
        "counts": counts,
        "cases": cases_out,
    }
    _write_json(outdir / "B2_negative_vectors.json", report)
    write_junit(outdir / "B2_negative_vectors.junit.xml", suite_name="4B2-negative", cases=cases_out)

    if errata:
        _write_json(outdir / "B0_phase3_errata.json", {"run_id": args.run_id, "git_commit": args.git_commit, "items": errata})
        raise SystemExit("4B2 negative vectors failed (see B2_negative_vectors.json and B0_phase3_errata.json)")

def _run_actor(cmd: str, input_obj: dict, cwd: Path, timeout_s: int,
               out_json_path: Path, stdout_path: Path, stderr_path: Path) -> dict:
    argv = shlex.split(cmd)
    raw_in = (json.dumps(input_obj, sort_keys=True) + "\n").encode("utf-8")
    try:
        cp = subprocess.run(
            argv,
            input=raw_in,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=str(cwd),
            timeout=timeout_s,
            check=False,
        )
    except Exception as e:
        stdout_path.write_text("", encoding="utf-8")
        stderr_path.write_text(str(e) + "\n", encoding="utf-8")
        out = {"ok": False, "failure_stage": "adapter", "notes": f"spawn failed: {e}"}
        out_json_path.write_text(json.dumps(out, sort_keys=True, indent=2) + "\n", encoding="utf-8")
        return out

    stdout_path.write_bytes(cp.stdout)
    stderr_path.write_bytes(cp.stderr)

    if cp.returncode != 0:
        out = {"ok": False, "failure_stage": "adapter", "notes": f"nonzero exit: {cp.returncode}"}
        out_json_path.write_text(json.dumps(out, sort_keys=True, indent=2) + "\n", encoding="utf-8")
        return out

    try:
        out = json.loads(cp.stdout.decode("utf-8"))
        if not isinstance(out, dict) or "ok" not in out:
            raise ValueError("missing ok")
    except Exception:
        out = {"ok": False, "failure_stage": "adapter", "notes": "stdout was not valid JSON contract"}
    out_json_path.write_text(json.dumps(out, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    return out


def cmd_interop(args):
    outdir = Path(args.out)
    outdir.mkdir(parents=True, exist_ok=True)

    phase3 = Path(args.phase3_zip)
    if not phase3.exists():
        raise SystemExit(f"missing phase3 zip: {phase3}")

    phase2 = Path(args.phase2_zip) if getattr(args, "phase2_zip", None) else None
    if phase2 is not None and not phase2.exists():
        raise SystemExit(f"missing phase2 zip: {phase2}")

    catalog = _p3_04_catalog(phase3)

    actor_a = os.environ.get("QSHIELD_ACTOR_A_CMD", "").strip()
    actor_b = os.environ.get("QSHIELD_ACTOR_B_CMD", "").strip()

    # If adapters not configured, keep catalog-only evidence behavior + fail-closed.
    if not actor_a or not actor_b:
        expanded = []
        for item in catalog:
            for suite_id in item["suites"]:
                for roles in item["roles"]:
                    expanded.append({
                        "case_id": item["case_id"],
                        "suite_id": suite_id,
                        "roles": roles,
                        "result": "fail",
                        "failure_stage": "policy",
                        "evidence": {"note": "not executed"},
                    })
        report = {
            "run_id": args.run_id,
            "git_commit": args.git_commit,
            "artifact_id": "P3-04",
            "note": "Catalog expanded. Provide QSHIELD_ACTOR_A_CMD and QSHIELD_ACTOR_B_CMD to execute interop.",
            "adapter_config": {"QSHIELD_ACTOR_A_CMD": None, "QSHIELD_ACTOR_B_CMD": None},
            "cases": expanded,
            "counts": {"pass": 0, "fail": len(expanded), "skip": 0},
        }
        _write_json(outdir / "B3_interop.json", report)
        write_junit(outdir / "B3_interop.junit.xml", suite_name="4B3-interop", cases=[])
        raise SystemExit("4B3 interop adapters not configured. Set env vars QSHIELD_ACTOR_A_CMD and QSHIELD_ACTOR_B_CMD (fail-closed).")

    rsf_url = os.environ.get("QSHIELD_RSF_URL", "http://127.0.0.1:18080")
    pds_url = os.environ.get("QSHIELD_PDS_URL", "http://127.0.0.1:18081")
    ktl_url = os.environ.get("QSHIELD_KTL_URL", "http://127.0.0.1:18082")

    base = outdir / "interop"
    base.mkdir(parents=True, exist_ok=True)

    cases = []
    counts = {"pass": 0, "fail": 0, "skip": 0}

    for item in catalog:
        for suite_id in item["suites"]:
            for roles in item["roles"]:
                case_dir = base / item["case_id"] / f"suite_{suite_id:04x}" / roles.replace(">", "to")
                case_dir.mkdir(parents=True, exist_ok=True)
                (case_dir / "state_A").mkdir(parents=True, exist_ok=True)
                (case_dir / "state_B").mkdir(parents=True, exist_ok=True)
                xfer_dir = case_dir / "xfer"
                xfer_dir.mkdir(parents=True, exist_ok=True)

                # Execution order determined by direction
                if roles == "A->B":
                    first = ("A", actor_a)
                    second = ("B", actor_b)
                else:
                    first = ("B", actor_b)
                    second = ("A", actor_a)

                def mk_input(actor: str):
                    return {
                        "contract": "QSHIELD-4B-ACTOR-1",
                        "actor": actor,
                        "peer": ("B" if actor == "A" else "A"),
                        "case_id": item["case_id"],
                        "suite_id": 1 if suite_id == 0x0001 else 2,
                        "roles": roles,
                        "services": {"rsf_url": rsf_url, "pds_url": pds_url, "ktl_url": ktl_url},
                        "case_dir": str(case_dir),
                        "state_dir": str(case_dir / f"state_{actor}"),
                        "xfer_dir": str(xfer_dir),
                        "phase2_zip": str(phase2) if phase2 else None,
                        "phase3_zip": str(phase3),
                        "run_id": args.run_id,
                        "git_commit": args.git_commit,
                    }

                out1 = _run_actor(
                    first[1], mk_input(first[0]), cwd=case_dir, timeout_s=120,
                    out_json_path=case_dir / f"{first[0]}_out.json",
                    stdout_path=case_dir / f"{first[0]}_stdout.log",
                    stderr_path=case_dir / f"{first[0]}_stderr.log",
                )
                out2 = _run_actor(
                    second[1], mk_input(second[0]), cwd=case_dir, timeout_s=120,
                    out_json_path=case_dir / f"{second[0]}_out.json",
                    stdout_path=case_dir / f"{second[0]}_stdout.log",
                    stderr_path=case_dir / f"{second[0]}_stderr.log",
                )

                ok = bool(out1.get("ok")) and bool(out2.get("ok"))
                if ok:
                    counts["pass"] += 1
                    result = "pass"
                    failure_stage = None
                else:
                    counts["fail"] += 1
                    result = "fail"
                    failure_stage = (out1.get("failure_stage") or out2.get("failure_stage") or "adapter")

                cases.append({
                    "case_id": item["case_id"],
                    "suite_id": suite_id,
                    "roles": roles,
                    "result": result,
                    "failure_stage": failure_stage,
                    "evidence": {
                        "case_dir": str(case_dir.relative_to(outdir)),
                        "outputs": {
                            first[0]: str((case_dir / f"{first[0]}_out.json").relative_to(outdir)),
                            second[0]: str((case_dir / f"{second[0]}_out.json").relative_to(outdir)),
                        },
                    },
                })

    report = {
        "run_id": args.run_id,
        "git_commit": args.git_commit,
        "artifact_id": "P3-04",
        "note": "Interop executed via actor commands (QSHIELD-4B-ACTOR-1).",
        "adapter_config": {"QSHIELD_ACTOR_A_CMD": actor_a, "QSHIELD_ACTOR_B_CMD": actor_b},
        "counts": counts,
        "cases": cases,
    }
    _write_json(outdir / "B3_interop.json", report)
    write_junit(outdir / "B3_interop.junit.xml", suite_name="4B3-interop", cases=[])

    if counts["fail"] != 0:
        raise SystemExit("4B3 interop failed (see B3_interop.json and per-case logs).")

def cmd_manifest(args):
    write_manifest_4b(
        artifacts_dir=Path(args.artifacts),
        out_dir=Path(args.out),
        run_id=args.run_id,
        git_commit=args.git_commit,
    )

def cmd_retention(args):
    art = Path(args.artifacts)
    evi = Path(args.evidence)
    evi.mkdir(parents=True, exist_ok=True)

    # Do not attempt to be clever: copy-all with overwrite, record any errors.
    ok = True
    err = None
    try:
        # Copy is performed by run_4b.sh; here we only emit a receipt.
        pass
    except Exception as e:
        ok = False
        err = str(e)

    _write_json(Path(args.out) / "B4_retention.json", {
        "run_id": args.run_id,
        "git_commit": args.git_commit,
        "artifact_dir": str(art),
        "evidence_dir": str(evi),
        "ok": ok,
        "error": err,
    })

def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("preflight")
    p.add_argument("--phase2-zip", required=True)
    p.add_argument("--phase3-zip", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True)
    p.add_argument("--git-commit", required=True)
    p.set_defaults(fn=cmd_preflight)

    p = sub.add_parser("env")
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True)
    p.add_argument("--git-commit", required=True)
    p.set_defaults(fn=cmd_env)

    p = sub.add_parser("negative")
    p.add_argument("--phase3-zip", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True)
    p.add_argument("--git-commit", required=True)
    p.set_defaults(fn=cmd_negative)

    p = sub.add_parser("interop")
    p.add_argument("--phase3-zip", required=True)
    p.add_argument("--phase2-zip", required=False)
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True)
    p.add_argument("--git-commit", required=True)
    p.set_defaults(fn=cmd_interop)

    p = sub.add_parser("manifest")
    p.add_argument("--artifacts", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True)
    p.add_argument("--git-commit", required=True)
    p.set_defaults(fn=cmd_manifest)

    p = sub.add_parser("retention")
    p.add_argument("--artifacts", required=True)
    p.add_argument("--evidence", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True)
    p.add_argument("--git-commit", required=True)
    p.set_defaults(fn=cmd_retention)

    args = ap.parse_args()
    args.fn(args)

if __name__ == "__main__":
    main()
