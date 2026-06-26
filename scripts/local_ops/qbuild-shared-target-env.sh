#!/usr/bin/env bash
set -Eeuo pipefail

VERSION="NA-0543-qbuild-shared-target-env-v0.1.0"

REPO="qsl-protocol"
BUILD_CLASS="default"
FORMAT="shell"
BASE="/srv/qbuild/cache/targets"
FIXTURE_ROOT=""
PREPARE=0
OPERATOR_PREPARE=0
ISOLATED_TARGET=""
ISOLATED_REASON=""
SOURCE_HINT="qwork-default"

usage() {
  printf '%s\n' \
    "Usage: qbuild-shared-target-env.sh [--repo qsl-protocol] [--build-class default]" \
    "                                    [--format shell|json|proof] [--fixture-root DIR]" \
    "                                    [--prepare [--operator-prepare]]" \
    "                                    [--isolated-target DIR --isolated-reason REASON]" \
    "" \
    "The helper prints target-selection environment/proof data. It never overrides" \
    "a preexisting CARGO_TARGET_DIR and creates directories only in explicit prepare mode."
}

die() {
  printf 'ERROR: %s\n' "${1:-failure}" >&2
  exit 2
}

shell_quote() {
  printf '%q' "$1"
}

json_escape() {
  local s="${1-}"
  s="${s//\\/\\\\}"
  s="${s//\"/\\\"}"
  s="${s//$'\n'/\\n}"
  s="${s//$'\r'/\\r}"
  s="${s//$'\t'/\\t}"
  printf '%s' "$s"
}

safe_component() {
  local name="${1:?missing name}" value="${2:-}"
  if [ -z "$value" ]; then
    die "$name is empty"
  fi
  if [[ ! "$value" =~ ^[A-Za-z0-9._-]+$ ]] || [[ "$value" == *..* ]] || [[ "$value" == */* ]]; then
    die "$name has unsafe characters: $value"
  fi
}

safe_abs_path() {
  local name="${1:?missing name}" path="${2:-}" real
  if [ -z "$path" ]; then
    die "$name is empty"
  fi
  real="$(realpath -m -- "$path")"
  case "$real" in
    /|/srv|/backup|*'/../'*|*'/..')
      die "$name is dangerous: $path"
      ;;
  esac
  printf '%s' "$real"
}

parse_rustc_field() {
  local field="${1:?missing field}"
  rustc -vV | awk -v field="$field" '$1 == field":" {print $2; exit}'
}

rustc_release() {
  if [ -n "${QBUILD_TEST_RUSTC_RELEASE:-}" ]; then
    printf '%s\n' "$QBUILD_TEST_RUSTC_RELEASE"
  else
    parse_rustc_field release
  fi
}

rustc_host() {
  if [ -n "${QBUILD_TEST_RUSTC_HOST:-}" ]; then
    printf '%s\n' "$QBUILD_TEST_RUSTC_HOST"
  else
    parse_rustc_field host
  fi
}

setup_fixture() {
  local root real
  root="${1:?missing fixture root}"
  real="$(realpath -m -- "$root")"
  case "$real" in
    /srv/qbuild|/srv/qbuild/work|/srv/qbuild/work/*|/srv/qbuild/cache|/srv/qbuild/cache/*|/srv/qbuild/mirrors|/srv/qbuild/mirrors/*|/backup|/backup/*)
      die "fixture root must not be a live qbuild or backup path: $real"
      ;;
  esac
  FIXTURE_ROOT="$real"
  BASE="$FIXTURE_ROOT/srv/qbuild/cache/targets"
}

parse_args() {
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --repo)
        REPO="${2:?missing --repo value}"
        shift
        ;;
      --build-class)
        BUILD_CLASS="${2:?missing --build-class value}"
        shift
        ;;
      --format)
        FORMAT="${2:?missing --format value}"
        shift
        ;;
      --base)
        BASE="${2:?missing --base value}"
        shift
        ;;
      --fixture-root)
        setup_fixture "${2:?missing --fixture-root value}"
        shift
        ;;
      --prepare)
        PREPARE=1
        ;;
      --operator-prepare)
        OPERATOR_PREPARE=1
        ;;
      --isolated-target)
        ISOLATED_TARGET="${2:?missing --isolated-target value}"
        SOURCE_HINT="directive-isolated"
        shift
        ;;
      --isolated-reason)
        ISOLATED_REASON="${2:?missing --isolated-reason value}"
        shift
        ;;
      --source)
        SOURCE_HINT="${2:?missing --source value}"
        shift
        ;;
      --help|-h)
        usage
        exit 0
        ;;
      *)
        die "unknown argument: $1"
        ;;
    esac
    shift
  done
}

validate_inputs() {
  safe_component repo "$REPO"
  safe_component build_class "$BUILD_CLASS"
  case "$FORMAT" in
    shell|json|proof) ;;
    *) die "unsupported format: $FORMAT" ;;
  esac
  if [ "$REPO" != "qsl-protocol" ]; then
    die "repo is not authorized for the ordinary shared target: $REPO"
  fi
  if [ "$BUILD_CLASS" != "default" ]; then
    die "ordinary shared target build class must be default: $BUILD_CLASS"
  fi
  BASE="$(safe_abs_path base "$BASE")"
  if [ -n "$ISOLATED_TARGET" ]; then
    ISOLATED_TARGET="$(safe_abs_path isolated_target "$ISOLATED_TARGET")"
    [ -n "$ISOLATED_REASON" ] || die "isolated target requires --isolated-reason"
  fi
}

prepare_target_if_requested() {
  local target="${1:?missing target}"
  if [ "$PREPARE" -eq 0 ]; then
    return 0
  fi
  if [ -n "$FIXTURE_ROOT" ]; then
    mkdir -p -- "$target"
    chmod 2775 "$target"
    return 0
  fi
  if [ "$OPERATOR_PREPARE" -ne 1 ]; then
    die "live target preparation requires --operator-prepare"
  fi
  mkdir -p -- "$target"
  chmod 2775 "$target"
}

emit_shell() {
  local mode="$1" dir="$2" source="$3" build_class="$4" toolchain_key="$5" explicit_preserved="$6" ready="$7"
  printf 'export CARGO_TARGET_DIR=%s\n' "$(shell_quote "$dir")"
  printf 'export QBUILD_CARGO_TARGET_MODE=%s\n' "$(shell_quote "$mode")"
  printf 'export QBUILD_CARGO_TARGET_SOURCE=%s\n' "$(shell_quote "$source")"
  printf 'export QBUILD_CARGO_TARGET_BUILD_CLASS=%s\n' "$(shell_quote "$build_class")"
  printf 'export QBUILD_CARGO_TARGET_TOOLCHAIN_KEY=%s\n' "$(shell_quote "$toolchain_key")"
  printf 'export QBUILD_EXPLICIT_TARGET_PRESERVED=%s\n' "$(shell_quote "$explicit_preserved")"
  printf 'export QBUILD_SHARED_TARGET_READY=%s\n' "$(shell_quote "$ready")"
}

emit_json() {
  local mode="$1" dir="$2" source="$3" build_class="$4" toolchain_key="$5" explicit_preserved="$6" ready="$7"
  printf '{\n'
  printf '  "version": "%s",\n' "$(json_escape "$VERSION")"
  printf '  "repo": "%s",\n' "$(json_escape "$REPO")"
  printf '  "cargo_target_mode": "%s",\n' "$(json_escape "$mode")"
  printf '  "cargo_target_dir": "%s",\n' "$(json_escape "$dir")"
  printf '  "cargo_target_source": "%s",\n' "$(json_escape "$source")"
  printf '  "cargo_target_build_class": "%s",\n' "$(json_escape "$build_class")"
  printf '  "cargo_target_toolchain_key": "%s",\n' "$(json_escape "$toolchain_key")"
  printf '  "explicit_target_preserved": "%s",\n' "$(json_escape "$explicit_preserved")"
  printf '  "shared_target_ready": "%s"\n' "$(json_escape "$ready")"
  printf '}\n'
}

emit_proof() {
  local mode="$1" dir="$2" source="$3" build_class="$4" toolchain_key="$5" explicit_preserved="$6" ready="$7"
  printf 'cargo_target_mode=%s\n' "$mode"
  printf 'cargo_target_dir=%s\n' "$dir"
  printf 'cargo_target_source=%s\n' "$source"
  printf 'cargo_target_build_class=%s\n' "$build_class"
  printf 'cargo_target_toolchain_key=%s\n' "$toolchain_key"
  printf 'explicit_target_preserved=%s\n' "$explicit_preserved"
  printf 'shared_target_ready=%s\n' "$ready"
}

main() {
  local release host toolchain_key shared_dir mode dir source explicit_preserved ready
  parse_args "$@"
  validate_inputs

  release="$(rustc_release)"
  host="$(rustc_host)"
  safe_component rustc_release "$release"
  safe_component rustc_host "$host"
  toolchain_key="rustc-${release}-${host}"
  safe_component toolchain_key "$toolchain_key"
  shared_dir="$BASE/$REPO/$toolchain_key/$BUILD_CLASS"

  if [ -n "$ISOLATED_TARGET" ]; then
    mode="isolated"
    dir="$ISOLATED_TARGET"
    source="directive-isolated"
    explicit_preserved="yes"
    ready="$([ -d "$dir" ] && printf yes || printf no)"
  elif [ -n "${CARGO_TARGET_DIR:-}" ]; then
    mode="explicit"
    dir="$CARGO_TARGET_DIR"
    source="preexisting-env"
    explicit_preserved="yes"
    ready="not-applicable"
  else
    mode="shared"
    dir="$shared_dir"
    source="$SOURCE_HINT"
    explicit_preserved="no"
    prepare_target_if_requested "$dir"
    ready="$([ -d "$dir" ] && printf yes || printf no)"
  fi

  case "$FORMAT" in
    shell) emit_shell "$mode" "$dir" "$source" "$BUILD_CLASS" "$toolchain_key" "$explicit_preserved" "$ready" ;;
    json) emit_json "$mode" "$dir" "$source" "$BUILD_CLASS" "$toolchain_key" "$explicit_preserved" "$ready" ;;
    proof) emit_proof "$mode" "$dir" "$source" "$BUILD_CLASS" "$toolchain_key" "$explicit_preserved" "$ready" ;;
  esac
}

main "$@"
