# NA-0672 Test Plan — qsc server-info consumer + relay token trio (D608)

## A. Socket-free classifier tests (in-suite, `tests/NA_0672_relay_server_info.rs`, 12 tests)
`relay_server_info_from_parts(status, body, token_was_sent)` over captured JSON bodies — no listener, no network:
- Reachable{Open}, Reachable{Bearer} (200 + valid `auth.mode`); full-document deserialize; optional-null → None; unknown-field tolerance (top-level + nested); classification keys only on `auth.mode` despite bad sibling field types.
- Both 401 disambiguations: `token_was_sent` true/false → AuthRequired.
- Both NotAQslRelay cases: 200 without `auth.mode`; 401 without a QSL challenge body (None and a generic `{"error":...}`); other statuses; unrecognised `auth.mode`.
- The live deployed 401 probe body classifies AuthRequired (pins the pre-flight contract).
CertNotTrusted / Unreachable are NOT in-suite (zero-external-networking); they are proven at live acceptance (§C).

## B. Adjacent automated coverage
- `relay_auth_header` (4/4) — token redaction / no-secret-leak on the send path (ENV token).
- qsc lib unit (47/47) — includes the transport inline diagnostic tests.
- ⚠ The token trio (`relay_token_set/_show/_clear`) has **no automated regression test** — proven live only (§C.2). Filed **ENG-0071**.

## C. Live acceptance — vs the tserver rig over real TLS (`https://172.20.10.2:8443`)
Rig: qsl-server `5235c2bfe518` (current main, the C-2 fix), built on the build box and copied; plain-HTTP behind a user Caddy `tls internal` front; executor drives the auth-mode flip unprivileged. **Expected rig mode per check is stated.**

### C.1 The seven outcomes
| # | Outcome | Rig auth mode | qsc CA | qsc token | Expected |
|---|---|---|---|---|---|
| 1 | Reachable{Bearer} | **bearer** | ca-set | valid (env) | `reachable auth_mode=bearer` + full doc |
| 2 | Reachable{Open} | **open** | ca-set | none | `reachable auth_mode=open` + full doc |
| 3 | AuthRequired{true} | **bearer** | ca-set | wrong | `auth_required token_was_sent=true` |
| 4 | AuthRequired{false} | **bearer** | ca-set | none | `auth_required token_was_sent=false` |
| 5 | CertNotTrusted | bearer (any) | **no CA** | none | `cert_not_trusted` |
| 6 | Unreachable | n/a (`127.0.0.1:9`) | any | none | `unreachable` |
| 7 | NotAQslRelay | n/a (`example.com`) | system | none | `not_a_qsl_relay` |

### C.2 The CA pair (first-class before/after — `relay ca-set/_clear/_show` live)
Rig **bearer** throughout: `ca-show`=false → probe **cert_not_trusted** → `ca-set` (path_hash) → `ca-show`=true → probe **AuthRequired (trusted)** → `ca-clear` → `ca-show`=false → probe **cert_not_trusted** (the check fails once the CA is removed — the proof).

### C.3 The trio + vault-resolution (dummy token; no rig auth change; no real secret)
`token-show`(false) → `token-set` dummy → `token-show`(true) → probe env-unset → **AuthRequired{true}** (vault-resolved) → `token-clear` → `token-show`(false) → probe → **AuthRequired{false}**.

### C.4 Field-by-field 200-document check (both Reachable rows)
Compare the raw server JSON (curl `--cacert`) against the parser's marker fields AND the DOC-SRV-006 contract; **flag any field the parser renders empty and confirm the raw JSON's value** (serde(default): an omitted field is indistinguishable from an empty one). Result: all fields match; the two empties (`attachments.service_url`, `min_client_version`) are `null` in the raw JSON → parser correct.

## D. Result class
`QSC_SERVER_INFO_CONSUMER_PASS`. Evidence: `docs/governance/evidence/NA-0672_as_built.md` (provenance, the seven outcomes, the CA pair, the field-by-field method, the coverage finding + dummy-vault proof).
