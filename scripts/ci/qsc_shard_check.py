#!/usr/bin/env python3
r"""qsc_shard_check.py — the D632 shard-manifest gate (Amendment 1 prototype).

Census truth: qsl/qsl-client/qsc/tests/*.rs (depth 1) + the three fixed targets
lib, bin:qsc, doc:qsc. The committed manifest must cover the census EXACTLY —
no missing target, no unknown target, no duplicate, no empty shard, shard count
<= 20 (the ruled Free-plan ceiling). The gate REPORTS the scope it examined and
FAILS on empty input (a census of zero test files is a broken checkout, never a
pass).

CONSUMER-VALIDATED EMISSION (SR-20). Every argument set this file emits is one
`cargo test` accepts. Measured cargo rule, drafting probes P1-P9:

    cargo test --doc --bins   -> exit 101, "error: can't mix --doc with other
                                 target selecting options", emitted BEFORE any
                                 build; identically for --doc --test / --doc --lib
    cargo test --lib --bins --test a --test b -> exit 0

so the doc target must be ALONE in its shard. That rule is enforced HERE, in the
gate, not preserved by memory in the generated manifest: a manifest that packs
doc:qsc with any other target is a named FAIL in BOTH the default mode and
--emit-args.

SR-20 EXTENSION (D-1338, ruled after this gate's first real CI run went red): THE
EMITTING STEP'S ENVIRONMENT IS PART OF THE ARTIFACT'S IDENTITY. A fixture borrowed
from a differently-configured job is not the artifact under test. Measured:
dtolnay/rust-toolchain writes CARGO_TERM_COLOR=always into $GITHUB_ENV, so cargo
emits SGR colour EVEN WHEN REDIRECTED TO A FILE, and the marker lines --verify-log
keys on arrive wrapped:

    '\x1b[1m\x1b[92m   Doc-tests\x1b[0m qsc'
    '\x1b[1m\x1b[92m     Running\x1b[0m tests/foo.rs (target/debug/deps/foo-...)'

which breaks a '^\s+' anchor twice over. This gate therefore strips SGR before
matching, so it works whether or not cargo colourises -- a ruled gate must not
depend on an environment variable a third-party action can flip behind us.

OUTPUT CHANNELS (contract): STDOUT is the machine channel — only --emit-args
writes to it, and it writes exactly one line, the cargo argument string. STDERR
is the diagnostic channel — every FAIL line and every "scope examined" line goes
there. A caller therefore needs no pipe to separate them.

Modes:
  (default)                 verify manifest vs census; exit 0 only on exact cover
  --emit-args SHARD         print one shard's cargo target args on STDOUT
  --verify-log SHARD LOG    verify a cargo test log ran EXACTLY the shard's
                            targets, BY NAME ('Running tests/<name>.rs' /
                            'Doc-tests' / unittests lines)
  --assert-workflow WF      verify the workflow file itself: (1) its `shard: [...]`
                            matrix runs EXACTLY the shard ids the manifest assigns
                            targets to, in BOTH directions; (2) every `runs-on:`
                            value in it is `ubuntu-latest` — the ruled
                            STANDARD-runner constraint, machine-enforced rather
                            than preserved by memory.

THIRD-PARTY DEPENDENCY BAR (ruled): this file imports NOTHING outside the Python
standard library. A third-party import would need an install step on the runner
and would mint a new failure mode inside the gate that guards the suite.
"""
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TESTS_DIR = os.path.join(REPO, "qsl", "qsl-client", "qsc", "tests")
MANIFEST = os.path.join(os.path.dirname(os.path.abspath(__file__)), "QSC_SHARD_MANIFEST.txt")
SPECIALS = ["lib", "bin:qsc", "doc:qsc"]
DOC_TARGET = "doc:qsc"
MAX_SHARDS = 20
USAGE = (
    "usage: qsc_shard_check.py [--emit-args SHARD | --verify-log SHARD LOG | "
    "--assert-workflow WORKFLOW]"
)
RULED_RUNNER = "ubuntu-latest"


def fail(msg):
    print(f"FAIL: {msg}", file=sys.stderr)


def scope(msg):
    print(f"scope examined: {msg}", file=sys.stderr)


def die(msg):
    fail(msg)
    sys.exit(1)


def shard_arg(raw):
    """Parse a shard id argument. Named FAIL, never a traceback."""
    try:
        return int(raw)
    except (TypeError, ValueError):
        die(f"shard argument must be an integer, got {raw!r}")


def census():
    if not os.path.isdir(TESTS_DIR):
        die(f"tests dir not found: {TESTS_DIR}")
    names = sorted(
        f"tests/{e}" for e in os.listdir(TESTS_DIR)
        if e.endswith(".rs") and os.path.isfile(os.path.join(TESTS_DIR, e))
    )
    if not names:
        die("census is EMPTY (zero tests/*.rs found) — empty input fails")
    return SPECIALS + names


def read_manifest():
    if not os.path.isfile(MANIFEST):
        die(f"manifest not found: {MANIFEST} — empty input fails")
    rows = []
    with open(MANIFEST, encoding="utf-8") as fh:
        for ln, line in enumerate(fh, 1):
            line = line.rstrip("\n")
            if not line or line.startswith("#"):
                continue
            parts = line.split("\t")
            if len(parts) != 2 or not parts[1].isdigit():
                die(f"malformed manifest line {ln}: {line!r}")
            rows.append((parts[0], int(parts[1])))
    if not rows:
        die("manifest has ZERO rows — empty input fails")
    return rows


def verify(cens, rows):
    ok = True
    seen = {}
    for name, shard in rows:
        if name in seen:
            fail(f"DUPLICATE manifest row: {name}")
            ok = False
        seen[name] = shard
    cset = set(cens)
    missing = [n for n in cens if n not in seen]
    unknown = [n for n in seen if n not in cset]
    for n in missing:
        fail(f"MISSING from manifest (present in tree): {n}")
        ok = False
    for n in unknown:
        fail(f"UNKNOWN in manifest (absent from tree): {n}")
        ok = False
    shards = sorted(set(s for _, s in rows))
    k = (max(shards) + 1) if shards else 0
    if k > MAX_SHARDS:
        fail(f"shard count {k} exceeds the ruled ceiling {MAX_SHARDS}")
        ok = False
    if shards != list(range(k)):
        fail(f"shard ids not contiguous 0..{k-1}: {shards}")
        ok = False
    counts = {i: 0 for i in range(k)}
    for _, s in rows:
        if s in counts:
            counts[s] += 1
    for i in range(k):
        if counts.get(i, 0) == 0:
            fail(f"shard {i} is EMPTY — empty input fails")
            ok = False
    # SR-20: the emitted argument set must be one cargo ACCEPTS. cargo refuses
    # --doc mixed with any other target selector, so the doc target is alone.
    doc_shard = seen.get(DOC_TARGET)
    doc_mates = []
    if doc_shard is not None:
        doc_mates = sorted(n for n, s in seen.items() if s == doc_shard and n != DOC_TARGET)
        if doc_mates:
            fail(
                f"shard {doc_shard} assigns {DOC_TARGET} alongside {len(doc_mates)} other "
                f"target(s) — cargo rejects --doc mixed with any other target selector "
                f"(measured: \"error: can't mix --doc with other target selecting "
                f"options\", exit 101). The doc target must be ALONE in its shard. "
                f"Co-tenants: {', '.join(doc_mates)}"
            )
            ok = False
    scope(
        f"census {len(cens)} targets / manifest {len(rows)} rows / {k} shards / "
        f"missing {len(missing)} / unknown {len(unknown)} / doc shard {doc_shard} "
        f"with {len(doc_mates)} co-tenant(s)"
    )
    return ok, seen, k


def cargo_args(name):
    if name == "lib":
        return ["--lib"]
    if name == "bin:qsc":
        return ["--bins"]
    if name == DOC_TARGET:
        return ["--doc"]
    assert name.startswith("tests/") and name.endswith(".rs")
    return ["--test", name[len("tests/"):-len(".rs")]]


def mode_emit_args(assign, shard):
    names = [n for n, s in sorted(assign.items()) if s == shard]
    if not names:
        die(f"shard {shard} resolves to ZERO targets")
    # SR-20, re-asserted on the emit path itself so this mode's contract stands
    # alone: what we print must be an argument set cargo accepts.
    if DOC_TARGET in names and len(names) > 1:
        others = [n for n in names if n != DOC_TARGET]
        die(
            f"shard {shard} would emit --doc together with {len(others)} other target "
            f"selector(s) — cargo rejects that combination outright. Co-tenants: "
            f"{', '.join(others)}"
        )
    out = []
    for n in names:
        out.extend(cargo_args(n))
    scope(f"shard {shard} emits {len(names)} target(s) / {len(out)} cargo argument(s)")
    print(" ".join(out))


def mode_verify_log(assign, shard, path):
    expected = sorted(n for n, s in assign.items() if s == shard)
    if not expected:
        die(f"shard {shard} resolves to ZERO targets — empty input fails")
    if not os.path.isfile(path):
        die(f"log file not found: {path} — empty input fails")
    if os.path.getsize(path) == 0:
        die(f"log file is EMPTY: {path} — empty input fails")
    ran = set()
    lines = 0
    # SGR strip (see the SR-20 extension in this file's docstring). Deliberately
    # narrow: it removes colour only. It does NOT strip a leading timestamp, so a
    # raw runner console stream still FAILS CLOSED, which is the contract.
    sgr = re.compile(r"\x1b\[[0-9;]*m")
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            lines += 1
            line = sgr.sub("", line)
            if re.search(r"^\s+Running unittests src/lib\.rs", line):
                ran.add("lib")
                continue
            if re.search(r"^\s+Running unittests src/main\.rs", line):
                ran.add("bin:qsc")
                continue
            if re.search(r"^\s+Doc-tests qsc", line):
                ran.add(DOC_TARGET)
                continue
            m = re.search(r"^\s+Running (tests/\S+\.rs)", line)
            if m:
                ran.add(m.group(1))
    missing = [n for n in expected if n not in ran]
    extra = sorted(n for n in ran if n not in expected)
    for n in missing:
        fail(f"shard {shard} did NOT run {n}")
    for n in extra:
        fail(f"shard {shard} ran UNASSIGNED target {n}")
    scope(
        f"log {path} / {lines} lines / shard {shard} expected {len(expected)} / "
        f"observed {len(ran)} / missing {len(missing)} / extra {len(extra)}"
    )
    if missing or extra:
        sys.exit(1)


def mode_assert_workflow(assign, k, path):
    """Assert the workflow file's two machine-enforceable ruled properties.

    (1) MATRIX <-> MANIFEST, both directions. The manifest gate must not pass a
        manifest whose shard set differs from the matrix the workflow expands: a
        manifest that grows leaves its new shard un-run, and a matrix that
        SHRINKS leaves an existing shard un-run. Both are silent coverage holes —
        the ENG-0092 class at shard-set level. Reading the matrix out of the
        workflow file, rather than comparing against a hand-copied literal, is
        what closes the second direction.
    (2) STANDARD RUNNERS. Every `runs-on:` value in this file must be
        `ubuntu-latest`. The constraint is operator-ruled; a ruled constraint
        preserved by memory is not preserved.

    Both parses FAIL CLOSED: a shard matrix in a shape this gate does not
    recognise is a named FAIL, never a skip.
    """
    if not os.path.isfile(path):
        die(f"workflow file not found: {path} — empty input fails")
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.read().splitlines()

    # --- (2) runs-on, measured first so a bad runner is named even if the matrix
    #         parse is what ultimately kills the run.
    runners = []
    for ln, line in enumerate(lines, 1):
        m = re.match(r"^\s*runs-on:\s*(\S.*?)\s*$", line)
        if m:
            runners.append((ln, m.group(1)))
    if not runners:
        die(f"no `runs-on:` line found in {path} — empty input fails")
    bad_runners = [(ln, v) for ln, v in runners if v != RULED_RUNNER]

    # --- (1) the shard matrix. The `shard:` key is looked for ONLY INSIDE a
    #         `matrix:` block. Scoping matters: this workflow also has a JOB named
    #         `shard`, so a bare `shard:` needle matches the job key too — a needle
    #         wider than its claim, which is the very class this gate exists to
    #         catch. Only the single-line flow form is understood; a `shard:` key
    #         inside a matrix block in any other shape is NAMED, never skipped.
    def report_runners_and_exit(scope_line):
        """Never lose a measured runs-on finding to an earlier parse failure."""
        for rln_, val_ in bad_runners:
            fail(
                f"{path}:{rln_} runs-on {val_!r} is not {RULED_RUNNER!r} — STANDARD "
                f"runners only is an operator-ruled constraint of this lane and is "
                f"enforced here mechanically, not by memory"
            )
        scope(scope_line + f" / runs-on values {len(runners)} / "
                           f"non-{RULED_RUNNER} {len(bad_runners)}")
        sys.exit(1)

    hits, malformed = [], []
    in_matrix_at = None
    for ln, line in enumerate(lines, 1):
        if not line.strip():
            continue
        indent = len(line) - len(line.lstrip(" "))
        if in_matrix_at is not None and indent <= in_matrix_at:
            in_matrix_at = None
        m = re.match(r"^(\s*)matrix:\s*$", line)
        if m:
            in_matrix_at = len(m.group(1))
            continue
        if in_matrix_at is None:
            continue
        m = re.match(r"^\s*shard:\s*(.*?)\s*$", line)
        if not m:
            continue
        body = m.group(1)
        flow = re.match(r"^\[([^\]]*)\]$", body)
        if flow:
            hits.append((ln, flow.group(1)))
        else:
            malformed.append((ln, body))
    if malformed:
        for ln, body in malformed:
            fail(
                f"{path}:{ln} carries a `shard:` matrix key in a shape this gate does "
                f"not recognise ({body!r} — the gate requires the single-line flow form "
                f"`shard: [0, 1, ...]`). An unrecognised matrix is a FAIL, never a skip: "
                f"the gate cannot prove the matrix matches the manifest."
            )
        report_runners_and_exit(
            f"workflow {path} / {len(lines)} lines / {len(malformed)} unrecognised "
            f"`shard:` matrix shape(s)")
    if not hits:
        fail(
            f"no `shard: [...]` matrix line found inside a `matrix:` block in {path} — "
            f"empty input fails. The manifest gate cannot prove the matrix matches the "
            f"manifest, so it refuses to pass."
        )
        report_runners_and_exit(
            f"workflow {path} / {len(lines)} lines / 0 shard matrix lines")
    if len(hits) > 1:
        fail(
            f"{len(hits)} `shard: [...]` matrix lines found in {path} "
            f"(lines {', '.join(str(ln) for ln, _ in hits)}); exactly one expected"
        )
        report_runners_and_exit(
            f"workflow {path} / {len(lines)} lines / {len(hits)} shard matrix lines")
    ln, body = hits[0]
    raw = [tok.strip() for tok in body.split(",") if tok.strip()]
    if not raw:
        die(f"the `shard: [...]` matrix at {path}:{ln} is EMPTY — empty input fails")
    matrix = set()
    for tok in raw:
        if not re.fullmatch(r"\d+", tok):
            die(f"non-integer matrix shard id {tok!r} at {path}:{ln}")
        matrix.add(int(tok))
    manifest_shards = set(s for s in assign.values())
    never_run = sorted(manifest_shards - matrix)
    no_targets = sorted(matrix - manifest_shards)
    for s in never_run:
        n = sum(1 for v in assign.values() if v == s)
        fail(
            f"manifest shard {s} has {n} target(s) but the workflow matrix at "
            f"{path}:{ln} never runs it — those targets would execute nowhere"
        )
    for s in no_targets:
        fail(
            f"workflow matrix at {path}:{ln} runs shard {s} but the manifest assigns "
            f"it zero targets"
        )
    for rln, val in bad_runners:
        fail(
            f"{path}:{rln} runs-on {val!r} is not {RULED_RUNNER!r} — STANDARD runners "
            f"only is an operator-ruled constraint of this lane and is enforced here "
            f"mechanically, not by memory"
        )
    scope(
        f"workflow {path}:{ln} / matrix {len(matrix)} shards / manifest {k} shards / "
        f"never-run {len(never_run)} / no-targets {len(no_targets)} / "
        f"runs-on values {len(runners)} / non-{RULED_RUNNER} {len(bad_runners)}"
    )
    if never_run or no_targets or bad_runners:
        sys.exit(1)


def main():
    argv = sys.argv[1:]
    if argv and argv[0] not in ("--emit-args", "--verify-log", "--assert-workflow"):
        die(f"unknown mode {argv[0]!r}. {USAGE}")

    cens = census()
    rows = read_manifest()
    ok, assign, k = verify(cens, rows)
    if not ok:
        sys.exit(1)

    if not argv:
        print("OK: manifest covers the census exactly", file=sys.stderr)
        return

    if argv[0] == "--emit-args":
        if len(argv) != 2:
            die(f"--emit-args takes exactly one SHARD argument. {USAGE}")
        mode_emit_args(assign, shard_arg(argv[1]))
        return

    if argv[0] == "--verify-log":
        if len(argv) != 3:
            die(f"--verify-log takes exactly SHARD and LOG. {USAGE}")
        mode_verify_log(assign, shard_arg(argv[1]), argv[2])
        return

    if argv[0] == "--assert-workflow":
        if len(argv) != 2:
            die(f"--assert-workflow takes exactly one WORKFLOW argument. {USAGE}")
        mode_assert_workflow(assign, k, argv[1])
        return


if __name__ == "__main__":
    main()
