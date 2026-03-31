#!/bin/bash
# pardus-browser test script
# Usage: ./test.sh [URL]
# Default URL: https://example.com

set -e

URL="${1:-https://example.com}"
BIN="cargo run --"

echo "============================================================"
echo "  pardus-browser test — $URL"
echo "============================================================"
echo ""

# --- 1. Default format (md) ---
echo "──────────────────────────────────────────────────────────────"
echo "  1. Default format (md)  —  ./pardus-browser navigate $URL"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "$URL"
echo ""

# --- 2. Tree format ---
echo "──────────────────────────────────────────────────────────────"
echo "  2. Tree format  —  --format tree"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "$URL" --format tree
echo ""

# --- 3. JSON format with navigation graph ---
echo "──────────────────────────────────────────────────────────────"
echo "  3. JSON + navigation graph  —  --format json --with-nav"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "$URL" --format json --with-nav
echo ""

# --- 4. Interactive-only (md) ---
echo "──────────────────────────────────────────────────────────────"
echo "  4. Interactive elements only  —  --interactive-only"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "$URL" --interactive-only
echo ""

# --- 5. Google.com ---
echo "──────────────────────────────────────────────────────────────"
echo "  5. Google.com  —  default (md) format"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.google.com"
echo ""

# --- 6. Hacker News ---
echo "──────────────────────────────────────────────────────────────"
echo "  6. Hacker News  —  default (md) format"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://news.ycombinator.com"
echo ""

echo "============================================================"
echo "  Done. All tests passed."
echo "============================================================"
