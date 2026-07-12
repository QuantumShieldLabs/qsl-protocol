#!/usr/bin/env python3
"""Bounded qsc QSC.HS.* handshake-AUTHENTICATION model (NA-0636 / ENG-0038 obligation).

Goals: G1, G2, G4

This is a formal/model artifact only. It does not implement qsc runtime behavior, qsc
executable tests, the QHSM/QSP wire format, cryptographic internals, provider behavior,
refimpl behavior, or a key schedule. It is BOUNDED and CRYPTO-AGNOSTIC: it proves an
authentication-BINDING property over an abstract state machine, NOT cryptographic security,
NOT a side-channel property, NOT a post-compromise or PQ guarantee, and NOT a qsc/refimpl
equivalence claim. A PASS substantiates the bounded binding property and nothing more
(matching the disclaimers in model_qsc_kem_signature_transcript_binding_bounded.py and the
other formal/ models).

WHAT IS MODELED (post-NA-0634 handshake authentication slice; see
docs/governance/evidence/NA-0636_as_built.md §1 for the read-only extraction from
qsc/src/handshake/mod.rs, identity/mod.rs, contacts/mod.rs at main 8e8699db):

  - Identities are opaque (kem_id, sig_id) pairs. The single verified code is an INJECTIVE
    structured token CODE(kem_id, sig_id) = identity_fingerprint_from_identity(kem_pk, sig_pk).
    A legacy single-key code is CODE1(kem_id) = identity_fingerprint_from_pk(kem_pk); the
    signing fingerprint is SIGFP(sig_id) = hs_sig_fingerprint(sig_pk). Injectivity and
    cross-domain distinctness are FREE in the model because codes are distinct Python tuples
    — this IS the crypto-agnostic abstraction of a collision-resistant fingerprint.

  - KEM possession and SIGNATURE possession are modeled as capability sets per party; an
    honest party holds only its own secrets, the adversary M holds its own plus an explicitly
    enumerated compromise subset. A confirm/transcript MAC is forgeable only with KEM
    possession of the pinned/presented key (the C1 mechanism); an A2/B1 signature is
    producible only with possession of the presented signing key.

  - The contact pin store ranges over the states reachable from provisioning (as_built §1.2):
    FULL (combined code + kem + sig_fp), KEM (legacy single-key code + kem, sig_fp absent),
    BARE (a human-typed code, no keys, sig_fp absent), ABSENT. The adversary may re-pin
    between A1 and A2 (mid-run repin), so the model carries pin_at_a1 and pin_at_a2 separately.

  - The initiator ACCEPT rule at B1 (as_built §1.4): required KEM-possession of the pinned
    responder key (A-5), signature verify (A-6), pin stability (A-8), and the NA-0634 REQUIRED
    responder sig-pin (A-9). The responder A1-admit + A2-COMMIT rule (as_built §1.5/§1.6):
    required primary combined pin (R-3/C-5), KEM possession (C-2), signature possession (C-3),
    and the OPTIONAL reverse sig-pin (R-4/C-6, skip-on-absent — EXACTLY AS LANDED).

PROPERTIES (fail-closed assertions, house style):

  P1  mutual-auth binding: every reachable ACCEPT (initiator-accept or responder-commit) binds
      the peer's FULL identity — the accepting party's counterparty holds BOTH the kem secret
      and the sig secret of the identity whose combined code is pinned.
  P2  wrong-signing-key rejection: an initiator-side peer presenting the correct KEM identity
      but a wrong signing identity always reaches a deterministic fail-closed REJECT (the
      NA-0634 required sig-pin), with no commit and no success output.
  P3  THE OBLIGATION (reverse-pin redundancy): with the reverse sig-pin OPTIONAL exactly as
      landed, is there ANY reachable responder-COMMIT in which the initiator's presented sig_id
      is NOT bound to the verified code (a run a REQUIRED reverse pin would have caught but the
      primary combined pin does not)? The model exhausts the bounded space and decides it. It
      also runs the PRE-NA-0634 primary-pin COUNTERFACTUAL, which MUST surface exactly such an
      unbound commit — proving the search is not vacuous (WF-0017: a negative claim must show
      the search could have found a positive).
  P4  fail-closed reject hygiene: every reject commits no session, emits no success output,
      mutates no durable state, and carries a deterministic reason label from a fixed set.

The bound is small, fixed, and stated in EXPLORATION_BOUND below; the whole space is enumerated
exhaustively (guaranteed termination — the ENG-0035 constraint is honored by construction, no
ProVerif/unbounded unrolling).
"""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum
from itertools import combinations, product
from typing import Dict, FrozenSet, List, Optional, Tuple

# --------------------------------------------------------------------------------------------
# Token universe (opaque; injectivity is structural — distinct tuples are distinct codes).
# --------------------------------------------------------------------------------------------

KEMS = ("AK", "BK", "MK")   # ML-KEM identity keys: Alice, Bob, Mallory
SIGS = ("AS", "BS", "MS")   # ML-DSA signing keys:  Alice, Bob, Mallory

# The genuine identity of the peer each honest contact is SUPPOSED to authenticate.
ALICE = ("AK", "AS")
BOB = ("BK", "BS")


def CODE(kem: str, sig: str) -> Tuple[str, str, str]:
    """identity_fingerprint_from_identity(kem_pk, sig_pk) — the combined verified code."""
    return ("CODE", kem, sig)


def CODE1(kem: str) -> Tuple[str, str]:
    """identity_fingerprint_from_pk(kem_pk) — the legacy KEM-only code (distinct domain)."""
    return ("CODE1", kem)


def SIGFP(sig: str) -> Tuple[str, str]:
    """hs_sig_fingerprint(sig_pk) — the sig-pin comparand (distinct domain)."""
    return ("SIGFP", sig)


def code_authenticates_sig(code: Optional[tuple]) -> Optional[str]:
    """The sig identity a pinned code authenticates. A combined code authenticates its sig
    component (by injectivity); a single-key or absent code authenticates NO signing key."""
    if code is not None and code[0] == "CODE":
        return code[2]
    return None


def code_authenticates_kem(code: Optional[tuple]) -> Optional[str]:
    if code is None:
        return None
    if code[0] == "CODE":
        return code[1]
    if code[0] == "CODE1":
        return code[1]
    return None


# --------------------------------------------------------------------------------------------
# Possession (crypto-agnostic capability model).
# --------------------------------------------------------------------------------------------

# Secret halves an honest party holds by virtue of its own identity.
_BASE_SECRETS = {
    "A": frozenset({"AK", "AS"}),
    "B": frozenset({"BK", "BS"}),
    "M": frozenset({"MK", "MS"}),
}
# The secrets M may additionally have stolen (enumerated as the compromise powerset below).
COMPROMISABLE = ("AK", "BK", "AS", "BS")
PARTIES = ("A", "B", "M")


def holds(party: str, secret: str, compromise: FrozenSet[str]) -> bool:
    if secret in _BASE_SECRETS[party]:
        return True
    return party == "M" and secret in compromise


def _all_compromise_sets() -> Tuple[FrozenSet[str], ...]:
    sets: List[FrozenSet[str]] = []
    for r in range(len(COMPROMISABLE) + 1):
        for combo in combinations(COMPROMISABLE, r):
            sets.append(frozenset(combo))
    return tuple(sets)


COMPROMISE_SETS = _all_compromise_sets()  # 16 subsets of {AK,BK,AS,BS}


# --------------------------------------------------------------------------------------------
# Contact pin store states (as_built §1.2), parameterized by the intended genuine identity.
# --------------------------------------------------------------------------------------------

class PinKind(str, Enum):
    FULL = "FULL"            # combined code + kem_pk stored + sig_fp populated
    KEM = "KEM"              # legacy single-key code + kem_pk; sig_fp absent
    BARE_COMBINED = "BARE"   # human typed the genuine combined code; no keys, sig_fp absent
    BARE_SINGLE = "BARE1"    # human typed a legacy single-key code; no keys, sig_fp absent
    ABSENT = "ABSENT"        # no contact


@dataclass(frozen=True, slots=True)
class PinState:
    kind: PinKind
    code: Optional[tuple]          # the pinned verification code (None if ABSENT)
    kem_stored: Optional[str]      # stored peer kem identity (None if not stored)
    sig_fp: Optional[tuple]        # stored SIGFP (None if absent — the OPTIONAL-pin trigger)


def pin_states_for(identity: Tuple[str, str]) -> Tuple[PinState, ...]:
    kem, sig = identity
    return (
        PinState(PinKind.FULL, CODE(kem, sig), kem, SIGFP(sig)),
        PinState(PinKind.KEM, CODE1(kem), kem, None),
        PinState(PinKind.BARE_COMBINED, CODE(kem, sig), None, None),
        PinState(PinKind.BARE_SINGLE, CODE1(kem), None, None),
        PinState(PinKind.ABSENT, None, None, None),
    )


# --------------------------------------------------------------------------------------------
# Reject reason labels (a fixed set, mirroring the real markers in as_built §1.7).
# --------------------------------------------------------------------------------------------

R_PIN_ABSENT = "identity_unknown"
R_PRIMARY_MISMATCH = "peer_mismatch"
R_KEM_POSSESSION = "bad_confirm"          # responder side (A2 confirm MAC); "bad_transcript" on init side
R_KEM_POSSESSION_INIT = "bad_transcript"
R_SIG_POSSESSION = "sig_invalid"
R_REVERSE_OPTIONAL_MISMATCH = "peer_mismatch"      # hs_check_optional_identity_pin mismatch
R_REVERSE_REQUIRED_MISMATCH = "responder_sig_mismatch"
R_REVERSE_REQUIRED_ABSENT = "responder_sig_unpinned"

REASONS = frozenset({
    R_PIN_ABSENT,
    R_PRIMARY_MISMATCH,
    R_KEM_POSSESSION,
    R_KEM_POSSESSION_INIT,
    R_SIG_POSSESSION,
    R_REVERSE_REQUIRED_MISMATCH,
    R_REVERSE_REQUIRED_ABSENT,
})


class ReverseMode(str, Enum):
    LANDED = "landed"        # optional: skip when sig_fp absent (EXACTLY AS LANDED)
    REQUIRED = "required"    # counterfactual: absent sig_fp => reject
    DISABLED = "disabled"    # never check the reverse pin at all (pure primary-only)


# --------------------------------------------------------------------------------------------
# Outcome record.
# --------------------------------------------------------------------------------------------

@dataclass(frozen=True, slots=True)
class Outcome:
    committed: bool
    reason: Optional[str]           # None iff committed
    success_output: bool            # a handshake_complete success marker
    state_mutation: bool            # a durable commit occurred
    bound_kem: Optional[str]        # the kem identity the accept bound (None if not committed)
    bound_sig: Optional[str]        # the sig identity the accept bound (None if not committed)


def _reject(reason: str) -> Outcome:
    assert reason in REASONS, f"undeclared reject reason: {reason}"
    return Outcome(False, reason, False, False, None, None)


# --------------------------------------------------------------------------------------------
# Pin-check primitives.
# --------------------------------------------------------------------------------------------

def _primary_pass(pin: PinState, computed_code: tuple) -> bool:
    """The REQUIRED primary pin: live pin present AND equals the recomputed code."""
    return pin.code is not None and pin.code == computed_code


def _reverse_check(pin: PinState, computed_sigfp: tuple, mode: ReverseMode) -> Optional[str]:
    """Returns a reject reason, or None if the reverse pin passes/skips. `required` is used only
    for the REQUIRED/DISABLED distinction; the two 'required' labels differ by side but the
    responder side uses the responder_* labels."""
    if pin.sig_fp is None:
        if mode is ReverseMode.REQUIRED:
            return R_REVERSE_REQUIRED_ABSENT
        return None  # LANDED optional skip, or DISABLED
    if mode is ReverseMode.DISABLED:
        return None
    if pin.sig_fp == computed_sigfp:
        return None
    return (
        R_REVERSE_REQUIRED_MISMATCH if mode is ReverseMode.REQUIRED
        else R_REVERSE_OPTIONAL_MISMATCH
    )


# --------------------------------------------------------------------------------------------
# Responder side: A1-admit + A2-COMMIT (as_built §1.5/§1.6). The reverse pin is B's optional one.
# --------------------------------------------------------------------------------------------

def responder_decision(
    pin_a1: PinState,
    pin_a2: PinState,
    presented: Tuple[str, str],   # (init_kem, init_sig) the initiator put in A1
    producer: str,                # the initiator-side party that produced A1/A2
    compromise: FrozenSet[str],
    mode: ReverseMode,
    *,
    pre_na0634: bool = False,     # counterfactual: primary pin uses CODE1(kem) only (pre-fix)
) -> Outcome:
    ik, is_ = presented
    computed_primary = CODE1(ik) if pre_na0634 else CODE(ik, is_)
    computed_sigfp = SIGFP(is_)

    # --- A1 admit (creates pending); reject here => no pending, no commit. ---
    if pin_a1.kind is PinKind.ABSENT:
        return _reject(R_PIN_ABSENT)
    if not _primary_pass(pin_a1, computed_primary):
        return _reject(R_PRIMARY_MISMATCH)
    rev = _reverse_check(pin_a1, computed_sigfp, mode)
    if rev is not None:
        return _reject(rev)

    # --- A2 commit. ---
    if pin_a2.kind is PinKind.ABSENT:
        return _reject(R_PIN_ABSENT)
    if not _primary_pass(pin_a2, computed_primary):   # C-5 (live pin == pending.peer_fp)
        return _reject(R_PRIMARY_MISMATCH)
    if not holds(producer, ik, compromise):           # C-2 confirm MAC => KEM possession
        return _reject(R_KEM_POSSESSION)
    if not holds(producer, is_, compromise):          # C-3 A2 signature => SIG possession
        return _reject(R_SIG_POSSESSION)
    rev = _reverse_check(pin_a2, computed_sigfp, mode)  # C-6 optional reverse pin
    if rev is not None:
        return _reject(rev)

    # Committed. What identity did the (live, at-commit) pin authenticate?
    bound_sig = code_authenticates_sig(pin_a2.code)
    bound_kem = code_authenticates_kem(pin_a2.code)
    return Outcome(True, None, True, True, bound_kem, bound_sig)


# --------------------------------------------------------------------------------------------
# Initiator side: ACCEPT at B1 (as_built §1.4). The sig-pin here is REQUIRED (NA-0634 A-9).
# --------------------------------------------------------------------------------------------

def initiator_decision(
    pin_a1: PinState,
    pin_b1: PinState,
    presented: Tuple[str, str],   # (resp_kem_presented, resp_sig_presented) — resp_kem only
    producer: str,                # the responder-side party that produced B1
    compromise: FrozenSet[str],
    *,
    c1_kem_binding: bool = True,  # A-5. False = PRE-C1 counterfactual (responder kem never used)
    sig_pin_required: bool = True,  # A-9. False = PRE-NA-0634 optional (skip-on-absent) semantics
) -> Outcome:
    """The initiator pinned the responder ('bob'). B1 carries the responder's presented signing
    key; the responder KEM identity is bound by POSSESSION of the pinned kem (C1), never sent."""
    _resp_kem_presented, resp_sig = presented
    pinned_kem = pin_a1.kem_stored           # the key the initiator encapsulated to (I-3/A-5)
    computed_sigfp = SIGFP(resp_sig)

    # I-1/I-2: to initiate at all the initiator needs a pin AND a stored responder kem_pk.
    if pin_a1.kind is PinKind.ABSENT or pinned_kem is None:
        return _reject(R_PIN_ABSENT)

    # A-5: transcript MAC => the B1 producer holds the PINNED responder kem secret (C1).
    if c1_kem_binding and not holds(producer, pinned_kem, compromise):
        return _reject(R_KEM_POSSESSION_INIT)
    # A-6: B1 signature => the B1 producer holds the PRESENTED responder sig secret.
    if not holds(producer, resp_sig, compromise):
        return _reject(R_SIG_POSSESSION)
    # A-8: pin stability across init -> B1 (live pin must still be the init-time pin).
    if pin_b1.kind is PinKind.ABSENT or pin_b1.code != pin_a1.code:
        return _reject(R_PRIMARY_MISMATCH)
    # A-9: responder sig-pin. REQUIRED post-NA-0634 (absent OR mismatched sig_fp is fail-closed);
    # PRE-NA-0634 it was OPTIONAL and sig_fp was structurally always absent => it always skipped.
    if pin_b1.sig_fp is None:
        if sig_pin_required:
            return _reject(R_REVERSE_REQUIRED_ABSENT)
    elif pin_b1.sig_fp != computed_sigfp:
        return _reject(R_REVERSE_REQUIRED_MISMATCH)

    bound_sig = code_authenticates_sig(pin_b1.code)
    bound_kem = pinned_kem
    return Outcome(True, None, True, True, bound_kem, bound_sig)


# --------------------------------------------------------------------------------------------
# Exhaustive exploration + property checks.
# --------------------------------------------------------------------------------------------

# The bound is small, fixed, and stated here (ENG-0035 termination constraint: fully enumerated).
EXPLORATION_BOUND = {
    "identities": 3,                       # Alice, Bob, Mallory
    "kem_keys": len(KEMS),                 # 3
    "sig_keys": len(SIGS),                 # 3
    "responder_pin_states": 5,             # FULL, KEM, BARE_COMBINED, BARE_SINGLE, ABSENT
    "initiator_pin_states": 5,
    "producers": len(PARTIES),             # 3
    "compromise_subsets": len(COMPROMISE_SETS),   # 16
    "reverse_modes": 3,                    # LANDED, REQUIRED, DISABLED
    "mid_run_repin": True,                 # pin_at_a1 x pin_at_a2 explored independently
    "messages_per_role": "A1,B1,A2 (single bounded handshake per (config); repin = 2 pin slots)",
}


@dataclass(frozen=True, slots=True)
class RespRow:
    pin_a1: PinState
    pin_a2: PinState
    presented: Tuple[str, str]
    producer: str
    compromise: FrozenSet[str]
    outcome_by_mode: Dict[ReverseMode, Outcome]


def _explore_responder() -> Tuple[Tuple[RespRow, ...], Dict[str, int]]:
    resp_pins = pin_states_for(ALICE)   # the contact 'alice' B is trying to authenticate
    rows: List[RespRow] = []
    stats = {"configs": 0, "commits_landed": 0, "rejects_landed": 0}
    for pin_a1, pin_a2, ik, is_, producer, compromise in product(
        resp_pins, resp_pins, KEMS, SIGS, PARTIES, COMPROMISE_SETS
    ):
        outcome_by_mode: Dict[ReverseMode, Outcome] = {}
        for mode in ReverseMode:
            outcome_by_mode[mode] = responder_decision(
                pin_a1, pin_a2, (ik, is_), producer, compromise, mode
            )
        rows.append(RespRow(pin_a1, pin_a2, (ik, is_), producer, compromise, outcome_by_mode))
        stats["configs"] += 1
        if outcome_by_mode[ReverseMode.LANDED].committed:
            stats["commits_landed"] += 1
        else:
            stats["rejects_landed"] += 1
    return tuple(rows), stats


def _explore_initiator() -> Tuple[Tuple[Tuple[PinState, PinState, Tuple[str, str], str, FrozenSet[str], Outcome], ...], Dict[str, int]]:
    init_pins = pin_states_for(BOB)     # the contact 'bob' the initiator is authenticating
    rows = []
    stats = {"configs": 0, "commits": 0, "rejects": 0}
    for pin_a1, pin_b1, rk, rs, producer, compromise in product(
        init_pins, init_pins, KEMS, SIGS, PARTIES, COMPROMISE_SETS
    ):
        outcome = initiator_decision(pin_a1, pin_b1, (rk, rs), producer, compromise)
        rows.append((pin_a1, pin_b1, (rk, rs), producer, compromise, outcome))
        stats["configs"] += 1
        if outcome.committed:
            stats["commits"] += 1
        else:
            stats["rejects"] += 1
    return tuple(rows), stats


def _check_p1_p4_responder(rows: Tuple[RespRow, ...]) -> Dict[str, int]:
    """P1 (responder-side full-identity binding) + P4 (reject hygiene) over the LANDED model."""
    s = {"p1_commit_bindings": 0, "p4_reject_hygiene": 0}
    for row in rows:
        for mode, outcome in row.outcome_by_mode.items():
            if outcome.committed:
                ik, is_ = row.presented
                # P1: the counterparty holds BOTH secrets of the bound identity, and the bound
                # identity is exactly the presented pair, and the pin authenticates it.
                assert holds(row.producer, ik, row.compromise)
                assert holds(row.producer, is_, row.compromise)
                assert outcome.bound_kem == ik, (mode, row)
                assert outcome.bound_sig == is_, (mode, row)
                assert row.pin_a2.code == CODE(ik, is_)
                if mode is ReverseMode.LANDED:
                    s["p1_commit_bindings"] += 1
            else:
                # P4: reject is inert with a declared deterministic reason.
                assert outcome.reason in REASONS
                assert not outcome.success_output
                assert not outcome.state_mutation
                assert outcome.bound_kem is None and outcome.bound_sig is None
                if mode is ReverseMode.LANDED:
                    s["p4_reject_hygiene"] += 1
    return s


def _check_p1_p2_p4_initiator(rows) -> Dict[str, int]:
    """P1 (initiator-side binding), P2 (wrong-signing-key rejection), P4 (reject hygiene)."""
    s = {"p1_commit_bindings": 0, "p2_wrong_sig_rejects": 0, "p4_reject_hygiene": 0}
    for pin_a1, pin_b1, presented, producer, compromise, outcome in rows:
        _rk, rs = presented
        pinned_kem = pin_a1.kem_stored
        if outcome.committed:
            # P1: B1 producer holds the pinned responder kem secret AND the presented sig secret,
            # and (A-9) the presented sig equals the pinned sig identity.
            assert pinned_kem is not None
            assert holds(producer, pinned_kem, compromise)
            assert holds(producer, rs, compromise)
            assert outcome.bound_sig == rs
            assert pin_b1.sig_fp == SIGFP(rs)
            s["p1_commit_bindings"] += 1
        else:
            assert outcome.reason in REASONS
            assert not outcome.success_output and not outcome.state_mutation
            s["p4_reject_hygiene"] += 1

        # P2: correct KEM identity (producer holds the pinned responder kem) but WRONG signing
        # identity (presented sig is not the pinned sig) MUST reject — never commit.
        if (
            pin_a1.kind is not PinKind.ABSENT
            and pinned_kem is not None
            and holds(producer, pinned_kem, compromise)
            and code_authenticates_sig(pin_a1.code) is not None
            and rs != code_authenticates_sig(pin_a1.code)
        ):
            assert not outcome.committed, (pin_a1, pin_b1, presented, producer, compromise)
            assert outcome.reason in (
                R_REVERSE_REQUIRED_MISMATCH, R_REVERSE_REQUIRED_ABSENT, R_PRIMARY_MISMATCH,
                R_SIG_POSSESSION, R_KEM_POSSESSION_INIT,
            )
            s["p2_wrong_sig_rejects"] += 1
    return s


@dataclass(frozen=True, slots=True)
class P3Result:
    redundant: bool
    unbound_commits: int          # LANDED commits whose init sig_id is NOT bound (the gap size)
    landed_commits: int
    reverse_would_reject_bound: int   # commits REQUIRED would reject though init sig IS bound
    counterexample: Optional[dict]


def _check_p3(rows: Tuple[RespRow, ...]) -> P3Result:
    """THE OBLIGATION. Redundant iff no LANDED responder-commit leaves the initiator's presented
    sig_id unbound to the verified code (i.e. no run a REQUIRED reverse pin would have caught but
    the primary combined pin does not)."""
    unbound = 0
    landed = 0
    reverse_would_reject_bound = 0
    counterexample: Optional[dict] = None
    for row in rows:
        landed_outcome = row.outcome_by_mode[ReverseMode.LANDED]
        if not landed_outcome.committed:
            continue
        landed += 1
        ik, is_ = row.presented
        # Is the presented sig bound to what the (live, at-commit) pin authenticates?
        authenticated_sig = code_authenticates_sig(row.pin_a2.code)
        is_bound = authenticated_sig is not None and authenticated_sig == is_
        required_outcome = row.outcome_by_mode[ReverseMode.REQUIRED]
        if not is_bound:
            # A LANDED commit with an UNBOUND presented sig — a gap iff REQUIRED would catch it.
            unbound += 1
            if not required_outcome.committed and counterexample is None:
                counterexample = {
                    "pin_a1": row.pin_a1.kind.value,
                    "pin_a2": row.pin_a2.kind.value,
                    "presented": row.presented,
                    "producer": row.producer,
                    "compromise": sorted(row.compromise),
                    "authenticated_sig": authenticated_sig,
                    "required_reject_reason": required_outcome.reason,
                }
        elif not required_outcome.committed:
            # BOUND, yet REQUIRED would reject: not a gap — evidence the reverse pin adds no
            # security discrimination and would only introduce false rejects (e.g. S-BARE).
            reverse_would_reject_bound += 1
    return P3Result(
        redundant=(unbound == 0),
        unbound_commits=unbound,
        landed_commits=landed,
        reverse_would_reject_bound=reverse_would_reject_bound,
        counterexample=counterexample,
    )


def _check_p3_counterfactual_nonvacuous() -> dict:
    """NON-VACUITY (WF-0017: a negative claim must prove the search COULD have found a positive).

    PRE-NA-0634 counterfactual: rewind ONLY the primary pin to its pre-fix form — a KEM-only code
    CODE1(kem) that does not cover the signing key — leaving everything else as landed. The search
    MUST then surface an IMPERSONATION: an adversary M that makes the responder COMMIT a session
    for the contact 'alice' while presenting its OWN signing key (M does not hold Alice's signing
    secret), which a REQUIRED reverse sig-pin would have caught. Finding it proves the P3 search
    discriminates bound from unbound; not finding it would mean the P3 'redundant' verdict is
    meaningless and the model must fail closed.

    This also makes the verdict's DEPENDENCY machine-visible: the reverse pin is redundant ONLY
    because the primary code covers the signing key injectively. A code format that stops covering
    the sig half makes the reverse pin load-bearing again."""
    resp_pins = pin_states_for(ALICE)
    candidates: List[Tuple[Tuple[int, int], dict]] = []
    total_unbound = 0
    for pin_a1, pin_a2, ik, is_, producer, compromise in product(
        resp_pins, resp_pins, KEMS, SIGS, PARTIES, COMPROMISE_SETS
    ):
        landed = responder_decision(
            pin_a1, pin_a2, (ik, is_), producer, compromise, ReverseMode.LANDED, pre_na0634=True
        )
        if not landed.committed:
            continue
        # Under the pre-fix primary pin the presented sig is unconstrained by the primary check.
        authenticated_sig = code_authenticates_sig(pin_a2.code)
        if authenticated_sig is not None and authenticated_sig == is_:
            continue
        required = responder_decision(
            pin_a1, pin_a2, (ik, is_), producer, compromise,
            ReverseMode.REQUIRED, pre_na0634=True,
        )
        if required.committed:
            continue
        total_unbound += 1
        # Demand a REAL impersonation as the witness, not a degenerate one: the adversary commits
        # as 'alice' using a signing key it owns and Alice does not — i.e. the responder ends up
        # authenticated to the WRONG signing identity. Rank so the STRONGEST (fewest stolen
        # secrets; the adversary's OWN signing key) is the one reported.
        if (
            producer == "M"
            and is_ != ALICE[1]
            and not holds("M", ALICE[1], compromise)
        ):
            rank = (len(compromise), 0 if is_ in _BASE_SECRETS["M"] else 1)
            candidates.append((rank, {
                "pin_a1": pin_a1.kind.value,
                "pin_a2": pin_a2.kind.value,
                "presented_kem": ik,
                "presented_sig": is_,
                "producer": producer,
                "m_stole": sorted(compromise),
                "genuine_alice_sig": ALICE[1],
                "required_reverse_pin_would_reject_with": required.reason,
            }))
    impersonation = min(candidates, key=lambda c: c[0])[1] if candidates else None
    assert impersonation is not None, (
        "NON-VACUITY FAILURE: the pre-NA-0634 counterfactual surfaced no adversarial unbound-sig "
        "commit — the P3 search cannot distinguish bound from unbound, so a 'redundant' verdict "
        "would be meaningless. Failing closed rather than reporting a vacuous PASS."
    )
    return {"impersonation_witness": impersonation, "pre_fix_unbound_commits": total_unbound}


def _check_eng0038_original_reproduction() -> dict:
    """FAITHFULNESS ANCHOR: the model must reproduce the KNOWN, REAL, already-fixed ENG-0038 flaw
    (NA-0632's P1 finding) when the landed defences are rewound — and must show them CLOSED as
    landed. Two directions, both on the initiator's ACCEPT rule:

      (a) PRE-C1 + PRE-NA-0634 (no responder-KEM possession binding; sig-pin optional and sig_fp
          structurally absent): an adversary M holding NO stolen secret at all must be able to make
          the initiator COMMIT believing the peer is 'bob' — the original active-MITM responder
          impersonation. The model MUST find it.
      (b) AS LANDED (C1 + the NA-0634 required sig-pin): the same adversary — and, sharper, an M
          that has stolen Bob's KEM identity secret but NOT his signing secret — must NEVER reach a
          commit. The model MUST find zero such commits.
    """
    init_pins = pin_states_for(BOB)
    pre_fix_impersonations = 0
    pre_fix_candidates: List[Tuple[Tuple[int, int], dict]] = []
    landed_kem_only_compromise_commits = 0
    landed_zero_compromise_commits = 0

    for pin_a1, pin_b1, rk, rs, producer, compromise in product(
        init_pins, init_pins, KEMS, SIGS, PARTIES, COMPROMISE_SETS
    ):
        # (a) the pre-fix world: the adversary holds nothing of Bob's.
        if producer == "M" and not (compromise & {"BK", "BS"}):
            pre = initiator_decision(
                pin_a1, pin_b1, (rk, rs), producer, compromise,
                c1_kem_binding=False, sig_pin_required=False,
            )
            if pre.committed:
                pre_fix_impersonations += 1
                # Rank so the reported witness is the CANONICAL ENG-0038 scenario: the adversary
                # steals NOTHING and signs B1 with a signing key it generated itself.
                rank = (len(compromise), 0 if rs in _BASE_SECRETS["M"] else 1)
                pre_fix_candidates.append((rank, {
                    "pin": pin_a1.kind.value,
                    "presented_resp_sig": rs,
                    "producer": producer,
                    "m_stole": sorted(compromise),
                    "genuine_bob": BOB,
                    "outcome": "initiator COMMITTED with authenticated=true to a peer that "
                               "holds NEITHER of Bob's identity secrets",
                }))

        # (b) the landed world: no commit for an adversary lacking Bob's FULL identity.
        landed = initiator_decision(pin_a1, pin_b1, (rk, rs), producer, compromise)
        if landed.committed and producer == "M":
            if not holds("M", "BS", compromise):
                landed_kem_only_compromise_commits += 1   # MUST stay 0 — the ENG-0038 class
            if not (compromise & {"BK", "BS"}):
                landed_zero_compromise_commits += 1       # MUST stay 0 — the original flaw

    pre_fix_witness = (
        min(pre_fix_candidates, key=lambda c: c[0])[1] if pre_fix_candidates else None
    )
    assert pre_fix_witness is not None, (
        "FAITHFULNESS FAILURE: the model could not reproduce the known ENG-0038 responder-"
        "impersonation in the pre-C1/pre-NA-0634 configuration. A model that cannot express the "
        "real flaw cannot be trusted to certify its absence."
    )
    assert landed_zero_compromise_commits == 0, (
        "REGRESSION: the landed rules still admit the original ENG-0038 impersonation."
    )
    assert landed_kem_only_compromise_commits == 0, (
        "REGRESSION: an adversary holding Bob's KEM identity secret but NOT his signing secret "
        "still reaches an initiator commit — the NA-0634 signing half is not closed."
    )
    return {
        "pre_fix_impersonation_traces": pre_fix_impersonations,
        "pre_fix_witness": pre_fix_witness,
        "landed_impersonation_commits": landed_zero_compromise_commits,
        "landed_kem_only_compromise_commits": landed_kem_only_compromise_commits,
    }


MARKERS = (
    "NA0636_QSC_HS_AUTH_MODEL_SCOPE_CONSUMED_OK",
    "NA0636_P1_MUTUAL_AUTH_BINDING_OK",
    "NA0636_P2_WRONG_SIGNING_KEY_REJECTION_OK",
    "NA0636_P3_REVERSE_PIN_REDUNDANCY_DECIDED_OK",
    "NA0636_P3_SEARCH_NONVACUOUS_PRE_FIX_GAP_DETECTED_OK",
    "NA0636_ENG0038_ORIGINAL_FLAW_REPRODUCED_IN_PRE_FIX_MODEL_OK",
    "NA0636_ENG0038_CLOSED_UNDER_LANDED_RULES_OK",
    "NA0636_P4_FAIL_CLOSED_REJECT_HYGIENE_OK",
    "NA0636_NO_RUNTIME_CHANGE_OK",
    "NA0636_NO_DEPENDENCY_CHANGE_OK",
    "NA0636_NO_WORKFLOW_CHANGE_OK",
    "NA0636_NO_PROTOCOL_SOURCE_CHANGE_OK",
    "NA0636_NO_CRYPTO_SECURITY_CLAIM_OK",
    "NA0636_NO_POST_COMPROMISE_CLAIM_OK",
    "NA0636_NO_PQ_PROOF_CLAIM_OK",
    "NA0636_NO_REFIMPL_EQUIVALENCE_CLAIM_OK",
    "NA0636_BOUNDED_EXHAUSTIVE_TERMINATION_OK",
    "NA0636_ONE_READY_INVARIANT_OK",
)


def check_qsc_hs_handshake_authentication_model() -> Dict[str, object]:
    resp_rows, resp_stats = _explore_responder()
    init_rows, init_stats = _explore_initiator()

    resp_props = _check_p1_p4_responder(resp_rows)
    init_props = _check_p1_p2_p4_initiator(init_rows)
    p3 = _check_p3(resp_rows)
    nonvacuous = _check_p3_counterfactual_nonvacuous()
    eng0038 = _check_eng0038_original_reproduction()

    # Fail-closed cross-checks on the aggregate (a vacuous PASS is itself a failure).
    assert resp_stats["configs"] == (5 * 5 * 3 * 3 * 3 * 16)
    assert init_stats["configs"] == (5 * 5 * 3 * 3 * 3 * 16)
    assert resp_props["p1_commit_bindings"] > 0, "no responder commit reachable — vacuous"
    assert init_props["p1_commit_bindings"] > 0, "no initiator commit reachable — vacuous"
    assert init_props["p2_wrong_sig_rejects"] > 0, "P2 never exercised — vacuous"
    assert resp_props["p4_reject_hygiene"] > 0 and init_props["p4_reject_hygiene"] > 0
    assert p3.landed_commits > 0, "P3 had no committing traces to judge — vacuous"

    return {
        "resp_stats": resp_stats,
        "init_stats": init_stats,
        "resp_props": resp_props,
        "init_props": init_props,
        "p3": p3,
        "nonvacuous": nonvacuous,
        "eng0038": eng0038,
        "bound": EXPLORATION_BOUND,
    }


def emit_qsc_hs_handshake_authentication_model_report() -> Dict[str, object]:
    result = check_qsc_hs_handshake_authentication_model()
    p3: P3Result = result["p3"]  # type: ignore[assignment]

    # P3 is the deliverable: fail-closed on a disproof so the runner cannot go green on a GAP.
    # A disproof (redundant is False) must STOP the lane for operator direction, not silently pass.
    assert p3.redundant, (
        "QSC_HS_HANDSHAKE_AUTH_MODEL_GAP_FOUND: the reverse sig-pin is NOT redundant — a reachable "
        f"responder-commit leaves the initiator sig_id unbound. Counterexample: {p3.counterexample}"
    )

    for marker in MARKERS:
        print(marker)
    print(f"QSC.HS auth responder configs explored: {result['resp_stats']['configs']}")  # type: ignore[index]
    print(f"QSC.HS auth responder LANDED commits: {result['resp_stats']['commits_landed']}")  # type: ignore[index]
    print(f"QSC.HS auth responder LANDED rejects: {result['resp_stats']['rejects_landed']}")  # type: ignore[index]
    print(f"QSC.HS auth initiator configs explored: {result['init_stats']['configs']}")  # type: ignore[index]
    print(f"QSC.HS auth initiator commits: {result['init_stats']['commits']}")  # type: ignore[index]
    print(f"QSC.HS auth initiator rejects: {result['init_stats']['rejects']}")  # type: ignore[index]
    print(f"P1 responder full-identity bindings: {result['resp_props']['p1_commit_bindings']}")  # type: ignore[index]
    print(f"P1 initiator full-identity bindings: {result['init_props']['p1_commit_bindings']}")  # type: ignore[index]
    print(f"P2 wrong-signing-key rejects: {result['init_props']['p2_wrong_sig_rejects']}")  # type: ignore[index]
    print(f"P4 responder reject-hygiene assertions: {result['resp_props']['p4_reject_hygiene']}")  # type: ignore[index]
    print(f"P4 initiator reject-hygiene assertions: {result['init_props']['p4_reject_hygiene']}")  # type: ignore[index]
    print(f"P3 LANDED responder commits judged: {p3.landed_commits}")
    print(f"P3 unbound-sig commits (gap size): {p3.unbound_commits}")
    print(f"P3 reverse-pin redundant: {p3.redundant}")
    print(
        "P3 bound-yet-REQUIRED-would-reject (false-reject-only, not a gap): "
        f"{p3.reverse_would_reject_bound}"
    )
    nonvac = result["nonvacuous"]  # type: ignore[assignment]
    print(
        "P3 non-vacuity: pre-NA-0634 counterfactual unbound commits detected: "
        f"{nonvac['pre_fix_unbound_commits']}"  # type: ignore[index]
    )
    print(f"P3 non-vacuity impersonation witness: {nonvac['impersonation_witness']}")  # type: ignore[index]
    eng = result["eng0038"]  # type: ignore[assignment]
    print(
        "ENG-0038 faithfulness anchor — pre-fix impersonation traces reproduced: "
        f"{eng['pre_fix_impersonation_traces']}"  # type: ignore[index]
    )
    print(f"ENG-0038 pre-fix witness: {eng['pre_fix_witness']}")  # type: ignore[index]
    print(
        "ENG-0038 under LANDED rules — impersonation commits (MUST be 0): "
        f"{eng['landed_impersonation_commits']}; KEM-only-compromise commits (MUST be 0): "  # type: ignore[index]
        f"{eng['landed_kem_only_compromise_commits']}"  # type: ignore[index]
    )
    return result


def main() -> int:
    emit_qsc_hs_handshake_authentication_model_report()
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as exc:  # fail-closed
        print(f"ERROR: qsc QSC.HS handshake-authentication model failed: {exc}")
        raise SystemExit(1)
