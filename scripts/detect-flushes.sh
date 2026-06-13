#!/usr/bin/env bash
#
# detect-flushes.sh - find "free" counter flushes in the ClaudeMeter history db.
#
# Two things can drop your weekly utilization to ~0:
#   1. Scheduled reset  - your real weekly window fired. resets_at JUMPS FORWARD
#                         (~7 days). This is your normal cycle.
#   2. Free flush        - Anthropic zeroed the counter out of band (a global flush).
#                         utilization drops but resets_at DOES NOT advance. You get
#                         a fresh bucket while your real reset stays where it was.
#
# This script reads the local SQLite history and prints both kinds, so a drop you
# see in the menu bar can be classified without guessing. It is read-only.
#
# Usage:
#   scripts/detect-flushes.sh [--db PATH] [--drop N] [--tol-seconds S]
#
#   --db PATH         path to claudemeter.db (default: auto-detect macOS/Linux)
#   --drop N          min utilization drop (percentage points) to count, default 10
#   --tol-seconds S   max resets_at movement still treated as "did not advance",
#                     default 3600 (1h). A scheduled reset moves it by days, so this
#                     cleanly separates a flush from a real reset.

set -euo pipefail

DROP=10
TOL=3600
DB=""

while [ $# -gt 0 ]; do
  case "$1" in
    --db)          DB="$2"; shift 2 ;;
    --drop)        DROP="$2"; shift 2 ;;
    --tol-seconds) TOL="$2"; shift 2 ;;
    -h|--help)     grep '^#' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

if [ -z "$DB" ]; then
  for cand in \
    "$HOME/Library/Application Support/ClaudeMeter/claudemeter.db" \
    "$HOME/.local/share/ClaudeMeter/claudemeter.db" \
    "$HOME/.config/ClaudeMeter/claudemeter.db"; do
    if [ -f "$cand" ]; then DB="$cand"; break; fi
  done
fi

if [ -z "$DB" ] || [ ! -f "$DB" ]; then
  echo "claudemeter.db not found. Pass one with --db PATH." >&2
  exit 1
fi

command -v sqlite3 >/dev/null 2>&1 || { echo "sqlite3 not on PATH." >&2; exit 1; }

echo "db: $DB"
echo "rule: utilization drop >= ${DROP} pts; resets_at movement <= ${TOL}s means anchor did NOT advance"
echo

# Compare each seven_day reading to the previous one. julianday() parses the ISO
# resets_at (fractional seconds + offset) and gives day-fractions; *86400 = seconds.
sqlite3 -header -column "$DB" "
WITH ordered AS (
  SELECT
    timestamp,
    utilization,
    resets_at,
    LAG(utilization) OVER (ORDER BY timestamp) AS prev_util,
    LAG(resets_at)   OVER (ORDER BY timestamp) AS prev_reset
  FROM usage_history
  WHERE provider = 'claude' AND metric = 'seven_day'
)
SELECT
  timestamp                                                         AS at_utc,
  CAST(prev_util AS INT) || '% -> ' || CAST(utilization AS INT) || '%' AS util_drop,
  CASE
    WHEN abs((julianday(resets_at) - julianday(prev_reset)) * 86400) <= ${TOL}
      THEN 'FREE FLUSH (anchor held)'
    ELSE 'scheduled reset (anchor +'
         || CAST(round((julianday(resets_at) - julianday(prev_reset))) AS INT) || 'd)'
  END                                                               AS kind,
  prev_reset                                                        AS reset_before,
  resets_at                                                         AS reset_after
FROM ordered
WHERE prev_util IS NOT NULL
  AND (prev_util - utilization) >= ${DROP}
ORDER BY timestamp DESC;
"
