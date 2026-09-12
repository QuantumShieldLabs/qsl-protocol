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

### First-time crossed invitations (draft)

When both peers redeem before connecting, the client preserves the outgoing attempt
and permits one provisional responder. Full pinned identity ordering chooses which
exchange to try; only existing B1/A2 authentication selects the session. Redeeming
after a responder has reserved its exchange coalesces into that exchange. The
identity guard, self-invitation rejection and unrelated stored sessions remain
protected. This draft changes local persistence, not handshake or envelope bytes.

Exact replies and selection intent live in the encrypted vault. Finish can resume
an unsent A2 after restart even when the local session already exists. Recovery
preserves advanced same-session ratchets and checks the generation, identity and
previous route before completing separate writes. After application, the recovery
capsule discards initial ratchet and candidate secrets; exact replies remain. Stored candidate routes match
the exact admitted envelope; the current wire does not cryptographically bind its
outer route. Existing storage guarantees apply, without a new power-loss or older
backup rollback claim.

The client permits at most 64 active attempts, one outgoing plus one responder per
binding, with 32 KiB per reply, 256 KiB serialized per record and 16 MiB total.
An attempt releases its active slot only after authenticated selection is applied
and required reply delivery is recorded. Replay records and exact replies remain
until full vault erase. Admission reserves each active record's full byte allowance;
retained history and JSON framing can make the byte limit refuse work before 64
active slots. Known exhaustion is checked before redeeming, then checked again at
local admission; remote redemption and local storage are not atomic. Older stores
remain readable and can recover under existing byte limits without increasing a
reservation shortfall. This does not promise unlimited connection history.

A reserved responder is not evicted by unauthenticated traffic or a timer. One
counterfeit admitted A1 arriving before the legitimate selected A1 can keep that
crossing blocked. The regression demonstrates a relay-response substitution with
a suppressed ACK, an ability available to a malicious relay or trusted transport
interceptor, not an ordinary network attacker through correctly validated TLS.
The added cost is persistent local blockage after that manipulation stops; no
identity possession or authenticated session is obtained. Other injection paths
require the relevant relay capability/access and are not proved by this regression.
This limitation remains open for independent security review. Cancellation UI,
session replacement, old-message draining and multi-device enrollment are excluded.

Independent review and Linux/macOS runtime acceptance remain separate gates. The
active desired-progress test requires the same authenticated SID at both peers and
actual production-crypto messages in both directions, with seed fallback disabled.
