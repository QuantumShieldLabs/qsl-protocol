#!/usr/bin/env python3
"""Operator-infrastructure literal gate.

Pattern provenance ("provenance over reinvention"): adapted 2026-07-25 from the
operator-side `added_line_publication_scan.py`, whose class vocabulary this keeps
so old lane records and new CI output speak one language. That tool is the
historical source; THIS file is the source of truth for the patterns.

REVIEWABILITY: the name values below are salted digests; THE PLAINTEXT LIST IS
OPERATOR-HELD, together with the procedure for regenerating these digests. What
is reviewable here is the tier structure, the class labels and the structural
regexes; what is deliberately not reviewable here is which names the digests
stand for. That trade is the point of the next section.

WHY THIS EXISTS
---------------
The spine's public-safety gate scans for private keys and cloud tokens. It has
never contained an address, path or host pattern -- which is exactly why it ran
green on every pull request that published a private LAN address. The failure was
the pattern set, not the scan's scope. This is the missing pattern set, wired as a
gate instead of a habit.

TIERS
-----
TIER 1   whole tracked tree, fail on any hit.
         Network-identifying literals and personal identity.
TIER 2b  ADDED LINES of the diff only, fail on any hit.
         Low-frequency private names: pre-existing occurrences are left alone,
         every NEW one fails.
TIER 2a  NOT SCANNED, deliberately. The build-root and home paths are added by
         ~60% of governance commits, because the convention cites directives and
         lane intents by absolute path. A gate on them is unadoptable and would be
         switched off within a week. They are published as recorded residue
         instead.

WHY THE PRIVATE NAMES ARE DIGESTS AND NOT TEXT
----------------------------------------------
This file is committed to PUBLIC repositories. A pattern file spelling out the
private host names would republish, in every repo it lands in, exactly what the
sanitize lane removed -- and the Tier-1 whole-tree scan would then hit its own
pattern file, so the gate would fail itself on the day it landed. The names are
stored as salted SHA-256 digests: the gate still recognises them, and reading this
file tells you nothing about what they are.

The salt is present here because it has to be: the scanner hashes candidate tokens
at scan time and cannot compare them against the stored digests without it. Its job
is narrow and worth stating honestly -- it makes this list useless as a PRECOMPUTED
rainbow-table target, and it is not, and cannot be, a secret from anyone who
already knows a name. The protection that actually matters is that the names
themselves are not in this tree.

Structural classes keep literal regexes, because they describe a SHAPE rather than
a name and disclose nothing: RFC-1918 address forms, a mail-provider domain, and
the tailnet-hostname form.

ANCHORING -- NO NAKED WORD BOUNDARIES
-------------------------------------
Name matching is TOKEN-WISE and CASE-INSENSITIVE. A `\\bname\\b` pattern does not
match inside `HOST_NAME`, because the underscore is a word character and kills the
left boundary -- and that is not hypothetical: a previous lane's own closeout draft
named a pattern class after the host it was redacting, which republished the
literal to a human reader while being invisible to a word-boundary scanner. It was
found by a case-insensitive human read, not by the tool.

So: `FOO_name`, `name_backup`, `NAME`, and `name-lan-relay` all hit. A quiet gate
that cannot see the thing it exists for is worse than no gate, because it is
believed.

Matching is on TOKENS, where a token boundary is any non-alphanumeric character OR
a camelCase transition -- an identifier is treated as a compound word and split
into its parts. `HOST_<name>` -> [HOST, <name>]; `SOME_<name>_THING` ->
[SOME, <name>, THING]; `<name>-lan-relay` -> [<name>, lan, relay]. All hit.

This is NOT a word boundary reintroduced under another name, and the distinction
matters. A `\b` pattern fails on `HOST_<name>` -- the case this gate exists to catch.
Raw substring matching, tried first and rejected here, catches that but also fires
on every identifier that merely SPANS a camelCase seam: a 7-character host name
sits inside `setServerBusy` and `commitServerSettings`, which flooded the first
control run with false positives from ordinary source code. Splitting on the seam
keeps every embedded case and drops the spanning ones.

Residual, stated rather than hidden: a name written with no delimiter and no case
change (`mynamehost`) is one token and will not match. That is a narrower gap than
the alternative's noise, and it is not a realistic way a host name gets written.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import subprocess
import sys

SALT = b"qsl-infra-literal-scan-v1|"

TIER1_STRUCTURAL = {
    "private_ipv4_192": re.compile(r"\b192\.168\.\d{1,3}\.\d{1,3}\b"),
    "private_ipv4_172": re.compile(r"\b172\.(?:1[6-9]|2\d|3[01])\.\d{1,3}\.\d{1,3}\b"),
    "private_ipv4_10": re.compile(r"\b10\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"),
    "personal_email": re.compile(r"[A-Za-z0-9._%+-]+@proton\.me\b", re.IGNORECASE),
    "tailnet_host": re.compile(r"\b[a-z0-9-]+\.[a-z]{4}[a-z0-9]{6}\.ts\.net\b", re.IGNORECASE),
}

TIER1_NAME_DIGESTS = {
    "7dc0c00110f4890e287392ee2fa0b4dc4cfc4c36acf5d17d2b969c06b8a68aee": "host_relay",
    "fc30453cef93d0847a563faa4b130b604025d74340166e98d98751815741b21e": "host_build",
    "492ec5762c6224cbcf55c577cb4406f4841f7ea9acf30521e6ba89bf8a950f4c": "host_laptop",
    "c9e6900807b68c9750ac2212681c5ab4b7bf0a38e6985250b6cdae585099ea17": "ssh_alias",
}

TIER2B_NAME_DIGESTS = {
    "41a3d1f2115a6fe3b80d2a78da13326a9c5e772391013bfa32c89a400a5e8944": "remote_account_a",
    "b3bd022a7eda33a356bc0620df4e67420289a8bc16c0289707aff2179ed41955": "remote_account_b",
    "5b81c919091280c902ef074e420f6c559bb239c1fcd4a1a3affa5b99eeb4be9e": "host_retired_rig",
}

# Token = a maximal alphanumeric run, further split at camelCase transitions.
_ALNUM_RUN = re.compile(r"[A-Za-z0-9]+")
_CAMEL_SPLIT = re.compile(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])")
_BINARY_HINT = b"\0"


def _digest(text: str) -> str:
    return hashlib.sha256(SALT + text.lower().encode("utf-8")).hexdigest()


class _NameMatcher:
    """Token-wise, case-insensitive, digest-compared.

    Tokens come from splitting on any non-alphanumeric character and on camelCase
    transitions, so a name embedded in an identifier is found while a name that
    merely spans a camelCase seam is not. A length filter keeps the common case to
    a single dict lookup.
    """

    def __init__(self, digests: dict[str, str], lengths: set[int]):
        self.digests = digests
        self.lengths = lengths

    def hits(self, line: str) -> list[str]:
        found: list[str] = []
        for match in _ALNUM_RUN.finditer(line):
            for token in _CAMEL_SPLIT.split(match.group(0)):
                if len(token) not in self.lengths:
                    continue
                cls = self.digests.get(_digest(token))
                if cls and cls not in found:
                    found.append(cls)
        return found


# Lengths are a derived fact about the digested names: they narrow the search and,
# like the digests, disclose nothing about the names themselves.
TIER1_MATCHER = _NameMatcher(TIER1_NAME_DIGESTS, {7, 8, 9, 10})
TIER2B_MATCHER = _NameMatcher(TIER2B_NAME_DIGESTS, {7, 8})


def _tracked_files() -> list[str]:
    out = subprocess.run(
        ["git", "ls-files", "-z"], capture_output=True, check=True
    ).stdout
    return [p.decode("utf-8", "replace") for p in out.split(b"\0") if p]


def _added_lines(base: str) -> list[tuple[str, str]]:
    """(path, line) for every ADDED line in the diff against `base`."""
    out = subprocess.run(
        ["git", "diff", "-U0", f"{base}...HEAD"], capture_output=True, check=False
    ).stdout.decode("utf-8", "replace")
    results: list[tuple[str, str]] = []
    path = "?"
    for line in out.splitlines():
        if line.startswith("+++ b/"):
            path = line[6:]
        elif line.startswith("+") and not line.startswith("+++"):
            results.append((path, line[1:]))
    return results


def _staged_lines() -> list[tuple[str, str]]:
    out = subprocess.run(
        ["git", "diff", "--cached", "-U0"], capture_output=True, check=False
    ).stdout.decode("utf-8", "replace")
    results: list[tuple[str, str]] = []
    path = "?"
    for line in out.splitlines():
        if line.startswith("+++ b/"):
            path = line[6:]
        elif line.startswith("+") and not line.startswith("+++"):
            results.append((path, line[1:]))
    return results


def _scan_line(line: str, tier1: bool, tier2b: bool) -> list[str]:
    classes: list[str] = []
    if tier1:
        for name, rx in TIER1_STRUCTURAL.items():
            if rx.search(line):
                classes.append(name)
        classes.extend(TIER1_MATCHER.hits(line))
    if tier2b:
        classes.extend(TIER2B_MATCHER.hits(line))
    return classes


def _report(hits: list[tuple[str, int, str, str]], tier: str) -> None:
    for path, lineno, cls, excerpt in hits:
        print(f"{path}:{lineno}: [{tier}:{cls}] {excerpt[:120]}")


def _fail_message() -> None:
    print("", file=sys.stderr)
    print("An operator-infrastructure literal was found.", file=sys.stderr)
    print("", file=sys.stderr)
    print("These are private host names, addresses, accounts or personal", file=sys.stderr)
    print("identifiers, and they must not appear in a published tree.", file=sys.stderr)
    print("", file=sys.stderr)
    print("WHAT TO DO: replace the literal with a descriptive placeholder that", file=sys.stderr)
    print("preserves the sentence's meaning -- for example <lan-address> or", file=sys.stderr)
    print("<host-name>. Do NOT substitute a different plausible-looking literal:", file=sys.stderr)
    print("inside a quotation that would fabricate an observation nobody made.", file=sys.stderr)
    print("", file=sys.stderr)
    print("If the hit is in a record ABOUT a redaction, name the FIELD rather", file=sys.stderr)
    print("than the value -- a redaction record written naively re-leaks what it", file=sys.stderr)
    print("redacts.", file=sys.stderr)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--mode",
        choices=["tree", "diff", "staged"],
        default="tree",
        help="tree: Tier 1 over all tracked files. "
        "diff: Tier 1 + Tier 2b over added lines vs --base. "
        "staged: Tier 1 + Tier 2b over staged added lines (pre-commit).",
    )
    ap.add_argument("--base", default="origin/main", help="diff mode base ref")
    args = ap.parse_args()

    hits: list[tuple[str, int, str, str]] = []
    files_seen = 0
    lines_seen = 0

    if args.mode == "tree":
        for path in _tracked_files():
            try:
                with open(path, "rb") as fh:
                    raw = fh.read()
            except OSError:
                continue
            if _BINARY_HINT in raw:
                continue
            files_seen += 1
            for lineno, line in enumerate(
                raw.decode("utf-8", "replace").splitlines(), start=1
            ):
                lines_seen += 1
                for cls in _scan_line(line, tier1=True, tier2b=False):
                    hits.append((path, lineno, cls, line))
        tier = "tier1"
    else:
        pairs = _staged_lines() if args.mode == "staged" else _added_lines(args.base)
        files_seen = len({path for path, _ in pairs})
        lines_seen = len(pairs)
        for path, line in pairs:
            for cls in _scan_line(line, tier1=True, tier2b=True):
                hits.append((path, 0, cls, line))
        tier = "added-line"

    # Report WHAT WAS EXAMINED, not merely that nothing was found. A scan that
    # inspected zero lines prints "clean" just as loudly as one that inspected
    # thousands, and a green that cannot be distinguished from a no-op is the
    # exact defect this gate was built to answer. The counts make a vacuous pass
    # visible in the CI log.
    scope = f"{files_seen} files, {lines_seen} lines examined"

    if hits:
        _report(hits, tier)
        print(f"infra-literal-scan: FAILED ({args.mode}; {scope})", file=sys.stderr)
        _fail_message()
        return 1

    # Refuse an empty input in the modes where empty means BROKEN, and only
    # those. In `tree` mode zero files means the checkout or `git ls-files` is
    # wrong; in `diff` mode zero added lines against a base almost always means
    # the base ref was not fetched, which is how a CI gate silently becomes a
    # no-op. In `staged` mode, by contrast, an empty added-line set is perfectly
    # legitimate -- a deletion-only or rename-only commit has no added lines --
    # and refusing there would block honest commits from the pre-commit hook.
    if lines_seen == 0 and args.mode in ("tree", "diff"):
        print(
            f"infra-literal-scan: NOTHING EXAMINED ({args.mode}) -- refusing to "
            "report a pass over an empty input. In diff mode this usually means "
            "the base ref was not fetched.",
            file=sys.stderr,
        )
        return 2

    print(f"infra-literal-scan: clean ({args.mode}; {scope})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
