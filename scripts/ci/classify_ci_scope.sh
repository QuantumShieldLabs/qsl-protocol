#!/usr/bin/env bash
set -euo pipefail

is_docs_path() {
  if [[ "$1" =~ ^tests/.+\.md$ ]]; then
    return 0
  fi
  case "$1" in
    NEXT_ACTIONS.md|TRACEABILITY.md|DECISIONS.md|STATUS.md|README.md|SECURITY.md|SUPPORT.md|CONTRIBUTING.md|CODE_OF_CONDUCT.md|THIRD_PARTY_NOTICES.md|LICENSE|docs/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

is_workflow_path() {
  case "$1" in
    .github/workflows/*|.github/actions/*|scripts/ci/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

emit_result() {
  printf 'docs_only=%s\n' "$1"
  printf 'workflow_security=%s\n' "$2"
  printf 'runtime_critical=%s\n' "$3"
  printf 'scope_class=%s\n' "$4"
}

docs_only=true
workflow_security=false
runtime_critical=false

declare -a paths=("$@")

if [ "${#paths[@]}" -eq 0 ]; then
  if [ "${GITHUB_EVENT_NAME:-}" != "pull_request" ]; then
    docs_only=false
    runtime_critical=true
  else
    : "${BASE_SHA:?BASE_SHA is required for pull_request classification}"
    : "${HEAD_SHA:?HEAD_SHA is required for pull_request classification}"
    mapfile -t paths < <(git diff --name-only "${BASE_SHA}" "${HEAD_SHA}" | LC_ALL=C sort)
  fi
fi

if [ "${#paths[@]}" -eq 0 ] && [ "${GITHUB_EVENT_NAME:-}" = "pull_request" ]; then
  docs_only=false
  runtime_critical=true
fi

for path in "${paths[@]}"; do
  [ -n "$path" ] || continue
  if is_workflow_path "$path"; then
    docs_only=false
    workflow_security=true
    continue
  fi
  if is_docs_path "$path"; then
    continue
  fi
  docs_only=false
  runtime_critical=true
done

scope_class="docs_only"
if [ "$runtime_critical" = true ] && [ "$workflow_security" = true ]; then
  scope_class="runtime_and_workflow"
elif [ "$runtime_critical" = true ]; then
  scope_class="runtime_critical"
elif [ "$workflow_security" = true ]; then
  scope_class="workflow_security"
fi

emit_result "$docs_only" "$workflow_security" "$runtime_critical" "$scope_class"
