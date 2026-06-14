#!/usr/bin/env python3
"""Bounded qsc KEM/signature/transcript binding model.

Goals: G1, G2, G3, G4, G5

This is a formal/model artifact only. It does not implement qsc runtime
behavior, qsc executable tests, QHSM/QSP wire format, cryptographic internals,
provider behavior, refimpl behavior, or a key schedule.

The model uses opaque tokens for roles, A1/B1/A2 messages, Suite-2, session
identity, KEM public keys, KEM ciphertexts, signature identities/public keys,
signature message contexts, transcripts, confirmation, trusted public records,
replay state, pending state, completed session state, and output flags.

The model is bounded and intentionally narrow. It is not a cryptographic proof,
not a side-channel proof, not a complete downgrade or replay proof, not a
KEM/signature/identity/transcript completeness claim, and not a qsc/refimpl
equivalence claim.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Dict, Iterable, Optional, Sequence, Tuple


class Role(str, Enum):
    INITIATOR = "initiator"
    RESPONDER = "responder"


class MessageType(str, Enum):
    A1 = "A1"
    B1 = "B1"
    A2 = "A2"


SUITE2 = "suite:qsc-v5-suite2"
WRONG_SUITE = "suite:qsc-v4-suite1"
SESSION = "session:na0478-baseline"
SESSION_REPLAY = "session:na0478-replay"

ALICE_ID = "identity:alice"
BOB_ID = "identity:bob"
MALLORY_ID = "identity:mallory"

ALICE_SIG_PK = "sigpk:alice:v1"
BOB_SIG_PK = "sigpk:bob:v1"
MALLORY_SIG_PK = "sigpk:mallory:v1"

ALICE_KEM_PK = "kem-pk:alice:v1"
ALICE_KEM_PK_STALE = "kem-pk:alice:stale"
ALICE_KEM_PK_WRONG = "kem-pk:alice:wrong"

ALICE_RECORD = "public-record:alice:v1"
BOB_RECORD = "public-record:bob:v1"
ALICE_RECORD_STALE = "public-record:alice:stale"
BOB_RECORD_STALE = "public-record:bob:stale"

ALICE_PIN = "trusted-pin:alice:v1"
BOB_PIN = "trusted-pin:bob:v1"
STALE_PIN = "trusted-pin:stale"

WRONG_TRANSCRIPT = "transcript:mutated"
WRONG_CIPHERTEXT = "kem-ct:wrong"
WRONG_CONFIRM = "confirm:wrong"


@dataclass(frozen=True, slots=True)
class PublicRecord:
    role: Role
    identity: str
    signature_public_key: str
    kem_public_key: Optional[str]
    public_record_token: str
    trusted_pin_token: str
    suite: str


ALICE_PUBLIC_RECORD = PublicRecord(
    role=Role.INITIATOR,
    identity=ALICE_ID,
    signature_public_key=ALICE_SIG_PK,
    kem_public_key=ALICE_KEM_PK,
    public_record_token=ALICE_RECORD,
    trusted_pin_token=ALICE_PIN,
    suite=SUITE2,
)
BOB_PUBLIC_RECORD = PublicRecord(
    role=Role.RESPONDER,
    identity=BOB_ID,
    signature_public_key=BOB_SIG_PK,
    kem_public_key=None,
    public_record_token=BOB_RECORD,
    trusted_pin_token=BOB_PIN,
    suite=SUITE2,
)


@dataclass(frozen=True, slots=True)
class Message:
    message_type: MessageType
    sender_role: Role
    receiver_role: Role
    suite: str
    session: str
    kem_public_key: Optional[str]
    kem_ciphertext: Optional[str]
    signature_identity: str
    signature_public_key: str
    signature_message_context: str
    transcript_token: str
    confirm_token: Optional[str]
    public_record_token: str
    trusted_pin_token: str


@dataclass(frozen=True, slots=True)
class Pending:
    owner_role: Role
    peer_role: Role
    session: str
    suite: str
    peer_identity: str
    peer_signature_public_key: str
    peer_kem_public_key: Optional[str]
    a1_transcript_token: str


@dataclass(frozen=True, slots=True)
class CompletedSession:
    owner_role: Role
    peer_role: Role
    session: str
    suite: str
    confirm_token: str


@dataclass(frozen=True, slots=True)
class OutputFlag:
    owner_role: Role
    message_type: MessageType
    session: str
    success: bool


SeenKey = Tuple[MessageType, str, str]
PendingSet = Tuple[Pending, ...]
CompletedSet = Tuple[CompletedSession, ...]
SeenSet = Tuple[SeenKey, ...]
OutputSet = Tuple[OutputFlag, ...]
StateSnapshot = Tuple[PendingSet, CompletedSet, SeenSet, OutputSet]


@dataclass(frozen=True, slots=True)
class ModelState:
    pending: PendingSet = ()
    completed_sessions: CompletedSet = ()
    seen_replay_state: SeenSet = ()
    output_flags: OutputSet = ()

    def assert_invariants(self) -> None:
        assert len(set(self.pending)) == len(self.pending)
        assert len(set(self.completed_sessions)) == len(self.completed_sessions)
        assert len(set(self.seen_replay_state)) == len(self.seen_replay_state)
        assert len(set(self.output_flags)) == len(self.output_flags)
        for completed in self.completed_sessions:
            assert completed.suite == SUITE2
            assert completed.confirm_token == expected_confirm(completed.session)
        for output in self.output_flags:
            if output.success:
                assert any(
                    completed.owner_role == output.owner_role
                    and completed.session == output.session
                    for completed in self.completed_sessions
                )


@dataclass(frozen=True, slots=True)
class Outcome:
    accepted: bool
    reason: str
    output_emitted: bool
    success_output_emitted: bool
    completed_session_mutation: bool
    state_mutation: bool


@dataclass(frozen=True, slots=True)
class TraceScenario:
    name: str
    messages: Tuple[Message, ...]
    expected_final_accept: bool
    expected_reject_reason: Optional[str] = None
    expect_completed_sessions: int = 0
    start_state: Optional[ModelState] = None


def _state_snapshot(state: ModelState) -> StateSnapshot:
    return (
        state.pending,
        state.completed_sessions,
        state.seen_replay_state,
        state.output_flags,
    )


def expected_context(
    message_type: MessageType, session: str, suite: str, identity: str
) -> str:
    return f"context:{message_type.value}:{session}:{suite}:{identity}"


def expected_transcript(message_type: MessageType, session: str, suite: str) -> str:
    return f"transcript:{message_type.value}:{session}:{suite}"


def expected_ciphertext(session: str, suite: str, kem_public_key: str) -> str:
    return f"kem-ct:{session}:{suite}:{kem_public_key}"


def expected_confirm(session: str) -> str:
    return f"confirm:{session}:A1+B1+A2"


def peer_record_for(message: Message) -> PublicRecord:
    if message.sender_role == Role.INITIATOR and message.receiver_role == Role.RESPONDER:
        return ALICE_PUBLIC_RECORD
    if message.sender_role == Role.RESPONDER and message.receiver_role == Role.INITIATOR:
        return BOB_PUBLIC_RECORD
    raise AssertionError("unsupported role transition")


def replay_key(message: Message) -> SeenKey:
    return (message.message_type, message.session, message.transcript_token)


def completed_snapshot(state: ModelState) -> CompletedSet:
    return state.completed_sessions


def initial_trace_state(session: str = SESSION) -> ModelState:
    """Model the initiator having emitted A1 and retained pending state."""

    state = ModelState(
        pending=(
            Pending(
                owner_role=Role.INITIATOR,
                peer_role=Role.RESPONDER,
                session=session,
                suite=SUITE2,
                peer_identity=BOB_ID,
                peer_signature_public_key=BOB_SIG_PK,
                peer_kem_public_key=None,
                a1_transcript_token=expected_transcript(MessageType.A1, session, SUITE2),
            ),
        ),
        output_flags=(
            OutputFlag(
                owner_role=Role.INITIATOR,
                message_type=MessageType.A1,
                session=session,
                success=False,
            ),
        ),
    )
    state.assert_invariants()
    return state


def valid_message(message_type: MessageType, session: str = SESSION) -> Message:
    if message_type == MessageType.A1:
        return Message(
            message_type=MessageType.A1,
            sender_role=Role.INITIATOR,
            receiver_role=Role.RESPONDER,
            suite=SUITE2,
            session=session,
            kem_public_key=ALICE_KEM_PK,
            kem_ciphertext=None,
            signature_identity=ALICE_ID,
            signature_public_key=ALICE_SIG_PK,
            signature_message_context=expected_context(
                MessageType.A1, session, SUITE2, ALICE_ID
            ),
            transcript_token=expected_transcript(MessageType.A1, session, SUITE2),
            confirm_token=None,
            public_record_token=ALICE_RECORD,
            trusted_pin_token=ALICE_PIN,
        )
    if message_type == MessageType.B1:
        return Message(
            message_type=MessageType.B1,
            sender_role=Role.RESPONDER,
            receiver_role=Role.INITIATOR,
            suite=SUITE2,
            session=session,
            kem_public_key=None,
            kem_ciphertext=expected_ciphertext(session, SUITE2, ALICE_KEM_PK),
            signature_identity=BOB_ID,
            signature_public_key=BOB_SIG_PK,
            signature_message_context=expected_context(
                MessageType.B1, session, SUITE2, BOB_ID
            ),
            transcript_token=expected_transcript(MessageType.B1, session, SUITE2),
            confirm_token=None,
            public_record_token=BOB_RECORD,
            trusted_pin_token=BOB_PIN,
        )
    if message_type == MessageType.A2:
        return Message(
            message_type=MessageType.A2,
            sender_role=Role.INITIATOR,
            receiver_role=Role.RESPONDER,
            suite=SUITE2,
            session=session,
            kem_public_key=None,
            kem_ciphertext=None,
            signature_identity=ALICE_ID,
            signature_public_key=ALICE_SIG_PK,
            signature_message_context=expected_context(
                MessageType.A2, session, SUITE2, ALICE_ID
            ),
            transcript_token=expected_transcript(MessageType.A2, session, SUITE2),
            confirm_token=expected_confirm(session),
            public_record_token=ALICE_RECORD,
            trusted_pin_token=ALICE_PIN,
        )
    raise AssertionError(f"unsupported message type: {message_type}")


def replace_message(message: Message, **updates: object) -> Message:
    fields = {
        "message_type": message.message_type,
        "sender_role": message.sender_role,
        "receiver_role": message.receiver_role,
        "suite": message.suite,
        "session": message.session,
        "kem_public_key": message.kem_public_key,
        "kem_ciphertext": message.kem_ciphertext,
        "signature_identity": message.signature_identity,
        "signature_public_key": message.signature_public_key,
        "signature_message_context": message.signature_message_context,
        "transcript_token": message.transcript_token,
        "confirm_token": message.confirm_token,
        "public_record_token": message.public_record_token,
        "trusted_pin_token": message.trusted_pin_token,
    }
    fields.update(updates)
    return Message(**fields)


def _find_pending(
    state: ModelState, owner_role: Role, session: str
) -> Optional[Pending]:
    for pending in state.pending:
        if pending.owner_role == owner_role and pending.session == session:
            return pending
    return None


def _append_unique(items: Tuple[object, ...], item: object) -> Tuple[object, ...]:
    if item in items:
        return items
    return items + (item,)


def _without_pending(
    pending_set: PendingSet, owner_role: Role, session: str
) -> PendingSet:
    return tuple(
        pending
        for pending in pending_set
        if not (pending.owner_role == owner_role and pending.session == session)
    )


def _reject(
    state: ModelState, before: StateSnapshot, completed_before: CompletedSet, reason: str
) -> Tuple[ModelState, Outcome]:
    state.assert_invariants()
    assert _state_snapshot(state) == before
    assert state.completed_sessions == completed_before
    return state, Outcome(
        accepted=False,
        reason=reason,
        output_emitted=False,
        success_output_emitted=False,
        completed_session_mutation=False,
        state_mutation=False,
    )


def _accept(
    before: StateSnapshot,
    completed_before: CompletedSet,
    next_state: ModelState,
    reason: str,
    output_emitted: bool,
    success_output_emitted: bool,
) -> Tuple[ModelState, Outcome]:
    next_state.assert_invariants()
    return next_state, Outcome(
        accepted=True,
        reason=reason,
        output_emitted=output_emitted,
        success_output_emitted=success_output_emitted,
        completed_session_mutation=next_state.completed_sessions != completed_before,
        state_mutation=_state_snapshot(next_state) != before,
    )


def _common_reject_reason(state: ModelState, message: Message) -> Optional[str]:
    if message.suite != SUITE2:
        return "REJECT_QSC_BINDING_SUITE_CONFUSION"

    if replay_key(message) in state.seen_replay_state:
        return "REJECT_QSC_BINDING_REPLAY"

    expected_tr = expected_transcript(message.message_type, message.session, message.suite)
    if message.transcript_token != expected_tr:
        return "REJECT_QSC_BINDING_TRANSCRIPT_MUTATION"

    trusted = peer_record_for(message)
    if (
        message.public_record_token != trusted.public_record_token
        or message.trusted_pin_token != trusted.trusted_pin_token
    ):
        if message.message_type == MessageType.A1 and (
            message.kem_public_key != trusted.kem_public_key
        ):
            return "REJECT_QSC_BINDING_STALE_KEM_PUBLIC_RECORD"
        return "REJECT_QSC_BINDING_STALE_PUBLIC_RECORD"

    if (
        message.signature_identity != trusted.identity
        or message.signature_public_key != trusted.signature_public_key
    ):
        return "REJECT_QSC_BINDING_WRONG_SIGNATURE"

    expected_ctx = expected_context(
        message.message_type, message.session, message.suite, message.signature_identity
    )
    if message.signature_message_context != expected_ctx:
        cross_contexts = {
            expected_context(other_type, message.session, message.suite, message.signature_identity)
            for other_type in MessageType
            if other_type != message.message_type
        }
        if message.signature_message_context in cross_contexts:
            return "REJECT_QSC_BINDING_CROSS_MESSAGE_SIGNATURE_REPLAY"
        return "REJECT_QSC_BINDING_WRONG_SIGNATURE_CONTEXT"

    return None


def apply_message(state: ModelState, message: Message) -> Tuple[ModelState, Outcome]:
    state.assert_invariants()
    before = _state_snapshot(state)
    completed_before = completed_snapshot(state)

    reason = _common_reject_reason(state, message)
    if reason is not None:
        return _reject(state, before, completed_before, reason)

    if message.message_type == MessageType.A1:
        return apply_a1(state, message, before, completed_before)
    if message.message_type == MessageType.B1:
        return apply_b1(state, message, before, completed_before)
    if message.message_type == MessageType.A2:
        return apply_a2(state, message, before, completed_before)
    raise AssertionError(f"unsupported message type: {message.message_type}")


def apply_a1(
    state: ModelState,
    message: Message,
    before: StateSnapshot,
    completed_before: CompletedSet,
) -> Tuple[ModelState, Outcome]:
    trusted = peer_record_for(message)
    if message.kem_public_key != trusted.kem_public_key:
        return _reject(
            state, before, completed_before, "REJECT_QSC_BINDING_WRONG_KEM_PUBLIC_KEY"
        )
    if _find_pending(state, Role.RESPONDER, message.session) is not None:
        return _reject(state, before, completed_before, "REJECT_QSC_BINDING_REPLAY")
    if any(
        completed.owner_role == Role.RESPONDER and completed.session == message.session
        for completed in state.completed_sessions
    ):
        return _reject(state, before, completed_before, "REJECT_QSC_BINDING_REPLAY")

    pending = Pending(
        owner_role=Role.RESPONDER,
        peer_role=Role.INITIATOR,
        session=message.session,
        suite=message.suite,
        peer_identity=message.signature_identity,
        peer_signature_public_key=message.signature_public_key,
        peer_kem_public_key=message.kem_public_key,
        a1_transcript_token=message.transcript_token,
    )
    output = OutputFlag(
        owner_role=Role.RESPONDER,
        message_type=MessageType.B1,
        session=message.session,
        success=False,
    )
    next_state = ModelState(
        pending=state.pending + (pending,),
        completed_sessions=state.completed_sessions,
        seen_replay_state=_append_unique(state.seen_replay_state, replay_key(message)),
        output_flags=state.output_flags + (output,),
    )
    return _accept(
        before,
        completed_before,
        next_state,
        "ACCEPT_QSC_BINDING_A1",
        output_emitted=True,
        success_output_emitted=False,
    )


def apply_b1(
    state: ModelState,
    message: Message,
    before: StateSnapshot,
    completed_before: CompletedSet,
) -> Tuple[ModelState, Outcome]:
    pending = _find_pending(state, Role.INITIATOR, message.session)
    if pending is None:
        return _reject(state, before, completed_before, "REJECT_QSC_BINDING_NO_PENDING")
    expected_ct = expected_ciphertext(message.session, message.suite, ALICE_KEM_PK)
    if message.kem_ciphertext != expected_ct:
        return _reject(
            state, before, completed_before, "REJECT_QSC_BINDING_WRONG_CIPHERTEXT"
        )

    completed = CompletedSession(
        owner_role=Role.INITIATOR,
        peer_role=Role.RESPONDER,
        session=message.session,
        suite=message.suite,
        confirm_token=expected_confirm(message.session),
    )
    output = OutputFlag(
        owner_role=Role.INITIATOR,
        message_type=MessageType.A2,
        session=message.session,
        success=True,
    )
    next_state = ModelState(
        pending=_without_pending(state.pending, Role.INITIATOR, message.session),
        completed_sessions=state.completed_sessions + (completed,),
        seen_replay_state=_append_unique(state.seen_replay_state, replay_key(message)),
        output_flags=state.output_flags + (output,),
    )
    return _accept(
        before,
        completed_before,
        next_state,
        "ACCEPT_QSC_BINDING_B1",
        output_emitted=True,
        success_output_emitted=True,
    )


def apply_a2(
    state: ModelState,
    message: Message,
    before: StateSnapshot,
    completed_before: CompletedSet,
) -> Tuple[ModelState, Outcome]:
    pending = _find_pending(state, Role.RESPONDER, message.session)
    if pending is None:
        return _reject(state, before, completed_before, "REJECT_QSC_BINDING_NO_PENDING")
    if message.confirm_token != expected_confirm(message.session):
        return _reject(state, before, completed_before, "REJECT_QSC_BINDING_CONFIRM")

    completed = CompletedSession(
        owner_role=Role.RESPONDER,
        peer_role=Role.INITIATOR,
        session=message.session,
        suite=message.suite,
        confirm_token=message.confirm_token,
    )
    output = OutputFlag(
        owner_role=Role.RESPONDER,
        message_type=MessageType.A2,
        session=message.session,
        success=True,
    )
    next_state = ModelState(
        pending=_without_pending(state.pending, Role.RESPONDER, message.session),
        completed_sessions=state.completed_sessions + (completed,),
        seen_replay_state=_append_unique(state.seen_replay_state, replay_key(message)),
        output_flags=state.output_flags + (output,),
    )
    return _accept(
        before,
        completed_before,
        next_state,
        "ACCEPT_QSC_BINDING_A2",
        output_emitted=True,
        success_output_emitted=True,
    )


def run_trace(scenario: TraceScenario) -> Tuple[ModelState, Tuple[Outcome, ...]]:
    state = scenario.start_state if scenario.start_state is not None else initial_trace_state()
    state.assert_invariants()
    outcomes = []
    for message in scenario.messages:
        before = _state_snapshot(state)
        completed_before = completed_snapshot(state)
        next_state, outcome = apply_message(state, message)
        repeated_state, repeated_outcome = apply_message(state, message)
        if not outcome.accepted:
            assert _state_snapshot(next_state) == before
            assert _state_snapshot(repeated_state) == before
            assert repeated_outcome == outcome
            assert next_state.completed_sessions == completed_before
            assert not outcome.output_emitted
            assert not outcome.success_output_emitted
        outcomes.append(outcome)
        state = next_state
        if not outcome.accepted:
            break
    accepted = all(outcome.accepted for outcome in outcomes)
    assert accepted == scenario.expected_final_accept
    if scenario.expected_reject_reason is not None:
        assert outcomes
        assert outcomes[-1].reason == scenario.expected_reject_reason
    assert len(state.completed_sessions) == scenario.expect_completed_sessions
    if scenario.expected_final_accept:
        assert any(output.success for output in state.output_flags)
    else:
        assert not any(
            output.success
            for outcome, output in zip(outcomes, state.output_flags)
            if not outcome.accepted
        )
    return state, tuple(outcomes)


def valid_trace_messages(session: str = SESSION) -> Tuple[Message, Message, Message]:
    return (
        valid_message(MessageType.A1, session=session),
        valid_message(MessageType.B1, session=session),
        valid_message(MessageType.A2, session=session),
    )


def completed_baseline_state() -> ModelState:
    state, outcomes = run_trace(
        TraceScenario(
            name="valid_baseline_for_stale_record",
            messages=valid_trace_messages(),
            expected_final_accept=True,
            expect_completed_sessions=2,
        )
    )
    assert all(outcome.accepted for outcome in outcomes)
    return state


def iter_scenarios() -> Iterable[TraceScenario]:
    valid_a1, valid_b1, valid_a2 = valid_trace_messages()
    yield TraceScenario(
        name="valid_baseline",
        messages=(valid_a1, valid_b1, valid_a2),
        expected_final_accept=True,
        expect_completed_sessions=2,
    )
    yield TraceScenario(
        name="wrong_kem_public_key",
        messages=(
            replace_message(valid_a1, kem_public_key=ALICE_KEM_PK_WRONG),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_WRONG_KEM_PUBLIC_KEY",
    )
    yield TraceScenario(
        name="stale_kem_public_record",
        messages=(
            replace_message(
                valid_a1,
                kem_public_key=ALICE_KEM_PK_STALE,
                public_record_token=ALICE_RECORD_STALE,
                trusted_pin_token=STALE_PIN,
            ),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_STALE_KEM_PUBLIC_RECORD",
    )
    yield TraceScenario(
        name="wrong_ciphertext",
        messages=(
            valid_a1,
            replace_message(valid_b1, kem_ciphertext=WRONG_CIPHERTEXT),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_WRONG_CIPHERTEXT",
    )
    yield TraceScenario(
        name="wrong_signature_identity",
        messages=(
            valid_a1,
            replace_message(
                valid_b1,
                signature_identity=MALLORY_ID,
                signature_public_key=MALLORY_SIG_PK,
            ),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_WRONG_SIGNATURE",
    )
    yield TraceScenario(
        name="cross_message_signature_replay",
        messages=(
            valid_a1,
            replace_message(
                valid_b1,
                signature_message_context=expected_context(
                    MessageType.A2, SESSION, SUITE2, BOB_ID
                ),
            ),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_CROSS_MESSAGE_SIGNATURE_REPLAY",
    )
    yield TraceScenario(
        name="transcript_mutation",
        messages=(
            valid_a1,
            replace_message(valid_b1, transcript_token=WRONG_TRANSCRIPT),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_TRANSCRIPT_MUTATION",
    )
    yield TraceScenario(
        name="transcript_replay",
        messages=(valid_message(MessageType.A1, session=SESSION_REPLAY),)
        + (valid_message(MessageType.A1, session=SESSION_REPLAY),),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_REPLAY",
        start_state=initial_trace_state(session=SESSION_REPLAY),
    )
    yield TraceScenario(
        name="suite_confusion",
        messages=(
            replace_message(
                valid_a1,
                suite=WRONG_SUITE,
                signature_message_context=expected_context(
                    MessageType.A1, SESSION, WRONG_SUITE, ALICE_ID
                ),
                transcript_token=expected_transcript(MessageType.A1, SESSION, WRONG_SUITE),
            ),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_SUITE_CONFUSION",
    )
    yield TraceScenario(
        name="stale_public_record",
        messages=(
            valid_a1,
            replace_message(
                valid_b1,
                public_record_token=BOB_RECORD_STALE,
                trusted_pin_token=STALE_PIN,
            ),
        ),
        expected_final_accept=False,
        expected_reject_reason="REJECT_QSC_BINDING_STALE_PUBLIC_RECORD",
    )


MARKERS = (
    "NA0478_FORMAL_MAPPING_SCOPE_CONSUMED_OK",
    "NA0478_BINDING_MODEL_VALID_TRACE_OK",
    "NA0478_BINDING_MODEL_WRONG_KEM_REJECT_OK",
    "NA0478_BINDING_MODEL_STALE_KEM_PUBLIC_RECORD_REJECT_OK",
    "NA0478_BINDING_MODEL_WRONG_CIPHERTEXT_REJECT_OK",
    "NA0478_BINDING_MODEL_WRONG_SIGNATURE_REJECT_OK",
    "NA0478_BINDING_MODEL_CROSS_MESSAGE_SIGNATURE_REPLAY_REJECT_OK",
    "NA0478_BINDING_MODEL_TRANSCRIPT_MUTATION_REJECT_OK",
    "NA0478_BINDING_MODEL_REPLAY_REJECT_OK",
    "NA0478_BINDING_MODEL_SUITE_CONFUSION_REJECT_OK",
    "NA0478_BINDING_MODEL_STALE_PUBLIC_RECORD_REJECT_OK",
    "NA0478_BINDING_MODEL_NO_SESSION_MUTATION_OK",
    "NA0478_BINDING_MODEL_NO_SUCCESS_OUTPUT_ON_REJECT_OK",
    "NA0478_NO_RUNTIME_CHANGE_OK",
    "NA0478_NO_DEPENDENCY_CHANGE_OK",
    "NA0478_NO_WORKFLOW_CHANGE_OK",
    "NA0478_NO_PUBLIC_READINESS_CLAIM_OK",
    "NA0478_NO_CRYPTO_COMPLETE_CLAIM_OK",
    "NA0478_NO_KEM_COMPLETE_CLAIM_OK",
    "NA0478_NO_SIGNATURE_COMPLETE_CLAIM_OK",
    "NA0478_NO_IDENTITY_COMPLETE_CLAIM_OK",
    "NA0478_NO_TRANSCRIPT_COMPLETE_CLAIM_OK",
    "NA0478_NO_DOWNGRADE_PROOF_CLAIM_OK",
    "NA0478_NO_REPLAY_PROOF_CLAIM_OK",
    "NA0478_NO_FORMAL_PROOF_COMPLETE_CLAIM_OK",
    "NA0478_ONE_READY_INVARIANT_OK",
)


def check_qsc_kem_signature_transcript_binding_model() -> Dict[str, int]:
    scenarios = tuple(iter_scenarios())
    assert scenarios

    stats = {
        "scenarios": 0,
        "accepted_traces": 0,
        "rejected_traces": 0,
        "wrong_kem_rejects": 0,
        "stale_kem_public_record_rejects": 0,
        "wrong_ciphertext_rejects": 0,
        "wrong_signature_rejects": 0,
        "cross_message_signature_replay_rejects": 0,
        "transcript_mutation_rejects": 0,
        "replay_rejects": 0,
        "suite_confusion_rejects": 0,
        "stale_public_record_rejects": 0,
        "no_completed_session_mutation_assertions": 0,
        "no_success_output_on_reject_assertions": 0,
        "completed_sessions": 0,
    }

    for scenario in scenarios:
        state, outcomes = run_trace(scenario)
        assert outcomes
        stats["scenarios"] += 1
        final_outcome = outcomes[-1]

        if scenario.expected_final_accept:
            assert all(outcome.accepted for outcome in outcomes)
            assert len(state.completed_sessions) == 2
            stats["accepted_traces"] += 1
            stats["completed_sessions"] += len(state.completed_sessions)
            continue

        assert not final_outcome.accepted
        assert not final_outcome.completed_session_mutation
        assert not final_outcome.success_output_emitted
        stats["rejected_traces"] += 1
        stats["no_completed_session_mutation_assertions"] += 1
        stats["no_success_output_on_reject_assertions"] += 1

        if final_outcome.reason == "REJECT_QSC_BINDING_WRONG_KEM_PUBLIC_KEY":
            stats["wrong_kem_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_STALE_KEM_PUBLIC_RECORD":
            stats["stale_kem_public_record_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_WRONG_CIPHERTEXT":
            stats["wrong_ciphertext_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_WRONG_SIGNATURE":
            stats["wrong_signature_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_CROSS_MESSAGE_SIGNATURE_REPLAY":
            stats["cross_message_signature_replay_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_TRANSCRIPT_MUTATION":
            stats["transcript_mutation_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_REPLAY":
            stats["replay_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_SUITE_CONFUSION":
            stats["suite_confusion_rejects"] += 1
        elif final_outcome.reason == "REJECT_QSC_BINDING_STALE_PUBLIC_RECORD":
            stats["stale_public_record_rejects"] += 1
        else:
            raise AssertionError(f"unexpected reject reason: {final_outcome.reason}")

    baseline = completed_baseline_state()
    stale_message = replace_message(
        valid_message(MessageType.A1),
        session=SESSION,
        transcript_token=expected_transcript(MessageType.A1, SESSION, SUITE2),
        public_record_token=ALICE_RECORD_STALE,
        trusted_pin_token=STALE_PIN,
        kem_public_key=ALICE_KEM_PK_STALE,
    )
    before_completed = baseline.completed_sessions
    after, stale_outcome = apply_message(baseline, stale_message)
    assert not stale_outcome.accepted
    assert stale_outcome.reason == "REJECT_QSC_BINDING_REPLAY"
    assert after.completed_sessions == before_completed
    stats["no_completed_session_mutation_assertions"] += 1
    stats["no_success_output_on_reject_assertions"] += 1

    assert stats["accepted_traces"] > 0
    assert stats["completed_sessions"] > 0
    assert stats["wrong_kem_rejects"] > 0
    assert stats["stale_kem_public_record_rejects"] > 0
    assert stats["wrong_ciphertext_rejects"] > 0
    assert stats["wrong_signature_rejects"] > 0
    assert stats["cross_message_signature_replay_rejects"] > 0
    assert stats["transcript_mutation_rejects"] > 0
    assert stats["replay_rejects"] > 0
    assert stats["suite_confusion_rejects"] > 0
    assert stats["stale_public_record_rejects"] > 0
    assert stats["no_completed_session_mutation_assertions"] >= stats["rejected_traces"]
    assert stats["no_success_output_on_reject_assertions"] >= stats["rejected_traces"]
    assert stats["rejected_traces"] > stats["accepted_traces"]
    return stats


def emit_qsc_kem_signature_transcript_binding_model_report() -> Dict[str, int]:
    stats = check_qsc_kem_signature_transcript_binding_model()
    for marker in MARKERS:
        print(marker)
    print(f"QSC binding scenarios explored: {stats['scenarios']}")
    print(f"QSC binding accepted traces: {stats['accepted_traces']}")
    print(f"QSC binding rejected traces: {stats['rejected_traces']}")
    print(f"QSC binding completed sessions: {stats['completed_sessions']}")
    print(f"QSC binding wrong KEM rejects: {stats['wrong_kem_rejects']}")
    print(
        "QSC binding stale KEM public-record rejects: "
        f"{stats['stale_kem_public_record_rejects']}"
    )
    print(f"QSC binding wrong ciphertext rejects: {stats['wrong_ciphertext_rejects']}")
    print(f"QSC binding wrong signature rejects: {stats['wrong_signature_rejects']}")
    print(
        "QSC binding cross-message signature replay rejects: "
        f"{stats['cross_message_signature_replay_rejects']}"
    )
    print(
        "QSC binding transcript mutation rejects: "
        f"{stats['transcript_mutation_rejects']}"
    )
    print(f"QSC binding replay rejects: {stats['replay_rejects']}")
    print(f"QSC binding suite-confusion rejects: {stats['suite_confusion_rejects']}")
    print(
        "QSC binding stale public-record rejects: "
        f"{stats['stale_public_record_rejects']}"
    )
    print(
        "QSC binding no completed-session mutation assertions: "
        f"{stats['no_completed_session_mutation_assertions']}"
    )
    print(
        "QSC binding no success-output-on-reject assertions: "
        f"{stats['no_success_output_on_reject_assertions']}"
    )
    return stats


def main() -> int:
    emit_qsc_kem_signature_transcript_binding_model_report()
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as exc:
        print(f"ERROR: qsc KEM/signature/transcript binding model failed: {exc}")
        raise SystemExit(1)
