//! NA-0770 (D-1411) — THE RETIREMENT GUARDS.
//!
//! `AckMode::Legacy` — delete-on-pull — is retired. Two properties held the retirement together and
//! neither is expressible as a behavioural test any more, because BOTH of the arms that used to
//! measure them worked by CONTRASTING the two modes. With one mode shipped there is nothing to
//! contrast, so these guards assert the properties AT THE SOURCE instead.
//!
//! ## ⚠⚠ WHY SOURCE-SHAPE, AND WHAT THAT COSTS
//!
//! A source-shape assertion is WEAKER than a runtime one in a specific, nameable way: it pins how
//! the tree is WRITTEN, not how the program BEHAVES. A future change that preserved the spelling
//! while altering the effect would pass. That trade is not chosen for convenience — it is forced:
//!
//!   * **G-A** replaces `NA_0644_ack_client.rs`'s negative pull-URL observation, which asserted
//!     that a legacy pull produced a URL WITHOUT `ack=lease`. That was the tree's last NEGATIVE
//!     pull-URL observation (loss **L6**) and it dies with the mode — there is no longer any way to
//!     make the client emit a non-lease pull URL, so no runtime arm can distinguish the two.
//!   * **G-B** pins ack CONFINEMENT. The behavioural evidence survives in
//!     `na0742_invite_finish_scan_producer_acks.rs`'s T7 lease half, which measures that the reply
//!     is the ONLY frame acked. G-B adds what that arm cannot see: that no THIRD caller has since
//!     appeared. A new caller would not make T7 red — T7 exercises its own flow only.
//!
//! ⚠ NEITHER GUARD IS A COUNT DRESSED UP AS A PROPERTY. G-A asserts SINGLENESS (one construction
//! site, so there is no second shape to drift) and G-B asserts IDENTITY (which functions may reach
//! the ack, by name — not how many times they do).
//!
//! ⚠ THE EXCLUSIONS ARE CHARACTERIZED, NOT LINE-PINNED. `adversarial/route.rs` contains `/v1/pull?`
//! inside RAW HTTP REQUEST BYTES used to drive the relay's own parser — they are not client URL
//! constructions. Excluding them by line number would rot the moment a line moved above them (this
//! lane already produced one such stale cite). So they are excluded by SHAPE — a `b"GET ` byte
//! literal — and anything in that file which is NOT that shape still fails the guard.

use std::fs;
use std::path::{Path, PathBuf};

fn src_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every `.rs` file under `src/`, as (path-relative-to-src, contents).
///
/// ⚠ SCOPED TO `src/` DELIBERATELY: these guards must never scan `tests/`, or they would match the
/// needles spelled in this very file and pass on their own text.
fn rust_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<(String, String)>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()))
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                walk(&p, root, out);
            } else if p.extension().map(|e| e == "rs").unwrap_or(false) {
                let rel = p
                    .strip_prefix(root)
                    .expect("under root")
                    .to_string_lossy()
                    .replace('\\', "/");
                out.push((rel, fs::read_to_string(&p).expect("read source")));
            }
        }
    }
    let root = src_root();
    let mut out = Vec::new();
    walk(&root, &root, &mut out);
    assert!(
        out.len() > 10,
        "the source walk found only {} files — the guard would pass vacuously",
        out.len()
    );
    out
}

// ===========================================================================
// G-A — the pull URL has exactly ONE source shape, and it carries ack=lease.
// ===========================================================================
#[test]
fn ga_the_pull_url_has_exactly_one_construction_and_it_is_lease() {
    let mut constructions: Vec<(String, usize, String)> = Vec::new();
    let mut excluded_raw_http = 0usize;

    for (rel, text) in rust_sources() {
        for (i, line) in text.lines().enumerate() {
            if !line.contains("/v1/pull?") {
                continue;
            }
            // ⚠ A COMMENT CANNOT CONSTRUCT A URL. This branch exists because the guard's own
            // description, written beside the construction it guards, matched its own needle and
            // made the guard red on first run. Prose mentioning the shape is not the shape.
            if line.trim_start().starts_with("//") {
                continue;
            }
            // Raw HTTP request bytes driving the relay's own parser are not URL constructions.
            if line.contains("b\"GET ") {
                excluded_raw_http += 1;
                continue;
            }
            constructions.push((rel.clone(), i + 1, line.trim().to_string()));
        }
    }

    // The exclusion must not be silently vacuous: if those fixtures ever vanish, this guard has
    // stopped exercising the branch that keeps it honest, and that is worth knowing.
    assert!(
        excluded_raw_http >= 2,
        "expected the adversarial raw-HTTP `/v1/pull?` fixtures to still exist (found \
         {excluded_raw_http}). If they were removed, drop this assertion deliberately; if the \
         guard simply stopped seeing them, the exclusion above is now matching the wrong thing."
    );

    assert_eq!(
        constructions.len(),
        1,
        "SINGLENESS IS THE ASSERTION. The client must build its pull URL in exactly ONE place, so \
         there is no second shape that can drift away from lease. Found {}:\n{}",
        constructions.len(),
        constructions
            .iter()
            .map(|(f, n, l)| format!("  {f}:{n}  {l}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let (file, line_no, line) = &constructions[0];
    assert!(
        line.contains("&ack=lease"),
        "the single pull-URL construction at {file}:{line_no} no longer carries `&ack=lease`. \
         Acknowledged-pull is the ONLY delivery mode this client has; a pull without it is the \
         retired delete-on-pull behaviour returning by the back door:\n  {line}"
    );
    assert!(
        line.contains("format!"),
        "the pull URL at {file}:{line_no} is expected to be a single `format!` literal — if it is \
         now assembled piecewise, this guard can no longer read the mode off it and must be \
         rewritten rather than deleted:\n  {line}"
    );
}

// ===========================================================================
// G-B — `producer_ack` is reachable through exactly TWO wrappers, by name.
// ===========================================================================

/// The wrappers permitted to reach `transport::producer_ack`, as (file, enclosing fn).
///
/// ⚠ THIS IS AN IDENTITY SET, NOT A BUDGET. Adding a caller is not "one more of the same"; it is a
/// new place where a frame can be acked, and every such place must be re-argued against the
/// ack-after-durable-persist contract. If you are adding one, change this list DELIBERATELY and say
/// why in the commit — do not relax the assertion to a count.
const PERMITTED_ACK_WRAPPERS: &[(&str, &str)] = &[
    ("handshake/mod.rs", "hs_emit_producer_ack"),
    ("invite/mod.rs", "emit_producer_ack"),
];

/// The name of the function enclosing `line_idx`, found by scanning backwards for the nearest
/// `fn <name>` at the start of a line (any indent).
fn enclosing_fn(text: &str, line_idx: usize) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    for i in (0..=line_idx).rev() {
        let t = lines[i].trim_start();
        for prefix in ["pub(crate) fn ", "pub fn ", "fn ", "async fn "] {
            if let Some(rest) = t.strip_prefix(prefix) {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }
    None
}

#[test]
fn gb_producer_ack_is_reached_through_exactly_two_named_wrappers() {
    let mut definitions = 0usize;
    let mut call_sites: Vec<(String, usize, String)> = Vec::new();

    for (rel, text) in rust_sources() {
        for (i, line) in text.lines().enumerate() {
            if line.contains("fn producer_ack(") {
                definitions += 1;
                continue;
            }
            // An invocation is path-qualified; a bare `"producer_ack"` is a marker STRING and a
            // mention in a comment is prose. Neither reaches the wire.
            if !line.contains("transport::producer_ack(") {
                continue;
            }
            let f = enclosing_fn(&text, i)
                .unwrap_or_else(|| panic!("no enclosing fn for {rel}:{}", i + 1));
            call_sites.push((rel.clone(), i + 1, f));
        }
    }

    assert_eq!(
        definitions, 1,
        "expected exactly one `producer_ack` definition; found {definitions}. Two definitions \
         would make the wrapper set below unreadable."
    );

    let mut found: Vec<(String, String)> = call_sites
        .iter()
        .map(|(f, _, func)| (f.clone(), func.clone()))
        .collect();
    found.sort();
    found.dedup();

    let mut expected: Vec<(String, String)> = PERMITTED_ACK_WRAPPERS
        .iter()
        .map(|(f, n)| ((*f).to_string(), (*n).to_string()))
        .collect();
    expected.sort();

    assert_eq!(
        found,
        expected,
        "ACK CONFINEMENT IS AN IDENTITY CLAIM. The set of functions that reach \
         `transport::producer_ack` changed.\n  permitted: {expected:?}\n  measured:  {found:?}\n\
         Every call site, for the record:\n{}",
        call_sites
            .iter()
            .map(|(f, n, func)| format!("  {f}:{n}  in `{func}`"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
