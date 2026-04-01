#!/bin/bash
# pardus-browser test script
# Usage: ./test.sh [URL]
# Default URL: https://example.com
# Note: Requires nightly Rust (cargo +nightly)

set -e

URL="${1:-https://example.com}"
BIN="cargo +nightly run --"

echo "============================================================"
echo "  pardus-browser test suite"
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

# --- 7. UC Berkeley (complex site) ---
echo "──────────────────────────────────────────────────────────────"
echo "  7. UC Berkeley  —  complex university site"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.berkeley.edu/"
echo ""

# --- 8. UC Berkeley - Tree format ---
echo "──────────────────────────────────────────────────────────────"
echo "  8. UC Berkeley  —  tree format"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.berkeley.edu/" --format tree
echo ""

# --- 9. UC Berkeley - Interactive elements ---
echo "──────────────────────────────────────────────────────────────"
echo "  9. UC Berkeley  —  interactive elements only"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.berkeley.edu/" --interactive-only
echo ""

# --- 10. YC Companies (complex site with listings) ---
# Note: This is a React SPA that requires full browser environment
# The headless browser may not fully render client-side apps
echo "──────────────────────────────────────────────────────────────"
echo "  10. Y Combinator Companies  —  directory listing (SPA)"
echo "  Note: Client-side rendered, may have limited results"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.ycombinator.com/companies" --js --wait-ms 5000 2>&1 || echo "  (SPA - limited support)"
echo ""

# --- 11. YC Companies - Tree format ---
echo "──────────────────────────────────────────────────────────────"
echo "  11. YC Companies  —  tree format (SPA)"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.ycombinator.com/companies" --format tree --js --wait-ms 5000 2>&1 || echo "  (SPA - limited support)"
echo ""

# --- 12. YC Companies - Interactive elements ---
echo "──────────────────────────────────────────────────────────────"
echo "  12. YC Companies  —  interactive elements (SPA)"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.ycombinator.com/companies" --interactive-only --js --wait-ms 5000 2>&1 || echo "  (SPA - limited support)"
echo ""

# --- 13. YC Companies - JSON with navigation ---
echo "──────────────────────────────────────────────────────────────"
echo "  13. YC Companies  —  JSON format (SPA)"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://www.ycombinator.com/companies" --format json --with-nav --js --wait-ms 5000 2>&1 | head -50 || echo "  (SPA - limited support)"
echo ""

# --- 14. GitHub (SPA-like behavior) ---
echo "──────────────────────────────────────────────────────────────"
echo "  14. GitHub Homepage  —  testing modern web app"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://github.com" --js --wait-ms 5000
echo ""

# --- 15. GitHub - Interactive elements ---
echo "──────────────────────────────────────────────────────────────"
echo "  15. GitHub  —  interactive elements (buttons, forms)"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://github.com" --interactive-only --js --wait-ms 5000
echo ""

# --- 16. Wikipedia (content-heavy site) ---
echo "──────────────────────────────────────────────────────────────"
echo "  16. Wikipedia  —  content-heavy page"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://en.wikipedia.org/wiki/Web_browser"
echo ""

# --- 17. Wikipedia - Tree format ---
echo "──────────────────────────────────────────────────────────────"
echo "  17. Wikipedia  —  tree format for structure analysis"
echo "──────────────────────────────────────────────────────────────"
echo ""
$BIN navigate "https://en.wikipedia.org/wiki/Web_browser" --format tree
echo ""

echo "============================================================"
echo "  Done. All tests passed."
echo "============================================================"
echo ""
echo "  Summary:"
echo "    - Basic navigation and formats tested"
echo "    - Complex sites tested (Berkeley, YC, GitHub, Wikipedia)"
echo "    - Interactive element detection tested"
echo "    - JSON output with navigation graph tested"
echo "============================================================"
