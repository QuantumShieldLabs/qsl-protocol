#!/usr/bin/env python3
import argparse
import json
import time
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import urlparse, parse_qs

# In-memory stores are acceptable for Phase 4B stubs-first.
RSF_QUEUE = {}  # route_token(str) -> [payload_b64u(str), ...]

def _json(handler: BaseHTTPRequestHandler, code: int, obj):
    body = json.dumps(obj, sort_keys=True, separators=(",", ":")).encode("utf-8")
    handler.send_response(code)
    handler.send_header("Content-Type", "application/json")
    handler.send_header("Content-Length", str(len(body)))
    handler.end_headers()
    handler.wfile.write(body)

class Handler(BaseHTTPRequestHandler):
    service = "rsf"

    def log_message(self, fmt, *args):
        # Keep logs minimal; JSON evidence is authoritative.
        return

    def do_GET(self):
        p = urlparse(self.path)
        if p.path == "/healthz":
            _json(self, 200, {"ok": True, "service": self.service, "ts": int(time.time())})
            return

        if self.service == "rsf" and p.path == "/v1/rsf/fetch":
            q = parse_qs(p.query)
            rt = (q.get("route_token") or [""])[0]
            if not rt:
                _json(self, 400, {"ok": False, "reason_code": "invalid_request"})
                return
            items = RSF_QUEUE.get(rt, [])
            if not items:
                _json(self, 200, {"ok": True, "items": []})
                return
            payload = items.pop(0)
            _json(self, 200, {"ok": True, "items": [{"payload_b64u": payload}]})
            return

        if self.service == "pds" and p.path.startswith("/v1/pds/bundle/"):
            # Stub bundle; real semantics are exercised by interop adapters, not by stubs.
            device = p.path.split("/")[-1]
            _json(self, 200, {"ok": True, "device": device, "bundle_stub": True})
            return

        if self.service == "ktl" and p.path == "/v1/ktl/sth":
            _json(self, 200, {"ok": True, "sth_stub": True})
            return

        _json(self, 404, {"ok": False, "reason_code": "not_found"})

    def do_POST(self):
        p = urlparse(self.path)
        n = int(self.headers.get("Content-Length", "0") or "0")
        raw = self.rfile.read(n) if n else b""
        try:
            obj = json.loads(raw.decode("utf-8") or "{}")
        except Exception:
            _json(self, 400, {"ok": False, "reason_code": "invalid_request"})
            return

        if self.service == "rsf" and p.path == "/v1/rsf/enqueue":
            rt = obj.get("route_token", "")
            payload = obj.get("payload_b64u", "")
            if not rt or not payload:
                _json(self, 400, {"ok": False, "reason_code": "invalid_request"})
                return
            RSF_QUEUE.setdefault(rt, []).append(payload)
            _json(self, 200, {"ok": True})
            return

        _json(self, 404, {"ok": False, "reason_code": "not_found"})

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--service", required=True, choices=["rsf", "pds", "ktl"])
    ap.add_argument("--port", required=True, type=int)
    args = ap.parse_args()

    Handler.service = args.service
    httpd = HTTPServer(("0.0.0.0", args.port), Handler)
    httpd.serve_forever()

if __name__ == "__main__":
    main()
