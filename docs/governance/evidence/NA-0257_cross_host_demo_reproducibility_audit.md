Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0257 Cross-Host Demo Reproducibility Audit

Directive: QSL-DIR-2026-05-09-053 / NA-0257

## Objective

Validate the strongest feasible real-world-ish public demo reproducibility proof
on the current qbuild host, document the exact network assumptions, and preserve
non-production/fail-closed boundaries without changing protocol, crypto,
qsl-server, qsl-attachments, website, workflow, branch-protection, or Cargo
semantics.

## Baseline Proof

- Starting `origin/main`: `98534671f422`.
- Starting PR #763 state: merged as `98534671f422`.
- PR #762, #761, #760, #759, #758, #757, and #708: merged.
- PR #750 and #722: closed and unmerged.
- Branch protection: `public-safety` required with the expected protected
  contexts; force pushes and deletions disabled; admin enforcement enabled.
- Latest main `public-safety`: success on the starting main SHA.
- Queue proof after correcting the clean stale worktree ref: `READY_COUNT 1`,
  sole READY `NA-0257`.
- Decision proof: D-0480 existed once; D-0481 and D-0482 were absent; duplicate
  decision count was zero.

## Environment / Network Preflight

- Hostname: `ideacentre`.
- Disk watermark at start: `/srv/qbuild` total `468G`, used `37G`, free `407G`,
  used `9%`.
- Local LAN address observed: `192.168.1.117/24`.
- Tailscale interface address observed: `100.82.111.69/32`.
- Tailscale status: authenticated with visible online Linux peers and a DNS
  health warning.
- Narrow reachability checks to visible peers succeeded for ICMP and TCP/22 on
  two Linux Tailscale peers.
- Strict SSH command execution was not already configured: strict host-key
  checking rejected both tested peer IPs because no ED25519 host key was known.
- No host keys were added.
- No credentials were requested or printed.
- No firewall/router ports were opened.
- No Tailscale admin/API mutation was performed.

## Selected Proof Mode

Mode 2: LAN-style same-host multi-endpoint proof.

Reason: a stronger real two-host/Tailscale proof was not safe under the
directive because remote command execution was not already configured without
host-key mutation or credential prompts. Mode 2 still exercises a non-loopback
Tailscale interface bind with distinct Alice/Bob stores and separate qshield
processes.

## Artifact Package

Artifact directory:

```text
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/
```

Generated artifacts:

- `ARTIFACT_MANIFEST.txt`
- `mode2_tailscale_same_host_proof.log`
- `mode2_tailscale_same_host_proof.sh`

The artifact script is outside the repository and contains the exact command set
used for the proof. The committed evidence records summary markers only and does
not embed live token values.

## Commands Run

Preflight and proof commands included:

```bash
df -BG /srv/qbuild
git fetch --all --prune
git rev-parse origin/main
gh pr view 763 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
hostname
ip addr
ss -ltnp
command -v tailscale
tailscale status
ping -c 1 -W 2 <visible-tailscale-peer>
nc -zvw3 <visible-tailscale-peer> 22
ssh -o BatchMode=yes -o ConnectTimeout=5 -o StrictHostKeyChecking=yes <visible-tailscale-peer> 'hostname; pwd'
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/mode2_tailscale_same_host_proof.sh
```

The selected proof script ran:

```bash
cargo build -p qshield-cli --locked
cargo build -p refimpl_actor --locked
qshield relay serve --listen 100.82.111.69:33821 --allow-public --i-understand-this-is-unsafe
qshield init/register/establish/send/recv with separate Alice/Bob stores
curl --noproxy '*' negative checks against the Tailscale relay URL
```

## Positive Proof Outcome

The transcript proves qshield CLI/refimpl build, independent Alice/Bob store
initialization, Tailscale-interface relay health, authorized registration,
demo-only establishment, send, receive/decrypt, plaintext verification, and
sender verification.

Positive marker:

```text
NA0257_POSITIVE_SEND_RECEIVE_DECRYPT_OK
```

## Negative / Reject Outcome

The transcript proves missing auth reject, malformed reject, invalid relay ID
reject, replay reject, and no silent fallback success.

Negative markers:

```text
NA0257_NEGATIVE_AUTH_REJECT_OK
NA0257_NEGATIVE_MALFORMED_REJECT_OK
NA0257_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
NA0257_NEGATIVE_REPLAY_REJECT_OK
```

## No Secret Leakage Outcome

The proof generated a runtime relay token but did not print the live token into
the transcript. It injected sentinel material into rejected malformed input and
checked command output, reject bodies, and relay startup output before accepting
the run.

Leak-safe marker:

```text
NA0257_NO_SECRET_LEAK_OK
```

## Recovered Failure Evidence

- Failing command: initial helper parser on the stale local worktree after
  fetch.
- Classification: recoverable ref-selection issue. `origin/main` matched the
  required SHA, the worktree was clean, and the local `main` branch still tracked
  stale `mirror/main`.
- Corrective action: created `na-0257-cross-host-demo-reproducibility` from
  `origin/main` and reran queue/decision helpers.
- Final result: READY `NA-0257`, D-0480 once, D-0481/D-0482 absent, duplicate
  decisions zero.

- Failing command: `git show origin/main:NEXT_ACTIONS.md | python3 - <<'PY'`.
- Classification: recoverable command-shape issue; the heredoc consumed stdin.
- Corrective action: read `NEXT_ACTIONS.md` directly after switching to the
  correct `origin/main`-based branch.
- Final result: NA-0257 READY block quoted successfully.

- Failing artifact command: first generated Mode 2 proof script used incorrect
  shell quoting around `"$@"` in helper functions.
- Classification: recoverable command-shape issue before demo behavior was
  exercised.
- Corrective action: regenerated the artifact script with corrected command
  invocation and reran.
- Final result: Mode 2 proof passed with positive, negative, and leak-safe
  markers.

## Scope Conclusion

NA-0257 changed only docs/governance/evidence files and the rolling operations
journal. It did not change protocol/crypto state machines, qsl-server,
qsl-attachments, qsc-desktop implementation, website/external website,
workflows, public-safety configuration, branch protection, Cargo manifests,
Cargo lockfiles, production relay/service code, or Tailscale/firewall settings.

## No-Production-Overclaim Statement

This audit supports only non-production public demo reproducibility in a
LAN-style same-host/Tailscale-bind mode. It does not prove real two-host
execution, production relay readiness, production service deployment,
qsl-server/qsl-attachments readiness, KT-negative demo readiness, attachment
demo readiness, native desktop package proof, or release approval.

## Residual Gaps

- Real two-host/Tailscale proof remains open for an operator host pair with
  already configured safe command execution.
- Native desktop package and screenshot proof remain NA-0258 work.
- KT-negative public demo readiness remains open.
- Attachment demo readiness remains open.
- qsl-server and qsl-attachments production hardening remain out of scope.
- Metadata phase-2 and external review readiness remain separate work.

## Recommendations

1. Close NA-0257 only after PR checks and post-merge public-safety are green.
2. Restore NA-0258 for native desktop package/screenshot proof on a provisioned
   host.
3. Keep true two-host/Tailscale execution as an operator-run follow-up unless
   safe peer command execution is already configured.
4. Do not infer production readiness from this demo evidence.

## Related Evidence

- [Cross-host demo runbook](../../demo/CROSS_HOST_PUBLIC_DEMO_REPRODUCIBILITY.md)
- [Public demo readiness](../../demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md)
- [NA-0257 testplan](../../../tests/NA-0257_cross_host_demo_reproducibility_testplan.md)
