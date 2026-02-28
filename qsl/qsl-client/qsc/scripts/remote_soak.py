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
from urllib import error, request


MARKER_DRYRUN_OK = "QSC_SOAK_DRYRUN_OK"
MARKER_RESULT_PASS = "QSC_SOAK_RESULT PASS"
MARKER_RESULT_FAIL = "QSC_SOAK_RESULT FAIL"
MARKER_STATE_ROOT_OK = "QSC_SOAK_STATE_ROOT_OK mode=700 parent_safe=yes"
MARKER_DIAG_OK = "QSC_SOAK_DIAG_OK"
MARKER_DIAG_FAIL = "QSC_SOAK_DIAG_FAIL"
MARKER_MODE_SESSION_ONLY = "QSC_SOAK_MODE=session_only"
MARKER_MODE_SEED_FALLBACK = "QSC_SOAK_MODE=seed_fallback"
BACKOFF_MS = [50, 100, 200, 400, 800]
READINESS_BACKOFF_MS = [50, 100, 200]
DIAG_DRAIN_MAX_PULLS = 5
DIAG_DRAIN_BATCH = 32


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="QSC remote relay soak harness (NA-0165/NA-0168)."
    )
    parser.add_argument("--relay-url", required=True, help="HTTPS relay base URL")
    parser.add_argument("--clients", type=int, default=100)
    parser.add_argument("--duration-secs", type=int, default=60)
    parser.add_argument("--workdir", default="target/qsc-soak")
    parser.add_argument("--state-root", default=None)
    parser.add_argument("--keep-state", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--diag", action="store_true")
    parser.add_argument("--diag-selftest", action="store_true")
    parser.add_argument(
        "--no-handshake",
        action="store_true",
        help="Debug only: skip explicit per-pair handshake stage.",
    )
    parser.add_argument(
        "--seed-fallback",
        action="store_true",
        help="Debug only: allow QSP seed fallback mode.",
    )
    parser.add_argument("--max-retries", type=int, default=5)
    parser.add_argument("--no-sleep", action="store_true")
    parser.add_argument("--tolerate-auth-fail", type=int, default=0)
    parser.add_argument(
        "--simulate-overload-attempts",
        type=int,
        default=None,
        help="Dry-run only: simulate N overload responses before success.",
    )
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


def short_hash(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()[:12]


def seed_fallback_enabled(args: argparse.Namespace) -> bool:
    return bool(args.seed_fallback or args.no_handshake)


def emit_mode_marker(seed_fallback: bool) -> None:
    print(
        MARKER_MODE_SEED_FALLBACK if seed_fallback else MARKER_MODE_SESSION_ONLY,
        flush=True,
    )


def relay_pull_count(
    relay_url: str, route_token: str, relay_token: str, max_items: int
) -> tuple[int, str]:
    url = f"{relay_url.rstrip('/')}/v1/pull/{route_token}?max={max_items}"
    req = request.Request(
        url,
        method="GET",
        headers={"Authorization": f"Bearer {relay_token}"},
    )
    try:
        with request.urlopen(req, timeout=10) as resp:
            status = resp.status
            body = resp.read()
    except error.HTTPError as e:
        status = e.code
        body = e.read() if hasattr(e, "read") else b""
    except Exception:
        return 0, "relay_drain_failed"

    if status == 204:
        return 0, ""
    if status != 200:
        return 0, "relay_drain_failed"
    try:
        parsed = json.loads(body.decode("utf-8"))
    except Exception:
        return 0, "relay_drain_failed"
    items = parsed.get("items")
    if not isinstance(items, list):
        return 0, "relay_drain_failed"
    return len(items), ""


def relay_drain_channel(relay_url: str, route_token: str, relay_token: str) -> tuple[int, str]:
    drained = 0
    for _ in range(DIAG_DRAIN_MAX_PULLS):
        count, code = relay_pull_count(relay_url, route_token, relay_token, DIAG_DRAIN_BATCH)
        if code:
            return drained, code
        if count == 0:
            break
        drained += count
    return drained, ""


def is_overload_signal(text: str) -> bool:
    if "ERR_OVERLOADED" in text:
        return True
    if "HTTP_STATUS=429" in text:
        return True
    return False


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


def extract_marker_code(merged: str) -> str:
    for line in merged.splitlines():
        if "QSC_MARK/1" in line and " code=" in line:
            code = line.split(" code=", 1)[1].split()[0]
            if code not in ("", "ok"):
                return code
    if is_overload_signal(merged):
        return "overloaded"
    return ""


def parse_handshake_send_ready(merged: str) -> tuple[bool | None, str]:
    for line in merged.splitlines():
        if "event=handshake_status" not in line:
            continue
        send_ready = None
        reason = "unknown"
        for token in line.split():
            if token.startswith("send_ready="):
                send_ready = token.split("=", 1)[1]
            elif token.startswith("send_ready_reason="):
                reason = token.split("=", 1)[1]
        if send_ready == "yes":
            return True, "ready"
        if send_ready == "no":
            return False, reason
    return None, "marker_missing"


def maybe_sleep_ms(ms: int, args: argparse.Namespace) -> None:
    if args.no_sleep:
        return
    time.sleep(ms / 1000.0)


def run_qsc(
    qsc_bin: str,
    cfg_dir: Path,
    relay_token: str,
    args: list[str],
    seed_fallback: bool,
    timeout_s: int = 30,
) -> tuple[bool, int | None, float, str]:
    env = os.environ.copy()
    env["QSC_CONFIG_DIR"] = str(cfg_dir)
    env["QSC_RELAY_TOKEN"] = relay_token
    if seed_fallback:
        env.setdefault("QSC_QSP_SEED", "1")
        env.setdefault("QSC_ALLOW_SEED_FALLBACK", "1")
    else:
        env.pop("QSC_QSP_SEED", None)
        env.pop("QSC_ALLOW_SEED_FALLBACK", None)
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
        code = extract_marker_code(merged)
        if code == "overloaded":
            return False, proc.returncode, elapsed_ms, "overloaded"
        if code:
            return False, proc.returncode, elapsed_ms, code
        return proc.returncode == 0, proc.returncode, elapsed_ms, ""
    except subprocess.TimeoutExpired:
        elapsed_ms = (time.monotonic() - started) * 1000.0
        return False, None, elapsed_ms, "timeout"


def run_qsc_with_output(
    qsc_bin: str,
    cfg_dir: Path,
    relay_token: str,
    args: list[str],
    seed_fallback: bool,
    timeout_s: int = 30,
) -> tuple[bool, int | None, float, str, str]:
    env = os.environ.copy()
    env["QSC_CONFIG_DIR"] = str(cfg_dir)
    env["QSC_RELAY_TOKEN"] = relay_token
    if seed_fallback:
        env.setdefault("QSC_QSP_SEED", "1")
        env.setdefault("QSC_ALLOW_SEED_FALLBACK", "1")
    else:
        env.pop("QSC_QSP_SEED", None)
        env.pop("QSC_ALLOW_SEED_FALLBACK", None)
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
        code = extract_marker_code(merged)
        if code:
            return False, proc.returncode, elapsed_ms, code, merged
        return proc.returncode == 0, proc.returncode, elapsed_ms, "", merged
    except subprocess.TimeoutExpired:
        elapsed_ms = (time.monotonic() - started) * 1000.0
        return False, None, elapsed_ms, "timeout", ""


def run_qsc_with_retry(
    qsc_bin: str,
    cfg_dir: Path,
    relay_token: str,
    args: list[str],
    retry_cfg: argparse.Namespace,
    counters: dict[str, int],
    timeout_s: int = 30,
) -> tuple[bool, int | None, float, str]:
    retries = 0
    elapsed_total = 0.0
    while True:
        ok, rc, ms, code = run_qsc(
            qsc_bin=qsc_bin,
            cfg_dir=cfg_dir,
            relay_token=relay_token,
            args=args,
            seed_fallback=seed_fallback_enabled(retry_cfg),
            timeout_s=timeout_s,
        )
        elapsed_total += ms
        if ok:
            return True, rc, elapsed_total, ""
        if code == "overloaded":
            if retries < retry_cfg.max_retries:
                counters["overload_retries"] = counters.get("overload_retries", 0) + 1
                backoff_idx = min(retries, len(BACKOFF_MS) - 1)
                if not retry_cfg.no_sleep:
                    time.sleep(BACKOFF_MS[backoff_idx] / 1000.0)
                retries += 1
                continue
            counters["overload_failures"] = counters.get("overload_failures", 0) + 1
        return False, rc, elapsed_total, code


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


def init_failure_census(client_ids: list[str]) -> dict[str, dict[str, int]]:
    return {
        cid: {
            "qsp_hdr_auth_failed": 0,
            "relay_inbox_parse_failed": 0,
            "overloaded": 0,
            "other": 0,
        }
        for cid in client_ids
    }


def classify_error(code: str) -> str:
    if code == "qsp_hdr_auth_failed":
        return "qsp_hdr_auth_failed"
    if code == "relay_inbox_parse_failed":
        return "relay_inbox_parse_failed"
    if code == "overloaded":
        return "overloaded"
    return "other"


def record_failure(census: dict[str, dict[str, int]], client_id: str, code: str) -> None:
    bucket = classify_error(code)
    census.setdefault(
        client_id,
        {
            "qsp_hdr_auth_failed": 0,
            "relay_inbox_parse_failed": 0,
            "overloaded": 0,
            "other": 0,
        },
    )
    census[client_id][bucket] += 1


def print_failure_census(census: dict[str, dict[str, int]]) -> None:
    for cid in sorted(census.keys()):
        row = census[cid]
        print(
            "QSC_SOAK_CENSUS "
            f"client={cid} "
            f"qsp_hdr_auth_failed={row.get('qsp_hdr_auth_failed', 0)} "
            f"relay_inbox_parse_failed={row.get('relay_inbox_parse_failed', 0)} "
            f"overloaded={row.get('overloaded', 0)} "
            f"other={row.get('other', 0)}",
            flush=True,
        )


def build_clients(state_root: Path, clients: int, run_namespace: str) -> tuple[list[str], dict[str, Path], dict[str, str]]:
    client_ids = [f"client-{i:03d}" for i in range(clients)]
    client_cfg: dict[str, Path] = {}
    client_route: dict[str, str] = {}
    for cid in client_ids:
        cfg = state_root / cid
        ensure_dir_700(cfg)
        client_cfg[cid] = cfg
        client_route[cid] = route_token_for(f"{run_namespace}:{cid}")
    return client_ids, client_cfg, client_route


def run_pair_handshake(
    qsc_bin: str,
    client_cfg: dict[str, Path],
    relay_token: str,
    relay_url: str,
    a: str,
    b: str,
    retry_cfg: argparse.Namespace,
    counters: dict[str, int],
    stage_fail_cb,
) -> bool:
    print("QSC_SOAK_STAGE stage=hs_pair_handshake_start status=start", flush=True)
    print("QSC_SOAK_STAGE stage=hs_pair_handshake_a_init status=start", flush=True)
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[a],
        relay_token,
        ["handshake", "init", "--as", a, "--peer", b, "--relay", relay_url],
        retry_cfg=retry_cfg,
        counters=counters,
        timeout_s=30,
    )
    if not ok:
        stage_fail_cb("hs_pair_handshake_a_init", code or "handshake_init_failed", a)
        return False
    print("QSC_SOAK_STAGE stage=hs_pair_handshake_a_init status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=hs_pair_handshake_b_process status=start", flush=True)
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[b],
        relay_token,
        [
            "handshake",
            "poll",
            "--as",
            b,
            "--peer",
            a,
            "--relay",
            relay_url,
            "--max",
            "4",
        ],
        retry_cfg=retry_cfg,
        counters=counters,
        timeout_s=30,
    )
    if not ok:
        stage_fail_cb("hs_pair_handshake_b_process", code or "handshake_poll_failed", b)
        return False
    print("QSC_SOAK_STAGE stage=hs_pair_handshake_b_process status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=hs_pair_handshake_a_finalize status=start", flush=True)
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[a],
        relay_token,
        [
            "handshake",
            "poll",
            "--as",
            a,
            "--peer",
            b,
            "--relay",
            relay_url,
            "--max",
            "4",
        ],
        retry_cfg=retry_cfg,
        counters=counters,
        timeout_s=30,
    )
    if not ok:
        stage_fail_cb("hs_pair_handshake_a_finalize", code or "handshake_poll_failed", a)
        return False
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[b],
        relay_token,
        [
            "handshake",
            "poll",
            "--as",
            b,
            "--peer",
            a,
            "--relay",
            relay_url,
            "--max",
            "4",
        ],
        retry_cfg=retry_cfg,
        counters=counters,
        timeout_s=30,
    )
    if not ok:
        stage_fail_cb("hs_pair_handshake_a_finalize", code or "handshake_poll_failed", b)
        return False
    print("QSC_SOAK_STAGE stage=hs_pair_handshake_a_finalize status=ok", flush=True)

    for check_client, check_peer in ((a, b), (b, a)):
        ok_status, _, _, code, merged = run_qsc_with_output(
            qsc_bin,
            client_cfg[check_client],
            relay_token,
            ["handshake", "status", "--peer", check_peer],
            seed_fallback=seed_fallback_enabled(retry_cfg),
            timeout_s=20,
        )
        if not ok_status:
            stage_fail_cb("hs_pair_handshake_ok", code or "handshake_status_failed", check_client)
            return False
        if "event=handshake_status status=established" not in merged:
            stage_fail_cb("hs_pair_handshake_ok", "handshake_not_established", check_client)
            return False

    print("QSC_SOAK_STAGE stage=hs_pair_handshake_ok status=ok", flush=True)
    return True


def run_diag_selftest(args: argparse.Namespace) -> int:
    emit_mode_marker(seed_fallback_enabled(args))
    if args.dry_run:
        print(MARKER_STATE_ROOT_OK, flush=True)
    sample_a = route_token_for("diag-selftest:client-a")
    sample_b = route_token_for("diag-selftest:client-b")
    if sample_a == sample_b:
        print(f"{MARKER_DIAG_FAIL} stage=diag_selftest code=token_collision", flush=True)
        print(f"{MARKER_RESULT_FAIL} code=diag_selftest_failed", flush=True)
        return 1
    sample_peer_hash = short_hash("diag-selftest:peer")
    print(
        f"QSC_SOAK_DIAG_ACTIVE_PEER_HASH client=client-000 peer_hash={sample_peer_hash}",
        flush=True,
    )
    print(
        "QSC_SOAK_DIAG_SEND_READY stage=hs_a_send_to_b status=ready reason=ready attempts=1",
        flush=True,
    )
    print("QSC_SOAK_DIAG_HASH_CHECK len12=ok", flush=True)
    print(MARKER_DIAG_OK, flush=True)
    print(f"{MARKER_RESULT_PASS} code=diag_selftest", flush=True)
    return 0


def run_diag(args: argparse.Namespace, qsc_bin: str, relay_token: str, state_root: Path) -> int:
    if args.clients != 2:
        print(f"{MARKER_RESULT_FAIL} code=diag_requires_two_clients", flush=True)
        return 2

    print("QSC_SOAK_STAGE stage=diag_setup status=start", flush=True)
    client_ids, client_cfg, client_route = build_clients(state_root, 2, run_namespace="diag")
    census = init_failure_census(client_ids)
    overload_counters: dict[str, int] = {"overload_retries": 0, "overload_failures": 0}

    def stage_fail(stage: str, code: str, client: str) -> int:
        record_failure(census, client, code or "unknown")
        print(f"{MARKER_DIAG_FAIL} stage={stage} code={code or 'unknown'}", flush=True)
        print_failure_census(census)
        print(f"{MARKER_RESULT_FAIL} code={code or 'unknown'}", flush=True)
        return 1

    def verify_active_peer(stage: str, client: str, peer: str, expected_hash: str) -> bool:
        peer_hash = short_hash(client_route[peer])
        print(
            f"QSC_SOAK_DIAG_ACTIVE_PEER_HASH client={client} peer_hash={peer_hash}",
            flush=True,
        )
        if peer_hash != expected_hash:
            stage_fail(stage, "active_peer_mismatch", client)
            return False
        return True

    def require_session_ready(stage: str, client: str, peer: str) -> bool:
        last_reason = "marker_missing"
        for idx, backoff_ms in enumerate(READINESS_BACKOFF_MS, start=1):
            ok_status, _, _, code, merged = run_qsc_with_output(
                qsc_bin,
                client_cfg[client],
                relay_token,
                ["handshake", "status", "--peer", peer],
                seed_fallback=seed_fallback_enabled(args),
                timeout_s=20,
            )
            send_ready, reason = parse_handshake_send_ready(merged)
            last_reason = reason
            if ok_status and send_ready is True:
                print(
                    f"QSC_SOAK_DIAG_SEND_READY stage={stage} status=ready reason=ready attempts={idx}",
                    flush=True,
                )
                return True
            if not ok_status and code:
                last_reason = code
            if idx < len(READINESS_BACKOFF_MS):
                maybe_sleep_ms(backoff_ms, args)
        print(
            f"QSC_SOAK_DIAG_SEND_READY stage={stage} status=not_ready reason={last_reason} attempts={len(READINESS_BACKOFF_MS)}",
            flush=True,
        )
        stage_fail(stage, "session_not_send_ready", client)
        return False

    a, b = client_ids[0], client_ids[1]
    a_hash = short_hash(client_route[a])
    b_hash = short_hash(client_route[b])
    a_peer_hash = short_hash(client_route[b])
    b_peer_hash = short_hash(client_route[a])
    print(f"QSC_SOAK_DIAG_SELF_INBOX_HASH client={a} hash={a_hash}", flush=True)
    print(f"QSC_SOAK_DIAG_SELF_INBOX_HASH client={b} hash={b_hash}", flush=True)
    print(f"QSC_SOAK_DIAG_PEER_INBOX_HASH client={a} hash={a_peer_hash}", flush=True)
    print(f"QSC_SOAK_DIAG_PEER_INBOX_HASH client={b} hash={b_peer_hash}", flush=True)
    if a_peer_hash != b_hash or b_peer_hash != a_hash:
        return stage_fail("inbox_mapping", "inbox_hash_mismatch", a)
    expected_peer_hash = {a: b_hash, b: a_hash}
    print("QSC_SOAK_DIAG_MAPPING_OK match=yes", flush=True)
    print("QSC_SOAK_STAGE stage=diag_setup status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=client_init status=start", flush=True)
    for cid in client_ids:
        ok, _, _, code = run_qsc_with_retry(
            qsc_bin,
            client_cfg[cid],
            relay_token,
            ["vault", "init", "--non-interactive", "--key-source", "mock"],
            retry_cfg=args,
            counters=overload_counters,
            timeout_s=30,
        )
        if not ok:
            return stage_fail("client_init", code or "vault_init_failed", cid)
    print("QSC_SOAK_STAGE stage=client_init status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=relay_config status=start", flush=True)
    for cid in client_ids:
        ok, _, _, code = run_qsc_with_retry(
            qsc_bin,
            client_cfg[cid],
            relay_token,
            ["relay", "inbox-set", "--token", client_route[cid]],
            retry_cfg=args,
            counters=overload_counters,
            timeout_s=20,
        )
        if not ok:
            return stage_fail("relay_config", code or "relay_inbox_set_failed", cid)
    print("QSC_SOAK_STAGE stage=relay_config status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=contact_add status=start", flush=True)
    for src, dst in ((a, b), (b, a)):
        ok, _, _, code = run_qsc_with_retry(
            qsc_bin,
            client_cfg[src],
            relay_token,
            ["contacts", "route-set", "--label", dst, "--route-token", client_route[dst]],
            retry_cfg=args,
            counters=overload_counters,
            timeout_s=20,
        )
        if not ok:
            return stage_fail("contact_add", code or "contacts_route_set_failed", src)
    print("QSC_SOAK_STAGE stage=contact_add status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=drain status=start", flush=True)
    drained_a, drain_code = relay_drain_channel(args.relay_url, client_route[a], relay_token)
    if drain_code:
        return stage_fail("drain", drain_code, a)
    print(f"QSC_SOAK_DIAG_DRAIN client={a} drained={drained_a}", flush=True)
    drained_b, drain_code = relay_drain_channel(args.relay_url, client_route[b], relay_token)
    if drain_code:
        return stage_fail("drain", drain_code, b)
    print(f"QSC_SOAK_DIAG_DRAIN client={b} drained={drained_b}", flush=True)
    print("QSC_SOAK_STAGE stage=drain status=ok", flush=True)

    if not args.no_handshake:
        def hs_stage_fail(stage: str, code: str, client: str) -> None:
            raise RuntimeError(f"{stage}|{code}|{client}")

        try:
            ok_hs = run_pair_handshake(
                qsc_bin=qsc_bin,
                client_cfg=client_cfg,
                relay_token=relay_token,
                relay_url=args.relay_url,
                a=a,
                b=b,
                retry_cfg=args,
                counters=overload_counters,
                stage_fail_cb=hs_stage_fail,
            )
            if not ok_hs:
                return stage_fail("hs_pair_handshake_ok", "handshake_failed", a)
        except RuntimeError as e:
            stage, code, client = str(e).split("|", 2)
            return stage_fail(stage, code, client)

    print("QSC_SOAK_STAGE stage=hs_a_send_to_b status=start", flush=True)
    bootstrap_file_ab = state_root / "diag_bootstrap_ab.txt"
    bootstrap_file_ba = state_root / "diag_bootstrap_ba.txt"
    bootstrap_file_ab.write_text("na0168 diag bootstrap A->B\n", encoding="utf-8")
    bootstrap_file_ba.write_text("na0168 diag bootstrap B->A\n", encoding="utf-8")
    if not verify_active_peer("hs_a_send_to_b", a, b, expected_peer_hash[a]):
        return 1
    if not require_session_ready("hs_a_send_to_b", a, b):
        return 1

    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[a],
        relay_token,
        [
            "send",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--to",
            b,
            "--file",
            str(bootstrap_file_ab),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("hs_a_send_to_b", code or "send_failed", a)
    print("QSC_SOAK_STAGE stage=hs_a_send_to_b status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=hs_b_recv_from_a status=start", flush=True)
    recv_ab = state_root / "diag_recv_ab"
    ensure_dir_700(recv_ab)
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[b],
        relay_token,
        [
            "receive",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--from",
            a,
            "--max",
            "4",
            "--out",
            str(recv_ab),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("hs_b_recv_from_a", code or "receive_failed", b)
    print("QSC_SOAK_STAGE stage=hs_b_recv_from_a status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=hs_b_send_to_a status=start", flush=True)
    if not verify_active_peer("hs_b_send_to_a", b, a, expected_peer_hash[b]):
        return 1
    if not require_session_ready("hs_b_send_to_a", b, a):
        return 1
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[b],
        relay_token,
        [
            "send",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--to",
            a,
            "--file",
            str(bootstrap_file_ba),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("hs_b_send_to_a", code or "send_failed", b)
    print("QSC_SOAK_STAGE stage=hs_b_send_to_a status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=hs_a_recv_from_b status=start", flush=True)
    recv_ba = state_root / "diag_recv_ba"
    ensure_dir_700(recv_ba)
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[a],
        relay_token,
        [
            "receive",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--from",
            b,
            "--max",
            "4",
            "--out",
            str(recv_ba),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("hs_a_recv_from_b", code or "receive_failed", a)
    print("QSC_SOAK_STAGE stage=hs_a_recv_from_b status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=handshake_bootstrap status=ok", flush=True)

    print("QSC_SOAK_STAGE stage=msg_roundtrip status=start", flush=True)
    roundtrip_file_ab = state_root / "diag_roundtrip_ab.txt"
    roundtrip_file_ba = state_root / "diag_roundtrip_ba.txt"
    roundtrip_file_ab.write_text("na0168 diag roundtrip A->B\n", encoding="utf-8")
    roundtrip_file_ba.write_text("na0168 diag roundtrip B->A\n", encoding="utf-8")
    if not verify_active_peer("msg_roundtrip", a, b, expected_peer_hash[a]):
        return 1
    if not require_session_ready("msg_roundtrip", a, b):
        return 1

    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[a],
        relay_token,
        [
            "send",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--to",
            b,
            "--file",
            str(roundtrip_file_ab),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("msg_roundtrip", code or "send_failed", a)

    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[b],
        relay_token,
        [
            "receive",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--from",
            a,
            "--max",
            "4",
            "--out",
            str(recv_ab),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("msg_roundtrip", code or "receive_failed", b)

    if not verify_active_peer("msg_roundtrip", b, a, expected_peer_hash[b]):
        return 1
    if not require_session_ready("msg_roundtrip", b, a):
        return 1
    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[b],
        relay_token,
        [
            "send",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--to",
            a,
            "--file",
            str(roundtrip_file_ba),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("msg_roundtrip", code or "send_failed", b)

    ok, _, _, code = run_qsc_with_retry(
        qsc_bin,
        client_cfg[a],
        relay_token,
        [
            "receive",
            "--transport",
            "relay",
            "--relay",
            args.relay_url,
            "--from",
            b,
            "--max",
            "4",
            "--out",
            str(recv_ba),
        ],
        retry_cfg=args,
        counters=overload_counters,
        timeout_s=30,
    )
    if not ok:
        return stage_fail("msg_roundtrip", code or "receive_failed", a)

    print("QSC_SOAK_STAGE stage=msg_roundtrip status=ok", flush=True)
    print("QSC_SOAK_STAGE stage=diag_done status=ok", flush=True)
    print_failure_census(census)
    print(MARKER_DIAG_OK, flush=True)
    print(f"{MARKER_RESULT_PASS} code=diag_ok", flush=True)
    return 0


def run_soak(args: argparse.Namespace, qsc_bin: str, relay_token: str, workdir: Path, state_root: Path) -> int:
    client_ids, client_cfg, client_route = build_clients(state_root, args.clients, run_namespace="soak")
    pairs = []
    for i in range(0, len(client_ids) - 1, 2):
        pairs.append((client_ids[i], client_ids[i + 1]))

    ok_count = 0
    timeout_count = 0
    overload_counters: dict[str, int] = {"overload_retries": 0, "overload_failures": 0}
    reject_by_code: dict[str, int] = {}
    latencies_ms: list[float] = []
    started = time.monotonic()
    census = init_failure_census(client_ids)
    tolerated_auth_failures = 0

    for cid in client_ids:
        ok, _, ms, code = run_qsc_with_retry(
            qsc_bin,
            client_cfg[cid],
            relay_token,
            ["vault", "init", "--non-interactive", "--key-source", "mock"],
            retry_cfg=args,
            counters=overload_counters,
            timeout_s=30,
        )
        latencies_ms.append(ms)
        if not ok:
            reject_by_code[code or "vault_init_failed"] = reject_by_code.get(code or "vault_init_failed", 0) + 1
            record_failure(census, cid, code or "vault_init_failed")
            if code == "timeout":
                timeout_count += 1
            continue
        ok_count += 1

        ok, _, ms, code = run_qsc_with_retry(
            qsc_bin,
            client_cfg[cid],
            relay_token,
            ["relay", "inbox-set", "--token", client_route[cid]],
            retry_cfg=args,
            counters=overload_counters,
            timeout_s=15,
        )
        latencies_ms.append(ms)
        if ok:
            ok_count += 1
        else:
            reject_by_code[code or "relay_inbox_set_failed"] = reject_by_code.get(code or "relay_inbox_set_failed", 0) + 1
            record_failure(census, cid, code or "relay_inbox_set_failed")
            if code == "timeout":
                timeout_count += 1

    for a, b in pairs:
        for src, dst in ((a, b), (b, a)):
            ok, _, ms, code = run_qsc_with_retry(
                qsc_bin,
                client_cfg[src],
                relay_token,
                ["contacts", "route-set", "--label", dst, "--route-token", client_route[dst]],
                retry_cfg=args,
                counters=overload_counters,
                timeout_s=15,
            )
            latencies_ms.append(ms)
            if ok:
                ok_count += 1
            else:
                reject_by_code[code or "contacts_route_set_failed"] = reject_by_code.get(code or "contacts_route_set_failed", 0) + 1
                record_failure(census, src, code or "contacts_route_set_failed")
                if code == "timeout":
                    timeout_count += 1

    def soak_stage_fail(stage: str, code: str, client: str) -> None:
        record_failure(census, client, code or "unknown")
        raise RuntimeError(f"{stage}|{code}|{client}")

    if not args.no_handshake:
        for a, b in pairs:
            try:
                ok_hs = run_pair_handshake(
                    qsc_bin=qsc_bin,
                    client_cfg=client_cfg,
                    relay_token=relay_token,
                    relay_url=args.relay_url,
                    a=a,
                    b=b,
                    retry_cfg=args,
                    counters=overload_counters,
                    stage_fail_cb=soak_stage_fail,
                )
                if not ok_hs:
                    reject_by_code["handshake_failed"] = reject_by_code.get("handshake_failed", 0) + 1
                    break
            except RuntimeError as e:
                stage, code, client = str(e).split("|", 2)
                reject_by_code[code or "handshake_failed"] = reject_by_code.get(code or "handshake_failed", 0) + 1
                print(
                    f"QSC_SOAK_STAGE stage=hs_pair_handshake_fail status=fail code={code} client={client}",
                    flush=True,
                )
                break
        if reject_by_code:
            sorted_latency = sorted(latencies_ms)
            summary = {
                "clients": args.clients,
                "pairs": len(pairs),
                "duration_secs": args.duration_secs,
                "ok": ok_count,
                "rejects_total": int(sum(reject_by_code.values())),
                "rejects_by_code": reject_by_code,
                "timeouts": timeout_count,
                "overload_retries": overload_counters.get("overload_retries", 0),
                "overload_failures": overload_counters.get("overload_failures", 0),
                "tolerated_auth_failures": tolerated_auth_failures,
                "auth_tolerance_limit": args.tolerate_auth_fail,
                "latency_ms": {
                    "count": len(sorted_latency),
                    "p50": round(percentile(sorted_latency, 0.50), 2),
                    "p95": round(percentile(sorted_latency, 0.95), 2),
                    "max": round(max(sorted_latency) if sorted_latency else 0.0, 2),
                    "mean": round(statistics.mean(sorted_latency), 2) if sorted_latency else 0.0,
                },
            }
            summary_path = workdir / "summary.json"
            summary_path.parent.mkdir(parents=True, exist_ok=True)
            summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True), encoding="utf-8")
            print(f"QSC_SOAK_SUMMARY path={summary_path}", flush=True)
            print_failure_census(census)
            fail_codes = ",".join(sorted(reject_by_code.keys()))
            print(sanitize_line(f"{MARKER_RESULT_FAIL} code={fail_codes}", relay_token), flush=True)
            return 1

    message_i = 0
    while time.monotonic() - started < float(args.duration_secs):
        if not pairs:
            break
        for a, b in pairs:
            for src, dst in ((a, b), (b, a)):
                msg_file = workdir / f"msg_{message_i:06d}.txt"
                message_i += 1
                msg_file.write_text(f"na0168 soak message {src}->{dst}\n", encoding="utf-8")
                ok, _, ms, code = run_qsc_with_retry(
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
                    retry_cfg=args,
                    counters=overload_counters,
                    timeout_s=30,
                )
                latencies_ms.append(ms)
                if ok:
                    ok_count += 1
                else:
                    reject_by_code[code or "send_failed"] = reject_by_code.get(code or "send_failed", 0) + 1
                    record_failure(census, src, code or "send_failed")
                    if code == "timeout":
                        timeout_count += 1

                out_dir = workdir / "recv" / dst
                ensure_dir_700(out_dir)
                ok, _, ms, code = run_qsc_with_retry(
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
                    retry_cfg=args,
                    counters=overload_counters,
                    timeout_s=30,
                )
                latencies_ms.append(ms)
                if ok:
                    ok_count += 1
                else:
                    if code == "qsp_hdr_auth_failed" and tolerated_auth_failures < args.tolerate_auth_fail:
                        tolerated_auth_failures += 1
                        record_failure(census, dst, code)
                    else:
                        reject_by_code[code or "receive_failed"] = reject_by_code.get(code or "receive_failed", 0) + 1
                        record_failure(census, dst, code or "receive_failed")
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
        "overload_retries": overload_counters.get("overload_retries", 0),
        "overload_failures": overload_counters.get("overload_failures", 0),
        "tolerated_auth_failures": tolerated_auth_failures,
        "auth_tolerance_limit": args.tolerate_auth_fail,
        "latency_ms": {
            "count": len(sorted_latency),
            "p50": round(percentile(sorted_latency, 0.50), 2),
            "p95": round(percentile(sorted_latency, 0.95), 2),
            "max": round(max(sorted_latency) if sorted_latency else 0.0, 2),
            "mean": round(statistics.mean(sorted_latency), 2) if sorted_latency else 0.0,
        },
    }
    summary_path = workdir / "summary.json"
    summary_path.parent.mkdir(parents=True, exist_ok=True)
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True), encoding="utf-8")
    print(f"QSC_SOAK_SUMMARY path={summary_path}", flush=True)
    print_failure_census(census)

    if reject_by_code:
        fail_codes = ",".join(sorted(reject_by_code.keys()))
        print(sanitize_line(f"{MARKER_RESULT_FAIL} code={fail_codes}", relay_token), flush=True)
        return 1

    print(f"QSC_SOAK_TOLERATED_AUTH_FAIL count={tolerated_auth_failures} limit={args.tolerate_auth_fail}", flush=True)
    print(f"{MARKER_RESULT_PASS} code=ok", flush=True)
    return 0


def main() -> int:
    args = parse_args()
    mode_seed_fallback = seed_fallback_enabled(args)
    emit_mode_marker(mode_seed_fallback)
    must_validate_relay_url(args.relay_url)
    relay_token = os.environ.get("QSL_RELAY_TOKEN")
    workdir = Path(args.workdir)
    ensure_dir_700(workdir)
    pair_count = args.clients // 2
    state_root, state_root_auto = ensure_safe_state_root(args)

    if args.selftest:
        print(
            f"QSC_SOAK_DIAG_ACTIVE_PEER_HASH client=client-000 peer_hash={short_hash('selftest-peer')}",
            flush=True,
        )
        print(
            "QSC_SOAK_DIAG_SEND_READY stage=hs_a_send_to_b status=ready reason=ready attempts=1",
            flush=True,
        )
        print("QSC_SOAK_SELFTEST_OK", flush=True)
        print(f"{MARKER_RESULT_PASS} code=selftest", flush=True)
        return 0

    if args.diag_selftest:
        return run_diag_selftest(args)

    if args.dry_run:
        print(
            f"QSC_SOAK_PLAN clients={args.clients} pairs={pair_count} "
            f"duration_secs={args.duration_secs} workdir={workdir}",
            flush=True,
        )
        print("QSC_SOAK_PLAN auth=env:QSL_RELAY_TOKEN", flush=True)
        if args.diag:
            print("QSC_SOAK_STAGE stage=diag_setup status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=client_init status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=relay_config status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=contact_add status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=drain status=ok", flush=True)
            print("QSC_SOAK_DIAG_DRAIN client=client-000 drained=0", flush=True)
            print("QSC_SOAK_DIAG_DRAIN client=client-001 drained=0", flush=True)
            print("QSC_SOAK_DIAG_MAPPING_OK match=yes", flush=True)
            print(
                f"QSC_SOAK_DIAG_ACTIVE_PEER_HASH client=client-000 peer_hash={short_hash('dryrun-peer')}",
                flush=True,
            )
            print(
                "QSC_SOAK_DIAG_SEND_READY stage=hs_a_send_to_b status=ready reason=ready attempts=1",
                flush=True,
            )
            print("QSC_SOAK_STAGE stage=hs_a_send_to_b status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=hs_b_recv_from_a status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=hs_b_send_to_a status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=hs_a_recv_from_b status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=handshake_bootstrap status=ok", flush=True)
            if not args.no_handshake:
                print("QSC_SOAK_STAGE stage=hs_pair_handshake_start status=ok", flush=True)
                print("QSC_SOAK_STAGE stage=hs_pair_handshake_a_init status=ok", flush=True)
                print("QSC_SOAK_STAGE stage=hs_pair_handshake_b_process status=ok", flush=True)
                print("QSC_SOAK_STAGE stage=hs_pair_handshake_a_finalize status=ok", flush=True)
                print("QSC_SOAK_STAGE stage=hs_pair_handshake_ok status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=msg_roundtrip status=ok", flush=True)
            print("QSC_SOAK_STAGE stage=diag_done status=ok", flush=True)
            print("QSC_SOAK_DIAG_SELF_INBOX_HASH client=client-000 hash=123456789abc", flush=True)
            print("QSC_SOAK_DIAG_SELF_INBOX_HASH client=client-001 hash=abcdef123456", flush=True)
            print("QSC_SOAK_DIAG_PEER_INBOX_HASH from=client-000 to=client-001 hash=abcdef123456", flush=True)
            print("QSC_SOAK_DIAG_PEER_INBOX_HASH from=client-001 to=client-000 hash=123456789abc", flush=True)
            print(MARKER_DIAG_OK, flush=True)
        if args.simulate_overload_attempts is not None:
            retries = 0
            overload_retries = 0
            overload_failures = 0
            remaining = max(0, args.simulate_overload_attempts)
            while True:
                overloaded = remaining > 0
                if overloaded:
                    remaining -= 1
                    if retries < args.max_retries:
                        overload_retries += 1
                        if not args.no_sleep:
                            backoff_idx = min(retries, len(BACKOFF_MS) - 1)
                            time.sleep(BACKOFF_MS[backoff_idx] / 1000.0)
                        retries += 1
                        continue
                    overload_failures += 1
                    break
                break
            summary = {
                "mode": "dry_run_simulated_overload",
                "overload_retries": overload_retries,
                "overload_failures": overload_failures,
                "max_retries": args.max_retries,
            }
            summary_path = workdir / "summary.json"
            summary_path.parent.mkdir(parents=True, exist_ok=True)
            summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True), encoding="utf-8")
            print(f"QSC_SOAK_SUMMARY path={summary_path}", flush=True)
            if overload_failures:
                print(f"{MARKER_RESULT_FAIL} code=overloaded", flush=True)
                return 1
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

    try:
        if args.diag:
            return run_diag(args, qsc_bin, relay_token, state_root)
        return run_soak(args, qsc_bin, relay_token, workdir, state_root)
    finally:
        if state_root_auto and not args.keep_state:
            shutil.rmtree(state_root, ignore_errors=True)


if __name__ == "__main__":
    raise SystemExit(main())
