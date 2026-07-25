# NA-0676 — as-built (D612, D-1307) — the TIER-1 operator-infrastructure sanitize

Result class: **`INFRA_SANITIZE_TIER1_PASS`**. Single LITE-CEREMONY spine PR (DOC-OPS-006 §9).
Seat: `qwork NA-0676 qsl-protocol qsl-desktop qsl-server`, base `7c81d9ab`, all three repos
`startup_result=OK`, spine `ready_count=1 · queue_top_ready=NA-0676 · requested_lane_status=READY`.

## 1. The instrument, and why its clean result is evidence

`/srv/qbuild/evidence/NA-0676/tier1_scan.sh` — tracked text only, per pattern class, per repo.

**It was proved capable of returning positive before any clean result was trusted** (D612 §5.2/§9.4).
A throwaway file carrying one literal of **every** Tier-1 class was seeded; **all eight classes
returned positive**; the seed was removed and a fresh scan found no trace of it.

> ⚠ `RFC1918_10` found the seed **and nothing else** — independently confirming the census finding
> that the tree contains no `10.x` address at all. That is a result the scan could only produce by
> actually working, and it is worth more than the assertion it confirms.

Record: `/srv/qbuild/evidence/NA-0676/positive_control.txt`.

## 2. The personal-email class — first, and proven on its own

**Operator priority ruling: this class is the priority of the whole lane.** It was done first and is
proven separately rather than buried in a total.

**7 hits / 6 tracked files → 0**, verified across all five trees. Record: `email_class_proof.txt`.

**Every occurrence had arrived through the ENG-0072 filing and its lane records** — the finding whose
*subject* was that the seat came up with that address. **OBS-H one level up: the record of a leak,
written naively, republishes what it was filed about.** Five of the six landed in the two days before
this lane.

Substituted to *"the machine's global personal address"*. Every passage still says the seat inherited
the machine's personal identity instead of GH007 — the entire point of each sentence.

## 3. The full Tier-1 result

**76 hits across 19 tracked files → 0**, all in `qsl-protocol`. Baseline **matched D612 §1 exactly**;
no drift. Per-file delta **by field name only, never `original → replacement`** (OBS-H) — in
`/srv/qbuild/evidence/NA-0676/tier1_delta.md`.

**Zero Tier-1 hits in all five trees** — `qsl-protocol`, `qsl-desktop`, `qsl-server`,
`qsl-attachments`, `.github` — under the corrected pattern set (§5).

## 4. ⚠ FLAG-B3 DROPPED — its premise was measured false

B3 ruled the three satellite hits be fixed in-lane, because *"Lane C cannot adopt a gate in a repo
that fails it on day one."* **That rationale does not hold.** Measured against the D613 §2a tiering
the operator ruled at C2:

| enumerated hit | class | fails Lane C's gate? |
|---|---|---|
| `qsl-desktop/DECISIONS.md` — a build-path evidence pointer | **Tier 2a** | **No** — Tier 2a is not scanned at all |
| `qsl-desktop/docs/DESIGN_SPEC_AppendixF.md` — a retired rig's host name | **Tier 2b** | **No** — added-line scoped; a pre-existing tracked line is not an added line |
| `qsl-server/tests/NA-0002_…` — a home-path reference | **Tier 2a** | **No** — not scanned |

**Measured directly: the Tier-1 instrument returns exit 0 on both satellites, unfixed.** They already
pass the gate Lane C will install.

**The source of the error is D612 §2 as drafted.** It defines Tier 1 as *"network-identifying +
personal identity"* and then lists those three **Tier-2-class** hits underneath it. The B3 ruling
adopted that internal inconsistency. It is recorded here rather than left as a quietly unexecuted
instruction, because **a dropped ruling that goes unrecorded is indistinguishable from one that was
forgotten.**

Operator ruled option (a): **B3 dropped; Lane B ships as one spine PR;** the three hits stay by the
same B1 ruling that keeps Tier 2 everywhere else. The set had also **grown** since B3 was ruled —
`qsl-desktop/DECISIONS.md` now carries three such references, two of them added by **NA-0675**
the day before.

## 5. ⚠ The census CLASS LIST was incomplete — the OBS-AK ruling

A further private host name, used as a **network target**, was never in the census's class list.
**Operator ruling: it is TIER 1 — that meets D612 §2's own definition. The definition was right; the
enumerated list was short.**

The list had been assembled by naming the hosts the enumerator already knew about, and therefore
inherited every host the enumerator had forgotten. Added as a new host class; re-run across all five
trees; **3 further hits in 3 files**, sanitized in the account@host form so the **account** name
(Tier 2b, stays by B1) survives and the **host** name goes.

**The corrected pattern set carries to NA-0677 by the same ruling.** The general lesson, cheap here
and expensive later: **a pattern set assembled by enumeration is only as complete as the enumerator's
memory** — Lane C is about to install one as a merge gate.

> ⚠ **And the recursion fired, exactly where D612 §4.5 said it would.** The first draft of the
> D-1307 decision text **restated the host name it was recording the redaction of** — a leak
> introduced by the record of the fix, in the same commit that removed it. Caught by re-running the
> instrument against the closeout artifacts *before* committing, which is why §4.5 requires it.
> **This is the fourth occurrence of this shape in three days** (the NA-0674 mockup header, its
> closeout drafts, the NA-0676 enqueue entry, and now this). The pattern is not carelessness: **a
> record of a redaction has to refer to what it redacted, and the shortest way to refer to something
> is to name it.** The discipline that works is mechanical — scan the artifacts you just wrote,
> before you commit them, with the same instrument you used on the tree.
>
> ⚠ **And a second, quieter instance in the same draft, which the instrument could NOT have caught.**
> The as-built first named the new pattern class after the host — `HOST_<name>` — which republishes
> the literal to a human reader while being **invisible to the scanner**: `\bname\b` does not match
> inside `HOST_NAME`, because the underscore is a word character and kills the left boundary.
> Found by a case-insensitive read, not by the tool. **This is a live gap in the pattern design Lane C
> is about to install as a merge gate:** a word-boundary-anchored pattern misses the literal whenever
> it is embedded in an identifier, a snake_case symbol, or a compound token. Carried to NA-0677 as a
> pattern-design finding — **report only; nothing was changed in the gate.**

## 6. ⚠ The substitution convention — a RULING, not a deviation

D612 §4.2 offered RFC 5737 TEST-NET for command/config-shaped text, *"so a reader never copies a
working-looking address out of a runsheet-shaped block."*

On reading every occurrence, the command/config-shaped cases in this tree are **quotations of
observed output** — a Caddyfile as deployed, a certificate's SAN as issued, a `ca-set`/`server-info`
transcript as it ran, a GUI footer as it rendered. **Substituting a plausible literal inside a
quotation would fabricate an observation that was never made.** A placeholder is visibly a redaction;
a TEST-NET address is visibly a fact, and it would be a false one.

Surfaced before acting. **Operator ruling: a descriptive placeholder uniformly, no TEST-NET anywhere —
*"quotations must never contain fabricated observations."*** Recorded as a ruling.

Two fields kept a role name because the sentence is *about* the role: the relay's advertised `name`,
and the flight footer (which evidences *that the footer showed the configured relay* — the literal was
never the point).

## 7. Meaning preserved — checked by reading, and the check found damage

Every changed passage was re-read. **The backstop substitution introduced four grammatical breaks**
(two `the REAL the …` collisions, one `by the the …`, one sentence-initial lowercase). All four fixed
before commit. An exhaustive artifact re-scan then returned **five** further matches — **all five
verified PRE-EXISTING in `HEAD`** (enumeration letters such as *"PR-A the crate split"*), and
correctly left alone.

**Nothing about what any document asserts has changed.** A check that read "unreachable at
`<address>`" still tells the reader the host was unreachable at its LAN address.

## 8. TIER 2 — the deliberate, recorded residue (B1)

**Not touched, and the reason matters more than the count.**

| class | hits | files |
|---|---:|---:|
| `/srv/qbuild` | 4,184 | 587 |
| `/home/victor` | 1,153 | 204 |
| remote account name (a) | 678 | 79 |
| remote account name (b) | 85 | 17 |
| retired rig host name | 376 | 68 |

**These are not accidental leaks; they are documented operational facts.** `DOC-OPS-006` §3/§5a
*specifies* the directive, qwork-proof and tooling paths as the authoritative locations of those
artifacts; `qwork` proof records, testplans and evidence docs cite them because that is where the
artifacts are. **Replacing ~5,300 such strings would leave the governance spine unable to say where
anything lives — destroying load-bearing documentation to remove a low-value disclosure.**

Measured while drafting D613 and recorded there as C8: the citation convention adds those two path
classes to **17 of 30** and **10 of 30** recent non-merge spine commits respectively. They cannot be
gated even on added lines, which is why Lane C leaves them unscanned (Tier 2a) and gates only the
low-frequency names (Tier 2b).

## 9. History is out of scope — recorded verbatim, as D612 §6 requires

**Git history retains every original literal and was not rewritten.** A rewrite would force-push every
published repository, break every recorded sha in `DECISIONS.md`, `TRACEABILITY.md` and the journal,
invalidate the `qsc` rev pins in `qsl-desktop`, and violate GH007's no-force-push rule.

**The policy goal is a clean current tree and clean future commits.** The disclosure already made is
**not** undone by this lane, and the lane does not claim otherwise. Anyone with the repository can
read the originals in history; the operator should treat those addresses and names as **already
public**. Nothing in the census is a credential.

## 10. Claim boundary

A PASS asserts that **zero Tier-1 literals remain in the four published working trees plus `.github`**,
measured by an instrument proved capable of finding them. It asserts **nothing** about history,
nothing about Tier 2 (published as residue), and no security property. ⚠ This closeout is `docs_only`,
so the behavioural suites correctly **SKIP** on its merge — **its green proves the edit is well-formed
and nothing else**; the evidence is the positive control and the per-file delta.
