# Release Checklist (Public)

This repository publishes **draft protocol documentation** and **conformance vectors**.

## Before tagging
- [ ] `public-safety` is green on `main`.
- [ ] The repository status text still accurately says “DRAFT / not audited / not production-ready.”
- [ ] The README points to the intended “Latest tagged release”.

## Tag + GitHub Release
- [ ] Create an annotated tag (e.g., `v0.x.y-draft`) pointing to the intended commit on `main`.
- [ ] Create a GitHub Release with:
  - [ ] clear highlights
  - [ ] explicit non-goals and status disclaimers
  - [ ] security reporting pointer (`SECURITY.md`)
- [ ] Confirm the tag points to the expected commit SHA.

## After publishing
- [ ] Update any public pointers (README/org profile) if the “Latest tagged release” changed.
- [ ] Spot-check that the file tree still matches the intended public surface (docs + vectors only).
