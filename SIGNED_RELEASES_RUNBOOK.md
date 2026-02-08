# Signed Releases Runbook

This runbook provides user instructions for creating and verifying signed tags and release checksums.

No keys are created or stored by this repository.

## Prerequisites

- Git with GPG or SSH signing configured locally.
- Maintainer permissions for the repository.
- Clean working tree before tagging.

## 1) Create a signed tag

```bash
git checkout main
git pull --ff-only
git status --porcelain
git tag -s vX.Y.Z -m "qsl-protocol vX.Y.Z"
git tag -v vX.Y.Z
git push origin vX.Y.Z
```

If your environment uses SSH signing for tags:

```bash
git tag -s vX.Y.Z -m "qsl-protocol vX.Y.Z"
git tag -v vX.Y.Z
```

## 2) Build checksums for release artifacts

```bash
sha256sum <artifact-1> <artifact-2> > SHA256SUMS
sha256sum -c SHA256SUMS
```

Optional detached signature for checksum manifest:

```bash
gpg --armor --detach-sign SHA256SUMS
gpg --verify SHA256SUMS.asc SHA256SUMS
```

## 3) Consumer verification steps

```bash
git fetch --tags origin
git tag -v vX.Y.Z
sha256sum -c SHA256SUMS
```

Accept a release only when:
- the tag signature verifies successfully, and
- checksums verify for downloaded artifacts, and
- commit/run evidence is traceable via `TRACEABILITY.md`.

## 4) Provenance linkage checklist

- Record the release tag and commit SHA in release notes.
- Link corresponding CI runs and evidence from `TRACEABILITY.md`.
- Do not publish binaries that cannot be linked to signed tag + commit + CI evidence.
