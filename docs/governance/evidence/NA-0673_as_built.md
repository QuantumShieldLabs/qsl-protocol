# NA-0673 as-built — GUI slice B: server connectivity (D609)

Cross-repo lane, qsl-desktop PRIMARY + this spine governance closeout. Executed
per **QSL-DIR-2026-07-24-609 (D609, APPROVED 2026-07-24, sha256
`eb6f9da01fc3f338df20f52baa8a4ae3569643ec4d795e26c8cca62a705473b9`, 678 lines)**,
all eight observations ruled and folded as binding R1–R8. Three gates.

> ⚠ **THE LIVE GUI ACCEPTANCE FLIGHT HAS NOT BEEN FLOWN.** It is the operator's
> (the build host cannot drive the GUI — xdotool is absent), and it had not run
> at the time this closeout was opened. This as-built records what was BUILT and
> PROVED IN CI; it does NOT claim the acceptance passed. The pending claims are
> enumerated in §4. The testplan `tests/NA-0673_server_connectivity_testplan.md`
> is the INSTRUMENT for that flight.

## §0 — What shipped, and where the evidence is

| Gate | PR | Merge | Result class | Evidence |
|---|---|---|---|---|
| GATE 1 — qsc pin bump ALONE | qsl-desktop #7 (D-0007) | `c6536aa` | `GUI_SLICE_B_PIN_BUMP_PASS` | `[rust]` CI green (5m09s); the delta enumerated (§1) |
| GATE 2 — the Server pane | qsl-desktop #8 (D-0008) | `5239d96e` | `GUI_SLICE_B_SERVER_PANE_PASS` | `[rust]` CI green (4m29s); `cargo test` 128/0 (§2) |
| GATE 2 addendum — Appendix F reasons | qsl-desktop #9 (D-0009) | *(open at closeout)* | docs-only | the ratified OBS-1/OBS-2 reasons |
| GATE 3 — this spine closeout | qsl-protocol *(this PR)* (D-1302 + D-1303) | *(this PR)* | governance | this file + the testplan |

The overall `GUI_SLICE_B_SERVER_CONNECTIVITY_PASS` is **PENDING the operator
acceptance flight** (§4). GATE 1 and GATE 2 are proved and merged.

## §1 — GATE 1: the pin bump (D-0007), the delta ENUMERATED

`src-tauri/Cargo.toml` qsc `rev` `81143dcd` → `ab5041cd` (the NA-0672 server-info
consumer). `Cargo.lock` **513 → 518 crates, five added, zero removed** — the
D599-sanctioned native-roots union: `rustls-native-certs` 0.8.4, `openssl-probe`
0.2.1 (Linux), `security-framework` 3.7.0 + `-sys` 2.17.0 (macOS), `schannel`
0.1.29 (Windows). `qsc` + `quantumshield_refimpl` rev lines → `ab5041cd`; the 32
other resolved deps and all 12 RustCrypto pins UNCHANGED (no cargo-1.95 resolver
drift; verified against a before/after lock diff). **rustls stays on the ring
backend (`default-features=false`); `aws-lc-rs` is ABSENT** — the precise failure
GATE 1 exists to catch. Hand-applied via `cargo update -p qsc` (the minimal
update was clean, so no drifted-pin hand-correction was needed).

## §2 — GATE 2: the Server pane (D-0008), automated coverage

`#pane-server` → the full pane (relay-address + access-token + CA disclosure +
Test/Save + results panel), in the existing fixed rail, no hamburger. Backend:
9 thin `relay_*` Tauri commands forwarding onto the qsc surface, **every qsc call
inside `st.gw.call(...)`** — no HTTP client on FE/BE, no `relay_server_info_from_parts`
(R1). `settings.rs` `relay_url` (the `self_alias` pattern; allowlist test → 3rd
case; `deny_unknown_fields` downgrade property KNOWINGLY untouched, R6).

- `cargo test -q --locked` — **128 passed, 0 failed** (1 ignored). Includes the
  design_round2/round3/system suites (unchanged), the new `server_pane.rs` guards
  (pane presence, no-bypass R8, status-banner reuse / no-invented-classes R7,
  claim-discipline regression), the `settings_key_allowlist` third case, and the
  refined slice-A R1 test.
- `cargo clippy -q --locked -- -D warnings` (the CI command) — clean.
  (`RelayTestDto`'s large variant was boxed for `large_enum_variant`.)
- `cargo fmt --all -- --check` — clean. `cargo metadata --locked` — OK (no dep
  motion since GATE 1). `git diff --check` — clean.
- STOP-class `gateway.rs`, `design_round2.rs`, `design_system.rs` — BYTE-UNCHANGED
  (`git diff --quiet` verified). `ui/style.css` — untouched.

### §2.1 — Ratified design calls (operator, 2026-07-24; reasons in Appendix F, D-0009)
- **R7 — the results panel uses NO red.** DESIGN_SPEC §2 reserves `status-danger`
  (red) for irreversible vault-loss (armed erasure, autolock-0, destroy ceremony).
  A connection failure is an inconvenience, not a danger, so failures render
  `status-accent`; Connected renders `status-neutral`; severity is carried by the
  message. The mockup's red "bad" / amber "warn" coding is deliberately not
  copied — that would be reading a mockup colour (the R7 STOP).
- **"Save persists ONLY the URL" → token & CA commit via their own Set/Clear
  controls** (the vault trios), because the probe reads them from the vault; this
  is a ruling-refinement (the only shape consistent with "URL to settings,
  secrets to the vault"), not a deviation.

### §2.2 — Three NECESSARY scope refinements (all recorded, all accepted)
- **lib.rs** was scoped "About comment ONLY", but the 9 new Tauri commands MUST be
  registered in `generate_handler` (also lib.rs) — unregistered commands cannot be
  invoked. Unavoidable, not discretionary.
- **ui/style.css** was not in the MAY-touch list; the pane's structural needs (the
  470px form cap, the results layout) were met with inline styles in index.html
  (shipped tokens only, no mockup hex) to stay within scope.
- **the slice-A `zero_networking_in_src_and_ui` test** asserted an invariant slice
  B is DEFINED to break. It was **REFINED, not deleted**, to the surviving R1
  invariant: the desktop crate builds no `reqwest`/`hyper` client of its own — all
  networking goes through qsc. **RULE (reusable): a test whose premise a lane
  intentionally invalidates is REFINED to its surviving invariant, never deleted —
  the opposite of weakening an assertion.**

## §3 — Claim-discipline sweep (R4) — five surfaces, two compound kept surgical
About in-app (`main.js` + the `commands.rs` slice string), About native menu
(`lib.rs`), footer (`index.html` + `main.js`), welcome stub (`index.html`). The
two compound surfaces kept their surviving true clause — **"no security-assurance
claims"** and **"Adding contacts arrives in a future update"** — only the network
clause changed. A regression test asserts the stale clauses are gone AND the
surviving clauses remain.

## §4 — ⚠ PENDING: the live acceptance flight (OWED, operator-flown)
The following are NOT yet proved and are the operator's to fly, per the testplan,
against **tserver over real TLS** (NA-0672 proved every state producible there):
1. The 8 results-panel states rendered correctly against the real relay (the 7
   probe outcomes + the "Not saved yet" save-state), each at its expected rig
   auth mode.
2. The R2(b) distinction LIVE: an unreadable configured CA renders as its own
   line, NOT as "Certificate not trusted".
3. The two-message 401 LIVE (token-rejected vs token-required), phrased as local
   observations.
4. The claim-discipline surfaces read correctly in the running app.
5. No connect-anyway control on any state (R8).

Until the flight is flown and its results appended here, the overall
`GUI_SLICE_B_SERVER_CONNECTIVITY_PASS` is **not asserted**. GATE 1 and GATE 2 are
independently proved (CI-green, merged); this closeout does not borrow the pane's
correctness from a mock (§7.4 discipline) — it records exactly what CI proved and
names the rest as owed.

## §5 — Filings & notes
- **ENG-0072** (filed here) — the qsl-desktop qwork seat does not set the GH007
  identity; it recurred on BOTH the GATE-1 and GATE-2 seats (handed back
  `tebbens@proton.me`), caught only because the executor checked. Not a one-off.
- **OBS-5 (observed, not fixed)** — `cargo clippy --all-targets` flags a
  pre-existing `field_reassign_with_default` in the settings test module; CI runs
  clippy without `--all-targets`, so it is not gated. Belongs with the fmt/clippy
  cleanup lane (roadmap 6a), not here.
- **R6 record** — the `settings.rs` `deny_unknown_fields` downgrade property was
  KNOWINGLY left untouched: a slice-B file carrying `relay_url` fails to parse on a
  slice-A reader and falls back to the default; a pre-existing class (`self_alias`
  already carries it), and downgrades are not a supported path.
