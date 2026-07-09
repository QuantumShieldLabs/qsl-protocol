#!/usr/bin/env python3
"""Entry point for formal model checks.

Goals: G4

This is a fail-closed check runner.
- Any AssertionError or unexpected exception => non-zero exit.
- Successful completion => exit 0.

See formal/README.md for scope and limitations.
"""

from __future__ import annotations

import os
import sys

# Allow invocation as `python3 formal/run_model_checks.py` without requiring callers
# to set PYTHONPATH (CI-friendly, fail-closed).
REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

from formal.model_scka_bounded import explore  # noqa: E402
from formal.model_qsc_handshake_suite_id_bounded import (  # noqa: E402
    emit_qsc_handshake_suite_id_model_report,
)
from formal.model_qsc_kem_signature_transcript_binding_bounded import (  # noqa: E402
    emit_qsc_kem_signature_transcript_binding_model_report,
)
from formal.model_suite2_negotiation_bounded import (  # noqa: E402
    check_suite2_negotiation_model,
)
from formal.model_suite2_root_composition_bounded import (  # noqa: E402
    emit_suite2_root_composition_model_report,
)


def main() -> int:
    # Keep bounds intentionally small and stable; adjust only with an explicit decision.
    stats = explore(max_depth=8, max_net=6, max_seen=12)
    print("OK: SCKA bounded model checks passed")
    print(f"Explored states: {stats['states']}")
    print(f"Transitions: {stats['transitions']}")
    print(f"Unique visited: {stats['visited']}")
    negotiation_stats = check_suite2_negotiation_model()
    print("OK: Suite-2 negotiation downgrade/no-mutation model checks passed")
    print(f"Negotiation attempts: {negotiation_stats['attempts']}")
    print(f"Accepted outcomes: {negotiation_stats['accepted']}")
    print(f"Rejected outcomes: {negotiation_stats['rejected']}")
    print(f"Downgrade rejects: {negotiation_stats['downgrade_rejects']}")
    print(
        "Capability commitment rejects: "
        f"{negotiation_stats['capability_commitment_rejects']}"
    )
    print(f"AD mismatch rejects: {negotiation_stats['ad_mismatch_rejects']}")
    print(
        "No-mutation assertions: "
        f"{negotiation_stats['no_mutation_assertions']}"
    )
    print("NA-0309 qsc handshake suite-id bounded model checks")
    qsc_suite_id_stats = emit_qsc_handshake_suite_id_model_report()
    print("OK: qsc handshake suite-id formal model checks passed")
    print(f"QSC suite-id scenarios: {qsc_suite_id_stats['scenarios']}")
    print(f"QSC suite-id accepts: {qsc_suite_id_stats['accepted']}")
    print(f"QSC suite-id rejects: {qsc_suite_id_stats['rejected']}")
    print("NA-0478 qsc KEM/signature/transcript binding bounded model checks")
    qsc_binding_stats = emit_qsc_kem_signature_transcript_binding_model_report()
    print("OK: qsc KEM/signature/transcript binding formal model checks passed")
    print(f"QSC binding scenarios: {qsc_binding_stats['scenarios']}")
    print(f"QSC binding accepted traces: {qsc_binding_stats['accepted_traces']}")
    print(f"QSC binding rejected traces: {qsc_binding_stats['rejected_traces']}")
    print("NA-0625 Suite-2 root-composition bounded model checks")
    root_stats = emit_suite2_root_composition_model_report()
    print("OK: Suite-2 root-composition formal model checks passed")
    print(f"Root composition states: {root_stats['states']}")
    print(f"Root composition transitions: {root_stats['transitions']}")
    print(f"Root composition unique visited: {root_stats['visited']}")
    print(f"Root composition regression shapes: {root_stats['regression_shapes']}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except SystemExit:
        raise
    except Exception as e:
        print(f"ERROR: formal model checks failed: {e}", file=sys.stderr)
        raise SystemExit(1)
