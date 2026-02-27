#!/bin/bash
# Performance comparison: Haskell Duckling vs Rustling

echo "=== Duckling Performance Comparison ==="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

# Use date command (macOS compatible)
get_time() {
    python3 -c "import time; print(int(time.time() * 1000000000))"
}

# Check services
echo "Checking services..."

if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} duckling-rust (8080) is running"
else
    echo -e "${RED}✗${NC} duckling-rust is NOT running"
    exit 1
fi

if curl -s -X POST http://localhost:8081/parse -d "text=test" > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} haskell-duckling (8081) is running"
else
    echo -e "${RED}✗${NC} haskell-duckling is NOT running"
    exit 1
fi

echo ""
echo "Running 10 iterations per query..."
echo ""

# Test queries - time related
QUERIES=(
    "tomorrow at 3pm"
    "in 5 minutes"
    "next monday at 9am"
    "3 days ago"
    "every monday at 6pm"
    "the third friday of next month"
    "tomorrow morning at 10"
    "two weeks from now"
)

for query in "${QUERIES[@]}"; do
    echo "Query: \"$query\""
    echo "---"

    # Rustling - 10 iterations
    rust_total=0
    for i in {1..10}; do
        start=$(get_time)
        rust_result=$(curl -s -X POST http://localhost:8080/parse \
            -H "Content-Type: application/json" \
            -d "{\"text\": \"$query\", \"locale\": \"en\"}" 2>/dev/null)
        end=$(get_time)
        rust_total=$((rust_total + (end - start)))
    done
    rust_avg=$((rust_total / 10 / 1000000))  # Convert ns to ms

    # Haskell Duckling - 10 iterations
    hs_total=0
    for i in {1..10}; do
        start=$(get_time)
        hs_result=$(curl -s -X POST http://localhost:8081/parse \
            -d "text=$query&locale=en_GB" 2>/dev/null)
        end=$(get_time)
        hs_total=$((hs_total + (end - start)))
    done
    hs_avg=$((hs_total / 10 / 1000000))  # Convert ns to ms

    # Calculate ratio
    if [ "$hs_avg" -gt 0 ]; then
        ratio=$(echo "scale=2; $rust_avg / $hs_avg" | bc 2>/dev/null || echo "N/A")
    else
        ratio="N/A"
    fi

    printf "Rustling:    %4dms (avg of 10)\n" "$rust_avg"
    printf "Haskell:     %4dms (avg of 10)\n" "$hs_avg"
    echo "Ratio:        ${ratio}x"
    echo ""
done

echo "=== Done ==="
