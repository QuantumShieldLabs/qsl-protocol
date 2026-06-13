Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0470 qsc TUI Account Bootstrap Identity Provider RNG Failure Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only NA-0470 scope authorization lane for qsc TUI
account bootstrap identity provider RNG failure. The lane must consume D-0927
and NA-0469, classify the TUI bootstrap surface, select exactly one successor,
and avoid implementation mutation or public overclaim.

## Protected invariants

- Exactly one READY item remains before optional closeout.
- NA-0470 is READY at start.
- NA-0469 is DONE at start.
- D-0925, D-0926, and D-0927 exist once at start.
- D-0928 is absent before patch and exists once after patch.
- D-0929 remains absent before optional closeout.
- D-0927 assurance recovery is consumed.
- NA-0469 CLI rotation implementation evidence is consumed as bounded
  background only.
- TUI bootstrap identity provider RNG scope is classified without runtime,
  crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz, vector,
  formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli, website,
  public docs, README, START_HERE, qwork/qstart/qresume/qshell, backup,
  restore, qsl-backup, backup status, backup plan, rollback, or backup tree
  mutation.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No secret-material-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## Required classification checks

Required:

- primary classification is one of the directive's allowed classifications;
- selected classification is `TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE`;
- selected successor is `NA-0471 -- QSL qsc TUI Account Bootstrap
  Transactionality Design Authorization Plan`;
- assurance gap trigger classification is
  `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`;
- implementation is not selected because current TUI bootstrap writes durable
  account/default/bootstrap state before identity provider key generation.

## Required source inventory checks

Read-only inspection must record:

- TUI account setup/bootstrap command entry point in
  `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`;
- identity KEM keypair generation during bootstrap;
- identity signature keypair generation during bootstrap;
- default account/vault writes;
- selected identity write/update surface;
- self public-record write/update surface;
- default route/account config writes;
- verification seed/account probe surfaces;
- user-visible setup output/diagnostics;
- CLI rotation completed background;
- lazy identity closed background;
- legacy/public-record closed background;
- X25519 / ephemeral excluded background;
- refimpl provider RNG excluded background.

## Required timing checks

Read-only review must prove:

- TUI bootstrap requires no current vault for `/init`.
- `tui_try_vault_init` creates the vault before account defaults.
- `initialize_account_after_init` stores alias before defaults.
- `init_account_defaults_with_passphrase` writes account/default settings,
  verification seed, relay endpoint/token fields, and TUI relay inbox token
  before identity generation.
- `init_identity_with_passphrase` generates both KEM and signature keypairs.
- identity generation occurs before identity secret and self public-record
  writes, but after earlier account/default/bootstrap writes.
- simple forced identity provider RNG failure cannot prove no partial
  account/bootstrap state under the current order.

## Required stewardship and assurance checks

Evidence must include:

- Crypto / Protocol Steward review.
- CI / Dependency / Release Health Steward review.
- Public Claims / External Review Steward review.
- Product / Demo / Service Boundary Steward review.
- Local Ops / Backup / Restore Steward review.
- Best-Known-Method Review with `BEST_KNOWN_METHOD_FOR_SCOPE`.
- Hostile Cryptographer Review with top concerns.
- Red-Team Review with top concerns.
- Production SRE Review with top concerns.
- Side-Channel Caveat.
- Formal-Model Mapping Residual classification
  `FORMAL_MODEL_MAPPING_RESIDUAL_ACTIVE`.
- External-Review Readiness classification
  `EXTERNAL_REVIEW_READINESS_UNCHANGED`.
- Release-Claim Boundary.
- Assurance Gap Review Trigger classification
  `HIGHER_PRIORITY_RESIDUAL_SUPERSEDES_ASSURANCE_REVIEW`.

## Required markers

Evidence must record:

- `NA0470_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0470_D0927_CONSUMED_OK`
- `NA0470_NA0469_INHERITANCE_CONSUMED_OK`
- `NA0470_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0470_TUI_BOOTSTRAP_TARGET_INVENTORY_OK`
- `NA0470_TUI_BOOTSTRAP_STATE_TIMING_REVIEW_OK`
- `NA0470_TUI_BOOTSTRAP_REQUIRES_DESIGN_CHANGE_OK`
- `NA0470_SUCCESSOR_NA0471_TRANSACTIONALITY_DESIGN_SELECTED_OK`
- `NA0470_ASSURANCE_TRIGGER_HIGHER_PRIORITY_RESIDUAL_OK`
- `NA0470_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0470_NO_DEPENDENCY_CHANGE_OK`
- `NA0470_NO_WORKFLOW_CHANGE_OK`
- `NA0470_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0470_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0470_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0470_NO_IDENTITY_COMPLETE_CLAIM_OK`
- `NA0470_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0470_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0470_ONE_READY_INVARIANT_OK`

## Validation commands

Run:

```bash
git diff --check
```

Run scope guard:

```bash
python3 - <<'PY'
import subprocess, sys
allowed = {
    "docs/governance/evidence/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_plan.md",
    "tests/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_testplan.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
}
changed = set(subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines())
bad = sorted(changed - allowed)
print("CHANGED_PATHS", sorted(changed))
print("SCOPE_GUARD_BAD_COUNT", len(bad))
if bad:
    print("\n".join(bad))
    sys.exit(1)
PY
```

Run link check:

```bash
python3 - <<'PY'
import pathlib, re
repo = pathlib.Path(".").resolve()
md_files = sorted({p for p in repo.glob("**/*.md") if ".git/" not in p.as_posix() and p.is_file()})
link_re = re.compile(r'\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)')
missing = []
for md in md_files:
    text = md.read_text(encoding="utf-8", errors="replace")
    for raw in link_re.findall(text):
        target = raw.strip()
        if not target or "://" in target or target.startswith("mailto:"):
            continue
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        if not (md.parent / target).resolve().exists():
            missing.append((md.relative_to(repo).as_posix(), target))
for src, target in missing:
    print(f"MISSING_LINK {src} -> {target}")
print(f"TOTAL_MISSING {len(missing)}")
raise SystemExit(1 if missing else 0)
PY
```

Run inherited health checks:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Run dependency/formal/script checks:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Run PR body preflight and goal-lint before PR creation.

## Public claim boundary

This testplan is governance-only. It does not implement a TUI bootstrap seam.
It does not prove production readiness.
It does not prove public-internet readiness.
It does not prove external review completion.
It does not prove crypto completion.
It does not prove KEM completion.
It does not prove signature completion.
It does not prove identity completion.
It does not prove RNG failure completion.
It does not prove provider RNG completion.
It does not prove side-channel freedom.
It does not prove vulnerability freedom.
It does not prove bug freedom.
It does not prove perfect crypto.
It does not prove secret-material lifecycle completion.
Cargo audit green is dependency-health evidence only.
