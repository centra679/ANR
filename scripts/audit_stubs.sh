#!/usr/bin/env bash
set -euo pipefail

echo "== stub markers =="
grep -rn "todo!\|unimplemented!\|placeholder\|PLACEHOLDER\|stub\|STUB\|FIXME\|XXX\|not yet implemented" src/ tests/ || true

echo "== fake asserts =="
grep -rn "assert!(true)\|assert_eq!(1, 1)" src/ tests/ || true

echo "== silenced warnings =="
grep -rn "#\[allow(" src/ tests/ || true

echo "== near-empty modules (< 30 bytes) =="
find src -name "*.rs" -size -30c || true

echo "== serde in neural/storage modules =="
grep -rn "use serde\|#\[derive(.*Serialize.*)\|#\[derive(.*Deserialize.*)\]" src/neural/ src/storage/ src/brain/ || true

echo "== unbounded Vec in hot paths =="
grep -rn "Vec::new()\|Vec::with_capacity" src/neural/ src/storage/ src/plugins/ src/perception/ src/hardware/ || true

echo "== Box<Cell> in hot path =="
grep -rn "Box<Cell>\|Box<Column>\|Box<Block>" src/ || true

echo "== empty Ok(()) returns in production =="
grep -rn "Ok(\s*(\s*)\s*)" src/ || true

echo "== empty test bodies =="
grep -rn "#\[test\]" tests/ | head -200 || true

echo "== forbidden persistent artifact refs in code =="
grep -rn "\.cx\b\|\.cm\b\|\.hs\b" src/ tests/ || true

echo "== forbidden dependency refs in code =="
grep -rn "network\|cloud\|llm\|transformer\|gpu" src/ tests/ || true

echo "== summary =="
TOTAL_STUBS=$(grep -rn "todo!\|unimplemented!\|placeholder\|PLACEHOLDER\|stub\|STUB\|FIXME\|XXX\|not yet implemented" src/ tests/ | wc -l || echo 0)
TOTAL_FAKE_TESTS=$(grep -rn "#\[test\]" tests/ | wc -l || echo 0)
TOTAL_EMPTY_MODS=$(find src -name "*.rs" -size -30c | wc -l || echo 0)
TOTAL_SERDE=$(grep -rn "use serde\|#\[derive(.*Serialize.*)\|#\[derive(.*Deserialize.*)\]" src/neural/ src/storage/ src/brain/ | wc -l || echo 0)
echo "stub markers: $TOTAL_STUBS"
echo "fake tests: $TOTAL_FAKE_TESTS"
echo "empty modules: $TOTAL_EMPTY_MODS"
echo "serde in neural/storage: $TOTAL_SERDE"
