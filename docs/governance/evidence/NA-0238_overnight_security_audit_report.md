Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0238 Overnight Security Audit Report

## Scope

This was a read-only audit and roadmap/policy lane. No runtime, protocol, crypto, demo, service, workflow, script, Cargo, qsc-desktop, qsl-server, qsl-attachments, or website code was changed.

## Public-safety proof

Post-PR `#723` `origin/main` was `d11c363380df`. The main `public-safety` check, `advisories`, `qsc-linux-full-suite`, and `macos-qsc-full-serial` were already completed success when NA-0238 began, so no pre-work timeout occurred.

## Commands run

- `df -BG /srv/qbuild`
- `git status --porcelain=v1 --branch`
- `git fetch --all --prune`
- `git rev-parse origin/main`
- `gh pr view 723 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url`
- `gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url`
- `gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url`
- `gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks`
- `gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection`
- deterministic NEXT_ACTIONS parser
- deterministic DECISIONS parser
- `gh api /repos/QuantumShieldLabs/qsl-protocol/commits/<PR-723-merge-commit>/check-runs --paginate`
- `gh run list --commit <PR-723-merge-commit> --json databaseId,workflowName,status,conclusion,createdAt,updatedAt,url,headBranch,event --limit 50`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked responder_requires_bundle_equivalent_initiator_evidence -- --nocapture`
- `cargo build --manifest-path tools/actors/refimpl_actor_rs/Cargo.toml --locked`
- `python3 scripts/ci/run_suite2_establish_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor --file inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json --actor-name suite2-establish`
- `CARGO_TARGET_DIR=target bash scripts/ci/demo_cli_smoke.sh`
- `CARGO_TARGET_DIR=target bash scripts/ci/metadata_conformance_smoke.sh`
- targeted `rg` static sweep over KT, QSP, actor, qsc, qshield-cli, public-safety helper, and workflow surfaces

## Pass/fail summary

Passed:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked` resolving `rustls-webpki v0.103.13`
- direct `send_commit` canary, 3 passed
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- KT verifier vector test, 1 passed
- responder initiator-KT evidence test, 1 passed
- `refimpl_actor` build
- Suite-2 establish vectors, 14 of 14
- demo CLI smoke
- metadata conformance smoke

Recovered local command/environment issues:

- The first Suite-2 establish-vector command used `target/debug/refimpl_actor`, but qbuild placed the actor in the configured target cache. Classification: recoverable command-path issue. Corrective action: reran with the actual built actor path. Final result: pass, 14 of 14.
- The first demo smoke used the script default while the qbuild target-dir setting put binaries outside `./target`; the script then could not find `./target/debug/qshield`. Classification: recoverable local environment/path issue. Corrective action: reran with `CARGO_TARGET_DIR=target`, matching workflow expectations. Final result: pass.

## Static audit findings

Targeted pattern counts over the requested security surfaces:

- `TODO`: 0
- `FIXME`: 0
- `panic!`: 6, apparently test/sentinel paths in the sampled output
- `unwrap(`: 45, mostly test/helper or precondition-shaped code in sampled output; qshield relay helper unwraps remain worth later review
- `expect(`: 13, mostly fixed-size output, mutex, and test/helper expectations in sampled output
- `disabled`: 19, concentrated in explicit KT disabled-nonproduction handling
- `always_accept`: 0
- `mock`: 3, qsc vault retired mock-provider markers
- `allow_`: 43, includes KT disabled-nonproduction and public-safety repair-admission options
- `bypass`: 1, a public-safety rejection string stating advisory-remediation bypass is not allowed
- `public-safety`: 17, workflow/helper governance surfaces
- `branch protection`: 0 in the scanned surfaces
- `kt`: 178, expected KT implementation/test density
- `verifier`: 19, expected KT verifier/test density

No proven runtime bug was established from this static sweep. The counts identify review targets for later implementation lanes, especially qshield relay helper unwraps and public-safety admission-rule hardening.

## Crypto and KT observations

The KT verifier now evaluates bundle signatures before enabled KT state update, rejects disabled KT shapes unless explicit non-production mode is allowed, enforces pinned log IDs, verifies STH freshness and signatures, requires inclusion and consistency proof validity, and commits accepted KT state only after evaluation succeeds.

The responder path now calls `verify_responder_binding` and the direct regression proves missing or mismatched initiator bundle-equivalent evidence rejects while matching evidence succeeds.

No-state-mutation evidence is strongest in the KT consistency test and existing QSP/Suite-2 reject tests. The audit recommends expanding these into explicit conformance vectors for KT state update rejection, SCKA persistence/rollback, skipped-key decrypt failure, and demo rejection paths.

Disabled/non-production KT mode is explicit in the code and tests. The report found no evidence that authenticated KT-enabled mode silently falls back to disabled acceptance, but broader demo-path KT carriage still needs future acceptance coverage.

## Demo and conformance observations

The local demo smoke proves a valid two-peer establish/send/receive path. The metadata smoke proves loopback binding, unsafe public-bind rejection without explicit acknowledgement, store permissions, relay authorization rejection, and padding-related setup.

Gaps remain because one-command demo acceptance does not yet consolidate invalid downgrade, malformed message, replay, malformed KT evidence, and attachment-path negative scenarios into one product-facing acceptance target.

## Public-safety red-main prevention recommendation

NA-0239 should implement executable public-safety red-main deadlock prevention hardening. The successor should prove an exact active-NA repair admission rule with positive and negative helper tests, while preserving fail-closed default behavior, required-check truth, no branch-protection exception, no check spoofing, and no blanket bypass.

## Suspected bugs or gaps

Proven bugs: none found in the read-only NA-0238 audit.

Recommendations and uncertain gaps:

- Public-safety repair admission needs executable helper tests so future known repair lanes do not require settings exceptions.
- Demo acceptance needs negative-path coverage for downgrade, malformed input, replay, and KT malformed evidence.
- SCKA persistence/monotonicity deserves the next vector/test expansion because rollback and tombstone invariants are release-critical.
- qshield relay helper unwraps should be reviewed in a future implementation lane; this audit did not prove they are reachable panics in shipped paths.

## No-code-changed statement

NA-0238 changed documentation, governance, queue, decision, traceability, and audit/testplan files only. It did not change code or CI configuration.
