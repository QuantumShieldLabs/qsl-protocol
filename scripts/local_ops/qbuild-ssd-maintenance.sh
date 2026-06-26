#!/usr/bin/env bash
set -Eeuo pipefail

VERSION="NA-0543-qbuild-ssd-maintenance-v0.1.0"

MODE="dry-run"
TARGET_DAYS=7
TMP_DAYS=7
LOG_RETENTION_DAYS=90
FIXTURE_ROOT=""
ACTIVE_PROCESS_FILE=""
NOW_EPOCH=""

WORK_ROOT="/srv/qbuild/work"
TMP_ROOT="/srv/qbuild/tmp"
CACHE_ROOT="/srv/qbuild/cache"
BACKUP_ROOT="/backup/qsl"
LOCK_ROOT="/var/lock"

EXIT_SUCCESS_NO_CANDIDATES=0
EXIT_FAILURE=2
EXIT_SAFE_SKIP_ACTIVE=3
EXIT_WARNING=4
EXIT_SUCCESS_RECLAIMED=10

classification="NOT_STARTED"
safe_skip_reason=""
failure_reason=""
target_candidate_count=0
target_candidate_bytes=0
proof_candidate_count=0
proof_candidate_bytes=0
broken_symlink_count=0
deleted_count=0
deleted_bytes=0
archived_count=0
archived_bytes=0
failed_count=0
reclaimed_bytes=0
disk_before=""
disk_after=""
log_file=""
json_file=""

declare -a target_candidates=()
declare -a proof_candidates=()
declare -a deleted_paths=()
declare -a archived_paths=()
declare -a skipped_paths=()
declare -a failed_paths=()
declare -a broken_symlinks=()

usage() {
  printf '%s\n' \
    "Usage: qbuild-ssd-maintenance.sh [--dry-run|--apply] [--target-days N] [--tmp-days N]" \
    "                                  [--fixture-root DIR] [--active-process-file FILE]" \
    "" \
    "Default mode is dry-run. Apply mode requires uid 0 outside explicit fixture mode." \
    "Fixture mode maps roots under DIR and refuses live qbuild/backup roots."
}

die() {
  failure_reason="${1:-failure}"
  classification="HARD_FAILURE"
  printf 'ERROR: %s\n' "$failure_reason" >&2
  write_summary "$EXIT_FAILURE" || true
  exit "$EXIT_FAILURE"
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

json_array() {
  local first=1 item
  printf '['
  for item in "$@"; do
    if [ "$first" -eq 0 ]; then
      printf ','
    fi
    first=0
    printf '"%s"' "$(json_escape "$item")"
  done
  printf ']'
}

write_summary() {
  local exit_code="${1:-$EXIT_FAILURE}"
  local tmp_json
  if [ -z "$json_file" ]; then
    return 0
  fi
  tmp_json="${json_file}.tmp.$$"
  {
    printf '{\n'
    printf '  "version": "%s",\n' "$(json_escape "$VERSION")"
    printf '  "mode": "%s",\n' "$(json_escape "$MODE")"
    printf '  "fixture_mode": %s,\n' "$([ -n "$FIXTURE_ROOT" ] && printf true || printf false)"
    printf '  "classification": "%s",\n' "$(json_escape "$classification")"
    printf '  "exit_code": %s,\n' "$exit_code"
    printf '  "safe_skip_reason": "%s",\n' "$(json_escape "$safe_skip_reason")"
    printf '  "failure_reason": "%s",\n' "$(json_escape "$failure_reason")"
    printf '  "work_root": "%s",\n' "$(json_escape "$WORK_ROOT")"
    printf '  "tmp_root": "%s",\n' "$(json_escape "$TMP_ROOT")"
    printf '  "cache_root": "%s",\n' "$(json_escape "$CACHE_ROOT")"
    printf '  "backup_root": "%s",\n' "$(json_escape "$BACKUP_ROOT")"
    printf '  "target_days": %s,\n' "$TARGET_DAYS"
    printf '  "tmp_days": %s,\n' "$TMP_DAYS"
    printf '  "target_candidate_count": %s,\n' "$target_candidate_count"
    printf '  "target_candidate_bytes": %s,\n' "$target_candidate_bytes"
    printf '  "proof_candidate_count": %s,\n' "$proof_candidate_count"
    printf '  "proof_candidate_bytes": %s,\n' "$proof_candidate_bytes"
    printf '  "broken_symlink_count": %s,\n' "$broken_symlink_count"
    printf '  "deleted_count": %s,\n' "$deleted_count"
    printf '  "deleted_bytes": %s,\n' "$deleted_bytes"
    printf '  "archived_count": %s,\n' "$archived_count"
    printf '  "archived_bytes": %s,\n' "$archived_bytes"
    printf '  "failed_count": %s,\n' "$failed_count"
    printf '  "reclaimed_bytes": %s,\n' "$reclaimed_bytes"
    printf '  "disk_before": "%s",\n' "$(json_escape "$disk_before")"
    printf '  "disk_after": "%s",\n' "$(json_escape "$disk_after")"
    printf '  "target_candidates": '
    json_array "${target_candidates[@]}"
    printf ',\n  "proof_candidates": '
    json_array "${proof_candidates[@]}"
    printf ',\n  "deleted_paths": '
    json_array "${deleted_paths[@]}"
    printf ',\n  "archived_paths": '
    json_array "${archived_paths[@]}"
    printf ',\n  "skipped_paths": '
    json_array "${skipped_paths[@]}"
    printf ',\n  "failed_paths": '
    json_array "${failed_paths[@]}"
    printf ',\n  "broken_symlinks": '
    json_array "${broken_symlinks[@]}"
    printf '\n}\n'
  } > "$tmp_json"
  mv -f -- "$tmp_json" "$json_file"
}

require_uint() {
  local name="${1:?missing name}" value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    printf 'ERROR: %s must be an unsigned integer\n' "$name" >&2
    exit "$EXIT_FAILURE"
  fi
}

safe_realpath() {
  realpath -m -- "$1"
}

reject_dangerous_root() {
  local name="${1:?missing name}" path="${2:-}" real
  if [ -z "$path" ]; then
    printf 'ERROR: empty %s\n' "$name" >&2
    exit "$EXIT_FAILURE"
  fi
  real="$(safe_realpath "$path")"
  case "$real" in
    /|/srv|/backup|/srv/..*|/backup/..*|*'/../'*|*'/..')
      printf 'ERROR: dangerous %s: %s\n' "$name" "$path" >&2
      exit "$EXIT_FAILURE"
      ;;
  esac
  printf '%s' "$real"
}

require_under() {
  local parent="${1:?missing parent}" child="${2:?missing child}"
  local parent_real child_real
  parent_real="$(safe_realpath "$parent")"
  child_real="$(safe_realpath "$child")"
  case "$child_real" in
    "$parent_real"/*) ;;
    *) die "path escapes root: $child_real not under $parent_real" ;;
  esac
}

validate_candidate_basename() {
  local base="${1:-}"
  if [[ ! "$base" =~ ^[A-Za-z0-9._-]+$ ]] || [[ "$base" == *..* ]]; then
    return 1
  fi
}

setup_fixture_roots() {
  local root real
  root="${1:?missing fixture root}"
  real="$(safe_realpath "$root")"
  case "$real" in
    /srv/qbuild|/srv/qbuild/work|/srv/qbuild/work/*|/srv/qbuild/cache|/srv/qbuild/cache/*|/srv/qbuild/mirrors|/srv/qbuild/mirrors/*|/backup|/backup/*)
      printf 'ERROR: fixture root must not be a live qbuild or backup path: %s\n' "$real" >&2
      exit "$EXIT_FAILURE"
      ;;
  esac
  FIXTURE_ROOT="$real"
  WORK_ROOT="$FIXTURE_ROOT/srv/qbuild/work"
  TMP_ROOT="$FIXTURE_ROOT/srv/qbuild/tmp"
  CACHE_ROOT="$FIXTURE_ROOT/srv/qbuild/cache"
  BACKUP_ROOT="$FIXTURE_ROOT/backup/qsl"
  LOCK_ROOT="$FIXTURE_ROOT/var/lock"
}

parse_args() {
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --dry-run) MODE="dry-run" ;;
      --apply) MODE="apply" ;;
      --target-days)
        TARGET_DAYS="${2:?missing --target-days value}"
        shift
        ;;
      --tmp-days)
        TMP_DAYS="${2:?missing --tmp-days value}"
        shift
        ;;
      --log-retention-days)
        LOG_RETENTION_DAYS="${2:?missing --log-retention-days value}"
        shift
        ;;
      --fixture-root)
        setup_fixture_roots "${2:?missing --fixture-root value}"
        shift
        ;;
      --active-process-file)
        ACTIVE_PROCESS_FILE="${2:?missing --active-process-file value}"
        shift
        ;;
      --now-epoch)
        NOW_EPOCH="${2:?missing --now-epoch value}"
        shift
        ;;
      --help|-h)
        usage
        exit 0
        ;;
      *)
        printf 'ERROR: unknown argument: %s\n' "$1" >&2
        usage >&2
        exit "$EXIT_FAILURE"
        ;;
    esac
    shift
  done
  require_uint TARGET_DAYS "$TARGET_DAYS"
  require_uint TMP_DAYS "$TMP_DAYS"
  require_uint LOG_RETENTION_DAYS "$LOG_RETENTION_DAYS"
  if [ -n "$NOW_EPOCH" ]; then
    require_uint NOW_EPOCH "$NOW_EPOCH"
  fi
}

prepare_roots() {
  WORK_ROOT="$(reject_dangerous_root WORK_ROOT "$WORK_ROOT")"
  TMP_ROOT="$(reject_dangerous_root TMP_ROOT "$TMP_ROOT")"
  CACHE_ROOT="$(reject_dangerous_root CACHE_ROOT "$CACHE_ROOT")"
  BACKUP_ROOT="$(reject_dangerous_root BACKUP_ROOT "$BACKUP_ROOT")"
  LOCK_ROOT="$(reject_dangerous_root LOCK_ROOT "$LOCK_ROOT")"

  if [ "$MODE" = "apply" ] && [ -z "$FIXTURE_ROOT" ] && [ "$(id -u)" -ne 0 ]; then
    printf 'ERROR: --apply requires root outside fixture mode\n' >&2
    exit "$EXIT_FAILURE"
  fi

  if [ -n "$FIXTURE_ROOT" ]; then
    if [ ! -f "$BACKUP_ROOT/.qbuild-fixture-mounted" ]; then
      printf 'ERROR: fixture mount marker missing: %s/.qbuild-fixture-mounted\n' "$BACKUP_ROOT" >&2
      exit "$EXIT_FAILURE"
    fi
  elif ! mountpoint -q "$BACKUP_ROOT"; then
    printf 'ERROR: backup root is not a mount point: %s\n' "$BACKUP_ROOT" >&2
    exit "$EXIT_FAILURE"
  fi

  mkdir -p "$LOCK_ROOT" "$BACKUP_ROOT/qbuild-tmp-archive/housekeeping-logs"
}

open_lock() {
  local lock="$LOCK_ROOT/qbuild-ssd-maintenance.lock"
  exec 9>"$lock"
  if ! flock -n 9; then
    die "another qbuild-ssd-maintenance run holds the lock"
  fi
}

init_logs() {
  local log_root stamp
  log_root="$BACKUP_ROOT/qbuild-tmp-archive/housekeeping-logs"
  stamp="$(date -u +%Y%m%dT%H%M%SZ)"
  log_file="$log_root/qbuild-ssd-maintenance_${stamp}.log"
  json_file="$log_root/qbuild-ssd-maintenance_${stamp}.json"
  exec > >(tee -a "$log_file") 2>&1
}

disk_line() {
  df -P "$1" | awk 'NR==2 {print $1 " size_k=" $2 " used_k=" $3 " avail_k=" $4 " use=" $5 " mount=" $6}'
}

root_use_percent() {
  df -P / | awk 'NR==2 {gsub("%","",$5); print $5}'
}

capture_disk_before() {
  disk_before="$(disk_line /)"
  printf 'DISK_BEFORE %s\n' "$disk_before"
  if [ "$BACKUP_ROOT" != "/" ]; then
    printf 'BACKUP_DISK_BEFORE %s\n' "$(disk_line "$BACKUP_ROOT")"
  fi
}

capture_disk_after() {
  disk_after="$(disk_line /)"
  printf 'DISK_AFTER %s\n' "$disk_after"
  if [ "$BACKUP_ROOT" != "/" ]; then
    printf 'BACKUP_DISK_AFTER %s\n' "$(disk_line "$BACKUP_ROOT")"
  fi
}

is_ancestor_pid() {
  local candidate="${1:?missing pid}" current parent
  current="$$"
  while [ -n "$current" ] && [ "$current" != "0" ]; do
    if [ "$candidate" = "$current" ]; then
      return 0
    fi
    parent="$(ps -o ppid= -p "$current" 2>/dev/null | awk '{print $1}')"
    current="${parent:-0}"
  done
  return 1
}

active_process_lines() {
  if [ -n "$ACTIVE_PROCESS_FILE" ]; then
    cat -- "$ACTIVE_PROCESS_FILE"
  else
    ps -eo pid=,ppid=,comm=,args=
  fi
}

detect_active_build() {
  local line pid ppid comm rest active=0
  local active_tmp
  active_tmp="$(mktemp)"
  while IFS= read -r line; do
    [ -n "$line" ] || continue
    pid="$(awk '{print $1}' <<<"$line")"
    ppid="$(awk '{print $2}' <<<"$line")"
    comm="$(awk '{print $3}' <<<"$line")"
    rest="${line#*"$comm"}"
    if [ -z "$pid" ] || [ "$pid" = "$$" ] || is_ancestor_pid "$pid"; then
      continue
    fi
    case "$comm:$rest" in
      cargo:*|rustc:*|sccache:*|qwork:*|qstart:*|qresume:*|*:*/cargo[[:space:]]*|*:*/rustc[[:space:]]*|*:*/sccache[[:space:]]*|*:*/qwork[[:space:]]*|*:*/qstart[[:space:]]*|*:*/qresume[[:space:]]*)
        printf '%s\n' "$line" >> "$active_tmp"
        active=1
        ;;
    esac
  done < <(active_process_lines)

  if [ "$active" -eq 1 ]; then
    safe_skip_reason="active cargo/rustc/sccache/qwork/qstart/qresume process detected"
    classification="SAFE_SKIP_ACTIVE_BUILD"
    printf 'SAFE_SKIP_ACTIVE_BUILD %s\n' "$safe_skip_reason"
    sed -n '1,40p' "$active_tmp"
    rm -f "$active_tmp"
    write_summary "$EXIT_SAFE_SKIP_ACTIVE"
    exit "$EXIT_SAFE_SKIP_ACTIVE"
  fi
  rm -f "$active_tmp"
}

newest_descendant_epoch() {
  local path="${1:?missing path}"
  find "$path" -printf '%T@\n' 2>/dev/null | awk 'BEGIN {max=0} {if ($1 > max) max=$1} END {printf "%.0f\n", max}'
}

path_size_bytes() {
  du -sb -- "$1" 2>/dev/null | awk '{print $1}'
}

older_than_days_by_newest_descendant() {
  local path="${1:?missing path}" days="${2:?missing days}" now newest cutoff
  now="${NOW_EPOCH:-$(date +%s)}"
  newest="$(newest_descendant_epoch "$path")"
  cutoff=$((now - (days * 86400)))
  [ "$newest" -le "$cutoff" ]
}

validate_target_candidate() {
  local target="${1:?missing target}" real
  real="$(safe_realpath "$target")"
  require_under "$WORK_ROOT" "$real"
  case "$real" in
    "$WORK_ROOT"/*/qsl-protocol/target) ;;
    *) die "invalid target cleanup candidate: $target" ;;
  esac
  case "$real" in
    "$CACHE_ROOT"|"$CACHE_ROOT"/*) die "refusing shared cache target cleanup: $target" ;;
  esac
  if [ -L "$target" ]; then
    die "refusing symlink target cleanup candidate: $target"
  fi
}

validate_proof_candidate() {
  local dir="${1:?missing dir}" real base
  real="$(safe_realpath "$dir")"
  require_under "$TMP_ROOT" "$real"
  base="$(basename -- "$real")"
  validate_candidate_basename "$base" || die "unsafe proof-root candidate basename: $base"
  case "$base" in
    NA*) ;;
    *) die "invalid proof-root candidate: $dir" ;;
  esac
  if [ -L "$dir" ]; then
    die "refusing symlink proof-root candidate: $dir"
  fi
}

collect_broken_symlinks() {
  local link
  while IFS= read -r -d '' link; do
    broken_symlinks+=("$link")
  done < <(find "$TMP_ROOT" -mindepth 1 -maxdepth 1 -type l ! -exec test -e {} \; -print0 2>/dev/null)
  broken_symlink_count="${#broken_symlinks[@]}"
}

collect_candidates() {
  local target dir bytes
  collect_broken_symlinks

  while IFS= read -r -d '' target; do
    validate_target_candidate "$target"
    if older_than_days_by_newest_descendant "$target" "$TARGET_DAYS"; then
      bytes="$(path_size_bytes "$target")"
      bytes="${bytes:-0}"
      target_candidates+=("$target")
      target_candidate_bytes=$((target_candidate_bytes + bytes))
    fi
  done < <(find "$WORK_ROOT" -path '*/qsl-protocol/target' -type d -prune -print0 2>/dev/null)
  target_candidate_count="${#target_candidates[@]}"

  while IFS= read -r -d '' dir; do
    validate_proof_candidate "$dir"
    if older_than_days_by_newest_descendant "$dir" "$TMP_DAYS"; then
      bytes="$(path_size_bytes "$dir")"
      bytes="${bytes:-0}"
      proof_candidates+=("$dir")
      proof_candidate_bytes=$((proof_candidate_bytes + bytes))
    fi
  done < <(find "$TMP_ROOT" -mindepth 1 -maxdepth 1 -type d -name 'NA*' -print0 2>/dev/null)
  proof_candidate_count="${#proof_candidates[@]}"
}

entry_count_and_bytes() {
  local path="${1:?missing path}"
  find "$path" -mindepth 1 -printf '%y %s\n' 2>/dev/null | awk '{count += 1; bytes += $2} END {printf "%s %s\n", count + 0, bytes + 0}'
}

verify_copy_equivalent() {
  local src="${1:?missing src}" dst="${2:?missing dst}"
  local src_stat dst_stat
  src_stat="$(entry_count_and_bytes "$src")"
  dst_stat="$(entry_count_and_bytes "$dst")"
  [ "$src_stat" = "$dst_stat" ]
}

archive_one_proof_root() {
  local dir="${1:?missing proof root}" archive_root base dest tmpdest moved bytes stamp
  validate_proof_candidate "$dir"
  archive_root="$BACKUP_ROOT/qbuild-tmp-archive/$(date -u +%Y)/$(date -u +%m)"
  mkdir -p "$archive_root"
  base="$(basename -- "$dir")"
  dest="$archive_root/$base"
  stamp="$(date -u +%Y%m%dT%H%M%SZ)"
  tmpdest="$archive_root/.${base}.tmp.${stamp}.$$"
  moved="${dir}.moved.${stamp}"

  if [ -e "$dest" ] || [ -L "$dest" ]; then
    failed_paths+=("$dir -> $dest")
    failed_count=$((failed_count + 1))
    return 1
  fi
  if [ -e "$tmpdest" ] || [ -L "$tmpdest" ]; then
    failed_paths+=("$dir -> $tmpdest")
    failed_count=$((failed_count + 1))
    return 1
  fi

  mkdir -p "$tmpdest"
  cp -a -- "$dir/." "$tmpdest/"
  verify_copy_equivalent "$dir" "$tmpdest" || return 1
  mv -- "$tmpdest" "$dest"
  mv -- "$dir" "$moved"
  ln -s -- "$dest" "$dir"
  if [ "$(readlink -- "$dir")" != "$dest" ]; then
    failed_paths+=("$dir symlink verification failed")
    failed_count=$((failed_count + 1))
    return 1
  fi
  bytes="$(path_size_bytes "$moved")"
  bytes="${bytes:-0}"
  rm -rf -- "$moved"
  archived_paths+=("$dir -> $dest")
  archived_count=$((archived_count + 1))
  archived_bytes=$((archived_bytes + bytes))
}

delete_one_target() {
  local target="${1:?missing target}" bytes
  validate_target_candidate "$target"
  bytes="$(path_size_bytes "$target")"
  bytes="${bytes:-0}"
  rm -rf -- "$target"
  if [ -e "$target" ]; then
    failed_paths+=("$target")
    failed_count=$((failed_count + 1))
    return 1
  fi
  deleted_paths+=("$target")
  deleted_count=$((deleted_count + 1))
  deleted_bytes=$((deleted_bytes + bytes))
}

apply_candidates() {
  local target dir
  for target in "${target_candidates[@]}"; do
    printf 'DELETE_TARGET %s\n' "$target"
    delete_one_target "$target" || die "target deletion failed: $target"
  done
  for dir in "${proof_candidates[@]}"; do
    printf 'ARCHIVE_PROOF_ROOT %s\n' "$dir"
    archive_one_proof_root "$dir" || die "proof-root archive transaction failed: $dir"
  done
  reclaimed_bytes=$((deleted_bytes + archived_bytes))
}

prune_old_logs() {
  local log_root
  log_root="$BACKUP_ROOT/qbuild-tmp-archive/housekeeping-logs"
  if [ "$LOG_RETENTION_DAYS" -eq 0 ]; then
    return 0
  fi
  find "$log_root" -maxdepth 1 -type f \( -name 'qbuild-ssd-maintenance_*.log' -o -name 'qbuild-ssd-maintenance_*.json' \) -mtime +"$LOG_RETENTION_DAYS" -print -delete 2>/dev/null | sed 's/^/PRUNE_OLD_LOG /'
}

print_report() {
  local path
  printf 'QBUILD_SSD_MAINTENANCE_VERSION=%s\n' "$VERSION"
  printf 'MODE=%s\n' "$MODE"
  printf 'TARGET_DAYS=%s\n' "$TARGET_DAYS"
  printf 'TMP_DAYS=%s\n' "$TMP_DAYS"
  printf 'WORK_ROOT=%s\n' "$WORK_ROOT"
  printf 'TMP_ROOT=%s\n' "$TMP_ROOT"
  printf 'BACKUP_ROOT=%s\n' "$BACKUP_ROOT"
  printf 'LOG_FILE=%s\n' "$log_file"
  printf 'JSON_FILE=%s\n' "$json_file"
  printf 'BROKEN_ARCHIVE_SYMLINK_COUNT=%s\n' "$broken_symlink_count"
  for path in "${broken_symlinks[@]}"; do
    printf 'BROKEN_ARCHIVE_SYMLINK %s\n' "$path"
  done
  printf 'TARGET_CANDIDATE_COUNT=%s\n' "$target_candidate_count"
  printf 'TARGET_CANDIDATE_BYTES=%s\n' "$target_candidate_bytes"
  for path in "${target_candidates[@]}"; do
    printf 'TARGET_CANDIDATE %s\n' "$path"
  done
  printf 'PROOF_ROOT_CANDIDATE_COUNT=%s\n' "$proof_candidate_count"
  printf 'PROOF_ROOT_CANDIDATE_BYTES=%s\n' "$proof_candidate_bytes"
  for path in "${proof_candidates[@]}"; do
    printf 'PROOF_ROOT_CANDIDATE %s\n' "$path"
  done
}

finish_with_policy_exit() {
  local root_use exit_code
  root_use="$(root_use_percent)"
  if [ "${root_use:-100}" -ge 95 ]; then
    classification="HARD_FAILURE_ROOT_USAGE_95_PLUS"
    failure_reason="root filesystem usage ${root_use}% is at or above hard stop"
    write_summary "$EXIT_FAILURE"
    printf 'HARD_STOP_ROOT_USAGE_PERCENT=%s\n' "$root_use"
    exit "$EXIT_FAILURE"
  fi
  if [ "${root_use:-100}" -ge 80 ]; then
    classification="WARNING_DISK_PRESSURE"
    write_summary "$EXIT_WARNING"
    printf 'WARNING_ROOT_USAGE_PERCENT=%s\n' "$root_use"
    exit "$EXIT_WARNING"
  fi

  if [ "$MODE" = "apply" ] && [ "$reclaimed_bytes" -gt 0 ]; then
    classification="SUCCESS_RECLAIMED"
    exit_code="$EXIT_SUCCESS_RECLAIMED"
  elif [ "$MODE" = "dry-run" ] && { [ "$target_candidate_count" -gt 0 ] || [ "$proof_candidate_count" -gt 0 ]; }; then
    classification="DRY_RUN_CANDIDATES"
    exit_code="$EXIT_SUCCESS_NO_CANDIDATES"
  else
    classification="SUCCESS_NO_CANDIDATES"
    exit_code="$EXIT_SUCCESS_NO_CANDIDATES"
  fi
  write_summary "$exit_code"
  exit "$exit_code"
}

main() {
  parse_args "$@"
  prepare_roots
  open_lock
  init_logs
  printf '=== qbuild SSD maintenance ===\n'
  date -u
  capture_disk_before
  detect_active_build
  collect_candidates
  print_report
  if [ "$MODE" = "dry-run" ]; then
    printf 'DRY_RUN_ONLY yes\n'
  else
    apply_candidates
    prune_old_logs
  fi
  capture_disk_after
  finish_with_policy_exit
}

main "$@"
