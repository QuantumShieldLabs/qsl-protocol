# Project direction

QSL is working toward a usable private messenger backed by explicit protocol and implementation evidence. This public roadmap describes work areas; it is not a launch queue, delivery-date promise or statement that a milestone is complete.

| Work area | Intended outcome | Evidence needed |
|---|---|---|
| Safe local state | Durable vault operations, bounded secret lifetime and truthful recovery | Actual-caller failure/recovery tests and platform review |
| Identity and devices | Authenticated contact/device authority and explicit recovery | Accepted interfaces and meaningful multi-device behavior |
| Messaging engine | Bounded delivery, duplicate handling and exact retry | Integrated caller, fairness, persistence and negative-path evidence |
| Private transport | Minimize exposed routing/activity metadata with honest limits | Reviewed transport design and traffic evidence |
| Desktop messaging | Connect, verify, exchange text and recover through the application | Real application acceptance and truthful delivery states |
| Files | Complete file delivery with bounded storage and concurrency | Full failure, restart, publication and integration workloads |
| Release qualification | Support the exact platform and security claims made | Required independent review, CI, packaging and operational evidence |

The public branches and [evidence map](docs/public/RELEASE_READINESS_EVIDENCE_MAP.md) show what has actually been published. Technical decisions and exact acceptance requirements remain with their accepted documents. This summary changes none of them.

For participation, see [CONTRIBUTING.md](CONTRIBUTING.md). For current application capabilities, see [qsl-desktop](https://github.com/QuantumShieldLabs/qsl-desktop). Privacy research proposals are not implemented protections.
