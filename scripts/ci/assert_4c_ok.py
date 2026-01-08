#!/usr/bin/env python3
import json
import os
import sys
from glob import glob


def die(msg: str, code: int = 1) -> None:
    print(msg, file=sys.stderr)
    raise SystemExit(code)


def latest_run_dir() -> str:
    runs = sorted(glob("artifacts/*"), key=lambda p: os.path.getmtime(p), reverse=True)
    if not runs:
        die("No artifacts/* run directories found.")
    return runs[0]


def read_json(path: str) -> dict:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def require_ok(obj: dict, label: str) -> None:
    ok = obj.get("ok")
    if ok is not True:
        die(f"{label} not ok: ok={ok}. See file for details.")


def main() -> int:
    run = latest_run_dir()
    base = os.path.join(run, "4C")
    required = {
        "B0_preflight.json": "preflight",
        "C1_interop_extended.json": "interop_extended",
    }
    for fn, label in required.items():
        path = os.path.join(base, fn)
        if not os.path.exists(path):
            die(f"Missing required 4C output: {path}")
        obj = read_json(path)
        require_ok(obj, label)
    print(f"4C OK: {run}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
