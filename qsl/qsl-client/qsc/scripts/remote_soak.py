#!/usr/bin/env python3
import argparse
import datetime as dt
import hashlib
import json
import os
import re
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path
from urllib.parse import urlparse


MARKER_DRYRUN_OK = "QSC_SOAK_DRYRUN_OK"
MARKER_RESULT_PASS = "QSC_SOAK_RESULT PASS"
MARKER_RESULT_FAIL = "QSC_SOAK_RESULT FAIL"
MARKER_STATE_ROOT_OK = "QSC_SOAK_STATE_ROOT_OK mode=700 parent_safe=yes"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="QSC remote relay soak harness (NA-0165)."
    )
    parser.add_argument("--relay-url", required=True, help="HTTPS relay base URL")
    parser.add_argument("--clients", type=int, default=100)
    parser.add_argument("--duration-secs", type=int, default=60)
    parser.add_argument("--workdir", default="target/qsc-soak")
    parser.add_argument("--state-root", default=None)
    parser.add_argument("--keep-state", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument(
        "--qsc-bin",
        default=None,
        help="Path to qsc binary (default: qsc or QSC_BIN env)",
    )
    parser.add_argument(
        "--files",
        action="store_true",
        help="Include small file transfer checks per pair (default: off).",
    )
    return parser.parse_args()


def sanitize_line(line: str, relay_token: str | None) -> str:
    out = line
    if relay_token:
        out = out.replace(relay_token, "<redacted-token>")
    out = re.sub(r"/v1/[A-Za-z0-9._~\-]+", "/v1/<redacted>", out)
    out = re.sub(r"([A-Fa-f0-9]{24,})", "<redacted-hex>", out)
    return out


def route_token_for(name: str) -> str:
    digest = hashlib.sha256(name.encode("utf-8")).hexdigest()
    return f"rt_{digest[:48]}"


def must_validate_relay_url(relay_url: str) -> None:
    parsed = urlparse(relay_url)
    if parsed.scheme.lower() != "https":
        print(f"{MARKER_RESULT_FAIL} code=relay_url_must_be_https", flush=True)
        sys.exit(2)
    if not parsed.netloc:
        print(f"{MARKER_RESULT_FAIL} code=relay_url_invalid", flush=True)
        sys.exit(2)


def repo_root() -> Path:
    return Path(__file__).resolve().parents[4]


def resolve_qsc_bin(selected: str | None) -> str | None:
    if selected:
        return selected
    env_bin = os.environ.get("QSC_BIN")
    if env_bin:
        return env_bin
    root = repo_root()
    rel = root / "target/release/qsc"
    dbg = root / "target/debug/qsc"
    if rel.is_file():
        return str(rel)
    if dbg.is_file():
        return str(dbg)
    if shutil.which("qsc"):
        return "qsc"
    return None


def ensure_safe_state_root(args: argparse.Namespace) -> tuple[Path, bool]:
    auto_created = False
    if args.state_root:
        state_root = Path(args.state_root).expanduser()
    else:
        ts = dt.datetime.now(dt.UTC).strftime("%Y%m%dT%H%M%SZ")
        state_root = Path.home() / ".qsl" / "qsc-soak" / ts
        auto_created = True
    state_root = state_root.resolve()
    old_umask = os.umask(0o077)
    try:
        state_root.mkdir(parents=True, exist_ok=True)
    finally:
        os.umask(old_umask)
    os.chmod(state_root, 0o700)
    mode = state_root.stat().st_mode & 0o777
    if mode != 0o700:
        print(f"{MARKER_RESULT_FAIL} code=unsafe_state_root_perms", flush=True)
        sys.exit(2)
    parent_mode = state_root.parent.stat().st_mode & 0o777
    if parent_mode & 0o022:
        print(f"{MARKER_RESULT_FAIL} code=unsafe_state_root_perms", flush=True)
        sys.exit(2)
    print(MARKER_STATE_ROOT_OK, flush=True)
    return state_root, auto_created


def ensure_dir_700(path: Path) -> None:
    old_umask = os.umask(0o077)
    try:
        path.mkdir(parents=True, exist_ok=True)
    finally:
        os.umask(old_umask)
    os.chmod(path, 0o700)


def run_qsc(
    qsc_bin: str,
    cfg_dir: Path,
    relay_token: str,
    args: list[str],
    timeout_s: int = 30,
) -> tuple[bool, int | None, float, str]:
    env = os.environ.copy()
    env["QSC_CONFIG_DIR"] = str(cfg_dir)
    env["QSC_RELAY_TOKEN"] = relay_token
    env.setdefault("QSC_QSP_SEED", "1")
    env.setdefault("QSC_ALLOW_SEED_FALLBACK", "1")
    started = time.monotonic()
    try:
        proc = subprocess.run(
            [qsc_bin] + args,
            env=env,
            capture_output=True,
            text=True,
            timeout=timeout_s,
            check=False,
        )
        elapsed_ms = (time.monotonic() - started) * 1000.0
        merged = (proc.stdout or "") + (proc.stderr or "")
        for line in merged.splitlines():
            if "QSC_MARK/1" in line and " code=" in line:
                code = line.split(" code=", 1)[1].split()[0]
                if code not in ("", "ok"):
                    return False, proc.returncode, elapsed_ms, code
        return proc.returncode == 0, proc.returncode, elapsed_ms, ""
    except subprocess.TimeoutExpired:
        elapsed_ms = (time.monotonic() - started) * 1000.0
        return False, None, elapsed_ms, "timeout"


def percentile(data: list[float], pct: float) -> float:
    if not data:
        return 0.0
    if len(data) == 1:
        return data[0]
    rank = (len(data) - 1) * pct
    lo = int(rank)
    hi = min(lo + 1, len(data) - 1)
    frac = rank - lo
    return data[lo] * (1.0 - frac) + data[hi] * frac


def main() -> int:
    args = parse_args()
    must_validate_relay_url(args.relay_url)
    relay_token = os.environ.get("QSL_RELAY_TOKEN")
    workdir = Path(args.workdir)
    ensure_dir_700(workdir)
    pair_count = args.clients // 2
    state_root, state_root_auto = ensure_safe_state_root(args)
    if args.dry_run:
        print(
            f"QSC_SOAK_PLAN clients={args.clients} pairs={pair_count} "
            f"duration_secs={args.duration_secs} workdir={workdir}",
            flush=True,
        )
        print("QSC_SOAK_PLAN auth=env:QSL_RELAY_TOKEN", flush=True)
        print(MARKER_DRYRUN_OK, flush=True)
        print(f"{MARKER_RESULT_PASS} code=dry_run", flush=True)
        return 0

    if not relay_token:
        print(f"{MARKER_RESULT_FAIL} code=relay_token_missing", flush=True)
        return 2
    if args.clients < 2:
        print(f"{MARKER_RESULT_FAIL} code=clients_min_2", flush=True)
        return 2

    qsc_bin = resolve_qsc_bin(args.qsc_bin)
    if not qsc_bin:
        print(f"{MARKER_RESULT_FAIL} code=qsc_bin_not_found", flush=True)
        print("Hint: cargo build -p qsc --release --locked", flush=True)
        return 2

    clients_root = state_root
    clients_root.mkdir(parents=True, exist_ok=True)
    try:
        ok_count = 0
        timeout_count = 0
        reject_by_code: dict[str, int] = {}
        latencies_ms: list[float] = []
        started = time.monotonic()

        client_ids = [f"client-{i:03d}" for i in range(args.clients)]
        client_cfg: dict[str, Path] = {}
        client_route: dict[str, str] = {}
        old_umask = os.umask(0o077)
        try:
            for cid in client_ids:
                cfg = clients_root / cid
                cfg.mkdir(parents=True, exist_ok=True)
                os.chmod(cfg, 0o700)
                client_cfg[cid] = cfg
                client_route[cid] = route_token_for(cid)
        finally:
            os.umask(old_umask)

        for cid in client_ids:
            ok, _, ms, code = run_qsc(
                qsc_bin,
                client_cfg[cid],
                relay_token,
                ["vault", "init", "--non-interactive", "--key-source", "mock"],
                timeout_s=30,
            )
            latencies_ms.append(ms)
            if not ok:
                reject_by_code[code or "vault_init_failed"] = (
                    reject_by_code.get(code or "vault_init_failed", 0) + 1
                )
                if code == "timeout":
                    timeout_count += 1
                continue
            ok_count += 1
            ok, _, ms, code = run_qsc(
                qsc_bin,
                client_cfg[cid],
                relay_token,
                ["relay", "inbox-set", "--token", client_route[cid]],
                timeout_s=15,
            )
            latencies_ms.append(ms)
            if ok:
                ok_count += 1
            else:
                reject_by_code[code or "relay_inbox_set_failed"] = (
                    reject_by_code.get(code or "relay_inbox_set_failed", 0) + 1
                )
                if code == "timeout":
                    timeout_count += 1

        pairs = []
        for i in range(0, len(client_ids) - 1, 2):
            pairs.append((client_ids[i], client_ids[i + 1]))

        for a, b in pairs:
            for src, dst in ((a, b), (b, a)):
                ok, _, ms, code = run_qsc(
                    qsc_bin,
                    client_cfg[src],
                    relay_token,
                    [
                        "contacts",
                        "route-set",
                        "--label",
                        dst,
                        "--route-token",
                        client_route[dst],
                    ],
                    timeout_s=15,
                )
                latencies_ms.append(ms)
                if ok:
                    ok_count += 1
                else:
                    reject_by_code[code or "contacts_route_set_failed"] = (
                        reject_by_code.get(code or "contacts_route_set_failed", 0) + 1
                    )
                    if code == "timeout":
                        timeout_count += 1

        message_i = 0
        while time.monotonic() - started < float(args.duration_secs):
            if not pairs:
                break
            for a, b in pairs:
                for src, dst in ((a, b), (b, a)):
                    msg_file = workdir / f"msg_{message_i:06d}.txt"
                    message_i += 1
                    msg_file.write_text(f"na0165 soak message {src}->{dst}\n", encoding="utf-8")
                    ok, _, ms, code = run_qsc(
                        qsc_bin,
                        client_cfg[src],
                        relay_token,
                        [
                            "send",
                            "--transport",
                            "relay",
                            "--relay",
                            args.relay_url,
                            "--to",
                            dst,
                            "--file",
                            str(msg_file),
                        ],
                        timeout_s=30,
                    )
                    latencies_ms.append(ms)
                    if ok:
                        ok_count += 1
                    else:
                        reject_by_code[code or "send_failed"] = (
                            reject_by_code.get(code or "send_failed", 0) + 1
                        )
                        if code == "timeout":
                            timeout_count += 1

                    out_dir = workdir / "recv" / dst
                    ensure_dir_700(out_dir)
                    ok, _, ms, code = run_qsc(
                        qsc_bin,
                        client_cfg[dst],
                        relay_token,
                        [
                            "receive",
                            "--transport",
                            "relay",
                            "--relay",
                            args.relay_url,
                            "--from",
                            src,
                            "--max",
                            "4",
                            "--out",
                            str(out_dir),
                        ],
                        timeout_s=30,
                    )
                    latencies_ms.append(ms)
                    if ok:
                        ok_count += 1
                    else:
                        reject_by_code[code or "receive_failed"] = (
                            reject_by_code.get(code or "receive_failed", 0) + 1
                        )
                        if code == "timeout":
                            timeout_count += 1
                if time.monotonic() - started >= float(args.duration_secs):
                    break

        sorted_latency = sorted(latencies_ms)
        summary = {
            "clients": args.clients,
            "pairs": len(pairs),
            "duration_secs": args.duration_secs,
            "ok": ok_count,
            "rejects_total": int(sum(reject_by_code.values())),
            "rejects_by_code": reject_by_code,
            "timeouts": timeout_count,
            "latency_ms": {
                "count": len(sorted_latency),
                "p50": round(percentile(sorted_latency, 0.50), 2),
                "p95": round(percentile(sorted_latency, 0.95), 2),
                "max": round(max(sorted_latency) if sorted_latency else 0.0, 2),
                "mean": round(statistics.mean(sorted_latency), 2)
                if sorted_latency
                else 0.0,
            },
        }
        summary_path = workdir / "summary.json"
        summary_path.parent.mkdir(parents=True, exist_ok=True)
        summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True), encoding="utf-8")
        print(f"QSC_SOAK_SUMMARY path={summary_path}", flush=True)

        if reject_by_code:
            fail_codes = ",".join(sorted(reject_by_code.keys()))
            print(sanitize_line(f"{MARKER_RESULT_FAIL} code={fail_codes}", relay_token), flush=True)
            return 1
        print(f"{MARKER_RESULT_PASS} code=ok", flush=True)
        return 0
    finally:
        if state_root_auto and not args.keep_state:
            shutil.rmtree(state_root, ignore_errors=True)


if __name__ == "__main__":
    raise SystemExit(main())
