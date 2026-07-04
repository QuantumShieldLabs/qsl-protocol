# NA-0603 LAN Minimal qsc E2EE Relay Verification Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-04

Goals: G1, G2, G3, G4, G5

## Result

NA-0603 classification:
`LAN_TINY_QSC_LAPTOP_READINESS_GAP`.

The build-server qsl-server side reached ready class and auth-negative
fail-closed class. Operator laptop proof reached qsc sender/receiver readiness,
TCP connect success, and qsl-server route shape ready, but qsc rejected the
private-LAN HTTP relay endpoint with TLS-required policy. Tiny send was
classified `tls_required_gap`; receive/decrypt/validate was `not_reached`.

## Required Markers

- NA0603_D1195_LAN_READINESS_CONSUMED_OK
- NA0603_D1196_CLOSEOUT_CONSUMED_OK
- NA0603_FRESH_QWORK_PROOF_OK
- NA0603_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0603_QSL_SERVER_VALIDATION_OK
- NA0603_PRIVATE_LAN_BIND_CLASSIFIED_OK
- NA0603_QSL_SERVER_STARTUP_CLASSIFIED_OK
- NA0603_OPERATOR_COMMAND_PACKET_CREATED_OK
- NA0603_LAPTOP_PROOF_CLASSIFIED_OK
- NA0603_TINY_SEND_CLASSIFIED_OK
- NA0603_TINY_RECEIVE_DECRYPT_VALIDATE_CLASSIFIED_OK
- NA0603_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK
- NA0603_SEED_FALLBACK_CLASSIFIED_OK
- NA0603_METADATA_REVIEW_OK
- NA0603_CLEANUP_DONE_OK
- NA0603_HOSTILE_ANALYST_METADATA_ROADMAP_RECORDED_OK
- NA0603_PRIVATE_MATERIAL_SCAN_OK
- NA0603_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0603_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0603_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0603_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0603_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0603_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0603_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0603_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0603_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0603_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0603_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0603_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0603_NO_PUBLIC_READINESS_CLAIM_OK
- NA0603_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0603_NO_REMOTE_READY_CLAIM_OK
- NA0603_NO_TAILNET_READY_CLAIM_OK
- NA0603_NO_LAN_READY_OVERCLAIM_OK
- NA0603_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0603_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0603_RESULT_CLASSIFICATION_SELECTED_OK
- NA0603_SUCCESSOR_SELECTED_OK
- NA0603_ONE_READY_INVARIANT_OK

## Validation Plan

Run before PR:

```bash
git diff --check
git diff --name-only
git diff --cached --name-only
git ls-files --others --exclude-standard
python3 - <<'PY'
from pathlib import Path
required = [
    'NA0603_D1195_LAN_READINESS_CONSUMED_OK',
    'NA0603_D1196_CLOSEOUT_CONSUMED_OK',
    'NA0603_FRESH_QWORK_PROOF_OK',
    'NA0603_CURRENT_MAIN_CHECKS_CLASSIFIED_OK',
    'NA0603_QSL_SERVER_VALIDATION_OK',
    'NA0603_PRIVATE_LAN_BIND_CLASSIFIED_OK',
    'NA0603_QSL_SERVER_STARTUP_CLASSIFIED_OK',
    'NA0603_OPERATOR_COMMAND_PACKET_CREATED_OK',
    'NA0603_LAPTOP_PROOF_CLASSIFIED_OK',
    'NA0603_TINY_SEND_CLASSIFIED_OK',
    'NA0603_TINY_RECEIVE_DECRYPT_VALIDATE_CLASSIFIED_OK',
    'NA0603_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK',
    'NA0603_SEED_FALLBACK_CLASSIFIED_OK',
    'NA0603_METADATA_REVIEW_OK',
    'NA0603_CLEANUP_DONE_OK',
    'NA0603_HOSTILE_ANALYST_METADATA_ROADMAP_RECORDED_OK',
    'NA0603_PRIVATE_MATERIAL_SCAN_OK',
    'NA0603_NO_ENDPOINT_VALUE_PUBLISHED_OK',
    'NA0603_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK',
    'NA0603_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK',
    'NA0603_NO_CAPABILITY_VALUE_PUBLISHED_OK',
    'NA0603_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK',
    'NA0603_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK',
    'NA0603_NO_QSL_ATTACHMENTS_RUNTIME_OK',
    'NA0603_NO_CODEX_SSH_TO_LAPTOP_OK',
    'NA0603_NO_LAPTOP_SSH_SERVER_SETUP_OK',
    'NA0603_NO_SECOND_CODEX_ON_LAPTOP_OK',
    'NA0603_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK',
    'NA0603_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK',
    'NA0603_NO_PUBLIC_READINESS_CLAIM_OK',
    'NA0603_NO_PRODUCTION_READINESS_CLAIM_OK',
    'NA0603_NO_REMOTE_READY_CLAIM_OK',
    'NA0603_NO_TAILNET_READY_CLAIM_OK',
    'NA0603_NO_LAN_READY_OVERCLAIM_OK',
    'NA0603_NO_CRYPTO_COMPLETE_CLAIM_OK',
    'NA0603_NO_ATTACHMENT_COMPLETE_CLAIM_OK',
    'NA0603_RESULT_CLASSIFICATION_SELECTED_OK',
    'NA0603_SUCCESSOR_SELECTED_OK',
    'NA0603_ONE_READY_INVARIANT_OK',
]
text = Path('docs/governance/evidence/NA-0603_lan_minimal_qsc_e2ee_relay_verification_harness.md').read_text()
text += Path('tests/NA-0603_lan_minimal_qsc_e2ee_relay_verification_testplan.md').read_text()
missing = [m for m in required if m not in text]
print('MISSING_MARKERS', len(missing))
for marker in missing:
    print(marker)
raise SystemExit(1 if missing else 0)
PY
cargo audit --deny warnings
cd qsl/qsl-client/qsc/fuzz && cargo audit --deny warnings
cd -
cargo metadata --locked --format-version=1 >/tmp/na0603_cargo_metadata.json
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run qsl-server validation in its NA-0603 workspace when available:

```bash
cargo metadata --locked --format-version=1
cargo audit --deny warnings
cargo fmt --check
cargo test --locked
cargo build --locked
```

## Boundary Checks

The only intended qsl-protocol changed paths for implementation evidence are:

- `docs/governance/evidence/NA-0603_lan_minimal_qsc_e2ee_relay_verification_harness.md`
- `tests/NA-0603_lan_minimal_qsc_e2ee_relay_verification_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No source, workflow, dependency, lockfile, public-site, qsc, qsl-server,
qsl-attachments, qwork, qstart, qresume, qshield, formal, refimpl, backup, or
deployment paths are in scope for this governance patch.
