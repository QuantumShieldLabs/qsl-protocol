# Contributing

Thank you for helping improve QSL. This repository covers specifications, cryptographic composition, conformance vectors, the reference implementation and client core.

## Choose the right route

| Contribution | Start with |
|---|---|
| Security-sensitive finding | [SECURITY.md](SECURITY.md); keep details private |
| Reproducible bug or specification ambiguity | An issue with the revision, expected behavior and minimal synthetic reproduction |
| Documentation or evidence correction | Identify the statement, source and proposed replacement |
| Substantive code, protocol or interface change | Discuss the problem and scope with maintainers before implementation |

External review, negative tests, conformance gaps and independent implementations are valuable. Maintainers coordinate accepted implementation work and required reviews; an issue is not permission to change an unsettled security contract.

## Prepare a focused change

Keep one coherent outcome per PR. Explain the problem, resulting behavior, validation and remaining limits. Preserve accepted technical requirements and historical evidence. Protocol or persisted-format changes need their affected vectors, traceability and independent review.

Use synthetic data. Remove credentials, private endpoints and sensitive material from logs, fixtures and screenshots. Do not reduce tests, bypass a scanner or change a baseline to make a check pass.

## Validate against the repository

Use the [workflow directory](.github/workflows/) for exact validation recipes. Choose the relevant vector, model, core or documentation checks with maintainers. The CI workflows carry their exact commands and configurations; some require fixtures or event inputs and are not generic quickstart commands.

All required repository checks remain binding. State exactly what you ran and what remains unverified; a skipped check is not a pass. Maintainers verify the current protection rules and review requirements at publication. Reviewed changes use merge commits; no merge or release is implied by opening a PR.

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community expectations. Security review and support availability remain subject to the project's research-stage status.
