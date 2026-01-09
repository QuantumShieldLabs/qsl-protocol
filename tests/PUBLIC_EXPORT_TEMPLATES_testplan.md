# Test Plan — Public Export Templates Sync

Goals: G4

## Purpose
Validate that the private allowlist/export pipeline includes community health files and GitHub templates so future public cuts remain deterministic.

## Scope
This test plan covers:
- Allowlist inclusion of:
  - CODE_OF_CONDUCT.md
  - SUPPORT.md
  - .github/ISSUE_TEMPLATE/*
  - .github/PULL_REQUEST_TEMPLATE.md
  - docs/INDEX.md
  - .github/workflows/public-ci.yml
  - .github/CODEOWNERS
  - docs/CHECKLIST_DOCS_PR.md
  - docs/CHECKLIST_RELEASE.md
- Public-cut safety checks:
  - denylist filenames = 0 (.env, *.pem, *.key, *.p12)
  - high-confidence credential scan = 0 (private key blocks / known token formats)
    - Exclude .github/workflows/public-ci.yml (intentional regex patterns)
    - Use `rg -g '!.github/workflows/public-ci.yml'` to keep exclusion narrow

Out of scope:
- Protocol semantics, cryptographic behavior, interop execution

## Preconditions
- Private repo working tree clean
- Export performed from a known clean SHA
- Workspace rooted at /tmp/qsl-public-release

## Procedure
1. Generate expected allowlist file list for the target SHA including the four template/community items above.
2. Produce an export directory using the expected list (e.g., tar -T or equivalent).
3. Verify denylist filename hits are zero.
4. Run high-confidence credential scan; verify zero hits.
5. Confirm exported file list contains the template/community files and matches expected list.
6. Confirm export includes docs/INDEX.md, .github/workflows/public-ci.yml, .github/CODEOWNERS, and both checklist docs.
7. (Optional) Compare exported file list to public repo main file list.

## Expected Results
- denylist filename hits: 0
- high-confidence scan hits: 0
- exported tree includes the template/community files and matches the expected list exactly
