Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0434 qsc Provider Error No-Mutation Test Implementation Stop Recovery

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0434 is BLOCKED, not DONE.

D278 attempted the exact NA-0433-authorized qsc provider-error no-mutation test
implementation and stopped before repository mutation. The stop was correct:
`pq_decap_failed` appears test-feasible through a malformed pending KEM secret,
but `pq_encap_failed` is not reachable through current qsc CLI/frame APIs with
the active provider. The recovery classification is:

`NA0434_PROVIDER_ERROR_NO_MUTATION_TEST_IMPLEMENTATION_BLOCKED_RUNTIME_HOOK_NEEDED`

This recovery restores:

`NA-0435 -- QSL qsc Provider Error Path Test Hook / Defensive Branch Authorization Plan`

as the sole READY successor.

## D278 inheritance

Inherited response:

`/home/victor/work/qsl/codex/responses/NA0434_20260607T013227Z_D278.md`

Inherited proof root:

`/srv/qbuild/tmp/NA0434_provider_error_no_mutation_test_impl_20260607T012707Z`

D278 recorded:

- result: STOP before repository mutation;
- queue: READY_COUNT 1 and READY NA-0434;
- decisions: latest D-0855, D-0854 once, D-0855 once, D-0856 absent, duplicate
  count zero;
- authorized test file absent:
  `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`;
- public-safety and qsc-adversarial-smoke green on `e4a73c2322`;
- root cargo audit and nested qsc fuzz lock audit green;
- qsl-backup SHA/source-list proof unchanged.

## Current repo/queue state

Recovery preflight verified refreshed `origin/main` at `e4a73c2322`.
PR #1137 was MERGED at that merge commit. The worktree was clean before the
recovery branch was created.

Queue state before this patch:

- READY_COUNT 1.
- READY NA-0434.
- NA-0433 DONE.
- NA-0432 DONE.
- NA-0431 DONE.
- NA-0430 DONE.
- NA-0429 BLOCKED.

Decision state before this patch:

- latest decision D-0855.
- D-0854 exists once.
- D-0855 exists once.
- D-0856 absent.
- duplicate decision count zero.

## `pq_encap_failed` reachability review

D278 provider probe output:

```text
encap zero: None
encap ff: None
encap a5: None
encap inc: None
```

Interpretation: correct-length malformed public-key byte patterns did not make
`StdCrypto.encap` fail. Wrong-length A1 KEM public keys are rejected by qsc frame
decode before qsc reaches provider encapsulation. Therefore the exact
`pq_encap_failed` qsc marker is not externally triggerable through current qsc
external APIs with the active provider.

## `pq_decap_failed` partial feasibility review

D278 provider probe output also recorded:

```text
decap short sk: Some(InvalidKey)
```

That supports a future test strategy for the initiator decapsulation failure
path through malformed pending KEM secret material. D278 did not implement that
single branch because NA-0434 required both exact provider-error markers.

## Existing API / no runtime hook proof

No current qsc CLI/frame path was found that can force `pq_encap_failed` while
remaining inside NA-0434 scope.

The proof chain is:

- qsc A1 decode enforces exact KEM public-key length before responder provider
  handling;
- wrong-length A1 KEM public keys stop at frame decode, not provider encap;
- correct-length malformed byte patterns are accepted by the active provider
  encapsulation path;
- forcing encapsulation failure would require a runtime hook, provider mock,
  provider fake, crypto/provider behavior change, dependency change, or broader
  test-scope mutation.

Those expansions were forbidden by NA-0434.

## Why NA-0434 is BLOCKED, not DONE

NA-0434 acceptance required exact qsc provider-error no-mutation tests for both
`pq_encap_failed` and `pq_decap_failed`. D278 showed only the decapsulation path
is plausibly reachable through existing APIs. Marking NA-0434 DONE would
overstate evidence and weaken the one-READY governance queue.

## Recovery classification

`NA0434_PROVIDER_ERROR_NO_MUTATION_TEST_IMPLEMENTATION_BLOCKED_RUNTIME_HOOK_NEEDED`

This classification preserves the D278 stop and records that a future
authorization lane must decide whether a test-only hook/seam is appropriate or
whether the defensive encap branch should be documented instead.

## NA-0435 successor rationale

NA-0435 is the correct successor because the remaining decision is not an
implementation task. It is an authorization task: choose one exact strategy
after D278 showed that `pq_encap_failed` is not externally triggerable through
current qsc APIs with the active provider.

Candidate strategies for NA-0435:

- authorize a test-only provider failure hook implementation harness;
- authorize a provider fake or trait-level test seam;
- document the `pq_encap_failed` branch as defensive/unreachable through current
  external inputs;
- narrow future implementation to `pq_decap_failed` only;
- select another exact-scope strategy with no runtime behavior change.

## Public claim / no external-review / website boundary

This recovery introduces no public-readiness claim, no production-readiness
claim, no public-internet-readiness claim, no external-review completion claim,
no crypto-complete claim, no vulnerability-free claim, no perfect-crypto claim,
no side-channel-free claim, and no bug-free claim.

No website, README, START_HERE, public docs, or public technical paper content is
changed.

## No runtime/no dependency/no workflow/no test/no vector mutation proof

This recovery does not create or modify the NA-0434 executable test file and
does not mutate runtime code, crypto code, dependencies, Cargo manifests,
lockfiles, workflows, scripts, executable tests, fuzz targets, vectors,
qsl-server, qsl-attachments, qshield runtime, website, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, or `/backup/qsl`.

The only recovery PR paths are:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0434_qsl_qsc_provider_error_no_mutation_test_implementation_stop_recovery.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0434_provider_error_no_mutation_stop_recovery_restore_na0435_testplan.md`

## Backup-impact statement

Backup impact: none. Codex did not run backup, restore, sudo, qwork, qstart, or
qresume. Codex did not mutate qsl-backup, backup status files, backup plan files,
rollback subtree paths, `/backup/qsl`, timers, fstab, source lists, retention, or
backup scripts.

Preflight SHA/source-list evidence remained unchanged:

- qsl-backup SHA:
  `e9ecff3d22ed`
- `/home/victor/work/qsl/codex/ops` appears exactly once in the installed
  qsl-backup source list.

## Next recommendation

Execute NA-0435 as a governance-only authorization lane. It should consume D278,
classify `pq_encap_failed` reachability, and decide whether the next step is a
test-only provider hook/seam, a provider fake, defensive branch documentation, a
narrowed `pq_decap_failed`-only test implementation, or another exact scoped
strategy. NA-0435 must not implement the chosen strategy.
