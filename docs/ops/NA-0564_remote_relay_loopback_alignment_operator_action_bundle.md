DO NOT RUN UNTIL DIRECTOR REVIEW

Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0564 Remote Relay Loopback Alignment Operator Action Bundle

## Bundle Status

NA-0564 does not authorize an operator alignment action.

Status:
`candidate-proof-only; no operator action authorized by NA-0564`.

Reason: the non-secret remote probe identified a coarse candidate listener
class with count class `one`, but expected-target alignment stayed `unknown`
because the expected target value was not disclosed to the probe. Candidate
confirmation is required before any action bundle can be executed.

## Preconditions

- Director reviews D-1118 and confirms the selected NA-0565 candidate
  confirmation proof lane.
- Operator performs any private review locally and does not paste private
  values into repository docs.
- No command in this document is runnable as written.
- Placeholder tokens must be resolved privately by the operator or service
  owner before any future action.

## Confirm Local Service / Listener Ownership Privately

Proof requirement only:

- confirm whether the candidate listener class belongs to the intended relay
  service;
- keep process identity private;
- keep private port values private;
- report only coarse yes/no/unknown fields to the repository.

Placeholder-only operator note:

```text
OPERATOR_LOCAL_PRIVATE_NOT_FOR_REPO: <confirm service/listener ownership using local private tooling>
```

## Confirm the Intended Tunnel Target Privately

Proof requirement only:

- confirm the intended tunnel target privately;
- compare it to the candidate listener privately;
- do not disclose endpoint values, private hostnames, private IPs, topology, or
  private port values.

Placeholder-only operator note:

```text
OPERATOR_LOCAL_PRIVATE_NOT_FOR_REPO: <confirm intended tunnel target and authorized forwarding target privately>
```

## Capture Rollback State Privately

Proof requirement only:

- capture rollback state before any future action;
- store rollback proof privately;
- publish only whether rollback was captured.

Placeholder-only operator note:

```text
OPERATOR_LOCAL_PRIVATE_NOT_FOR_REPO: <capture rollback state privately before changes>
```

## Alignment Options For Future Review Only

No option is authorized by NA-0564.

Future Director-reviewed options may include one of these private actions after
candidate confirmation:

- align the service listener target;
- align the authorized forwarding target;
- align the tunnel target;
- change the workflow access model.

Any future action must be represented as a new directive or operator-owned
private step. Codex must not perform the action unless a later directive
explicitly authorizes an exact non-mutating proof or review step.

## Restart Or Reload Boundary

Restart/reload is not authorized by NA-0564.

Proof requirement only:

- service owner privately decides whether restart/reload is necessary;
- repository proof may report only `service_restart_performed: yes/no`.

## Post-Action Non-Secret Proof

Future proof requirement only:

- expected target alignment after action: aligned/mismatched/unknown;
- listener presence after action: yes/no/unknown;
- private values disclosed: no;
- redaction review: pass/fail.

## Preserve Rollback Proof

Future proof requirement only:

- rollback captured: yes/no;
- rollback proof stored privately: yes/no;
- rollback values disclosed to repository: no.

## Stop Conditions

Stop before action or publication if any output contains endpoint values,
private hosts/IPs/topology, private port values, route-token/capability values,
bearer values, Authorization headers, payloads, response bodies, process
identity, raw authorized key material, public SSH key material, private keys,
secret environment values, Cloudflare tokens, API keys, raw logs, raw
artifacts, or private material.

Stop if the candidate listener class cannot be confirmed without exposing
private material.

## Director Proof Summary Template

```text
action_performed: yes/no/declined
action_owner: operator/service-owner
rollback_captured: yes/no
expected_target_alignment_after_action: aligned/mismatched/unknown
listener_presence_after_action: yes/no/unknown
tunnel_target_reviewed: yes/no
service_restart_performed: yes/no
authorized_keys_changed: yes/no
service_config_changed: yes/no
private_values_disclosed: no
endpoint_values_disclosed: no
token_values_disclosed: no
process_identity_disclosed: no
response_body_disclosed: no
redaction_review: pass/fail
```
