# NA-0655 testplan — satellite community-health docs (D591, D-1278)

Docs-only cross-repo lane; every check below executed 2026-07-18 with the
result recorded. Raw command output lives in the proof root
(`/srv/qbuild/tmp/NA0655_satellite_community_health_docs_*/`); this table is
the class summary.

| # | Check | Where | Result |
|---|-------|-------|--------|
| 1 | qwork proof (startup_result=OK, lane=NA-0655, head==origin/main==`35bd5fa8`, ready_count=1, top=NA-0655, clean ×3, shared_target_ready=yes) | spine | PASS |
| 2 | D-1278 next-and-absent (canonical count 0); anchored `Status: READY` ×1; disk < 95%; /backup/qsl mounted | spine | PASS |
| 3 | Main health on `35bd5fa8`: 8/9 SUCCESS incl. public-ci; formal-ci in normal window | spine | PASS |
| 4 | Fresh clones (house rail); heads UNMOVED: `3cc551a8` / `a3ebad2f` / `31558fa4` — Phase-0 STOP not triggered | satellites | PASS |
| 5 | File gap live ×3 (case-insensitive, maxdepth 2, incl. `.github/`): zero pre-existing S/CoC/C files | satellites | PASS |
| 6 | Satellite decision counters next-and-absent: tops D-0012/D-0010/D-0017; D-0013/D-0011/D-0018 zero hits pre-lane | satellites | PASS |
| 7 | PVR recorded AS FOUND: ENABLED on qsl-server, qsl-attachments, qsl-protocol, qsl-desktop (operator F2 step confirmed live); website endpoint 404 (unavailable on private) | org/API | PASS (recorded) |
| 8 | SECURITY.md reporting section VERBATIM from spine (mechanical head-7 extraction; `cmp` proof) | ×3 | PASS |
| 9 | SECURITY.md scope sections = the D591-prescribed per-repo text; reporting only | ×3 | PASS |
| 10 | CODE_OF_CONDUCT.md sha256 `2cbf021e…` — identical to spine source | ×3 + spine | PASS |
| 11 | CONTRIBUTING.md states the repo's REAL gate in its CI's exact step order; no spine-only tool references | ×3 | PASS |
| 12 | DECISIONS append in the repo's own canonical form (D-0013 / D-0011 / D-0018), provenance D591/NA-0655/D-1278 | ×3 | PASS |
| 13 | Delta census: EXACTLY four files per satellite; README/LICENSE/NOTICE absent from every diff | ×3 | PASS |
| 14 | Assurance-language census over all nine files (audited/secure(d)/hardened/production-ready/guarantee, ci, word-bounded): 0 hits | ×9 files | PASS |
| 15 | `git diff --check` clean | ×3 + spine | PASS |
| 16 | Added-line private-material scan: 0 hits | ×3 | PASS |
| 17 | Website guards WITH files present, UNCHANGED: scan-overclaims files=40 PASS; integrity guardrail PASS; zero guard edits | website | PASS |
| 18 | Satellite PR checks: `rust` PASS on #63 (required), `rust` PASS on #40 (required), `website-validation` PASS on #32 (observed; not required — private plan) | PRs | PASS |
| 19 | Spine closeout: goal-lint local OK (literal `Goals: G4` from creation); `cargo metadata --locked` OK; audits green; fmt --check = known pre-existing residue only | spine | PASS |
| 20 | Queue flip: STATE → `READY=NONE \| HIGHEST_NA=0655 \| HIGHEST_D=1278`; anchored `Status: READY` ×0 after flip | spine | PASS |

STOP conditions armed and NOT fired: satellite head moved; target file
pre-existing; assurance-claim wording need; website guard trip; forbidden-path
need (README/LICENSE/NOTICE, code, deps, workflows, settings, qsl-desktop);
required check red.
