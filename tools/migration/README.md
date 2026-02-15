# Duckling → Rustling Migration Toolchain

This directory contains the automated toolchain for migrating Duckling's 48 languages and 14 dimensions to Rustling.

## Overview

**Migration Strategy**: Semi-automated (70%+ automation)

```
Haskell Source → JSON Intermediate → Rust Code Generation → Corpus Validation
      ↓                ↓                    ↓                      ↓
  extract_rules    rule_schema.json      codegen.rs         corpus_converter.py
```

## Tools

### 1. `rule_schema.json`
**Purpose**: JSON Schema defining the intermediate representation of Duckling rules

**Capabilities**:
- Represents 95%+ of Haskell rule structures
- Supports 4 pattern types: dictionary, regex, composite, predicate
- Handles all 14 dimension types
- Validates extracted rules

**Status**: ✅ Complete

---

### 2. `extract_rules.hs` (TODO)
**Purpose**: Haskell AST parser to extract rules from Duckling source code

**Input**: `~/Project/duckling/Duckling/**/*.hs`
**Output**: JSON files per dimension/locale

**Features**:
- Parse Haskell AST using `haskell-src-exts`
- Extract HashMap dictionaries (e.g., `ruleNumeralMap`)
- Convert regex patterns to standard format
- Identify composite rules (`ruleIntersect`, `sequence`)
- Handle predicates (`isNotLatent`, `isInteger`)

**Example**:
```bash
cd ~/Project/duckling
./tools/migration/extract_rules Duckling/Numeral/ZH/Rules.hs > extracted/numeral_zh.json
```

**Dependencies**:
```bash
cabal install haskell-src-exts aeson text unordered-containers
```

---

### 3. `codegen.rs` (TODO)
**Purpose**: Rust code generator from JSON rule definitions

**Input**: JSON files from `extract_rules.hs`
**Output**: Rust source files in `languages/<locale>/<dimension>.rs`

**Features**:
- Template-based code generation (using `tera` or `handlebars`)
- Generates `RuleSetBuilder` code
- Creates localized string tables
- Auto-updates `mod.rs` files
- Maintains directory structure

**Example**:
```bash
cargo run --bin codegen -- \
  --input extracted/numeral/*.json \
  --output languages/ \
  --dimension numeral
```

**Dependencies**:
```toml
tera = "1.19"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

### 4. `corpus_converter.py` (TODO)
**Purpose**: Convert Duckling Corpus YAML to Rust test cases

**Input**: `~/Project/duckling/Duckling/**/Corpus.yml`
**Output**: Rust test files in `languages/<locale>/*_tests.rs`

**Features**:
- Parse YAML test definitions
- Generate `#[test]` functions
- Create Criterion benchmarks
- Organize by dimension and locale

**Example**:
```bash
python tools/migration/corpus_converter.py \
  --dimension Numeral \
  --output languages/tests/
```

**Dependencies**:
```bash
pip install pyyaml jinja2
```

---

## Workflow

### Phase 0: Setup
1. Design JSON schema ✅
2. Implement `extract_rules.hs`
3. Implement `codegen.rs`
4. Implement `corpus_converter.py`

### Phase 1: Pilot (Numeral)
```bash
# Extract rules
cd ~/Project/duckling
find Duckling/Numeral -name "*.hs" | xargs tools/migration/extract_rules

# Generate Rust code
cd ~/Project/duckling-rust
cargo run --bin codegen -- --input extracted/numeral/*.json --output languages/

# Convert tests
python tools/migration/corpus_converter.py --dimension Numeral

# Validate
cargo test --all -- numeral_corpus
cargo bench --bench numeral_bench
```

### Phase 2-4: Scale to All Dimensions
Repeat for: Time, Duration, Distance, Volume, Temperature, AmountOfMoney, PhoneNumber, Email, URL, Ordinal, Quantity, CreditCardNumber, TimeGrain

### Phase 5: Integration
- Dynamic locale loading
- Memory optimization
- Performance tuning
- Final validation

---

## Automation Estimates

| Rule Type | Proportion | Automation | Method |
|-----------|-----------|------------|--------|
| HashMap dictionaries | 40% | 95% | Direct JSON extraction |
| Regex patterns | 30% | 80% | Pattern conversion + validation |
| Composite rules | 20% | 60% | Template generation + checks |
| Complex logic | 10% | 20% | Manual porting + tests |
| **Total** | 100% | **70-75%** | Mixed strategy |

---

## Directory Structure (After Migration)

```
duckling-rust/
├── tools/
│   └── migration/
│       ├── README.md (this file)
│       ├── rule_schema.json
│       ├── extract_rules.hs
│       ├── codegen.rs
│       └── corpus_converter.py
├── templates/
│   ├── dictionary_rule.rs.tera
│   ├── regex_rule.rs.tera
│   ├── composite_rule.rs.tera
│   └── time_composite.rs.tera
├── extracted/ (generated)
│   ├── numeral/
│   │   ├── en.json
│   │   ├── zh.json
│   │   └── ...
│   ├── time/
│   └── ...
└── languages/ (generated)
    ├── en/
    │   ├── mod.rs
    │   ├── numeral.rs
    │   ├── time.rs
    │   └── ...
    ├── zh/
    └── ...
```

---

## Success Criteria

### Functionality
- ✅ Support all 48 languages
- ✅ Support all 14 dimensions
- ✅ 98%+ Duckling Corpus tests pass

### Performance
- ✅ Parse latency < 1ms (simple inputs)
- ✅ Throughput ≥ Duckling (ideally 2-3x faster)
- ✅ Memory < 100MB (all languages loaded)

### Code Quality
- ✅ `cargo clippy` passes with no warnings
- ✅ Test coverage > 85%
- ✅ Documentation complete

---

## Maintenance

### Adding New Languages
1. Extract rules: `extract_rules Duckling/Numeral/XX/Rules.hs`
2. Generate code: `codegen --input extracted/numeral/xx.json`
3. Convert tests: `corpus_converter.py --locale xx`
4. Validate: `cargo test xx_corpus`

**Time**: < 1 hour per language (fully automated)

### Syncing Duckling Updates
Monitor https://github.com/facebook/duckling/releases

When new rules are added:
1. Re-run extraction on updated files
2. Regenerate affected Rust code
3. Run regression tests

---

## Known Limitations

1. **Complex Haskell Logic**: ~10% of rules use advanced Haskell features (monads, complex pattern matching) that cannot be automatically translated. These require manual porting.

2. **Predicate Functions**: Some predicates like `isNotLatent`, `isOkForAnyTime` need manual implementation in Rust.

3. **Time Zone Handling**: Duckling's time zone logic is complex. May need to leverage `chrono-tz` crate.

4. **Context-Dependent Rules**: Some rules depend on runtime context (e.g., reference time). These need special handling in Rust.

---

## Contributors

See `docs/MIGRATION_GUIDE.md` for detailed instructions on contributing to the migration effort.
