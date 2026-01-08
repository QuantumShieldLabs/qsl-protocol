from __future__ import annotations

from typing import Any, Dict, List, Tuple

# ---- Phase 4 canonical ID helpers -------------------------------------------------

def _sanitize_id(raw: str) -> str:
    """Return a schema-safe non-empty id component: ^[A-Za-z0-9._:-]+$ (or empty if nothing remains)."""
    return "".join(ch for ch in (raw or "") if (ch.isalnum() or ch in "._:-"))


def _interop_case_id(*, p3_case_id: str, suite: str, direction: str) -> str:
    """Deterministically derive a schema-safe Phase 4 interop case id.

    The Phase 4 schema requires: ^[A-Za-z0-9._:-]+$ and non-empty.
    Direction strings like 'A->B' contain '>' which is not allowed by the schema regex,
    so we normalize to 'AtoB' / 'BtoA'.
    """
    if direction == "A->B":
        dir_tag = "AtoB"
    elif direction == "B->A":
        dir_tag = "BtoA"
    else:
        raise ValueError(f"Unsupported direction for interop case id: {direction!r}")

    cid = _sanitize_id(f"{p3_case_id}.{suite}.{dir_tag}")
    if not cid:
        raise ValueError("Derived interop case id is empty after sanitization")
    return cid

# -------------------------------------------------------------------------------


def _td_from_p3_input(p3_input: Dict[str, Any]) -> Dict[str, Any]:
    """Normalize P3 'input' into a Phase4 typed_data object.

    P3-23 uses a few ad-hoc types (b64u, string, path).
    Phase4 schemas allow: b64u | hex | utf8 | json.
    """
    t = (p3_input or {}).get("type")
    if t == "b64u":
        return {"type": "b64u", "data": p3_input.get("data", "")}
    if t == "string":
        # P3-23 uses this for intentionally-invalid base64url strings.
        return {"type": "utf8", "data": p3_input.get("data", "")}
    if t == "path":
        # Keep as a reference (do not inline large blobs).
        return {"type": "json", "data": {"path": p3_input.get("path", "")}, "semantic": "zip_member"}
    # Fall back to utf8 string representation.
    return {"type": "utf8", "data": str(p3_input.get("data", ""))}


def adapt_p3_23_negative_to_vector_set(
    p3_doc: Dict[str, Any],
    *,
    generated_at: str,
    phase3_member: str,
) -> Dict[str, Any]:
    cases = p3_doc.get("cases")
    if not isinstance(cases, list) or not cases:
        raise ValueError("P3-23 payload missing 'cases' list")

    vectors: List[Dict[str, Any]] = []
    for c in cases:
        vid = c.get("id")
        op = c.get("op")
        expect = c.get("expect")
        p3_in = c.get("input") or {}
        if not isinstance(vid, str) or not isinstance(op, str) or not isinstance(expect, dict):
            raise ValueError("P3-23 case missing required fields")

        td = _td_from_p3_input(p3_in)
        vectors.append({
            "id": vid,
            "op": op,
            "notes": c.get("notes") or "",
            "maps_to": c.get("maps_to") or [],
            "tags": ["phase3", "p3-23", "negative"],
            "input": {"data": td},
            "expect": expect,
            "ext": {
                "p3": {
                    "artifact_id": p3_doc.get("artifact_id"),
                    "canonical": p3_doc.get("canonical"),
                }
            }
        })

    return {
        "format": "QSHIELD-P4-VECTOR-SET-1",
        "schema_version": "1.0.0",
        "generated_at": generated_at,
        "source": {
            "phase": "3",
            "artifact_id": "P3-23",
            "member": phase3_member,
            "format": str(p3_doc.get("artifact_id", "P3-23")),
            "version": "1.0",
        },
        "protocol": {
            "protocol_version": "0x0403",
            "suite_id": "0x0000",
            "suite_name": "Mixed",
        },
        "vectors": vectors,
    }


def make_handshake_smoke_case(
    *,
    case_id: str,
    suite: str,
    direction: str,
    p3_case_id: str,
    seed: str,
    plaintext_utf8: str,
) -> Dict[str, Any]:
    if direction == "A->B":
        initiator = "A"
        responder = "B"
    elif direction == "B->A":
        initiator = "B"
        responder = "A"
    else:
        raise ValueError(f"Unsupported direction: {direction!r}")

    participants = [
        {"id": initiator, "role": "initiator"},
        {"id": responder, "role": "responder"},
    ]

    steps = [
        {
            "seq": 1,
            "actor": initiator,
            "op": "reset",
            "input": {"seed": {"type": "utf8", "data": seed}},
            "expect": {"ok": True},
        },
        {
            "seq": 2,
            "actor": responder,
            "op": "reset",
            "input": {"seed": {"type": "utf8", "data": seed}},
            "expect": {"ok": True},
        },
        {
            "seq": 3,
            "actor": initiator,
            "op": "handshake_init",
            "input": {
                "suite": {"type": "utf8", "data": suite},
                "options": {"type": "json", "data": {}},
            },
            "expect": {"ok": True},
        },
        {
            "seq": 4,
            "actor": responder,
            "op": "handshake_respond",
            "input": {
                "suite": {"type": "utf8", "data": suite},
                "msg1": {"type": "json", "data": {"from_step": 3, "field": "msg1_b64"}},
                "options": {"type": "json", "data": {}},
            },
            "expect": {"ok": True},
        },
        {
            "seq": 5,
            "actor": initiator,
            "op": "handshake_finish",
            "input": {
                "suite": {"type": "utf8", "data": suite},
                "msg2": {"type": "json", "data": {"from_step": 4, "field": "msg2_b64"}},
                "options": {"type": "json", "data": {}},
            },
            "expect": {"ok": True},
        },
        {
            "seq": 6,
            "actor": initiator,
            "op": "encrypt",
            "input": {
                "session_id": {"type": "json", "data": {"from_step": 5, "field": "session_id"}},
                "plaintext": {"type": "utf8", "data": plaintext_utf8},
            },
            "expect": {"ok": True},
        },
        {
            "seq": 7,
            "actor": responder,
            "op": "decrypt",
            "input": {
                "session_id": {"type": "json", "data": {"from_step": 5, "field": "session_id"}},
                "ciphertext": {"type": "json", "data": {"from_step": 6, "field": "ciphertext_b64"}},
            },
            "expect": {"ok": True},
        },
    ]

    assertions = [
        {"name": "session_established", "ok": True},
        {"name": "decrypt_matches_plaintext", "ok": True},
    ]

    return {
        "id": case_id,
        "participants": participants,
        "steps": steps,
        "assertions": assertions,
        "profile": f"{suite} {direction}",
        "ext": {
            "suite": suite,
            "direction": direction,
            "p3_case_id": p3_case_id,
        },
    }


def adapt_p3_04_handshake_smoke_to_interop_set(
    p3_cases: List[Dict[str, Any]],
    *,
    generated_at: str,
    phase3_member: str,
    plaintext_utf8: str = "interop-smoke",
) -> Tuple[Dict[str, Any], Dict[str, Any]]:
    """Return (interop_set, stats) for the currently-enforced subset.

    Today, Phase 4B enforces a smoke subset:
      - IT-HS-001 (Suite-1)
      - IT-HS-003 (Suite-1B)

    All other P3-04 catalog entries are tracked as coverage gaps.
    """
    required_prefixes = {"IT-HS-001", "IT-HS-003"}

    selected: List[Dict[str, Any]] = []
    skipped: List[str] = []

    for c in p3_cases:
        p3_id = str(c.get("p3_case_id", ""))
        if not any(p3_id.startswith(pref) for pref in required_prefixes):
            skipped.append(p3_id)
            continue
        suite = str(c.get("suite", ""))
        direction = str(c.get("direction", ""))

        # Fail closed: required smoke cases must declare suite and direction.
        if suite not in ("Suite-1", "Suite-1B"):
            raise ValueError(f"P3-04 smoke case missing/invalid suite for {p3_id!r}: {suite!r}")
        if direction not in ("A->B", "B->A"):
            raise ValueError(f"P3-04 smoke case missing/invalid direction for {p3_id!r}: {direction!r}")

        # Prefer upstream case_id if present and schema-safe; otherwise derive one deterministically.
        cid_raw = str(c.get("case_id", ""))
        cid = _sanitize_id(cid_raw)
        if not cid:
            cid = _interop_case_id(p3_case_id=p3_id, suite=suite, direction=direction)

        seed = f"p4b::{cid}"
        selected.append(
            make_handshake_smoke_case(
                case_id=cid,
                suite=suite,
                direction=direction,
                p3_case_id=p3_id,
                seed=seed,
                plaintext_utf8=plaintext_utf8,
            )
        )

    stats = {
        "selected_cases": len(selected),
        "skipped_catalog_entries": len(skipped),
        "skipped_p3_case_ids": skipped,
        "required_prefixes": sorted(required_prefixes),
    }

    interop_set = {
        "format": "QSHIELD-P4-INTEROP-SET-1",
        "schema_version": "1.0.0",
        "generated_at": generated_at,
        "protocol": {
            "protocol_version": "0x0403",
            "suite_id": "0x0000",
            "suite_name": "Mixed",
        },
        "cases": selected,
    }

    return interop_set, stats
