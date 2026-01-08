# Phase 4B Actor Contract (JSONL over stdio)

This contract allows the Phase 4B harness to drive two independent implementations (Impl-A, Impl-B) in a deterministic and auditable way.

## Transport

The harness spawns each actor as a process. Requests are written to the actor's stdin as UTF-8 JSON, one object per line.
Responses are read from stdout as UTF-8 JSON, one object per line.

- The actor MUST flush stdout after each response.
- The actor MUST NOT write non-JSON to stdout. Logs go to stderr.

## Request envelope

```json
{
  "id": "uuid-or-monotonic-string",
  "op": "capabilities|reset|handshake_init|handshake_respond|handshake_finish|encrypt|decrypt",
  "params": { "..." : "..." }
}
```

## Response envelope

Success:

```json
{ "id": "...", "ok": true, "result": { } }
```

Failure (fail-closed):

```json
{
  "id": "...",
  "ok": false,
  "error": { "code": "UNSUPPORTED|INVALID|CRYPTO|INTERNAL", "message": "..." }
}
```

The harness treats missing response, invalid JSON, or id mismatch as a hard failure.

## Minimum required ops

### `capabilities`
Returns actor metadata and supported suites.

Result shape:

```json
{
  "name": "impl_a",
  "suites": ["Suite-1", "Suite-1B"],
  "features": ["handshake", "encrypt", "decrypt"]
}
```

### `reset`
Resets actor state. For determinism, actors SHOULD accept a seed.

Params:
- `seed` (optional string)

### Handshake (3-step)
The harness assumes a 3-message handshake:

1) `handshake_init` (initiator) -> `msg1_b64`
2) `handshake_respond` (responder) consumes `msg1_b64` -> `msg2_b64`
3) `handshake_finish` (initiator) consumes `msg2_b64` -> `session_id`

Each op must accept `suite` (string) and an `options` map.

### `encrypt` / `decrypt`
Encrypt takes: `session_id`, `plaintext_b64`, optional `aad_b64`.
Returns: `ciphertext_b64`.

Decrypt takes: `session_id`, `ciphertext_b64`, optional `aad_b64`.
Returns: `plaintext_b64`.

The harness verifies decrypt(encrypt(m)) == m across actors.

## Compliance

This contract does not modify QSP/QSE; it is harness control-plane only.
