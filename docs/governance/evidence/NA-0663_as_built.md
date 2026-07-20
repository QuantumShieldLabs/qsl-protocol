# NA-0663 as built — qsc client TLS trust (D599, D-1286)

Directive: QSL-DIR-2026-07-20-599, sha256
`925b56cd2f1861fcc2334bcbdcf6b9db517b1df86c135aece7d07c31409067a2`, 611 lines,
APPROVED 2026-07-20 (F1 UNION; F2 typed detection + the enumerated vocabulary,
no ErrorCode variant; F3 the rcgen + tokio-rustls dev rig), AMENDED IN PLACE at
execution by the operator ruling of 2026-07-20 recorded in §2.

Base: main == origin/main == `e5313fa3` (the NA-0663 seating merge; parents
`83b6b4a4` + `b269c220`). qwork-proven: startup_result=OK, head==origin_main==main,
worktree/index/untracked clean, ready_count=1, queue_top_ready=NA-0663.

## 1. WHAT SHIPPED

**Part 1 — the OS trust store is honored.** `qsc/Cargo.toml` gains the reqwest
feature `rustls-tls-native-roots`; `rustls-tls` STAYS, so roots are the UNION of
the baked-in webpki set and the system set. Nothing trusted before this change
stops being trusted. `SSL_CERT_FILE`/`SSL_CERT_DIR` now behave as standard
runbooks describe.

**Part 2 — an explicit CA file, ADDITIVE.** Resolution mirrors the auth-token
precedent exactly: env `QSC_RELAY_CA_FILE` → env `RELAY_CA_FILE` → vault secret
`tui.relay.ca_file`. Every certificate in the bundle is added via
`add_root_certificate`; nothing is ever removed or replaced. FAIL-CLOSED: a
configured-but-unusable CA yields a typed failure and never silently proceeds.
Deliberate recorded asymmetry vs the token file: **NO 0600 gate** — a CA is
public material and a world-readable CA file is correct.

**Part 3 — a distinguishable typed trust outcome.** A certificate-verification
failure on push/pull/ack now returns `relay_tls_untrusted` instead of collapsing
into the opaque per-op value. Detection is BY VALUE, not by string.

## 2. THE OPERATOR RULINGS AT EXECUTION

1. **cmd/mod.rs admitted to STRICT SCOPE**, bounded to the three additive
   `RelayCmd` variants, every existing variant byte-identical. The directive
   required additive CLI verbs while forbidding the only file in which a clap
   verb can be declared: `RelayCmd` lives at `cmd/mod.rs:595`, and all 17
   `#[derive(Subcommand)]` enums are there with ZERO elsewhere in `qsc/src`;
   `main.rs` only consumes them. NA-0645 (`2efc9dab`), the lane that created the
   token setters this directive says to mirror, touched both files for exactly
   this reason. The withholding was a drafting oversight, not a boundary.
2. **`pem` + `yasna` ACCEPTED** in the lockfile delta: the execute paste's
   three-crate list restated the PART-1 ruling (the reqwest feature change);
   rcgen's transitives are the separate F3-sanctioned expectation, enumerated
   verbatim in the directive's family-6 text.
3. **`relay_ca_file_store_failed` ACCEPTED** as a vocabulary expansion from four
   values to five. The F2 bound governs the TRANSPORT ERROR CONTRACT — the
   caller-visible outcomes of push/pull/ack. A vault-write failure raised by a
   CLI setter verb is not on that contract; it is the same class as the
   pre-existing `relay_token_store_failed`, `relay_token_file_store_failed` and
   `relay_inbox_token_store_failed`, and can only arise from the new verbs.
   Reusing `relay_ca_file_invalid` was refused as misleading.
4. **`require_unlocked` on all three verbs including `ca-show`** — confirmed as
   applied; consistency across the trio preferred to a lock-tolerant exception.

## 3. FILES (7 changed + 1 new; +395/-19 before tests)

| file | what |
|---|---|
| `qsc/Cargo.toml` | exactly the three ruled movements, landed LAST |
| `Cargo.lock` | mechanical consequence only |
| `src/transport/mod.rs` | vocabulary, CA plumbing, constructor, classifier, pub surface, 3 call sites |
| `src/attachments/mod.rs` | the 5 constructor swaps ONLY |
| `src/store/mod.rs` | the new secret-key const |
| `src/cmd/mod.rs` | 11 lines, PURE ADDITION: `CaSet`/`CaClear`/`CaShow` |
| `src/main.rs` | ONE additive hunk: the 3 handlers |
| `tests/NA_0663_relay_tls_trust.rs` | NEW, the only test file touched |

`lib.rs` was NOT touched: the pub surface lives in `transport`, already
`pub mod transport`, so `qsc::transport::relay_ca_file_*` is GUI-reachable with
no re-export — one fewer file than the design lock anticipated.

SCOPE GUARD: zero forbidden paths in the diff. `adversarial/route.rs` untouched
and unneeded — `validate_relay_endpoint_url` already admits any https URL
(`route.rs:58-59`, unconditional), so the loopback-https rig needed no
endpoint-validation change. `model/mod.rs` untouched: ErrorCode stays at 12
variants. No qsl-desktop touch of any kind.

## 4. THE HOUSE CONSTRUCTOR

`relay_http_client() -> Result<HttpClient, RelayHttpClientError>` in
`transport/mod.rs` replaces ALL EIGHT `HttpClient::new()` sites (transport
push/pull/ack; attachments ×5). Built through `ClientBuilder`, never
`Client::new()` — which panics; the builder form must not.

`RelayHttpClientError { CaFile(&'static str), Build }` is an internal Rust type,
NOT a caller-visible value: `CaFile` carries an enumerated value the call site
returns verbatim, and `Build` carries nothing so each site reports its OWN
pre-existing failure value (`relay_inbox_push_failed`, `relay_inbox_pull_failed`,
`relay_ack_failed`, `attachment_service_create_failed`, `_status_failed`,
`_upload_failed`, `_commit_failed`, `attachment_fetch_failed`). That is how
construction and CA-load failures land in each site's EXISTING error contract
without inventing a build-failure value.

**The webpki-continuity pin:** the constructor never calls
`tls_built_in_root_certs(false)`. That ABSENCE is the pin. An in-suite
public-endpoint probe would breach the zero-external-networking discipline, so
the source pin is the honest substitute — recorded as such, not claimed as an
empirical proof of webpki continuity.

## 5. THE TYPED DETECTION, AND THE BUG THE TESTS CAUGHT

Detection walks the std error source chain and matches
`rustls::Error::InvalidCertificate(_)` by VALUE.

The first implementation walked `source()` alone and **did not fire** — the
untrusted case still reported the opaque value. Root cause, now pinned in a
source comment: tokio-rustls reports a handshake refusal as
`io::Error::new(InvalidData, rustls::Error)`, and `std::io::Error::source()`
delegates to the INNER error's source rather than yielding the inner error
itself. Without `get_ref()` the walk steps straight past the rustls value. This
was found by family 3 failing, not by review, and the fix is verified by the
same test going green.

`rustls` is a TYPES-ONLY direct dependency with `default-features = false`
MANDATORY: rustls declares `default = ["aws_lc_rs", "logging",
"prefer-post-quantum", "std", "tls12"]`, so defaults would drag the aws-lc-rs
provider stack in. The active provider stays **ring**.

Existing diagnostic byte-identity: `relay_push_error_class_for_send_error` is NOT
modified, so the string-matched `"tls_error"` diagnostic class and the whole
field set stay byte-identical. The new outcome flows only through the EXISTING
`qsc_error` field.

Scope honesty: rustls reports the whole certificate-verification class (unknown
issuer, name mismatch, expiry) as `InvalidCertificate`. The typed outcome covers
that CLASS, which IS "certificate not trusted" for taxonomy purposes. Finer GUI
wording is slice B's to refine over the typed value.

## 6. LOCKFILE DELTA — 6 additions, 0 removals, 0 version changes (386 → 392)

| crate | version | tree | provenance |
|---|---|---|---|
| `rustls-native-certs` | 0.8.4 | production | reqwest `rustls-tls-native-roots` |
| `openssl-probe` | 0.2.1 | production | its Unix SSL_CERT_FILE/DIR helper |
| `schannel` | 0.1.29 | production (Windows target) | its Windows store |
| `rcgen` | 0.13.2 | **dev-only** | the F3 rig |
| `pem` | 3.0.6 | **dev-only** | transitive of rcgen |
| `yasna` | 0.5.2 | **dev-only** | transitive of rcgen |

Placement PROVEN by `cargo tree -i`, not assumed: `rustls-native-certs` resolves
under reqwest/hyper-rustls in the production tree; `rcgen`/`pem`/`yasna` are
reachable ONLY through `[dev-dependencies]`.

**TRIPWIRE CLEAR:** `aws-lc-rs` 0 and `aws-lc-sys` 0 at base AND head; `ring` 1
unchanged. The `rustls` types-only line contributed ZERO entries (0.23.36 before
and after); `tokio-rustls` contributed ZERO (0.26.4, matched by the `"0.26"`
spec). Both directive STOP conditions untripped.

## 7. A RECORDED PROCESS FINDING — `cargo fmt --all` IS UNSAFE IN THIS TREE

Running `cargo fmt --all` during validation reformatted **45 files**, including
forbidden paths (`lib.rs`, `vault/`, `handshake/`, `contacts/`, `timeline/`,
`adversarial/`) and `tests/NA_0640_full_stack_e2e.rs` — a scope breach and a
byte-identity STOP, both caught before commit and fully reverted.

Cause: the base tree is NOT rustfmt-clean. With this lane's changes stashed,
`cargo fmt --all -- --check` still reports drift at base (e.g.
`adversarial/binding_fuzz.rs:320`, `attachments/mod.rs:1721`). There is also NO
`cargo fmt` gate in `.github/workflows/` at all.

Remediation applied: all out-of-scope files reverted; the four in-scope source
files restored to base and this lane's edits re-applied surgically, so every
remaining hunk is intentional. Byte-identity re-verified afterwards.

Recommendation for future lanes: never run `cargo fmt --all` here; format only
the touched region, or fix the drift in a dedicated micro-lane (the NA-0651
rustfmt-drift micro-lane remains owed).

## 8. PROOFS

- **New tests: 11 passed, 0 failed** (`NA_0663_relay_tls_trust`), families 1-4
  plus the pub-surface check.
- **Full suite (head):** see §9 — figures recorded post-run.
- **Phase-0 baseline (base `e5313fa3`):** 109 binaries, **423 passed, 0 failed,
  1 ignored**, exit 0, 90m44s. The 1 ignored is
  `attachment_large_local_roundtrip_proof` ("local large-file proof only"),
  ignored by design.
- **Byte-identity:** EVERY pre-existing test file byte-identical to base,
  `NA_0640_full_stack_e2e.rs` included (git-diff proven). NA-0640 e2e green at
  base: 2 passed, 115.72s.
- **Base-binary A/B (family 5):** the stashed base binary
  (sha256 `5dc5accc…`) vs head over a fixed NO-NETWORK exercise set covering the
  pre-existing relay verbs' success paths, error paths, locked-vault refusal,
  and the adjacent `status` / `config get` surfaces — **byte-for-byte
  IDENTICAL**. The ONLY intended CLI surface delta is `relay --help` gaining
  exactly three lines for the new verbs.
- **Audits:** root `cargo audit --deny warnings` EXIT 0 (392 deps); nested qsc
  fuzz `cargo audit --deny warnings` EXIT 0 (287 deps; that lockfile untouched).
- `cargo metadata --locked` OK — no resolution drift.
- `git diff --check` clean.
- **Added-line private-material scan: ZERO** PEM/key blocks. The new test file
  contains ZERO embedded certificate or key material — all of it is generated at
  runtime. The only hex32+ strings in the whole diff are the six new Cargo.lock
  checksums (zero outside the lockfile).
- **No-bypass:** zero occurrences of any accept-invalid / skip-verify /
  accept-any-certificate needle across `src/`, `tests/` and `Cargo.toml`,
  pinned as an executable test with no exemption list.
- **clippy:** `-D warnings` fails at BASE with pre-existing `result_unit_err`
  findings in `lib.rs` (`push`, `bounded_retry`) — a file this lane leaves
  byte-identical. Not lane-caused; related in kind to the ledgered ENG-0032
  clippy-1.95 lint debt. No clippy or fmt gate exists in CI.

## 9. SUITE FIGURES (head) — DELTA == EXACTLY THE NEW FILE

Head run: `HEADSUITE_START 20260720T171716Z` → `END 20260720T190542Z`, **EXIT 0**,
108m26s, branch `na0663-qsc-client-tls-trust`.

| | base `e5313fa3` | head | delta |
|---|---|---|---|
| test binaries | 109 | 110 | **+1** |
| passed | 423 | 434 | **+11** |
| failed | 0 | 0 | 0 |
| ignored | 1 | 1 | 0 |

+1 binary and +11 tests correspond EXACTLY to
`tests/NA_0663_relay_tls_trust.rs` (11 passed, 19.98s). No pre-existing count
moved in either direction. Every one of the 110 binaries reported `ok`; zero
non-ok result lines and zero failure/panic needles across the whole log.

Both runs were derived on the same machine with the same toolchain
(rustc 1.95.0, cargo 1.95.0) and the same shared target dir, and both ran
detached so neither could be truncated mid-run.

## 10. NOT CLAIMED

- NOT that any GUI exists — slice B owns the Server pane and remains OWED.
- NOT that the inspiron LAN private-CA path was exercised live; slice-B
  acceptance owns the live-rig demo.
- NOT that transport security improved beyond the stated trust change.
- NOT that webpki continuity was empirically probed (feature presence + the
  source pin only, as recorded in §4).
- NOT that the external-review gate moved.
- No public / production / crypto-complete / attachment-complete / bug-free /
  vulnerability-free claim is made anywhere.

## 11. COLLATERAL, RECORDED AND NOT FIXED

`apps/qsl-tui` is a live workspace member with its own reqwest
(`{"json","rustls-tls"}`, async, `Cargo.toml:11`). Cargo resolver-v2 feature
unification means a WORKSPACE-WIDE build of qsl-tui gains native-root trust from
qsc's feature line, while a standalone `-p qsl-tui` build does not. qsl-tui
source is BYTE-IDENTICAL. This is a known side effect of workspace feature
unification, not a change to that binary.

## 12. NOTED FOR SLICE B

`relay_ca_file_show()` resolves through `vault::secret_get`, which fails closed
when the vault is locked; the status then reports `configured=false` rather than
"unknown". A locked vault therefore reads as "no CA configured" through the pub
surface. This matches base token-accessor behaviour and is NOT lane-caused —
recorded here so the GUI lane does not rediscover it. No change this lane
(operator ruling Q4).
