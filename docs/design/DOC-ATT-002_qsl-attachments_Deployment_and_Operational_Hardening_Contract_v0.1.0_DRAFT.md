# DOC-ATT-002 — qsl-attachments Deployment and Operational Hardening Contract v0.1.0 DRAFT

Status: Draft

Purpose:
- Define the deployment / operational hardening contract and readiness ladder for `qsl-attachments` after attachment correctness and coexistence validation are complete.
- Make constrained hardware and a weak relay, especially the AWS relay, first-class validation inputs.
- Freeze the next implementation-grade operational step without changing client, service, or relay semantics.

Non-goals:
- No runtime implementation changes.
- No deployment automation.
- No default-path promotion.
- No legacy `<= 4 MiB` deprecation.
- No qsl-server changes.

## 1. Current posture and blocker

Current trustworthy state:
- `qsl-attachments` is a single-node local-disk runtime.
- Opaque ciphertext parts and committed objects are stored on local disk.
- Session/object metadata is persisted in local JSON journals.
- Minimal CI/protection exists today: `main` requires only the `rust` check.
- The service/runtime contract, qsc integration, coexistence rule, route-token migration, and no-false-`peer_confirmed` behavior are already grounded.

Current blocker:
- The blocker is operational maturity, not protocol correctness.
- The project does not yet have an implementation-grade deployment/readiness contract for `qsl-attachments`.
- The project does not yet have an agreed constrained-host validation ladder over deployed `qsl-attachments` plus the real relay.
- Therefore default-path promotion and legacy deprecation remain blocked.

## 2. Deployment profiles

| Profile | Intended use | Minimum storage / disk assumption | Restart / recovery expectation | Ingress / TLS assumption | Observability requirement | Mandatory validation stages |
| --- | --- | --- | --- | --- | --- | --- |
| Local development | Developer bring-up, contract-faithfulness checks, local debugging | Enough free local disk for one staged copy, one committed copy, and journal overhead for the largest local test object in that run | Normal restart must preserve committed objects and journal integrity; manual cleanup of abandoned staging is acceptable in development | Loopback or other explicitly private development ingress only; no public exposure; plaintext attachment handling remains forbidden | Redacted structured logs and manual inspection are sufficient | Single-flow smoke, threshold ladder, basic reject/no-mutation checks |
| Constrained-host validation | Real-world validation on weak hardware and weak relay conditions, including the AWS relay class when it is the limiting relay | Enough durable free disk for at least one full staged copy, one committed copy, and journal/retention headroom for the largest active validation object on the same host; if the `100 MiB` target class is attempted, reserve enough headroom for that object class plus staging and cleanup overhead before the run | Service restart and client restart are expected validation events; committed objects must survive restart; interrupted sessions may resume or expire only according to the existing contract and must never silently mutate | Real network ingress with TLS termination is mandatory before public exposure; secret-bearing headers must be preserved but never logged; canonical URLs remain secret-free | Redacted structured logs plus explicit capture of CPU, memory, disk, retries, backpressure, latency, and throughput for each stage | Full single-flow ladder, threshold ladder, large-file ladder, interruption/resume ladder, expiry/quota/reject paths, secret-hygiene audit, resource observation; limited concurrency only after single-flow stages pass |
| Reference deployment | Stronger operator-managed deployment used to confirm that failures seen on constrained hosts are saturation-related rather than correctness-related | Durable storage sized for at least the largest supported object class, full staging overhead, retention headroom, and backup/restore workspace for the deployment plan | Restart, recovery, backup, and restore expectations are explicit and repeatable; operator must be able to classify loss boundaries honestly | Operator-managed TLS and ingress configuration are required; reverse proxy or direct ingress must preserve secret-safe headers and log redaction rules | Redacted structured logs, metrics, and operator-visible alert thresholds are required | Full ladder including limited concurrency and repeat restart/recovery runs |

## 3. Readiness categories

### 3.1 Storage durability and recovery
- Operators must document where opaque ciphertext parts, committed objects, and JSON journals live.
- The contract boundary is local durable disk; this item does not authorize a different backend.
- Evidence must show what survives restart, what is rebuilt, and what is cleaned up.
- Backup/restore expectations must be explicit before reference deployment can claim readiness.

### 3.2 Retention, expiry, and cleanup
- Retention classes and expiry behavior already exist in runtime semantics and must not be redefined here.
- Operational hardening must define cleanup cadence, orphaned-staging expectations, and operator-visible evidence for expired-session/object behavior.
- Expired objects must be classified as expected contract behavior, not unexplained loss.

### 3.3 Quota, abuse, and saturation handling
- Quota and abuse rejects already exist semantically and must not be weakened.
- Operational hardening must define how operators observe quota saturation, abuse escalation, and disk-pressure conditions.
- Saturation must produce bounded, classifiable outcomes; it must not be misreported as protocol correctness success or failure.

### 3.4 Observability, metrics, and alerting
- Logs must remain redacted and must never contain plaintext attachments, fetch capabilities, resume tokens, or secret-bearing URLs.
- Minimum operational observation for constrained hosts is: CPU, memory, disk, retry count, backpressure indicators, latency, throughput, reject-code counts, and restart events.
- Reference deployment must add operator-visible metrics and alert thresholds for disk pressure, repeated restarts, repeated reject spikes, and abnormal retry growth.

### 3.5 Ingress, TLS, and secret handling
- Public or shared-network exposure requires TLS.
- Secrets remain in headers/body only as already defined by canonical docs; no capability-like secrets in canonical URLs.
- Reverse proxies and ingress layers must preserve required headers and apply log redaction.
- Any ingress configuration that logs raw secret-bearing headers is non-compliant.

### 3.6 Restart, resume, and interruption handling
- Client disconnect, service restart, and relay slowness/timeouts are mandatory validation events.
- Resume behavior must remain bounded and fail closed.
- Interrupted flows may degrade in speed or require retry, but they must not falsely advance delivery or mutate state silently.

### 3.7 Resource and load characterization
- Weak hardware and a weak relay are first-class engineering inputs, not exceptional excuses.
- The contract must record measured CPU, memory, disk, latency, throughput, retry/backpressure behavior, and concurrency limits.
- Operational maturity is not established by success on strong hardware alone.

### 3.8 Rollout and promotion gates
- Default-path promotion above threshold and legacy deprecation are explicitly blocked until the ladder in Section 4 is executed truthfully.
- NA-0200 defines the gate; it does not authorize crossing it.

## 4. Constrained-host validation ladder

### Stage 1 — Single-flow smoke on deployed service + real relay
Capture:
- end-to-end success markers
- wall-clock duration
- CPU, memory, disk, retry count
Pass:
- no secret leakage
- no false `peer_confirmed`
- no silent mutation
Stop / escalate:
- any protocol correctness violation stops the ladder immediately

### Stage 2 — Threshold ladder
Cases:
- `< 4 MiB`
- `= 4 MiB`
- `> 4 MiB`
Capture:
- path selected
- explicit reject or success markers
- relay latency and retries
Pass:
- `< 4 MiB` and `= 4 MiB` remain on the legacy path
- `> 4 MiB` requires deployed `qsl-attachments`
- route-token and honest-delivery semantics remain intact
Stop / escalate:
- any silent path flip, false `peer_confirmed`, or secret-bearing output is blocking

### Stage 3 — Large-file ladder
Target classes:
- `16 MiB`
- `64 MiB`
- `100 MiB` target class when the constrained host can sustain it honestly
Capture:
- throughput
- wall-clock duration
- peak CPU, memory, disk usage
- retry/backpressure behavior
Pass:
- success or bounded, classifiable degradation
- no silent corruption or state drift
Interpretation:
- inability to sustain `100 MiB` on a weak host is not automatically a correctness failure; it is acceptable only if the run ends in a bounded, truthful classification and the smaller stages remain clean

### Stage 4 — Interruption and resume
Events:
- client disconnect
- service restart
- relay slowness / timeout / retry
Capture:
- resumed vs expired outcome
- retry counts
- restart timestamps
Pass:
- contract-faithful resume or contract-faithful expiry/reject only
- no false `peer_confirmed`
- no silent state mutation

### Stage 5 — Expiry, quota, and reject paths
Capture:
- reject code
- disk state before/after
- journal/object mutation before/after
Pass:
- deterministic reject behavior
- no-mutation behavior where required
- operator can classify quota vs expiry vs abuse vs saturation cleanly

### Stage 6 — Secret-hygiene audit under real traffic
Capture:
- ingress logs
- application logs
- command/operator artifacts
Pass:
- no plaintext attachment material
- no resume/fetch capability leakage
- no secret-bearing canonical URLs

### Stage 7 — Resource observation and saturation classification
Capture:
- CPU
- memory
- disk free / disk growth
- retry growth
- backpressure behavior
- latency / throughput
Pass:
- bounded degradation is acceptable
- correctness failures are not masked as saturation
Interpretation:
- saturation means slower throughput, higher latency, or bounded retries while integrity and state invariants remain intact
- correctness failure means false `peer_confirmed`, secret leakage, no-mutation violation, resume corruption, descriptor/service mismatch, or any contract-breaking behavior

### Stage 8 — Limited concurrency
Precondition:
- Stages 1 through 7 passed in single-flow mode
Capture:
- per-flow latency
- aggregate retries
- disk pressure
- memory growth
Pass:
- bounded degradation only
- operator can state an honest concurrency envelope for the deployment profile
Stop / escalate:
- if concurrency causes ambiguous state or correctness drift, stop and record the envelope as not yet ready for promotion

## 5. Interpretation rules

Acceptable degradation on constrained hardware:
- slower transfer speed
- higher latency
- serialized or very low concurrency envelopes
- bounded retries and backpressure
- failure to reach the `100 MiB` target class when the result is recorded honestly as host saturation rather than protocol correctness failure

Unacceptable outcomes:
- false `peer_confirmed`
- silent state mutation
- secret-bearing URLs or logs
- unbounded retry storms
- disk exhaustion without deterministic classification
- integrity, decrypt, or retrieval behavior that violates the existing contract

Stop rules for the ladder:
- stop immediately on any correctness failure
- stop escalation when the constrained host cannot sustain the next stage without unbounded resource growth
- do not classify a weak relay alone as proof that the protocol is broken; confirm on the reference profile before calling it a correctness defect when the only symptom is saturation

## 6. Default-path promotion and legacy deprecation gates

Default-path promotion above threshold is blocked until all of the following are true:
- the full constrained-host ladder has been executed against deployed `qsl-attachments` plus the real relay
- at least one stronger reference deployment also completes the ladder cleanly enough to separate saturation from correctness
- ingress/TLS/log-hygiene requirements are met
- restart/recovery/retention/quota evidence is recorded
- no unresolved correctness blocker remains
- the project can state an honest operating envelope for constrained hosts and for the reference profile

Legacy `<= 4 MiB` deprecation is blocked until all of the following are true:
- default-path promotion above threshold is already justified by the gate above
- an explicit migration and rollback plan exists
- the project can prove no silent break for legacy-sized flows during the migration window
- any remaining fallback behavior is explicit and operator-visible

NA-0200 does not authorize default-path promotion or legacy deprecation.

## 7. Queue implications

The next truthful implementation item after this contract is:
- qsl-protocol: `NA-0200A — qsl-attachments Operational Hardening Implementation + Constrained-Host Real-World Validation`
- qsl-attachments: `NA-0003 — Operational Hardening Implementation + Constrained-Host Validation`

Both items implement and execute this contract. Neither item reopens attachment architecture or client/runtime semantics unless a concrete correctness defect is proven.

## 8. References

- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`
- `tests/NA-0199_legacy_transition_validation.md`
- `qsl-attachments/README.md`
- `qsl-attachments/NEXT_ACTIONS.md`
- `qsl-attachments/TRACEABILITY.md`
