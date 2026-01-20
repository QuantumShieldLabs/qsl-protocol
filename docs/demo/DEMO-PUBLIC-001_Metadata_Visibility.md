# DEMO-PUBLIC-001 — Metadata Visibility Demo (qsl-tui)

Goals: G4

## What this demo proves (in plain terms)

- The message **content** is encrypted in the client before it goes to the network.
- The relay can still observe **metadata** like timing, packet sizes, and the channel identifier.
- This demo shows a **client-layer mitigation**: size-padding into buckets, so ciphertext sizes reveal less.

## Quick start (headless, single command)

From repo root:

```bash
# Build + run the headless demo (prints plaintext_len / ciphertext_len / bucket)
cargo run -p qsl-tui --release -- \
  demo --privacy basic --mode local --message "hello"
```

## Relay mode (explicit opt-in required)

```bash
QSL_ALLOW_REMOTE=1 cargo run -p qsl-tui --release -- \
  demo --privacy padded --mode relay \
  --relay-base-url http://qsl.ddnsfree.com:8080 \
  --relay-channel demo-$(date -u +%Y%m%dT%H%M%SZ)
```

## What the output means

You should see lines like:

- `QSL_TUI_META plaintext_len=5 ciphertext_len=256 bucket=256 mode=padded`
- `QSL_TUI_HEADLESS_OK plaintext=hello`

These show:
- **plaintext_len**: length of your original message.
- **ciphertext_len**: length of the encrypted payload actually sent.
- **bucket**: the padding bucket chosen (only in padded mode).
- **mode**: `basic` (no padding) or `padded` (bucketed padding).

## Metadata reality (explicit and honest)

Even with padding:
- **Visible** to relay/network: timing, channel ID, and coarse size patterns.
- **Hidden** from relay/network: message contents and keys.
- **Mitigated**: size correlation (padding buckets reduce exact size leakage).
- **Not mitigated**: IP address or timing correlation (use a proxy/Tor for IP, or add traffic shaping for timing).

## Evidence (paste-ready)

If you need a log bundle, run the helper script:

```bash
scripts/demo/demo_public_metadata_visibility.sh
```

It will write all outputs under `_forensics/` and print the directory path.
