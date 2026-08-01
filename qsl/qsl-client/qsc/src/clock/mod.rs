//! NA-0688 / D622 C1 (R4a–R4e) — THE ONE CLOCK.
//!
//! Every component that asks for wall-clock "now" comes through here. Before this module
//! there were **six** private accessors — `invite::now_unix_s`, `msgqueue::now_unix_s`,
//! `vault::protection::now_unix_s`, `attachments::attachment_now_unix_s`,
//! `dedup::seen_now_unix` and the qsp trigger clock in `lib.rs` — each calling
//! `SystemTime::now()` directly and each independently untestable.
//!
//! ⚠ THIS IS A CONSOLIDATION, NOT A NEW CAPABILITY. The `_at(now)` seam this repo already
//! uses in thirteen places (invite ×3, msgqueue ×7, vault::protection ×3) stays exactly as
//! it is and remains the preferred way to write a deterministic test: pass the instant in.
//! What this module adds is a single place where "now" comes from, so the two modules that
//! had **no** seam at all (`attachments`, `dedup`) stop being unreachable from a test, and
//! so an end-to-end run driven through the CLI can be pinned to an instant.
//!
//! # R4c — wall clock, and the limitation is STATED not defended
//!
//! This is `SystemTime`: wall-clock, settable, and not monotonic. **Clock skew between two
//! devices is an ACCEPTED, STATED limitation, not something this module defends against.**
//! A peer whose clock is wrong will disagree with us about invite expiry, and that
//! disagreement is visible as a refused invite rather than as silent breakage. Anything that
//! needs elapsed-time measurement rather than a date must use `Instant` instead — see
//! `fs_store`, which does exactly that and is deliberately NOT routed through here.
//!
//! # R4d — the privacy invariant this clock serves, restated verbatim
//!
//! **Invite expiry is the ONLY relay-visible time in v1.** Message send-times live inside
//! the session AEAD and are recipient-visible only; log timestamps are local-only and never
//! leave the device without an explicit redacted export. **Any new relay-visible timestamp
//! requires a ruling BEFORE it crosses.** Routing every clock read through one function is
//! what makes that invariant auditable — there is now a single call site to grep for.
//!
//! ⚠ Measured during the D622 census and recorded so nobody re-derives it: **the
//! "log timestamps" consumer named in R4a is EMPTY at base.** `output/mod.rs` mentions
//! timestamps only inside the *redactor* (`looks_like_timestamp`). Nothing was invented here
//! to satisfy the list.

use std::time::{SystemTime, UNIX_EPOCH};

/// Test-only override, honoured by [`now_unix_s`].
///
/// ⚠ NAMED `UNSAFE_TEST` DELIBERATELY, following `QSC_UNSAFE_TEST_SEED_FALLBACK`. The tests
/// in this repo drive `qsc` as a SUBPROCESS, so a thread-local or a compile-time hook cannot
/// reach the code under test — an environment variable is the only injection that crosses
/// the process boundary. That is a real, if small, attack surface: whoever can set this
/// variable can move the client's idea of the current time, and invite expiry is evaluated
/// against it. It is therefore named so that its presence in any real deployment reads as an
/// alarm, and every honoured override emits a loud marker (below).
pub const CLOCK_OVERRIDE_ENV: &str = "QSC_UNSAFE_TEST_CLOCK_UNIX_S";

/// Seconds since the Unix epoch. **The single wall-clock accessor for the whole client.**
///
/// ⚠ FAIL-CLOSED ON A MALFORMED OVERRIDE. If [`CLOCK_OVERRIDE_ENV`] is set but does not
/// parse, this PANICS rather than quietly falling back to the real clock. A silent fallback
/// is the worse failure by far: a negative control that believes it has forced an expired
/// invite, but is actually reading real time, **passes vacuously** and reports a property it
/// never exercised. A loud stop is recoverable; a vacuous pass is not.
pub fn now_unix_s() -> u64 {
    match std::env::var(CLOCK_OVERRIDE_ENV) {
        Ok(raw) => {
            let v = parse_override(&raw);
            // Loud by design: an overridden clock must never be invisible in a log.
            crate::output::emit_marker(
                "clock_override",
                None,
                &[("source", "env"), ("unix_s", raw.trim())],
            );
            v
        }
        Err(_) => SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    }
}

/// The override's parsing rule, split out as a PURE function.
///
/// ⚠ SPLIT OUT DELIBERATELY, AND THE REASON MATTERS MORE THAN THE SHAPE. Unit-testing the
/// fail-closed rule by actually setting the environment variable would mutate PROCESS-GLOBAL
/// state while sibling unit tests run in parallel in the same process — any one of them that
/// happened to call `now_unix_s()` would panic on a value it never set. That is exactly the
/// cross-test contamination class ENG-0094 filed after NA-0687 chased it through twelve
/// capture sites. Testing the rule where it is pure removes the hazard by construction
/// rather than by scheduling luck.
fn parse_override(raw: &str) -> u64 {
    let trimmed = raw.trim();
    trimmed.parse::<u64>().unwrap_or_else(|_| {
        panic!(
            "{CLOCK_OVERRIDE_ENV} is set but is not a whole number of seconds \
             ({trimmed:?}). Refusing to fall back to the real clock: a test that believes \
             it pinned the time, and did not, passes vacuously."
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real clock is used when nothing is pinned, and it is plainly a real date.
    #[test]
    fn the_default_is_the_real_wall_clock() {
        // 2020-01-01. Any real run is after this; a zero or epoch value would fail.
        assert!(
            now_unix_s() > 1_577_836_800,
            "the unpinned clock must return real wall-clock time"
        );
    }

    /// A well-formed override parses, including with surrounding whitespace.
    #[test]
    fn a_well_formed_override_parses() {
        assert_eq!(parse_override("1700000000"), 1_700_000_000);
        assert_eq!(parse_override("  1700000000\n"), 1_700_000_000);
    }

    /// ⚠ THE FAIL-CLOSED RULE. Without it a malformed override degrades to the real clock,
    /// and every negative control built on the injected clock silently stops testing what it
    /// claims to test — a vacuous pass, which is the one failure mode worse than a red.
    #[test]
    fn a_malformed_override_panics_rather_than_falling_back_to_real_time() {
        for bad in ["not-a-number", "", "-1", "12.5", "9999999999999999999999"] {
            let res = std::panic::catch_unwind(|| parse_override(bad));
            assert!(
                res.is_err(),
                "a malformed clock override ({bad:?}) must fail closed, never fall back"
            );
        }
    }
}
