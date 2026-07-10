#!/usr/bin/env python3
"""NA-0628 (ENG-0034) — WF-0014 BYTE-SCAN: prove the existing Suite-2 vector set is unchanged.

Operator Decision 4 (D565): "a machine-asserted byte-scan proving every EXISTING inputs/suite2/**
vector is byte-identical, with sha256 cross-set guards. A prose 'vectors unchanged' note is not
acceptable. If any existing vector byte changes, that is a STOP — it means the guard rejects an
honest transcript."

The claim is PER-VECTOR, not per-file: a purely additive append necessarily changes a file's bytes
while changing no existing vector, and the STOP's own rationale ("the guard would be rejecting an
honest transcript") is a statement about transcripts, not about file offsets. This scan therefore
asserts, against `git show <BASE_REF>:<file>` (BASE_REF = the pinned pre-lane base 1fdd5b9b):

  1. no pre-existing vector id was removed;
  2. every pre-existing vector's canonical serialization is byte-identical (sha256 per vector);
  3. every inputs/suite2/** file OTHER than the two appended files is byte-identical at FILE level;
  4. the set of newly added ids is exactly the NA-0628 allowlist — nothing else may appear;
  5. cross-set guard: sha256 over the sorted per-vector hashes of the pre-existing set matches the
     baseline.

Exit non-zero on any violation. Run from the repo root.
"""
from __future__ import annotations

import hashlib
import os
import json
import subprocess
import sys
from pathlib import Path

VECTOR_GLOB = "inputs/suite2/**/*.json"

EXPECTED_NEW_IDS = {
    "S2-RECV-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001",
    "S2-SEND-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001",
}
EXPECTED_APPENDED_FILES = {
    "inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json",
    "inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json",
}

# Pinned to the lane's pre-lane base (the base at NA-0628 open, per D-1251), NOT the moving `main`
# ref: once this lane merges, `main` contains the two additions and a re-run against it would compare
# the tree to itself. A fixed commit keeps the byte claim reproducible for any later reviewer.
# Overridable for local dry-runs via NA0628_BYTE_SCAN_BASE.
BASE_REF = os.environ.get("NA0628_BYTE_SCAN_BASE", "1fdd5b9b347edc694e6ac29a8433f1c5acc286dd")


def sh(*args: str) -> bytes:
    return subprocess.run(args, check=True, capture_output=True).stdout


def vector_id(vector: dict, index: int) -> str:
    """Vector sets are not uniform: most use `id`, `qsc_handshake_suite_id_vectors_na0310.json` uses
    `vector_id`. Fall back to positional identity so no vector escapes the byte check."""
    for key in ("id", "vector_id"):
        if isinstance(vector, dict) and key in vector:
            return str(vector[key])
    return f"#index:{index}"


def canon(vector: dict) -> bytes:
    """Canonical bytes of one vector: key order and separators fixed, so the hash tracks CONTENT."""
    return json.dumps(vector, sort_keys=True, separators=(",", ":")).encode("utf-8")


def vhash(vector: dict) -> str:
    return hashlib.sha256(canon(vector)).hexdigest()


def fhash(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def baseline_file(rel: str) -> bytes | None:
    try:
        return sh("git", "show", f"{BASE_REF}:{rel}")
    except subprocess.CalledProcessError:
        return None


def main() -> int:
    failures: list[str] = []
    files = sorted(str(p) for p in Path().glob(VECTOR_GLOB))
    if not files:
        print("FATAL: no vector files found (run from the repo root)", file=sys.stderr)
        return 2

    all_base_hashes: list[str] = []
    all_head_hashes: list[str] = []
    new_ids: set[str] = set()
    checked_vectors = 0

    for rel in files:
        head_bytes = Path(rel).read_bytes()
        base_bytes = baseline_file(rel)
        if base_bytes is None:
            failures.append(f"{rel}: NEW FILE — not permitted by Decision 4 (additive cases only)")
            continue

        file_changed = head_bytes != base_bytes
        if file_changed and rel not in EXPECTED_APPENDED_FILES:
            failures.append(f"{rel}: FILE BYTES CHANGED but the file is not in the append allowlist")
            continue
        if not file_changed:
            # (3) untouched files: byte-identical at file level. Still hash their vectors for (5).
            pass

        base = json.loads(base_bytes.decode("utf-8"))
        head = json.loads(head_bytes.decode("utf-8"))
        base_vs = {vector_id(v, i): v for i, v in enumerate(base.get("vectors", []))}
        head_vs = {vector_id(v, i): v for i, v in enumerate(head.get("vectors", []))}

        # (1) nothing removed
        for vid in base_vs:
            if vid not in head_vs:
                failures.append(f"{rel}: vector {vid} was REMOVED")

        # (2) every pre-existing vector byte-identical, per-vector sha256
        for vid, bv in base_vs.items():
            if vid not in head_vs:
                continue
            bh, hh = vhash(bv), vhash(head_vs[vid])
            checked_vectors += 1
            all_base_hashes.append(bh)
            all_head_hashes.append(hh)
            if bh != hh:
                failures.append(
                    f"{rel}: EXISTING vector {vid} CHANGED\n"
                    f"    baseline sha256 {bh}\n"
                    f"    current  sha256 {hh}\n"
                    f"    STOP: the guard would be rejecting an honest transcript."
                )

        # (4) only the allowlisted new ids may appear
        for vid in head_vs:
            if vid not in base_vs:
                new_ids.add(vid)

        # metadata outside `vectors` must not drift
        base_meta = {k: v for k, v in base.items() if k != "vectors"}
        head_meta = {k: v for k, v in head.items() if k != "vectors"}
        if base_meta != head_meta:
            failures.append(f"{rel}: vector-set METADATA changed (outside the `vectors` array)")

    if new_ids != EXPECTED_NEW_IDS:
        unexpected = new_ids - EXPECTED_NEW_IDS
        missing = EXPECTED_NEW_IDS - new_ids
        if unexpected:
            failures.append(f"UNEXPECTED new vector ids: {sorted(unexpected)}")
        if missing:
            failures.append(f"MISSING expected new vector ids: {sorted(missing)}")

    # (5) cross-set guard over the pre-existing set
    base_cross = fhash("".join(sorted(all_base_hashes)).encode("utf-8"))
    head_cross = fhash("".join(sorted(all_head_hashes)).encode("utf-8"))
    if base_cross != head_cross:
        failures.append(
            f"CROSS-SET GUARD FAILED\n    baseline {base_cross}\n    current  {head_cross}"
        )

    print(f"WF-0014 byte-scan — base ref: {BASE_REF}")
    print(f"  vector files scanned            : {len(files)}")
    print(f"  pre-existing vectors byte-checked: {checked_vectors}")
    print(f"  cross-set sha256 (pre-existing) : {head_cross}")
    print(f"  files appended to (allowlisted)  : {sorted(EXPECTED_APPENDED_FILES)}")
    print(f"  new vector ids                   : {sorted(new_ids)}")

    if failures:
        print("\nBYTE-SCAN FAILED — this is a STOP:\n", file=sys.stderr)
        for f in failures:
            print(f"  - {f}", file=sys.stderr)
        return 2

    print("\nBYTE-SCAN PASSED: every pre-existing vector is byte-identical; additions are exactly")
    print("the two NA-0628 negative vectors; no existing file's bytes changed outside the allowlist.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
