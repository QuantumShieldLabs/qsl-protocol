# NA-0657 testplan — qsl-desktop bootstrap (D593, D-1280 + D-0001)

Row-per-proof; every row PASS at closeout. Raw outputs proof-root-only
(`/srv/qbuild/tmp/NA0657_qsl_desktop_bootstrap_20260719T011958Z/`).

| # | Check | Method | Result |
|---|---|---|---|
| 1 | qwork invariants (all) | startup.qsl-protocol.kv parse | PASS (OK/NA-0657/`131e1cdc`/clean/ready_count=1/top=NA-0657/shared yes) |
| 2 | Disk + mount gates | df / mountpoint | PASS (52% <95%; /backup/qsl mounted) |
| 3 | D-1280 next-and-absent pre-lane | decision_id_counter.py --expect 0 | PASS (count 0; D-1279 count 1) |
| 4 | Sole READY = NA-0657 at start | anchored `^Status: READY` grep -c | PASS (×1) |
| 5 | Main health on base `131e1cdc` | gh run list --commit | PASS (9/9 concluded success before first push; formal-ci wait honored) |
| 6 | STOP 1: qsl-desktop zero refs | git ls-remote (fresh) | PASS (empty output; NOT fired) |
| 7 | STOP 2: PVR enabled | gh api private-vulnerability-reporting | PASS (true; NOT fired) |
| 8 | Mirror unchanged: qsl-server ci.yml | sha256 vs drafting record | PASS (`594dbbce…` identical) |
| 9 | Mirror unchanged: qsl-server protection | gh api branches/main/protection | PASS (rust/strict/enforce_admins=true) |
| 10 | Byte-copy sources | sha256 spine CoC + LICENSE | PASS (`2cbf021e…` / `459cd3e0…`) |
| 11 | Appendix extraction fidelity ×9 | cmp directive-extract vs landed | PASS (ci.yml, README, SECURITY, CONTRIBUTING, CLAUDE.md, Cargo.toml, main.rs, .gitignore, NOTICE — all cmp 0) |
| 12 | DECISIONS.md fidelity | diff after single `<DATE>`→2026-07-19 | PASS (exact; opens D-0001) |
| 13 | LICENSE byte-copy | cmp vs spine | PASS (cmp 0) |
| 14 | CoC byte-copy | cmp vs spine | PASS (cmp 0) |
| 15 | SECURITY reporting section verbatim | awk-extract both, cmp | PASS (cmp exit 0) |
| 16 | Lock single-package assert | Cargo.lock inspect | PASS (only qsl-desktop 0.1.0; zero external deps) |
| 17 | cargo fmt --all -- --check | local, own target | PASS (clean) |
| 18 | cargo test -q | local | PASS (1 passed, 0 failed) |
| 19 | cargo clippy -q -- -D warnings | local | PASS (clean, exit 0) |
| 20 | cargo metadata --locked | local | PASS |
| 21 | Binary version line | cargo run -q | PASS (`qsl-desktop 0.1.0 (bootstrap placeholder; no application functionality)`) |
| 22 | Publication scans ×13 | added_line_publication_scan.py --new-file | PASS (0 overclaim/secret hits; 4 review-class = deliberate canonical links, dispositioned) |
| 23 | GH007 identity both commits | git log --format='%an <%ae>' | PASS (noreply identity ×2) |
| 24 | Root anchor = ONLY CLAUDE.md | git show --stat | PASS (1 file; direct push sanction spent) |
| 25 | ls-remote after anchor = exactly main | git ls-remote | PASS (single ref at `fc7c00d9…`) |
| 26 | PR #1 delta = exactly 12 files | git diff --name-only main..branch | PASS (+858 lines, the allow-list exactly) |
| 27 | `rust` check runs and passes on PR #1 | gh pr checks | PASS |
| 28 | Spine scope = exactly 6 paths | git diff --cached --name-only | PASS (evidence add -f staged-confirmed) |
| 29 | Queue flip + anchored ×0 | grep STATE / anchored count | PASS (`READY=NONE \| HIGHEST_NA=0657 \| HIGHEST_D=1280`; ×0) |
| 30 | D-1280 canonical ×1 post-append | decision_id_counter.py --expect 1 | PASS |
| 31 | git diff --check both repos | both worktrees | PASS (clean) |
| 32 | goal-lint local | synthesized event from LIVE spine PR body | PASS (OK first run) |
| 33 | Validation defaults (spine) | DOC-OPS-006 §4 set | PASS (audits/metadata/fmt-residue/scans — see as-built) |
