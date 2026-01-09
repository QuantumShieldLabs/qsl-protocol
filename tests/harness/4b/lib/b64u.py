import base64
import re

_B64U_RE = re.compile(r"^[A-Za-z0-9_-]*$")


def b64u_decode_strict(s: str) -> bytes:
    # P3-23: no padding allowed; reject "=" and non-base64url chars.
    if "=" in s:
        raise ValueError("padding not allowed")
    if not _B64U_RE.match(s):
        raise ValueError("invalid base64url alphabet")
    if len(s) % 4 == 1:
        raise ValueError("invalid base64url length")
    pad = "=" * ((4 - (len(s) % 4)) % 4)
    return base64.urlsafe_b64decode((s + pad).encode("ascii"))
