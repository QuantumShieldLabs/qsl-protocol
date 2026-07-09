#!/usr/bin/env python3
"""NA-0626 Phase-2(a) WF-0014 byte-scan (D563, Operator Decision 5).

Decodes EVERY pinned byte string in inputs/**/*.json (superset of the directive's
inputs/**/vectors/*.json glob, fail-closed) as (a) a Suite-2 wire envelope, via a
faithful port of tools/refimpl/quantumshield_refimpl/src/suite2/parse.rs
`decode_suite2_wire` (same field order, same strictness, same reject reasons), and
(b) a QS2S session snapshot, via a faithful port of state.rs `restore_bytes` v2
layout (magic + version reported for any version; strict full-layout parse for v2).

Reports, deterministically sorted:
  1. every hex string that decodes as a Suite-2 wire envelope (file, vector id,
     JSON path, length, flags, prefix shape, sha256_12 fingerprint) and whether its
     flags/shape intersect the NA-0626 lane semantics (any FLAG_BOUNDARY-bearing
     frame transits the receiver dispatch this lane restructures; flags==0 frames
     do not);
  2. every hex string that carries the QS2S magic (all versions; a pinned snapshot
     is ALWAYS lane-affected: this lane bumps QS2S v2 -> v3 with v2 fail-closed);
  3. an inventory of state-shaped JSON objects carrying an `rk` member (the
     recv-state schema surface the single-root refactor may reshape) and of
     DH-ratchet-state markers (`dhs_priv`/`dhr`);
  4. a per-file AFFECTED verdict with the reason classes, which IS the Decision-5
     vector-artifact scope input.

No raw frame hex is emitted (sha256_12 fingerprints only) per the AGENTS.md
evidence-wording rules. Output is deterministic for a given tree (no timestamps).
"""

import glob
import hashlib
import json
import os
import re
import subprocess
import sys

FLAG_PQ_ADV = 0x0001
FLAG_PQ_CTXT = 0x0002
FLAG_BOUNDARY = 0x0004
KNOWN_FLAGS = FLAG_PQ_ADV | FLAG_PQ_CTXT | FLAG_BOUNDARY
SUITE2_PROTOCOL_VERSION = 0x0500
SUITE2_SUITE_ID = 0x0002
HDR_CT_LEN = 24
PQ_ADV_PUB_LEN = 1184
PQ_CT_LEN = 1088
BODY_CT_MIN = 16

HEXCHARS = set("0123456789abcdefABCDEF")


def is_hexstr(s):
    return len(s) >= 20 and len(s) % 2 == 0 and all(c in HEXCHARS for c in s)


def be16(b, i):
    return (b[i] << 8) | b[i + 1]


def be32(b, i):
    return (b[i] << 24) | (b[i + 1] << 16) | (b[i + 2] << 8) | b[i + 3]


def parse_ratchet_header(h):
    """Port of parse.rs parse_ratchet_header. Returns (dict, used) or (None, reason)."""
    if len(h) < 32 + 2:
        return None, "REJECT_S2_PARSE_PREFIX"
    off = 0
    dh_pub = h[0:32]
    off += 32
    flags = be16(h, off)
    off += 2
    if flags & ~KNOWN_FLAGS:
        return None, "REJECT_S2_PARSE_FLAGS"
    if (flags & FLAG_PQ_ADV) and not (flags & FLAG_BOUNDARY):
        return None, "REJECT_S2_PARSE_FLAGS"
    if (flags & FLAG_PQ_CTXT) and not (flags & FLAG_BOUNDARY):
        return None, "REJECT_S2_PARSE_FLAGS"
    pq_adv_id = pq_target_id = None
    if flags & FLAG_PQ_ADV:
        if len(h) < off + 4 + PQ_ADV_PUB_LEN:
            return None, "REJECT_S2_PQPREFIX_PARSE"
        pq_adv_id = be32(h, off)
        off += 4 + PQ_ADV_PUB_LEN
    if flags & FLAG_PQ_CTXT:
        if len(h) < off + 4 + PQ_CT_LEN:
            return None, "REJECT_S2_PQPREFIX_PARSE"
        pq_target_id = be32(h, off)
        off += 4 + PQ_CT_LEN
    if len(h) < off + HDR_CT_LEN:
        return None, "REJECT_S2_PARSE_HDR_LEN"
    hdr_ct_off = off
    off += HDR_CT_LEN
    return (
        {
            "dh_pub_sha12": hashlib.sha256(dh_pub).hexdigest()[:12],
            "flags": flags,
            "pq_adv_id": pq_adv_id,
            "pq_target_id": pq_target_id,
            "hdr_ct_off_in_header": hdr_ct_off,
        },
        off,
    )


def decode_suite2_wire(b):
    """Port of parse.rs decode_suite2_wire. Returns dict or None."""
    if len(b) < 10:
        return None
    pv = be16(b, 0)
    sid = be16(b, 2)
    msg_type = b[4]
    header_len = be16(b, 6)
    body_len = be16(b, 8)
    if pv != SUITE2_PROTOCOL_VERSION or sid != SUITE2_SUITE_ID or msg_type != 0x02:
        return None
    if len(b) < 10 + header_len + body_len:
        return None
    if 10 + header_len + body_len != len(b):
        return None
    header = b[10 : 10 + header_len]
    body = b[10 + header_len :]
    parsed, used = parse_ratchet_header(header)
    if parsed is None:
        return None
    if used != len(header):
        return None
    if len(body) < BODY_CT_MIN:
        return None
    parsed["total_len"] = len(b)
    parsed["header_len"] = header_len
    parsed["body_ct_len"] = body_len
    parsed["hdr_ct_range"] = [10 + parsed.pop("hdr_ct_off_in_header"), 0]
    parsed["hdr_ct_range"][1] = parsed["hdr_ct_range"][0] + HDR_CT_LEN
    return parsed


def decode_suite2_ratchet_message(b):
    """Port of parse.rs decode_suite2_ratchet_message (headerless ratchet msg, no
    10-byte envelope; parse_vectors pin THIS format). Returns (dict|None, reason)."""
    parsed, used_or_reason = parse_ratchet_header(b)
    if parsed is None:
        return None, used_or_reason
    used = used_or_reason
    body_ct = b[used:]
    if len(body_ct) < BODY_CT_MIN:
        return None, "REJECT_S2_PARSE_BODY_LEN"
    parsed["total_len"] = len(b)
    parsed["body_ct_len"] = len(body_ct)
    hdr_off = parsed.pop("hdr_ct_off_in_header")
    parsed["hdr_ct_range"] = [hdr_off, hdr_off + HDR_CT_LEN]
    return parsed, None


def parse_qs2s(b):
    """QS2S snapshot detector + strict v2 layout parse (port of state.rs restore_bytes)."""
    if len(b) < 5 or b[0:4] != b"QS2S":
        return None
    ver = b[4]
    out = {"version": ver, "total_len": len(b), "v2_strict_parse": False}
    if ver != 2:
        return out
    i = 5

    def take(n):
        nonlocal i
        if i + n > len(b):
            raise ValueError("truncated")
        s = b[i : i + n]
        i += n
        return s

    try:
        take(16); take(2); take(2)                     # send: session_id, pv, suite
        for _ in range(4):
            take(32)                                   # dh_pub, hk_s, ck_ec, ck_pq
        take(4); take(4)                               # ns, pn
        take(16); take(2); take(2)                     # recv: session_id, pv, suite
        for _ in range(6):
            take(32)                                   # dh_pub, hk_r, rk, ck_ec, ck_pq_send, ck_pq_recv
        take(4)                                        # nr
        role = take(1)[0]
        if role not in (0, 1):
            raise ValueError("role")
        take(4)                                        # peer_max_adv_id_seen
        for _set in range(3):                          # known/consumed/tombstoned
            n = be32(take(4), 0)
            if n > 10_000:
                raise ValueError("set len")
            take(4 * n)
        n = be32(take(4), 0)                           # mkskipped
        if n > 1000:
            raise ValueError("mk len")
        take(68 * n)
        for _ in range(4):
            take(32)                                   # dh: dhs_priv, dhs_pub, dhr, rk
        if i != len(b):
            raise ValueError("trailing")
        out["v2_strict_parse"] = True
    except ValueError as e:
        out["v2_parse_error"] = str(e)
    return out


def flags_name(f):
    parts = []
    if f & FLAG_PQ_ADV:
        parts.append("PQ_ADV")
    if f & FLAG_PQ_CTXT:
        parts.append("PQ_CTXT")
    if f & FLAG_BOUNDARY:
        parts.append("BOUNDARY")
    return "|".join(parts) if parts else "NONE"


def walk(node, path, hexes, rk_nodes, knobs):
    if isinstance(node, dict):
        keys = set(node.keys())
        if "rk" in keys and isinstance(node.get("rk"), str) and is_hexstr(node["rk"]):
            rk_nodes.append((path + ".rk(parent)", sorted(keys)))
        if {"dhs_priv", "dhr"} & keys:
            rk_nodes.append((path + ".DH-STATE-MARKER", sorted(keys)))
        # Constructed-frame knobs: vectors that build frames at RUN TIME carry a
        # message spec or a flags/hdr_key override instead of pinned wire bytes.
        # They are byte-freeze-independent but SEMANTICS-relevant to the lane's
        # receiver-dispatch change; inventory them for the Decision-5 verdict.
        for kn in ("flags", "hdr_key", "hdr_key_hex", "message"):
            if kn in keys:
                v = node[kn]
                if isinstance(v, dict) and set(v.keys()) == {"type", "data"}:
                    v = v["data"]
                if kn == "message":
                    if isinstance(v, dict):
                        knobs.append((f"{path}.{kn}", "spec-keys=" + ",".join(sorted(v.keys()))))
                elif kn == "flags":
                    knobs.append((f"{path}.{kn}", f"value={json.dumps(v)}"))
                elif not isinstance(v, (dict, list)):
                    knobs.append((f"{path}.{kn}", f"value={json.dumps(v)[:80]}"))
        for k in sorted(node.keys()):
            walk(node[k], f"{path}.{k}", hexes, rk_nodes, knobs)
    elif isinstance(node, list):
        for idx, v in enumerate(node):
            walk(v, f"{path}[{idx}]", hexes, rk_nodes, knobs)
    elif isinstance(node, str) and is_hexstr(node):
        hexes.append((path, node))


def vector_id_for_path(doc, path):
    # path like .vectors[3].input....  -> return vectors[3].id
    if path.startswith(".vectors["):
        idx = int(path[len(".vectors[") :].split("]")[0])
        try:
            return doc["vectors"][idx].get("id", f"<vectors[{idx}]>")
        except Exception:
            return f"<vectors[{idx}]>"
    return "<top-level>"


def main():
    repo = os.getcwd()
    head = subprocess.run(
        ["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True
    ).stdout.strip()

    directive_glob = sorted(glob.glob("inputs/**/vectors/*.json", recursive=True))
    all_inputs = sorted(glob.glob("inputs/**/*.json", recursive=True))
    extra = [f for f in all_inputs if f not in directive_glob]

    print("NA-0626 WF-0014 PINNED-FRAME BYTE-SCAN (Phase 2(a), D563)")
    print(f"repo_head={head}")
    print(f"directive_glob_files={len(directive_glob)} extra_inputs_json={len(extra)}")
    print()

    frames = []       # envelope-level (file, vecid, path, info)
    msgs = []         # headerless ratchet-message-level (file, vecid, path, info)
    nearmiss = []     # msg/wire-named hex that decodes as NEITHER (file, vecid, path, reasons)
    snapshots = []    # (file, vecid, path, info)
    rk_inventory = [] # (file, vecid, path, keys)
    knob_inventory = []  # constructed-frame knobs (file, vecid, path, desc)
    per_file = {}

    unparseable = []
    for f in directive_glob + extra:
        with open(f, "rb") as fh:
            raw = fh.read()
        fsha = hashlib.sha256(raw).hexdigest()[:12]
        hexes, rk_nodes, knobs = [], [], []
        try:
            doc = json.loads(raw)
            walk(doc, "", hexes, rk_nodes, knobs)
        except json.JSONDecodeError as e:
            if f in directive_glob:
                # A directive-glob vector file that does not parse is a STOP-grade
                # inconsistency: fail closed, scan nothing further.
                print(f"FATAL: directive-glob file {f} is not valid JSON: {e}")
                return 2
            # Deliberate malformed-JSON fixtures (inputs/local_ops/**): fall back to a
            # raw-bytes hex-token scan so nothing is skipped silently.
            doc = None
            unparseable.append((f, str(e).split("\n")[0]))
            for m in re.finditer(rb"[0-9a-fA-F]{20,}", raw):
                tok = m.group(0).decode()
                if len(tok) % 2 == 0:
                    hexes.append((f".rawbytes[{m.start()}]", tok))
        stats = {
            "sha12": fsha,
            "hex_strings": len(hexes),
            "wire_frames": [],
            "msgs": [],
            "nearmiss": [],
            "qs2s": [],
            "rk_nodes": [],
            "knobs": [],
        }
        for path, hx in hexes:
            b = bytes.fromhex(hx)
            vid = vector_id_for_path(doc, path) if doc is not None else "<raw>"
            hit = False
            w = decode_suite2_wire(b)
            if w is not None:
                hit = True
                w["flags_name"] = flags_name(w["flags"])
                w["sha12"] = hashlib.sha256(b).hexdigest()[:12]
                w["intersects_lane"] = w["flags"] != 0
                frames.append((f, vid, path, w))
                stats["wire_frames"].append((vid, path, w))
            else:
                m, _reason = decode_suite2_ratchet_message(b)
                if m is not None:
                    hit = True
                    m["flags_name"] = flags_name(m["flags"])
                    m["sha12"] = hashlib.sha256(b).hexdigest()[:12]
                    m["intersects_lane"] = m["flags"] != 0
                    msgs.append((f, vid, path, m))
                    stats["msgs"].append((vid, path, m))
            q = parse_qs2s(b)
            if q is not None:
                hit = True
                q["sha12"] = hashlib.sha256(b).hexdigest()[:12]
                snapshots.append((f, vid, path, q))
                stats["qs2s"].append((vid, path, q))
            # Fail-closed: a hex string living under a msg/wire-named path that
            # decodes as NOTHING is a deliberately malformed pinned frame; report
            # it rather than skipping it silently (the WF-0014 blind spot).
            if not hit and re.search(r"(msg|wire)", path, re.IGNORECASE):
                _, wire_reason = None, "not-an-envelope"
                m, msg_reason = decode_suite2_ratchet_message(b)
                flags_guess = be16(b, 32) if len(b) >= 34 else None
                info = {
                    "total_len": len(b),
                    "msg_reject": msg_reason,
                    "flags_at_32": (f"0x{flags_guess:04x}" if flags_guess is not None else "n/a"),
                    "sha12": hashlib.sha256(b).hexdigest()[:12],
                }
                nearmiss.append((f, vid, path, info))
                stats["nearmiss"].append((vid, path, info))
        for path, keys in rk_nodes:
            vid = vector_id_for_path(doc, path)
            rk_inventory.append((f, vid, path, keys))
            stats["rk_nodes"].append((vid, path))
        for path, desc in knobs:
            vid = vector_id_for_path(doc, path)
            knob_inventory.append((f, vid, path, desc))
            stats["knobs"].append((vid, path))
        per_file[f] = stats

    print("== SECTION 1: pinned Suite-2 wire envelopes (exhaustive) ==")
    if not frames:
        print("none")
    for f, vid, path, w in frames:
        print(
            f"FRAME file={os.path.basename(f)} vector={vid} path={path} "
            f"len={w['total_len']} flags=0x{w['flags']:04x}({w['flags_name']}) "
            f"pq_adv_id={w['pq_adv_id']} pq_target_id={w['pq_target_id']} "
            f"hdr_ct_range={w['hdr_ct_range']} body_ct_len={w['body_ct_len']} "
            f"dh_pub_sha12={w['dh_pub_sha12']} frame_sha12={w['sha12']} "
            f"intersects_lane={'YES' if w['intersects_lane'] else 'no'}"
        )
    print()
    print("== SECTION 1b: pinned HEADERLESS ratchet messages (parse_vectors format) ==")
    if not msgs:
        print("none")
    for f, vid, path, m in msgs:
        print(
            f"MSG file={os.path.basename(f)} vector={vid} path={path} "
            f"len={m['total_len']} flags=0x{m['flags']:04x}({m['flags_name']}) "
            f"pq_adv_id={m['pq_adv_id']} pq_target_id={m['pq_target_id']} "
            f"hdr_ct_range={m['hdr_ct_range']} body_ct_len={m['body_ct_len']} "
            f"dh_pub_sha12={m['dh_pub_sha12']} frame_sha12={m['sha12']} "
            f"intersects_lane={'YES' if m['intersects_lane'] else 'no'}"
        )
    print()
    print("== SECTION 1c: msg/wire-named pinned hex decoding as NEITHER (deliberate malformed) ==")
    if not nearmiss:
        print("none")
    for f, vid, path, info in nearmiss:
        print(
            f"MALFORMED file={os.path.basename(f)} vector={vid} path={path} "
            f"len={info['total_len']} msg_reject={info['msg_reject']} "
            f"flags_at_32={info['flags_at_32']} sha12={info['sha12']}"
        )
    print()
    print("== SECTION 2: pinned QS2S snapshots (any version; all lane-affected) ==")
    if not snapshots:
        print("none")
    for f, vid, path, q in snapshots:
        print(
            f"QS2S file={os.path.basename(f)} vector={vid} path={path} "
            f"version={q['version']} len={q['total_len']} "
            f"v2_strict_parse={q['v2_strict_parse']} sha12={q['sha12']}"
        )
    print()
    print("== SECTION 3: state-shaped JSON objects with an `rk` member / DH-state markers ==")
    if not rk_inventory:
        print("none")
    for f, vid, path, keys in rk_inventory:
        print(f"RKNODE file={os.path.basename(f)} vector={vid} path={path} keys={','.join(keys)}")
    print()
    print("== SECTION 3b: constructed-frame knobs (runtime-built frames; semantics surface) ==")
    if not knob_inventory:
        print("none")
    for f, vid, path, desc in knob_inventory:
        print(f"KNOB file={os.path.basename(f)} vector={vid} path={path} {desc}")
    print()
    print("== SECTION 4: per-file verdict (Decision-5 artifact-scope input) ==")
    for f in sorted(per_file):
        s = per_file[f]
        n_frames = len(s["wire_frames"])
        n_flagged = sum(1 for _, _, w in s["wire_frames"] if w["flags"] != 0)
        n_msgs = len(s["msgs"])
        n_msg_flagged = sum(1 for _, _, m in s["msgs"] if m["flags"] != 0)
        n_near = len(s["nearmiss"])
        n_qs2s = len(s["qs2s"])
        n_rk = len(s["rk_nodes"])
        n_knobs = len(s["knobs"])
        reasons = []
        if n_flagged:
            reasons.append(f"pinned-flagged-envelopes={n_flagged}")
        if n_msg_flagged:
            reasons.append(f"pinned-flagged-msgs={n_msg_flagged}")
        if n_qs2s:
            reasons.append(f"pinned-qs2s={n_qs2s}")
        if n_rk:
            reasons.append(f"rk-state-nodes={n_rk}")
        if n_knobs:
            reasons.append(f"constructed-frame-knobs={n_knobs}")
        verdict = "AFFECTED-or-semantics-adjacent" if reasons else "not-affected-by-bytes"
        print(
            f"FILE {f} sha12={s['sha12']} hex_strings={s['hex_strings']} "
            f"envelopes={n_frames} msgs={n_msgs} malformed={n_near} qs2s={n_qs2s} "
            f"rk_nodes={n_rk} knobs={n_knobs} -> {verdict}"
            + (f" [{'; '.join(reasons)}]" if reasons else "")
        )
    print()
    if unparseable:
        print("== non-JSON files raw-scanned (deliberate malformed fixtures, non-vector) ==")
        for f, err in unparseable:
            print(f"RAWSCANNED {f} ({err})")
        print()
    print(
        "note: rk-state-nodes marks the actor state-JSON schema surface (recv_state/new_state\n"
        "carrying `rk`); whether those vectors change is decided by the design-lock state-shape\n"
        "pin (2b/2c), not by wire bytes. pinned-flagged envelopes/msgs transit the boundary\n"
        "dispatch this lane restructures: the regenerator must assert byte-identity for every\n"
        "one it does not deliberately change. pinned-qs2s blobs are ALWAYS affected (QS2S\n"
        "v2->v3, v2 fail-closed). constructed-frame-knobs mark vectors that BUILD frames at run\n"
        "time (flags/hdr_key/message specs): byte-freeze-independent, but their expected\n"
        "dispositions sit on the receiver-dispatch surface and each must be dispositioned in\n"
        "the design-lock (esp. any flags override reaching the recv_wire dispatch)."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
