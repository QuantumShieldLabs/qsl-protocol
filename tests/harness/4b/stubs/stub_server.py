#!/usr/bin/env python3
import argparse
import json
from http.server import BaseHTTPRequestHandler, HTTPServer

class Handler(BaseHTTPRequestHandler):
    server_version = "QShieldStub/1.0"

    def _send(self, code:int, body:dict):
        data = json.dumps(body, separators=(",", ":")).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        if self.path in ("/healthz", "/health", "/"):
            self._send(200, {"ok": True, "service": self.server.service_name})
            return
        self._send(404, {"ok": False, "error": {"code": "NOT_FOUND", "path": self.path}})

    def do_POST(self):
        # Fail-closed stub (expanded later when service semantics are required).
        self._send(501, {"ok": False, "error": {"code": "NOT_IMPLEMENTED", "service": self.server.service_name, "path": self.path}})

    def log_message(self, fmt, *args):
        return

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--service", required=True, choices=["rsf","pds","ktl"])
    ap.add_argument("--port", type=int, required=True)
    args = ap.parse_args()

    httpd = HTTPServer(("127.0.0.1", args.port), Handler)
    httpd.service_name = args.service
    httpd.serve_forever()

if __name__ == "__main__":
    main()
