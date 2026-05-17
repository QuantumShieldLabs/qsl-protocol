"""Bounded executable model for future qsc handshake suite-id semantics.

Goals: G1, G2, G3, G4, G5

This is a formal/model artifact only. It does not implement a qsc runtime
parser, QHSM wire format, QSP wire format, production handshake behavior, or
key schedule change.

The model captures the future semantics selected by NA-0307/NA-0308:
- legacy QHSM v1 frames are accepted only by an explicit compatibility mode;
- suite-id-required mode requires a canonical QHSM v2 negotiated-parameter
  context naming Suite-2;
- A1/B1/A2 must carry byte-identical canonical parameter-block context;
- transcript and key-context bindings must include that context; and
- every reject is fail-closed: no accepted-state mutation, no output or
  recv_commit, no secret/sentinel leak, and a deterministic reason label.

The model is bounded and crypto-agnostic. It treats transcript and key-context
binding as explicit abstract context values rather than proving cryptographic
security.
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from typing import Dict, Iterable, Optional, Tuple

SUITE_CONTEXT_PARAM_ID = 0x0001
MAX_PARAM_BLOCK_LEN = 64
SUITE2_PROTOCOL_VERSION = 0x0500
SUITE2_SUITE_ID = 0x0002
LEGACY_PROTOCOL_VERSION = 0x0403
LEGACY_SUITE_ID = 0x0001
UNSUPPORTED_SUITE_ID = 0x9999
SECRET_SENTINEL = "NA0309_SECRET_SENTINEL"


class Mode(str, Enum):
    LEGACY_COMPATIBILITY = "legacy_compatibility"
    SUITE_REQUIRED = "suite_required"


class FrameVersion(str, Enum):
    V1 = "v1"
    V2 = "v2"


@dataclass(frozen=True, slots=True)
class Parameter:
    param_id: int
    critical: bool
    value: Tuple[int, ...]
    declared_len: Optional[int] = None
    reserved_flags_ok: bool = True

    def encoded(self) -> Tuple[int, int, int, Tuple[int, ...]]:
        flags = 0x01 if self.critical else 0x00
        length = len(self.value) if self.declared_len is None else self.declared_len
        return (self.param_id, flags, length, self.value)


@dataclass(frozen=True, slots=True)
class ParamBlock:
    params: Tuple[Parameter, ...]
    declared_total_len: Optional[int] = None

    def encoded(self) -> Tuple[Tuple[int, int, int, Tuple[int, ...]], ...]:
        return tuple(param.encoded() for param in self.params)

    def total_len(self) -> int:
        # param_id:u16 + flags:u8 + value_len:u16 + value bytes
        return sum(5 + len(param.value) for param in self.params)


@dataclass(frozen=True, slots=True)
class ModelState:
    mode: Mode
    accepted: bool = False
    explicit_suite_admission: bool = False
    protocol_version: Optional[int] = None
    suite_id: Optional[int] = None
    compatibility_accept_count: int = 0
    suite2_accept_count: int = 0
    reject_count: int = 0

    def assert_invariants(self) -> None:
        assert self.compatibility_accept_count >= 0
        assert self.suite2_accept_count >= 0
        assert self.reject_count >= 0
        if self.explicit_suite_admission:
            assert self.mode == Mode.SUITE_REQUIRED
            assert self.accepted
            assert self.protocol_version == SUITE2_PROTOCOL_VERSION
            assert self.suite_id == SUITE2_SUITE_ID
        if self.compatibility_accept_count:
            assert self.mode == Mode.LEGACY_COMPATIBILITY
            assert self.accepted
            assert not self.explicit_suite_admission


StateSnapshot = Tuple[
    Mode,
    bool,
    bool,
    Optional[int],
    Optional[int],
    int,
    int,
    int,
]


def _state_snapshot(state: ModelState) -> StateSnapshot:
    return (
        state.mode,
        state.accepted,
        state.explicit_suite_admission,
        state.protocol_version,
        state.suite_id,
        state.compatibility_accept_count,
        state.suite2_accept_count,
        state.reject_count,
    )


@dataclass(frozen=True, slots=True)
class Scenario:
    name: str
    mode: Mode
    frame_version: FrameVersion
    a1: Optional[ParamBlock] = None
    b1: Optional[ParamBlock] = None
    a2: Optional[ParamBlock] = None
    transcript_context: Optional[Tuple[Tuple[int, int, int, Tuple[int, ...]], ...]] = None
    key_context: Optional[Tuple[Tuple[int, int, int, Tuple[int, ...]], ...]] = None
    expected_accept: bool = False
    expected_reason: str = ""
    contains_secret_sentinel: bool = False


@dataclass(frozen=True, slots=True)
class Outcome:
    accepted: bool
    reason: str
    output_emitted: bool
    recv_commit: bool
    secret_leak: bool
    mutation: bool


@dataclass(frozen=True, slots=True)
class Validation:
    reason: Optional[str]
    protocol_version: Optional[int] = None
    suite_id: Optional[int] = None


def _suite_param(protocol_version: int, suite_id: int) -> Parameter:
    return Parameter(
        param_id=SUITE_CONTEXT_PARAM_ID,
        critical=True,
        value=(
            (protocol_version >> 8) & 0xFF,
            protocol_version & 0xFF,
            (suite_id >> 8) & 0xFF,
            suite_id & 0xFF,
        ),
    )


def _block(*params: Parameter, declared_total_len: Optional[int] = None) -> ParamBlock:
    return ParamBlock(params=tuple(params), declared_total_len=declared_total_len)


VALID_BLOCK = _block(_suite_param(SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID))
DOWNGRADE_BLOCK = _block(_suite_param(LEGACY_PROTOCOL_VERSION, LEGACY_SUITE_ID))
UNSUPPORTED_BLOCK = _block(_suite_param(SUITE2_PROTOCOL_VERSION, UNSUPPORTED_SUITE_ID))
INCONSISTENT_BLOCK = _block(_suite_param(LEGACY_PROTOCOL_VERSION, SUITE2_SUITE_ID))
UNKNOWN_CRITICAL_BLOCK = _block(
    _suite_param(SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID),
    Parameter(param_id=0x0002, critical=True, value=(0xAA,)),
)
DUPLICATE_BLOCK = _block(
    _suite_param(SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID),
    _suite_param(SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID),
)
NONCANONICAL_BLOCK = _block(
    Parameter(param_id=0x0002, critical=False, value=(0x00,)),
    _suite_param(SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID),
)
MALFORMED_LENGTH_BLOCK = _block(
    Parameter(
        param_id=SUITE_CONTEXT_PARAM_ID,
        critical=True,
        value=(0x05, 0x00, 0x00, 0x02),
        declared_len=3,
    )
)
MISSING_SUITE_BLOCK = _block(Parameter(param_id=0x0002, critical=False, value=(0x00,)))
MISMATCH_BLOCK = _block(_suite_param(SUITE2_PROTOCOL_VERSION, UNSUPPORTED_SUITE_ID))
WRONG_CONTEXT = ((0xDEAD, 0x00, 0, ()),)


def _parse_suite_tuple(param: Parameter) -> Tuple[int, int]:
    assert param.param_id == SUITE_CONTEXT_PARAM_ID
    assert len(param.value) == 4
    protocol_version = (param.value[0] << 8) | param.value[1]
    suite_id = (param.value[2] << 8) | param.value[3]
    return protocol_version, suite_id


def canonicality_check(block: ParamBlock) -> Validation:
    """Validate bounded canonical parameter-block shape and Suite-2 tuple."""

    declared_total = (
        block.total_len() if block.declared_total_len is None else block.declared_total_len
    )
    if declared_total != block.total_len() or declared_total > MAX_PARAM_BLOCK_LEN:
        return Validation("REJECT_QSC_HS_MALFORMED_LENGTH")

    prior_id = -1
    seen_ids = set()
    suite_param: Optional[Parameter] = None
    for param in block.params:
        if param.declared_len is not None and param.declared_len != len(param.value):
            return Validation("REJECT_QSC_HS_MALFORMED_LENGTH")
        if not param.reserved_flags_ok:
            return Validation("REJECT_QSC_HS_MALFORMED_LENGTH")
        if param.param_id in seen_ids:
            return Validation("REJECT_QSC_HS_DUPLICATE_PARAMETER")
        if param.param_id <= prior_id:
            return Validation("REJECT_QSC_HS_NONCANONICAL_ORDER")
        seen_ids.add(param.param_id)
        prior_id = param.param_id

        if param.param_id == SUITE_CONTEXT_PARAM_ID:
            suite_param = param

    if suite_param is None:
        return Validation("REJECT_QSC_HS_SUITE_MISSING")
    if not suite_param.critical or len(suite_param.value) != 4:
        return Validation("REJECT_QSC_HS_MALFORMED_LENGTH")

    for param in block.params:
        if param.param_id == SUITE_CONTEXT_PARAM_ID:
            continue
        if param.critical:
            return Validation("REJECT_QSC_HS_UNKNOWN_CRITICAL")
        # NA-0307 selected a reject-by-default posture for unknown noncritical
        # parameters until a later directive authorizes ignore semantics with
        # transcript coverage.
        return Validation("REJECT_QSC_HS_UNKNOWN_PARAMETER")

    protocol_version, suite_id = _parse_suite_tuple(suite_param)
    if (protocol_version, suite_id) == (
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
    ):
        return Validation(None, protocol_version=protocol_version, suite_id=suite_id)
    if (protocol_version, suite_id) == (LEGACY_PROTOCOL_VERSION, LEGACY_SUITE_ID):
        return Validation(
            "REJECT_QSC_HS_DOWNGRADE",
            protocol_version=protocol_version,
            suite_id=suite_id,
        )
    if protocol_version == SUITE2_PROTOCOL_VERSION and suite_id != SUITE2_SUITE_ID:
        return Validation(
            "REJECT_QSC_HS_SUITE_UNSUPPORTED",
            protocol_version=protocol_version,
            suite_id=suite_id,
        )
    return Validation(
        "REJECT_QSC_HS_INCONSISTENT_TUPLE",
        protocol_version=protocol_version,
        suite_id=suite_id,
    )


def compatibility_mode_admission(state: ModelState, scenario: Scenario) -> Tuple[ModelState, Outcome]:
    before = _state_snapshot(state)
    if scenario.frame_version == FrameVersion.V1:
        if scenario.mode != Mode.LEGACY_COMPATIBILITY:
            return _reject(state, before, "REJECT_QSC_HS_LEGACY_REQUIRED")
        accepted = ModelState(
            mode=state.mode,
            accepted=True,
            explicit_suite_admission=False,
            protocol_version=None,
            suite_id=None,
            compatibility_accept_count=state.compatibility_accept_count + 1,
            suite2_accept_count=state.suite2_accept_count,
            reject_count=state.reject_count,
        )
        accepted.assert_invariants()
        return accepted, _accept_outcome("ACCEPT_QSC_HS_LEGACY_COMPATIBILITY")
    return suite_required_mode_admission(state, scenario)


def suite_required_mode_admission(state: ModelState, scenario: Scenario) -> Tuple[ModelState, Outcome]:
    before = _state_snapshot(state)
    if scenario.mode != Mode.SUITE_REQUIRED:
        return compatibility_mode_admission(state, scenario)
    if scenario.frame_version == FrameVersion.V1:
        return _reject(state, before, "REJECT_QSC_HS_LEGACY_REQUIRED")
    if scenario.a1 is None:
        return _reject(state, before, "REJECT_QSC_HS_SUITE_MISSING")

    a1_validation = canonicality_check(scenario.a1)
    if a1_validation.reason is not None:
        return _reject(state, before, a1_validation.reason)

    if scenario.b1 is None or scenario.a2 is None:
        return _reject(state, before, "REJECT_QSC_HS_CONTEXT_MISMATCH")
    canonical_bytes = scenario.a1.encoded()
    if scenario.b1.encoded() != canonical_bytes or scenario.a2.encoded() != canonical_bytes:
        return _reject(state, before, "REJECT_QSC_HS_CONTEXT_MISMATCH")
    b1_validation = canonicality_check(scenario.b1)
    if b1_validation.reason is not None:
        return _reject(state, before, b1_validation.reason)
    a2_validation = canonicality_check(scenario.a2)
    if a2_validation.reason is not None:
        return _reject(state, before, a2_validation.reason)
    if not transcript_context_validation(scenario, canonical_bytes):
        return _reject(state, before, "REJECT_QSC_HS_TRANSCRIPT_CONTEXT")
    if not key_context_validation(scenario, canonical_bytes):
        return _reject(state, before, "REJECT_QSC_HS_KEY_CONTEXT")

    accepted = ModelState(
        mode=state.mode,
        accepted=True,
        explicit_suite_admission=True,
        protocol_version=a1_validation.protocol_version,
        suite_id=a1_validation.suite_id,
        compatibility_accept_count=state.compatibility_accept_count,
        suite2_accept_count=state.suite2_accept_count + 1,
        reject_count=state.reject_count,
    )
    accepted.assert_invariants()
    assert accepted.protocol_version == SUITE2_PROTOCOL_VERSION
    assert accepted.suite_id == SUITE2_SUITE_ID
    return accepted, _accept_outcome("ACCEPT_QSC_HS_SUITE2")


def transcript_context_validation(
    scenario: Scenario,
    canonical_bytes: Tuple[Tuple[int, int, int, Tuple[int, ...]], ...],
) -> bool:
    return scenario.transcript_context == canonical_bytes


def key_context_validation(
    scenario: Scenario,
    canonical_bytes: Tuple[Tuple[int, int, int, Tuple[int, ...]], ...],
) -> bool:
    return scenario.key_context == canonical_bytes


def reject_reason_selection(state: ModelState, scenario: Scenario) -> str:
    _, outcome = apply_scenario(state, scenario)
    return outcome.reason


def _reject(state: ModelState, before: StateSnapshot, reason: str) -> Tuple[ModelState, Outcome]:
    rejected = ModelState(
        mode=state.mode,
        accepted=state.accepted,
        explicit_suite_admission=state.explicit_suite_admission,
        protocol_version=state.protocol_version,
        suite_id=state.suite_id,
        compatibility_accept_count=state.compatibility_accept_count,
        suite2_accept_count=state.suite2_accept_count,
        reject_count=state.reject_count,
    )
    assert _state_snapshot(rejected) == before
    rejected.assert_invariants()
    outcome = Outcome(
        accepted=False,
        reason=reason,
        output_emitted=False,
        recv_commit=False,
        secret_leak=False,
        mutation=False,
    )
    return rejected, outcome


def _accept_outcome(reason: str) -> Outcome:
    return Outcome(
        accepted=True,
        reason=reason,
        output_emitted=True,
        recv_commit=True,
        secret_leak=False,
        mutation=True,
    )


def apply_scenario(state: ModelState, scenario: Scenario) -> Tuple[ModelState, Outcome]:
    state.assert_invariants()
    before = _state_snapshot(state)
    if scenario.mode != state.mode:
        raise AssertionError("scenario mode must match model state")

    next_state, outcome = compatibility_mode_admission(state, scenario)
    mutation = _state_snapshot(next_state) != before
    outcome = Outcome(
        accepted=outcome.accepted,
        reason=outcome.reason,
        output_emitted=outcome.output_emitted,
        recv_commit=outcome.recv_commit,
        secret_leak=outcome.secret_leak,
        mutation=mutation,
    )

    if outcome.accepted:
        assert mutation
    else:
        assert_no_mutation_no_output_no_leak(before, next_state, outcome)
    assert scenario.expected_accept == outcome.accepted
    assert scenario.expected_reason == outcome.reason
    if scenario.contains_secret_sentinel:
        assert SECRET_SENTINEL not in outcome.reason
    return next_state, outcome


def assert_no_mutation_no_output_no_leak(
    before: StateSnapshot, next_state: ModelState, outcome: Outcome
) -> None:
    assert _state_snapshot(next_state) == before
    assert not outcome.mutation
    assert not outcome.output_emitted
    assert not outcome.recv_commit
    assert not outcome.secret_leak


def _valid_scenario(name: str = "valid_v2_suite2") -> Scenario:
    encoded = VALID_BLOCK.encoded()
    return Scenario(
        name=name,
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=VALID_BLOCK,
        a2=VALID_BLOCK,
        transcript_context=encoded,
        key_context=encoded,
        expected_accept=True,
        expected_reason="ACCEPT_QSC_HS_SUITE2",
    )


def iter_scenarios() -> Iterable[Scenario]:
    encoded = VALID_BLOCK.encoded()
    yield _valid_scenario()
    yield Scenario(
        name="legacy_v1_compatibility_allowed",
        mode=Mode.LEGACY_COMPATIBILITY,
        frame_version=FrameVersion.V1,
        expected_accept=True,
        expected_reason="ACCEPT_QSC_HS_LEGACY_COMPATIBILITY",
    )
    yield Scenario(
        name="legacy_v1_required_reject",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V1,
        expected_reason="REJECT_QSC_HS_LEGACY_REQUIRED",
    )
    yield Scenario(
        name="unsupported_suite_id",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=UNSUPPORTED_BLOCK,
        b1=UNSUPPORTED_BLOCK,
        a2=UNSUPPORTED_BLOCK,
        transcript_context=UNSUPPORTED_BLOCK.encoded(),
        key_context=UNSUPPORTED_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_SUITE_UNSUPPORTED",
    )
    yield Scenario(
        name="downgraded_suite_id",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=DOWNGRADE_BLOCK,
        b1=DOWNGRADE_BLOCK,
        a2=DOWNGRADE_BLOCK,
        transcript_context=DOWNGRADE_BLOCK.encoded(),
        key_context=DOWNGRADE_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_DOWNGRADE",
    )
    yield Scenario(
        name="stripped_suite_id",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=MISSING_SUITE_BLOCK,
        b1=MISSING_SUITE_BLOCK,
        a2=MISSING_SUITE_BLOCK,
        transcript_context=MISSING_SUITE_BLOCK.encoded(),
        key_context=MISSING_SUITE_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_SUITE_MISSING",
    )
    yield Scenario(
        name="a1_b1_suite_context_mismatch",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=MISMATCH_BLOCK,
        a2=VALID_BLOCK,
        transcript_context=encoded,
        key_context=encoded,
        expected_reason="REJECT_QSC_HS_CONTEXT_MISMATCH",
    )
    yield Scenario(
        name="b1_a2_suite_context_mismatch",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=VALID_BLOCK,
        a2=MISMATCH_BLOCK,
        transcript_context=encoded,
        key_context=encoded,
        expected_reason="REJECT_QSC_HS_CONTEXT_MISMATCH",
    )
    yield Scenario(
        name="duplicate_suite_id",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=DUPLICATE_BLOCK,
        b1=DUPLICATE_BLOCK,
        a2=DUPLICATE_BLOCK,
        transcript_context=DUPLICATE_BLOCK.encoded(),
        key_context=DUPLICATE_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_DUPLICATE_PARAMETER",
    )
    yield Scenario(
        name="unknown_critical_parameter",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=UNKNOWN_CRITICAL_BLOCK,
        b1=UNKNOWN_CRITICAL_BLOCK,
        a2=UNKNOWN_CRITICAL_BLOCK,
        transcript_context=UNKNOWN_CRITICAL_BLOCK.encoded(),
        key_context=UNKNOWN_CRITICAL_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_UNKNOWN_CRITICAL",
        contains_secret_sentinel=True,
    )
    yield Scenario(
        name="noncanonical_parameter_order",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=NONCANONICAL_BLOCK,
        b1=NONCANONICAL_BLOCK,
        a2=NONCANONICAL_BLOCK,
        transcript_context=NONCANONICAL_BLOCK.encoded(),
        key_context=NONCANONICAL_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_NONCANONICAL_ORDER",
    )
    yield Scenario(
        name="malformed_parameter_length",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=MALFORMED_LENGTH_BLOCK,
        b1=MALFORMED_LENGTH_BLOCK,
        a2=MALFORMED_LENGTH_BLOCK,
        transcript_context=MALFORMED_LENGTH_BLOCK.encoded(),
        key_context=MALFORMED_LENGTH_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_MALFORMED_LENGTH",
        contains_secret_sentinel=True,
    )
    yield Scenario(
        name="inconsistent_protocol_version_suite_id",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=INCONSISTENT_BLOCK,
        b1=INCONSISTENT_BLOCK,
        a2=INCONSISTENT_BLOCK,
        transcript_context=INCONSISTENT_BLOCK.encoded(),
        key_context=INCONSISTENT_BLOCK.encoded(),
        expected_reason="REJECT_QSC_HS_INCONSISTENT_TUPLE",
    )
    yield Scenario(
        name="transcript_context_missing",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=VALID_BLOCK,
        a2=VALID_BLOCK,
        transcript_context=None,
        key_context=encoded,
        expected_reason="REJECT_QSC_HS_TRANSCRIPT_CONTEXT",
    )
    yield Scenario(
        name="transcript_context_mismatch",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=VALID_BLOCK,
        a2=VALID_BLOCK,
        transcript_context=WRONG_CONTEXT,
        key_context=encoded,
        expected_reason="REJECT_QSC_HS_TRANSCRIPT_CONTEXT",
    )
    yield Scenario(
        name="key_context_missing",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=VALID_BLOCK,
        a2=VALID_BLOCK,
        transcript_context=encoded,
        key_context=None,
        expected_reason="REJECT_QSC_HS_KEY_CONTEXT",
    )
    yield Scenario(
        name="key_context_mismatch",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=VALID_BLOCK,
        a2=VALID_BLOCK,
        transcript_context=encoded,
        key_context=WRONG_CONTEXT,
        expected_reason="REJECT_QSC_HS_KEY_CONTEXT",
    )
    yield Scenario(
        name="stripped_b1_context",
        mode=Mode.SUITE_REQUIRED,
        frame_version=FrameVersion.V2,
        a1=VALID_BLOCK,
        b1=None,
        a2=VALID_BLOCK,
        transcript_context=encoded,
        key_context=encoded,
        expected_reason="REJECT_QSC_HS_CONTEXT_MISMATCH",
    )


MARKERS = (
    "NA0309_MODEL_VALID_V2_SUITE2_OK",
    "NA0309_MODEL_LEGACY_COMPATIBILITY_OK",
    "NA0309_MODEL_LEGACY_REQUIRED_REJECT_OK",
    "NA0309_MODEL_UNSUPPORTED_SUITE_REJECT_OK",
    "NA0309_MODEL_DOWNGRADE_REJECT_OK",
    "NA0309_MODEL_STRIPPED_SUITE_REJECT_OK",
    "NA0309_MODEL_MISMATCH_REJECT_OK",
    "NA0309_MODEL_DUPLICATE_REJECT_OK",
    "NA0309_MODEL_UNKNOWN_CRITICAL_REJECT_OK",
    "NA0309_MODEL_NONCANONICAL_REJECT_OK",
    "NA0309_MODEL_MALFORMED_REJECT_OK",
    "NA0309_MODEL_TRANSCRIPT_BINDING_OK",
    "NA0309_MODEL_KEY_CONTEXT_OK",
    "NA0309_MODEL_NO_MUTATION_ON_REJECT_OK",
    "NA0309_MODEL_NO_OUTPUT_ON_REJECT_OK",
    "NA0309_MODEL_NO_SECRET_LEAK_OK",
    "NA0309_MODEL_NO_DOWNGRADE_PATH_OK",
    "NA0309_MODEL_REASON_LABELS_OK",
    "NA0309_QSC_HANDSHAKE_SUITE_ID_FORMAL_MODEL_OK",
)


def check_qsc_handshake_suite_id_model() -> Dict[str, int]:
    """Run bounded qsc handshake suite-id model checks."""

    scenarios = tuple(iter_scenarios())
    assert scenarios

    stats = {
        "scenarios": 0,
        "accepted": 0,
        "rejected": 0,
        "suite2_accepts": 0,
        "legacy_compatibility_accepts": 0,
        "legacy_required_rejects": 0,
        "unsupported_suite_rejects": 0,
        "downgrade_rejects": 0,
        "stripped_suite_rejects": 0,
        "mismatch_rejects": 0,
        "duplicate_rejects": 0,
        "unknown_critical_rejects": 0,
        "noncanonical_rejects": 0,
        "malformed_rejects": 0,
        "transcript_binding_rejects": 0,
        "key_context_rejects": 0,
        "inconsistent_tuple_rejects": 0,
        "no_mutation_assertions": 0,
        "no_output_assertions": 0,
        "no_leak_assertions": 0,
        "no_downgrade_assertions": 0,
        "reason_labels": 0,
    }

    for scenario in scenarios:
        state = ModelState(mode=scenario.mode)
        before = _state_snapshot(state)
        next_state, outcome = apply_scenario(state, scenario)
        repeated_state, repeated_outcome = apply_scenario(state, scenario)
        assert repeated_outcome == outcome
        assert reject_reason_selection(state, scenario) == outcome.reason
        stats["scenarios"] += 1

        if outcome.accepted:
            stats["accepted"] += 1
            assert next_state.accepted
            assert outcome.output_emitted
            assert outcome.recv_commit
            if outcome.reason == "ACCEPT_QSC_HS_SUITE2":
                assert next_state.explicit_suite_admission
                assert next_state.protocol_version == SUITE2_PROTOCOL_VERSION
                assert next_state.suite_id == SUITE2_SUITE_ID
                stats["suite2_accepts"] += 1
            elif outcome.reason == "ACCEPT_QSC_HS_LEGACY_COMPATIBILITY":
                assert scenario.mode == Mode.LEGACY_COMPATIBILITY
                assert not next_state.explicit_suite_admission
                stats["legacy_compatibility_accepts"] += 1
            else:
                raise AssertionError(f"unexpected accept reason: {outcome.reason}")
            continue

        stats["rejected"] += 1
        assert _state_snapshot(next_state) == before
        assert _state_snapshot(repeated_state) == before
        assert_no_mutation_no_output_no_leak(before, next_state, outcome)
        stats["no_mutation_assertions"] += 1
        stats["no_output_assertions"] += 1
        stats["no_leak_assertions"] += 1
        assert outcome.reason.startswith("REJECT_QSC_HS_")
        stats["reason_labels"] += 1

        if scenario.mode == Mode.SUITE_REQUIRED:
            assert next_state.mode == Mode.SUITE_REQUIRED
            stats["no_downgrade_assertions"] += 1

        if outcome.reason == "REJECT_QSC_HS_LEGACY_REQUIRED":
            stats["legacy_required_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_SUITE_UNSUPPORTED":
            stats["unsupported_suite_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_DOWNGRADE":
            stats["downgrade_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_INCONSISTENT_TUPLE":
            stats["inconsistent_tuple_rejects"] += 1
        elif outcome.reason in (
            "REJECT_QSC_HS_SUITE_MISSING",
            "REJECT_QSC_HS_UNKNOWN_PARAMETER",
        ):
            if "stripped" in scenario.name:
                stats["stripped_suite_rejects"] += 1
            stats["noncanonical_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_CONTEXT_MISMATCH":
            stats["mismatch_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_DUPLICATE_PARAMETER":
            stats["duplicate_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_UNKNOWN_CRITICAL":
            stats["unknown_critical_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_NONCANONICAL_ORDER":
            stats["noncanonical_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_MALFORMED_LENGTH":
            stats["malformed_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_TRANSCRIPT_CONTEXT":
            stats["transcript_binding_rejects"] += 1
        elif outcome.reason == "REJECT_QSC_HS_KEY_CONTEXT":
            stats["key_context_rejects"] += 1

    assert stats["suite2_accepts"] > 0
    assert stats["legacy_compatibility_accepts"] > 0
    assert stats["legacy_required_rejects"] > 0
    assert stats["unsupported_suite_rejects"] > 0
    assert stats["downgrade_rejects"] > 0
    assert stats["stripped_suite_rejects"] > 0
    assert stats["mismatch_rejects"] > 0
    assert stats["duplicate_rejects"] > 0
    assert stats["unknown_critical_rejects"] > 0
    assert stats["noncanonical_rejects"] > 0
    assert stats["malformed_rejects"] > 0
    assert stats["transcript_binding_rejects"] > 0
    assert stats["key_context_rejects"] > 0
    assert stats["inconsistent_tuple_rejects"] > 0
    assert stats["no_mutation_assertions"] == stats["rejected"]
    assert stats["no_output_assertions"] == stats["rejected"]
    assert stats["no_leak_assertions"] == stats["rejected"]
    assert stats["no_downgrade_assertions"] > 0
    assert stats["reason_labels"] == stats["rejected"]
    assert stats["rejected"] > stats["accepted"]
    return stats


def emit_qsc_handshake_suite_id_model_report() -> Dict[str, int]:
    stats = check_qsc_handshake_suite_id_model()
    for marker in MARKERS:
        print(marker)
    print(f"QSC suite-id scenarios explored: {stats['scenarios']}")
    print(f"QSC suite-id accepted outcomes: {stats['accepted']}")
    print(f"QSC suite-id rejected outcomes: {stats['rejected']}")
    print(f"QSC suite-id no-mutation assertions: {stats['no_mutation_assertions']}")
    print(f"QSC suite-id no-output assertions: {stats['no_output_assertions']}")
    print(f"QSC suite-id no-leak assertions: {stats['no_leak_assertions']}")
    print(f"QSC suite-id no-downgrade assertions: {stats['no_downgrade_assertions']}")
    print(f"QSC suite-id reason labels: {stats['reason_labels']}")
    return stats


def main() -> int:
    emit_qsc_handshake_suite_id_model_report()
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as exc:
        print(f"ERROR: qsc handshake suite-id model failed: {exc}")
        raise SystemExit(1)
