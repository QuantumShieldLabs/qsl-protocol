# NA-0195B advisory baseline evidence

Goals: G4, G5

This evidence note records the exact qsl-protocol advisory-baseline remediation
performed under `NA-0195B`.

Exact CI workflow command:

```bash
cargo install cargo-binstall --locked --version 1.10.0
cargo binstall cargo-audit --version 0.22.0 --no-confirm
cargo audit --deny warnings
```

Safe direct remediation applied in this item:

- `bytes` `1.11.0 -> 1.11.1`
- `quinn-proto` `0.11.13 -> 0.11.14`
- `keccak` `0.1.5 -> 0.1.6`

Residual advisories governed narrowly via `.cargo/audit.toml`:

- `RUSTSEC-2024-0388` (`derivative`)
- `RUSTSEC-2024-0384` (`instant`)
- `RUSTSEC-2024-0436` (`paste`)
- `RUSTSEC-2025-0144` (`ml-dsa`)
- `RUSTSEC-2024-0380` (`pqcrypto-dilithium`)
- `RUSTSEC-2024-0381` (`pqcrypto-kyber`)
- `RUSTSEC-2026-0002` (`lru`)

Constraints:

- No qsl-server changes.
- No workflow changes.
- No attachment work.
- No hidden protocol, wire, or crypto semantic drift.
- NA-0195A route-token migration behavior remains intact.
