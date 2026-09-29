Goals: G4

Status: Authoritative (normative registry; append-only)
Owner: QSL governance
Last-Updated: 2026-09-28

# DOC-CAN-009 -- QSC client refusal codes (normative registry) v0.1.0 DRAFT

## 1. Purpose and scope

This document is the home of the CLIENT-LOCAL refusal codes of qsc: the spellings a qsc client reports when it refuses
an operation on its own state (C01 AM-6.16 (a), O21 CLOSED; C07 AM-12). DOC-SCL-002 keeps the service layer ("service
error model + reason codes"); it is the model here for FORM only: a normative code list with one meaning per code, and
coarse, actionable codes that leak nothing. DOC-SCL-002's versioning rule is NOT copied: this registry is append-only BY
ROW (C01 AM-6.16 (a)), and a new version is minted only if the column set changes.

## 2. Rules (normative)

- R1 APPEND-ONLY. A row is added by the implementing PR that allocates the code. A row is never edited, never removed
  and its code is never reused for another cause.
- R2 SPELLING. A code is ASCII `[a-z][a-z0-9_]*` and unique across qsc.
- R3 BEFORE EFFECTS. Every code in this table is a refusal before any effect is released (C07 :238).
- R4 NO COUNT. No code in this table increments the unlock-failure counter (C01 O7; C07 :238-239). The one counted code
  is the EXISTING vault_locked (C07 ER1), which is not a row here.
- R5 CAUSE, NOT GUESS. A code names its cause; "rollback" appears in user text only where the evidence supports it
  (C07 :237).

## 3. The registry

Columns as C01 AM-6.16 (a) rules them. Rows QRC-0001..QRC-0021 are allocated by the PR that creates this document (PLAN
card F04-C07P, lane NA-0787, sub-step S6); their D-record and TRACEABILITY line are owed to the NA-0787 records PR on
main (RULING_NA0787_S6a Q-8), a named deviation from C01 AM-6.16 (a)'s "allocated by that PR's D-record".

| Row | Code | Cause | Contract cell | Status |
|---|---|---|---|---|
| QRC-0001 | freshness_checkpoint_missing | LOCAL profile: the freshness checkpoint head is absent; FREEZE, never rebuilt from the supplied vault | C07 T3.6 ER2; T4.3 | ALLOCATED |
| QRC-0002 | freshness_checkpoint_corrupt | LOCAL checkpoint fails its MAC, has a bad length or trailing bytes, or a wrong magic; FREEZE | C07 T3.6 ER3; T6.5 | ALLOCATED |
| QRC-0003 | freshness_checkpoint_unsupported | checkpoint version or profile byte unsupported, or its authentic profile disagrees with the vault's protection_mode; FREEZE | C07 T3.6 ER4; T6.5 | ALLOCATED |
| QRC-0004 | freshness_generation_regression | the only retained vault is authentic and OLDER than the checkpoint; FREEZE | C07 T3.6 ER5 | ALLOCATED |
| QRC-0005 | freshness_digest_conflict | the only retained vault is authentic, at the checkpoint's generation, and not its blob; FREEZE | C07 T3.6 ER6 | ALLOCATED |
| QRC-0006 | committed_state_missing | the checkpoint names neither retained vault; FREEZE (never largest generation, never one step behind) | C07 T3.6 ER7; T4.3 | ALLOCATED |
| QRC-0007 | storage_durability_failed | a write or flush of the transaction (C2, C3, C5, B4, a recovery promotion) returned an error; the session stops and must be reconciled | C07 T3.6 ER17; T8.1 G-365; T4.4 IC3 | ALLOCATED |
| QRC-0008 | freshness_generation_exhausted | generation + 1 would exceed u64; refused before any write | C07 T3.6 ER18; T6.1 F3 | ALLOCATED |
| QRC-0009 | vault_protection_mode_unsupported | the vault's stored protection_mode is not exactly "local-checkpoint" or "tpm" | C07 T3.6 ER19; T6.1 F2 | ALLOCATED |
| QRC-0010 | anchor_unqualified | protection "tpm" requested or found and no TPM is qualified (the T2 PL8 list is empty); no TPM command, no write | C07 T3.6 ER21; AM-3; T6.3 | ALLOCATED |
| QRC-0011 | freshness_state_dir_invalid | the freshness state location is unusable: HOME or the test override is relative, or a non-directory occupies a component of the state path; refused before any authority write | C07 AM-1 provider kinds StateRootError::HomeNotAbsolute, StateRootError::OverrideNotAbsolute, CheckpointDirError::RootNotAbsolute, CheckpointDirError::NotADirectory (MAPPING rows 2, 3, 10, 12); NEW | ALLOCATED |
| QRC-0012 | freshness_lineage_lock_contended | the per-lineage lock is held by another process: this lineage is open through another copy or pathname of the store; a refusal, never a wait | C07 AM-1 provider kind LockError::Contended (MAPPING row 16; C07 T4.2 FN1); NEW | ALLOCATED |
| QRC-0013 | freshness_lineage_lock_unstable | the per-lineage lock path was replaced after every locking attempt (the inode re-check is exhausted); treated as tampering, not as a system failure | C07 AM-1 provider kinds LockError::InodeRetriesExhausted, LockError::InodeChanged (MAPPING rows 19, 84, 85; rows 84-85 by the S7b addendum; C07 T4.2 FN1); NEW | ALLOCATED |
| QRC-0014 | freshness_lineage_changed | the store changed between the quarantined read and the read under the lineage lock (the authority belongs to another vault, or no longer authenticates); transient: open again | C07 AM-1 provider kinds BeginError::LineageChanged, RecoverError::OpenFailed under the lineage lock (MAPPING rows 23, 36); NEW | ALLOCATED |
| QRC-0015 | freshness_lineage_mismatch | the current and prepared vaults both authenticate but disagree on their lineage (vault identity, checkpoint key or protection mode); FREEZE | C07 AM-1 provider kind FreezeKind::LineageMismatch (MAPPING row 32); NEW | ALLOCATED |
| QRC-0016 | freshness_checkpoint_unreadable | the local checkpoint exists but could not be read, at recovery or at the commit-time head re-verification | C07 AM-1 provider kinds RecoverError::CheckpointRead, ChainBreak::HeadRead (MAPPING rows 25, 47, 67); NEW | ALLOCATED |
| QRC-0017 | freshness_head_changed | at commit the checkpoint head no longer verifies as the committed state under the successor's checkpoint key; the transaction is refused before any write | C07 AM-1 provider kind ChainBreak::HeadChanged (MAPPING row 46; C07 T4.2 FN2, local form); NEW | ALLOCATED |
| QRC-0018 | freshness_checkpoint_exists | at init the store is empty but a checkpoint already exists for the new vault's lineage (a replayed genesis vault or a copied state directory); refused before any write | C07 AM-1 provider kind GenesisError::CheckpointExists (MAPPING row 70); NEW | ALLOCATED |
| QRC-0019 | freshness_successor_invalid | the vault the client built for a commit or for init does not authenticate, or does not extend the committed lineage (vault identity, protection mode, generation, predecessor anchor); a client defect, never a passphrase failure; refused before any write | C07 AM-1 provider kinds CommitError::SuccessorOpenFailed, ChainBreak::VaultId, ChainBreak::Mode, ChainBreak::Generation, ChainBreak::PredecessorAnchor, ChainBreak::CheckpointKey, GenesisError::B0OpenFailed, GenesisBreak::Generation, GenesisBreak::PredecessorAnchor (MAPPING rows 40-45, 56-58; row 45 as changed by the S7b addendum); NEW | ALLOCATED |
| QRC-0020 | vault_file_oversized | a vault file on disk is longer than the client's bound; refused unread, never shown to the vault opener | C07 AM-1 provider kind RecoverError::BlobTooLarge (MAPPING rows 8, 22, 64); C01 AM-6.17 O14; NEW | ALLOCATED |
| QRC-0021 | vault_capacity_exceeded | the vault the client built exceeds the vault size bound; remove content; refused before any write | C07 AM-1 provider kinds CommitError::SuccessorTooLarge, GenesisError::B0TooLarge (MAPPING rows 39, 55); NEW | ALLOCATED |

"MAPPING" is the ruled map of NA-0787 sub-step S6 (MAPPING.md TABLE 1, ruled by RULING_NA0787_S6a R1). No row carries a
path, a key or an identifier.

## 4. Not in this registry (pointers, no rows)

- (a) EXISTING qsc codes the freshness provider reuses; they were not allocated here and keep their existing homes:
  vault_locked, vault_missing, vault_read_failed, vault_exists, missing_home, unsafe_path_symlink, unsafe_parent_perms,
  io_write_failed, io_read_failed, lock_open_failed, lock_failed, vault_parse_failed.
- (b) C07 T3.6 spellings not produced by the local-checkpoint provider: ER1 vault_locked (EXISTING, unchanged, the one
  counted code) and the TPM-profile spellings ER8-ER16, ER20 and ER22 (anchor_missing, anchor_uninitialized,
  anchor_identity_changed, anchor_auth_failed, anchor_lockout, anchor_rate_limited, anchor_unavailable,
  anchor_template_mismatch, commit_indeterminate, anchor_capacity_exhausted, tpm_owner_auth_set), and T6.1 F6's
  vault_parse_failed. Each is a PROPOSED spelling registered here by its own implementing PR (C07 AM-12).
- (c) C01's PROPOSED client-local codes, and those of its amendments, are registered here by their own implementing PRs
  (C01 AM-6.16 (a)).
