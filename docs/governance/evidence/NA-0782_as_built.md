# NA-0782 -- AS BUILT (IMPL): F1 THE rustls ADVISORY BUMP

Lane `NA-0782`. Impl seat (sonnet/medium), per `RULING_NA0782_formalization_2026-09-22.md`
(sha256 `1dc1d695f19b11985b4944ff0521c92696d7a631df21851e957c3ec7975a5a7d`) and
`tmp/main-red-formalization/DIRECTIVE_DRAFT.md` (sha256
`c0511229e1c11700ebcd9c0e45cc1c9ea21e0c6343df1b11e735e747a75f5e41`), both read in full and
sha-verified before use. F1 ONLY. F2 (the cargo-fuzz install step, `.github`) is NOT this
seat's act -- see section 5.

## 1. THE ADVISORY

RUSTSEC-2026-0285, "TLS 1.3 handshake messages incorrectly accepted across encryption level
boundaries", published 2026-09-14, severity 5.3 medium. Solution: upgrade to >=0.23.45.

## 2. STEP 2 -- X1 PRECONDITION (before the bump)

`cargo update -p rustls --dry-run` refreshed the crates.io index cache (it was stale at 44
versions before the refresh); `cargo info rustls` printed EXIT=0. The refreshed sparse index
cache for `rustls` was parsed directly: 46 distinct 0.23.x versions, newest `0.23.45`,
`yanked=false`. X1 precondition holds. Log: `tmp/na0782-impl/logs/02_x1_precondition.log`.

## 3. STEP 3 -- F1, THE ROOT LOCKFILE BUMP (sealed X1)

`cargo update -p rustls` (no `--precise`). Result, matching the sealed expectation exactly:

    git diff --stat
      Cargo.lock | 8 ++++----
      1 file changed, 4 insertions(+), 4 deletions(-)

    rustls        0.23.36 -> 0.23.45   checksum c665f33d... -> 0d41d731...
    rustls-webpki 0.103.13 -> 0.103.15 checksum 61c429a8... -> f3c3cf1d...

    Cargo.lock sha256 BEFORE: 8604b08b43aae1ef705aa4d9c5189b6e3b34d267f02d0e07731b409df93bf7dc
    Cargo.lock sha256 AFTER:  5d923b530dda775704a62dcc61b406fd44188ce6f218022da2ab8f2e7d7b18f2
      (== sealed X1)
    git status --porcelain: exactly " M Cargo.lock"

No file other than `Cargo.lock` changed. No `Cargo.toml` edit and no code change was needed
(the design's re-rule clause does not fire). Log: `tmp/na0782-impl/logs/03_f1_update.log`.

## 4. STEP 4 -- PROOFS

(a) `cargo audit --deny warnings`: `cargo-audit-audit 0.22.1`. EXIT=0. Output: "Loaded 1261
security advisories", "Scanning Cargo.lock for vulnerabilities (347 crate dependencies)", no
findings printed. Log: `tmp/na0782-impl/logs/04a_cargo_audit.log`.

(b) `cargo build --workspace --all-targets --locked`: EXIT=0, "Finished `dev` profile
[unoptimized + debuginfo] target(s) in 1m 10s"; the only warnings are pre-existing unused-import
lines in `qsl/qsl-client/qsc/tests/na0768_handshake_a1_offer.rs`, not introduced by this bump.
Cargo.lock sha256 AFTER the `--locked` build: still
`5d923b530dda775704a62dcc61b406fd44188ce6f218022da2ab8f2e7d7b18f2` (== X1, unchanged). Log:
`tmp/na0782-impl/logs/04b_cargo_build.log`.

Per `RULING_NA0782_formalization_2026-09-22.md` R7(a), the impl seat is Light and runs update,
audit, build ONLY; the sec 3.6(d) test step is struck from this seat's scope. CI's own
`qsc-sharded-suite` is the suite instrument (acceptance A).

## 5. WHAT MOVED IN THE TLS STACK, 0.23.36 -> 0.23.45 (carried from the draft sec 3.5)

44 source files changed, +1056 / -232 lines, across `client/{builder,client_conn,common,ech,hs,
tls12,tls13}.rs`, `server/{builder,hs,server_conn,tls12,tls13}.rs`, `conn.rs`, `quic.rs`,
`msgs/{base,enums,handshake,persist}.rs`, `msgs/deframer/handshake.rs`, `suites.rs`, `stream.rs`.
rustls ships no CHANGELOG in its published `.crate`; this source-diff summary stands in for it
(ruling R5).

THE ADVISORY'S OWN FIX, located in `src/msgs/deframer/handshake.rs`:

    -    /// We are "aligned" if there is no partial fragment of a handshake
    -    /// message.
    +    /// We are "aligned" if there are no pending handshake messages, complete or partial.
         pub(crate) fn is_aligned(&self) -> bool {
    -        self.spans
    -            .iter()
    -            .all(|span| span.is_complete())
    +        // If we have any handshake spans, either:
    +        // - it is a full pending handshake message, or
    +        // - it is not, which means it's a partial fragment
    +        !self.is_active()
         }

In 0.23.36 a COMPLETE but still-pending handshake message counted as "aligned", so a message
buffered under one encryption level could be accepted after the level changed. 0.23.45 requires
no pending handshake data at all. This is the advisory, in the code.

## 6. F2 -- NOT THIS SEAT'S COMMIT

F2 (`cargo +nightly install cargo-fuzz --version 0.13.2 --locked` in
`.github/workflows/qsc-adversarial.yml`) is the OPERATOR's separate commit on this branch
(`na0782-impl`), applied from the proposal diff `tmp/main-red-formalization/workflow-proposal.diff`,
sha256 `1187cb85d1e495c7c6620e9fff60fc250cd0a5d576b81f36af3b4715ec87cd49`. This seat did not touch
`.github` and did not apply that diff.

## 7. ACCEPTANCE, RESTATED FROM THE DRAFT SEC 7

ACCEPTANCE A. On the impl merge commit, read from `gh run list --branch main` and NOT from a PR
  page, all green: `public-ci` jobs `advisories`, `public-safety`; `qsc-adversarial` jobs
  `qsc-adversarial-smoke`, `qsc-adversarial-miri`; AND, because `public-safety` on push waits on
  them, `qsc-sharded-suite`, `macos-qsc-sharded-suite`. A green run is not a complete run (SR-05
  as amended): the count is reconciled, not believed.
ACCEPTANCE B. `cargo audit --deny warnings` EXIT=0 in the impl PR's evidence, at the merged
  lockfile, output quoted verbatim with the instrument's own version printed (section 4(a)).
ACCEPTANCE C. `git diff --stat main..impl` names ONLY: `Cargo.lock`,
  `.github/workflows/qsc-adversarial.yml`, and the records/evidence files. It does NOT name
  `qsl/qsl-client/qsc/fuzz/Cargo.lock`.
ACCEPTANCE D. Landing verified: two parents, the STATE line advanced, the declaring ids present
  on declaring forms, the evidence file in the tree.

## 8. CLAIM BOUNDARY

ONE BOX. Every figure in sections 2-4 was measured on this box's checkout of
`lanes/NA-0782/repo/qsl-protocol` at branch `na0782-impl` from main `80367aa690ae16db56992a7aba
515a08d0952e60`, with stable rustc 1.95.0 and cargo-audit 0.22.1. CI runs different toolchains
(public-ci's `advisories` job pins 1.85.1 and cargo-audit 0.22.0). NOTHING HERE ESTABLISHES A CI
RESULT; CI is the instrument for acceptance A, read from the branch after merge, not from this
seat's local run. Synthetic passes on this box establish nothing about production or the real
relay. This tree is named by `$QBUILD_ROOT` only; no absolute host path, username, LAN address,
or former-root path appears above.
