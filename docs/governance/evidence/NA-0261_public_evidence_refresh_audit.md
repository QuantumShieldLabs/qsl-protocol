Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# NA-0261 Public Evidence Refresh Audit

## Objective

Refresh public and demo evidence summaries after the NA-0259 KT-negative demo
proof and NA-0260 attachment demo proof, without changing implementation,
website source, protocol, wire, crypto, branch-protection, public-safety, or
Cargo behavior.

## Stale Sources Found

- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` still listed KT-negative
  demo acceptance and attachment demo readiness as `NOT_READY`.
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md` still described KT-negative and
  attachment demo readiness as open review gaps.
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md` still told future website
  work to keep KT-negative demo readiness and attachment demo readiness visible
  as open gaps.
- `docs/demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md` still described
  KT-negative demo proof, attachment demo proof, and native package proof as
  open.
- `docs/demo/CROSS_HOST_PUBLIC_DEMO_REPRODUCIBILITY.md` still described
  KT-negative demo readiness and attachment demo readiness as open without
  distinguishing local demo proof from cross-host proof.
- `docs/demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md` and
  `docs/demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md` still pointed to a future
  public-doc refresh lane.

## Updates Made

- Updated the release-readiness map to classify KT-negative and attachment demo
  readiness as proven only for bounded non-production demo paths.
- Updated the external review package so reviewers can inspect NA-0259 and
  NA-0260 evidence while production KT, production attachments, and external
  review completion remain open.
- Updated the website handoff to feed future website work with current
  evidence links and safe gaps rather than stale KT/attachment-open language.
- Updated public demo touch-and-feel readiness with the latest demo markers,
  KT-negative proof, attachment proof, and NA-0258 native package/screenshot
  evidence.
- Updated cross-host reproducibility wording so local KT/attachment proof is
  not confused with cross-host/private-network proof.
- Updated KT-negative and attachment readiness docs to state that NA-0261
  refreshes public summaries without strengthening the underlying claims.

## Claims Strengthened

- The public demo includes a non-production KT-negative verifier proof through
  canonical verifier/vector checks and accepted-state no-mutation proof.
- The public demo includes a non-production attachment proof through encrypted
  descriptor/payload fetch/decrypt, descriptor-bound integrity validation,
  tampered-ciphertext rejection, and checked no plaintext/token leakage.
- Native desktop package and screenshot evidence exists for one provisioned
  Linux AppImage proof.

## Claims Still Forbidden

- Production-ready QSL protocol, production-ready Triple Ratchet, or proven true
  Triple Ratchet.
- Quantum-proof, anonymity, metadata-free, or untraceable messaging.
- Production KT deployment readiness or live qshield KT evidence ingestion.
- Production attachment readiness, qsl-server production relay readiness, or
  qsl-attachments production service readiness.
- Cross-host/private-network attachment proof.
- External cryptographic review completion.
- Production desktop release readiness.

## Evidence Links

- [KT-negative public demo readiness](../../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md)
- [Attachment public demo readiness](../../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md)
- [Public demo touch-and-feel readiness](../../demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md)
- [Cross-host public demo reproducibility](../../demo/CROSS_HOST_PUBLIC_DEMO_REPRODUCIBILITY.md)
- [Release readiness evidence map](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- [External review package](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [Website implementation handoff](../../public/WEBSITE_IMPLEMENTATION_HANDOFF.md)
- [NA-0259 KT-negative demo evidence](NA-0259_kt_negative_demo_readiness_audit.md)
- [NA-0260 attachment demo evidence](NA-0260_attachment_demo_readiness_audit.md)
- [NA-0258 native desktop package evidence](NA-0258_native_desktop_package_screenshot_audit.md)

## No Implementation Change Statement

NA-0261 changes documentation, governance evidence, and test planning only. It
does not edit protocol, runtime, crypto, demo implementation, qsl-server,
qsl-attachments, qsc-desktop, website source, workflows, public-safety
configuration, branch protection, Cargo manifests, or lockfiles.

## Known Remaining Gaps

- Production KT service/log operation and live qshield KT evidence ingestion.
- Cross-host KT-negative behavior.
- Cross-host/private-network attachment proof.
- qsl-server production relay hardening.
- qsl-attachments production service hardening.
- Metadata phase-2 work for identifier rotation, padding defaults,
  retention/purge, and broader sanitized-error coverage.
- External cryptographic review completion and reviewer findings.
- Production desktop release readiness, signed installers, and broader platform
  packaging evidence.
