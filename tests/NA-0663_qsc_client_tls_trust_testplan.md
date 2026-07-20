# NA-0663 test plan — qsc client TLS trust (D599, D-1286)

Scope: the qsc client's TLS trust behaviour only. Three parts, one concern:
the OS trust store honored, an explicit ADDITIVE CA-file option, and a
DISTINGUISHABLE typed certificate-trust outcome.

Test file: `qsl/qsl-client/qsc/tests/NA_0663_relay_tls_trust.rs` (new; the only
test file this lane adds or changes).

## THE HARD BOUNDARY

No insecure-skip-verify / accept-any-certificate / accept-invalid-certificate
option exists in any form, for any reason, INCLUDING in the tests. An explicit
CA path is the sanctioned escape; a blanket bypass is not. Family 4 pins this
as an executable test rather than a review promise.

## RIG (F3)

Certificates are generated at RUNTIME with `rcgen` and served over a loopback
`tokio-rustls` listener. NO key material is committed to the repository — the
added-line private-material scan is fail-closed and stays that way.

Per test, ephemerally:
- CA-A + a leaf carrying a `127.0.0.1` IP SAN — the trusted pair
- CA-B, installed nowhere — the untrusted pair

Every endpoint is `127.0.0.1` except one RFC-2606 `.invalid` name used solely
for the DNS-failure distinctness case, which is guaranteed never to resolve.
No test reaches an external relay, host, or network endpoint.

The trust seam is exercised per-Command (`SSL_CERT_FILE=…` on the spawned qsc
process), because the relay ops are `pub(super)` and are reached through the
CLI, not linked directly.

## FAMILIES

### Family 1 — a system-trusted CA is accepted (acceptance a)
`family1_system_trusted_ca_is_accepted_with_no_explicit_ca`
Ephemeral CA-A installed via `SSL_CERT_FILE`; NO explicit CA option set
anywhere. The push round-trip SUCCEEDS. Proves the OS trust store is honored
and that `SSL_CERT_FILE` behaves as standard runbooks describe.

### Family 2 — an explicit CA is accepted, and is purely additive (acceptance b)
- `family2_explicit_ca_accepted_via_env_ingress` — `QSC_RELAY_CA_FILE`
- `family2_explicit_ca_accepted_via_fallback_env_ingress` — `RELAY_CA_FILE`
- `family2_explicit_ca_accepted_via_cli_verb_ingress` — `relay ca-set` writes
  the vault secret; a later send with NO env set honors it. Also asserts
  `relay ca-show` reports `configured=true`, that `relay ca-clear` returns it
  to `false`, and that the raw path NEVER appears in either verb's output.
- `family2_explicit_ca_is_additive_and_never_narrows_existing_trust` — the
  ADDITIVITY PIN. With an explicit CA configured: (i) a plain-http loopback
  exchange still succeeds, and (ii) a 401 still yields `relay_unauthorized`
  and NOT a trust error. The option only ever adds trust.

No `SSL_CERT_FILE` is set in family 2: the explicit option alone must carry it.

### Family 3 — an untrusted certificate yields the typed outcome (acceptance c)
- `family3_untrusted_cert_yields_typed_outcome_not_opaque_failure` — the
  listener presents a leaf from CA-B. The op returns `relay_tls_untrusted`,
  and is asserted NOT to be `relay_inbox_push_failed`. This is the whole point
  of the lane: the opaque value no longer swallows the trust failure.
- `family3_trust_failure_is_distinct_from_refused_dns_and_auth` — a trust
  failure is distinct from connection-refused (closed loopback port), DNS
  failure (`.invalid`), and a 401 over a TRUSTED certificate.
- `family3_ca_config_failures_are_each_enumerated_and_fail_closed` — a
  configured CA that is missing / unreadable / not PEM yields
  `relay_ca_file_missing` / `_unreadable` / `_invalid` respectively, and each
  FAILS CLOSED. qsc never silently proceeds without the operator's CA.

### Family 4 — no bypass exists (acceptance d)
- `family4_no_certificate_bypass_exists_in_source_or_tests` — a needle scan
  PINNED AS A TEST across `src/`, `tests/` and `Cargo.toml`, asserting ZERO
  occurrences. The needles are assembled at RUNTIME from fragments so the
  literals never appear in the scanned tree, which is why the scan can include
  its own file with no exemption list.
- `family4_every_trust_knob_misset_still_refuses_an_untrusted_certificate` —
  the FAIL-CLOSED proof. With BOTH trust knobs deliberately mis-set (an
  unrelated explicit CA and an unrelated `SSL_CERT_FILE`), an untrusted
  listener STILL yields the typed refusal. No configuration accepts an
  unverifiable certificate.

### The pub library surface
`pub_library_surface_reports_presence_and_hash_without_the_path` — the
GUI-reachable accessor reports presence class + hash only, never the path.
Slice B's Server pane consumes this surface, not the CLI.

### Family 5 — byte-identity + baseline (acceptance, outside this file)
- every pre-existing test file BYTE-IDENTICAL to base, `NA_0640_full_stack_e2e.rs`
  included, verified by `git diff` against the base commit
- full `cargo test -p qsc` green, delta == exactly this new file
- base-binary A/B: the PRE-EXISTING relay verbs' output compared byte-for-byte
  between the Phase-0 stashed base binary and the head binary over a fixed
  no-network exercise set (success paths, error paths, locked-vault refusal,
  and adjacent `status`/`config get` surfaces)

### Family 6 — audits + lock delta
root `cargo audit` + nested qsc fuzz `cargo audit`; `cargo metadata --locked`;
the Cargo.lock delta enumerated line-by-line and explained.

## WHAT THESE TESTS DO NOT PROVE

- that any GUI exists (slice B owns the Server pane)
- that the inspiron LAN private-CA path was exercised live (slice-B acceptance
  owns the live-rig demo)
- that transport security improved beyond the stated trust change
- any claim about the external-review gate

Finer GUI wording over the typed value is slice B's to refine: rustls reports
the whole certificate-verification class (unknown issuer, name mismatch,
expiry) as `InvalidCertificate`, and the typed outcome covers that CLASS,
which IS "certificate not trusted" for taxonomy purposes.
