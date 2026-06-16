Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16
Replaces:
Superseded-By:

# NA-0487 qsc Binding Fuzz Source-Boundary Recovery Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0487 recovery authorization lane. The lane must consume D347,
inventory the current qsc source boundary, review recovery options, select an
exact future scope, preserve NA-0487 as the only READY item, and avoid all
implementation mutation.

## Protected invariants

- qwork proof files are read, not regenerated.
- exactly one READY item remains.
- READY remains NA-0487.
- NA-0486 remains DONE.
- D347 stop evidence is consumed.
- D-0962 is added once.
- D-0963 remains absent.
- no duplicate decision IDs exist.
- no qsc source mutation occurs in this recovery directive.
- no fuzz target, qsc fuzz Cargo, script, workflow, dependency, lockfile,
  vector/input, corpus, formal, refimpl, service, public-doc, backup, restore,
  or qsl-backup mutation occurs.
- NA-0487 is not marked DONE.
- NA-0488 is not restored.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no vector-complete claim is introduced.
- no KEM-complete claim is introduced.
- no signature-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_plan.md`
- this testplan
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- `qsl/qsl-client/qsc/src/**` mutation in this recovery directive;
- `qsl/qsl-client/qsc/fuzz/**` mutation in this recovery directive;
- `scripts/ci/qsc_adversarial.sh` mutation in this recovery directive;
- `.github/workflows/**` mutation;
- dependency or lockfile mutation;
- vector/input/corpus mutation;
- formal mutation;
- refimpl mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli mutation;
- public docs, website, README, START_HERE mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, and backup
  tree mutation.

## Startup validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
```

Required:

- qwork proof files exist and report startup OK for NA-0487;
- proof HEAD and proof `origin/main` match live pre-fetch refs;
- PR #1243 is merged at merge commit prefix `9a321cee5924`;
- `origin/main` equals or descends from `9a321cee5924`;
- READY_COUNT 1;
- READY NA-0487;
- NA-0486 DONE;
- D-0960 exists once;
- D-0961 exists once;
- D-0962 absent at startup;
- duplicate decision count 0;
- D347 response exists;
- public-safety success on current `origin/main`;
- root cargo audit green;
- nested qsc fuzz lock audit green.

## Source-boundary inventory validation

Run read-only probes:

```bash
sed -n '1,220p' qsl/qsl-client/qsc/src/lib.rs
sed -n '1,260p' qsl/qsl-client/qsc/src/main.rs
sed -n '1,240p' qsl/qsl-client/qsc/src/adversarial/mod.rs
find qsl/qsl-client/qsc/src/adversarial -maxdepth 2 -type f | sort
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/Cargo.toml
find qsl/qsl-client/qsc/fuzz/fuzz_targets -maxdepth 1 -type f | sort
rg -n "REJECT_QSC_HS_|handshake_reject|hs_decode_|hs_transcript|hs_sig|hs_reject|identity_pin|decap|encap|replay" qsl/qsl-client/qsc/src/handshake/mod.rs
rg -n "identity_fingerprint|identity_pin_matches_seen|identity_read_pin|identity_read_sig_pin|identity_write_public_record|identity_self_kem_keypair" qsl/qsl-client/qsc/src/identity/mod.rs
sed -n '1,980p' qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs
```

Required classification:

- qsc library exports only `adversarial` and `envelope` today;
- qsc binary root owns `handshake` and `identity` today;
- existing qsc fuzz targets are parser-boundary targets;
- real semantic reject paths live in qsc handshake/identity internals;
- current helper-only path would create fake-oracle risk;
- minimal recovery requires future `lib.rs`, `adversarial`, and
  `handshake/mod.rs` scope, with `identity/mod.rs` only if needed for
  stale-public-record/trusted-pin reachability.

## Recovery option validation

Required selected classification:

`SOURCE_BOUNDARY_RECOVERY_MINIMAL_READY`

Required rejected classifications:

- `SOURCE_BOUNDARY_RECOVERY_PROCESS_HARNESS_READY`;
- `SOURCE_BOUNDARY_RECOVERY_PARSER_ONLY_READY`;
- `SOURCE_BOUNDARY_RECOVERY_VECTOR_CONSUMER_NEXT`;
- `SOURCE_BOUNDARY_RECOVERY_TOO_RISKY`;
- `SOURCE_BOUNDARY_RECOVERY_AMBIGUOUS`.

Required recovery option findings:

- Option 1 selected with exact future source-boundary paths;
- Option 2 rejected for next implementation but retained as fallback;
- Option 3 rejected as parser/metadata-only downgrade;
- Option 4 deferred;
- Option 5 rejected.

## Scope guard

Changed paths must be limited to:

```text
DECISIONS.md
NEXT_ACTIONS.md
TRACEABILITY.md
docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_plan.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_testplan.md
```

Require zero changes under:

- `qsl/`;
- `qsl/qsl-client/qsc/src/`;
- `qsl/qsl-client/qsc/fuzz/`;
- `scripts/ci/`;
- `.github/`;
- `inputs/`;
- `formal/`;
- `tools/refimpl/`;
- service/public/backup/qwork tooling paths.

## Governance validation

Run:

```bash
git diff --check
python3 - <<'PY'
import pathlib, subprocess, sys
allowed = {
    "DECISIONS.md",
    "NEXT_ACTIONS.md",
    "TRACEABILITY.md",
    "docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_plan.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
    "tests/NA-0487_qsl_qsc_binding_fuzz_source_boundary_recovery_authorization_testplan.md",
}
changed = set(subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines())
extra = sorted(changed - allowed)
missing = sorted(allowed - changed)
print("CHANGED", *sorted(changed), sep="\n")
print("EXTRA", extra)
print("MISSING", missing)
if extra or missing:
    sys.exit(2)
PY
python3 - <<'PY'
import pathlib, re, sys
text = pathlib.Path("NEXT_ACTIONS.md").read_text()
ready = re.findall(r"^Status: READY$", text, flags=re.M)
print("READY_COUNT", len(ready))
ready_items = []
for m in re.finditer(r"^### (NA-\d+) .+\nStatus: READY$", text, flags=re.M):
    print("READY", m.group(1))
    ready_items.append(m.group(1))
if len(ready) != 1 or ready_items != ["NA-0487"]:
    sys.exit(2)
PY
```

Required:

- `git diff --check` passes;
- exact scope guard passes;
- READY_COUNT remains 1;
- READY remains NA-0487;
- NA-0487 remains READY, not DONE;
- NA-0488 remains absent or non-READY;
- D-0962 exists once;
- D-0963 absent;
- duplicate decision count 0.

## Link, leak, and overclaim validation

Run the manual local-link existence check from `AGENTS.md` after patching.

Run a changed-path leak scan and added-line overclaim scan. Required result:

- no missing local links;
- no sensitive endpoint, token, auth header, route token, or long-hex evidence
  dump introduced;
- no affirmative public-readiness claim introduced.
- no affirmative production-readiness claim introduced.
- no affirmative external-review-complete claim introduced.
- no affirmative crypto-complete claim introduced.
- no affirmative fuzz-complete claim introduced.
- no affirmative vector-complete claim introduced.
- no affirmative KEM-complete claim introduced.
- no affirmative signature-complete claim introduced.
- no affirmative replay-proof claim introduced.
- no affirmative downgrade-proof claim introduced.
- no affirmative side-channel-free claim introduced.
- no affirmative vulnerability-free claim introduced.
- no affirmative bug-free claim introduced.
- no affirmative perfect-crypto claim introduced.

## Inherited validation

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
PYTHONDONTWRITEBYTECODE=1 python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
PYTHONDONTWRITEBYTECODE=1 python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- all commands pass;
- if local full qsc-adversarial execution reaches missing cargo-fuzz, record
  exact output and rely on PR CI qsc-adversarial-smoke if green.

## PR and merge validation

PR body must include standalone:

```text
Goals: G1, G2, G3, G4, G5
```

PR body must state:

- D347 stop consumed;
- recovery is governance-only;
- NA-0487 remains READY;
- no implementation mutation;
- no qsc source/fuzz/Cargo/script/workflow mutation in this directive;
- selected revised scope;
- no public overclaim.

Merge requirements:

- required checks green or accepted skipped/neutral;
- no red checks;
- merge commit only, no squash/rebase;
- post-merge public-safety green;
- queue remains exactly one READY item;
- READY remains NA-0487;
- D-0962 exists on main.
