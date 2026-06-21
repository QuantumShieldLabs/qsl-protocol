Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0516 Build-to-Inspiron remote qsc client-to-client E2EE implementation harness

## Executive summary

NA-0516 consumed the D409 stop, cleaned the D409 local sensitive runtime and the unintended remote literal-dollar-HOME tree, hardened the command manifest to use resolved absolute remote paths, rechecked the retained Inspiron qsc binary, built local Build qsc from the clean checkout, initialized isolated local and remote qsc roots with synthetic data only, and attempted the bounded Build-to-Inspiron remote qsc E2EE flow.

The E2EE flow did not complete. The result classification is `REMOTE_E2EE_TRANSPORT_FAILURE`: both SSH reverse forwarding and SSH local forwarding were administratively blocked before handshake/send/receive could succeed. The lane did not introduce qsl-server, qsl-attachments, package installation, remote source checkout/build, qsc source/test/fuzz/Cargo mutation, workflow/script/helper/dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, or public/production readiness claims.

Selected successor: `NA-0517 -- QSL Remote qsc E2EE Transport Remediation Scope Authorization Plan`.

## Live NA-0516 scope

Allowed repository mutation stayed limited to this evidence file, `tests/NA-0516_qsl_build_to_inspiron_remote_qsc_client_to_client_e2ee_implementation_testplan.md`, `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Runtime scope stayed limited to proof-root-local artifacts under `/srv/qbuild/tmp/NA0516_cleanup_retry_remote_e2ee_20260621T170007Z/`, local sensitive runtime under that proof root until cleanup, the retained remote qsc binary at `/home/qslcodex/qsl-remote-test/bin/qsc`, and the retry remote E2EE root under the resolved absolute remote home until cleanup.

## qwork proof-file verification

Codex read, copied, and verified the qwork proof files without running qwork, qstart, or qresume.

- qwork proof result: `startup_result=OK`.
- Lane/repo/path: `NA-0516`, `qsl-protocol`, `/srv/qbuild/work/NA-0516/qsl-protocol`.
- Proof and live pre-fetch refs matched: `67fac8c8d0d6`.
- Proof cleanliness passed: `worktree_clean=yes`, `index_clean=yes`, `untracked_clean=yes`.
- Queue proof passed: READY_COUNT 1 and READY `NA-0516`.
- Disk proof passed: `/` usage 83%, below the 95% stop threshold.

## D409 inheritance and cleanup proof

D409 stopped before governance patch, branch, commit, PR, merge, or closeout. Its classification was `REMOTE_E2EE_AMBIGUOUS_STOP`, caused by an out-of-scope remote path mutation that created this literal-dollar-HOME root:

`/home/qslcodex/$HOME/qsl-remote-test/e2ee/NA0516_build_to_inspiron_remote_e2ee_impl_20260621T163755Z`

D409 also left this local sensitive runtime root:

`/srv/qbuild/tmp/NA0516_build_to_inspiron_remote_e2ee_impl_20260621T163755Z/sensitive_runtime/`

Inherited D409 facts consumed here:

- Retained remote qsc recheck had passed before D409 stopped.
- Local qsc provenance had passed before D409 stopped.
- No qsl-server or qsl-attachments path was used.
- No send/receive, reply, negative boundary, cleanup, repo edit, PR, or merge completed.
- D-1021 was absent before this retry.

D410 cleanup proof:

- Local D409 sensitive root existed before cleanup and was removed exactly.
- Remote one-shot cleanup removed the exact bad root and empty literal-dollar-HOME parents.
- Remote cleanup markers: `NA0516_D409_REMOTE_BAD_ROOT_CLEANED_OK`, `NA0516_D409_LITERAL_HOME_PARENT_ABSENT_OK`, `NA0516_D409_RETAINED_QSC_UNTOUCHED_OK`.
- Retained qsc remained present, owned by `qslcodex`, and matched sha256 `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.

## Command surface inspection and hardened manifest

Read-only inspection covered the current qsc tests and command modules for:

- passphrase-file vault initialization.
- identity rotate/show.
- contacts add, device list, and device trust.
- relay inbox-set and relay serve.
- handshake init/poll.
- send and receive over relay transport.
- wrong-mailbox negative/no-mutation behavior.

The hardened manifest resolved remote home with a bounded read-only SSH command and verified it was exactly `/home/qslcodex`. The manifest computed the retry root locally as an absolute path and rejected executable remote paths containing literal `$HOME`, `..`, or any prefix outside the retry root or retained qsc binary.

Manifest proof markers:

- `NA0516_REMOTE_HOME_RESOLVED_OK`
- `NA0516_NO_LITERAL_DOLLAR_HOME_PATHS_OK`
- `NA0516_REMOTE_PATH_CONTAINMENT_OK`
- `NA0516_QSL_SERVER_NOT_USED_OK`
- `NA0516_QSL_ATTACHMENTS_NOT_USED_OK`

The first remote retry mutation ran a containment preflight before creating the E2EE root.

## Literal-dollar-HOME prevention proof

No executable retry path used a `$HOME` placeholder. The retry manifest used only absolute remote paths under `/home/qslcodex/qsl-remote-test/e2ee/NA0516_cleanup_retry_remote_e2ee_20260621T170007Z` or the retained qsc binary path.

Remote boundary recheck found no literal `$HOME` path component under `/home/qslcodex` after D409 cleanup and before retry. Retry cleanup rechecked the same condition after deleting the retry root.

## Local / remote boundary rechecks

Local `ssh -G inspiron` safe-field parsing recorded user `qslcodex`, hostname `inspiron`, `BatchMode yes`, `PasswordAuthentication no`, `IdentitiesOnly yes`, strict host-key checking enabled, no agent forwarding, no X11 forwarding, and identityfile basename `qslcodex_ed25519`.

Remote boundary recheck passed:

- user `qslcodex`; uid `1003`; groups `qslcodex`.
- negative `sudo -n true` failed as required.
- `/backup/qsl` absent or not readable.
- `qwork` absent.
- `qsl-backup` absent.
- retry E2EE root absent before first retry mutation.
- no literal-dollar-HOME residue.

## Retained remote qsc hash/path/owner recheck

Retained qsc path: `/home/qslcodex/qsl-remote-test/bin/qsc`.

Recheck result:

- owner: `qslcodex`.
- sha256: `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- help smoke: passed.
- post-cleanup retained qsc recheck: passed.

## Local qsc provenance

Local source commit: `67fac8c8d0d6`.

Diff from retained source commit `6e0796de79c9` over qsc source/Cargo/lockfile paths was empty.

Local qsc was built with:

`cargo build -p qsc --locked --bin qsc`

Build output stayed under the proof root. Local qsc size was `102103920` bytes and sha256 was `dcd4c004a4f6ee535e96e96bac046e3751d9b0c1afc64f6c3738115e609313cd`. Local qsc `--help` smoke passed.

## Isolated runtime roots and synthetic data policy

Local sensitive runtime was created only under the proof root and was deleted during cleanup. Remote qsc runtime was created only under the resolved retry E2EE root and was deleted during cleanup.

Synthetic message labels were:

- Build-to-Inspiron label derived from `QSL_REMOTE_E2EE_SYNTHETIC_BUILD_TO_INSPIRON_<PROOF_ID>`.
- Inspiron-to-Build label derived from `QSL_REMOTE_E2EE_SYNTHETIC_INSPIRON_TO_BUILD_<PROOF_ID>`.

No production data, personal data, private keys, passphrases, tokens, credentials, backup material, or private qsc state is included in checked-in evidence.

## Build-to-Inspiron send/receive proof

Build-to-Inspiron send/receive did not complete. The first transport attempt used a proof-root-local qsc relay and SSH reverse forwarding; SSH reported remote port forwarding failure before handshake.

The bounded recovery switched to a remote qsc relay under the E2EE root plus SSH local forwarding. That also failed before successful handshake/send/receive because the SSH server rejected forwarding with `administratively prohibited`.

No successful Build-to-Inspiron `send`/`receive` marker is claimed.

## Inspiron-to-Build reply proof

No Inspiron-to-Build reply was attempted after the transport failure blocked the positive flow before successful Build-to-Inspiron send/receive.

No successful reply marker is claimed.

## Negative/no-mutation boundary proof or deferral

The wrong-mailbox negative/no-mutation boundary was not executed because the positive transport path failed before send/receive. The successor should keep wrong-mailbox no-mutation as a required validation item once a transport path is authorized and working.

## No-secret-output review

Runtime command-output logs were scanned after cleanup.

Markers:

- `NO_PRIVATE_KEY_BLOCKS_OK`
- `NO_SYNTHETIC_PASSPHRASE_OUTPUT_OK`
- `NO_API_TOKEN_STYLE_OUTPUT_OK`
- `NO_RAW_ROUTE_TOKEN_OUTPUT_OUTSIDE_MANIFEST_OK`
- `NO_SECRET_OUTPUT_REVIEW_OK`

An initial overly broad scanner included compiled Rust artifacts and found static library/help strings. That was recorded as a recovered scanner-scope issue; the corrected runtime-output scan passed.

## Cleanup / retention proof

Cleanup passed:

- Remote retry E2EE root deleted.
- Local proof-root sensitive runtime deleted.
- Retained remote qsc binary preserved and rechecked.
- Literal-dollar-HOME residue remained absent.

Cleanup markers:

- `REMOTE_E2EE_ROOT_CLEANED_OK`
- `RETAINED_QSC_POST_CLEANUP_OK`
- `NO_LITERAL_DOLLAR_HOME_RESIDUE_AFTER_RETRY_OK`
- `LOCAL_SENSITIVE_RUNTIME_CLEANED_OK`

## Result classification

`REMOTE_E2EE_TRANSPORT_FAILURE`

Reason: SSH TCP forwarding was unavailable for both the approved reverse-forwarding and local-forwarding transport shapes. Continuing with a custom proxy or other non-authorized transport would risk behavior drift beyond the approved NA-0516 command family.

## Hostile Cryptographer Review

The attempted flow proves only bounded setup, cleanup, and transport-block evidence. It does not prove remote E2EE interoperability because send/receive/reply did not complete.

Replay resistance, downgrade resistance, side-channel behavior, and secret-material lifecycle remain unproven by this lane. The retained binary replacement risk remains bounded only by hash/path/owner rechecks before and after use. Synthetic messages avoided secret exposure, and no public/production readiness claim follows from this evidence.

## Red-Team Review

If the retained remote qsc changes after recheck, future lanes must stop on hash/path/owner mismatch. If transfer selection could include private material, future lanes must stop before transfer. If qsc output leaks secret-looking material, raw output must stay quarantined and checked-in evidence must exclude it.

If relay/tunnel artifacts remain, cleanup proof must fail. This lane cleaned the retry root and local sensitive runtime, and it rechecked retained qsc after cleanup. If a negative boundary later fails open, the successor must stop or select remediation rather than weakening fail-closed behavior.

## Production SRE Review

This run is operationally useful but bounded: it proved cleanup of prior residue, retained binary freshness, local build provenance, isolated runtime setup, and a concrete transport blocker. Logs were kept under the proof root; checked-in evidence summarizes markers and omits private qsc material.

Failure remained isolated from qwork, qsl-backup, backup trees, production data, public services, qsl-server, and qsl-attachments. This does not imply public readiness, production readiness, public-internet readiness, crypto completeness, replay proof, downgrade proof, secret-material completeness, side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## Release-Claim Boundary Review

No public-ready claim is made. No production-ready claim is made. No public-internet-ready claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## Successor selection

Selected successor:

`NA-0517 -- QSL Remote qsc E2EE Transport Remediation Scope Authorization Plan`

Objective: authorize the next bounded remote qsc E2EE transport lane after NA-0516 proved that SSH TCP forwarding is unavailable, selecting a truthful forwarding-free or operator-authorized transport approach before any renewed send/receive/reply attempt.

## Future scope bundle

Future scope should decide one of:

- operator-authorized SSH forwarding enablement evidence without mutating SSH config from Codex.
- a qsc-native forwarding-free relay/inbox approach already present in the repo.
- a separately authorized diagnostic lane if transport requires new helper/proxy behavior.

The successor must continue to forbid qsl-server, qsl-attachments, package installation, remote source checkout/build, qsc source/test/fuzz/Cargo mutation unless separately authorized, public exposure, and public/production readiness claims.

## Future validation / marker plan

Future retry markers should include:

- retained qsc hash/path/owner/help recheck.
- absolute remote path lint with no literal-dollar-HOME executable paths.
- transport authorization proof.
- Build-to-Inspiron send/receive exact synthetic message match.
- Inspiron-to-Build reply exact synthetic message match.
- wrong-mailbox no-mutation proof.
- no-secret-output proof.
- remote/local cleanup proof.

## No qsl-server / no qsl-attachments boundary

No qsl-server command, process, path, or service was used. No qsl-attachments command, process, path, or service was used.

## No public/production readiness boundary

This lane records a transport blocker, not readiness. No public-ready, production-ready, or public-internet-ready claim is made.

## Backup-impact statement

No qsl-backup command was run. No backup or restore was run. The installed qsl-backup helper was read only; its sha256 matched the expected value and the Codex ops source inclusion count remained exactly 1.

## Rejected alternatives

- Custom SSH-over-stdio or Python proxy after SSH forwarding failed: rejected because it would broaden the approved command family and risk untruthful behavior drift.
- qsl-server or qsl-attachments: rejected and not used.
- Package installation, remote source checkout/build, SSH config mutation, known_hosts mutation, or sudo/admin action beyond the negative probe: rejected and not used.
- Public exposure of a relay service: rejected and not used.

## Next recommendation

Proceed to the selected NA-0517 transport remediation authorization lane. It should decide the transport shape before any new remote E2EE run, then repeat retained-qsc, absolute-path, no-secret-output, and cleanup proof.
