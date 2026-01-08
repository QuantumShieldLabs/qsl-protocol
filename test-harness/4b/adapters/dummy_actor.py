#!/usr/bin/env python3
import json, sys, time

def main():
    inp = json.loads(sys.stdin.read() or "{}")
    out = {
        "ok": False,
        "failure_stage": "adapter",
        "notes": "dummy actor: no implementation wired (expected fail-closed)",
        "echo": {
            "case_id": inp.get("case_id"),
            "suite_id": inp.get("suite_id"),
            "roles": inp.get("roles"),
            "actor": inp.get("actor"),
        },
        "ts": int(time.time()),
    }
    sys.stdout.write(json.dumps(out, sort_keys=True) + "\n")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
