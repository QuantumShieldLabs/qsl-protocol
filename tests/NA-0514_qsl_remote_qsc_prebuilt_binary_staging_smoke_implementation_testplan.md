Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0514 remote qsc prebuilt binary staging and smoke implementation testplan

## Objective

Validate that NA-0514 consumed NA-0513/D406 inheritance, built or selected a
local qsc binary from the clean checkout, staged it to the approved remote test
account, verified local/remote hash equality, ran only non-protocol qsc smoke,
retained the binary only with cleanup proof, and selected NA-0515 with no
remote E2EE, no public-readiness claim, and no production-readiness claim.

## Scope checks

Required changed paths for the implementation PR:

- `docs/governance/evidence/NA-0514_qsl_remote_qsc_prebuilt_binary_staging_smoke_implementation_harness.md`
- `tests/NA-0514_qsl_remote_qsc_prebuilt_binary_staging_smoke_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden implementation mutation:

- qsc source/test/fuzz/Cargo files.
- workflow/script/helper files.
- dependency or lockfiles.
- corpus/vector/input files.
- formal/refimpl/service/public/backup files.
- qsl-server, qsl-attachments, qshield, or qshield-cli paths.
- qwork/qstart/qresume or qsl-backup mutation.

## Startup verification

Pass criteria:

- qwork proof files exist and are copied without rerunning qwork.
- qwork `.kv` and `.json` mirror required fields.
- proof HEAD and proof origin/main match live refs before fetch.
- `/` usage is below 95%.
- READY_COUNT is 1.
- READY item is NA-0514.
- NA-0513 and NA-0512 are DONE.
- D-1015 exists once.
- D-1016 exists once.
- D-1017 is absent before patch.
- duplicate decision count is zero.

## Inheritance checks

Pass criteria:

- D406 response exists.
- NA-0513/D406 triaged the three red scheduled remote checks as non-required
  residuals.
- NA-0512 classification `REMOTE_MARKER_PROBE_PASS_TOOLCHAIN_ABSENT` is
  consumed.
- remote `git`, `cargo`, `rustc`, and `qsc` absence is consumed.
- prebuilt binary staging/smoke with retention preference is consumed.
- no remote E2E is authorized.
- no source checkout/build or package install is authorized.

## Local build checks

Pass criteria:

- `cargo build -p qsc --locked --bin qsc` succeeds using proof-root-local target
  output.
- exactly one executable qsc binary is selected.
- source commit, target dir, binary size, sha256, and file type are recorded.
- Cargo manifests and lockfiles remain unchanged.
- qsc source/test/fuzz/corpus files remain unchanged.

## Local smoke checks

Pass criteria:

- local qsc `--help` exits 0.
- no local qsc send/receive protocol command is run.
- local smoke output contains no private key block, credential assignment,
  production endpoint marker, or actual `qsc send` / `qsc receive` invocation
  marker.

## SSH config checks

Pass criteria:

- `ssh -G inspiron` is read locally only.
- safe parsed fields show host `inspiron`, user `qslcodex`, batch mode,
  password authentication disabled, identities-only enabled, strict host key
  checking enabled, no forwarding, and identityfile basename only.
- no private key content or credential assignment appears.
- no SSH config, known_hosts, authorized_keys, or key material is mutated.

## Remote prep checks

Pass criteria:

- exactly one bounded SSH prep invocation occurs.
- account is `qslcodex`.
- UID is not 0.
- privileged groups are absent.
- negative `sudo -n true` check fails as expected.
- backup exposure is absent.
- qwork and qsl-backup are absent.
- qsl-remote-test workdir exists and is writable.
- qsl-remote-test bin directory is ready.
- final and stage paths are absent before transfer.
- no file is written during prep except permitted bin directory creation if
  absent.

## Transfer checks

Pass criteria:

- exactly one scp transfer is attempted.
- transfer writes only to `qsl-remote-test/bin/qsc_<proof_id>.stage`.
- transfer exits 0.
- no rsync or sftp is used.
- no retry occurs.

## Remote verify and smoke checks

Pass criteria:

- exactly one bounded SSH verify/smoke invocation occurs.
- boundary checks are repeated.
- stage file exists and is not a symlink.
- remote stage sha256 equals local sha256.
- chmod is limited to the stage path.
- qsc `--help` exits 0.
- final path is absent before move.
- stage moves to final path.
- stage residue is absent.
- final sha256 equals local sha256.
- final owner is `qslcodex` if stat supports owner reporting.
- retained final path is qsl-remote-test-relative.

## Redaction checks

Pass criteria:

- raw proof remains proof-root-local.
- checked-in evidence includes only summaries.
- no private key block appears.
- no passphrase/token/password/API-key assignment pattern appears.
- no production endpoint marker appears.
- no backup material marker appears.
- no qwork execution marker appears.
- no qsl-backup execution marker appears.
- no actual `qsc send` / `qsc receive` invocation marker appears.

## Governance checks

Pass criteria:

- evidence doc contains all required NA-0514 sections.
- evidence doc contains all NA-0514 markers.
- D-1017 exists once.
- D-1018 is absent before optional closeout.
- TRACEABILITY contains NA-0514 row.
- rolling journal contains NA-0514 entry and recovered-failure records.
- selected successor is NA-0515 remote E2EE scope authorization.
- exactly one READY item remains mandatory.

## Required validation commands

Run:

- `git diff --check`
- exact five-path scope guard.
- deterministic relative markdown link check.
- leak scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for NA-0514 evidence.
- private-key-block proof.
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Expected markers

- `NA0514_REMOTE_STAGING_SCOPE_CONSUMED_OK`
- `NA0514_LOCAL_QSC_BINARY_BUILT_OR_SELECTED_OK`
- `NA0514_LOCAL_QSC_BINARY_HASH_RECORDED_OK`
- `NA0514_LOCAL_QSC_SMOKE_OK`
- `NA0514_REMOTE_PREP_BOUNDARY_OK`
- `NA0514_REMOTE_QSC_BINARY_STAGED_OK`
- `NA0514_REMOTE_QSC_BINARY_HASH_MATCH_OK`
- `NA0514_REMOTE_QSC_SMOKE_OK`
- `NA0514_REMOTE_QSC_RETENTION_DECISION_OK`
- `NA0514_NO_REMOTE_E2E_OK`
- `NA0514_NO_REMOTE_SOURCE_BUILD_OK`
- `NA0514_NO_PACKAGE_INSTALL_OK`
- `NA0514_NO_SUDO_ADMIN_OK`
- `NA0514_NO_BACKUP_EXPOSURE_OK`
- `NA0514_NO_QWORK_QSLBACKUP_OK`
- `NA0514_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0514_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0514_ONE_READY_INVARIANT_OK`

## Post-fix hardening review checklist

1. Correctness under stress: hash/path/owner/smoke/retention proof must remain
   explicit, and future E2EE must recheck retained binary state before use.
2. Minimality: only the five allowed governance/testplan/journal paths may
   change in the implementation PR.
3. Maintainability: evidence must separate raw proof-root logs from checked-in
   summaries and preserve exact cleanup instructions.
4. Coverage quality: validation must include scope, link, leak, claim, marker,
   qsc tests, corpus validators, formal checks, audits, fmt, and shell syntax.
5. Cross-lane stability: Linux/macOS full-suite skip policy remains inherited
   docs/governance context; no qsc runtime, workflow, dependency, or corpus
   mutation occurs.
