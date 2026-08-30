# NA-0773 — RNG-FAILURE TEST SEAM REPAIR — TEST PLAN AND RUN RECORD

**Decision:** `D-1416`. **Base:** qsl-protocol main `c6687bbdd19927e3f3ed3e0f63f8c1672b804730`,
re-derived bare and unpiped at the named `github` remote. **Product change:** one line at
`qsl/qsl-client/qsc/src/vault/mod.rs:726`. **No test weakened, skipped, deleted or re-aimed.**

## 1. WHAT IS UNDER TEST, AND WHAT THE FINISH LINE IS

`RULING_NA0773_002_20260830.md` sec 2 sets it: **"THE 19 HAVE RUN AND THE ERR-PATH CLEANUP
BEHAVIOUR IS RECORDED"**, restated from *"the 27 have run"*. This plan therefore records the run of
**all 27 gated tests by name**, and states plainly what their result does and does not establish.

## 2. THE RED ARM — MEASURED BOTH WAYS ON THIS TREE, EXPECTATION WRITTEN FIRST

**Expectation, written before running:** with the cfg, exit 101 and four errors at
`vault/mod.rs:570,:578,:728,:733`; without the cfg, exit 0.

| arm | command | before the repair | after the repair |
|---|---|---|---|
| seam, lib | `RUSTFLAGS="--cfg qsc_rng_failure_test_seam" cargo check -p qsc --lib` | **exit 101**, 4 errors | **exit 0**, 0 errors |
| seam, tests | `RUSTFLAGS="--cfg qsc_rng_failure_test_seam" cargo check -p qsc --tests` | **exit 101** — halts on the lib's 4 errors, never reaches a test target | **exit 0**, 0 errors, 2 unrelated warnings |
| ordinary | `cargo check -p qsc --lib` (no cfg) | **exit 0** | **exit 0** |

⇒ **the arms differ and the only variable is the cfg.** The `--tests` row is the discharge of the
specification's largest declared uncertainty (`STOP_NA0773_002` claim boundary `n3`): **the one line
is necessary AND sufficient**; nothing else had drifted in 46 days.

## 3. THE DEFAULT-BUILD CONTROL

The changed line lies strictly between the `#[cfg(qsc_rng_failure_test_seam)]` gate at `:725` and
the `#[cfg(not(qsc_rng_failure_test_seam))]` twin at `:736`, so **no default build compiles it**.
The cfg is set by **no build surface**, re-measured at this base: `.cargo/config*` absent ·
`**/Cargo.toml` **0** · `**/build.rs` **0** · `.github` **0 files** · `scripts` **0**.
**POSITIVE CONTROL, so the zeros are real:** the same instrument finds `cargo` in **11**
`.github` files. The SR-15 read additionally constructed the *default build's view* of the file —
every seam-gated item stripped — before and after: both `2ad568d6a001e515fdf982c4f269af47cf387e0a22b9cba9e52fc4b9b33d8cd9`, **identical**, with a
negative control (renaming one symbol in that view changes the hash) proving the comparison can fail.
⚠ **BOUNDARY:** this is byte-identity of the **source the default build sees** — weaker than
byte-identity of the emitted artifact, which needs build determinism this lane did not establish.
**It is not claimed.**

## 4. THE 27, BY NAME, WITH THEIR RESULT

Run on the exact committed tree, each target in its own invocation, **keyed by unique target path
and census-reconciled** (`PR-7`: a green run is not a complete run).

**CENSUS 27 SPECIFIED · 27 EXECUTED · 26 PASSED · 1 FAILED · 0 IGNORED · 0 NOT OBSERVED.**

| target | test | result |
|---|---|---|
| `a2_signature_provider_rng_failure` | `a2_signature_rng_failure_emits_no_a2_output` | ok |
| `a2_signature_provider_rng_failure` | `common_na0463_markers` | ok |
| `b1_signature_provider_rng_failure` | `b1_signature_rng_failure_writes_no_responder_state_or_b1` | ok |
| `b1_signature_provider_rng_failure` | `common_na0461_markers` | ok |
| `cli_identity_rotation_provider_rng_failure` | `cli_identity_rotate_kem_rng_failure_writes_no_partial_rotation_state` | ok |
| `cli_identity_rotation_provider_rng_failure` | `cli_identity_rotate_sig_rng_failure_writes_no_partial_rotation_state` | ok |
| `cli_identity_rotation_provider_rng_failure` | `common_na0469_markers` | ok |
| `kem_provider_rng_failure` | `kem_keypair_rng_failure_writes_no_identity_or_session_state` | ok |
| `kem_provider_rng_failure` | `kem_encap_rng_failure_writes_no_responder_state_or_b1` | ok |
| `kem_provider_rng_failure` | `common_na0458_markers` | ok |
| `lazy_identity_provider_rng_failure` | `lazy_identity_kem_rng_failure_writes_no_identity_state` | ok |
| `lazy_identity_provider_rng_failure` | `lazy_identity_sig_rng_failure_writes_no_identity_state` | ok |
| `lazy_identity_provider_rng_failure` | `common_na0465_markers` | ok |
| `legacy_identity_public_record_provider_rng_failure` | `legacy_identity_migrate_sig_rng_failure_writes_no_partial_upgrade_state` | ok |
| `legacy_identity_public_record_provider_rng_failure` | `public_record_upgrade_sig_rng_failure_writes_no_partial_upgrade_state` | ok |
| `legacy_identity_public_record_provider_rng_failure` | `common_na0467_markers` | ok |
| `na0742_invite_finish_scan_producer_acks` | `t8_the_a2_sig_failure_exit_emits_no_producer_ack` | ⚠⚠ **FAILED** |
| `rng_failure_behavior` | `common_na0449_markers` | ok |
| `rng_failure_behavior` | `handshake_session_id_rng_failure_writes_no_pending_state` | ok |
| `rng_failure_behavior` | `vault_rng_failure_writes_no_vault_file` | ok |
| `rng_failure_behavior` | `session_store_rng_failure_writes_no_session_blob` | ok |
| `rng_failure_residual_surfaces` | `common_na0452_markers` | ok |
| `rng_failure_residual_surfaces` | `route_default_token_rng_failure_writes_no_vault_file` | ok |
| `rng_failure_residual_surfaces` | `contact_add_without_route_token_writes_no_contact_state` | ok |
| `rng_failure_residual_surfaces` | `attachment_id_rng_failure_writes_no_stage_or_journal` | ok |
| `rng_failure_residual_surfaces` | `attachment_cek_rng_failure_writes_no_stage_or_journal` | ok |
| `rng_failure_residual_surfaces` | `attachment_nonce_prefix_rng_failure_writes_no_stage_or_journal` | ok |

⚠ **Eight of the 26 passes are `common_na04xx_markers` fixtures that assert nothing** (`ENG-0268`)
and pass trivially. **The assurance is in the other 18**, of which **16** pin the fail-safe property.

## 5. THE ONE FAILURE, AND THE RULE WRITTEN BEFORE THE RE-RUN

`t8_the_a2_sig_failure_exit_emits_no_producer_ack`, at
`qsl/qsl-client/qsc/tests/na0742_invite_finish_scan_producer_acks.rs:1462`. **Both antecedent
assertions PASSED**, so the exit under test was genuinely taken.

```
assertion `left == right` failed: A FRAME WHOSE A2 NEVER LEFT MUST NOT BE ACKED. The session was
committed, but the peer received nothing; acking here would retire a frame whose effect never happened.
  left: 1
 right: 0
```
The product's own markers, in emission order:
```
QSC_MARK/1 event=session_store ok=true format=v3 enc=aead
QSC_MARK/1 event=handshake_reject reason=sig_sign_failed
QSC_MARK/1 event=relay_pull_diagnostic ... op=ack ack_mode=lease acked_count=1
QSC_MARK/1 event=producer_ack caller=finish sent=1 acked=1
```

**THE RULE, WRITTEN BEFORE THE RE-RUN:** *passes alone = contention; fails alone = real.*
Re-run alone and single-threaded (`--exact --test-threads=1`, 11 filtered out): **FAILED again,
exit 101, 51.57s.** ⇒ **REAL.** Filed as **`ENG-0269`**, argued at `P2`, **NOT REPAIRED** — the
ruling is explicit that a failing assertion is filed with its severity argued from what it asserts.

⚠ **It cannot redden any required check**, and that is the problem restated rather than a
mitigation: no CI job compiles the cfg, so the default build runs this file's `#[cfg(not(...))]`
companion instead. **The defect is invisible to every gate the project runs** — `ENG-0197`'s thesis,
now demonstrated on a live defect rather than argued.

## 6. WHAT A GREEN RUN DOES *NOT* PROVE — CARRIED HERE BECAUSE THE RULING REQUIRES IT

- **NOT** that the product fails safe when the OS RNG actually fails (**`ENG-0265`**): every draw
  goes through `OsRng::fill_bytes`, which **panics**; the seam returns `Err` *instead of* drawing;
  and `assert_no_secret_output`, reached by **10 of the 19**, asserts `"panicked"` is **absent**.
- **NOT** the **13 shipped randomness draws the seam cannot reach** (**`ENG-0266`**), including the
  vault master key at `vault/mod.rs:1476` and both persisted store keys; nor the **5 wrapped labels
  no test forces**, all AEAD nonce draws.
- **The coverage figure for the fail-safe property is 16**, not 27: 27 declared, 8 assert nothing,
  1 is not an RNG test, 1 pins a surviving partial write (`ENG-0267`), 1 pins ack behaviour by design.
