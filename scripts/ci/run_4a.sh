#!/usr/bin/env bash
set -euo pipefail

RUN_ID="${RUN_ID:-$(date -u +%Y%m%dT%H%M%SZ)_$(git rev-parse --short=7 HEAD 2>/dev/null || echo local)}"
OUT_DIR="${OUT_DIR:-artifacts/${RUN_ID}/4A}"
mkdir -p "${OUT_DIR}"

echo "[4A] Run ID: ${RUN_ID}"
echo "[4A] Output: ${OUT_DIR}"

PHASE2_ZIP="inputs/phase2/QuantumShield_Phase2_CANONICAL_FROZEN_QSP4.3.2_QSE1.8.2.zip"
PHASE3_ZIP="inputs/phase3/QuantumShield_Phase3_SUPPORTING_COMPLETE_P3-02_to_P3-30.zip"

python3 scripts/ci/verify_inputs.py   --phase2-zip "${PHASE2_ZIP}"   --phase2-sha inputs/phase2/phase2.zip.sha256   --phase3-zip "${PHASE3_ZIP}"   --phase3-sha inputs/phase3/phase3.zip.sha256   --phase3-lock inputs/phase3/phase3.lock.json   --out "${OUT_DIR}/A1_bundle_integrity.json"   --ledger-out "${OUT_DIR}/A1_phase3_ledger_crosscheck.json"   --errata-md "${OUT_DIR}/../phase4_errata/phase3_packaging_errata.md"   2>&1 | tee "${OUT_DIR}/A1_bundle_integrity.log"

python3 scripts/ci/validate_openapi.py   --require-secondary   --phase3-zip "${PHASE3_ZIP}"   --out "${OUT_DIR}/A2_openapi_validation.json"   2>&1 | tee "${OUT_DIR}/A2_openapi_validation.log"

python3 scripts/ci/validate_schemas.py   --phase3-zip "${PHASE3_ZIP}"   --out "${OUT_DIR}/A3_schema_validation.json"   2>&1 | tee "${OUT_DIR}/A3_schema_validation.log"

python3 scripts/ci/qse_bounds_audit.py   --phase2-zip "${PHASE2_ZIP}"   --phase3-zip "${PHASE3_ZIP}"   --out "${OUT_DIR}/A4_bounds_results.json"   2>&1 | tee "${OUT_DIR}/A4_bounds_results.log"

echo "[4A] Complete."
