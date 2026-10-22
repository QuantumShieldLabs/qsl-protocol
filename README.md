# QSL Protocol

**Specifications, vectors and Rust implementations for research into post-quantum messaging.**

QSL explores hybrid message-key derivation, sparse post-quantum key agreement and explicit failure/recovery behavior. This repository contains the protocol drafts, conformance inputs, reference implementation and `qsc` client core used by the desktop prototype.

> [!WARNING]
> Research stage. Not independently audited or production-ready. Do not use QSL for safety-critical communications.

## Choose a starting point

| You want to… | Read |
|---|---|
| Understand the design | [Canonical specifications](docs/canonical/) |
| Assess the evidence | [Public evidence guide](docs/public/INDEX.md) and [claim boundary](docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) |
| Inspect an implementation | [Rust reference implementation](tools/refimpl/) and [client core](qsl/qsl-client/qsc/) |
| Review expected behavior | [Suite-2 conformance inputs](inputs/suite2/) |
| Contribute | [Contribution guide](CONTRIBUTING.md) |
| Report a sensitive finding | [Security policy](SECURITY.md) |

## Design at a glance

Suite-2 specifies classical and post-quantum message-key chains combined for application encryption. Sparse Continuous Key Agreement supplies new post-quantum material without requiring a KEM exchange for every message. The specifications define the key schedule, authentication, ordering and rejection rules.

| Design area | Review question |
|---|---|
| Hybrid key derivation | Which secrets contribute to each message key, and under which compromise assumptions? |
| Ratchet and SCKA transitions | What authenticates an epoch change, and what happens after loss or reordering? |
| Durable state | Which commit authorizes external output, and which bytes must a retry preserve? |
| Untrusted input | What bounds parsing, skipped keys, work and retained state? |
| Metadata | Which identifiers, lengths, timings and access patterns remain visible? |

Start with DOC-CAN-003 and DOC-CAN-004 in the canonical directory. Architecture terminology describes a construction; it does not establish a complete security proof or release readiness.

## Evidence and maturity

The repository publishes vectors, negative-path tests and bounded formal/model work. The [release-readiness map](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md) records their scope and gaps; [traceability](TRACEABILITY.md) connects requirements to artifacts.

Read evidence against its named revision, configuration and assumptions. A passing test or bounded symbolic result does not establish computational composition, implementation equivalence, side-channel freedom or production safety. Historical reports are retained with their original scope.

The desktop prototype has a separate feature status. A core-library or CLI experiment is not evidence that the desktop can perform the same operation. See [qsl-desktop](https://github.com/QuantumShieldLabs/qsl-desktop).

## Repository map

```text
docs/canonical/      Protocol specifications
docs/public/         Evidence navigation and claim boundaries
inputs/suite2/       Conformance inputs
tools/refimpl/       Rust reference implementation
qsl/qsl-client/qsc/  Client core and laboratory CLI
formal/             Bounded models and analysis
```

`qsc` is a laboratory and operator interface. Its commands and test affordances are not the desktop product specification. Follow the [reviewer and developer reading paths](START_HERE.md) before running experiments.

## Contribute to the evidence

We welcome specification ambiguities, reproducible bugs, missing negative cases and independent implementation work. Include the affected revision and enough context to reproduce a result. Discuss protocol or implementation changes before starting a large PR; accepted interfaces and review requirements still apply.

See [CONTRIBUTING.md](CONTRIBUTING.md), [SUPPORT.md](SUPPORT.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Report vulnerabilities privately through [SECURITY.md](SECURITY.md).

## Related components

[Desktop prototype](https://github.com/QuantumShieldLabs/qsl-desktop) · [Relay](https://github.com/QuantumShieldLabs/qsl-server) · [Encrypted-object service](https://github.com/QuantumShieldLabs/qsl-attachments)

## License

`AGPL-3.0-only`. See [LICENSE](LICENSE). Service or support offerings do not change the license on this source.
