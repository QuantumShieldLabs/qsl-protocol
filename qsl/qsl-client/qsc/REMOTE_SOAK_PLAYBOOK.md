# QSC Remote Soak Playbook (NA-0165)

## One-Command Run (100 Clients)

Never paste relay tokens into shell history. Use an environment variable:

```bash
export QSL_RELAY_TOKEN='<set-via-secret-store>'
python3 qsl/qsl-client/qsc/scripts/remote_soak.py \
  --relay-url https://relay.example.com \
  --clients 100 \
  --duration-secs 60
```

`remote_soak.py` now auto-detects `qsc` in this order when `--qsc-bin` is omitted:
1. `target/release/qsc`
2. `target/debug/qsc`
3. `qsc` from `PATH`

Override explicitly when needed:

```bash
python3 qsl/qsl-client/qsc/scripts/remote_soak.py \
  --relay-url https://relay.example.com \
  --qsc-bin target/release/qsc
```

Dry-run (no network):

```bash
python3 qsl/qsl-client/qsc/scripts/remote_soak.py \
  --relay-url https://relay.example.com \
  --dry-run
```

## AWS-Side Checks

1. `sudo systemctl status qsl-server --no-pager`
2. `sudo ss -lntp | rg ':(443|80|8080)'`
3. `sudo /opt/qsl-server/scripts/qsl_relay_audit.sh` (if installed)
4. Validate Caddy `/v1/*` log hygiene is active (no token-like paths in access logs).

## Common Failure Signatures

### 401 Unauthorized
- Check `RELAY_TOKEN` on server env and `QSL_RELAY_TOKEN` in harness env.
- Rotate token if exposure is suspected.

### TLS Required / Endpoint Rejected
- Verify relay URL is `https://...`.
- Confirm reverse proxy certificate chain is valid for the hostname.

### Pull/Push Timeouts
- Check relay process health and queue depth/backpressure logs.
- Verify security groups/firewall: public 443/80 only, no public 8080.

### Route/Inbox Parse or Missing Route Markers
- Re-run client route setup flow for impacted client pair.
- Confirm no stale client state dirs are reused between distinct runs.

### `unsafe_parent_perms`
- Use a safe state root (`0700`) via `--state-root <dir>` or rely on the default auto root under `~/.qsl/qsc-soak/...`.
- Do not place soak state under group/world-writable parents.

## Triage Sequence (Deterministic)

1. Run dry-run first and confirm `QSC_SOAK_DRYRUN_OK`.
2. Run live soak with bounded duration.
3. Inspect `summary.json` for reject code distribution.
4. Resolve top reject code first, then re-run the same command.
