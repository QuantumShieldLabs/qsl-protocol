import os
import threading
import time
import unittest
from http.server import BaseHTTPRequestHandler, HTTPServer

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from lib import relay_http


class _RelayState:
    def __init__(self, max_body=1024):
        self.max_body = max_body
        self.queues = {}


class RelayHandler(BaseHTTPRequestHandler):
    server_version = "qsl-relay-test"

    def do_POST(self):
        if not self.path.startswith("/v1/push/"):
            self.send_response(404)
            self.end_headers()
            return
        length = int(self.headers.get("Content-Length", "0"))
        body = self.rfile.read(length)
        if len(body) == 0:
            self.send_response(400)
            self.end_headers()
            self.wfile.write(b"ERR_EMPTY_BODY")
            return
        if len(body) > self.server.state.max_body:
            self.send_response(413)
            self.end_headers()
            self.wfile.write(b"ERR_TOO_LARGE")
            return
        channel = self.path.split("/v1/push/")[-1]
        self.server.state.queues.setdefault(channel, []).append(body)
        self.send_response(200)
        self.end_headers()

    def do_GET(self):
        if not self.path.startswith("/v1/pull/"):
            self.send_response(404)
            self.end_headers()
            return
        channel = self.path.split("/v1/pull/")[-1]
        q = self.server.state.queues.setdefault(channel, [])
        if not q:
            self.send_response(204)
            self.end_headers()
            return
        data = q.pop(0)
        self.send_response(200)
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, format, *args):
        # Silence test output; no payload logging.
        pass


class RelayHttpAdapterTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        state = _RelayState(max_body=4)
        httpd = HTTPServer(("127.0.0.1", 0), RelayHandler)
        httpd.state = state
        cls.httpd = httpd
        cls.addr = httpd.server_address
        cls.thread = threading.Thread(target=httpd.serve_forever, daemon=True)
        cls.thread.start()
        os.environ["QSL_RELAY_BASE_URL"] = f"http://{cls.addr[0]}:{cls.addr[1]}"
        os.environ["QSL_RELAY_CHANNEL"] = "testchan"
        os.environ["QSL_RELAY_TIMEOUT_SECS"] = "1"
        os.environ["QSL_RELAY_MAX_POLL_SECS"] = "1"

    @classmethod
    def tearDownClass(cls):
        cls.httpd.shutdown()
        cls.thread.join(timeout=2)

    def test_push_pull_roundtrip(self):
        ok, err = relay_http.push(b"hi")
        self.assertTrue(ok)
        self.assertIsNone(err)

        data, err = relay_http.pull()
        self.assertEqual(data, b"hi")
        self.assertIsNone(err)

        data, err = relay_http.pull()
        self.assertIsNone(data)
        self.assertEqual(err, "ERR_RELAY_TIMEOUT")

    def test_empty_body_rejects_deterministically(self):
        ok, err = relay_http.push(b"")
        self.assertFalse(ok)
        self.assertIsNotNone(err)
        self.assertTrue(err.startswith("ERR_RELAY_HTTP_400_ERR_EMPTY_BODY"))

    def test_directional_channels_prevent_self_receive(self):
        ok, err = relay_http.push(b"ok", side="a")
        self.assertTrue(ok)
        self.assertIsNone(err)

        data, err = relay_http.pull(side="a")
        self.assertEqual(data, b"ok")
        self.assertIsNone(err)

        data, err = relay_http.pull(side="b")
        self.assertIsNone(data)
        self.assertEqual(err, "ERR_RELAY_TIMEOUT")


if __name__ == "__main__":
    unittest.main()
