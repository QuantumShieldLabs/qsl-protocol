#!/usr/bin/env bash
# NA-0632 (ENG-0038) reproducible probe — corroborates that the responder's signing-key pin
# (`sig_fp`) is never populated by any shipped qsc contact/handshake path, so the initiator's
# OPTIONAL sig-pin check (handshake/mod.rs hs_check_optional_identity_pin) is always skipped.
# Read-only; runs from the repo root. Not a substitute for a running PoC (see the testplan).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
SRC=qsl/qsl-client/qsc/src

echo "== 1. Every sig_fp assignment in shipped contact provisioning is None =="
grep -nE 'sig_fp *: *None' "$SRC/contacts/mod.rs" || true
echo "   (contacts_add + contacts_device_add both set sig_fp: None, incl. the verify=true 'verified' path)"

echo
echo "== 2. No shipped path writes a real (Some) CONTACT sig_fp =="
echo "   (excluding pending.peer_sig_fp — the transient in-handshake field, never persisted to a contact)"
# Match a field literally named sig_fp (contact/device record), not peer_sig_fp (pending handshake).
if grep -rnE '(^|[^_[:alnum:]])sig_fp *: *Some|(^|[^_[:alnum:]])sig_fp *= *Some' \
     "$SRC/contacts" "$SRC/handshake" "$SRC/identity"; then
  echo "   !! unexpected: a Some(sig_fp) contact writer exists — re-evaluate ENG-0038" ; exit 1
else
  echo "   OK: no Some(sig_fp) contact writer (the only Some is pending.peer_sig_fp, transient, not a contact pin)"
fi

echo
echo "== 3. The optional sig-pin check returns Ok on a missing pin (skips) =="
grep -nA1 'Ok(None) => Ok(())' "$SRC/handshake/mod.rs" | head -4

echo
echo "== 4. The initiator's primary (KEM) pin check is fed pending.peer_fp, itself = identity_read_pin =="
grep -nE 'peer_fp = match identity_read_pin|peer_fp: Some\(peer_fp\)|hs_require_primary_identity_pin\(peer, peer_fp' "$SRC/handshake/mod.rs" | head

echo
echo "== 5. The responder sends NO KEM public key in B1 (only kem_ct) — nothing to pin the responder's KEM identity to =="
grep -nE 'struct HsResp' "$SRC/handshake/mod.rs"
sed -n '/struct HsResp/,/^}/p' "$SRC/handshake/mod.rs" | grep -nE 'kem_ct|kem_pk|sig_pk' || true

echo
echo "== 6. The only place a real sig_fp is set is a TEST that hand-injects it via JSON =="
grep -rnE 'fn set_contact_sig_pin' qsl/qsl-client/qsc/tests/ || true
echo "   (its existence proves no product path sets sig_fp; see NA-0632_adversarial_reanalysis.md §2.3)"

echo
echo "PROBE COMPLETE — findings consistent with ENG-0038 (responder signing key is never pinned on the shipped path)."
