#!/usr/bin/env python3
"""NA-0627 / ENG-0028 (D564): fail-closed ProVerif gate for the Suite-2 DH+PQ composition.

Goals: G1, G2, G4

Design-lock Decision 2: the gate asserts, PER QUERY, the exact expected `RESULT` line — never
merely a zero exit code. A green gate that cannot fail is not a gate.

The TOOL SANITY PAIR is the FIRST assertion (D564, binding): a positive control that must prove
(`is true.`) and a negative control that must REFUTE (`is false.`). If the negative control ever
returns `true`, the verifier is vacuously accepting and this script exits non-zero before any
protocol model runs.

Some expected results are deliberately `is false.`:
  * the negative sanity control (the tool must be able to refute);
  * the healing models' CANARIES (`m0`): under the modeled compromise the PRE-heal traffic MUST
    be readable, otherwise the healing `is true.` beside it would be vacuous;
  * the healing models' EXPECTED-RED PROBES (`m_rs`, `m_cb`): a reseed/combined boundary frame's
    OWN body rides the PRE-reseed key schedule (DOC-CAN-003 §8.5.3; ratchet.rs:1752-1758,
    :1908-1911), so under full classical compromise the boundary message itself is readable. The
    reseed protects the messages AFTER it. Asserting these RED is what proves the model actually
    models the compromise.

Usage:
    python3 formal/proverif/run_proverif_checks.py [--proverif /path/to/proverif]

Exits 0 iff every expected RESULT line below is present in the corresponding run's output.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE / "suite2_dhpq_lib.pvl"

# The tool sanity pair is written to a temp dir at run time: it must be the FIRST thing checked,
# and it must not depend on any file the protocol models could accidentally shadow.
SANITY_POS = """free c: channel.
free s: bitstring [private].
query attacker(s).
process 0
"""

SANITY_NEG = """free c: channel.
free s: bitstring [private].
query attacker(s).
process out(c, s)
"""

# (model filename, [expected RESULT-line regexes])
#
# Each pattern is matched (re.search) against the model's `RESULT ...` lines. Regexes, not plain
# substrings, because ProVerif renames bound query variables (`n_28`, ...) as a model evolves; an
# assertion keyed on a generated name would fail for the wrong reason. The SEMANTIC content of
# each line -- which event implies which, and `is true.` vs `is false.` -- is pinned exactly.
MODELS: list[tuple[str, list[str]]] = [
    (
        "suite2_dhpq_main.pv",
        [
            # Q1 message-key secrecy (active Dolev-Yao adversary, no compromise)
            r"^RESULT not attacker\(m1\[\]\) is true\.$",
            r"^RESULT not attacker\(m2\[\]\) is true\.$",
            r"^RESULT not attacker\(m4\[\]\) is true\.$",
            # Q2 injective agreement on the session transcript
            r"^RESULT inj-event\(MsgAcc\(.*\)\) ==> inj-event\(MsgSent\(.*\)\) is true\.$",
            # Q6 control plane: a planted/replayed advertisement is never tracked
            r"^RESULT inj-event\(AdvTracked\(.*\)\) ==> inj-event\(AdvSent\(.*\)\) is true\.$",
            # Q7 guard-form, DH-boundary arm (NOT an attack-existence proof — see the model)
            r"^RESULT not event\(BoundaryAccepted\(zeroG\)\) is true\.$",
        ],
    ),
    (
        "suite2_dhpq_q3_pq_reseed_healing.pv",
        [
            # Q3 PQ healing after a PQ reseed, all classical secrets + root snapshot leaked
            r"^RESULT not attacker_p2\(secret_q3\[\]\) is true\.$",
            # canary: the compromise is real
            r"^RESULT not attacker_p2\(m0\[\]\) is false\.$",
            # expected-red probe: the reseed frame's own body rides the pre-reseed schedule
            r"^RESULT not attacker_p2\(m_rs\[\]\) is false\.$",
        ],
    ),
    (
        "suite2_dhpq_q4_combined_healing.pv",
        [
            # Q4 PQ healing after the NA-0626 combined DH+PQ boundary
            r"^RESULT not attacker_p2\(secret_q4\[\]\) is true\.$",
            r"^RESULT not attacker_p2\(m0\[\]\) is false\.$",
            r"^RESULT not attacker_p2\(m_cb\[\]\) is false\.$",
            # Q7 guard-form, combined-boundary arm
            r"^RESULT not event\(BoundaryAccepted\(zeroG\)\) is true\.$",
        ],
    ),
    (
        "suite2_dhpq_q5_dh_healing.pv",
        [
            # Q5 classical healing after a DH boundary, ML-KEM decap key + root snapshot leaked
            r"^RESULT not attacker_p2\(secret_q5\[\]\) is true\.$",
            r"^RESULT not attacker_p2\(m0\[\]\) is false\.$",
        ],
    ),
]


def run(proverif: str, model: pathlib.Path, use_lib: bool = True) -> str:
    cmd = [proverif]
    if use_lib:
        cmd += ["-lib", str(LIB)]
    cmd += [str(model)]
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=3600)
    out = proc.stdout + proc.stderr
    if proc.returncode != 0:
        sys.stderr.write(f"\n[FAIL] {model.name}: proverif exited {proc.returncode}\n{out}\n")
        raise SystemExit(1)
    return out


def check(name: str, out: str, expected: list[str]) -> list[str]:
    """Match each expected regex against the run's `RESULT ...` lines. Fail-closed on any miss."""
    result_lines = [ln.strip() for ln in out.splitlines() if ln.startswith("RESULT")]
    missing = [pat for pat in expected if not any(re.search(pat, ln) for ln in result_lines)]
    for pat in expected:
        status = "MISSING" if pat in missing else "ok"
        print(f"  [{status:>7}] {pat}")
    if missing:
        sys.stderr.write(f"\n[FAIL] {name}: {len(missing)} expected RESULT line(s) absent.\n")
        sys.stderr.write("----- RESULT lines seen -----\n")
        sys.stderr.write("\n".join(result_lines) + "\n" if result_lines else "(none)\n")
    return missing


def sanity_pair(proverif: str) -> None:
    """D564 binding: the verifier must PROVE and must REFUTE. Run before any protocol model."""
    print("== tool sanity pair (FIRST assertion; a verifier that only accepts is worse than none)")
    with tempfile.TemporaryDirectory() as td:
        tdp = pathlib.Path(td)
        pos, neg = tdp / "sanity_pos.pv", tdp / "sanity_neg.pv"
        pos.write_text(SANITY_POS)
        neg.write_text(SANITY_NEG)
        pos_out = run(proverif, pos, use_lib=False)
        neg_out = run(proverif, neg, use_lib=False)

    failures = []
    failures += check("sanity_pos", pos_out, [r"^RESULT not attacker\(s\[\]\) is true\.$"])
    # THE LOAD-BEARING ONE: if this ever reports `true`, the gate is lying.
    failures += check("sanity_neg", neg_out, [r"^RESULT not attacker\(s\[\]\) is false\.$"])
    if failures:
        sys.stderr.write(
            "\n[STOP] The tool sanity pair failed. If the NEGATIVE control returned `true`, the\n"
            "       verifier is vacuously accepting and every green below would be meaningless.\n"
        )
        raise SystemExit(1)
    print("  sanity pair OK: the tool both proves and refutes.\n")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--proverif", default=shutil.which("proverif") or "proverif")
    args = ap.parse_args()

    if not shutil.which(args.proverif) and not pathlib.Path(args.proverif).exists():
        sys.stderr.write(f"[STOP] proverif not invocable: {args.proverif}\n")
        return 1

    ver = subprocess.run([args.proverif, "-help"], capture_output=True, text=True)
    banner = (ver.stdout + ver.stderr).splitlines()[0] if (ver.stdout or ver.stderr) else ""
    print(f"== tool: {banner}")
    if "2.05" not in banner:
        sys.stderr.write(f"[STOP] expected ProVerif 2.05 (D564 pin), got: {banner!r}\n")
        return 1
    print()

    sanity_pair(args.proverif)

    failures: list[str] = []
    for name, expected in MODELS:
        print(f"== {name}")
        out = run(args.proverif, HERE / name)
        failures += check(name, out, expected)
        print()

    if failures:
        sys.stderr.write(f"[FAIL] {len(failures)} expected RESULT line(s) missing overall.\n")
        return 1
    print("All expected ProVerif RESULT lines present (fail-closed gate green).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
