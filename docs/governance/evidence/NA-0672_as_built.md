# NA-0672 As-Built — qsc server-info consumer + relay token trio (D608; D-1300 impl, D-1301 closeout)

Goals: G4

## 0. Identity
- Lane NA-0672; directive **QSL-DIR-2026-07-23-608 (D608)**, sha256 `52d8499f296149ce7689649323659ac881a067cd0ab26507c4a8412cf84677d5`, 168 lines, all five flags at drafted defaults.
- Spine decisions **D-1300** (implementation, rode PR #1640, merge `37b70ca8`) and **D-1301** (this closeout).
- DOC-PROG-004 step 5. Implementation landed the §2 surface; this closeout carries acceptance + governance.

## 1. Implementation (D-1300, merged PR #1640)
`transport::relay_server_info(base) -> RelayServerInfoOutcome` via the pure socket-free `relay_server_info_from_parts` classifier (five variants → the seven observable network states); the pub token trio `relay_token_set/_clear/_show`; CLI verbs `relay server-info` / `token-clear` / `token-show`; `RelayCmd::TokenSet` refactored through `relay_token_set`. `ServerInfoDoc` parses the full DOC-SRV-006 nested contract (serde-tolerant). 12 socket-free classifier tests. The three send/pull/ack `Authorization` attach points byte-unchanged (ENG-0051); zero Cargo delta. **Its merge ran `qsc-linux-full-suite` + the macOS serial (not a docs_only skip); 34/34 PR checks green.** Two design points, both ratified by the operator:
- **`require_unlocked` on the CLI verb — RATIFIED, with the trade-off recorded:** the self-hoster diagnostic (OBS-G) needs an unlocked vault; the library fn `relay_server_info` stays unlock-free for GUI/env-token callers. A probe that silently skipped a vault-stored token would make the `AuthRequired` disambiguation **lie** (report token-required-but-missing when a token exists but could not be read) — the wrong-error class this lane exists to prevent.
- **`relay_server_info_from_parts` is `pub` — a deliberate §2 deviation, not an oversight:** the directive's named integration test drives it socket-free and an integration test reaches only `pub` items; the pure taxonomy is also reusable by a future GUI fetching on its own runtime.

## 2. ⚠ COVERAGE FINDING — stated plainly, NOT as a caveat
**NONE of the seven network-outcome acceptance checks exercise the VAULT-stored token.** Both token-bearing outcomes (Reachable{Bearer}, AuthRequired{true}) supply the token via `QSC_RELAY_TOKEN` (the environment). `relay_auth_token` resolves **env → vault → file**, so setting `QSC_RELAY_TOKEN` for convenience **silently shadows the vault path slice B will actually use** (the GUI saves the token to the vault, not an env var). That precedence is exactly why the gap was invisible — a testing-methodology trap ("the instrument pointed slightly off the question").

Before the dedicated dummy-vault sequence below, **`relay_token_set` (the wrapper), `relay_token_show`, and `relay_token_clear` had ZERO coverage — live or automated.** The vault token-READ resolution was unit-covered only by NA_0671 (via a direct library `secret_set`, in-process — not the CLI/wrapper). **Half the lane (the token trio) was compile-checked only.** "7/7 outcomes" therefore does NOT mean "the trio was proven"; the trio is proven by the **separate** dummy-vault sequence, and the residue (no automated regression test) is filed as **ENG-0071**.

### 2a. Dummy-vault proof (build box; no real secret; proves the trio + vault resolution live)
`QSC_RELAY_TOKEN` unset throughout; token resolved from the VAULT:
```
token-show                         -> configured=false
token-set  <dummy>                 -> relay_token=set
token-show                         -> configured=true
server-info (env unset, dummy)     -> auth_required token_was_sent=true   [vault token resolved, attached, sent, rejected]
token-clear                        -> relay_token=cleared
token-show                         -> configured=false
server-info (env unset, cleared)   -> auth_required token_was_sent=false  [empty vault secret = absent]
```
This proves `relay_token_set` / `_show` / `_clear` + `relay_auth_token_from_account_secret` (vault → attach → real relay) live. A valid-vault-token→Reachable{200} was **deliberately NOT captured**: it is the composition of two already-proven paths (this vault resolution ∘ the env-valid-token→200 in §4) — no untraversed code path — and it would argv-expose the real token. The precedence trap and this proof are the sharpest findings of the lane.

## 3. Deployment provenance (the thing inspiron LACKED)
inspiron was lost mid-acceptance; acceptance was relocated to a LAN laptop (`qscwork@notebook`, LAN `172.20.10.2`), driven entirely by the executor unprivileged — no sudo, no operator hand-run. Provenance recorded so "the deployed relay cannot silently diverge from what was built" is a **verifiable** claim (one build environment produces both binaries):
- **qsl-server main `5235c2bfe518ed06dfd94f2df9be2ea6366f8835`** — this IS the NA-0670 **C-2 constant-time-bearer** merge (inspiron ran the pre-fix `b4f86a3c`). **Building current main fixes OBS-J half (a) by construction.**
- Built on the build box (Ubuntu 24.04, glibc 2.39), `cargo build --release --locked`, 52.18s.
- Binary **sha256 `60d703ef0305849ce9c632412fc3c0754ad56a53c6b010b3e1897a603f8ee8c8`**, 5,905,704 bytes.
- Copied to tserver (Ubuntu 26.04, glibc 2.43); **sha byte-exact after scp**; `ldd` clean (`linux-vdso`, `libgcc_s`, `libm`, `libc`, `ld-linux` — no OpenSSL, no libsqlite → rustls + statically-bundled SQLite, self-contained); **`qsl-server --version` → `qsl-server 0.1.0`, rc=0.** glibc forward-compat (2.39 build → 2.43 host) confirmed on its own before anything was stacked on it.

## 4. TLS rig — self-signed, unprivileged; PATH (b) is the headline
### ⚠ FINDING (for the qsl-server README deployment section — docs lane, not fixed here)
**qsl-server is PLAIN-HTTP ONLY — verified, not inferred:** `TcpListener::bind` + `axum::serve`, no native TLS server (the `reqwest rustls-tls` is its outbound client). **Every deployment needs a TLS-terminating front.**

TLS front = **user Caddy** in `~/bin`, unprivileged, high port; provenance recorded (a downloaded binary in the test path is part of the deployment story OBS-J is about):
- version **v2.11.4**; url `https://github.com/caddyserver/caddy/releases/download/v2.11.4/caddy_2.11.4_linux_amd64.tar.gz`; tarball **sha512 `8220d1f0…f5ebf1c9` — VERIFIED == published**; tarball sha256 `527fbf91…`; installed binary sha256 `b7105518e3ed1c0761f232e44fc09345535533c9cb0abf0e12809416c7ac64d9`.
- Caddyfile: `auto_https disable_redirects` (drops the privileged `:80` redirect listener); `https://172.20.10.2:8443 { tls internal; reverse_proxy 127.0.0.1:8080 }`.
- Caddy internal CA root: `~/.local/share/caddy/pki/authorities/local/root.crt`, "Caddy Local Authority - 2026 ECC Root", sha256 `9b63605f45927588e3c86f0f855bde704a80e0aa07def547bafd2d7ea9f5aa58`; leaf **SAN = IP:172.20.10.2** (matches the qsc URL).
- **Path (b) [`relay ca-set`] used; path (c) [OS trust store] skipped per operator ruling; path (a) [no CA] exercised as CertNotTrusted.** No sudo anywhere.

## 5. ⚠ THE CA PAIR — first live exercise of `relay_ca_file_set/_clear/_show` since NA-0663
A check never observed rejecting anything is the artifact this project keeps filing; this pair shows the CA file doing the work by watching the probe FAIL once it is removed:
```
ca-show                          -> configured=false
server-info https://172.20.10.2:8443 (no CA)   -> cert_not_trusted        [BEFORE]
ca-set  <caddy root.crt>         -> relay_ca_file=set   path_hash=acefd80d
ca-show                          -> configured=true     path_hash=acefd80d
server-info https://172.20.10.2:8443 (CA set)  -> auth_required (trusted; reaches the relay)   [SUCCEEDS]
ca-clear                         -> relay_ca_file=cleared
ca-show                          -> configured=false
server-info https://172.20.10.2:8443 (no CA)   -> cert_not_trusted        [AFTER — the proof]
```

## 6. THE SEVEN OUTCOMES over real TLS (qsc → `https://172.20.10.2:8443`, vault-unlocked CLI)
| # | Outcome | Rig / qsc condition | Result (marker `outcome=` / human) |
|---|---|---|---|
| 1 | Reachable{Bearer} | bearer + CA set + valid token (env) | `reachable auth_mode=bearer` + full doc |
| 2 | Reachable{Open} | rig flipped to open + CA set, no token | `reachable auth_mode=open` + full doc |
| 3 | AuthRequired{token_was_sent:true} | bearer + CA set + WRONG token | `auth_required token_was_sent=true` |
| 4 | AuthRequired{token_was_sent:false} | bearer + CA set + no token | `auth_required token_was_sent=false` |
| 5 | CertNotTrusted | no CA configured | `cert_not_trusted` |
| 6 | Unreachable | `https://127.0.0.1:9` | `unreachable` |
| 7 | NotAQslRelay | `https://example.com` | `not_a_qsl_relay` |

The auth-mode flip (bearer↔open) was performed by the executor unprivileged (`RELAY_TOKEN` toggle + restart), restored to bearer, loopback 401 re-confirmed — the inspiron privilege blocker is gone.

## 7. Field-by-field 200-document check (the serde(default) silent-miss method, repeated vs current main)
Reachable{Bearer} raw authed 200 and Reachable{Open} raw 200 both:
```
{"api":["push_v1","pull_v1","pull_ack_lease_v1"],"attachments":{"service_url":null},
 "auth":{"mode":"bearer|open"},"directory":{"mode":"none"},"kt":{"mode":"none"},
 "limits":{"max_body_bytes":1048576,"max_queue_depth":257},"min_client_version":null,
 "name":"tserver-lan-relay","retention":{"ttl_secs":604800},"server":"qsl-server","version":"0.1.0"}
```
Parser vs raw JSON, every field: **match.** The two values the parser renders empty — `attachments.service_url` and `min_client_version` — are explicitly **`null` in the raw JSON**, so the server genuinely sends null (no attachments service, no minimum client version); the parser is correct. **That is the exact serde(default) ambiguity the check exists to eliminate (an omitted field is indistinguishable from an empty one), and it resolved cleanly against current main — the same method the superseded inspiron capture established.** `server:"qsl-server"` is present in the payload but NOT in the marker: deliberate under FLAG-2 (classification keys on `auth.mode`, not the server string, so AGPL forks work) — though the value is a useful self-hoster diagnostic. tserver's 200 doc is field-identical to inspiron's except `name` (config) and `auth.mode`; `version` is still `0.1.0` (C-2 did not bump it) and `max_queue_depth:257` is the **default**, not an inspiron customization.

## 8. Marker cosmetic artifacts (recorded, not filed)
`should_redact_value` masks two marker values that are not secret: `token_was_sent` (key contains "token", output/mod.rs:269) and `api` (value ≥24 chars with a digit → `looks_high_cardinality` false-positive, :277/:294). The human line, the raw document, and the library variants all carry the real values, so nothing is lost where it matters. Candidate future micro-nit; not filed.

## 9. ⚠ CLI micro-nit (record, do NOT fix — documentation/ergonomics)
`relay token-set --token <value>` has no `--token-file` / stdin variant (unlike `token-file-set --path`), so setting the vault token from the CLI transits the token via argv (`/proc/PID/cmdline`, world-readable). It will bite the first self-hoster who follows the CLI docs with a real token. Not a defect; a docs/ergonomics gap.

## 10. inspiron Phase-1 capture — HISTORICAL / SUPERSEDED (do not mix rigs)
Captured against inspiron (qsl-server `b4f86a3c`, front `server: Caddy`) before the host was lost mid-acceptance; superseded by the tserver run against current main. Preserved because it is real and it is what caught the 200-document gap. Reachable{Bearer} raw 200: `name=inspiron-lan-relay, version=0.1.0, api=[push_v1,pull_v1,pull_ack_lease_v1], limits={1048576,257}, retention.ttl_secs=604800, directory/kt=none, attachments.service_url=null, min_client_version=null`. The field-by-field method resolved the serde(default) risk there first (the two empties are null in the raw). tserver runs newer code, so a legitimate difference would be attributable — but the two docs are field-identical (bar name/auth.mode), and the pre-Caddy loopback baseline on tserver proved the relay's own contract independent of any proxy.

## 11. Filings (this closeout)
- **ENG-0070** — OBS-J: the deployed relay lags main and nothing tracks the gap. Both halves — (a) rebuild/restart at/after `5235c2bf` (fixed by construction here via the fresh build), (b) a mechanism that makes deployment drift visible.
- **ENG-0071** — the token trio has LIVE proof (§2a) but no AUTOMATED regression test; a socket-free micro-test is cheap.
- **WF-0043** — the directive template scopes a two-PR impl PR to src+test, but goal-lint (a required check) fails any core `src/` change omitting DECISIONS.md+TRACEABILITY.md, so every src-touching impl PR must carry them (NA-0663 #1609 precedent). ENG-0068's shape; rule: when a directive template and an enforced CI gate disagree, the gate governs. Cross-linked to ENG-0068.

## 12. Not claimed / scope
No public/production/crypto-complete/constant-time-measured/bug-free/vulnerability-free claim. The C-2 constant-time property of the freshly-built relay is inherited from NA-0670's structural change, not re-measured here. Result class **QSC_SERVER_INFO_CONSUMER_PASS**. Rig state at closeout: tserver relay up (bearer) + Caddy up, unprivileged, the operator's to keep or stop; qsc CA config left cleared; build-box acceptance artifacts (throwaway vault, scripts, clone, CA root) removed at closeout (token file was shredded by the operator when inspiron was live).
