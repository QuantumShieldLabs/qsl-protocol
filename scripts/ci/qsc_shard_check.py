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
                            value in it is inside the CALLER-DECLARED allowed set,
                            which must itself be a subset of STANDARD_RUNNERS;
                            (3) named jobs run on the exact runner declared for
                            them; (4) the matrix is no larger than the caller's
                            platform budget; (5) the `--ratchet` ceiling literal in
                            the file agrees with the `shard` job's timeout-minutes.

Options (NA-0724, D-1360):
  --manifest PATH           read this manifest instead of the Linux default. It IS
                            defaulted, deliberately, so every pre-existing call site
                            is unchanged.
  --expect-runners CSV      REQUIRED by --assert-workflow, NOT defaulted. Every
                            `runs-on:` in the asserted file must be a member of this
                            set, and the set must be a subset of the hard-coded
                            STANDARD_RUNNERS allowlist below — so no caller can admit
                            a larger-runner or self-hosted label merely by naming it.
  --expect-job-runner J=L   REQUIRED by --assert-workflow, NOT defaulted, repeatable.
                            Job J's `runs-on` must exist and equal L exactly. A SET
                            membership check cannot express this: on the macOS file
                            `--expect-runners ubuntu-latest,macos-latest` admits
                            `ubuntu-latest` everywhere, which is precisely the slip a
                            hand-authored mirror of a Linux file invites — it would
                            pass every test (the census is platform-portable) while
                            macOS coverage was silently zero.
  --max-shards N            REQUIRED by --assert-workflow, NOT defaulted. The matrix
                            must satisfy K <= N, tightening the global MAX_SHARDS per
                            workflow to the platform's real slot budget.
  --ratchet SECS CEILING    print measured runtime vs the ceiling; WARN at >=80%,
                            FAIL at >=90%. Reads NEITHER census NOR manifest and
                            therefore takes no --manifest: it short-circuits ahead of
                            them so its exit code is never coupled to manifest health.
  --ratchet-arm ARM         REQUIRED by --ratchet, NOT defaulted. On `push` the >=90%
                            FAIL is downgraded to a WARN, because the repair for suite
                            GROWTH is a workflow_security PR — exactly the class the
                            admission freeze refuses while main is red. Alarm and
                            freeze must not share a trigger.

BECAUSE NEITHER --expect-runners, --expect-job-runner, --max-shards NOR --ratchet-arm
HAS A DEFAULT, A CALLER THAT DECLARES NOTHING FAILS CLOSED. That is the ruled property
(655 §5.4, A1 §6.1); R298.1 confirmed it against the call-site-invariance gate, whose
own baseline is the flagged call site, not the superseded bare one.

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
    "usage: qsc_shard_check.py [--manifest PATH] [--emit-args SHARD | "
    "--verify-log SHARD LOG | --assert-workflow WORKFLOW --expect-runners CSV "
    "--expect-job-runner JOB=LABEL --max-shards N | "
    "--ratchet SECONDS CEILING_MINUTES --ratchet-arm ARM]"
)
RULED_RUNNER = "ubuntu-latest"

# The hard-coded GitHub-hosted STANDARD-runner allowlist. --expect-runners may only
# name labels from this set, so a caller cannot admit a larger-runner or self-hosted
# label by declaring it. This is the outer bound; --expect-runners is the inner one.
STANDARD_RUNNERS = ("ubuntu-latest", "macos-latest", "windows-latest")

# WF-0076's thresholds, folded in UNALTERED by D-1360. Only which arm may turn them
# into a non-zero exit changes (see --ratchet-arm).
RATCHET_WARN_PCT = 80.0
RATCHET_FAIL_PCT = 90.0
RATCHET_SUPPRESSED_ARM = "push"


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


def read_manifest(path=MANIFEST):
    if not os.path.isfile(path):
        die(f"manifest not found: {path} — empty input fails")
    rows = []
    with open(path, encoding="utf-8") as fh:
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


def parse_jobs(lines):
    """Job keys under `jobs:`, and each job's SCALAR `runs-on`, in file order.

    F13's cure needs both halves. The pre-existing `runs-on` needle matched a scalar on
    the same line, so YAML block-sequence form (`runs-on:` newline `  - label`) was
    INVISIBLE and the zero-found guard still passed if any OTHER scalar `runs-on`
    existed. Counting JOBS as well as discovered scalar runners turns that silent hole
    into a named FAIL: a job whose runner this parser cannot see is never skipped.
    """
    jobs, runs_on = [], {}
    in_jobs = False
    current = None
    for ln, line in enumerate(lines, 1):
        if re.match(r"^jobs:\s*$", line):
            in_jobs = True
            continue
        if not in_jobs:
            continue
        if line.strip() and not line.startswith(" ") and not line.startswith("#"):
            break
        m = re.match(r"^  ([A-Za-z_][A-Za-z0-9_-]*):\s*$", line)
        if m:
            current = m.group(1)
            jobs.append((ln, current))
            continue
        m = re.match(r"^\s+runs-on:\s*(\S.*?)\s*$", line)
        if m and current is not None and current not in runs_on:
            runs_on[current] = (ln, m.group(1))
    return jobs, runs_on


def parse_job_timeout(lines, job):
    """The `timeout-minutes` of one job, by the SAME rules the consumer's awk uses.

    G1d gates this parser against `public-ci.yml`'s `ceiling_of()` awk, because F14's
    cure introduces a SECOND parser of the same property and two parsers of one property
    drift. The semantics mirrored exactly: match a line EQUAL to "  <job>:", then return
    the FIRST `timeout-minutes:` at any indent inside the block, stopping at the next
    2-space job key.
    """
    marker = f"  {job}:"
    injob = False
    for line in lines:
        if not injob:
            if line == marker:
                injob = True
            continue
        if re.match(r"^  [a-zA-Z_]+:[ \t]*$", line):
            return None
        parts = line.split()
        if parts and parts[0] == "timeout-minutes:":
            return parts[1] if len(parts) > 1 else None
    return None


def parse_ratchet_calls(lines):
    """Every `--ratchet SECONDS CEILING` INVOCATION in a workflow file (F14).

    The needle is built from the invocation's own bytes, not from the flag name alone:
    it must be a non-comment line that actually runs THIS script. A bare `--ratchet`
    needle also matches PROSE — the macOS workflow's own header comment explains that
    its `timeout-minutes` is "bound to the --ratchet argument below", and a flag-name
    needle counts that sentence as a second invocation and fails a correct file. An
    instrument's scope must equal its claim.
    """
    hits = []
    for ln, line in enumerate(lines, 1):
        if line.lstrip().startswith("#"):
            continue
        if "qsc_shard_check.py" not in line:
            continue
        m = re.search(r"--ratchet\s+(?:\"[^\"]*\"|'[^']*'|\S+)\s+(\S+)", line)
        if m:
            hits.append((ln, m.group(1)))
    return hits


def parse_assert_options(rest):
    """The three REQUIRED, UNDEFAULTED --assert-workflow arguments. FAILS CLOSED.

    A caller that declares nothing gets a NAMED refusal, never a permissive default:
    a ruled constraint preserved by default is not preserved.
    """
    expect_runners = None
    job_runners = []
    max_shards = None
    i = 0
    while i < len(rest):
        flag = rest[i]
        if flag in ("--expect-runners", "--expect-job-runner", "--max-shards"):
            if i + 1 >= len(rest):
                die(f"{flag} takes exactly one argument. {USAGE}")
            value = rest[i + 1]
            if flag == "--expect-runners":
                expect_runners = value
            elif flag == "--max-shards":
                max_shards = value
            else:
                job_runners.append(value)
            i += 2
            continue
        die(f"unknown --assert-workflow option {flag!r}. {USAGE}")

    missing = []
    if expect_runners is None:
        missing.append("--expect-runners CSV")
    if not job_runners:
        missing.append("--expect-job-runner JOB=LABEL")
    if max_shards is None:
        missing.append("--max-shards N")
    if missing:
        die(
            f"--assert-workflow requires {', '.join(missing)} — these arguments have NO "
            f"DEFAULT, so a caller that declares nothing FAILS CLOSED rather than "
            f"inheriting a permissive one. Declare the runner set, the per-job runner and "
            f"the platform shard budget explicitly. {USAGE}"
        )

    declared = [r.strip() for r in expect_runners.split(",") if r.strip()]
    if not declared:
        die(
            f"--expect-runners {expect_runners!r} declares an EMPTY set — empty input "
            f"fails. An empty allowed set admits no runner at all."
        )
    outside = [r for r in declared if r not in STANDARD_RUNNERS]
    if outside:
        die(
            f"--expect-runners names {', '.join(sorted(outside))}, which is outside the "
            f"hard-coded STANDARD_RUNNERS allowlist ({', '.join(STANDARD_RUNNERS)}). A "
            f"caller may TIGHTEN the runner set but may never widen it to admit a "
            f"larger-runner or self-hosted label by naming it."
        )

    pairs = []
    for spec in job_runners:
        if spec.count("=") != 1 or spec.startswith("=") or spec.endswith("="):
            die(f"--expect-job-runner expects JOB=LABEL, got {spec!r}")
        job, label = spec.split("=")
        pairs.append((job, label))

    if not re.fullmatch(r"\d+", max_shards):
        die(f"--max-shards expects a non-negative integer, got {max_shards!r}")

    return {
        "expect_runners_raw": expect_runners.strip(),
        "declared": declared,
        "job_runners": pairs,
        "max_shards": int(max_shards),
    }


def mode_ratchet(rest):
    """WF-0076's ratchet, folded in by D-1360 with its thresholds UNALTERED.

    Reads NEITHER the census NOR the manifest, and is dispatched BEFORE either is read,
    so a shard's runtime alarm can never be coupled to manifest health.

    The arm decides only whether the >=90% case is an exit code. On `push` it is a WARN:
    the repair for suite GROWTH is a ceiling re-fit, i.e. an edit to .github/workflows/**,
    which classifies workflow_security — exactly the class every admission path refuses
    while main is red. Alarm and freeze must not share a trigger.
    """
    if len(rest) < 2:
        die(f"--ratchet takes SECONDS and CEILING_MINUTES. {USAGE}")
    secs_raw, ceiling_raw = rest[0], rest[1]
    arm = None
    i = 2
    while i < len(rest):
        if rest[i] == "--ratchet-arm":
            if i + 1 >= len(rest):
                die(f"--ratchet-arm takes exactly one ARM argument. {USAGE}")
            arm = rest[i + 1]
            i += 2
            continue
        die(f"unknown --ratchet option {rest[i]!r}. {USAGE}")
    if arm is None:
        die(
            f"--ratchet-arm is REQUIRED and has NO DEFAULT. The arm decides whether a "
            f">={RATCHET_FAIL_PCT:.0f}% overrun fails the step or only warns, and guessing "
            f"it either freezes main on suite growth or silences the alarm. {USAGE}"
        )
    if not re.fullmatch(r"\d+", secs_raw):
        die(f"--ratchet SECONDS must be a non-negative integer, got {secs_raw!r}")
    if not re.fullmatch(r"\d+", ceiling_raw) or int(ceiling_raw) == 0:
        die(f"--ratchet CEILING_MINUTES must be a positive integer, got {ceiling_raw!r}")

    secs, ceiling = int(secs_raw), int(ceiling_raw)
    budget = ceiling * 60
    pct = secs * 100.0 / budget
    scope(
        f"ratchet arm {arm} / measured {secs}s / ceiling {ceiling}m ({budget}s) / "
        f"{pct:.1f}% of ceiling / warn {RATCHET_WARN_PCT:.0f}% / fail {RATCHET_FAIL_PCT:.0f}%"
    )
    if pct >= RATCHET_FAIL_PCT:
        if arm == RATCHET_SUPPRESSED_ARM:
            print(
                f"WARN: shard used {pct:.1f}% of its {ceiling}m ceiling "
                f"(>={RATCHET_FAIL_PCT:.0f}%). The FAIL was SUPPRESSED because this is the "
                f"{arm!r} arm and failing here would red main on suite GROWTH, whose repair "
                f"is a workflow_security PR — the class the admission freeze refuses. "
                f"Re-fit the ceiling from a measured run.",
                file=sys.stderr,
            )
            return
        fail(
            f"shard used {pct:.1f}% of its {ceiling}m ceiling "
            f"(>={RATCHET_FAIL_PCT:.0f}%) on the {arm!r} arm — the ceiling is expiring "
            f"rather than being re-fitted. Re-fit it from a measured run."
        )
        sys.exit(1)
    if pct >= RATCHET_WARN_PCT:
        print(
            f"WARN: shard used {pct:.1f}% of its {ceiling}m ceiling "
            f"(>={RATCHET_WARN_PCT:.0f}%) — re-fit the ceiling from a measured run before "
            f"it expires.",
            file=sys.stderr,
        )


def mode_assert_workflow(assign, k, path, opts):
    """Assert the workflow file's two machine-enforceable ruled properties.

    (1) MATRIX <-> MANIFEST, both directions. The manifest gate must not pass a
        manifest whose shard set differs from the matrix the workflow expands: a
        manifest that grows leaves its new shard un-run, and a matrix that
        SHRINKS leaves an existing shard un-run. Both are silent coverage holes —
        the ENG-0092 class at shard-set level. Reading the matrix out of the
        workflow file, rather than comparing against a hand-copied literal, is
        what closes the second direction.
    (2) STANDARD RUNNERS. Every `runs-on:` value in this file must be a member of
        the CALLER-DECLARED set, which must itself be a subset of the hard-coded
        STANDARD_RUNNERS allowlist. The constraint is operator-ruled; a ruled
        constraint preserved by memory is not preserved.
    (3) PER-JOB RUNNERS. Each `--expect-job-runner JOB=LABEL` must match exactly.
        Set membership alone cannot express this, and on a hand-authored mirror it
        is the difference between real macOS coverage and silently zero.
    (4) PLATFORM BUDGET. K <= --max-shards, so a matrix that outgrows its platform's
        slot budget fails closed and forces the watchdog's FANOUT_WAVES literal to
        be revisited.
    (5) RATCHET BINDING (F14). The `--ratchet ... N` ceiling literal inside this
        file must equal the `shard` job's `timeout-minutes`, so a re-fit cannot move
        one without the other.

    Every parse FAILS CLOSED: a shard matrix in a shape this gate does not
    recognise, or a job whose runner it cannot see, is a named FAIL, never a skip.
    """
    if not os.path.isfile(path):
        die(f"workflow file not found: {path} — empty input fails")
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.read().splitlines()

    declared = opts["declared"]
    declared_raw = opts["expect_runners_raw"]

    # --- (2) runs-on, measured first so a bad runner is named even if the matrix
    #         parse is what ultimately kills the run.
    runners = []
    for ln, line in enumerate(lines, 1):
        m = re.match(r"^\s*runs-on:\s*(\S.*?)\s*$", line)
        if m:
            runners.append((ln, m.group(1)))
    if not runners:
        die(f"no `runs-on:` line found in {path} — empty input fails")
    bad_runners = [(ln, v) for ln, v in runners if v not in declared]

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
                f"{path}:{rln_} runs-on {val_!r} is not in the caller-declared set "
                f"{declared_raw!r} — STANDARD runners only is an operator-ruled "
                f"constraint of this lane and is enforced here mechanically, not by memory"
            )
        scope(scope_line + f" / runs-on values {len(runners)} / "
                           f"non-{declared_raw} {len(bad_runners)}")
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
            f"{path}:{rln} runs-on {val!r} is not in the caller-declared set "
            f"{declared_raw!r} — STANDARD runners only is an operator-ruled constraint "
            f"of this lane and is enforced here mechanically, not by memory"
        )

    # --- (3) F13: every job's runner must be VISIBLE to this parser, and (4)/(5) the
    #         per-job runner, platform budget and ratchet binding. Reported on their own
    #         `scope examined:` line so the pre-existing line above stays byte-identical
    #         for a caller declaring exactly the runner set the file already used.
    jobs, job_runners = parse_jobs(lines)
    unseen = [name for _, name in jobs if name not in job_runners]
    for name in unseen:
        fail(
            f"{path}: job {name!r} has no runner this gate can SEE. `runs-on` is read as a "
            f"scalar on its own line, so YAML block-sequence form is invisible to it — and "
            f"an invisible runner is a silent hole, never a skip. Write `runs-on: <label>`."
        )

    job_runner_bad = []
    for job, label in opts["job_runners"]:
        if job not in job_runners:
            job_runner_bad.append(job)
            fail(
                f"{path}: --expect-job-runner names job {job!r}, which this file does not "
                f"define (jobs: {', '.join(n for _, n in jobs) or '<none>'})"
            )
            continue
        rln, val = job_runners[job]
        if val != label:
            job_runner_bad.append(job)
            fail(
                f"{path}:{rln} job {job!r} runs on {val!r}, but {label!r} was declared. Set "
                f"membership cannot express this: a mirror of a Linux file whose shard job "
                f"says {RULED_RUNNER!r} passes every test — the census is platform-portable "
                f"— while the platform coverage this job exists to provide is silently zero."
            )

    over_budget = len(matrix) > opts["max_shards"]
    if over_budget:
        fail(
            f"{path}:{ln} matrix runs {len(matrix)} shards but --max-shards declares a "
            f"platform budget of {opts['max_shards']}. A matrix larger than its platform's "
            f"slot budget invalidates the watchdog's FANOUT_WAVES arithmetic, so this fails "
            f"closed and forces that literal to be revisited."
        )

    # (5) F14: the ratchet ceiling literal is BOUND to the shard job's timeout-minutes.
    shard_timeout = parse_job_timeout(lines, "shard")
    ratchet_hits = parse_ratchet_calls(lines)
    ratchet_bad = False
    if shard_timeout is None or not re.fullmatch(r"\d+", shard_timeout):
        ratchet_bad = True
        fail(
            f"{path}: no integer `timeout-minutes` found for job 'shard' — the ratchet "
            f"literal cannot be bound to a ceiling that cannot be read, and the "
            f"public-safety watchdog derives its whole budget from that same value."
        )
    if len(ratchet_hits) != 1:
        ratchet_bad = True
        fail(
            f"{path}: expected exactly ONE `--ratchet SECONDS CEILING` invocation, found "
            f"{len(ratchet_hits)}"
            + (f" (lines {', '.join(str(h[0]) for h in ratchet_hits)})" if ratchet_hits else "")
            + ". No call, or more than one, is a named FAIL: the binding below can only be "
              "proven against a single unambiguous literal."
        )
    elif not re.fullmatch(r"\d+", ratchet_hits[0][1]):
        ratchet_bad = True
        fail(
            f"{path}:{ratchet_hits[0][0]} `--ratchet` ceiling {ratchet_hits[0][1]!r} is not "
            f"an integer"
        )
    elif shard_timeout is not None and ratchet_hits[0][1] != shard_timeout:
        ratchet_bad = True
        fail(
            f"{path}:{ratchet_hits[0][0]} `--ratchet` ceiling {ratchet_hits[0][1]} disagrees "
            f"with the `shard` job's timeout-minutes {shard_timeout}. A ceiling re-fit that "
            f"moves one literal and not the other would leave the alarm measuring against a "
            f"ceiling that no longer exists, so the two are bound here."
        )

    scope(
        f"workflow {path}:{ln} / matrix {len(matrix)} shards / manifest {k} shards / "
        f"never-run {len(never_run)} / no-targets {len(no_targets)} / "
        f"runs-on values {len(runners)} / non-{declared_raw} {len(bad_runners)}"
    )
    scope(
        f"workflow {path} / jobs {len(jobs)} / runners seen {len(job_runners)} / "
        f"runners unseen {len(unseen)} / job-runner asserts {len(opts['job_runners'])} "
        f"failing {len(job_runner_bad)} / max-shards {opts['max_shards']} K {len(matrix)} / "
        f"shard timeout-minutes {shard_timeout} ratchet ceiling "
        f"{ratchet_hits[0][1] if len(ratchet_hits) == 1 else '<none>'}"
    )
    if never_run or no_targets or bad_runners or unseen or job_runner_bad or over_budget \
            or ratchet_bad:
        sys.exit(1)


def main():
    argv = sys.argv[1:]

    # --ratchet SHORT-CIRCUITS ahead of census()/read_manifest()/verify(). It reads
    # neither, and coupling a shard's runtime alarm to manifest health would make the
    # alarm's exit code mean two different things.
    if argv and argv[0] == "--ratchet":
        mode_ratchet(argv[1:])
        return

    # --manifest is an OPTIONAL PREFIX and it IS defaulted, deliberately: every
    # pre-existing call site must keep working unchanged (655 §5.5). It is stripped
    # before the positional dispatcher below ever sees argv[0], so the dispatcher's
    # shape — and every one of its error messages — is preserved exactly.
    manifest_path = MANIFEST
    if argv and argv[0] == "--manifest":
        if len(argv) < 2:
            die(f"--manifest takes exactly one PATH argument. {USAGE}")
        manifest_path = argv[1]
        argv = argv[2:]

    if argv and argv[0] not in ("--emit-args", "--verify-log", "--assert-workflow"):
        die(f"unknown mode {argv[0]!r}. {USAGE}")

    cens = census()
    rows = read_manifest(manifest_path)
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
        if len(argv) < 2:
            die(f"--assert-workflow takes exactly one WORKFLOW argument. {USAGE}")
        mode_assert_workflow(assign, k, argv[1], parse_assert_options(argv[2:]))
        return


if __name__ == "__main__":
    main()
