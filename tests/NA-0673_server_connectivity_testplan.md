# NA-0673 Test Plan — GUI slice B: server connectivity (D609)

## Status
- **Automated coverage (§A): DONE** — merged and CI-green (qsl-desktop #7, #8).
- **Live GUI acceptance (§B): OWED — the operator flies it.** This section is the
  INSTRUMENT for that flight; it is not a record of a passed run. The build host
  cannot drive the GUI (xdotool absent). Until §B is flown and its results are
  recorded in `docs/governance/evidence/NA-0673_as_built.md`, the overall
  `GUI_SLICE_B_SERVER_CONNECTIVITY_PASS` is not asserted.

## §A — Automated coverage (in-suite, socket-free; qsl-desktop `cargo test`)
- `server_pane.rs` (4) — pane presence & controls; no-bypass guard (R8); results
  reuse the shipped status-banner with no invented colour classes (R7);
  claim-discipline regression (stale clauses gone, surviving clauses kept).
- `settings.rs::settings_key_allowlist` — the `relay_url` key added deliberately
  (third assertion case).
- `slice_a_rules.rs::desktop_builds_no_networking_client_of_its_own` — the refined
  R1 invariant: no `reqwest`/`hyper` in the desktop crate.
- design_round2 / design_round3 / design_system — unchanged, still green (the
  markup edits broke no frozen assertion).
- Total: **128 passed, 0 failed**. The probe's own taxonomy is unit-tested in qsc
  (NA-0672, 12 socket-free classifier tests); this lane RE-DERIVES none of it.

## §B — Live GUI acceptance — the operator flies it, against tserver over real TLS
**Rig:** tserver `https://172.20.10.2:8443` — qsl-server (current main, the C-2
fix) behind a user Caddy `tls internal`, unprivileged. NA-0672 proved every state
producible here. **CC/operator flips the rig auth mode (bearer↔open) — the
expected rig mode is STATED PER CHECK.** No mocks (§7.4): a mock passes a layout
check and hides a wrong-error-mapping bug — the whole reason this section exists.

Preconditions: the vault is unlocked (Settings is unlock-gated); the app is at
Settings › Server.

| # | Check | Rig auth mode | Rig / field condition | Expected GUI result |
|---|---|---|---|---|
| 1 | Reachable{Bearer} | **bearer** | CA set (`ca-set`), valid token set, URL = tserver | banner **Connected** (neutral); "Token required — accepted. Certificate trusted."; doc rows (relay name, Certificate=Trusted, Access, retention, max size, version) show the REAL doc values |
| 2 | Reachable{Open} | **open** (flip) | CA set, token cleared | banner **Connected**; "Open relay — anyone who can reach this address can use it…" |
| 3 | Token rejected | **bearer** | CA set, a WRONG token set | banner **This app's token was not accepted** → "Token rejected" (accent); phrased as a LOCAL observation ("the one this app sent") |
| 4 | Token required | **bearer** | CA set, token cleared | "This relay requires an access token" (accent); "this app sent no token" — a local observation, not a server verdict |
| 5 | Certificate not trusted | either | CA CLEARED (bare self-signed), URL = tserver | "Certificate not trusted" (accent); the interception-attack copy; remedy = add the CA |
| 6 | Unreachable | n/a | URL = a dead address (e.g. `https://127.0.0.1:9`) | "Couldn't reach the server" (accent) |
| 7 | Not a QSL relay | n/a | URL = a non-relay HTTPS endpoint that answers (e.g. `https://example.com`) | "Not a QSL relay" (accent) — NOT "requires a token" (the FLAG-2 boundary) |
| 8 | Save-state | bearer | after check 1, EDIT the URL to a new valid tserver form and Test again | "Not saved yet." + Save takes the accent (primary) treatment; press **Save** → note clears, Save returns to secondary; then EDIT any field → the results panel CLEARS (state 10) |
| 9 | ⚠ CA unreadable ≠ CertNotTrusted (R2b) | bearer | set the CA path to a non-existent / non-cert file, Test | "Certificate authority file couldn't be read" (accent) — **CONFIRM it is NOT rendered as "Certificate not trusted"**; remedy points at the file path |
| 10 | Bad address (R2a) | n/a | URL = a malformed address (e.g. `not a url`), Test or Save | INLINE validation under the address field; **no results card appears** |
| 11 | Claim-discipline surfaces | either | open Settings › About; the main-window footer; the welcome "Add contact" stub | About reads "Slice B (server connectivity…). This build makes no security-assurance claims."; footer reflects the configured relay (or "No server configured — add one in Settings › Server."); the stub reads "Adding contacts arrives in a future update." — NO "serverless / no network connections" claim survives |
| 12 | No-bypass (R8) | any failing state (5 or 3) | inspect every failing results state | there is NO "connect anyway" / "trust this certificate" control anywhere; the only remedy offered for an untrusted cert is the CA file |

**Recording:** on completion, append the outcome of each check to
`docs/governance/evidence/NA-0673_as_built.md §4` (pass/observed-difference), then
the overall `GUI_SLICE_B_SERVER_CONNECTIVITY_PASS` may be asserted.
