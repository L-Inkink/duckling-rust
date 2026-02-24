#!/bin/bash
# Performance comparison: Haskell Duckling vs Rustling

set -e

echo "=== Duckling Performance Comparison ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if containers are running
echo "Checking services..."
if docker ps | grep -q duckling-rust; then
    echo -e "${GREEN}✓${NC} duckling-rust is running"
    RUSTLING_OK=true
else
    echo -e "${RED}✗${NC} duckling-rust is NOT running"
    RUSTLING_OK=false
fi

if docker ps | grep -q haskell-duckling; then
    echo -e "${GREEN}✓${NC} haskell-duckling is running"
    HASKELL_OK=true
else
    echo -e "${RED}✗${HASKELL_OK=false}haskell-duckling is NOT running"
    HASKELL_OK=false
fi

echo ""

if [ "$RUSTLING_OK" = false ] || [ "$HASKELL_OK" = false ]; then
    echo "Please start both containers first:"
    echo ""
    echo "# Start duckling-rust (port 8080):"
    echo "docker run -d --name duckling-rust -p 8080:8080 duckling-rust:latest"
    echo ""
    echo "# Start haskell-duckling (port 8081):"
    echo "docker run -d --name haskell-duckling -p 8081:8080 ghcr.io/facebookincubator/duckling:latest"
    echo ""
    exit 1
fi

# Test queries
QUERIES=(
    'tomorrow at 3pm'
    'in 5 minutes'
    'next monday at 9am'
    '3 days ago'
    'every monday at 6pm'
    'the third friday of next month'
    'tomorrow morning at 10'
    'two weeks from now'
)

echo "=== Running Performance Tests ==="
echo ""

for query in "${QUERIES[@]}"; do
    echo "Query: \"$query\""
    echo "---"

    # Rustling
    start_rust=$(date +%s%N)
    rust_result=$(curl -s -X POST http://localhost:8080/parse \
        -H "Content-Type: application/json" \
        -d "{\"text\": \"$query\", \"locale\": \"en\"}" || echo "ERROR")
    end_rust=$(date +%s%N)
    rust_time=$(( (end_rust - start_rust) / 1000000 )) # ms

    # Haskell Duckling
    start_hs=$(date +%s%N)
    hs_result=$(curl -s -X POST http://localhost:8081/parse \
        -H "Content-Type: application/json" \
        -d "{\"text\": \"$query\", \"latent\": true, \"locale\": \"en_US\"}" || echo "ERROR")
    end_hs=$(date +%s%N)
    hs_time=$(( (end_hs - start_hs) / 1000000 )) # ms

    # Calculate ratio
    if [ "$hs_time" -gt 0 ]; then
        ratio=$(echo "scale=2; $rust_time / $hs_time" | bc 2>/dev/null || echo "N/A")
    else
        ratio="N/A"
    fi

    echo "Rustling:    ${rust_time}ms"
    echo "Haskell:     ${hs_time}ms"
    echo "Ratio:       ${ratio}x"
    echo ""
done

echo "=== Summary ==="
echo "Run multiple times to get average results"
