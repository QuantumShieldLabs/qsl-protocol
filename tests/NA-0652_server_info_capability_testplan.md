# NA-0652 Testplan — qsl-server GET /v1/server-info capability document (D588, D-1275)

Goals: G4, G5

Execution surface: qsl-server PR #62 (`na-0652-server-info` @ `8c5627e3` off
`8e4ea278`), run locally in the lane's fresh clone. All rows executed 2026-07-17;
raw log `/srv/qbuild/tmp/NA0652_server_info_capability_20260717T170500Z/full_suite.log`.

| # | Check | Method | Result |
|---|---|---|---|
| 1 | qwork proof invariants (spine) | `.qwork/startup.qsl-protocol.kv` inspection | PASS — OK / NA-0652 / head==origin==`07c38254` / clean ×3 / ready_count=1 / top=NA-0652 |
| 2 | D-1275 next-and-absent; sole READY | grep DECISIONS.md / NEXT_ACTIONS.md | PASS — D-1274 ×1, D-1275 ×0; anchored READY ×1 = NA-0652 |
| 3 | qsl-server FRESH clone at pin | `git clone` + rev-parse | PASS — HEAD == origin/main == `8e4ea278` (unmoved; no STOP) |
| 4 | D-0012 next-and-absent (qsl-server) | grep DECISIONS.md | PASS — D-0011 ×1, D-0012 ×0 |
| 5 | Pre-lane route set exactly push/pull/ack; server-info unclaimed | grep src/ tests/ docs/ packaging/ | PASS — 3 routes; 0 hits for server-info + all three env names |
| 6 | Zero dependency change expected | Cargo.toml inspection | PASS — serde_json already direct; final diff has no Cargo.toml/lock |
| 7 | (a) open relay, unauthenticated → full doc | `open_relay_unauthenticated_gets_full_document` | PASS — 200, auth.mode "open" |
| 8 | (b) bearer, missing token → exact probe 401 | `bearer_missing_token_gets_exact_probe_401` | PASS — exact-value + key-count guards both nesting levels (2 top / 1 nested) |
| 9 | (c) bearer, WRONG token → identical probe | `bearer_wrong_token_gets_byte_identical_probe` | PASS — response bytes identical missing-vs-wrong (no oracle) |
| 10 | (d) bearer, valid token → full doc | `bearer_valid_token_gets_full_document` | PASS — 200, auth.mode "bearer" |
| 11 | (e) values track injected config | `document_values_track_injected_config` | PASS — limits 4096/9, ttl 3600, name/service_url/min_client_version verbatim; api + "none" modes pinned |
| 12 | (e′) unset fields ""-/null-safe | `document_optional_fields_are_empty_and_null_safe_when_unset` | PASS — "", null, null |
| 13 | (f) full-doc top-level exact field set | `full_document_top_level_field_set_is_exact` | PASS — exactly the 11 documented keys (additive baseline pinned) |
| 14 | Env plumbing END-TO-END (real binary) | `relay_env_vars_flow_to_document_end_to_end` | PASS — env_clear + RELAY_* set: probe + reflected doc (ttl 7200); bare: "open"/""/null/null |
| 15 | (g) existing tests byte-untouched; whole suite | `git diff --name-only` + `cargo test -q` | PASS — zero existing-test edits; **108/0 across 26 sets** = NA-0642 baseline (100/25) + this file |
| 16 | rust gate: fmt | `cargo fmt --all -- --check` | PASS (clean after fmt touched only the new test file) |
| 17 | rust gate: clippy | `cargo clippy --all-targets -- -D warnings` | PASS — exit 0 |
| 18 | rust gate: check | `cargo check --all-targets` | PASS — 0/0 |
| 19 | Purely additive diff; handlers untouched | `git diff --stat` + hunk inspection | PASS — +116/−0; 4 insertion hunks in lib.rs; no hunk in push/pull/ack/auth_ok |
| 20 | ENG-0039 TRAP held | diff inspection of auth_ok | PASS — reused AS-IS, compare untouched, debt stays filed |
| 21 | Spine scope guard | `git diff --name-only` (spine) | PASS — governance/queue/decision records only; DOC-PROG-004 byte-untouched |
| 22 | Main-health anomaly investigated | gh run logs at `07c38254` + `b3cfd5df` | FLAGGED — public-ci link check red SINCE NA-0651 merge (7 stale links, DENY/HC 0); pre-existing, out of scope, surfaced §7 of the as-built |

Non-vacuity notes: row 8's exact-value + key-count guards fail on ANY added or
missing probe field (the "no config leaked" claim is exercised, not asserted);
row 9 compares raw response bytes; row 13 pins the full-doc registry so any
future additive change must consciously edit the guard.
