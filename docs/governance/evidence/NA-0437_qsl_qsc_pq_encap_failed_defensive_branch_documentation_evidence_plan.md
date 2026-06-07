Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

# NA-0437 qsc pq_encap_failed Defensive Branch Documentation Evidence Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0437 documents `pq_encap_failed` as a qsc defensive provider-error branch
under current active provider and qsc external API behavior.

The classification is:

`PQ_ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTED`

Supporting caveat:

`PQ_ENCAP_FAILED_PROVIDER_BEHAVIOR_DEPENDENT`

No executable coverage is claimed for `pq_encap_failed`. D278 showed that
wrong-length A1 KEM public keys are rejected during frame decode before provider
encapsulation and that the active provider did not fail encapsulation for the
tested correct-length malformed public-key byte patterns. NA-0436 adds bounded
executable no-mutation evidence for `pq_decap_failed` only.

Selected successor:

`NA-0438 -- QSL qsc Provider Error Path Fuzz / Adversarial Coverage Authorization Plan`

## Live NA-0437 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0437 -- QSL qsc pq_encap_failed Defensive Branch Documentation / Evidence Plan`

Status: READY.

Allowed NA-0437 mutation paths:

- `docs/governance/evidence/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_evidence_plan.md`
- `tests/NA-0437_qsl_qsc_pq_encap_failed_defensive_branch_documentation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden current-lane mutation scope:

- no runtime code mutation;
- no crypto code mutation;
- no dependency, Cargo manifest, or lockfile mutation;
- no workflow, script, executable test, fuzz target, or vector mutation;
- no qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup or restore execution;
- no qsl-backup, backup status, backup plan, rollback subtree, or `/backup/qsl`
  mutation;
- no public technical paper content;
- no production readiness claim;
- no public-internet readiness claim;
- no external-review completion claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no bug-free claim;
- no vulnerability-free claim;
- no perfect-crypto claim.

Acceptance criteria:

- `pq_encap_failed` defensive-branch status is documented from D278 evidence.
- `pq_decap_failed` test evidence from NA-0436 is referenced without overclaim.
- no executable coverage overclaim is made for `pq_encap_failed`.
- no implementation mutation occurs.
- cargo audit remains green.
- nested fuzz lock audit remains green.
- public-safety is green before merge and after merge.
- exactly one READY item remains.

Stop conditions:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1142 not merged at `a896e335191d`;
- queue not READY NA-0437 at start;
- D-0860 absent or D-0861 already present at start;
- root or nested cargo audit not green;
- D278, D279, D280, D281, or D282 evidence cannot be consumed;
- defensive branch status cannot be supported by evidence;
- successor cannot be selected safely;
- forbidden mutation, backup/restore, qsl-backup mutation, or public overclaim
  occurs;
- more than one READY item exists.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0437/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0437/.qwork/startup.qsl-protocol.json`

Required `.kv` markers were present:

- `startup_result=OK`
- `lane=NA-0437`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0437/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0437`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, head, origin/main, clean-state fields, READY count, queue top, and
requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `a896e335191d`. PR #1142 was verified MERGED with merge
commit `a896e335191d`.

Proof root:

`/srv/qbuild/tmp/NA0437_pq_encap_failed_defensive_branch_doc_20260607T150152Z`

## D278 / D279 / D280 / D281 / D282 inheritance

### D278 reachability evidence

D278 response:

`/home/victor/work/qsl/codex/responses/NA0434_20260607T013227Z_D278.md`

D278 recorded a STOP before repository mutation because exact
`pq_encap_failed` coverage could not be reached through current qsc CLI/frame
APIs with the active provider.

Inherited D278 provider probe output:

```text
encap zero: None
encap ff: None
encap a5: None
encap inc: None
decap short sk: Some(InvalidKey)
```

Interpretation:

- wrong-length A1 KEM public keys do not reach provider encapsulation because
  qsc frame decode requires the exact runtime KEM public-key length;
- correct-length malformed public-key byte patterns tested by D278 did not make
  `StdCrypto.encap` fail;
- `pq_decap_failed` was partially feasible through malformed pending KEM secret
  material, but NA-0434 required both exact markers.

### D279 stop-recovery inheritance

D279 response:

`/home/victor/work/qsl/codex/responses/NA0434_20260607T023903Z_D279.md`

D279 restored NA-0435 after recording NA-0434 as BLOCKED, not DONE. It
preserved the finding that no existing qsc CLI/frame path could force
`pq_encap_failed` without a runtime hook, provider mock/fake, provider behavior
change, dependency change, or broader test-scope mutation.

### D280 strategy authorization inheritance

D280 response:

`/home/victor/work/qsl/codex/responses/NA0435_20260607T033622Z_D280.md`

D280 selected:

`NARROW_DECAP_ONLY_TEST_AUTHORIZATION_READY`

It classified `pq_encap_failed` as:

- `ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTATION_CANDIDATE`
- `ENCAP_FAILED_TEST_ONLY_SEAM_REQUIRES_RUNTIME_CHANGE`

It classified `pq_decap_failed` as:

- `DECAP_ONLY_TEST_IMPLEMENTATION_READY`
- `DECAP_ONLY_TEST_SCOPE_NEEDS_AUTHORIZATION`

### D281 decap-only test inheritance

D281 response:

`/home/victor/work/qsl/codex/responses/NA0436_20260607T062003Z_D281.md`

D281 implemented and merged PR #1141 with the exact narrowed
`pq_decap_failed` no-mutation test. It preserved the `pq_encap_failed` caveat
and made no executable coverage claim for that branch.

### D282 closeout inheritance

D282 response:

`/home/victor/work/qsl/codex/responses/NA0436_20260607T071721Z_D282.md`

D282 verified PR #1142 merged, post-merge public-safety was green, NA-0436 was
DONE, and NA-0437 was restored as the sole READY successor. It preserved the
boundary that NA-0437 must document `pq_encap_failed` without executable
coverage overclaim.

## `pq_encap_failed` defensive branch source review

Source:

`qsl/qsl-client/qsc/src/handshake/mod.rs`

The responder-side A1 path decodes an initializer frame through
`hs_decode_init`. The decode path computes the expected KEM public-key length
from `runtime_pq_kem_public_key_bytes()` and slices the message only after the
overall frame length matches the exact expected payload length. Wrong-length A1
KEM public keys therefore fail the frame shape before the responder has an
`init.kem_pk` to pass into provider encapsulation.

The `pq_encap_failed` marker appears in the responder A1 path:

```text
let c = StdCrypto;
let (kem_ct, ss_pq) = match c.encap(&init.kem_pk) {
    Ok(v) => v,
    Err(_) => {
        emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
        continue;
    }
};
```

Required condition to reach it:

- A1 frame decode succeeds with an exact-length `init.kem_pk`.
- Identity pin checks do not reject first.
- `StdCrypto.encap(&init.kem_pk)` returns `Err(_)`.

Current evidence before the branch:

- qsc frame decode rejects wrong-length A1 KEM public-key material before the
  provider call;
- D278 correct-length malformed public-key byte patterns reached provider
  parsing but did not make `StdCrypto.encap` fail;
- qsc constructs `StdCrypto` directly at this path and has no currently
  authorized provider fake or injection seam.

The active provider path:

`tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`

The active `StdCrypto.encap` implementation converts public-key bytes through
`ml_kem768_ek_from_bytes(pubk)?` and then calls provider encapsulation. Wrong
lengths are rejected by the provider wrapper as `CryptoError::InvalidKey`, but
qsc A1 decode prevents wrong-length A1 public keys from reaching this call.

Why correct-length malformed public keys did not trigger the marker:

- D278 tested correct-length byte patterns (`zero`, `ff`, `a5`, `inc`).
- Each probe recorded `None` for provider encapsulation error.
- Under this evidence, those candidate public keys did not produce
  `pq_encap_failed`.

Why NA-0436 did not cover it:

- NA-0436 was explicitly decap-only.
- Its test mutates Alice's persisted pending `kem_sk` after a valid A1/B1
  exchange and then exercises the initiator B1 decapsulation path.
- The test includes an explicit caveat that `pq_encap_failed` is not covered.
- No runtime hook or provider fake was used.

Truthful defensive-branch conclusion:

`pq_encap_failed` is real fail-closed code for the case where provider
encapsulation returns an error after a structurally valid A1 decode. Under
current evidence, no qsc external input path has been found that can force that
provider error with the active provider. Documenting it as a defensive branch is
therefore truthful, provided the documentation does not claim executable
coverage.

Future caveat:

This classification is provider behavior dependent. It could change if:

- the active provider starts rejecting additional correct-length public-key
  encodings during encapsulation;
- an alternate provider has stricter public-key validation;
- qsc adds an explicitly authorized test-only provider fake or injection seam;
- a future fuzz/adversarial campaign discovers a valid-length A1 input that
  reaches `pq_encap_failed` without runtime mutation.

Future executable proof would need:

- production behavior unchanged;
- no weakening of fail-closed behavior;
- an exact test-only seam or provider fake authorized by a future lane; or
- provider-behavior differential evidence showing a natural provider error from
  exact-length A1 public-key bytes.

## `pq_decap_failed` test evidence reference

Test file:

`qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

Exact test:

`pq_decap_failed_reject_does_not_mutate_sessions_or_pending_state`

Required markers emitted by the test:

```text
NA0436_PQ_DECAP_FAILED_MARKER_OK
NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK
NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK
NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK
NA0436_NO_RUNTIME_HOOK_USED_OK
```

What it proves:

- a malformed pending KEM secret causes the qsc initiator B1 path to reject with
  `pq_decap_failed`;
- Alice and Bob session stores remain unchanged by that reject;
- Alice and Bob pending/vault state remain unchanged by that reject;
- no A2 is emitted after the reject;
- no runtime hook is used.

Boundary:

- evidence is bounded to `pq_decap_failed`;
- it does not cover `pq_encap_failed`;
- it does not prove every provider-error path;
- it does not remove the need for future provider-error fuzz/adversarial or
  formal/model alignment planning.

## Defensive branch classification

Selected classification:

`PQ_ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTED`

Supporting caveat:

`PQ_ENCAP_FAILED_PROVIDER_BEHAVIOR_DEPENDENT`

Rejected classifications:

- `PQ_ENCAP_FAILED_REACHABILITY_AMBIGUOUS`: rejected because D278/D279/D280 and
  current source review give a coherent current-provider/current-API explanation.
- `PQ_ENCAP_FAILED_TEST_SEAM_REQUIRED_FOR_EXECUTABLE_COVERAGE`: true as a
  future executable-coverage caveat, but not the primary current classification.
- `PQ_ENCAP_FAILED_RUNTIME_BUG_SUSPECTED`: rejected because no runtime bug was
  found.

## Future strategy matrix

| Option | Status | Rationale |
|---|---|---|
| Option 1 - Keep defensive branch documented only | Selected for NA-0437 | Current evidence supports documentation without executable overclaim. |
| Option 2 - Future test-only provider fake or seam authorization | Future-gated | Required only if QSL wants executable `pq_encap_failed` coverage; must prove production behavior unchanged. |
| Option 3 - Future provider-behavior differential or KAT-style exploration | Future candidate | Useful if provider behavior changes or alternate providers are considered; must avoid public assurance overclaim. |
| Option 4 - Future runtime/crypto change | Rejected for now | No runtime bug was found; runtime/crypto mutation is out of scope. |
| Option 5 - Stop / ambiguity | Rejected | Evidence supports defensive-branch documentation. |

Selected option:

`Option 1 - Keep defensive branch documented only`

Future path:

Move next to bounded fuzz/adversarial provider-error coverage authorization,
not immediate runtime seams.

## Successor selection

Selected successor:

`NA-0438 -- QSL qsc Provider Error Path Fuzz / Adversarial Coverage Authorization Plan`

Rationale:

- deterministic decap-only evidence is now merged;
- `pq_encap_failed` has been documented as a current defensive branch;
- D278/D279/D280 left fuzz/adversarial provider-error coverage as a future
  candidate;
- fuzz/adversarial authorization can decide whether future fuzz target or
  adversarial harness changes are warranted without implementing them in this
  closeout.

Not selected:

- formal/model alignment successor: useful later, but fuzz/adversarial coverage
  is the next direct provider-error evidence question;
- nonce/key/RNG lifecycle audit successor: valuable backlog, but less directly
  connected to this provider-error evidence chain;
- test-only seam authorization successor: not selected because immediate
  executable `pq_encap_failed` coverage is not required by current evidence;
- ambiguity resolution successor: not selected because the current defensive
  classification is supportable.

## Future path/scope bundle

For the selected NA-0438 successor, future mutable paths should be limited to:

- `docs/governance/evidence/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_plan.md`
- `tests/NA-0438_qsl_qsc_provider_error_path_fuzz_adversarial_coverage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0438 may inspect read-only:

- `qsl/qsl-client/qsc/fuzz/`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- qsc provider-error paths
- `formal/`
- `inputs/`
- relevant evidence docs

Future forbidden scope unless exact later authorization permits it:

- runtime or crypto implementation mutation;
- dependency, Cargo manifest, or lockfile mutation;
- workflow mutation;
- executable test, fuzz target, or vector mutation;
- public docs, README, START_HERE, or website mutation;
- qsl-server, qsl-attachments, or qshield runtime mutation;
- backup, restore, qsl-backup, backup status, or backup plan mutation;
- no public assurance claim expansion;
- secret material handling.

## Future validation/marker plan

For the selected NA-0438 successor:

- `NA0438_PROVIDER_ERROR_FUZZ_COVERAGE_AUTHORIZATION_OK`
- `NA0438_PQ_ENCAP_FAILED_CAVEAT_CONSUMED_OK`
- `NA0438_PQ_DECAP_FAILED_TEST_EVIDENCE_CONSUMED_OK`
- `NA0438_NO_RUNTIME_CHANGE_OK`
- `NA0438_NO_DEPENDENCY_CHANGE_OK`
- `NA0438_NO_WORKFLOW_CHANGE_OK`
- `NA0438_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0438_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0438_NO_SECRET_MATERIAL_OK`
- `NA0438_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0438_ONE_READY_INVARIANT_OK`

## Public claim/external review/website boundary

This is internal governance evidence only.

- Defensive branch documentation is not production readiness.
- Defensive branch documentation is not public-internet readiness.
- Defensive branch documentation is not external-review completion.
- Defensive branch documentation is not crypto-complete proof.
- Defensive branch documentation is not side-channel-free proof.
- Defensive branch documentation is not bug-free proof.
- Defensive branch documentation is not vulnerability-free proof.
- Defensive branch documentation is not perfect-crypto proof.
- Defensive branch documentation is not public technical paper content.
- No README, START_HERE, public docs, or website update is made.
- No public-readiness or public-security assurance claim is made.
- Cargo audit green is dependency-health evidence only.
- `pq_encap_failed` defensive branch documentation does not claim executable
  coverage.
- `pq_decap_failed` test evidence remains bounded to that marker.

## Rejected alternatives

- Immediate runtime or crypto implementation change: rejected because no runtime
  bug was found and current scope forbids it.
- Immediate provider fake or test seam: rejected for this lane because it would
  require separate exact authorization and proof that production behavior is
  unchanged.
- Claiming NA-0436 covered both provider-error markers: rejected because the
  merged test is decap-only and explicitly preserves the encap caveat.
- Treating cargo audit green as public assurance: rejected because it is only
  dependency-health evidence.
- Mutating public docs, website, README, or START_HERE: rejected because this is
  internal governance evidence only.

## Backup-impact statement

Backup impact: none.

Codex did not run backup, restore, sudo, qwork, qstart, or qresume. Codex did
not mutate qsl-backup, `/backup/qsl`, backup status files, backup plan files,
rollback subtree paths, timers, fstab, source lists, retention, or backup
scripts.

Preflight read-only proof:

- qsl-backup SHA matched `e9ecff3d22ed`;
- the codex ops source-list inclusion count was exactly 1.

## Next recommendation

After this evidence PR merges with public-safety green, close out NA-0437 and
restore:

`NA-0438 -- QSL qsc Provider Error Path Fuzz / Adversarial Coverage Authorization Plan`

NA-0438 should remain authorization-only. It should consume the
`pq_encap_failed` defensive-branch caveat and the bounded `pq_decap_failed`
test evidence, then decide whether future fuzz/adversarial coverage mutation is
warranted without implementing that mutation in the authorization lane.
