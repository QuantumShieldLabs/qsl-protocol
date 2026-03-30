Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-03-30

# DOC-G4-001 — Adversarial Validation / Fuzz / Chaos Program and Post-Audit Priority Decision v0.1.0 DRAFT

Purpose:
- execute `NA-0216` using the completed audit set plus refreshed qbuild state proofs;
- freeze the adversarial validation / fuzz / chaos program strongly enough that implementation scope is no longer ambiguous;
- evaluate whether direct adversarial-program implementation, secret-ingress remediation, or one smaller finalization gap is the truthful next blocker; and
- define the smallest truthful successor lane after the GUI sequence.

Non-goals:
- no qsl-protocol runtime change in this item;
- no qsl-server or qsl-attachments edits in this item;
- no website or `.github` work;
- no protocol, wire, auth, crypto, or state-machine semantic change; and
- no AWS-dependent validation plan.

Result:
- `AVP1` is chosen.
- Closeout path `CA2` is truthful.
- The adversarial program is now explicit enough that no smaller finalization gap remains, but the next implementation blocker is still high-severity client / relay secret-ingress remediation.

## 1. Authoritative inputs reviewed

This decision is grounded by the current merged state of:
- qsl-protocol governance:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
- qbuild authority proof for this directive:
  - refreshed `mirror/main` heads for qsl-protocol, qsl-attachments, and qsl-server;
  - matching directive checkout `HEAD` values and `git branch -vv` proof;
  - matching `git ls-remote origin refs/heads/main` proof for all three repos.
- qsl-protocol runtime/docs evidence relevant to the priority decision:
  - `qsl/qsl-client/qsc/src/main.rs`
  - `qsl/qsl-client/qsc/src/cmd/mod.rs`
  - `qsl/qsl-client/qsc/src/vault/mod.rs`
  - `docs/design/DOC-QSC-009_Desktop_GUI_Prototype_Boundary_v0.1.0_DRAFT.md`
  - `docs/design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md`
  - representative `qsc` regression tests covering vault ingress, route-header discipline, delivery truth, and GUI-sidecar behavior.
- qsl-server read-only evidence relevant to the priority decision:
  - qsl-server `README.md`
  - qsl-server `NEXT_ACTIONS.md`
  - qsl-server `TRACEABILITY.md`
  - qsl-server `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
  - qsl-server `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`
  - qsl-server `src/lib.rs`
- sibling repo posture:
  - qsl-attachments `READY=0`;
  - qsl-server `READY=0`; and
  - qsl-server `NA-0011` remains `DONE`.
- the completed Director-approved audit set summarized for this directive:
  - audit index priorities;
  - adversarial validation / fuzz / chaos audit;
  - metadata / secrecy audit;
  - crypto / protocol audit;
  - maintainability audit; and
  - website / docs / legal / public-surface audit.

## 2. Frozen audit conclusions

The audit set now acts as an explicit input to queue prioritization.

### 2.1 Cross-cutting actions that ranked highest in the audit index

The audit index prioritizes:
1. retiring URI-carried route-token compatibility paths;
2. removing `qsc` passphrase-via-argv / env / re-echo style ingress;
3. adding fuzz / property / sanitizer coverage;
4. splitting `qsc/src/main.rs`; and
5. aligning website / legal / public metadata and inventory surfaces.

### 2.2 Adversarial-program conclusion

Current repo evidence already shows a strong deterministic negative-path, vector, and bounded soak culture.

What is still visibly missing:
- coverage-guided fuzzing;
- property-based/stateful invariant testing; and
- sanitizer / Miri style execution lanes.

### 2.3 High-severity secret-hygiene conclusion

The strongest service-side hygiene is already in qsl-server and qsl-attachments, but the highest-severity practical remaining issues are:
- qsl-server legacy URI-carried route-token compatibility paths still being live;
- qsc passphrase acquisition and reuse through argv / env / direct operator-visible ingress patterns; and
- public/demo normalization of env-secret handling patterns.

Current merged-state proof matches that conclusion:
- qsl-server still documents and accepts compatibility-only `/v1/push/:channel` and `/v1/pull/:channel` shapes even after the migration decision froze path carriage as compatibility-only and operator-hostile.
- qsc still supports direct passphrase arguments, environment-variable ingress, and process-level `QSC_PASSPHRASE` reuse in current runtime paths and current bounded GUI docs.

### 2.4 Crypto / protocol conclusion

No obvious catastrophic flaw was surfaced in the canonical + refimpl slice reviewed by the audit set.

Assurance remains bounded:
- formal/model coverage is still partial rather than end-to-end;
- provider assumptions remain under-explicit in operator-facing client surfaces; and
- side-channel and misuse resistance claims are still weaker than the desired release-gate posture.

### 2.5 Maintainability conclusion

`qsl/qsl-client/qsc/src/main.rs` remains the dominant maintainability and auditability risk in the client lane:
- `21,787` lines;
- `88.85%` of `qsl/qsl-client/qsc/src`; and
- the largest structural obstacle to targeted review and future harness extraction.

### 2.6 Website / docs / legal / public-surface conclusion

The audit set also found stale public-surface metadata and demo guidance drift:
- public copy is often more honest than the metadata layer;
- structured data and public inventory remain stale in places; and
- some public/demo guidance still normalizes weaker secret-handling patterns.

This matters to queue ranking, but it does not create a smaller adversarial-program finalization blocker. It reinforces that any secret-ingress remediation lane must align qsl-protocol public/demo docs that still teach the weaker pattern.

## 3. Frozen adversarial validation / fuzz / chaos program

The current deterministic negative-path culture should be extended, not replaced. The frozen program is therefore four complementary workstreams rather than one generic "add fuzzing" task.

| Workstream | Frozen target surfaces | Seed inputs / corpora | Required invariants | Truthful execution placement |
| --- | --- | --- | --- | --- |
| Coverage-guided fuzzing | qsc vault-envelope and unlock-ingress parsing; qsc receive / descriptor / marker parse boundaries; qsl-server push/pull route-token header/path/query parsing | existing `inputs/suite2/vectors/*.json`; current `qsc` regression fixtures; qsl-server compatibility tests and probe inputs | fail-closed reject behavior; no mutation on reject; no secret-bearing URL or passive-output leakage; route-token/header mismatch remains deterministic | qbuild-first local targets and corpora; only fast deterministic subsets should become required CI later |
| Property / invariant testing | qsc vault/session state transitions, readiness gating, route-header carriage, receive-mode and honest-delivery transitions | current `qsc` deterministic tests, especially vault, route-header, lifecycle, GUI-contract, and receive/send flows | explicit state markers stay truthful; legacy/header mismatch never mutates state; route tokens never migrate back into canonical URLs; reject paths stay no-mutation | repo-local deterministic harnesses first; promote to CI only when runtime and signal cost are explicit |
| Sanitizer / Miri lane | selected qsc vault/store surfaces, qsl-server request-handler boundaries, and any newly extracted parser/helper crates used by the adversarial lane | existing unit/integration tests reused as the first driver set | no panic/UB assumptions hiding under malformed input; fail-closed behavior remains intact under instrumented builds | qbuild-first nightly/instrumented evidence lane before any required CI promotion |
| Chaos / restart / disruption | qsc local vault/session restart behavior, qsl-server compatibility-restore and mismatch boundaries, qsl-attachments recovery/stress surfaces already proven in prior lanes | qsl-server `NA-0011` restore evidence; qsl-attachments `NA-0005` stress/soak/chaos evidence; local qsc runbooks/tests | restart/recovery remains truthful; no dishonest delivery; no secret-bearing evidence leaks; bounded degradations remain classified honestly | bounded qbuild runs only; AWS-free and evidence-driven |

Additional frozen implementation rules:
- prefer extracting targetable helpers or small parser/state modules rather than stacking more harness plumbing into the `21,787` line `main.rs` monolith;
- reuse current deterministic fixtures and negative-path markers as initial corpora before introducing generated inputs;
- keep qsl-server transport-only and qsl-attachments opaque ciphertext-only; and
- do not claim a CI lane exists until it is actually wired and signal-bearing.

## 4. Post-audit option set

| Option | Operational risk reduction | Security urgency | Implementation readiness | Queue / roadmap fit | Cross-repo blast radius and prerequisites | Judgment |
| --- | --- | --- | --- | --- | --- | --- |
| `AVP0` direct adversarial validation / fuzz / chaos implementation | Medium: materially improves assurance, but leaves the most concrete current secret-ingress exposures live | Medium: missing fuzz/property/sanitizer coverage is real, but it is primarily an assurance gap rather than the highest-severity live operator risk | High: the frozen program above is implementation-grade enough | Partial: sensible as the next major hardening lane, but not the most urgent blocker | Medium/high: touches qsl-protocol first and likely qsl-server / qsl-attachments targeted surfaces later; no missing contract, but not the smallest urgent cut | Rejected |
| `AVP1` secret-ingress remediation outranks direct adversarial implementation | Highest: removes current practical secret-handling risk directly and aligns public/demo posture with the frozen metadata contract | Highest: the audit set explicitly ranks these issues above the adversarial-program gap | High: the failing surfaces are concrete, already visible on `main`, and bounded enough for one direct follow-on lane | Strongest: resolves the known high-severity debt before expanding assurance instrumentation | Medium: qsl-protocol + qsl-server lane, but the contracts and failure modes are already explicit and no new prerequisite docs are missing | Chosen |
| `AVP2` one smaller adversarial-program finalization gap still blocks implementation | Low: would mostly restate decisions without removing current risk | Low: no evidence shows a remaining ambiguity stronger than the already frozen program | Low: no missing contract remains that prevents truthful implementation planning | Weak: would add another docs-only lane between a decision-grade audit set and already concrete remediation work | Low: narrow scope, but unnecessary churn | Rejected |

## 5. Decision

Chosen result:
- `AVP1`

Closeout path:
- `CA2`

Exact reason:
- the adversarial validation / fuzz / chaos program is now explicit enough that a later implementation lane is no longer blocked by missing contracts;
- the audit set and current repo proof still show higher-severity live secret-ingress debt on `main`;
- implementing fuzz/property/sanitizer work before removing those explicit secret-ingress paths would mis-rank the current security posture; and
- the website/public-surface drift is real, but in this queue position it should be handled only where it overlaps the same secret-ingress remediation inside qsl-protocol docs.

Exact remaining blocker:
- retire qsc passphrase-via-argv / env / re-echo style ingress and state reuse patterns;
- retire qsl-server URI-carried route-token compatibility paths;
- stop teaching weaker env-secret handling patterns in qsl-protocol public/demo/operator docs that remain in scope; and
- preserve current fail-closed delivery truth, route-token/header discipline, qsl-server transport-only posture, and qsl-attachments opaque ciphertext-only posture while doing so.

Smallest truthful successor lane:
- `NA-0216B — Client / Relay Secret-Ingress Remediation`

## 6. Frozen post-remediation implementation boundary

Once `NA-0216B` is complete, the adversarial-program implementation lane should proceed without reopening the decision.

That later lane should:
1. implement the frozen workstreams in Section 3 with qbuild-first, AWS-free execution;
2. start with the concrete qsc and qsl-server surfaces already identified here;
3. reuse existing deterministic vectors/tests/evidence as seed corpora before widening coverage;
4. keep sanitizer / Miri work truthful as informational until signal/noise is proven; and
5. prefer small targetable helper extractions over further centralizing harness logic inside `qsl/qsl-client/qsc/src/main.rs`.

What that later lane must not do:
- claim secret-ingress remediation is optional;
- hide current deployment/compatibility truth behind synthetic harness behavior;
- reintroduce capability-like secrets into canonical URLs;
- weaken route-token/header carriage rules; or
- use AWS validation as a prerequisite for repo-local adversarial coverage.

## 7. References

- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `DECISIONS.md`
- `FORMAL_VERIFICATION_PLAN.md`
- `formal/README.md`
- `docs/design/DOC-G5-004_Metadata_Leakage_Surface_Review_and_Logging_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-QSC-009_Desktop_GUI_Prototype_Boundary_v0.1.0_DRAFT.md`
- `docs/design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- qsl-server `README.md`
- qsl-server `NEXT_ACTIONS.md`
- qsl-server `TRACEABILITY.md`
- qsl-server `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
- qsl-server `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`
- qsl-server `src/lib.rs`
