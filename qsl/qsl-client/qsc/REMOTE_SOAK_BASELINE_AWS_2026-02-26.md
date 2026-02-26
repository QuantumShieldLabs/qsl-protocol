# Remote Soak Baseline (AWS) — 2026-02-26

- UTC window: 2026-02-26T03:20Z to 2026-02-26T03:32Z
- Relay URL: `https://qsl.ddnsfree.com`
- Server binary version: qsl-server `v0.0.3`
- Server binary SHA256: `01412802bf094a821ca7fd5852adcaee04b164780ec5df3f4926c72c18169e25`

## Outcomes

Warm-up (10 clients, 30s):
- `QSC_SOAK_STATE_ROOT_OK mode=700 parent_safe=yes`
- `QSC_SOAK_SUMMARY path=/home/victor/.qsl-soakroots/qsc-soak-cWRLRG/work/summary.json`
- `QSC_SOAK_RESULT FAIL code=qsp_hdr_auth_failed,vault_exists`

Baseline (100 clients, 120s):
- `QSC_SOAK_STATE_ROOT_OK mode=700 parent_safe=yes`
- `QSC_SOAK_SUMMARY path=/home/victor/.qsl-soakroots/qsc-soak-cWRLRG/work/summary.json`
- `QSC_SOAK_RESULT FAIL code=qsp_hdr_auth_failed,vault_exists`

Summary artifact:
- path: `/home/victor/.qsl-soakroots/qsc-soak-cWRLRG/work/summary.json`
- size: `312` bytes

## Server Observability (safe counters)

Pre-snapshot:
- qsl-server RSS/CPU line: `6715 qsl-server 0.0 4464 ... /opt/qsl-server/bin/qsl-server`
- caddy RSS/CPU line: `509 caddy 0.0 45220 ... /usr/bin/caddy run ...`
- established conn count (443/8080): `0`
- journald `/v1/` counts (10m): caddy `0`, qsl-server `0`

Post-snapshot:
- qsl-server RSS/CPU line: `6715 qsl-server 0.0 4548 ... /opt/qsl-server/bin/qsl-server`
- caddy RSS/CPU line: `509 caddy 0.0 48844 ... /usr/bin/caddy run ...`
- established conn count (443/8080): `1`
- qsl-server error-ish count (10m): `27`
- journald `/v1/` counts (10m): caddy `0`, qsl-server `0`

## Leak-Scan Proofs (local harness logs)

- `/tmp/qsl_soak_10c.log`: `/v1/` count `0`; long-hex (`[0-9a-f]{32,}`) count `0`
- `/tmp/qsl_soak_100c.log`: `/v1/` count `0`; long-hex (`[0-9a-f]{32,}`) count `0`

No token values, Authorization headers, or raw `/v1/<token>` URIs were emitted in this report.
