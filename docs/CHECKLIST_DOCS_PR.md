# Docs PR Checklist (Public)

Use this checklist when proposing changes to **specifications**, **supporting documents**, or **conformance vectors**.

## Scope / intent
- [ ] My PR is limited to the intended scope (no unrelated formatting sweeps).
- [ ] I can explain the change in 1–2 sentences.

## Safety / hygiene
- [ ] No secrets or credentials are included (see `public-safety` CI).
- [ ] No internal endpoints, private emails, or non-public operational details are included.

## Documentation quality
- [ ] The change is written clearly and uses consistent terminology.
- [ ] Cross-references and filenames are correct.
- [ ] If I changed a rule/invariant, I updated the relevant rationale or decision record (see `DECISIONS.md` / `TRACEABILITY.md`).

## Vectors (if applicable)
- [ ] JSON vectors remain valid JSON and follow the existing schema patterns.
- [ ] Any new/updated vector files have a short note in the relevant README or index.
