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

# NA-0686 (ENG-0089): TIER 2b classes that describe a SHAPE. They sit in Tier 2b
# rather than Tier 1 because the tree carries pre-existing, deliberately-LEFT
# occurrences of both -- dated records that report what was true when they were
# written (D-1322's property). Added-line semantics are exactly right for that:
# the history stays, and every NEW one fails.
#
# Neither pattern can match its own source text: the dots are escaped here
# (`\.`), so the literal sequences these regexes look for do not appear in this
# file. That is the same trick the Tier-1 mail-domain and tailnet-host patterns
# use, and it is load-bearing -- a pattern file that matched itself would fail
# the gate on the day it landed.
TIER2B_STRUCTURAL = {
    # CGNAT / tailnet address space (100.64/10). NA-0684 found 40 occurrences
    # this gate could not see: the Tier-1 structural set covers RFC-1918, the
    # tailnet HOSTNAME form and the mail domain, but `100.64/10` matched nothing.
    "tailnet_cgnat": re.compile(
        r"\b100\.(?:6[4-9]|[7-9]\d|1[01]\d|12[0-7])\.\d{1,3}\.\d{1,3}\b"
    ),
    # The public dynamic-DNS domain the two retired operator names lived under.
    # Publishing the DOMAIN discloses nothing the tree does not already carry in
    # its dated records, and it names a provider rather than a host.
    "public_ddns_host": re.compile(r"\b[a-z0-9-]+\.ddnsfree\.com\b", re.IGNORECASE),
}

TIER1_NAME_DIGESTS = {
    "7dc0c00110f4890e287392ee2fa0b4dc4cfc4c36acf5d17d2b969c06b8a68aee": "host_relay",
    "fc30453cef93d0847a563faa4b130b604025d74340166e98d98751815741b21e": "host_build",
    "492ec5762c6224cbcf55c577cb4406f4841f7ea9acf30521e6ba89bf8a950f4c": "host_laptop",
    "c9e6900807b68c9750ac2212681c5ab4b7bf0a38e6985250b6cdae585099ea17": "ssh_alias",
    # NA-0686 (ENG-0089): PROMOTED from Tier 2b to Tier 1, tree-wide.
    #
    # The finding was that a class firing on ADDED LINES ONLY means the tree is
    # only as clean as its last edit -- `--mode tree` reported clean while the
    # literal sat on `main`, and any lane that touched such a line inherited a
    # gate failure it did not create. The promotion was sequenced AFTER the two
    # sanitization lanes precisely so it could not turn a gate red on already
    # published content.
    #
    # Its pre-existing population is met as a per-path expected-count baseline
    # (TIER1_TREE_ALLOWLIST below), not waved through wholesale.
    "5b81c919091280c902ef074e420f6c559bb239c1fcd4a1a3affa5b99eeb4be9e": "host_retired_rig",
}

TIER2B_NAME_DIGESTS = {
    "41a3d1f2115a6fe3b80d2a78da13326a9c5e772391013bfa32c89a400a5e8944": "remote_account_a",
    "b3bd022a7eda33a356bc0620df4e67420289a8bc16c0289707aff2179ed41955": "remote_account_b",
}

# ---------------------------------------------------------------- the allowlist
#
# NA-0686 (ENG-0089): the Tier-1 `host_retired_rig` promotion meets a KNOWN
# population rather than discovering one. Two censuses fed it -- NA-0684's
# (the historical proof labels, the 10 tracked paths whose NAMES carry the
# token, and the dated-record class) and NA-0685's -- and it is expressed as a
# PER-PATH EXPECTED COUNT, not a per-path exemption.
#
# ⚠ WHY COUNTS AND NOT JUST PATHS. A path-only allowlist would let a NEW
# occurrence into an already-listed file silently, which reintroduces the exact
# hole this finding was filed about: "the tree is only as clean as its last
# edit". With counts, a file may keep the occurrences it has and may LOSE them
# (sanitization is always welcome), but it may not GAIN one. That is Option B
# -- "any line a lane re-adds carries no such literal; grandfathered lines
# stay" -- enforced as a tree invariant instead of a habit.
#
# ⚠ THE INTENDED BITE, stated so nobody mistakes it for a bug: a future lane
# that writes the retired name into DECISIONS.md, the journal or any other
# record WILL GO RED, and must take the placeholder as part of that edit. The
# history already in the tree is untouched.
#
# Keys are salted digests of "<repo>:<path>", for the same reason the names are
# digests: TEN OF THESE PATHS CARRY THE TOKEN IN THE PATH ITSELF, so listing
# them as plaintext would republish in this file exactly what the sanitization
# lanes removed -- and the Tier-1 scan would then hit its own allowlist. The
# scan PRINTS the real paths it met at runtime, which is where a human should
# read them: an exception you cannot see is not an exception.
TIER1_TREE_ALLOWLIST = {
    "9b0f0b7863af8c7e567a1e0a9601937bbd99092f4932d1fcdc1f81bd7d8cae57": 145,
    "417f7f3601b90876eee160dc9df9545a77511a3c0aed7684cdf29792709c77dd": 86,
    "2328e052a0fc4c18ef8a6317f9bf9d5f5b4720a5d8858721f92f9341a6c03f5f": 80,
    "7bdf16f2375ff3f2b7f19ab094beb3c068b1db9c7d628e0ddabf02d5a016e6be": 69,
    "f10d949c1000a4e998dd2abfa6212f413d61ff562b5eaf214e5ec82d61d8e64e": 39,
    "bddab28a27d5f4647affc1d5211e3bc8c7512a34bf5402ffa968c410dfc89de0": 21,
    "b6e7746484cbdea116f5d6b60bccb0eaeb4cbcfa7303466b41fe58af5b8251f1": 20,
    "bd804d965533ca3a7547e238363f6c008bd96a9355528d7f530e0d8a48be32d9": 17,
    "5deaa8f32b36f0642157d03215c609350fe22899a7b3b84ab40008f8827f4da8": 16,
    "3c4db8ebee579a6b0c63387ac16dea654ca88bf77538f67c3020b5094d9353d6": 14,
    "94a9b0a600b158a29d58ee83ad3a347cd19fa0894771ec6028357b021a357a2b": 13,
    "f3d37b34343230c71b25378287eb60a913f2f47c8433a92dbf503c200a794526": 13,
    "9d04fda43326468c8b18cd1f472b9715b0c327b396a497c1fb5984d316bce020": 12,
    "a253ee412b9efe6d0a8a2e0431b63540d77306ce9b8c896a58d706519207d740": 12,
    "87a084c7ee7cf2f70f34f21dbae2b3aedb2ea15a32230702b07102eb5e2e848e": 11,
    "759f90cb35ee2e5b54d2657dbc20e9718da3819c63e88319c2bbd1451b0f59f2": 10,
    "38c7e97710353219868d47f8b4280a78ac274c1d91fdd4f2eb801c1714d9bde9": 8,
    "c2d7b7d381637da4f933c954b88047d595593e550938b629c8a0ac13478b5f54": 8,
    "0ec8501a62cb5f4ff99c462060e8cbbbd8f369b0203f45463ab06a523cc683b7": 7,
    "d4536c74b00cfdc6c6089308bc2dbd4aa403af9b5cac2db3a961cc3efed85292": 7,
    "e7e60c9180d92a1bd699d6e6aa19b728f8ec76b1659a938a169bcc54a5d849d0": 7,
    "ff428424348f282db2d52973b6bda808c806976cc5e7f3d315d632f18ad1ed5a": 7,
    "849bed231e6589a252472a46a515a7bf16ca04a65511a9fbf9f65080a3dc193a": 6,
    "a8323c5327d37fcff36b505f95ddfb2bec0c2b94716413775cb2156d07b34b4f": 6,
    "d701bf9ec07fb59d5cefc690a5e72dcd584d927aff7b1e0f07cff0f236362213": 6,
    "e0cabc28c82f9e214292f33415ecd2576d4f578aae080d969cd926173e5ab9e0": 6,
    "2d2267b7addaa1ab35ef5e3cef8fe596d74af986bd9c816408db3260a137708f": 5,
    "7891c1d2f230ff40c4e3f845340c07e90200dc70e547fe872ee4b6bed35dbbee": 5,
    "78a1ed9730bf44299293f165b2144ab89fe0d0eabe0b43b8bb174a3b76363066": 5,
    "8331a131a2b7951fd5897949c65cb7d2cc24177773b477322f3d77af83336478": 5,
    "bd13eb488a02548e95a36f19be4d76ce57ea112ef9da79a674cb2ae258a1c6f3": 5,
    "cddbd9ffc0df5f168ab3e645a95c9ad6821c1e07716f587c488420455d37ab05": 5,
    "ddd46d7d34e20ebe2d1c8664184ee5a8b38aa87c412afc9345850120de0d9db8": 5,
    "eda1694b40bc0fbc5221e9cfbce6310fad980c7270bbb4eec11f65c703276815": 5,
    "130ca0e92f6d2c423909cf43a168c719d483573c22812ebc94e39d755d506ec1": 4,
    "6f0d04861372e83ddf64edfe416bd5df8caffb48df24d5ae8dae1efd06c0e1ea": 4,
    "a038446cdd58893aa59c8f73c93b32a5bb75c471322df7ebb206751eb4db91c3": 4,
    "c96749e2081196c66e472ed90509876b2da69fd581cfb877fddb3978dee2d7c6": 4,
    "e684316c269a9f3c6ff77db76e4c16a986ada7efb398dc83e6bf7bbc8e7f8062": 4,
    "11c16d5bda0e6e1c457eca0769f06ec998d4d062be722ff58eb57680b821dfb8": 3,
    "44e6c2e58f114668580b327cbf043337844ab88dff2a812b29780677c241c72d": 3,
    "4577647dab7abbc2128f9ab7ceda81bffc747ffad6640d0f79eb5104391d5c7f": 3,
    "67f94b74b0c847d83a329b6d2955226ee9ced619e92a8f60255d08b99131bf5a": 3,
    "b6bf707e4b3b2c8c15b130d6db1765b764a651207961026d8ab307010ff04d73": 3,
    "f627bb30a8c2b21548d3a076d028164cf883453ba428f5f5a69134f035c8fb70": 3,
    "02c8fdb634900bb1c520fd24282008fa22f2f1c49f020a6012d3bac82a78122f": 2,
    "02ca2d1502091aa95cb3733e61bc7917b6850956ce8dca3ec63a267683b046fa": 2,
    "0f7aab54fc73853ec83e981f9e1808128e15cb07c736c3ffffbe8738c81b2bab": 2,
    "54661d7371b3263d13bc9f001cbf17956aec2d57f3f063220019dcc27461666b": 2,
    "6937ab83434312c2acb5f06a2bd895e055c710ebfc5b9023b77041d388d6f91c": 2,
    "7f29ae88f3868aa04013b687753bf000100ccb0d66e85e0d41fd6f96a79374c3": 2,
    "96a8017826cbe6e380baa545a87ae89c2bfaaa753fae9c2e5e23766e9f329c7e": 2,
    "a7b8632c321014d3e484fdcb7b8dcc2ac30e6ca7f7f81b8e9570b07dfe0d7a63": 2,
    "b22ee3d57b571bc66e0d2c409f0424d15f6300d9c8981555e742acbae120aa3f": 2,
    "ca9371465b967d4ad4ec30ae0cd56a505f78c5d1feb2ea9c33f049d2c0ee16f7": 2,
    "d0707909e2190be1212aecccc31cfe95778cc5a4fbb53dcb0616c66f4bbaa8c3": 2,
    "d2e21c117d5d8efe301af34bc771a768e00d18e82c50476b044832c6e6525eac": 2,
    "e5dd105f0049851d0956e54ddeec975d7d5866ca9a32d729254345461989cea3": 2,
    "0d62d4104804bbf2952bada861885fef26b89e4a19d9050626adecc8f05f6e4e": 1,
    "0e7fd2462e8b4cf95422ccfdef9a17dd00ff72561ed15f717a72a433840484fd": 1,
    "100171ff07a00c3c59ff120a9a120869e51e62220d4d973a04a667c8c56fb590": 1,
    "24bd2390f003cbf74ea13f5dbecff266ab24b3b9adda3f7d3ec6e289fe321803": 1,
    "2b1c79c0543b6dfb4ec19c339e8fb9264ab0f1b8c9692cc564c4e2320dd96ffb": 1,
    "3067181f0a717fea0bb7caa1c46b68b9886111a5877274235954f7854f7b6ad4": 1,
    "4fab349caf486d56e53a9d381c8dcb850d04e826239c6157bd1bda208952c2b6": 1,
    "582163f5689ccf92b4a3034f7aab5c9508b854526ff5ce93e959951329d3f3ee": 1,
    "59a2b345a020393610ef76dc31d9fed35c4a443037b3722fb00dc461948022b1": 1,
    "5cf44366fcb7d0f9693d2a2273c035cd3a430fe40d6604ae1276e67305c866b4": 1,
    "60bcab5323c61972e0e265bb24b64529e2de3299c977526b9b56b47e77a84a31": 1,
    "66975db48df20b23c615c6d0e4e6d45684f980d6e918ceb067c1e2d1b9cad440": 1,
    "7b5b0d28d13b44d76b1954eae8fcab8b73fbe79740580a51502aea9565f60734": 1,
    "7ec78cb45cf0920d9d852ab81d6ff11dacc1e27b9ac5bee394e4ee618bddc480": 1,
    "821286a3be853568e757c7a11b6a3ec64a919cea563e0a62c7d0da53c48d638a": 1,
    "92a1b17ba36ece369aea7d9252d59939c918e472fc64286ffbd3423723deb8b8": 1,
    "98b0cb745bc056af16398c66392535cb41186bf3d58dc515b65c609909619f7b": 1,
    "a2410bd47fe3989db5c1c52c1a9d4a3aa51e741a91e204ff40d778fee074ef98": 1,
    "c2819379c51c2d94d5e3ccf0287f9d2a9e5f5e2fb4e2f720af8aa025aef1ee90": 1,
    "e3763f69118ca6654baff94cfba09aa94f80bcb797efcc0450be8f8c15fd7e79": 1,
    "e5d76895a4e8ec326770b115722dc0c7faf5f88b2235f34f024c1dc51e8464ba": 1,
}

# WHICH Tier-1 classes carry a per-path baseline, and why it is a SHORT list.
#
# Only a class with a legitimate, ruled history belongs here. `host_retired_rig`
# qualifies: two sanitization lanes measured its population, ruled the dated
# records LEAVE under "reports what was true" and zeroed everything that directs
# traffic today. The other Tier-1 name classes are ZERO across every repository
# and must stay that way, so giving them a budget mechanism would only create a
# place for one to hide. Structural classes are never allowlisted at all -- a
# shape has no history to grandfather.
TIER1_ALLOWLISTED_CLASSES = {"host_retired_rig"}

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


def _repo_name() -> str:
    """Which repository we are scanning, for allowlist keys.

    The gate file is byte-identical across all four repositories and must stay
    that way -- a per-repo copy is a per-repo drift. So the allowlist is keyed on
    `<repo>:<path>` and the repo is derived at run time from the origin remote,
    falling back to the checkout's directory name for a detached or remote-less
    clone.
    """
    url = subprocess.run(
        ["git", "config", "--get", "remote.origin.url"],
        capture_output=True,
        check=False,
    ).stdout.decode("utf-8", "replace").strip()
    if url:
        return url.rstrip("/").rsplit("/", 1)[-1].removesuffix(".git")
    top = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"], capture_output=True, check=False
    ).stdout.decode("utf-8", "replace").strip()
    return top.rsplit("/", 1)[-1] if top else "unknown"


def _allowlisted_count(repo: str, path: str) -> int:
    return TIER1_TREE_ALLOWLIST.get(_digest(f"{repo}:{path}"), 0)


def _count_class_occurrences(line: str, cls: str) -> int:
    """How many TIMES `cls` appears on this line, not merely whether it does.

    The allowlist is a per-path OCCURRENCE budget, so a line that carries the
    token twice must count twice -- otherwise a lane could double an occurrence
    on an existing line and stay under budget.
    """
    total = 0
    for match in _ALNUM_RUN.finditer(line):
        for token in _CAMEL_SPLIT.split(match.group(0)):
            if len(token) in TIER1_MATCHER.lengths:
                if TIER1_MATCHER.digests.get(_digest(token)) == cls:
                    total += 1
    return total


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
        for name, rx in TIER2B_STRUCTURAL.items():
            if rx.search(line):
                classes.append(name)
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
    print("", file=sys.stderr)
    # NA-0686 (ENG-0089, Phase 4d): the two standing rules are printed HERE, at
    # the moment the gate fires, because that is when they are read. A ruling
    # that lives only in DECISIONS.md gets re-derived by whoever trips the gate
    # at 2am; a ruling printed by the tool that blocked them does not.
    print("IF YOU ARE EDITING A LINE THAT ALREADY CARRIED THIS LITERAL", file=sys.stderr)
    print("-----------------------------------------------------------", file=sys.stderr)
    print("You have hit the case the gate was designed for, and the answer is", file=sys.stderr)
    print("recorded -- do not re-derive it, and do not work around the gate:", file=sys.stderr)
    print("", file=sys.stderr)
    print("  (a) A line you RE-ADD carries no such literal. Placeholder it as", file=sys.stderr)
    print("      part of your edit; that is not scope creep, it is the edit.", file=sys.stderr)
    print("  (b) GRANDFATHERED LINES STAY. Untouched legacy content remains", file=sys.stderr)
    print("      under report-don't-touch. That is the tier's designed", file=sys.stderr)
    print("      migration semantics, not an exemption you are abusing.", file=sys.stderr)
    print("  (c) On an added line THE GATE WINS, even against a ruling that", file=sys.stderr)
    print("      said the literal stays. The gate and the redaction rule are", file=sys.stderr)
    print("      one policy.", file=sys.stderr)
    print("", file=sys.stderr)
    print("A tree where one line carries the token and the four beside it carry", file=sys.stderr)
    print("a placeholder is CORRECT: grandfathering is meant to be visible.", file=sys.stderr)
    print("", file=sys.stderr)
    print("CHOOSING THE PLACEHOLDER", file=sys.stderr)
    print("------------------------", file=sys.stderr)
    print("Adopt the vocabulary THE TREE ALREADY USES and derive your mapping", file=sys.stderr)
    print("from that usage -- grep for how neighbouring documents already write", file=sys.stderr)
    print("it. 'Which token?' is a MEASUREMENT, not a matter of taste, and a new", file=sys.stderr)
    print("coinage beside an established one is drift with good intentions.", file=sys.stderr)


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

    allowlisted_classes = TIER1_ALLOWLISTED_CLASSES
    repo = _repo_name() if args.mode == "tree" else ""
    met: dict[str, int] = {}
    exceeded: list[tuple[str, int, int]] = []

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
            per_path = 0
            deferred: list[tuple[str, int, str, str]] = []
            for lineno, line in enumerate(
                raw.decode("utf-8", "replace").splitlines(), start=1
            ):
                lines_seen += 1
                for cls in _scan_line(line, tier1=True, tier2b=False):
                    if cls in allowlisted_classes:
                        per_path += _count_class_occurrences(line, cls)
                        deferred.append((path, lineno, cls, line))
                    else:
                        hits.append((path, lineno, cls, line))
            if per_path:
                budget = _allowlisted_count(repo, path)
                if per_path > budget:
                    # Over budget: every line of the class in this file becomes a
                    # real hit, so the reviewer sees the file, not just a number.
                    hits.extend(deferred)
                    exceeded.append((path, per_path, budget))
                else:
                    met[path] = per_path
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

    # NA-0686: PRINT THE EXCEPTIONS THAT WERE MET. NA-0684's rule — "an exception
    # you cannot see is not an exception" — applies with extra force here,
    # because the allowlist keys are digests: this listing is the only place a
    # human can read which files the baseline actually covers.
    if met:
        total = sum(met.values())
        print(
            f"infra-literal-scan: allowlisted history met — {total} occurrences "
            f"in {len(met)} files, all within their recorded per-path baseline "
            f"[{repo}]"
        )
        for path, n in sorted(met.items(), key=lambda kv: (-kv[1], kv[0])):
            budget = _allowlisted_count(repo, path)
            drift = "" if n == budget else f"  (baseline {budget}; DECREASED)"
            print(f"    {n:5d}  {path}{drift}")

    if exceeded:
        print("", file=sys.stderr)
        for path, found, budget in exceeded:
            print(
                f"OVER BASELINE: {path} carries {found} occurrences, baseline is "
                f"{budget} (+{found - budget})",
                file=sys.stderr,
            )

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
