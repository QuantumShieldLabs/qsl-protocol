# qsl-tui (Linux TUI demo client)

This is a Linux TUI demo client intended to exercise existing QSL protocol behavior.
The relay server is transport-only and forwards **opaque bytes**; it does not encrypt or interpret protocol messages.

## Modes

- Local (default): run an in-process A/B demo using existing protocol code.
- Relay (optional): push/pull encrypted packets through a transport-only relay.

## Environment

- `QSL_RELAY_BASE_URL` (default: `http://127.0.0.1:8080`)
- `QSL_RELAY_CHANNEL` (default: `demo`)
- `QSL_ALLOW_REMOTE=1` required for non-localhost relay URLs

## Run

Local demo:

- `cargo run -p qsl-tui -- --mode local`

Headless demo (non-interactive shells/CI-safe):

- `cargo run -p qsl-tui -- --headless --mode local`
- `QSL_ALLOW_REMOTE=1 QSL_RELAY_BASE_URL=http://qsl.ddnsfree.com:8080 \
   cargo run -p qsl-tui -- --headless --mode relay --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo`

Relay demo (opt-in required; needs real TTY/PTY):

- `QSL_ALLOW_REMOTE=1 QSL_RELAY_BASE_URL=http://qsl.ddnsfree.com:8080 \
   cargo run -p qsl-tui -- --mode relay --relay-base-url http://qsl.ddnsfree.com:8080 --relay-channel demo`
