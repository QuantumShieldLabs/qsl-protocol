QSL DESKTOP SECURITY AUDIT -- 2026-09-03
=========================================

Auditor: Director chair (Claude), static read. No build, no run, n=0 executions.
Audience: the Director and the CC seats that will lane the fixes.
Pure ASCII by house rule. Every figure names its instrument (WF-0087).

BASE
----
  qsl-desktop-main.zip     sha256 aef9f742512558a83006ef8a302cc9ff39fb217575f03d88f8ebc87d408e6b0d
                           125 files; product source = src-tauri/src (9 .rs files, 2,609 lines
                           by `wc -l`), ui/main.js 3,715 lines, ui/index.html 742 lines.
  qsc (pinned dependency)  git rev 63ece4fe25eeaa39b03c3eb1f234d50bca31d0f8 (Cargo.toml [dependencies])
                           fetched as codeload tarball, sha256
                           f769dc99a1fa8f355bbaba08f8fdd7dea69c5749b5662ea7b3d9514104d9ac14
                           qsl/qsl-client/qsc/src = 32,335 lines (`wc -l` over *.rs).
  Cargo.lock               518 packages (regex parse of [[package]] blocks).

Line numbers are for these exact bytes. Re-derive at the edit.

SEVERITY VOCABULARY
-------------------
  High / Medium / Low / Info, plus OWED for a measurement not yet taken. Mapping onto the
  ledger's P-levels is the Director's act at filing; none of these is proposed as P1.

BOTTOM LINE
-----------
  No Critical or remotely-exploitable finding. The product code is disciplined: tight CSP,
  no dynamic HTML, atomic 0600 writes, symlink-safe deletes, no TLS bypass anywhere.
  The exposure concentrates in three places:
    (1) the process ENVIRONMENT is used as a control channel -- mutated after threads exist
        (memory-safety hazard) and inherited unscrubbed (ambient authority);
    (2) qsc holds the PASSPHRASE itself in memory for the whole session and re-runs Argon2
        on every vault read; nothing at the desktop boundary is zeroized;
    (3) several unsafe TEST SEAMS compile into the release binary and are honored from the
        launching environment.


TRIAGE TABLE
------------
  #   Finding                                                              Sev      Owner
  1   set_var on a live multithreaded process                              Medium   desktop+spine
  2   Unscrubbed environment: env token/CA override the vault silently;    Medium   desktop (mitigation)
      unsafe seed + clock + env-passphrase seams compiled into release              spine (root)
  3   Passphrase (not derived key) resident for the session; Argon2 per   Medium   spine (root)
      vault read; no zeroize on IPC args or VaultRuntime/VaultPayload               desktop (args)
  4   Argon2id at the OWASP floor (m=19 MiB, t=2, p=1)                     Low-Med  spine
  5   Autolock enforced only in webview JS; no Rust-side lock gate         Low-Med  desktop
  6   Passphrase floor (>=12) enforced UI-only                             Low      desktop
  7   Invite label unvalidated on create; becomes contact alias on accept  Low      desktop
  8   Wrong passphrase persists in the unlock field after rejection        Low      desktop
  9   Corrupt settings.json silently becomes defaults                      Low      desktop
 10   CI actions pinned to mutable tags; no permissions: block             Low      operator (.github)
 11   ml-dsa 0.1.0-rc.7 pre-release signs identities; duplicate majors     Low/Info spine
 12   Invite code written to the system clipboard                          Info     ruled design
 13   withGlobalTauri:true + home_dir expose the whole command surface     Info     desktop
 14   "Certificate: Trusted" rendered on a loopback http:// relay          Low      desktop
 15   Invite-code decoder has no length cap before base64 decode           Low      spine (+desktop)
 16   Protection-state files plaintext and unauthenticated (0600-checked)  Info     spine


FINDINGS
--------

1. std::env::set_var on a live process                                          [Medium]
   Where: src-tauri/src/commands.rs:840-858 (relay_probe), :876-907 (EnvGuard);
          src-tauri/src/lib.rs:322 (mark_webview_wipe_pending, env fallback) reached from
          commands.rs:315, :439, :500.
   What:  relay_probe sets QSC_RELAY_TOKEN and QSC_RELAY_CA_FILE on a spawn_blocking thread
          while GTK, WebKitGTK and tokio threads are running. Rust std documents set_var as
          unsound in multithreaded programs (glibc setenv can reallocate `environ` under a
          concurrent getenv); Rust 2024 marks it unsafe. The comment at :826-833 scopes the
          claim to "nothing in this tree reads these two variables" -- an SR-21 mismatch:
          the hazard is ANY thread calling getenv for ANY name, and GLib/GTK/WebKit/
          rustls-native-certs do so routinely (G_MESSAGES_DEBUG, SSL_CERT_FILE, HTTP_PROXY,
          LANG). The wipe-marker fallback has the same shape when XDG_RUNTIME_DIR is unset.
   Impact: memory-unsafe race -> crash class. Also places the bearer token on the heap via
          `environ` (it is already in memory via the vault, so the delta is small).
   Fix (spine): expose relay_server_info_with(address, token: Option<&str>,
          ca_path: Option<&Path>) -- explicit parameters, no env.
   Fix (desktop): delete EnvGuard; make the wipe marker a file under data_dir
          (e.g. <data_dir>/.webview-wipe-pending) and drop the env fallback. Standing rule:
          after bootstrap() returns, the desktop never calls set_var/remove_var. Gate it with
          a `git grep -n 'env::set_var\|env::remove_var' src-tauri/src` allowlist test.

2. Ambient authority inherited from the launching environment                  [Medium, as a class]
   Instrument: `grep -rnE 'env::(var|var_os)'` over qsc/src plus the `*_ENV` constants
   (the literal-only grep under-counts by 8 names; both were run).
   The desktop sets QSC_CONFIG_DIR and QSC_MARK_FORMAT (lib.rs:448, :457) but never scrubs
   what it inherited. Measured in qsc at the pinned rev:
   (a) Token/CA override, invisible in the UI. relay_auth_token() consults QSC_RELAY_TOKEN ->
       RELAY_TOKEN -> vault -> token FILE named in the vault (transport/mod.rs:2059-2072).
       CA: QSC_RELAY_CA_FILE -> RELAY_CA_FILE -> vault (:2136-2146). But relay_token_show()
       reports ONLY the vault (:2620-2623), and the desktop renders that (commands.rs:742-749).
       So Settings can read "no token configured" while an env token is sent, and a trust
       anchor can be injected by env with relay_ca_file_show reading configured=false.
       The operator's own relay.env exports RELAY_TOKEN: launching the GUI from that shell
       silently authenticates with it.
   (b) Deterministic session keys from a u64 seed, in RELEASE builds.
       allow_unsafe_seed_fallback_for_tests() (protocol_state/mod.rs:1017-1019) has NO cfg
       gate (measured: grep for `cfg(` in the six lines above each use returns nothing;
       contrast the RNG-failure seam, which IS gated by cfg(qsc_rng_failure_test_seam) at
       identity/mod.rs:50, vault/mod.rs:51, protocol_state/mod.rs:33). With
       QSC_ALLOW_SEED_FALLBACK=1 QSC_UNSAFE_TEST_SEED_FALLBACK=1 QSC_QSP_SEED=<n>, a missing
       session (:1040-1075) or a locked vault (:143-155) yields hk/ck/rk/dh material derived
       from sha512(seed_le). Latent today (no GUI messaging); linked into the binary now.
   (c) Clock override QSC_UNSAFE_TEST_CLOCK_UNIX_S (clock/mod.rs:49, :59). The desktop
       deliberately withholds invite_list_at so the front end cannot choose the expiry clock
       (lib.rs:527-528); the env route grants exactly that.
   (d) QSC_DESKTOP_SESSION_PASSPHRASE (vault/mod.rs:49, :1408-1424): env passphrase ingress
       exists in the library; its only caller unlock_with_passphrase_env (:183-190) is not
       reached by the desktop. Latent.
   (e) Desktop's own QSLD_DATA_DIR, QSLD_TICK_MS, QSLD_INJECT_MARKER: the ENG-0127 class,
       already filed.
   Threat: an attacker controlling the session environment (~/.config/environment.d, a
       .desktop launcher, ~/.profile) usually has easier options; the realistic exposure is
       ACCIDENTAL (the build box where the app is flight-tested carries these variables in
       scripts) plus low-noise persistence.
   Fix (desktop, one small lane): in bootstrap() (lib.rs:439), before any thread exists,
       enumerate std::env::vars_os() and remove_var every QSC_*, RELAY_TOKEN, RELAY_CA_FILE;
       then set the two the app owns. Keep QSLD_* but behind a cargo feature (test-seams)
       that release builds do not enable. Can-fail test: launch with QSC_RELAY_TOKEN=x and
       assert the probe sent no token.
   Fix (spine): gate the seed fallback, clock override and env-passphrase ingress behind a
       feature/cfg as the RNG seam already is; make relay_token_show() report the EFFECTIVE
       source.

3. Passphrase resident for the session; no zeroization at the desktop boundary [Medium]
   Where (qsc): vault/mod.rs:869 (PROCESS_PASSPHRASE), :1386-1394 (set_process_passphrase),
       :203-212 (unlock_with_passphrase), :876-929 (load_vault_runtime_with_passphrase ->
       derive_runtime_key), :226-233 (secret_get), :838-841 (VaultRuntime), :86-90
       (VaultPayload). Zeroize census: `grep -rc '\.zeroize()'` = 27 sites in 4 files;
       `grep -n 'impl Drop\|ZeroizeOnDrop'` in vault/ = VaultSession, EnvSnapshot,
       DestroyConfirmToken only.
   Where (desktop): commands.rs:186-189, :244-247, :388-391, :727 take passphrase/confirm/
       token as String and drop them un-zeroized; Cargo.toml declares no zeroize/secrecy
       (zeroize 1.9.0 is in the lock only via qsc).
   What: a successful unlock stores the PASSPHRASE STRING in a process static. Every vault
       access re-reads vault.qsv, clones the passphrase and RE-RUNS Argon2id (the
       PERF_KDF_CALLS counter confirms the design). VaultRuntime.key ([u8;32]) has no
       zeroize-on-drop; VaultPayload derives Debug + Clone and holds every secret as String.
   Impact: the resident secret is the highest-value one (humans reuse passphrases); ptrace by
       the same user, core dumps and swap expose it. Derived key and decrypted secrets linger
       in freed heap after each of many calls. Any `{:?}` of a VaultPayload prints all
       secrets. Each liveness tick (main.js:2986-3020: relay_config_get -> connect_status ->
       invite_list -> invite_finish) pays several 19 MiB Argon2 runs, and this is why the
       KDF params must stay low (Finding 4).
   Fix (spine): derive once at unlock; keep the key in a Zeroizing<[u8;32]>; zeroize on
       lock(); #[derive(ZeroizeOnDrop)] on VaultRuntime/VaultPayload; remove Debug from
       VaultPayload. SR-15 triggers (crypto region).
   Fix (desktop): wrap secret IPC arguments in a SecretString newtype (Zeroizing<String>);
       at bootstrap call prctl(PR_SET_DUMPABLE, 0) and setrlimit(RLIMIT_CORE, 0). Boundary:
       this covers the Rust process only; the WebKit web process still holds what was typed.

4. Argon2id parameters                                                          [Low-Medium, spine]
   Where: vault/mod.rs:45-47 (KDF_M_KIB=19456, KDF_T=2, KDF_P=1); :893 rejects any other
       stored profile.
   What: the OWASP MINIMUM profile. For a desktop vault whose threat is offline cracking of a
       stolen vault.qsv, m=64-256 MiB, t=3 is affordable once Finding 3 stops re-deriving
       per call. The parser pin is correct against downgrade but means a raise is a
       format-version lane (QSCV02 -> 03 with migration). The online guard (unlock_guarded,
       delays, wipe-after-N) does not bound an attacker holding the file; only passphrase
       entropy and these params do -- worth one plain sentence in user-facing copy.

5. Autolock lives only in the webview                                           [Low-Medium]
   Where: ui/main.js:1928-1951; commands.rs (settings_set, relay_config_set, wipe_arm,
       wipe_disarm, erase_all, restart_app, notice_dismiss carry no lock-state check).
   What: a 5 s setInterval over in-app input events. Nothing fires on system suspend,
       screensaver/session lock or window hide; the Rust side has no timer. Non-vault
       commands are callable regardless of lock state; the gate is which screen is showing.
   Mitigation in place: CSP default-src 'self' (tauri.conf.json:12) and textContent-only
       rendering. Instrument: `grep -nE 'innerHTML|outerHTML|insertAdjacentHTML|
       document.write|eval\(|new Function' ui/main.js` -> innerHTML only at :189/:206
       (static SVG map keyed by a fixed kind) and clears at :1233, :1366, :1399, :1636,
       :1675, :1746, :2763, :3099, :3164; textContent 121 sites; no href=/src=/window.open.
   Fix: duplicate the idle timer in Rust (authoritative); lock on logind PrepareForSleep and
       org.freedesktop.ScreenSaver ActiveChanged; add a Rust-side "unlocked surface" gate
       for the settings/erase family so the invariant does not depend on the DOM.

6. Passphrase floor enforced UI-only                                            [Low]
   main.js:290 requires length >= 12; commands.rs:191 accepts any non-empty. Enforce the same
   floor (better: a byte floor) in vault_create.

7. Invite label -> contact alias: validation asymmetry                          [Low]
   Redeem side: REDEEM_NAME_RE = /^[A-Za-z0-9_#-]+$/ (main.js:2863). Create side: non-empty
   only, no maxlength (main.js:2537; index.html:512). On accept the label becomes the alias
   (main.js:3471). qsc backstops with channel_label_ok (lib.rs:2635-2640) enforced at
   contacts_provision_from_invite (contacts/mod.rs:884-886), and the alias IS a path
   component (qsp_session_path, protocol_state/mod.rs:131-137) -- so no traversal, but a
   label like "Dana K" fails only after the peer has redeemed, stranding the invite.
   Fix: apply REDEEM_NAME_RE and maxlength=32 at create; validate again in Rust invite_create.

8. Unlock field retains the rejected passphrase                                 [Low]
   main.js:515-531 clears unlock-pass only on `unlocked`. Clear on rejected/delayed too.

9. settings::load swallows corruption                                           [Low]
   settings.rs:100-103 turns an unparseable file into defaults silently ("silence is not
   success"). Surface it -- a notice kind is the existing channel.

10. CI supply chain                                                             [Low, operator]
   .github/workflows/ci.yml:11,87,108,162,235 actions/checkout@v5; :30,121,182
   dtolnay/rust-toolchain@master; :242 @stable; :216 upload-artifact@v4. No top-level
   permissions: block (grep 'permissions' = 0 hits). Pin to commit SHAs; add
   `permissions: contents: read`. cargo-audit 0.22.0 is installed --locked and run with
   --deny warnings (:130-136) -- good.

11. Crypto dependency status                                                    [Low/Info, spine]
   Cargo.lock: ml-dsa 0.1.0-rc.7 (pre-release) signs identities; ml-kem 0.2.1; sha3 0.10.9
   AND 0.11.0-rc.8; getrandom 0.2.17/0.3.4/0.4.3; rand_core 0.6.4/0.10.1; argon2 0.5.3;
   chacha20poly1305 0.10.1; rustls 0.23.42; ring 0.17.14; rustls-native-certs 0.8.4;
   webpki-roots 1.0.9; x25519-dalek 2.0.1; ed25519-dalek 2.2.0; no hkdf/hmac crate -- the KDF
   is a custom KMAC over sha3 (kmac_out, protocol_state/mod.rs:1030), NOT reviewed here.
   .cargo/audit.toml carries 17 dated waivers; RUSTSEC-2024-0429 (glib unsoundness) is a
   real, stated acceptance. Track the ml-dsa 1.0 release and re-run test vectors at the bump.

12. Invite code on the system clipboard                                         [Info, ruled]
   main.js:2606-2625. Clipboard managers persist history. Consider a timed clear once the
   invite reaches `redeemed`.

13. withGlobalTauri:true and home_dir                                           [Info]
   tauri.conf.json:9 exposes all 46 registered commands (lib.rs:488-546) to any page script;
   commands.rs:868-870 hands $HOME to the page. Matters only given XSS; revisit at the
   messaging slice, when remote-authored text first reaches the DOM.

14. "Certificate: Trusted" on a plaintext loopback relay                        [Low]
   RESOLVED from OWED: validate_relay_endpoint_url (qsc adversarial/route.rs:57-68) accepts
   http:// ONLY when the host is loopback; everything else is relay_tls_required or
   relay_endpoint_invalid_scheme. So main.js:1683 renders "Certificate: Trusted" falsely on
   exactly one configuration: the operator's own loopback http rig. Fix: render
   "Plaintext (loopback only)" when the stored URL scheme is http.

15. Invite-code decoder has no length cap                                       [Low, spine + desktop]
   decode_invite_code (qsc invite/mod.rs:423-441) trims, strips the QSLI-1- prefix and
   base64-decodes the WHOLE remainder before any bound; inner fields are u16-bounded only
   after decode (:239, :344, :474-476, :751-752). The desktop textarea redeem-code
   (index.html:674) has no maxlength, so a multi-megabyte paste allocates ~0.75x its size
   in the serial gateway. Fix: cap at the desktop (maxlength ~2048) AND in
   decode_invite_code before decode.

16. Protection-state files plaintext and unauthenticated                        [Info, spine]
   protection_state_load (qsc vault/protection.rs:464-500) reads the attempt-limit config
   and the failed_unlocks counter from two plaintext files (parse_vault_failed_unlocks
   :425-433), perms-enforced 0600 but with no MAC. A same-user local attacker can reset the
   counter; that attacker already holds vault.qsv for offline attack, so the online guard is
   a UX brake, not a security bound (see Finding 4). Record; no act proposed.


CHECKED AND SOUND (absence is easy to misread)
----------------------------------------------
  - CSP default-src 'self'; style-src 'self' 'unsafe-inline' (tauri.conf.json:12).
  - No dynamic HTML; no navigation/attribute sinks; BANNER_ICONS is a static map.
  - Passphrase inputs type=password autocomplete=off; every screen transition clears the
    ceremony fields (main.js:126-135) and closes the invite/redeem overlays (:96-114).
  - QSCV01 pre-flight gates unlock_guarded so a correct passphrase cannot burn an attempt or
    wipe a legacy vault (commands.rs:270-272); destroy has the independent gate (:415-417).
  - invite_list_at deliberately unregistered (lib.rs:527-528).
  - NOTICE_KINDS positive allowlist; no raw marker text reaches the UI (markers.rs:22, :28-43,
    :126-135).
  - Data dir and qsc/ created 0700 (lib.rs:468-473); settings.json written 0600 via
    create_new + rename (settings.rs:117-128); tighten_mode at launch (lib.rs:464).
  - remove_dir_all / symlink_metadata used so a planted symlink cannot redirect a delete
    (lib.rs:352-387); erase refuses the CLI profile dir by canonical path (commands.rs:483-490).
  - EnvGuard restores on unwind (the hazard is Finding 1, not the restore).
  - qsc states and, per its own fail-closed needle scan, enforces NO TLS-verification bypass
    of any kind (transport/mod.rs:2091-2107); trust = webpki roots UNION native roots, plus
    an additive operator CA.
  - Fixture na0754_ca.pem is a certificate only (`grep -c 'PRIVATE KEY'` = 0).
  - No curl|sh, no secrets, no pull_request_target in ci.yml.
  - Wire error DTO carries facade codes, never Debug renderings (commands.rs:942-974); the
    identity_ensure/identity_show errors DO use {e:?} (commands.rs:226, :237) -- minor.


CLAIM BOUNDARY
--------------
  Static read only. Not covered: the hybrid handshake and the KMAC-based KDF construction;
  ML-KEM decapsulation usage; the `max: usize` bound in invite_accept/invite_finish (facade
  passes it through; invite/mod.rs:726 shows a ttl clamp, not a max clamp -- unmeasured);
  src-tauri/tests/harness/runner.py; scripts/ci/infra_literal_scan.py; the 3,280-line
  design_polish.rs. No cargo audit / cargo build executed here.


SUGGESTED LANE ORDER
--------------------
  1. Env-scrub + drop EnvGuard (desktop, XS-S). Closes 1 and 2's desktop half. Needs the
     spine's explicit-parameter probe API, or ship the scrub alone and keep EnvGuard behind
     it temporarily. SR-15 not triggered mechanically; the shape (retiring a mechanism)
     argues for a short read.
  2. Spine: key-not-passphrase in memory + ZeroizeOnDrop (M). SR-15 triggers. Then the
     Argon2 raise as its own format-version lane.
  3. Spine: cfg-gate the seed/clock/env-passphrase seams (S); cap decode_invite_code.
  4. Desktop bundle (XS each): Rust-side autolock + lock gate; passphrase floor; label
     validation; unlock-field clear; settings corruption notice; loopback-http copy;
     redeem-code maxlength.
  5. CI SHA pins + permissions: (operator, XS).

END
