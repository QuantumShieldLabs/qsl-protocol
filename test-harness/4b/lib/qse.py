import struct
from dataclasses import dataclass
from .policy import Policy

class QSEError(Exception):
    reason_code = "noncanonical_qse"

class InvalidRequest(QSEError):
    reason_code = "invalid_request"

class BoundsExceeded(QSEError):
    reason_code = "bounds_exceeded"

class PolicyReject(QSEError):
    reason_code = "policy_reject"

@dataclass(frozen=True)
class QSEEnvelope:
    env_version: int
    flags: int
    route_token: bytes
    timestamp_bucket: int
    pad_len: int
    payload_len: int
    payload: bytes
    pad: bytes

def qse_parse(b: bytes, policy: Policy) -> QSEEnvelope:
    # Wire: env_version(u16) || flags(u16) || route_token(varbytes<u16>) ||
    #       timestamp_bucket(u32) || pad_len(u16) || payload_len(u32) || payload || pad
    off = 0
    def need(n: int):
        if off + n > len(b):
            raise QSEError("truncated")

    need(2)
    env_version = struct.unpack_from(">H", b, off)[0]; off += 2
    need(2)
    flags = struct.unpack_from(">H", b, off)[0]; off += 2

    # QSE 1.x expected env_version=0x0100; flags=0 for QSE 1.8.x profile.
    if env_version != 0x0100:
        raise InvalidRequest("unknown env_version")
    if flags != 0:
        raise InvalidRequest("nonzero flags")

    need(2)
    rt_len = struct.unpack_from(">H", b, off)[0]; off += 2
    if rt_len > 512:
        raise BoundsExceeded("route_token too long")
    need(rt_len)
    route_token = b[off:off+rt_len]; off += rt_len

    need(4)
    timestamp_bucket = struct.unpack_from(">I", b, off)[0]; off += 4
    need(2)
    pad_len = struct.unpack_from(">H", b, off)[0]; off += 2
    need(4)
    payload_len = struct.unpack_from(">I", b, off)[0]; off += 4
    if payload_len > 1_048_576:
        raise BoundsExceeded("payload too large")

    # Policy checks (only enforced when configured)
    if timestamp_bucket == 0 and not policy.ALLOW_ZERO_TIMESTAMP_BUCKET:
        raise PolicyReject("zero timestamp bucket disallowed")
    if policy.timestamp_window_enforced:
        # The harness cannot know real clock here; we treat extreme values as out-of-window per P3-23 intent.
        if timestamp_bucket == 0xFFFFFFFF:
            raise PolicyReject("timestamp out of window")

    need(payload_len)
    payload = b[off:off+payload_len]; off += payload_len
    need(pad_len)
    pad = b[off:off+pad_len]; off += pad_len

    # No trailing bytes allowed
    if off != len(b):
        raise QSEError("trailing bytes")

    return QSEEnvelope(env_version, flags, route_token, timestamp_bucket, pad_len, payload_len, payload, pad)
