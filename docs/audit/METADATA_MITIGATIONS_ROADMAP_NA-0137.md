# NA-0137 Metadata Mitigations Roadmap (Padding/Batching/Fixed-Interval)

## Context
This document defines a prioritized metadata-mitigation roadmap based on findings in `docs/audit/METADATA_LEAKAGE_AUDIT_NA-0134.md` (especially sections **B) Leakage Matrix**, **C) Mitigation Options with Cost Matrix**, and **E) Findings + Follow-on NA Recommendations**).

This is a docs-only artifact. No implementation changes are included.

## A) Leakage Drivers (From NA-0134)
Top leakage channels to address first:
- Traffic timing/cadence (send/receive timing and poll cadence) remains observable.
- Traffic size/volume patterns remain observable without stronger bucketing.
- Delivered receipt timing creates correlation between sender and receiver activity.
- File chunking patterns leak file size class and transfer rhythm.
- Error timing/classes can leak behavior classes.

These drivers map directly to NA-0134 findings P0-1, P1-1, P1-2, and P2-2.

## B) Mitigation Catalog (Prioritized)
### 1) Fixed-Interval Polling Mode
- Reduces: variable polling cadence leakage.
- Cannot hide: endpoint IP/relay visibility or message-size leakage by itself.
- New leakage risk: stable cadence fingerprint if profile is unique.
- Status: `Optional`.

### 2) Size Bucketing/Padding (Messages)
- Reduces: fine-grained message size leakage.
- Cannot hide: timing and endpoint metadata.
- New leakage risk: bucket-boundary fingerprinting.
- Status: `Optional`.

### 3) Receipt Shaping (Delay/Batch/Suppress Rules)
- Reduces: immediate delivery/read timing linkage.
- Cannot hide: all timing correlation under active traffic.
- New leakage risk: predictable delay policy may become fingerprintable.
- Status: `Optional`.

### 4) File Chunk Size Normalization
- Reduces: chunk count/size fingerprinting.
- Cannot hide: transfer start/stop timing and total volume class entirely.
- New leakage risk: normalized chunk policy can itself identify client mode.
- Status: `Optional`.

### 5) Client-Side Batching Windows
- Reduces: per-event timing granularity.
- Cannot hide: broader traffic windows and endpoint metadata.
- New leakage risk: burst-at-window-boundary pattern.
- Status: `Optional`.

## C) Cost/Impact Table
| Mitigation | Bandwidth | Latency | Battery/CPU | Complexity/Risk | Notes |
|---|---|---|---|---|---|
| Fixed-interval polling mode | Low-Medium | Medium | Low | Medium | Best near-term cadence control with bounded complexity |
| Message size bucketing/padding | Medium | Low | Low | Medium | Stronger size privacy, moderate overhead |
| Receipt shaping | Low | Medium | Low | Medium | Helps sender/receiver timing unlinkability |
| File chunk normalization | Medium-High | Low-Medium | Low | Medium | Material benefit for file metadata at transfer overhead cost |
| Batching windows | Low | Medium | Low | Medium | Useful with careful bounds and deterministic policy |

Scale source: NA-0134 section **C) Mitigation Options with Cost Matrix**, refined for implementation sequencing.

## D) Default Policy (Standard Mode)
- Keep standard mode practical and deterministic.
- Default stance:
  - Keep mitigation-heavy controls disabled by default unless they impose low operational burden and low UX disruption.
  - Expose mitigations as explicit, auditable, opt-in controls.
  - Maintain deterministic/no-mutation reject behavior for invalid combinations.
- Therefore:
  - Fixed-interval mode: `Optional` (recommended first optional control).
  - Padding/bucketing: `Optional`.
  - Receipt shaping: `Optional`.
  - File chunk normalization: `Optional`.
  - Cover traffic/constant-rate behavior: `Not recommended` for default mode at this stage due to high operational cost.

## E) NA-0138 MVP Selection
### Selected MVP for NA-0138
`Fixed-interval polling mode (optional)` is selected for NA-0138.

Rationale:
- Highest immediate reduction in variable-cadence leakage from NA-0134 P0-1.
- Smaller implementation surface than broad size or file-transfer normalization.
- Deterministic behavior can be validated with existing metadata polling test patterns.

## F) NA-0138 Acceptance Criteria and Test Plan
### Acceptance Criteria
1. Add explicit toggle/config for fixed-interval mode with bounded parameters.
2. Mode is deterministic and auditable when enabled.
3. Disabled mode preserves current behavior and compatibility.
4. Invalid configuration fails closed with deterministic reject and no state mutation.
5. Existing message/file state machines remain truthful and unchanged.

### Test Plan (Required for NA-0138)
1. **Enabled behavior**
   - Verify fixed cadence under enabled mode with deterministic markers.
2. **Disabled behavior**
   - Verify baseline polling behavior remains unchanged when mode is disabled.
3. **No mutation on reject**
   - Invalid interval/config path must reject deterministically and leave state unchanged.
4. **State-machine regression guard**
   - Existing message/file truth semantics tests remain green.
5. **Basic performance impact**
   - Record before/after measurements for polling count, latency envelope, and resource overhead under representative short runs.

### Marker/UX Guidance (Minimal)
- Prefer reusing existing metadata poll markers for cadence verification.
- If new markers are required, keep them deterministic and non-secret-bearing.
- UI indication, if added later, should remain minimal and explicit (mode on/off only).

