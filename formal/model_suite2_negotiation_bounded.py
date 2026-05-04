"""Bounded executable model checks for Suite-2 downgrade rejection.

Goals: G3, G4

This model is intentionally narrow and crypto-agnostic. It checks the
negotiation slice described by DOC-CAN-003 section 2:

- when both peers support Suite-2, a weaker committed suite is rejected;
- inconsistent capability or suite commitments are rejected; and
- rejected inputs do not mutate modeled accepted/durable negotiation state.

It does not model authentication, transcript hashing, AEAD, KDFs, or the
non-Suite-2 fallback lanes.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Iterable, Optional, Tuple

SUITE2 = "qsp-v5-suite2"
SUITE1B = "qsp-v4.3-suite1b"
UNKNOWN = "unknown-suite"
COMMITTED_SUITES = (SUITE2, SUITE1B, UNKNOWN)


@dataclass(frozen=True, slots=True)
class NegotiationState:
    accepted_suite: Optional[str] = None
    accepted_capability_commitment: Optional[Tuple[bool, bool]] = None
    durable_accept_count: int = 0

    def assert_invariants(self) -> None:
        assert self.durable_accept_count >= 0
        if self.accepted_suite is None:
            assert self.accepted_capability_commitment is None
            assert self.durable_accept_count == 0
        else:
            assert self.accepted_suite == SUITE2
            assert self.accepted_capability_commitment == (True, True)
            assert self.durable_accept_count > 0
        assert _restore_state(_state_snapshot(self)) == self


StateSnapshot = Tuple[Optional[str], Optional[Tuple[bool, bool]], int]


def _state_snapshot(state: NegotiationState) -> StateSnapshot:
    return (
        state.accepted_suite,
        state.accepted_capability_commitment,
        state.durable_accept_count,
    )


def _restore_state(snapshot: StateSnapshot) -> NegotiationState:
    accepted_suite, accepted_capability_commitment, durable_accept_count = snapshot
    return NegotiationState(
        accepted_suite=accepted_suite,
        accepted_capability_commitment=accepted_capability_commitment,
        durable_accept_count=durable_accept_count,
    )


@dataclass(frozen=True, slots=True)
class NegotiationAttempt:
    local_supports_suite2: bool
    peer_supports_suite2: bool
    committed_local_supports_suite2: bool
    committed_peer_supports_suite2: bool
    negotiated_suite: str
    local_transcript_suite: str
    peer_transcript_suite: str


@dataclass(frozen=True, slots=True)
class Outcome:
    accepted: bool
    reason: str


def _iter_attempts() -> Iterable[NegotiationAttempt]:
    # NA-0249 covers the downgrade-resistant Suite-2-required case where both
    # implementations support Suite-2. Other policy/fallback lanes are outside
    # this model's claim boundary.
    local_supports_suite2 = True
    peer_supports_suite2 = True
    for committed_local in (False, True):
        for committed_peer in (False, True):
            for negotiated_suite in COMMITTED_SUITES:
                for local_transcript_suite in COMMITTED_SUITES:
                    for peer_transcript_suite in COMMITTED_SUITES:
                        yield NegotiationAttempt(
                            local_supports_suite2=local_supports_suite2,
                            peer_supports_suite2=peer_supports_suite2,
                            committed_local_supports_suite2=committed_local,
                            committed_peer_supports_suite2=committed_peer,
                            negotiated_suite=negotiated_suite,
                            local_transcript_suite=local_transcript_suite,
                            peer_transcript_suite=peer_transcript_suite,
                        )


def _reject_reason(attempt: NegotiationAttempt) -> Optional[str]:
    if not attempt.local_supports_suite2:
        return "REJECT_S2_LOCAL_UNSUPPORTED"
    if not attempt.peer_supports_suite2:
        return "REJECT_S2_PEER_UNSUPPORTED"

    committed_caps = (
        attempt.committed_local_supports_suite2,
        attempt.committed_peer_supports_suite2,
    )
    actual_caps = (attempt.local_supports_suite2, attempt.peer_supports_suite2)
    if committed_caps != actual_caps:
        return "REJECT_S2_CAPABILITY_COMMITMENT_MISMATCH"

    if (
        attempt.local_transcript_suite != attempt.negotiated_suite
        or attempt.peer_transcript_suite != attempt.negotiated_suite
    ):
        return "REJECT_S2_AD_MISMATCH"

    if attempt.negotiated_suite != SUITE2:
        return "REJECT_S2_SUITE_MISMATCH"

    return None


def apply_negotiation(
    state: NegotiationState, attempt: NegotiationAttempt
) -> Tuple[NegotiationState, Outcome]:
    state.assert_invariants()
    before = _state_snapshot(state)
    reason = _reject_reason(attempt)
    if reason is not None:
        # P2: reject => no accepted/durable negotiation state mutation.
        assert _state_snapshot(state) == before
        return state, Outcome(accepted=False, reason=reason)

    accepted = NegotiationState(
        accepted_suite=SUITE2,
        accepted_capability_commitment=(True, True),
        durable_accept_count=state.durable_accept_count + 1,
    )
    accepted.assert_invariants()
    assert accepted.accepted_suite == SUITE2
    assert accepted.accepted_capability_commitment == (True, True)
    return accepted, Outcome(accepted=True, reason="ACCEPT_S2")


def check_suite2_negotiation_model() -> Dict[str, int]:
    """Run the bounded Suite-2 negotiation checks.

    Returns model statistics; raises AssertionError on any invariant violation.
    """

    empty = NegotiationState()
    empty.assert_invariants()

    valid_attempt = NegotiationAttempt(
        local_supports_suite2=True,
        peer_supports_suite2=True,
        committed_local_supports_suite2=True,
        committed_peer_supports_suite2=True,
        negotiated_suite=SUITE2,
        local_transcript_suite=SUITE2,
        peer_transcript_suite=SUITE2,
    )
    accepted_state, accepted_outcome = apply_negotiation(empty, valid_attempt)
    assert accepted_outcome == Outcome(accepted=True, reason="ACCEPT_S2")
    assert _state_snapshot(empty) != _state_snapshot(accepted_state)

    attempts = tuple(_iter_attempts())
    assert valid_attempt in attempts

    downgrade_rejects = 0
    commitment_rejects = 0
    ad_mismatch_rejects = 0
    no_mutation_assertions = 0
    accepted = 0
    rejected = 0

    for state in (empty, accepted_state):
        for attempt in attempts:
            before = _state_snapshot(state)
            next_state, outcome = apply_negotiation(state, attempt)
            repeated_state, repeated_outcome = apply_negotiation(state, attempt)

            if outcome.accepted:
                assert attempt == valid_attempt
                assert outcome.reason == "ACCEPT_S2"
                accepted += 1
                continue

            rejected += 1
            assert repeated_outcome == outcome
            assert _state_snapshot(next_state) == before
            assert _state_snapshot(repeated_state) == before
            no_mutation_assertions += 2

            if outcome.reason == "REJECT_S2_SUITE_MISMATCH":
                assert attempt.negotiated_suite != SUITE2
                downgrade_rejects += 1
            elif outcome.reason == "REJECT_S2_CAPABILITY_COMMITMENT_MISMATCH":
                commitment_rejects += 1
            elif outcome.reason == "REJECT_S2_AD_MISMATCH":
                ad_mismatch_rejects += 1

    assert downgrade_rejects > 0
    assert commitment_rejects > 0
    assert ad_mismatch_rejects > 0
    assert rejected > accepted
    return {
        "attempts": len(attempts),
        "accepted": accepted,
        "rejected": rejected,
        "downgrade_rejects": downgrade_rejects,
        "capability_commitment_rejects": commitment_rejects,
        "ad_mismatch_rejects": ad_mismatch_rejects,
        "no_mutation_assertions": no_mutation_assertions,
    }
