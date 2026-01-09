import struct

class KTError(Exception):
    reason_code = "kt_fail"

def kt_parse_sth(b: bytes) -> None:
    # P3-23: exact length, no trailing bytes
    if len(b) != 3453:
        raise KTError("bad sth length")

def kt_parse_inclusion_proof(b: bytes) -> None:
    if len(b) < 2:
        raise KTError("truncated")
    count = struct.unpack_from(">H", b, 0)[0]
    if count > 64:
        raise KTError("count too large")
    expect_len = 2 + 32 * count + 8
    if len(b) != expect_len:
        raise KTError("bad inclusion proof length")

def kt_parse_consistency_proof(b: bytes) -> None:
    if len(b) < 2:
        raise KTError("truncated")
    count = struct.unpack_from(">H", b, 0)[0]
    if count > 64:
        raise KTError("count too large")
    expect_len = 2 + 32 * count
    if len(b) != expect_len:
        raise KTError("bad consistency proof length")
