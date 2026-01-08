import struct


class QSPError(Exception):
    reason_code = "invalid_request"


def qsp_parse(b: bytes) -> dict:
    # QSP 4.3 ProtocolMessage Prefix per QSP 4.3.2 §7.1
    off = 0

    def need(n: int):
        if off + n > len(b):
            raise QSPError("truncated")

    need(2)
    protocol_version = struct.unpack_from(">H", b, off)[0]
    off += 2
    need(2)
    suite_id = struct.unpack_from(">H", b, off)[0]
    off += 2
    if protocol_version != 0x0403:
        raise QSPError("unknown protocol_version")
    if suite_id not in (0x0001, 0x0002):
        raise QSPError("unknown suite_id")

    need(16)
    session_id = b[off:off + 16]
    off += 16
    need(32)
    dh_pub = b[off:off + 32]
    off += 32
    need(2)
    flags = struct.unpack_from(">H", b, off)[0]
    off += 2
    need(12)
    nonce_hdr = b[off:off + 12]
    off += 12

    FLAG_PQ_ADV = 0x0001
    FLAG_PQ_CTXT = 0x0002
    FLAG_BOUNDARY = 0x0004
    known = FLAG_PQ_ADV | FLAG_PQ_CTXT | FLAG_BOUNDARY
    if flags & ~known:
        raise QSPError("unknown flags")
    if (flags & FLAG_PQ_ADV) and not (flags & FLAG_BOUNDARY):
        raise QSPError("PQ_ADV requires BOUNDARY")
    if (flags & FLAG_PQ_CTXT) and not (flags & FLAG_BOUNDARY):
        raise QSPError("PQ_CTXT requires BOUNDARY")

    if flags & FLAG_PQ_ADV:
        need(4)
        pq_adv_id = struct.unpack_from(">I", b, off)[0]
        off += 4
        need(1184)
        pq_adv_pub = b[off:off + 1184]
        off += 1184
    else:
        pq_adv_id = None
        pq_adv_pub = None

    if flags & FLAG_PQ_CTXT:
        need(4)
        pq_target_id = struct.unpack_from(">I", b, off)[0]
        off += 4
        need(1088)
        pq_ct = b[off:off + 1088]
        off += 1088
    else:
        pq_target_id = None
        pq_ct = None

    need(2)
    hdr_ct_len = struct.unpack_from(">H", b, off)[0]
    off += 2
    if hdr_ct_len != 24:
        raise QSPError("hdr_ct_len must be 24")
    need(hdr_ct_len)
    hdr_ct = b[off:off + hdr_ct_len]
    off += hdr_ct_len

    need(4)
    body_ct_len = struct.unpack_from(">I", b, off)[0]
    off += 4
    if body_ct_len < 16:
        raise QSPError("body_ct_len too small")
    need(body_ct_len)
    body_ct = b[off:off + body_ct_len]
    off += body_ct_len

    if off != len(b):
        raise QSPError("trailing bytes")

    return {
        "protocol_version": protocol_version,
        "suite_id": suite_id,
        "session_id": session_id,
        "dh_pub": dh_pub,
        "flags": flags,
        "nonce_hdr": nonce_hdr,
        "hdr_ct_len": hdr_ct_len,
        "body_ct_len": body_ct_len,
        "pq_adv_id": pq_adv_id,
        "pq_target_id": pq_target_id,
    }
