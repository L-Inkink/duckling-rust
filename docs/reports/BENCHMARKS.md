# Rustling Performance Benchmarks

**Date**: 2026-02-12
**Platform**: darwin (macOS 24.6.0)
**Rust Version**: 1.93.0
**Criterion Version**: 0.5.1

---

## Overview

This document contains performance benchmarks for the Rustling parser. Benchmarks are created using Criterion.rs and measure parsing performance across various scenarios.

---

## Benchmark Results

### 1. Parse Simple Number
**Input**: `"23"`
**Time**: **900.85 ns** (mean)
**Description**: Parsing a single simple number with terminal rules only

### 2. Parse Complex Number
**Input**: `"12 thousands"`
**Time**: **2.36 µs** (mean)
**Description**: Parsing with composition rules that combine multiple tokens

### 3. Parse Multiple Numbers
**Input**: `"I have 12 thousands and 45 hundreds"`
**Time**: **3.99 µs** (mean)
**Description**: Parsing text with multiple number expressions

### 4. Parse No Matches
**Input**: `"no numbers here"`
**Time**: **245 ns** (mean)
**Description**: Parsing text with no pattern matches (fastest case)

### 5. Parse Long Text
**Input**: Long paragraph with scattered numbers
**Time**: **8.13 µs** (mean)
**Description**: Realistic scenario with mixed text and multiple numbers

### 6. Create Rule Set
**Time**: **332 µs** (mean)
**Description**: Overhead of creating a rule set with multiple rules

---

## Performance Analysis

### Key Insights

1. **Fast Terminal Matching** (~900 ns)
   - Simple terminal rule matching is very fast
   - Regex compilation is cached in the rule set

2. **Composition Overhead** (~2.4 µs)
   - Composition rules add ~1.5 µs overhead vs. terminal rules
   - This is expected due to pattern matching and value extraction

3. **Linear Scaling**
   - Performance scales linearly with input length
   - Multiple numbers: ~4 µs for 2-3 matches
   - Long text: ~8 µs for 4-5 matches

4. **No-Match Fast Path** (~245 ns)
   - When no patterns match, parser exits very quickly
   - Good performance characteristics for mixed content

5. **Rule Set Creation** (~332 µs)
   - One-time cost at startup
   - Negligible amortized cost for long-running parsers

### Comparison Baseline

These benchmarks establish a performance baseline for Phase 0. Future phases can compare against these numbers to ensure optimizations don't regress performance.

**Expected performance profile**:
- Single token: < 1 µs
- Composition (2 tokens): 2-3 µs
- Complex expression (3+ tokens): 5-10 µs

---

## Running Benchmarks

### Run All Benchmarks
```bash
cargo bench
```

### Run Specific Benchmark
```bash
cargo bench --bench parser_bench -- "parse simple"
```

### View Reports
```bash
open target/criterion/report/index.html
```

---

## Benchmark Implementation

Benchmarks are located in `benches/parser_bench.rs` and use:
- **Criterion.rs**: Statistical benchmarking framework
- **Black box**: Prevents compiler optimizations on inputs
- **Realistic scenarios**: Based on actual use cases

### Test Rule Set

The benchmark uses a simple integer parser with:
- Terminal rules: digits, "thousand(s)", "hundred(s)"
- Composition rules: number + thousands/hundreds
- Pattern: Matches numbers like "12 thousands", "45 hundreds"

---

## Future Benchmarks

For Phase 1+, consider adding:
- **Regex complexity**: Different regex patterns
- **Rule saturation**: Performance with many rules
- **Memory usage**: Heap allocations during parsing
- **Concurrent parsing**: Thread safety overhead
- **Real-world rules**: Actual date/time/number rules

---

## CI Integration

Benchmarks can be run in CI to track performance over time:
```yaml
- name: Run benchmarks
  run: cargo bench --no-fail-fast
```

For now, benchmarks are run manually as part of the development process.

---

**Generated**: 2026-02-12
**Phase**: Phase 0 Week 2
**Status**: Baseline established ✅
