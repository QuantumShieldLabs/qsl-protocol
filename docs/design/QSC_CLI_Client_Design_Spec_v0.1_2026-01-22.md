# QuantumShield Chat (QSC) — Terminal-First CLI Client Design Specification (Design Lane)
Date: 2026-01-22 (America/Chicago)  
Status: Design draft for Build/Director review  
Scope: Client UX/logic only; **no protocol wire format changes**; no new cryptographic primitives.

---

## 1. Purpose

QSC is a terminal-first chat client and public demo app for QuantumShield (QSL). It must be:
- **Dead-simple to run** by third parties (one command → `qsc>`).
- **Credible** about metadata minimization (explicitly scoped; no overclaims).
- **Extremely secure by default** (fail-closed, deterministic behavior, encrypted at rest, no secret printing).
- **Deterministic for demos/CI** (machine-readable markers, stable error classes, scriptable commands).

### Non-goals (avoid drift)
- No redesign of QSL protocol, no wire format changes.
- No claims of “metadata-free,” “anonymous,” or “traffic-analysis resistant.”
- No new crypto primitives beyond what exists in the protocol build.

---

## 2. UX Model

### 2.1 Shell-first
Running `qsc` with no arguments enters an interactive shell immediately at the prompt:

- Neutral prompt: `qsc>`
- Active chat prompt: `qsc[bob]>`

Plain text lines in `qsc[bob]>` are sent to Bob. In neutral `qsc>`, plain text is rejected with a hint (“Use `/chat bob`.”). This prevents accidental misdelivery.

### 2.2 Scriptable subcommands
All core operations exist as non-interactive commands for automation, CI, and reproducible demos:
- `qsc send bob --text "hi"`
- `qsc recv --once --json`
- `qsc status --json`
- `qsc export --out demo.jsonl`

### 2.3 Deterministic “demo credibility” markers
QSC can emit an optional machine-readable event stream (JSON Lines) prefixed with a stable marker header:
- `QSC_MARK/1 { ...json... }`

Markers are used for demo scripts and CI assertions. See §5.

---

## 3. Security Model and Invariants (Client-Level)

### 3.1 Core invariants
1. **Fail-closed**: any parse/auth/state error → deterministic reject, no partial success.
2. **No-mutation-on-reject**: when a frame is rejected, **cryptographic/session state must not advance**.
3. **Deterministic error classes**: stable, small set of error codes; avoid verbose oracle strings.
4. **No secret material printed**: never output private keys, session keys, raw vault, or unredacted state.
5. **Encrypted at rest** by default; no silent insecure mode.
6. **Atomic state writes**: write temp → fsync → atomic rename → fsync directory; no partial state.
7. **Locking**: exclusive locks for mutations; shared/read locks for reads.
8. **Terminal output safety**: sanitize/escape inbound message text to prevent terminal injection.

### 3.2 Trust semantics
- **Identity fingerprint is the identity.**
- Local alias/username is optional and local-only by default.
- Trust levels:
  - `unverified`: fingerprint known but not out-of-band verified.
  - `verified`: user attested out-of-band verification (voice/QR/OOB).
  - `blocked`: refuse open/send; optionally drop/reject inbound for that fingerprint.
- Verified contacts are **pinned**: identity mismatch is a hard fail; no silent key rollover.

### 3.3 Endpoint semantics (routing hint only)
Endpoint/route hints are not identity. Endpoints are optional and **hidden by default** because they are high-signal metadata (relay choice, topology). Endpoints are used only for dialing, and never treated as proof of identity.

---

## 4. Local Storage and Hardening

### 4.1 Store location
Default: `~/.qshield` (configurable via `--store <path>`).

### 4.2 File permissions
- Directories: `0700`
- Files: `0600`
- Init enforces restrictive umask (e.g., `077`) and rejects unsafe ownership/permissions.

### 4.3 Vault / encryption at rest
The client creates a profile master key by one of:
1. **Preferred:** OS keychain (store random 32-byte master key).
2. **Fallback:** passphrase-derived master key (Argon2id; stored salt + params).

No silent plaintext store. Any “unencrypted mode” (if allowed at all) must require an explicit danger flag and must not be suggested in normal help output.

### 4.4 Atomicity + rollback
All mutations use atomic replace patterns. On failure, QSC must not leave partial state. Incomplete temp artifacts are cleaned or quarantined under `tmp/`.

### 4.5 Suggested store layout (conceptual)
- `identity/` (vault + public metadata)
- `contacts/` (encrypted contacts store)
- `sessions/` (encrypted session state + index)
- `messages/` (encrypted inbox/outbox/index)
- `logs/` (redacted event/marker log, still protected)
- `tmp/` (atomic write staging)

---

## 5. Machine-Readable Markers (Optional)

### 5.1 Format
Each marker line begins with:
- `QSC_MARK/1 ` followed by a single JSON object.

Minimum schema fields:
- `event` (string)
- `ts` (timestamp or monotonic)
- `session` (optional; short id)
- `role` (optional)
- `data` (object)

Example:
- `QSC_MARK/1 {"event":"send.ok","ts":"...","session":"8d3e...","data":{"to":"bob","msg_id":"m-123","trust":"verified"}}`

### 5.2 Backward compatibility
- Adding new fields is allowed.
- Renaming/removing fields is forbidden within major marker version.

### 5.3 Placeholder behavior rule
If a placeholder feature is invoked, exit code `2` and (if markers enabled):
- `event = "feature.unavailable"` with `data.feature = "<name>"`.

---

## 6. Identifiers

### 6.1 Identity fingerprint (client fingerprint)
A public, stable identifier derived from canonical encoding of identity public key material, domain-separated and versioned (e.g., `QSC1-...`). Safe to display and share out-of-band.

### 6.2 Session fingerprint (optional)
A hash/identifier derived from session transcript/public parameters to confirm both sides share the same session. Not a substitute for identity verification.

### 6.3 Optional username/display name
Optional local label. Must not be treated as identity and must not be transmitted by default.

---

## 7. Command Inventory (Implemented + Placeholders)

**Entry / Modes**
- `qsc` — start interactive shell at `qsc>` (bootstrap if not initialized).
- `qsc shell` — explicit shell entry (same as `qsc`).
- `qsc demo local` — one-command local demo (Alice/Bob in-process).
- `qsc demo relay` — two-process relay demo (requires relay endpoint).
- `qsc --help` — help/usage.

**Identity**
- `qsc init` — create secure local profile (vault, keys, config) (local-only).
- `qsc whoami` — show public identity fingerprint and safe capabilities.
- `qsc device list` — placeholder (multi-device).
- `qsc device add` — placeholder (multi-device).
- `qsc keys rotate` — placeholder/phase 2 (identity/prekey/PQ rotation).

**Contacts**
- `qsc contacts list` — list contacts (fingerprint + trust; endpoints hidden by default).
- `qsc contacts add` — add/update contact (fingerprint required; trust default unverified).
- `qsc contacts remove` — remove contact (atomic, confirmed).
- `qsc contacts trust` — set trust level (verified/unverified/blocked).
- `qsc contacts verify` — guided OOB verification (voice/QR/OOB) and attestation record.

**Sessions**
- `qsc open` — explicit session open/handshake (optional; send can auto-open).
- `qsc close` — disconnect/unload session; optional `--forget` destroys persisted session.
- `qsc status` — show posture: init/vault, active session(s), trust, last reject.

**Messaging**
- `qsc send` — send message; auto-handshake if needed; can target alias or fingerprint.
- `qsc inbox` — list locally stored inbound messages (local-only).
- `qsc recv` — network receive: pull/stream inbound, validate, store locally.
- `qsc ack` — placeholder (receipt/ACK semantics; metadata-sensitive).

**Export / Audit**
- `qsc export` — create redacted proof artifact (session logs, markers, counts).
- `qsc selftest` — local crypto/backend/environment checks (no secrets).
- `qsc log level` — placeholder/phase 2 (log verbosity control).

**Conformance / Vectors**
- `qsc vectors run` — run suite vectors (CI-friendly exit codes).

**Adversarial Demo Hooks**
- `qsc attack tamper` — boundary tamper next outbound frame (demo proof).
- `qsc attack replay` — replay last N frames (prove replay reject + no-mutation).
- `qsc attack reorder` — reorder frames (prove state-machine robustness).

**Privacy Controls (Metadata Credibility)**
- `qsc privacy status` — show active privacy posture (bucketing/padding/tick if enabled).
- `qsc privacy set` — configure size bucketing/padding at client boundary (no wire changes).
- `qsc privacy envelope set` — placeholder (tick schedule, bundle packing; roadmap-only unless implemented).

**Groups (Placeholder)**
- `qsc group create/add/send` — placeholder (group messaging).

**Attachments (Placeholder)**
- `qsc send --file` / `qsc recv --save-dir` — placeholder (attachments).

---

## 8. Command Specifications (Step-by-step)

### 8.1 `qsc init`
**What it is**: Creates the secure local client profile (vault, keys, store skeleton, config). No network calls.  
**Used for**: Hardening foundation for all other operations.

**Step-by-step**
1. Preflight: restrictive umask; resolve store path; reject unsafe path/ownership; confirm CSPRNG availability.
2. Acquire exclusive init lock (no concurrent mutation).
3. Create store skeleton (`0700` dirs); reject symlinks; verify permissions/ownership.
4. Establish encryption-at-rest:
   - keychain master key preferred; else passphrase → Argon2id-derived master key.
   - if neither available: fail closed (no silent plaintext).
5. Generate identity key material via CSPRNG (per protocol suite needs).
6. Compute identity fingerprint from canonical public identity bytes (domain-separated/versioned).
7. Write vault and config atomically:
   - `identity/vault.enc` (AEAD)
   - `identity/public.*` (public only)
   - encrypted empty stores for contacts/sessions/messages.
8. Permission + invariant audit:
   - traverse and verify permissions, ownership, no symlinks.
   - optional decrypt header check to confirm keychain/passphrase wiring works.
9. Output: `INIT_OK name=<optional> fingerprint=<...>` and exit 0.

**Deterministic errors (examples)**: `NOT_INITIALIZED` (n/a), `INIT_LOCKED`, `STORE_UNSAFE`, `RNG_UNAVAILABLE`, `VAULT_SETUP_FAILED`, `WRITE_FAILED`.

---

### 8.2 `qsc whoami`
**What it is**: Read-only identity introspection.  
**Used for**: Confirm active identity; share fingerprint OOB.

**Steps**
1. Resolve store; ensure initialized.
2. Optional shared lock.
3. Load public identity metadata; validate schema/version.
4. Optionally verify vault accessibility only if explicitly requested.
5. Output fingerprint and optional name; exit 0.

Errors: `NOT_INITIALIZED`, `CORRUPT_IDENTITY`.

---

### 8.3 `qsc contacts list`
**What it is**: Local listing of contacts (encrypted store).  
**Used for**: Confirm saved identities/trust; drive open/send flows.

**Steps**
1. Ensure initialized; shared lock; unlock vault if required.
2. Decrypt contacts store; integrity-check.
3. Apply filters; deterministic ordering.
4. Render table (endpoints hidden by default); JSON if `--json`.

Errors: `NOT_INITIALIZED`, `VAULT_LOCKED`, `CONTACTS_CORRUPT`.

---

### 8.4 `qsc contacts add`
**What it is**: Add/update a contact (fingerprint required).  
**Used for**: Save peers; store trust and optional endpoints (route hints).

**Steps**
1. Ensure initialized; exclusive contacts lock; unlock vault.
2. Validate fingerprint format; normalize canonical representation.
3. Validate alias and endpoint schemes (reject control chars).
4. Decrypt contacts store; integrity-check.
5. Dedup rules:
   - primary key = fingerprint.
   - alias conflicts fail deterministically unless explicit rename flow.
6. Default trust = `unverified` for new contacts.
7. Write encrypted store atomically; output `CONTACT_ADD_OK`.

Errors: `FINGERPRINT_INVALID`, `ALIAS_CONFLICT`, `UNSUPPORTED_ENDPOINT_SCHEME`.

---

### 8.5 `qsc contacts remove`
**What it is**: Delete contact record (local-only).  
**Used for**: Clean up; reduce local relationship exposure.

**Steps**
1. Ensure initialized; exclusive lock; unlock vault.
2. Resolve selector (alias or fingerprint); fail on ambiguity.
3. Confirm unless `--yes`.
4. Remove record; write atomically; output `CONTACT_REMOVE_OK`.

Errors: `CONTACT_NOT_FOUND`, `ALIAS_AMBIGUOUS`, `CONFIRMATION_REQUIRED`.

---

### 8.6 `qsc contacts trust`
**What it is**: Set trust to verified/unverified/blocked (policy setting).  
**Used for**: Enforce secure posture across open/send/recv.

**Steps**
1. Ensure initialized; exclusive lock; unlock vault.
2. Resolve contact; validate transition; confirm on block/unblock unless `--yes`.
3. Update trust metadata; write atomically; output `CONTACT_TRUST_OK`.

Errors: `CONTACT_NOT_FOUND`, `CONFIRMATION_REQUIRED`.

---

### 8.7 `qsc contacts verify`
**What it is**: Guided OOB verification workflow (voice/QR/OOB).  
**Used for**: Promote contact to verified with recorded attestation.

**Steps**
1. Ensure initialized; exclusive lock; unlock vault.
2. Resolve contact; compute expected peer verification code derived from fingerprint.
3. Present method-specific instructions; require explicit user confirmation or `--code` match.
4. On success: record attestation (method, timestamp); set trust verified (default).
5. On mismatch: keep unverified (or optionally block via explicit user choice).
6. Write atomically; output `CONTACT_VERIFY_OK` or `CONTACT_VERIFY_FAIL`.

Errors: `CODE_MISMATCH`, `CONFIRMATION_REQUIRED`.

---

### 8.8 `qsc open`
**What it is**: Explicitly create/activate a session and run handshake. Optional for normal use.  
**Used for**: Demo clarity, preflight connectivity, diagnostics.

**Steps**
1. Ensure initialized; unlock vault.
2. Resolve contact; enforce trust (blocked → refuse; unverified → warn or refuse under strict policy).
3. Choose suite/transport/endpoint; connect with timeouts.
4. Run handshake fail-closed; authenticate transcript binding; verify pinned identity match.
5. Commit session state atomically **only on success**.
6. Output `SESSION_OPEN_OK` with session id + transcript hash (safe).

Errors: `CONTACT_BLOCKED`, `ENDPOINT_REQUIRED`, `HANDSHAKE_TIMEOUT`, `PEER_IDENTITY_MISMATCH`.

---

### 8.9 `qsc close`
**What it is**: Disconnect/unload a session; optional `--forget` deletes persisted session state.  
**Used for**: End chat, unload sensitive state, force fresh handshake.

**Steps**
1. Ensure initialized; unlock vault.
2. Resolve target session; exclusive session lock.
3. Disconnect transport; stop receiver tasks; zeroize in-memory sensitive state.
4. If `--forget`: confirm; delete session record atomically.
5. Output `SESSION_CLOSE_OK`.

Errors: `SESSION_NOT_FOUND`, `CONFIRMATION_REQUIRED`.

---

### 8.10 `qsc status`
**What it is**: Read-only posture report (demo-safe).  
**Used for**: Confirm init/vault, active sessions, trust posture, last reject, privacy posture.

**Steps**
1. Resolve store; show init state.
2. Do not prompt by default; report `vault=locked` if locked.
3. If unlocked: load session index; summarize deterministic.
4. Output minimal by default; add safe details with `--verbose` and/or JSON.

Errors: `NOT_INITIALIZED` (optional; may be informational only).

---

### 8.11 `qsc send`
**What it is**: Send message to alias or fingerprint. Auto-handshakes if needed; `open` not required.  
**Used for**: Normal messaging, automation, demos.

**Steps**
1. Ensure initialized; unlock vault.
2. Resolve recipient (contact or fingerprint). Enforce trust:
   - blocked → refuse with no network traffic.
   - unverified → warn or require explicit allow under strict profile.
3. Resolve suite/transport/endpoint; acquire per-peer session lock.
4. If no valid session: perform handshake implicitly (same invariants as `open`).
5. Frame/encrypt; apply privacy bucketing/padding (if enabled).
6. Send over transport with bounded timeout.
7. Commit durable session/message state **only if send is durably accepted/queued** (director decision; see §10).  
8. Output `SEND_OK` (and handshake status reused/new).

Errors: `ENDPOINT_REQUIRED`, `UNVERIFIED_REFUSED`, `TRANSPORT_SEND_FAILED`.

---

### 8.12 `qsc inbox`
**What it is**: Local listing of stored inbound messages (no network).  
**Used for**: Review messages; demo without live receive.

**Steps**
1. Ensure initialized; shared lock; unlock vault if required.
2. Decrypt message index; integrity-check.
3. Filter/limit; sanitize previews.
4. Render minimal table by default; timestamps/endpoints opt-in.

Errors: `MESSAGES_CORRUPT`, `VAULT_LOCKED`.

---

### 8.13 `qsc recv`
**What it is**: Network receive: poll or follow inbound; validate fail-closed; store locally.  
**Used for**: Pull queued relay messages; streaming chat receive.

**Steps**
1. Ensure initialized; unlock vault; load contacts + sessions.
2. Choose receive scope (all sessions or `--contact`).
3. Transport loop:
   - `--once`: bounded poll/read then exit.
   - `--follow`: fixed-interval polling for relay if configured; stream for direct.
4. For each inbound frame:
   - strict parse, authenticate, decrypt, state-machine validate
   - enforce pinned identity match
   - enforce trust (blocked → drop/reject with no mutation, no store)
   - **no-mutation-on-reject** for all rejects
5. On success: store message encrypted; commit session update atomically.
6. Output summary; optionally print sanitized previews/full (explicit flag).

Errors: `PARSE_FAILED`, `FRAME_AUTH_FAILED`, `REPLAY_DETECTED`, `CONTACT_BLOCKED`.

---

### 8.14 `qsc export`
**What it is**: Produce redacted proof artifact (JSONL/JSON/text).  
**Used for**: Shareable demo/audit evidence without secrets.

**Steps**
1. Ensure initialized; unlock vault.
2. Resolve scope (`--session`, `--contact`, `--all` or active session default).
3. Load session/message/marker data; integrity-check.
4. Apply redaction policy (strict by default; no endpoints, no plaintext).
5. Serialize deterministically; write atomically to `--out`.
6. Output `EXPORT_OK path=... redact=strict events=n`.

Errors: `SCOPE_REQUIRED`, `WRITE_FAILED`, `EXPORT_SOURCE_CORRUPT`.

---

### 8.15 `qsc vectors run`
**What it is**: Runs protocol conformance vectors and returns CI-friendly exit codes.  
**Used for**: Prove engine compliance from CLI; integrated into demo pipeline.

**Steps**
1. Invoke existing vector runner for suite; surface summary.
2. Exit 0 on pass; non-zero on failure.
3. Optional markers: `vectors.start`, `vectors.ok`, `vectors.fail`.

---

### 8.16 Demo hooks: `qsc attack tamper|replay|reorder`
**What it is**: Boundary-level adversarial hooks for demo proof.  
**Used for**: Demonstrate deterministic rejects and no-mutation invariants.

**Rules**
- Must operate at the transport/frame boundary (not by poking secret engine internals).
- Must not weaken production posture unless explicitly compiled/flagged for demo mode.

---

### 8.17 Privacy controls: `qsc privacy status|set`
**What it is**: Surfacing and controlling metadata-relevant client boundary behavior.  
**Used for**: Credible claims about what is and is not mitigated (e.g., size leakage bucketing).

**Phase guidance**
- Phase 1: implement `privacy status` and a minimal fixed bucket schedule + padding (if feasible without wire changes).
- Tick scheduling/bundle packing is roadmap-only unless built and tested without overclaims.

---

## 9. Interactive Shell Commands (Minimum Set)

Inside `qsc>`:
- `/help` — show commands.
- `/contacts` — list contacts.
- `/chat <contact>` (alias `/open <contact>`) — set active chat target; auto-handshake if needed.
- `/back` — leave active chat (return to neutral prompt).
- `/status` — show posture.
- `/export --out <path>` — export proof artifact.
- `/quit` — exit.

Plain text:
- In `qsc[bob]>`: send to bob.
- In `qsc>`: refuse with hint (prevents accidental sends).

---

## 10. Director Notes / Decisions Needed (Build Impact)

These are the key decisions that affect implementation details and tests:

1. **Default policy for unverified contacts**
   - Option A: allow sends/opens but warn prominently.
   - Option B: strict default refuse unless user explicitly opts in (`--allow-unverified` or profile setting).

2. **Behavior for sending to an unknown fingerprint (no contact record)**
   - Refuse by default (secure) vs treat as unverified with explicit endpoint requirement.

3. **Send-state commit semantics**
   - For “extremely secure” and “no mutation on failure,” prefer committing session state only when the outbound is durably accepted/queued.
   - If the engine advances send state upon encryption, Build must provide a safe mechanism (staging/transaction) or accept a defined behavior with tests.

4. **Prompting policy**
   - Strong recommendation: `status` and other read-only commands should not prompt by default (better for scripts).
   - Mutation commands may prompt unless `--yes/--no-prompt` is provided.

5. **Relay receive strategy for metadata credibility**
   - Fixed polling interval option (`--poll-ms`) surfaced explicitly with clear claims.
   - Avoid implying anonymity; state it reduces timing variability in polling only.

6. **Markers default**
   - Recommended default: markers off; enable with `--markers file|stdout`.
   - CI/demos can turn on markers deterministically.

7. **Endpoints as metadata**
   - Endpoints hidden by default in lists/status/export; shown only with explicit flags.

8. **Demo hooks safety**
   - Ensure attack hooks are demo-only or explicitly gated, and cannot be triggered accidentally in normal mode.

---

## 11. Public Claim Language (Safe)
- “QSC is a secure-by-default terminal chat client and demo app for QuantumShield.”
- “It supports deterministic demo logs (markers) and conformance vector execution.”
- “It can reduce message length leakage via explicit size bucketing/padding (if enabled).”
- “Proxy support is for connectivity; it does not imply anonymity or traffic-analysis resistance.”

---

End of document.

---

## QSC Recommended Additions (public-ready, defensible)

### A) Policy Profiles (baseline vs strict)
Add an explicit policy profile table to make defaults legible and avoid surprising behavior.

| Profile  | Unverified contacts | Unknown fingerprints | Verified contacts | Blocked contacts |
|----------|---------------------|----------------------|------------------|------------------|
| baseline | allow with warning  | allow with warning   | pin verified     | refuse           |
| strict (default) | refuse unless explicit allow flag | refuse by default | pin verified | refuse |

Notes:
- Strict is the recommended default posture for public demos and production.
- Baseline may exist for internal comparison/testing but must not be the default.

### B) Recovery/Repair command (explicit, safe)
Add a command deliberately separated from normal flows:
- qsc doctor --check-only (or qsc repair --check-only)
  - Detect permission problems, corrupt stores, missing keychain entries, mis-owned dirs, unsafe symlinks.
  - Never auto-fixes unless explicitly requested.
  - Emits a deterministic report and stable markers suitable for support and CI (without secrets).

### C) Threat/Metadata disclosure checklist (for demos)
Add a short checklist ensuring public demos remain honest and repeatable:
- What status shows by default (no endpoints/timestamps unless explicitly requested).
- What export --redact guarantees (and what it does not).
- What proxy mode does and does not claim.
- What bucketing/padding does and does not claim.
- What receipts/acks do and do not claim.

---

## Director/Build Correctness Edges (must be explicit and testable)

### 1) Send-state commit semantics (highest priority)
Recommended resolution:
- Define qsc send success as “durably queued”:
  - Encryption outputs + state transitions are committed atomically only when the outbox entry is persisted.
  - Delivery is handled separately (e.g., qsc flush / qsc send --sync) and does not redefine send success.

Test obligations:
- If a command exits non-zero, session/ratchet state MUST NOT advance (no-mutation-on-failure).
- Crash consistency: no partial state after abrupt termination (atomic rename discipline).

### 2) Session selection/routing in recv (speed vs determinism vs oracle risk)
Requirements:
- Prefer deterministic routing by safe hints if available.
- If try-decrypt is needed, bound attempts deterministically and normalize reject classes.

Test obligations:
- Bounded attempt count enforced.
- Uniform rejects (stable codes/markers; avoid verbose “why” strings).
- No-mutation-on-reject for malformed/auth-fail/replay probes.

### 3) Cross-platform secure storage (keychain + fallback)
Requirements:
- Non-interactive unlock path for CI (e.g., --passphrase-file / env var / fd).
- Non-interactive commands must not block on prompts by default.

Test obligations:
- Deterministic behavior under “no keychain available” conditions.
- Deterministic “locked” vs “unlocked” command semantics.

### 4) Deterministic tests for interactive behavior
Requirements:
- Use a pseudo-tty harness for shell-mode tests.
- Assert on stable markers (not human text).
- Avoid wall-clock timestamps by default (or provide deterministic substitute).
## YubiKey integration roadmap (plumbing now, enforce later)

### What we are solving
We want a **hardware-backed option** to unlock QSC’s encrypted-at-rest vault (protecting stored secrets even if the disk is copied).
This is a **data-at-rest** control, not an anonymity feature.

### Non-goals (explicit)
- Not a guarantee against a fully compromised host (malware can still act as the user while unlocked).
- Not a replacement for protocol E2EE; this protects **local storage**.
- Not a claim of tamper-proofing beyond the token’s properties.

### “Plumbing now”
We standardize on a **keyslot model** for the vault master key:
- Slot type A: passphrase-derived key (Argon2id; deterministic noninteractive behavior).
- Slot type B: OS keychain binding when available (preferred).
- Slot type C (future): hardware token (YubiKey) slot provider.

Design requirements for the future YubiKey slot:
- The vault file format MUST support **multiple keyslots** (for migration/recovery).
- Token integration MUST be implemented behind an explicit feature/policy gate (no silent behavior changes).
- Noninteractive mode MUST never prompt; it MUST fail closed with a stable marker.

Suggested interfaces (conceptual; not necessarily code names):
- `KeySlotProvider` abstraction to derive/unwrap a vault master key.
- A deterministic “provider unavailable” error path with stable marker/code.
- A test hook that uses a mock provider to exercise state boundaries without requiring hardware.

### “Enforce later”
Policy posture should remain legible:
- baseline profile: token optional; passphrase/keychain allowed.
- strict profile: token can be REQUIRED only when explicitly configured; never silently.

### Recovery/rotation notes
- Support adding a new slot and retiring an old slot (migration).
- Require an explicit operator action for destructive changes (no “silent repair”).
- Provide a deterministic `doctor` output indicating whether a token slot is configured/required.

