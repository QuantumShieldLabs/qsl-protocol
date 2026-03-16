# NA-0197CA Attachment Encryption Context Contract Evidence
Goals: G4, G5

## Purpose

This evidence file records why `NA-0197CA` was required, why the chosen encryption-context strategy is the smallest faithful clarification, and how the canonical-document edits stay within docs/governance scope only.

## Source inputs used

- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`

## Blocker summary before NA-0197CA

The pre-clarification state proved all of the following:
- `DOC-ATT-001` already required qsc to derive/load an attachment encryption context for upload and to obtain/derive the matching decrypt context for download.
- `DOC-CAN-005` froze the attachment descriptor field set, peer-visible/local-only/service-only split, and transcript-bound compare set, but it did not define how the receiver obtains or derives the decrypt context.
- `DOC-CAN-006` froze the opaque service/session/object contract and kept `resume_token` / `fetch_capability` out of canonical URLs, but it did not supply any peer decryption material beyond service-plane secrets.
- Therefore `NA-0197C` could not honestly implement sender encryption and receiver decryption without inventing semantics.

## Canonical doc-ID proof

The following pre-create search returned no matches on live `main` before the doc was added:

```text
rg -n "DOC-CAN-007|NA-0197CA_encryption_context_contract_evidence|Attachment Encryption Context" docs/canonical tests DECISIONS.md TRACEABILITY.md NEXT_ACTIONS.md
```

Result: no matches.

## Candidate strategy comparison

| Candidate | Summary | Result | Why |
|---|---|---|---|
| `C0` | leave the docs as-is and let qsc infer decrypt context | reject | would force `NA-0197C` to invent protocol semantics locally |
| `C1` | sender-generated per-attachment content-encryption context carried only inside the authenticated descriptor | choose | keeps qsl-attachments opaque, avoids canonical-URL secret leakage, and adds the smallest control-plane change needed for sender/receiver symmetry |
| `C2` | derive attachment context from existing session/shared-secret state plus transcript inputs | reject | couples attachment encryption to ratchet/session semantics, complicates restart/resume, and is a larger semantic change than the current contracts need |
| `C3` | service-assisted or service-stored decrypt context | reject | conflicts with the opaque/plaintext-free attachment-plane posture and would require runtime/service redesign |

## Why `C1` is the smallest faithful fix

`C1` keeps the split chosen by `NA-0197` intact:
- qsl-protocol canonical docs define the peer-visible attachment contract,
- qsl-attachments remains a blind ciphertext/object service,
- qsl-server remains transport-only,
- and qsc can later implement upload/download without guessing how decrypt context moves between sender and receiver.

The live qsc tree already carries `ChaCha20Poly1305` and secure random generation, so the canonical v1 clarification uses a per-part `ChaCha20Poly1305` attachment cipher instead of introducing a new primitive family unnecessarily.

## Canonical-document changes made by NA-0197CA

- Created `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
- Amended `DOC-CAN-005` to:
  - add `enc_ctx_alg` and `enc_ctx_b64u` to the descriptor field set,
  - freeze the peer-visible/local-only/service-only split for encryption-context material,
  - freeze the transcript-bound compare rules and reject matrix updates,
  - and patch the safe descriptor example plus source-of-truth mapping.
- Amended `DOC-CAN-006` to:
  - point service/runtime work at `DOC-CAN-007` for attachment encryption-context and part-cipher semantics,
  - and explicitly keep `enc_ctx_*` off the service plane.

## Scope proof

This item is docs/governance only.

No runtime changes were made to:
- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**` product/runtime paths
- `qsl-attachments/**`
- `qsl-server/**`
- workflows
- website files
- `.github/**`

## Source-of-truth linkage

`TRACEABILITY.md` and `DECISIONS.md` are updated to point to:
- `DOC-CAN-007`
- this evidence artifact
- and the `NA-0197CA` queue progression

## Outcome test for closeout path

If these edits are merged unchanged, `NA-0197CA` may truthfully close on path `P1` because:
- sender generation, receiver acquisition, field split, service exclusion, and decrypt-order semantics are now explicit,
- qsl-attachments runtime need not change,
- and `NA-0197C` can proceed without semantic guesswork.
