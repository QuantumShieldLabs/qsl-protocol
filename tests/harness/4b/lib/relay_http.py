import os
import time
import urllib.error
import urllib.request
from typing import Optional, Tuple


def _env_int(name: str, default: int) -> int:
    try:
        return int(os.getenv(name, str(default)))
    except Exception:
        return default


def cfg() -> Tuple[str, str, int, int]:
    base = os.getenv("QSL_RELAY_BASE_URL", "http://127.0.0.1:8080").rstrip("/")
    channel = os.getenv("QSL_RELAY_CHANNEL", "demo")
    timeout = _env_int("QSL_RELAY_TIMEOUT_SECS", 5)
    max_poll = _env_int("QSL_RELAY_MAX_POLL_SECS", 10)
    return base, channel, timeout, max_poll


def _err_token(status: int, body: bytes) -> str:
    text = body.decode("utf-8", errors="ignore").strip()
    if text:
        return f"ERR_RELAY_HTTP_{status}_{text}"
    return f"ERR_RELAY_HTTP_{status}"


def _remote_allowed(base: str) -> bool:
    if base.startswith("http://127.0.0.1") or base.startswith("http://localhost") or base.startswith("http://[::1]"):
        return True
    return os.getenv("QSL_ALLOW_REMOTE") == "1"


def push(raw: bytes) -> Tuple[bool, Optional[str]]:
    base, channel, timeout, _ = cfg()
    if not _remote_allowed(base):
        return False, "ERR_RELAY_REMOTE_DISABLED"
    url = f"{base}/v1/push/{channel}"
    req = urllib.request.Request(url, data=raw, method="POST")
    req.add_header("Content-Type", "application/octet-stream")
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            status = resp.status
            body = resp.read()
            if status == 200:
                return True, None
            return False, _err_token(status, body)
    except urllib.error.HTTPError as e:
        body = e.read() if e.fp else b""
        return False, _err_token(e.code, body)
    except urllib.error.URLError:
        return False, "ERR_RELAY_CONNECT"
    except TimeoutError:
        return False, "ERR_RELAY_TIMEOUT"


def pull() -> Tuple[Optional[bytes], Optional[str]]:
    base, channel, timeout, max_poll = cfg()
    if not _remote_allowed(base):
        return None, "ERR_RELAY_REMOTE_DISABLED"
    url = f"{base}/v1/pull/{channel}"
    deadline = time.time() + max_poll

    while True:
        try:
            with urllib.request.urlopen(url, timeout=timeout) as resp:
                status = resp.status
                body = resp.read()
                if status == 200:
                    return body, None
                if status == 204:
                    if time.time() >= deadline:
                        return None, "ERR_RELAY_TIMEOUT"
                    time.sleep(0.25)
                    continue
                return None, _err_token(status, body)
        except urllib.error.HTTPError as e:
            body = e.read() if e.fp else b""
            return None, _err_token(e.code, body)
        except urllib.error.URLError:
            return None, "ERR_RELAY_CONNECT"
        except TimeoutError:
            return None, "ERR_RELAY_TIMEOUT"
