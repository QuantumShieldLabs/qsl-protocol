# NA-0646 test plan — Extract qsc-core (D582): PR-A crate split + PR-B exit->Result

Status: PR-A COMPLETE (PR #1573); PR-B PENDING (extends this plan at closeout).
Lane: ONE lane, TWO sequential PRs (D582). PR-A = move + visibility only,
SEMANTIC DELTA ZERO. PR-B = exit->Result on the five funnels. The compiled qsc
binary must produce BYTE-IDENTICAL stdout/stderr/exit codes throughout.

## The byte-identity prover (landed in PR-A; the load-bearing proof of the lane)

Runner: `scripts/local_ops/na0646_byte_identity_prover.sh` — captures
stdout + stderr + exit code per corpus case into a capture tree; two trees are
compared with `diff -r` (empty output = byte-identical). Fixture (vault +
identity + pinned contact peer-0) is created ONCE and reused so key material is
identical across captures.

Corpus (14 cases):
- S1 plain &'static-code error (print_error_marker family): `send` without
  `--file` → `error code=send_file_required`, exit 1.
- S2 code-via-ErrorCode (print_error family): `doctor --check-only --export
  /nonexistent/dir/x.json` → `error code=io_write_failed`, exit 1.
- S3 code+kv (require_unlocked): `contacts add` with the vault locked →
  `error code=vault_locked op=contacts_add reason=explicit_unlock_required`, exit 1.
- S4 dynamic-reason (protocol_inactive_exit): `send --file …` to the pinned
  contact with no protocol state → `error code=protocol_inactive
  reason=missing_seed`, exit 1.
- S5 marker-THEN-error (file_xfer_reject): `file send --path /nonexistent` →
  `file_xfer_reject …` marker THEN `error code=file_xfer_read_failed`, exit 1.
- S6 usage exit(2) (util_sanitize): `util sanitize` without `--print` → usage
  on stderr + `qsc_mark`, exit 2 (must STAY 2 through PR-B).
- H1-H7 happy paths: help stub (no args), status, config set/get
  policy-profile, identity show, contacts show, util sanitize --print.
- N1: `--unlock-passphrase-env <wrong-env>` → `vault_passphrase_env_retired`, exit 1.

Exclusion (recorded): the send/receive ROUND is not in the corpus — send output
embeds fresh-nonce-derived material and is not byte-deterministic run-to-run;
the round is covered behaviorally by the standard suite + the NA-0640 e2e.

## PR-A results (2026-07-15, branch na0646-pr-a-crate-split vs base d3f4df7d)

- Determinism: two pre-move corpus runs `diff -r` EMPTY (byte-identical).
- **PR-A byte-identity: PASS** — pre-move BEFORE vs final-tree AFTER `diff -r`
  EMPTY across all 14 cases.
- Purity (machine-checked): 11 module files = 119 changed lines ALL
  visibility-only; new lib.rs+main.rs = 2,836 verbatim + 94 visibility-widened
  + 46 crate-wiring lines, 0 unexplained; old main.rs 0 lines lost;
  `git diff --color-moved=dimmed-zebra` concurs (2,192 moved-out / 2,183 moved-in).
- `cargo check --all-targets -p qsc`: 0 errors / 0 warnings.
- Full local `cargo test -p qsc`: **405 passed / 0 failed / 1 ignored
  (pre-existing) across all 107 result sets, exit 0** (niced, --test-threads=3
  for bounded host load; semantics-neutral). Baseline reconciliation BY NAME vs
  NA-0645 (422/0/1 across 107): exactly the 17 adversarial/envelope unit tests
  that ran TWICE under the old bin+lib double-compile (64 executions of 47
  unique unit tests) now run ONCE — no test lost, zero test-file edits.
- NA-0640 e2e within the run: 2 passed / 0 failed (116.53s), zero e2e edits.

## PR-B plan (pending — extends at closeout)

- BEFORE baseline = the corpus captured at PR-A's MERGED state; AFTER = PR-B
  head; `diff -r` must be EMPTY (S6 proving exit(2) stayed 2).
- WF-0017 non-vacuity: a deliberately reordered emission (the require_unlocked
  kv order) must turn the differ RED; demonstrate, record, revert.
- Full local suite green + the NA-0640 e2e re-run green (zero e2e edits).
- Special scrutiny: transport::receive_pull_and_write before/after with the
  explicit control-flow-equivalence argument (8 in-loop exits incl. the
  ENG-0042 seam — PRESERVED, not fixed).
