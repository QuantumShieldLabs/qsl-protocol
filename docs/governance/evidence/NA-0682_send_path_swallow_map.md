# NA-0682 — THE SEND-PATH SWALLOW MAP

**Why this exists:** operator-ruled (STOP 020, order-of-work item A) — *pin the swallow map first,
measurement before fix.* For every failure class the `NA_0663_relay_tls_trust` guard and the C11
census cover: where the cause is **detected**, and where it was **lost**.

⚠ **The headline result overturned my own first hypothesis.** I identified a plausible swallow site
by reading (`pack`'s routing arm, §3 below) and reported the finding in that shape. **Measurement
showed the classifier was never the problem.** The cause is structural and lives in the queue.

---

## 1. THE ACTUAL CAUSE — per-contact FIFO head-of-line blocking

**The classifier works. The message was never attempted.**

| step | file:line | what happens |
|---|---|---|
| detect (CA trio) | `src/transport/mod.rs:1613-1617` | `read_relay_ca_file` → `RELAY_CA_FILE_MISSING` (no metadata) / `RELAY_CA_FILE_UNREADABLE` (not a file) |
| detect (CA invalid) | `src/transport/mod.rs:1641-1643` | not-PEM / empty → `RELAY_CA_FILE_INVALID` |
| classify | `src/transport/mod.rs:2371` | `Err(RelayHttpClientError::CaFile(code)) → fail(code, PushFailClass::CertUntrusted)` |
| classify (HTTP) | `src/transport/mod.rs` (status match) | `401 \| 403 → relay_unauthorized` (C11 Option B keeps the status **class** distinct) |
| map to queue | `RelayMessageSender::classify` | `CertUntrusted → Pause(Cert)`, `TokenRejected → Pause(TokenRejected)`, else `Retry` |
| ⚠ **LOST HERE** | `src/msgqueue/mod.rs:216` + the `break` in `drain_at` | `is_sendable_at` is false for a not-due **or** paused record, and the drain **`break`s the contact's loop**. The message the user just typed is **never attempted**, so no code is ever produced for it. |
| ⚠ **REPORTED HERE** | `src/transport/mod.rs` (the `outcome.sent == 0` block) | with no attempt there is no `last_code()` and no `paused_cause` on **this** record, so it fell to the default: `msgqueue_queued` / *"will send when the relay is reachable"* |

**Nothing is swallowed by the classifier. The reporting layer described the wrong message** — it
reported the state of the row the user just created, when the reason nothing moved belonged to the
row **ahead of it**.

---

## 2. WHY `relay_ca_file_missing` SURVIVED — the asymmetry, explained

`prepared_cfg` (`tests/NA_0663_relay_tls_trust.rs:194`) builds **one** config with **one** contact,
and each test issues several `run_send` calls against it. So:

| sub-case | position in the contact's FIFO | outcome before the fix |
|---|---|---|
| **first** sub-case of each test | **head** — attempted | ✅ real cause surfaced (`relay_ca_file_missing`) |
| every later sub-case | **behind** a not-due or paused head | ❌ never attempted → generic queued line |

⚠ **So the "asymmetry" was not about the CA codes at all.** `missing` survived only because it
happened to be **first**. Had the test ordered `unreadable` first, `unreadable` would have survived
and `missing` would have been the one reported as swallowed.

**And the 401 case was INTERMITTENT for the same reason.** Sub-case (iii) is attempted only if the
head's retry backoff (5 s + jitter, `BACKOFF_LADDER_SECS[0]`) has expired — which depended on how
long TLS key generation took in the preceding sub-cases. **Byte-identical source passed 11/11 twice
and failed on a third run.** That is recorded in §5.

---

## 3. A LATENT SWALLOW SITE — found by reading, NOT the cause of this bug

`src/transport/mod.rs` — `RelayMessageSender::pack`:

```rust
let routing = match resolve_send_routing_target(rec.peer.as_str()) {
    Ok(v) => v,
    Err("device_revoked") => return Err(msgqueue::AttemptResult::FailPermanent),
    Err(_) => return Err(msgqueue::AttemptResult::Retry),   // ⚠ every other code collapses
};
```

**Every routing error other than `device_revoked` collapses to a bare `Retry`** — no marker, no
`last_code`, no cause. ⚠ **This did NOT fire in the NA_0663 failures** (the routing marker is
absent from those runs because `pack` was never reached at all). It is recorded here because the
map's purpose is to be exhaustive, and because it is the same defect shape one layer over: a
`Err(_)` arm that discards a typed cause. **Filed, not fixed in this lane** — fixing it needs its
own controls and no guard currently covers it.

---

## 4. THE FIX (ruled shape, STOP 020)

1. **Enqueue stays unconditional** (O1). Rejected alternative: never-enqueue on config faults —
   the operator ruled it makes the user's typed message lossable, which is the exact silent loss O1
   exists to prevent. Fail-closed governs **transmission**, and enqueueing transmits nothing.
2. **Local, message-independent relay-config faults are classified without a network attempt.**
   `relay_http_client()` is consulted in the not-sent path; a `CaFile` error names the precise code
   even when FIFO held this message behind another.
3. **The record is PAUSED with the trust cause named** (`PausedCause::Cert`) — a sub-state of
   QUEUED saying *why* retries are not running; saving relay settings resumes it.
4. **When this message was not attempted, the HEAD's pause cause is reported** rather than a line
   about this message. ⚠ *"will send when the relay is reachable"* is now **reserved for the
   transient class only.**
5. Existing taxonomy strings reused **verbatim** — no rewording (the consistency sweep owns old
   strings); no new user-facing strings were introduced.

---

## 5. CONTROLS — each breaks the CLASSIFICATION, not a string

| control | what was broken | red produced |
|---|---|---|
| **A** | local config-fault classification disabled (`local_relay_cfg_fault = None`) | `family3_ca_config_failures` RED — *expected relay_ca_file_unreadable, got `msgqueue_enqueued …`* |
| **B** | the three CA causes collapsed (`UNREADABLE → MISSING`) | RED — `code=relay_ca_file_missing` where `unreadable` is required. ⚠ **Distinctness, not mere presence** |
| **C** | auth/trust distinction collapsed (`401\|403 → relay_tls_untrusted`) | `family3_trust_failure…` **and** `family2_explicit_ca_is_additive…` RED |
| **C2** | control C re-run against the **migrated** guard | RED — and the output shows `QSC_ROUTING` + `qsp_pack`, proving the migrated fixture genuinely attempts the send |

Source restored **byte-identical** (`cmp`-verified) after every control.

---

## 6. GUARD MIGRATION (operator clause C) — one sub-case, recorded

**Forcing assertion:** `tests/NA_0663_relay_tls_trust.rs:494` —
`assert!(auth.contains("relay_unauthorized") && !auth.contains("relay_tls_untrusted"))`.

**Why untouched-pass was not reliably possible:** sub-case (iii) shared its config with (i) and
(ii), so it was attempted only once the head's ~5 s backoff expired. ⚠ **Not a clean impossibility
— a FLAKE**, which is worse: byte-identical source passed 11/11 twice and failed the third run.

**Migration:** sub-case (iii) now runs on its own `prepared_cfg("family3_distinct_auth")`.
**Nothing was weakened** — (i) refused and (ii) DNS still assert they are not reported as trust
failures, and (iii) still asserts auth is distinct from trust. Only the fixture is isolated, so the
assertion tests **cause distinctness** instead of queue timing. **Red-capability re-proven on the
migrated form (control C2); three consecutive 11/11 runs after migration.**
