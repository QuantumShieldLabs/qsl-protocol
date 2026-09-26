Goals: G4, G5

Status: Supporting
Owner: QSC maintainers
Last-Updated: 2026-03-29

# QSC Front Door

Use this directory as the qbuild-first, AWS-free operator front door for `qsc`.

## Validated baseline

- Build from a fresh local checkout (qbuild-scoped when applicable):
  `cargo build -p qsc --release --locked`
- Use [LOCAL_TWO_CLIENT_RUNBOOK.md](./LOCAL_TWO_CLIENT_RUNBOOK.md) for the current truthful
  end-to-end baseline.
- That baseline is the one aligned to current TUI/operator behavior, route-token header carriage,
  and the validated post-`w0` migration posture.

## Compatibility surfaces

- [REMOTE_TWO_CLIENT_AWS_RUNBOOK.md](./REMOTE_TWO_CLIENT_AWS_RUNBOOK.md) is compatibility-only
  evidence for a non-baseline remote lane.
- [REMOTE_SOAK_PLAYBOOK.md](./REMOTE_SOAK_PLAYBOOK.md) is compatibility-only operational evidence.
- `docs/qsc/DOC-QSC-003_*` and `docs/qsc/DOC-QSC-004_*` remain supporting packaging/demo context,
  but they are not the operator front door.

## Product-surface guardrails

- Do not paste route tokens, bearer tokens, passphrases, or other secrets into logs.
- Canonical relay examples use `X-QSL-Route-Token` headers and token-free `/v1/pull?max=N` paths.
- When `QSC_ATTACHMENT_SERVICE` is set, the validated post-`w0` lane uses `w2` for new
  `<= 4 MiB` sends and defaults legacy receive handling to retired.

### Local self-invitation guard (NA-0780)

Desktop Connect can call `qsc::facade::invite_preflight(code, self_label)` before
redemption. On `FacadeError::SelfInvitation` (`self_invitation`), show:
“This invitation was created by this app. Ask the other person for their invitation.”
The redemption handler repeats the same ownership check before saving redemption
state, contacting the relay, or creating a pending contact. Direct engine callers
receive `invite_self`. Do not cache preflight success across edits or lock changes.

Ownership is retained separately from the visible invitation list in the encrypted
vault: a versioned set of minted IDs and public identity commitments, with no
invite blobs, capabilities, private keys, labels, endpoints or timestamps. Minting
persists ownership before its first network attempt and before exporting a code.
Unlock, clear/revoke and rotation seed recoverable existing records and public
identities before discard. Clear/revoke, restart and identity rotation/deletion
within the same vault preserve recognition. Full vault erase removes the history.
Independent public identity files surviving a vault-only erase can still identify
invitations belonging to those keys; they are not erased by this change.

Both preflight and redemption use the same read-only ownership predicate. They
create or migrate no keys and do not bootstrap history. Locked or corrupt ownership
storage fails closed; `invite_ownership_unavailable` maps to `store_unavailable`.
Failed retention aborts mint/clear/rotation; post-authentication retention failures
leave the app locked without counting a correct password as a failed attempt.
Stale vault sessions preserve the latest ownership record. No history is emitted
outside the vault; it grows with minted IDs and identities until vault erase.

Information already deleted before this change cannot be recovered. An old code
whose mint record and original public identity were both previously removed may
remain unrecognized. KEM-only legacy public records cannot reconstruct a missing
signing-key commitment; recoverable mint IDs are still retained. Restoring an old
vault backup also restores its older history; no rollback-detection redesign is
included.

Preflight success means only that available local data did not identify a self
invitation. It does not authenticate the code, check relay availability, or replace
redemption's expiry, commitment, signature and single-use checks. The existing
single-identity label resolution still applies. Desktop integration and observed
two-device acceptance remain separate work.

### Existing identity bindings (NA-0780)

An invitation cannot replace the identity bound to an existing contact alias.
Redeem and accept return `FacadeError::IdentityChanged` (`identity_changed`) when
full identity, signing or primary-device bindings disagree or are incomplete.
Contact comparison does not migrate or rewrite the existing record. Matching
contacts retain their verification, block, device, route and display metadata.
A matching alias with any stored session returns `FacadeError::SessionExists`
(`session_exists`); replacing or reconnecting an established session is separate
work. Unreadable storage returns `store_unavailable`, never an empty contact.

These checks run before provisioning and the subsequent handshake operation.
Redemption/pull may already have happened to obtain the public identity bundle;
a rejected redemption does not make the consumed invitation reusable. Existing
expiry, signature, commitment, self-invitation and possession checks remain in
force. No collision election, cancellation or device enrollment is implemented.

### Directional development profile (NA-0780 candidate)

This checkout implements one directional development profile,
`NA0780-DIR-INTEGRATION-03`. Its vault uses the `QSCV03` envelope with payload
version 4. The profile is selected explicitly, in an empty private config
directory, with `qsc vault init --protocol directional-v1`. Default vault
initialization refuses. Existing development state is preserved and refused;
there is no reset, migration or legacy interoperability path.

These identifiers (the profile `NA0780-DIR-INTEGRATION-03`, the `QSCV03` envelope,
payload version 4 and the `directional-v1` selector) are RETIRED for the successor
(C01 APPENDIX A row A21). They are replaced when the successor identity lands
(DOC-CAN-003 sec 12.9; its placement is ruled at F04's formalization). See
DOC-CAN-003 sec 12
(`docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`) and
`docs/ops/contracts/C01_versions_and_boundaries.md` at the repository root.

The disabled-by-default `na0780-test-hooks` Cargo feature enables local acceptance
instrumentation. It must not be enabled in distributed builds.

This is a draft, not release acceptance. Actual relay and macOS gates, independent
security review and the formal verification work remain outstanding. A format
version guard does not detect restoring an older complete vault backup; no new
power-loss guarantee is claimed. PQ recovery requires an unexposed honest target
and delivery of the corresponding event. File sends still refuse.

#### Message padding (development profile)

Message padding resolves once at enqueue. Auto (also the absent explicit choice)
uses saved `policy-profile`: baseline selects Standard (1024-byte floor), strict
selects Private (4096). A genuinely absent policy uses baseline; malformed or
unknown saved policy refuses, including with an explicit override. Enhanced (2048)
is explicit. Concrete `--pad-bucket` overrides a valid saved policy.

`--pad-to` specifies the exact total inner plaintext body size, including the
header, operation ID, typed payload and space reserved for all three closures.
The reservation is 16 + ID bytes + 135 + payload bytes. It must fit the profile
floor, saved maximum (default 4096, hard ceiling 65536) and tighter 60000-byte body
ceiling. Automatic sizing selects the smallest fitting power of two; impossible
requests refuse rather than clamp, grow or truncate. Padding reduces effective
payload capacity. Closure space unused at packing becomes zero padding. Only
same-frame-class lengths are comparable; boundary and receipt overhead still differ.

Typed maintenance bodies currently use Standard padding (1024 bytes), independently
of the saved application profile. Thus same-class control traffic can be visibly
smaller than Enhanced or Private application traffic; this is not control/application
length uniformity. That privacy versus
bandwidth tradeoff needs an explicit policy decision before release; Stage A does not
silently change maintenance sizing or already queued work.
Core advertisements bypass this typed-body padding and retain their existing encoding.
Explicit per-operation profiles and queued work from older policy settings also mean
that an account's traffic need not have one application size.


### NA-0780 directional option and receipt contract (draft implementation)

Goals: G4. The fixed first-release directional profile has no optional padding or
metadata seed/bucketing support. Explicit send padding, bucket and seed requests
refuse before payload reads, queue writes or network effects. Absence selects
fixed framing, not the old implicit Standard padding profile. Directional exact
receipts are mandatory: default/Delivered and explicit Immediate are supported;
Off/Batched, batch-window and jitter overrides refuse. Explicit saved receipt
policy values must be supported or operations refuse without clearing them.
Absent saved policy does not inherit the old Batched default.

Receive pacing options retain their existing bounded scheduler behavior. Explicit
legacy coexistence, attachment service/file limits/file-confirm options, receive
bucketing and metadata seed refuse before output/store/pull effects. Retired
selects the only supported non-legacy receive path. Fixed wire, byte, record,
context, skip and event limits are unchanged. No file option replaces those
limits. File-send explicitly refuses before reading/staging/uploading because
this slice has no directional attachment descriptor/assembly consumer; attachment
storage/cryptography are not rewritten or reset.

Message delivery is handled by durable directional receipts and Delivered
projection; obsolete Message/FileComplete internal receipt producers/variants and
unused metadata plumbing are removed. The attachment completion arm is retained
with its consumer, but attachment operations remain explicitly unsupported in
this slice. No legacy crypto is restored. QueueFull and RetryExhausted name the
existing failure results; callback unit errors, capacities, attempts (including
zero), backoff/jitter and CLI failure codes are preserved. No release readiness,
macOS/relay acceptance or external independent review is claimed.


NA-0780 local integration acceptance (2026-09-14, Goals: G4): both receive
orderings and 56 application rounds completed with 2,112 independently verified
phase records, exit zero, in 2h43m26s. The backpressure-aware fixture retains the
same queued operation through bounded normal receive/retry and requires durable
Delivered plus the original exact delivery counts. Profile/authentication,
retirement and process-cut/stale-generation assertions also completed. This is
Linux local mock-relay evidence; required new-head CI, focused independent review
and actual relay/macOS acceptance remain separate gates.

Attachment and padding support remain unfinished first-release work. Their
explicit refusals are interim limitations, not removal from the roadmap.
Conditional PQ recovery, older-complete-backup rollback exposure and unproven
power-loss guarantees remain as stated in "Directional development profile (NA-0780 candidate)" above; no default activation or release.

