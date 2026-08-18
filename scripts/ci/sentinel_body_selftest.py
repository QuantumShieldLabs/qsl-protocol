#!/usr/bin/env python3
"""Gate for the text `main-red-sentinel` publishes to a PUBLIC issue tracker.

WF-0074 tranche 1 item B / R319. Item B's issue body is the first text in this
program published AUTOMATICALLY to a public surface with no gate of any kind
between the program and the world. The SR-15 cold read asked which existing gate
would catch operator infrastructure reaching a public issue and answered: NONE.
`infra-literal-scan` examines REPO FILES; nothing examines runtime-generated
issue bodies. This file is that missing gate.

⚠ WHAT IT CONVERTS. Before it, the body was safe because of what it OMITS, and
the only thing holding that line was a comment in the workflow. A future edit --
the single most obvious feature request this issue will attract is "it should say
WHY it failed", i.e. paste a log excerpt -- would breach the repository's own
publication rule with nothing able to catch it. Now the omission discipline is
held by a test: widen the body, and this refuses you.

⚠ IT EXECUTES THE TEMPLATE, IT DOES NOT MODEL IT. The body-building bytes are
extracted from `.github/workflows/main-red-sentinel.yml` between two anchor
comments and run under bash with fixture values in the environment. A gate that
re-implemented the printf in Python would drift from the workflow silently and
pass while the real body leaked. If the anchors or the MARKER assignment are
missing, this exits 2 rather than reporting a pass over nothing.

⚠ THE NEEDLES ARE INFRA-LITERAL-SCAN'S OWN. `_scan_line` is imported from
scripts/ci/infra_literal_scan.py, so the two gates can never disagree about what
an operator literal is, and a class added there is enforced here for free.

⚠ WHY THIS FILE CARRIES NO OPERATOR LITERAL. It lives in the tree the Tier-1 scan
examines, so a red control written the obvious way -- paste the literal, assert
the scan finds it -- would make that gate fail on this file. Every needle here is
ASSEMBLED AT RUN TIME from fragments that never appear contiguously in this
source. Same discipline as infra_literal_scan_selftest.py.

Run: python3 scripts/ci/sentinel_body_selftest.py
Exit 0 = all checks passed. 1 = a check failed. 2 = the instrument examined nothing.
"""

from __future__ import annotations

import importlib.util
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
WORKFLOW = os.path.join(REPO, ".github", "workflows", "main-red-sentinel.yml")
SCANNER = os.path.join(HERE, "infra_literal_scan.py")

BEGIN = "# >>> BODY RENDER"
END = "# <<< BODY RENDER"

FAILURES: list[str] = []
CHECKS = 0


def check(name: str, ok: bool, detail: str = "") -> None:
    global CHECKS
    CHECKS += 1
    if ok:
        print(f"  ok    {name}")
    else:
        print(f"  FAIL  {name}")
        if detail:
            for line in detail.strip().splitlines()[:12]:
                print(f"          {line}")
        FAILURES.append(name)


def refuse(message: str) -> None:
    """Exit 2 -- "the instrument examined nothing".

    Deliberately distinct from exit 1, "a literal was found". Both refuse, but a CI
    reader who cannot tell them apart will read a broken gate as a leak, or worse,
    read a leak as a broken gate. Same convention as infra_literal_scan.py.
    """
    print(message, file=sys.stderr)
    raise SystemExit(2)


def load_scanner():
    spec = importlib.util.spec_from_file_location("infra_literal_scan", SCANNER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def extract_render_block(text: str) -> tuple[str, str]:
    """The MARKER assignment and the anchored BODY block, as real bytes.

    Both are taken from the workflow file itself. Returning them separately keeps
    the failure messages specific about which one went missing.
    """
    marker_lines = [ln for ln in text.splitlines() if re.match(r"\s*MARKER=", ln)]
    if len(marker_lines) != 1:
        refuse(
            f"NOTHING EXAMINED -- expected exactly one MARKER= assignment in "
            f"{WORKFLOW}, found {len(marker_lines)}. The de-duplication key moved; "
            f"this gate refuses to report a pass it cannot substantiate."
        )
    if text.count(BEGIN) != 1 or text.count(END) != 1:
        refuse(
            f"NOTHING EXAMINED -- the '{BEGIN}' / '{END}' anchors must appear exactly "
            f"once each in {WORKFLOW}. Without them this gate cannot render the body, "
            f"and a gate that cannot render refuses rather than passes."
        )
    body = text.split(BEGIN, 1)[1].split(END, 1)[0]
    body = "\n".join(body.splitlines()[1:])  # drop the rest of the anchor comment
    return marker_lines[0], body


def render(marker_line: str, body_block: str, fields: dict[str, str]) -> str:
    """Run the workflow's own body-building bytes with fixture values."""
    script = f"set -u\n{marker_line}\n{body_block}\nprintf '%s' \"$BODY\"\n"
    proc = subprocess.run(
        ["bash", "-c", script],
        capture_output=True,
        text=True,
        env={**os.environ, **fields},
    )
    if proc.returncode != 0:
        refuse(
            f"NOTHING EXAMINED -- the extracted body block did not execute "
            f"(exit {proc.returncode}): {proc.stderr.strip()[:400]}"
        )
    return proc.stdout


def fields(name="macos-build", conclusion="failure", url="", sha="") -> dict[str, str]:
    return {
        "WF_NAME": name,
        "WF_CONCLUSION": conclusion,
        "WF_URL": url or "https://github.com/o/r/actions/runs/12345678901",
        "WF_SHA": sha or "0123456789abcdef0123456789abcdef01234567",
    }


# ---------------------------------------------------------------------------
# THE NEEDLES, assembled so this file never contains them contiguously.
# ---------------------------------------------------------------------------

def rfc1918() -> str:
    return "host-" + "192." + "168." + "1.50" + "-probe"


def personal_mail() -> str:
    return "someone" + "@" + "proton" + ".me"


def cgnat() -> str:
    return "https://" + "100." + "99." + "234.5" + ":8443/v1/inbox"


def ddns() -> str:
    return "probe-host." + "ddns" + "free" + ".com"


# The 13 workflows the sentinel watches, read from the file rather than retyped:
# a name added there must be exercised here without anyone remembering to.
def watched(text: str) -> list[str]:
    block = text.split("workflows:", 1)[1].split("types:", 1)[0]
    return [ln.strip()[2:] for ln in block.splitlines() if ln.strip().startswith("- ")]


def main() -> int:
    print("main-red-sentinel body gate (R319)")
    scanner = load_scanner()
    if not os.path.exists(WORKFLOW):
        # A PR that deletes the sentinel must not make this gate crash with a
        # traceback that reads like a broken test rather than a missing subject.
        refuse(
            f"NOTHING EXAMINED -- {WORKFLOW} does not exist, so there is no issue body to "
            f"render. If the sentinel was removed deliberately, remove this gate in the "
            f"same change; until then this refuses rather than reports a pass."
        )
    with open(WORKFLOW, encoding="utf-8") as fh:
        text = fh.read()
    marker_line, body_block = extract_render_block(text)

    def hits(body: str) -> list[str]:
        found: list[str] = []
        for line in body.splitlines():
            for cls in scanner._scan_line(line, tier1=True, tier2b=True):
                if cls not in found:
                    found.append(cls)
        return found

    # -------------------------------------------------------------------
    print("\n[1] the RED control: an infra literal in ANY field is REFUSED")
    # -------------------------------------------------------------------
    # One class per field, so a body that widened to carry any of the four
    # cannot pass. A gate only ever watched green is not known to be a gate.
    for label, fld, needle, cls in (
        ("private IPv4 in the workflow name", "WF_NAME", rfc1918(), "private_ipv4_192"),
        ("personal mail domain in the conclusion", "WF_CONCLUSION", personal_mail(), "personal_email"),
        ("CGNAT address in the run URL", "WF_URL", cgnat(), "tailnet_cgnat"),
        ("public dynamic-DNS host in the sha", "WF_SHA", ddns(), "public_ddns_host"),
    ):
        f = fields()
        f[fld] = needle
        got = hits(render(marker_line, body_block, f))
        check(f"{label}: REFUSED and the class is NAMED", cls in got, f"classes={got}")

    # -------------------------------------------------------------------
    print("\n[2] every watched workflow renders CLEAN")
    # -------------------------------------------------------------------
    names = watched(text)
    check("the watch list was parsed from the workflow file", len(names) == 13, f"{names}")
    for name in names:
        got = hits(render(marker_line, body_block, fields(name=name)))
        check(f"{name}: no infra literal in the rendered body", got == [], f"classes={got}")

    # -------------------------------------------------------------------
    print("\n[3] adversarial field values still render CLEAN")
    # -------------------------------------------------------------------
    # The body carries four fields and no branch, log, actor or job name. The
    # ruling's adversarial classes are therefore applied to the fields that
    # EXIST, which is the honest mapping and is stated rather than quietly
    # narrowed: a hostile ref name and an empty log arrive here as a hostile
    # string and an empty string in the fields that would carry them.
    adversarial = {
        "empty (every field empty, the 'empty log' case)": fields("", "", " ", " "),
        "hostile shell metacharacters": fields("$(id) `whoami` ;rm -rf / ${IFS}"),
        "hostile ref name": fields("refs/heads/../../etc/passwd\\n$(curl x)"),
        "markdown and comment breakout": fields("--> <!-- [x](http://a) <img src=y>"),
        "unicode, RTL override and combining marks": fields("wf-‮gnṕ\U0001f4a5-中文"),
        "very long name (5000 chars)": fields("a" * 5000),
        "newline injection into the body": fields("first\nsecond\n- run: http://x"),
    }
    for label, f in adversarial.items():
        got = hits(render(marker_line, body_block, f))
        check(f"{label}: renders clean", got == [], f"classes={got}")

    # -------------------------------------------------------------------
    print("\n[4] the de-duplication marker actually reaches the body")
    # -------------------------------------------------------------------
    # R318.3 keys dedup on this marker. If it stopped being emitted the sentinel
    # would open a new issue per failure -- the 120-issue flood it exists to
    # prevent -- and every other check here would still pass.
    body = render(marker_line, body_block, fields(name="macos-build"))
    check("the rendered body carries the watch marker", "main-red-sentinel:watch=macos-build" in body, body[-300:])
    check("the rendered body is not empty", len(body.strip()) > 0, repr(body[:200]))

    print(f"\n{CHECKS} checks, {len(FAILURES)} failed")
    if FAILURES:
        print("FAILED: " + ", ".join(FAILURES))
        return 1
    if CHECKS == 0:
        print("NOTHING EXAMINED -- refusing to report a pass over zero checks")
        return 2
    print("main-red-sentinel body gate: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
