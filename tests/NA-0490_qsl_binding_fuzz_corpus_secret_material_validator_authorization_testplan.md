Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-17

# NA-0490 Binding Fuzz Corpus Secret-Material Validator Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0490 authorization-only decision for a future binding fuzz
corpus secret-material validator while preserving no implementation mutation,
no corpus/vector/input mutation, no qsc source/fuzz/Cargo/script/workflow
mutation, no dependency/lockfile mutation, no backup/restore mutation, and no
public-claim expansion.

## Protected invariants

- qwork proof files are read, not regenerated.
- Codex does not run qwork, qstart, or qresume.
- READY_COUNT is exactly 1 at startup.
- READY item is NA-0490 at startup.
- NA-0489 is DONE.
- NA-0488 is DONE.
- NA-0487 is DONE.
- D-0967 exists once.
- D-0968 exists once.
- D-0969 is absent before the patch and exists once after the patch.
- D-0970 is absent during the evidence PR.
- duplicate decision count is zero.
- PR #1249 is merged.
- D355 closeout evidence or in-tree D-0968 evidence is consumed.
- `/` usage remains below 95 percent.
- qsl-backup installed SHA and source inclusion boundary match directive
  expectations.
- no checked-in `qsc_binding_semantics` corpus exists.
- future checked-in binding corpus remains blocked until validator
  implementation.
- no public-readiness claim, no production-readiness claim,
  no public-internet-readiness claim, no external-review-complete claim,
  no crypto-complete claim, no fuzz-complete claim, no corpus-complete claim,
  no vector-complete claim, no replay-proof claim, no downgrade-proof claim,
  no side-channel-free claim, no vulnerability-free claim, no bug-free claim,
  or no perfect-crypto claim is introduced.

## Allowed scope

- `docs/governance/evidence/NA-0490_qsl_binding_fuzz_corpus_secret_material_validator_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- validator implementation
- helper implementation
- script implementation outside governance/testplan evidence
- qsc source mutation
- qsc fuzz target mutation
- qsc fuzz Cargo mutation
- qsc fuzz lockfile mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency mutation
- root or nested lockfile mutation
- vector/input/corpus mutation
- formal mutation
- refimpl mutation
- qsl-server, qsl-attachments, qshield, qshield-cli, service mutation
- public-doc, website, README, START_HERE, public technical paper mutation
- backup, restore, qsl-backup, backup status, backup plan, rollback, backup
  tree, nightly/local-ops script mutation
- qwork, qstart, qresume, qshell mutation

## Required authorization evidence

Inspect the evidence doc and governance updates.

Required:

- NA-0489/D355 or in-tree D-0968 evidence consumed.
- current corpus and validator surface inventoried.
- secret-material pattern matrix defined.
- validator strategy options reviewed.
- CI/workflow/dependency impact reviewed.
- primary classification selected.
- selected NA-0491 successor recorded.
- exact future scope recorded.
- no implementation mutation in NA-0490.
- no corpus/vector/input mutation in NA-0490.
- no qsc source/fuzz/Cargo/script/workflow mutation in NA-0490.
- no dependency/lockfile/formal/refimpl/service/public/backup mutation in
  NA-0490.
- exactly one READY remains mandatory.

## Startup proof commands

Run:

```bash
test -f /srv/qbuild/work/NA-0490/.qwork/startup.qsl-protocol.kv
test -f /srv/qbuild/work/NA-0490/.qwork/startup.qsl-protocol.json
python3 -m json.tool /srv/qbuild/work/NA-0490/.qwork/startup.qsl-protocol.json >/dev/null
git status --porcelain=v1 --branch
git diff --name-only
git ls-files --others --exclude-standard
git rev-parse HEAD
git rev-parse origin/main
```

Required:

- qwork fields match the directive.
- proof HEAD and proof origin/main match live refs before fetch.
- no fetch occurs before proof/live freshness is verified.
- worktree is clean before mutation.

## Queue and decision proof

Run deterministic parsing over `NEXT_ACTIONS.md` and `DECISIONS.md`.

Required:

- `READY_COUNT 1`.
- READY `NA-0490`.
- `NA-0489_STATUS DONE`.
- `NA-0488_STATUS DONE`.
- `NA-0487_STATUS DONE`.
- `D-0967_COUNT 1`.
- `D-0968_COUNT 1`.
- `D-0969_COUNT 1` after patch.
- `D-0970_COUNT 0` for the evidence PR.
- duplicate decision count 0.

## Corpus inventory validation

Run:

```bash
find qsl/qsl-client/qsc/fuzz/corpus -maxdepth 2 -type d -print | sort
find qsl/qsl-client/qsc/fuzz/corpus -maxdepth 2 -type f -printf '%h\n' | sort | uniq -c
test ! -d qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
```

Required:

- existing corpus dirs are limited to the parser/payload/vault-envelope corpus
  dirs unless a later directive authorizes more.
- `qsc_binding_semantics` corpus is absent.
- future likely binding corpus path is identified but not created.

## Secret-material matrix validation

Inspect evidence doc.

Required reject/flag classes:

- PEM private key headers.
- OpenSSH private key headers.
- age/minisign/PGP/private-key style markers if present.
- obvious API tokens.
- passphrase labels.
- KEM secret key names.
- signature secret key names.
- identity secret key names.
- backup key or recovery key names.
- runtime/service secret labels.
- private endpoints or production-like identifiers if disallowed by policy.
- high-entropy unallowlisted material.
- actual qsc vault secret filenames or contents if detected.
- user/operator data markers.

Required allow/safe classes:

- short synthetic byte arrays.
- mutated public message bytes.
- public keys only when clearly synthetic/test/public.
- vector IDs.
- manifest category names.
- non-secret labels.
- expected reject reason strings.
- small structured JSON metadata.
- comments documenting internal-only status.

Required classification:

- `SECRET_PATTERN_MATRIX_READY`.

## Strategy and CI impact validation

Inspect evidence doc.

Required:

- Option 2 dedicated repo-local script selected.
- `BINDING_FUZZ_CORPUS_VALIDATOR_SCRIPT_READY` selected.
- `VALIDATOR_NO_DEPENDENCY_NO_WORKFLOW_READY` selected.
- `VALIDATOR_WORKFLOW_INTEGRATION_LATER` recorded.
- immediate workflow integration rejected.
- no Cargo change required.
- no lockfile change required.
- default corpus scan scope is all qsc fuzz corpora.
- future binding corpus remains the immediate blocker.
- explicit allowlist/provenance is required for high-entropy corpus files.

## Local validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- PASS for all required commands unless a directive-approved local tool
  availability caveat is recorded.
- root cargo audit PASS and nested qsc fuzz lock audit PASS are dependency
  health evidence only.
- qsc-adversarial syntax checks PASS.

## Exact scope guard

Changed paths for the NA-0490 evidence PR must be exactly:

```text
DECISIONS.md
TRACEABILITY.md
docs/governance/evidence/NA-0490_qsl_binding_fuzz_corpus_secret_material_validator_authorization_plan.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
tests/NA-0490_qsl_binding_fuzz_corpus_secret_material_validator_authorization_testplan.md
```

Required:

- EXTRA_COUNT 0.
- MISSING_COUNT 0.
- no implementation/corpus/vector/input/source/fuzz/Cargo/script/workflow/
  dependency/lockfile/formal/refimpl/service/public/backup path changes.

## PR validation

Before PR:

- READY_COUNT 1.
- READY NA-0490.
- D-0969 exists once.
- D-0970 absent.
- duplicate decision count 0.
- exact five-path scope.
- PR body contains `Goals: G1, G2, G3, G4, G5`.
- PR body includes Impact, No-regression, and Tests/Vectors.
- PR body says authorization-only.
- PR body says selected successor NA-0491.
- PR body says no implementation mutation.
- PR body says no corpus/vector/input mutation.
- PR body says no qsc source/fuzz/Cargo/script/workflow mutation.
- PR body says no dependency/lockfile mutation.
- PR body says no public overclaim.

After PR merge:

- D-0969 exists once on main.
- queue still has exactly one READY item, NA-0490.
- public-safety is green.
- no red required check.
- worktree is clean.

## Optional closeout prerequisites

Optional closeout to NA-0491 may run only after:

- NA-0490 evidence PR merged.
- post-merge public-safety on evidence merge commit is green.
- queue still has exactly one READY item, NA-0490.
- D-0969 exists once on main.
- D-0970 absent before closeout patch.
- worktree is clean.

Closeout must:

- mark NA-0490 DONE.
- restore exactly one READY successor, NA-0491.
- add D-0970.
- not implement NA-0491.
- touch only the closeout-authorized paths.

## Post-fix hardening review

Report:

1. Correctness under stress.
2. Minimality.
3. Maintainability.
4. Coverage quality.
5. Cross-lane stability.
