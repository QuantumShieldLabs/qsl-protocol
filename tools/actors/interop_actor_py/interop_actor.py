#!/usr/bin/env python3
# Minimal independent actor for Suite-2 KDF/transcript/mk ops.
# STDLIB ONLY. Fail-closed parsing. JSONL over stdin/stdout.
from __future__ import annotations
import base64
import binascii
import hashlib
import hmac
import json
import os
import struct
import sys
from typing import Any, Dict, List, Optional, Tuple

def _err(msg: str) -> Dict[str, Any]:
    return {"ok": False, "error": {"code": "INVALID", "message": msg}}

def _ok(result: Any) -> Dict[str, Any]:
    return {"ok": True, "result": result}

def _read_lines() -> List[str]:
    return sys.stdin.read().splitlines()

def _write(obj: Dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(obj, separators=(",", ":")) + "\n")
    sys.stdout.flush()

def _hex(b: bytes) -> str:
    return binascii.hexlify(b).decode("ascii")

def _b64u_encode(b: bytes) -> str:
    return base64.urlsafe_b64encode(b).rstrip(b"=").decode("ascii")

def _b64u_decode(s: str) -> bytes:
    pad = "=" * (-len(s) % 4)
    return base64.urlsafe_b64decode(s + pad)

def _unhex(s: str) -> bytes:
    try:
        return binascii.unhexlify(s)
    except Exception as e:
        raise ValueError(f"bad hex: {e}")

def _get(params: Dict[str, Any], k: str) -> Any:
    if k not in params:
        raise ValueError(f"missing params.{k}")
    return params[k]

def _unwrap_json(v: Any) -> Any:
    if isinstance(v, dict) and v.get("type") == "json":
        return v.get("data")
    return v

def _parse_bytes(v: Any, where: str) -> bytes:
    # Accept shapes used by existing vectors/actor: typed {"type":"hex","data":"..."},
    # {"hex":"..."}, {"b64":"..."}, {"bytes":[...]} or raw hex string.
    if isinstance(v, str):
        # assume hex string
        return _unhex(v)
    if isinstance(v, dict):
        if v.get("type") == "hex" and isinstance(v.get("data"), str):
            return _unhex(v["data"])
        if v.get("type") == "json" and isinstance(v.get("data"), dict):
            # Some vectors wrap bytes inside json typed object
            inner = v.get("data")
            if isinstance(inner, dict) and "hex" in inner:
                return _unhex(str(inner["hex"]))
        if "hex" in v and isinstance(v["hex"], str):
            return _unhex(v["hex"])
        if "b64" in v and isinstance(v["b64"], str):
            try:
                return base64.urlsafe_b64decode(v["b64"] + "==")
            except Exception as e:
                raise ValueError(f"{where}: bad b64: {e}")
        if "bytes" in v and isinstance(v["bytes"], list):
            return bytes(int(x) & 0xFF for x in v["bytes"])
    raise ValueError(f"{where}: unsupported bytes encoding")

SESSIONS: Dict[bytes, Dict[str, Any]] = {}

def _parse_u16(v: Any, where: str) -> int:
    if isinstance(v, int):
        n = v
    elif isinstance(v, str):
        s = v.strip()
        if s.lower().startswith("0x"):
            try:
                n = int(s, 16)
            except Exception as e:
                raise ValueError(f"{where}: bad hex u16: {e}")
        else:
            try:
                n = int(s, 10)
            except Exception as e:
                raise ValueError(f"{where}: bad u16 string: {e}")
    elif isinstance(v, dict):
        if v.get("type") == "json" and isinstance(v.get("data"), dict):
            v = v.get("data")
        if "u16" in v and isinstance(v["u16"], int):
            n = v["u16"]
        elif "value" in v and isinstance(v["value"], int):
            n = v["value"]
        else:
            raise ValueError(f"{where}: unsupported u16 object")
    else:
        raise ValueError(f"{where}: unsupported u16 type")
    if not (0 <= n <= 0xFFFF):
        raise ValueError(f"{where}: u16 out of range")
    return n

def _parse_u32(v: Any, where: str) -> int:
    if isinstance(v, int):
        n = v
    elif isinstance(v, str):
        s = v.strip()
        if s.lower().startswith("0x"):
            try:
                n = int(s, 16)
            except Exception as e:
                raise ValueError(f"{where}: bad hex u32: {e}")
        else:
            try:
                n = int(s, 10)
            except Exception as e:
                raise ValueError(f"{where}: bad u32 string: {e}")
    elif isinstance(v, dict):
        if v.get("type") == "json" and isinstance(v.get("data"), dict):
            v = v.get("data")
        if "u32" in v and isinstance(v["u32"], int):
            n = v["u32"]
        elif "value" in v and isinstance(v["value"], int):
            n = v["value"]
        else:
            raise ValueError(f"{where}: unsupported u32 object")
    else:
        raise ValueError(f"{where}: unsupported u32 type")
    if not (0 <= n <= 0xFFFFFFFF):
        raise ValueError(f"{where}: u32 out of range")
    return n

def _sha512(b: bytes) -> bytes:
    return hashlib.sha512(b).digest()

def _left_encode(x: int) -> bytes:
    if x < 0:
        raise ValueError("left_encode: negative")
    n = 1
    tmp = x
    while tmp > 0xFF:
        n += 1
        tmp >>= 8
    return bytes([n]) + x.to_bytes(n, "big")

def _right_encode(x: int) -> bytes:
    if x < 0:
        raise ValueError("right_encode: negative")
    n = 1
    tmp = x
    while tmp > 0xFF:
        n += 1
        tmp >>= 8
    return x.to_bytes(n, "big") + bytes([n])

def _encode_string(s: bytes) -> bytes:
    return _left_encode(len(s) * 8) + s

def _bytepad(x: bytes, w: int) -> bytes:
    if w <= 0:
        raise ValueError("bytepad: invalid width")
    z = _left_encode(w) + x
    pad_len = (-len(z)) % w
    return z + b"\x00" * pad_len

# Keccak-f[1600] implementation (little-endian lanes)
_RC = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
]
_R = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14],
]

def _rotl64(x: int, n: int) -> int:
    return ((x << n) | (x >> (64 - n))) & 0xFFFFFFFFFFFFFFFF

def _keccak_f1600(state: List[int]) -> None:
    for rnd in range(24):
        # Theta
        c = [state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20] for x in range(5)]
        d = [c[(x - 1) % 5] ^ _rotl64(c[(x + 1) % 5], 1) for x in range(5)]
        for x in range(5):
            for y in range(5):
                state[x + 5 * y] ^= d[x]
        # Rho + Pi
        b = [0] * 25
        for x in range(5):
            for y in range(5):
                b[y + 5 * ((2 * x + 3 * y) % 5)] = _rotl64(state[x + 5 * y], _R[x][y])
        # Chi
        for x in range(5):
            for y in range(5):
                state[x + 5 * y] = b[x + 5 * y] ^ ((~b[((x + 1) % 5) + 5 * y]) & b[((x + 2) % 5) + 5 * y])
        # Iota
        state[0] ^= _RC[rnd]

def _keccak_sponge(rate: int, suffix: int, msg: bytes, outlen: int) -> bytes:
    if rate <= 0 or rate > 200:
        raise ValueError("invalid rate")
    state = [0] * 25
    rate_words = rate // 8
    off = 0
    while off + rate <= len(msg):
        block = msg[off:off + rate]
        for i in range(rate_words):
            state[i] ^= int.from_bytes(block[8 * i:8 * i + 8], "little")
        _keccak_f1600(state)
        off += rate
    block = bytearray(msg[off:])
    block.append(suffix)
    if len(block) > rate:
        block = block[:rate]
    while len(block) < rate:
        block.append(0)
    block[-1] ^= 0x80
    for i in range(rate_words):
        state[i] ^= int.from_bytes(block[8 * i:8 * i + 8], "little")
    _keccak_f1600(state)
    out = bytearray()
    while len(out) < outlen:
        for i in range(rate_words):
            out += state[i].to_bytes(8, "little")
        if len(out) >= outlen:
            break
        _keccak_f1600(state)
    return bytes(out[:outlen])

def _cshake256(x: bytes, outlen_bytes: int, n: bytes, s: bytes) -> bytes:
    rate = 136
    if not n and not s:
        # SHAKE256 domain separation
        return _keccak_sponge(rate, 0x1F, x, outlen_bytes)
    prefix = _bytepad(_encode_string(n) + _encode_string(s), rate)
    return _keccak_sponge(rate, 0x04, prefix + x, outlen_bytes)

def _kmac256(key: bytes, data: bytes, outlen_bytes: int, custom: bytes) -> bytes:
    rate = 136
    new_x = _bytepad(_encode_string(key), rate) + data + _right_encode(outlen_bytes * 8)
    return _cshake256(new_x, outlen_bytes, b"KMAC", custom)

_SBOX = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
]

_RCON = [0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36]

def _aes_sub_word(w: int) -> int:
    return (
        (_SBOX[(w >> 24) & 0xFF] << 24) |
        (_SBOX[(w >> 16) & 0xFF] << 16) |
        (_SBOX[(w >> 8) & 0xFF] << 8) |
        (_SBOX[w & 0xFF])
    )

def _aes_rot_word(w: int) -> int:
    return ((w << 8) & 0xFFFFFFFF) | ((w >> 24) & 0xFF)

def _aes_key_expand(key: bytes) -> List[int]:
    if len(key) != 32:
        raise ValueError("AES-256 key must be 32 bytes")
    nk = 8
    nb = 4
    nr = 14
    w = [0] * (nb * (nr + 1))
    for i in range(nk):
        w[i] = int.from_bytes(key[4 * i:4 * i + 4], "big")
    for i in range(nk, nb * (nr + 1)):
        temp = w[i - 1]
        if i % nk == 0:
            temp = _aes_sub_word(_aes_rot_word(temp)) ^ (_RCON[i // nk] << 24)
        elif i % nk == 4:
            temp = _aes_sub_word(temp)
        w[i] = w[i - nk] ^ temp
    return w

def _aes_add_round_key(state: List[int], w: List[int], rnd: int) -> None:
    for c in range(4):
        k = w[rnd * 4 + c]
        state[4 * c + 0] ^= (k >> 24) & 0xFF
        state[4 * c + 1] ^= (k >> 16) & 0xFF
        state[4 * c + 2] ^= (k >> 8) & 0xFF
        state[4 * c + 3] ^= k & 0xFF

def _aes_sub_bytes(state: List[int]) -> None:
    for i in range(16):
        state[i] = _SBOX[state[i]]

def _aes_shift_rows(state: List[int]) -> None:
    # state is column-major
    state[1], state[5], state[9], state[13] = state[5], state[9], state[13], state[1]
    state[2], state[6], state[10], state[14] = state[10], state[14], state[2], state[6]
    state[3], state[7], state[11], state[15] = state[15], state[3], state[7], state[11]

def _xtime(a: int) -> int:
    return ((a << 1) & 0xFF) ^ (0x1B if a & 0x80 else 0x00)

def _aes_mix_columns(state: List[int]) -> None:
    for c in range(4):
        i = 4 * c
        a0, a1, a2, a3 = state[i:i+4]
        t = a0 ^ a1 ^ a2 ^ a3
        state[i+0] ^= t ^ _xtime(a0 ^ a1)
        state[i+1] ^= t ^ _xtime(a1 ^ a2)
        state[i+2] ^= t ^ _xtime(a2 ^ a3)
        state[i+3] ^= t ^ _xtime(a3 ^ a0)

def _aes_encrypt_block(key: bytes, block: bytes) -> bytes:
    w = _aes_key_expand(key)
    state = list(block)
    _aes_add_round_key(state, w, 0)
    for rnd in range(1, 14):
        _aes_sub_bytes(state)
        _aes_shift_rows(state)
        _aes_mix_columns(state)
        _aes_add_round_key(state, w, rnd)
    _aes_sub_bytes(state)
    _aes_shift_rows(state)
    _aes_add_round_key(state, w, 14)
    return bytes(state)

def _gf_mul(x: int, y: int) -> int:
    z = 0
    v = x
    r = 0xE1000000000000000000000000000000
    for i in range(128):
        if (y >> (127 - i)) & 1:
            z ^= v
        if v & 1:
            v = (v >> 1) ^ r
        else:
            v >>= 1
    return z

def _ghash(h: bytes, aad: bytes, ct: bytes) -> bytes:
    h_int = int.from_bytes(h, "big")
    y = 0
    def _blocks(data: bytes) -> List[bytes]:
        out = []
        for i in range(0, len(data), 16):
            block = data[i:i+16]
            if len(block) < 16:
                block = block + b"\x00" * (16 - len(block))
            out.append(block)
        return out
    for block in _blocks(aad):
        y = _gf_mul(y ^ int.from_bytes(block, "big"), h_int)
    for block in _blocks(ct):
        y = _gf_mul(y ^ int.from_bytes(block, "big"), h_int)
    aad_bits = (len(aad) * 8) & ((1 << 64) - 1)
    ct_bits = (len(ct) * 8) & ((1 << 64) - 1)
    len_block = aad_bits.to_bytes(8, "big") + ct_bits.to_bytes(8, "big")
    y = _gf_mul(y ^ int.from_bytes(len_block, "big"), h_int)
    return y.to_bytes(16, "big")

def _inc32(counter: bytes) -> bytes:
    prefix = counter[:12]
    ctr = int.from_bytes(counter[12:16], "big")
    ctr = (ctr + 1) & 0xFFFFFFFF
    return prefix + ctr.to_bytes(4, "big")

def _gcm_seal(key: bytes, nonce: bytes, aad: bytes, pt: bytes) -> bytes:
    if len(key) != 32 or len(nonce) != 12:
        raise ValueError("bad key/nonce length")
    h = _aes_encrypt_block(key, b"\x00" * 16)
    j0 = nonce + b"\x00\x00\x00\x01"
    # CTR encrypt
    ctr = _inc32(j0)
    ct = bytearray()
    for i in range(0, len(pt), 16):
        block = pt[i:i+16]
        stream = _aes_encrypt_block(key, ctr)
        ctr = _inc32(ctr)
        ct_block = bytes(b ^ s for b, s in zip(block, stream[:len(block)]))
        ct.extend(ct_block)
    s = _ghash(h, aad, bytes(ct))
    tag = bytes(a ^ b for a, b in zip(_aes_encrypt_block(key, j0), s))
    return bytes(ct) + tag

def _gcm_open(key: bytes, nonce: bytes, aad: bytes, ct_and_tag: bytes) -> bytes:
    if len(ct_and_tag) < 16:
        raise ValueError("bad ciphertext length")
    ct = ct_and_tag[:-16]
    tag = ct_and_tag[-16:]
    h = _aes_encrypt_block(key, b"\x00" * 16)
    j0 = nonce + b"\x00\x00\x00\x01"
    s = _ghash(h, aad, ct)
    expected = bytes(a ^ b for a, b in zip(_aes_encrypt_block(key, j0), s))
    if not hmac.compare_digest(tag, expected):
        raise ValueError("auth fail")
    ctr = _inc32(j0)
    pt = bytearray()
    for i in range(0, len(ct), 16):
        block = ct[i:i+16]
        stream = _aes_encrypt_block(key, ctr)
        ctr = _inc32(ctr)
        pt_block = bytes(b ^ s for b, s in zip(block, stream[:len(block)]))
        pt.extend(pt_block)
    return bytes(pt)

def _nonce_hdr(session_id: bytes, dh_pub: bytes, n: int) -> bytes:
    m = b"QSP5.0/HDR-NONCE" + session_id + dh_pub + struct.pack(">I", n)
    return _sha512(m)[:12]

def _nonce_body(session_id: bytes, dh_pub: bytes, n: int) -> bytes:
    m = b"QSP5.0/BODY-NONCE" + session_id + dh_pub + struct.pack(">I", n)
    return _sha512(m)[:12]

def _derive_mk_step(ck_ec: bytes, ck_pq: bytes) -> Tuple[bytes, bytes, bytes]:
    ck_ec_p = _kmac256(ck_ec, b"\x01", 32, b"QSP5.0/CK")
    ec_mk = _kmac256(ck_ec, b"\x02", 32, b"QSP5.0/MK")
    ck_pq_p = _kmac256(ck_pq, b"\x01", 32, b"QSP5.0/PQCK")
    pq_mk = _kmac256(ck_pq, b"\x02", 32, b"QSP5.0/PQMK")
    mk = _kmac256(ec_mk, pq_mk + b"\x01", 32, b"QSP5.0/HYBRID")
    return ck_ec_p, ck_pq_p, mk

def _init_from_handshake(
    role: str,
    session_id: bytes,
    dh_init: bytes,
    pq_init_ss: bytes,
    dh_self_pub: bytes,
    dh_peer_pub: bytes,
) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    rk0 = _kmac256(dh_init, session_id + b"\x01", 32, b"QSP5.0/RK0")
    rk = _kmac256(rk0, pq_init_ss + b"\x01", 32, b"QSP5.0/RKPQ")
    hk_a2b = _kmac256(rk, b"\x01", 32, b"QSP5.0/HK/A->B")
    hk_b2a = _kmac256(rk, b"\x01", 32, b"QSP5.0/HK/B->A")
    ck0_a2b = _kmac256(rk, b"\x01", 32, b"QSP5.0/CK0/A->B")
    pq0_a2b = _kmac256(rk, b"\x01", 32, b"QSP5.0/PQ0/A->B")
    zero32 = b"\x00" * 32

    if role == "A":
        send = {
            "session_id": session_id,
            "dh_pub": dh_self_pub,
            "hk_s": hk_a2b,
            "ck_ec": ck0_a2b,
            "ck_pq": pq0_a2b,
            "ns": 0,
            "pn": 0,
        }
        recv = {
            "session_id": session_id,
            "dh_pub": dh_peer_pub,
            "hk_r": hk_b2a,
            "rk": rk,
            "ck_ec": zero32,
            "ck_pq_send": pq0_a2b,
            "ck_pq_recv": zero32,
            "nr": 0,
            "role": "A",
            "peer_max_adv_id_seen": 0,
            "known_targets": [],
            "consumed_targets": [],
            "tombstoned_targets": [],
            "mkskipped": [],
        }
    else:
        send = {
            "session_id": session_id,
            "dh_pub": dh_self_pub,
            "hk_s": hk_b2a,
            "ck_ec": zero32,
            "ck_pq": zero32,
            "ns": 0,
            "pn": 0,
        }
        recv = {
            "session_id": session_id,
            "dh_pub": dh_peer_pub,
            "hk_r": hk_a2b,
            "rk": rk,
            "ck_ec": ck0_a2b,
            "ck_pq_send": zero32,
            "ck_pq_recv": pq0_a2b,
            "nr": 0,
            "role": "B",
            "peer_max_adv_id_seen": 0,
            "known_targets": [],
            "consumed_targets": [],
            "tombstoned_targets": [],
            "mkskipped": [],
        }
    return send, recv

def _parse_send_state(v: Any) -> Dict[str, Any]:
    obj = _unwrap_json(v)
    if not isinstance(obj, dict):
        raise ValueError("params.send_state: expected object")
    session_id = _parse_bytes(_get(obj, "session_id"), "params.send_state.session_id")
    dh_pub = _parse_bytes(_get(obj, "dh_pub"), "params.send_state.dh_pub")
    hk_s = _parse_bytes(_get(obj, "hk_s"), "params.send_state.hk_s")
    ck_ec = _parse_bytes(_get(obj, "ck_ec"), "params.send_state.ck_ec")
    ck_pq = _parse_bytes(_get(obj, "ck_pq"), "params.send_state.ck_pq")
    ns = _parse_u32(_get(obj, "ns"), "params.send_state.ns")
    pn = _parse_u32(_get(obj, "pn"), "params.send_state.pn")
    if len(session_id) != 16 or len(dh_pub) != 32 or len(hk_s) != 32 or len(ck_ec) != 32 or len(ck_pq) != 32:
        raise ValueError("params.send_state: invalid field length")
    return {
        "session_id": session_id,
        "dh_pub": dh_pub,
        "hk_s": hk_s,
        "ck_ec": ck_ec,
        "ck_pq": ck_pq,
        "ns": ns,
        "pn": pn,
    }

def _parse_recv_state(v: Any) -> Dict[str, Any]:
    obj = _unwrap_json(v)
    if not isinstance(obj, dict):
        raise ValueError("params.recv_state: expected object")
    session_id = _parse_bytes(_get(obj, "session_id"), "params.recv_state.session_id")
    dh_pub = _parse_bytes(_get(obj, "dh_pub"), "params.recv_state.dh_pub")
    hk_r = _parse_bytes(_get(obj, "hk_r"), "params.recv_state.hk_r")
    rk = _parse_bytes(_get(obj, "rk"), "params.recv_state.rk")
    ck_ec = _parse_bytes(_get(obj, "ck_ec"), "params.recv_state.ck_ec")
    ck_pq_send = _parse_bytes(_get(obj, "ck_pq_send"), "params.recv_state.ck_pq_send")
    ck_pq_recv = _parse_bytes(_get(obj, "ck_pq_recv"), "params.recv_state.ck_pq_recv")
    nr = _parse_u32(_get(obj, "nr"), "params.recv_state.nr")
    role = _get(obj, "role")
    if role not in ("A", "B"):
        raise ValueError("params.recv_state.role: expected \"A\" or \"B\"")
    peer_max_adv_id_seen = _parse_u32(_get(obj, "peer_max_adv_id_seen"), "params.recv_state.peer_max_adv_id_seen")
    known_targets = list(_unwrap_json(_get(obj, "known_targets")) or [])
    consumed_targets = list(_unwrap_json(_get(obj, "consumed_targets")) or [])
    tombstoned_targets = list(_unwrap_json(_get(obj, "tombstoned_targets")) or [])
    mkskipped = list(_unwrap_json(_get(obj, "mkskipped")) or [])
    if len(session_id) != 16 or len(dh_pub) != 32 or len(hk_r) != 32 or len(rk) != 32 or len(ck_ec) != 32 or len(ck_pq_send) != 32 or len(ck_pq_recv) != 32:
        raise ValueError("params.recv_state: invalid field length")
    return {
        "session_id": session_id,
        "dh_pub": dh_pub,
        "hk_r": hk_r,
        "rk": rk,
        "ck_ec": ck_ec,
        "ck_pq_send": ck_pq_send,
        "ck_pq_recv": ck_pq_recv,
        "nr": nr,
        "role": role,
        "peer_max_adv_id_seen": peer_max_adv_id_seen,
        "known_targets": known_targets,
        "consumed_targets": consumed_targets,
        "tombstoned_targets": tombstoned_targets,
        "mkskipped": mkskipped,
    }

def _encode_wire(pv: int, sid: int, dh_pub: bytes, flags: int, hdr_ct: bytes, body_ct: bytes) -> bytes:
    header = dh_pub + struct.pack(">H", flags) + hdr_ct
    wire = struct.pack(">H", pv) + struct.pack(">H", sid) + bytes([0x02, 0x00]) + struct.pack(">H", len(header)) + struct.pack(">H", len(body_ct)) + header + body_ct
    return wire

def _decode_wire(buf: bytes) -> Tuple[int, int, int, bytes, int, bytes, bytes]:
    if len(buf) < 10:
        raise ValueError("REJECT_S2_PARSE_PREFIX")
    pv = int.from_bytes(buf[0:2], "big")
    sid = int.from_bytes(buf[2:4], "big")
    msg_type = buf[4]
    header_len = int.from_bytes(buf[6:8], "big")
    body_len = int.from_bytes(buf[8:10], "big")
    if msg_type != 0x02:
        raise ValueError("REJECT_S2_PARSE_PREFIX")
    if len(buf) < 10 + header_len + body_len:
        raise ValueError("REJECT_S2_PARSE_PREFIX")
    header = buf[10:10 + header_len]
    body_ct = buf[10 + header_len:10 + header_len + body_len]
    if 10 + header_len + body_len != len(buf):
        raise ValueError("REJECT_S2_PARSE_PREFIX")
    if len(header) < 32 + 2 + 24:
        raise ValueError("REJECT_S2_PARSE_HDR_LEN")
    dh_pub = header[0:32]
    flags = int.from_bytes(header[32:34], "big")
    hdr_ct = header[34:]
    if flags != 0:
        raise ValueError("REJECT_S2_LOCAL_UNSUPPORTED")
    if len(header) != 32 + 2 + 24 or len(hdr_ct) != 24:
        raise ValueError("REJECT_S2_PARSE_HDR_LEN")
    if len(body_ct) < 16:
        raise ValueError("REJECT_S2_PARSE_BODY_LEN")
    return pv, sid, msg_type, dh_pub, flags, hdr_ct, body_ct

def pq_bind_sha512_32(flags_u16: int, pq_prefix: bytes) -> bytes:
    # pq_bind = H("QSP5.0/PQ-BIND" || u16(flags) || PQ_PREFIX)[0:32]
    label = b"QSP5.0/PQ-BIND"
    m = label + struct.pack(">H", flags_u16) + pq_prefix
    return _sha512(m)[:32]

def ad_hdr(session_id: bytes, protocol_version_u16: int, suite_id_u16: int, dh_pub: bytes, flags_u16: int, pq_bind: bytes) -> bytes:
    return session_id + struct.pack(">H", protocol_version_u16) + struct.pack(">H", suite_id_u16) + dh_pub + struct.pack(">H", flags_u16) + pq_bind

def ad_body(session_id: bytes, protocol_version_u16: int, suite_id_u16: int, pq_bind: bytes) -> bytes:
    return session_id + struct.pack(">H", protocol_version_u16) + struct.pack(">H", suite_id_u16) + pq_bind

# NOTE: Suite-2 KDF and mk derivation details must match repo spec/runners exactly.
# Implement concrete kdf ops after inspecting scripts/ci/run_suite2_kdf_vectors.py and refimpl_actor param conventions.

def handle(req: Dict[str, Any]) -> Dict[str, Any]:
    op = req.get("op")
    params = req.get("params") or {}
    if not isinstance(params, dict):
        return _err("invalid params")

    if op == "capabilities":
        # Minimal capabilities needed by the runners we will gate.
        return _ok({
            "ops": [
                "capabilities",
                "suite2.establish.run",
                "suite2.transcript.check",
                "suite2.mk_hybrid.check",
                "suite2.kdf_ec_ck",
                "suite2.kdf_pq_ck",
                "suite2.kdf_hybrid",
                "suite2.kdf_rk_dh",
                "suite2.kdf_rk_pq",
                "suite2.kdf_pq_reseed",
                "suite2.e2e.send",
                "suite2.e2e.recv",
            ]
        })

    if op == "suite2.establish.run":
        msg_type = _parse_u16(_get(params, "msg_type"), "params.msg_type")
        if msg_type != 0x01:
            return _err("reject: REJECT_S2_ESTABLISH_BAD_MSG_TYPE")
        negotiated = _unwrap_json(_get(params, "negotiated"))
        if not isinstance(negotiated, dict):
            return _err("invalid params.negotiated")
        pv = _parse_u16(_get(negotiated, "protocol_version"), "params.negotiated.protocol_version")
        sid = _parse_u16(_get(negotiated, "suite_id"), "params.negotiated.suite_id")
        if pv != 0x0500 or sid != 0x0002:
            return _err("reject: REJECT_S2_SUITE_MISMATCH")
        session_id = _parse_bytes(_get(params, "session_id"), "params.session_id")
        dh_init = _parse_bytes(_get(params, "dh_init"), "params.dh_init")
        pq_init_ss = _parse_bytes(_get(params, "pq_init_ss"), "params.pq_init_ss")
        dh_self_pub = _parse_bytes(_get(params, "dh_self_pub"), "params.dh_self_pub")
        dh_peer_pub = _parse_bytes(_get(params, "dh_peer_pub"), "params.dh_peer_pub")
        role = _get(params, "role")
        if isinstance(role, dict):
            role = role.get("role") or role.get("value")
        if role not in ("A", "B"):
            return _err("reject: REJECT_S2_ESTABLISH_BAD_INPUT_LEN")
        authenticated = _get(params, "authenticated")
        if authenticated is not True:
            return _err("reject: REJECT_S2_ESTABLISH_UNAUTHENTICATED")
        if (
            len(session_id) != 16
            or len(dh_init) != 32
            or len(pq_init_ss) != 32
            or len(dh_self_pub) != 32
            or len(dh_peer_pub) != 32
        ):
            return _err("reject: REJECT_S2_ESTABLISH_BAD_INPUT_LEN")

        send, recv = _init_from_handshake(role, session_id, dh_init, pq_init_ss, dh_self_pub, dh_peer_pub)
        SESSIONS[session_id] = {"send": send, "recv": recv}
        return _ok({"session_id": _b64u_encode(session_id)})

    if op == "suite2.transcript.check":
        negotiated = _get(params, "negotiated")
        if isinstance(negotiated, dict) and negotiated.get("type") == "json":
            negotiated = negotiated.get("data")
        if not isinstance(negotiated, dict):
            return _err("invalid params.negotiated")
        pv = _parse_u16(_get(negotiated, "protocol_version"), "params.negotiated.protocol_version")
        sid = _parse_u16(_get(negotiated, "suite_id"), "params.negotiated.suite_id")
        session_id = _parse_bytes(_get(params, "session_id"), "params.session_id")
        dh_pub = _parse_bytes(_get(params, "DH_pub"), "params.DH_pub")
        flags = _parse_u16(_get(params, "flags"), "params.flags")
        pq_prefix = _parse_bytes(_get(params, "pq_prefix"), "params.pq_prefix")
        ad_hdr_in = _parse_bytes(_get(params, "ad_hdr"), "params.ad_hdr")
        ad_body_in = _parse_bytes(_get(params, "ad_body"), "params.ad_body")

        pqb = pq_bind_sha512_32(flags, pq_prefix)
        ah = ad_hdr(session_id, pv, sid, dh_pub, flags, pqb)
        ab = ad_body(session_id, pv, sid, pqb)
        if ah != ad_hdr_in or ab != ad_body_in:
            return _err("reject: REJECT_S2_AD_MISMATCH")
        return _ok({
            "pq_bind": {"type": "hex", "data": _hex(pqb)},
            "ad_hdr": {"type": "hex", "data": _hex(ah)},
            "ad_body": {"type": "hex", "data": _hex(ab)},
        })

    if op == "suite2.mk_hybrid.check":
        ck_ec = _parse_bytes(_get(params, "CK_ec"), "params.CK_ec")
        ck_pq = _parse_bytes(_get(params, "CK_pq"), "params.CK_pq")
        if len(ck_ec) != 32:
            return _err("reject: REJECT_S2_MK_BAD_CK_EC")
        if len(ck_pq) != 32:
            return _err("reject: REJECT_S2_MK_BAD_CK_PQ")
        count = _parse_u32(_get(params, "count"), "params.count")
        mk_list: List[str] = []
        for _ in range(count):
            ck_ec_p = _kmac256(ck_ec, b"\x01", 32, b"QSP5.0/CK")
            ec_mk = _kmac256(ck_ec, b"\x02", 32, b"QSP5.0/MK")
            ck_pq_p = _kmac256(ck_pq, b"\x01", 32, b"QSP5.0/PQCK")
            pq_mk = _kmac256(ck_pq, b"\x02", 32, b"QSP5.0/PQMK")
            mk = _kmac256(ec_mk, pq_mk + b"\x01", 32, b"QSP5.0/HYBRID")
            mk_list.append(_hex(mk))
            ck_ec = ck_ec_p
            ck_pq = ck_pq_p
        if "expected_mk_list" in params:
            expected = _get(params, "expected_mk_list")
            exp_list: List[str] = []
            if isinstance(expected, dict) and expected.get("type") == "json":
                expected = expected.get("data")
            if not isinstance(expected, list):
                return _err("reject: REJECT_S2_MK_MISMATCH")
            for item in expected:
                if isinstance(item, dict) and item.get("type") == "hex":
                    exp_list.append(str(item.get("data", "")).lower())
                elif isinstance(item, str):
                    exp_list.append(item.lower())
                else:
                    return _err("reject: REJECT_S2_MK_MISMATCH")
            if len(exp_list) != len(mk_list):
                return _err("reject: REJECT_S2_MK_MISMATCH")
            for a, b in zip(exp_list, mk_list):
                if a != b.lower():
                    return _err("reject: REJECT_S2_MK_MISMATCH")
        return _ok({
            "mk_list": {"type": "json", "data": [{"type": "hex", "data": m} for m in mk_list]},
            "CK_ec_final": {"type": "hex", "data": _hex(ck_ec)},
            "CK_pq_final": {"type": "hex", "data": _hex(ck_pq)},
        })

    if op == "suite2.kdf_ec_ck":
        ck = _parse_bytes(_get(params, "CK_ec"), "params.CK_ec")
        ck_p = _kmac256(ck, b"\x01", 32, b"QSP5.0/CK")
        ec_mk = _kmac256(ck, b"\x02", 32, b"QSP5.0/MK")
        return _ok({"CK_ec_prime": {"type": "hex", "data": _hex(ck_p)}, "ec_mk": {"type": "hex", "data": _hex(ec_mk)}})

    if op == "suite2.kdf_pq_ck":
        ck = _parse_bytes(_get(params, "CK_pq"), "params.CK_pq")
        ck_p = _kmac256(ck, b"\x01", 32, b"QSP5.0/PQCK")
        pq_mk = _kmac256(ck, b"\x02", 32, b"QSP5.0/PQMK")
        return _ok({"CK_pq_prime": {"type": "hex", "data": _hex(ck_p)}, "pq_mk": {"type": "hex", "data": _hex(pq_mk)}})

    if op == "suite2.kdf_hybrid":
        ec_mk = _parse_bytes(_get(params, "ec_mk"), "params.ec_mk")
        pq_mk = _parse_bytes(_get(params, "pq_mk"), "params.pq_mk")
        mk = _kmac256(ec_mk, pq_mk + b"\x01", 32, b"QSP5.0/HYBRID")
        return _ok({"mk": {"type": "hex", "data": _hex(mk)}})

    if op == "suite2.kdf_rk_dh":
        rk = _parse_bytes(_get(params, "RK"), "params.RK")
        dh = _parse_bytes(_get(params, "dh_out"), "params.dh_out")
        tmp = _kmac256(rk, dh, 64, b"QSP5.0/RKDH")
        rk_p = tmp[:32]
        ck0 = tmp[32:64]
        return _ok({"RK_prime": {"type": "hex", "data": _hex(rk_p)}, "CK_ec0": {"type": "hex", "data": _hex(ck0)}})

    if op == "suite2.kdf_rk_pq":
        rk = _parse_bytes(_get(params, "RK"), "params.RK")
        ss = _parse_bytes(_get(params, "pq_ss"), "params.pq_ss")
        rk_p = _kmac256(rk, ss + b"\x01", 32, b"QSP5.0/RKPQ")
        return _ok({"RK_prime": {"type": "hex", "data": _hex(rk_p)}})

    if op == "suite2.kdf_pq_reseed":
        rk = _parse_bytes(_get(params, "RK"), "params.RK")
        tid = _parse_u32(_get(params, "pq_target_id"), "params.pq_target_id")
        ct = _parse_bytes(_get(params, "pq_ct"), "params.pq_ct")
        ss = _parse_bytes(_get(params, "pq_epoch_ss"), "params.pq_epoch_ss")
        h = _sha512(ct)
        ctx = b"QSP5.0/SCKA/CTXT" + struct.pack(">I", tid) + h[:32] + ss
        a2b = _kmac256(rk, ctx, 32, b"QSP5.0/PQSEED/A->B")
        b2a = _kmac256(rk, ctx, 32, b"QSP5.0/PQSEED/B->A")
        return _ok({"CK_pq_seed_A2B": {"type": "hex", "data": _hex(a2b)}, "CK_pq_seed_B2A": {"type": "hex", "data": _hex(b2a)}})

    if op == "suite2.e2e.send":
        negotiated = _get(params, "negotiated")
        negotiated = _unwrap_json(negotiated)
        if not isinstance(negotiated, dict):
            return _err("invalid params.negotiated")
        pv = _parse_u16(_get(negotiated, "protocol_version"), "params.negotiated.protocol_version")
        sid = _parse_u16(_get(negotiated, "suite_id"), "params.negotiated.suite_id")
        session_id_b64 = params.get("session_id")
        session_id = None
        if isinstance(session_id_b64, str):
            session_id = _b64u_decode(session_id_b64)
        if "send_state" in params:
            send_state = _parse_send_state(_get(params, "send_state"))
        elif session_id is not None and session_id in SESSIONS:
            send_state = SESSIONS[session_id]["send"]
        else:
            return _err("params.send_state missing")
        if session_id is not None and send_state["session_id"] != session_id:
            return _err("params.session_id does not match send_state.session_id")
        flags = _parse_u16(_get(params, "flags"), "params.flags") if "flags" in params else 0
        if flags != 0:
            return _err("reject: REJECT_S2_LOCAL_UNSUPPORTED")
        plaintext = _parse_bytes(_get(params, "plaintext_hex"), "params.plaintext_hex")
        ck_ec_p, ck_pq_p, mk = _derive_mk_step(send_state["ck_ec"], send_state["ck_pq"])
        pq_bind = pq_bind_sha512_32(flags, b"")
        ad_h = ad_hdr(send_state["session_id"], pv, sid, send_state["dh_pub"], flags, pq_bind)
        ad_b = ad_body(send_state["session_id"], pv, sid, pq_bind)
        hdr_pt = struct.pack(">II", send_state["pn"], send_state["ns"])
        hdr_ct = _gcm_seal(send_state["hk_s"], _nonce_hdr(send_state["session_id"], send_state["dh_pub"], send_state["ns"]), ad_h, hdr_pt)
        body_ct = _gcm_seal(mk, _nonce_body(send_state["session_id"], send_state["dh_pub"], send_state["ns"]), ad_b, plaintext)
        wire = _encode_wire(pv, sid, send_state["dh_pub"], flags, hdr_ct, body_ct)
        new_state = {
            "session_id": _hex(send_state["session_id"]),
            "dh_pub": _hex(send_state["dh_pub"]),
            "hk_s": _hex(send_state["hk_s"]),
            "ck_ec": _hex(ck_ec_p),
            "ck_pq": _hex(ck_pq_p),
            "ns": {"u32": send_state["ns"] + 1},
            "pn": {"u32": send_state["pn"]},
        }
        if session_id is not None:
            if "recv_state" in params:
                recv_state = _parse_recv_state(_get(params, "recv_state"))
            elif session_id in SESSIONS:
                recv_state = SESSIONS[session_id]["recv"]
            else:
                return _err("params.recv_state missing for new suite2 session")
            SESSIONS[session_id] = {
                "send": {
                    "session_id": send_state["session_id"],
                    "dh_pub": send_state["dh_pub"],
                    "hk_s": send_state["hk_s"],
                    "ck_ec": ck_ec_p,
                    "ck_pq": ck_pq_p,
                    "ns": send_state["ns"] + 1,
                    "pn": send_state["pn"],
                },
                "recv": recv_state,
            }
        return _ok({
            "wire_hex": {"type": "hex", "data": _hex(wire)},
            "meta": {"type": "json", "data": {"flags": {"u16": flags}, "pn": {"u32": send_state["pn"]}, "n": {"u32": send_state["ns"]}}},
            "new_state": {"type": "json", "data": new_state},
        })

    if op == "suite2.e2e.recv":
        negotiated = _get(params, "negotiated")
        negotiated = _unwrap_json(negotiated)
        if not isinstance(negotiated, dict):
            return _err("invalid params.negotiated")
        pv = _parse_u16(_get(negotiated, "protocol_version"), "params.negotiated.protocol_version")
        sid = _parse_u16(_get(negotiated, "suite_id"), "params.negotiated.suite_id")
        session_id_b64 = params.get("session_id")
        session_id = None
        if isinstance(session_id_b64, str):
            session_id = _b64u_decode(session_id_b64)
        if "recv_state" in params:
            recv_state = _parse_recv_state(_get(params, "recv_state"))
        elif session_id is not None and session_id in SESSIONS:
            recv_state = SESSIONS[session_id]["recv"]
        else:
            return _err("params.recv_state missing")
        if session_id is not None and recv_state["session_id"] != session_id:
            return _err("params.session_id does not match recv_state.session_id")
        wire = _parse_bytes(_get(params, "wire_hex"), "params.wire_hex")
        try:
            pv_w, sid_w, _mt, dh_pub, flags, hdr_ct, body_ct = _decode_wire(wire)
        except ValueError as e:
            return _err(f"reject: {e}")
        if pv_w != pv or sid_w != sid:
            return _err("reject: REJECT_S2_PARSE_PREFIX")
        if flags != 0:
            return _err("reject: REJECT_S2_LOCAL_UNSUPPORTED")
        pq_bind = pq_bind_sha512_32(flags, b"")
        ad_h = ad_hdr(recv_state["session_id"], pv, sid, dh_pub, flags, pq_bind)
        ad_b = ad_body(recv_state["session_id"], pv, sid, pq_bind)
        try:
            hdr_pt = _gcm_open(recv_state["hk_r"], _nonce_hdr(recv_state["session_id"], dh_pub, recv_state["nr"]), ad_h, hdr_ct)
        except Exception:
            return _err("reject: REJECT_S2_HDR_AUTH_FAIL")
        if len(hdr_pt) != 8:
            return _err("reject: REJECT_S2_HDR_AUTH_FAIL")
        pn, n = struct.unpack(">II", hdr_pt)
        if n != recv_state["nr"]:
            return _err("reject: REJECT_S2_LOCAL_UNSUPPORTED")
        ck_ec_p, ck_pq_p, mk = _derive_mk_step(recv_state["ck_ec"], recv_state["ck_pq_recv"])
        try:
            body_pt = _gcm_open(mk, _nonce_body(recv_state["session_id"], dh_pub, n), ad_b, body_ct)
        except Exception:
            return _err("reject: REJECT_S2_BODY_AUTH_FAIL")
        new_state = {
            "session_id": _hex(recv_state["session_id"]),
            "dh_pub": _hex(dh_pub),
            "hk_r": _hex(recv_state["hk_r"]),
            "rk": _hex(recv_state["rk"]),
            "ck_ec": _hex(ck_ec_p),
            "ck_pq_send": _hex(recv_state["ck_pq_send"]),
            "ck_pq_recv": _hex(ck_pq_p),
            "nr": {"u32": n + 1},
            "role": recv_state["role"],
            "peer_max_adv_id_seen": {"u32": recv_state["peer_max_adv_id_seen"]},
            "known_targets": recv_state["known_targets"],
            "consumed_targets": recv_state["consumed_targets"],
            "tombstoned_targets": recv_state["tombstoned_targets"],
            "mkskipped": recv_state["mkskipped"],
        }
        if session_id is not None:
            if "send_state" in params:
                send_state = _parse_send_state(_get(params, "send_state"))
            elif session_id in SESSIONS:
                send_state = SESSIONS[session_id]["send"]
            else:
                return _err("params.send_state missing for new suite2 session")
            SESSIONS[session_id] = {
                "send": send_state,
                "recv": {
                    "session_id": recv_state["session_id"],
                    "dh_pub": dh_pub,
                    "hk_r": recv_state["hk_r"],
                    "rk": recv_state["rk"],
                    "ck_ec": ck_ec_p,
                    "ck_pq_send": recv_state["ck_pq_send"],
                    "ck_pq_recv": ck_pq_p,
                    "nr": n + 1,
                    "role": recv_state["role"],
                    "peer_max_adv_id_seen": recv_state["peer_max_adv_id_seen"],
                    "known_targets": recv_state["known_targets"],
                    "consumed_targets": recv_state["consumed_targets"],
                    "tombstoned_targets": recv_state["tombstoned_targets"],
                    "mkskipped": recv_state["mkskipped"],
                },
            }
        return _ok({
            "plaintext_hex": {"type": "hex", "data": _hex(body_pt)},
            "meta": {"type": "json", "data": {"flags": {"u16": flags}, "pn": {"u32": pn}, "n": {"u32": n}}},
            "new_state": {"type": "json", "data": new_state},
        })

    return _err("reject: REJECT_UNSUPPORTED_OP")

def main() -> int:
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
            if not isinstance(req, dict):
                _write(_err("bad request"))
                continue
            rid = req.get("id")
            try:
                resp = handle(req)
            except Exception as e:
                resp = _err(str(e))
            # Match actor response envelope expected by runners: include id if present.
            out = {"id": rid, **resp} if rid is not None else resp
            _write(out)
        except Exception as e:
            _write(_err(f"internal: {e}"))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
