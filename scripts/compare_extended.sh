#!/bin/bash
# Extended performance comparison: Haskell Duckling vs Rustling

echo "=== Extended Duckling Performance Comparison ==="
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
echo "Running 20 iterations per query..."
echo ""

# Extended test queries - more comprehensive
QUERIES=(
    # Simple time
    "tomorrow"
    "today"
    "yesterday"
    "now"
    "midnight"
    "noon"

    # Time with hour
    "3pm"
    "5am"
    "10 o'clock"
    "2:30"

    # Relative time
    "in 5 minutes"
    "in 2 hours"
    "in 3 days"
    "in 2 weeks"

    # Past time
    "3 days ago"
    "2 hours ago"
    "1 week ago"

    # Specific time
    "tomorrow at 3pm"
    "next monday at 9am"
    "next friday at 6pm"

    # Complex patterns
    "every monday"
    "every day at 5pm"
    "the third friday of next month"

    # Multi-language (should use default)
    "morning"
    "evening"
    "afternoon"
)

total_rust=0
total_hs=0
count=0

for query in "${QUERIES[@]}"; do
    echo "Query: \"$query\""
    echo "---"

    # Rustling - 20 iterations
    rust_total=0
    for i in {1..20}; do
        start=$(get_time)
        rust_result=$(curl -s -X POST http://localhost:8080/parse \
            -H "Content-Type: application/json" \
            -d "{\"text\": \"$query\", \"locale\": \"en\"}" 2>/dev/null)
        end=$(get_time)
        rust_total=$((rust_total + (end - start)))
    done
    rust_avg=$((rust_total / 20 / 1000000))  # Convert ns to ms

    # Haskell Duckling - 20 iterations
    hs_total=0
    for i in {1..20}; do
        start=$(get_time)
        hs_result=$(curl -s -X POST http://localhost:8081/parse \
            -d "text=$query&locale=en_GB" 2>/dev/null)
        end=$(get_time)
        hs_total=$((hs_total + (end - start)))
    done
    hs_avg=$((hs_total / 20 / 1000000))  # Convert ns to ms

    # Calculate ratio
    if [ "$hs_avg" -gt 0 ]; then
        ratio=$(echo "scale=2; $rust_avg / $hs_avg" | bc 2>/dev/null || echo "N/A")
    else
        ratio="N/A"
    fi

    # Track totals
    total_rust=$((total_rust + rust_avg))
    total_hs=$((total_hs + hs_avg))
    count=$((count + 1))

    printf "Rustling:    %4dms (avg of 20)\n" "$rust_avg"
    printf "Haskell:     %4dms (avg of 20)\n" "$hs_avg"
    echo "Ratio:        ${ratio}x"
    echo ""
done

# Summary
echo "=== Summary ==="
echo "Total queries: $count"
echo "Average Rustling: $((total_rust / count))ms"
echo "Average Haskell:  $((total_hs / count))ms"
echo "Overall Ratio:    $(echo "scale=2; $total_rust / $total_hs" | bc 2>/dev/null || echo "N/A")x"
echo ""
echo "=== Done ==="
