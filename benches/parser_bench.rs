use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustling::*;

// Define test types (same as in src/lib.rs tests)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MyPayload;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Int(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MyValueKind {
    UI,
}

impl StashIndexable for Int {
    type Index = MyValueKind;
    fn index(&self) -> Self::Index {
        MyValueKind::UI
    }
}

impl AttemptFrom<Int> for Int {
    fn attempt_from(v: Int) -> Option<Int> {
        Some(v)
    }
}

impl NodePayload for Int {
    type Payload = MyPayload;
    fn extract_payload(&self) -> Option<Self::Payload> {
        Some(MyPayload)
    }
}

impl InnerStashIndexable for Int {
    type Index = MyValueKind;
    fn index() -> Self::Index {
        MyValueKind::UI
    }
}

/// Create a simple rule set for benchmarking
fn create_benchmark_rules() -> RuleSet<Int> {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );

    // Terminal rule: match digits
    b.rule_1(
        "integer (numeric)",
        b.reg(r#"(\d{1,18})"#).unwrap(),
        |text_match| Ok(Int(text_match.group(0).parse::<usize>()?)),
    );

    // Terminal rule: match "thousand(s)"
    b.rule_1("integer (thousand)", b.reg("thousands?").unwrap(), |_| {
        Ok(Int(1000))
    });

    // Terminal rule: match "hundred(s)"
    b.rule_1("integer (hundred)", b.reg("hundreds?").unwrap(), |_| {
        Ok(Int(100))
    });

    // Composition rule: number + thousands
    b.rule_2(
        "number thousands",
        dim!(Int, vec![Box::new(|a: &Int| a.0 > 1 && a.0 < 99)]),
        dim!(Int, vec![Box::new(|a: &Int| a.0 == 1000)]),
        |a, _| Ok(Int(a.value().0 * 1000)),
    );

    // Composition rule: number + hundreds
    b.rule_2(
        "number hundreds",
        dim!(Int, vec![Box::new(|a: &Int| a.0 > 1 && a.0 < 99)]),
        dim!(Int, vec![Box::new(|a: &Int| a.0 == 100)]),
        |a, _| Ok(Int(a.value().0 * 100)),
    );

    b.build()
}

/// Benchmark: Parse a simple single number
fn benchmark_parse_simple_number(c: &mut Criterion) {
    let rule_set = create_benchmark_rules();

    c.bench_function("parse simple number", |b| {
        b.iter(|| {
            let result = rule_set.apply_all(black_box("23"));
            result.unwrap()
        })
    });
}

/// Benchmark: Parse a complex expression with composition
fn benchmark_parse_complex_number(c: &mut Criterion) {
    let rule_set = create_benchmark_rules();

    c.bench_function("parse complex number", |b| {
        b.iter(|| {
            let result = rule_set.apply_all(black_box("12 thousands"));
            result.unwrap()
        })
    });
}

/// Benchmark: Parse multiple numbers in text
fn benchmark_parse_multiple_numbers(c: &mut Criterion) {
    let rule_set = create_benchmark_rules();

    c.bench_function("parse multiple numbers", |b| {
        b.iter(|| {
            let result = rule_set.apply_all(black_box("I have 12 thousands and 45 hundreds"));
            result.unwrap()
        })
    });
}

/// Benchmark: Parse text with no matches
fn benchmark_parse_no_matches(c: &mut Criterion) {
    let rule_set = create_benchmark_rules();

    c.bench_function("parse no matches", |b| {
        b.iter(|| {
            let result = rule_set.apply_all(black_box("no numbers here"));
            result.unwrap()
        })
    });
}

/// Benchmark: Parse long text with scattered numbers
fn benchmark_parse_long_text(c: &mut Criterion) {
    let rule_set = create_benchmark_rules();
    let long_text = "The company reported 42 thousands in revenue, with 15 hundreds \
                     in expenses and a profit of 27 thousands. This represents \
                     a 12 percent increase over last year's 25 thousands.";

    c.bench_function("parse long text", |b| {
        b.iter(|| {
            let result = rule_set.apply_all(black_box(long_text));
            result.unwrap()
        })
    });
}

/// Benchmark: Rule set creation overhead
fn benchmark_rule_set_creation(c: &mut Criterion) {
    c.bench_function("create rule set", |b| {
        b.iter(|| black_box(create_benchmark_rules()))
    });
}

// Benchmark: Parse using Value types (Phase 1)
fn benchmark_parse_integer_value(c: &mut Criterion) {
    use rustling::rules::integer;
    use rustling::values::Value;

    let mut b = RuleSetBuilder::<Value>::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    integer::rules(&mut b);
    let rule_set = b.build();

    c.bench_function("parse integer value", |b| {
        b.iter(|| {
            let result = rule_set.apply_all(black_box("12345"));
            result.unwrap()
        })
    });
}

// Benchmark: Levenshtein distance calculation
fn benchmark_levenshtein(c: &mut Criterion) {
    use rustling::fuzzy::LevenshteinMatcher;

    let matcher = LevenshteinMatcher::new(0.8);
    let s1 = "tomorrow";
    let s2 = "tomorow";

    c.bench_function("levenshtein distance", |b| {
        b.iter(|| {
            matcher.distance(black_box(s1), black_box(s2))
        })
    });
}

// Benchmark: Pattern normalization
fn benchmark_pattern_normalizer(c: &mut Criterion) {
    use rustling::fuzzy::PatternNormalizer;

    let normalizer = PatternNormalizer::new();
    let input = "明早";

    c.bench_function("pattern normalize", |b| {
        b.iter(|| {
            normalizer.normalize(black_box(input))
        })
    });
}

// Benchmark: Unified Parse API
fn benchmark_parse_api(c: &mut Criterion) {
    use rustling::parse::Parser;

    let parser = Parser::new();

    c.bench_function("parse api integer", |b| {
        b.iter(|| {
            parser.parse(black_box("42"), black_box(Some("en")))
        })
    });
}

// Benchmark: Unified Parse API with duration
fn benchmark_parse_api_duration(c: &mut Criterion) {
    use rustling::parse::Parser;

    let parser = Parser::new();

    c.bench_function("parse api duration", |b| {
        b.iter(|| {
            parser.parse(black_box("5 minutes"), black_box(Some("en")))
        })
    });
}

// Benchmark: Batch parse
fn benchmark_parse_api_batch(c: &mut Criterion) {
    use rustling::parse::Parser;

    let parser = Parser::new();
    let texts = vec![
        "42".to_string(),
        "5 minutes".to_string(),
        "tomorrow".to_string(),
        "2024-01-15".to_string(),
    ];

    c.bench_function("parse api batch 4", |b| {
        b.iter(|| {
            parser.parse_batch(black_box(&texts), black_box(Some("en")))
        })
    });
}

criterion_group!(
    benches,
    benchmark_parse_simple_number,
    benchmark_parse_complex_number,
    benchmark_parse_multiple_numbers,
    benchmark_parse_no_matches,
    benchmark_parse_long_text,
    benchmark_rule_set_creation,
    benchmark_parse_integer_value,
    benchmark_levenshtein,
    benchmark_pattern_normalizer,
    benchmark_parse_api,
    benchmark_parse_api_duration,
    benchmark_parse_api_batch,
);

criterion_main!(benches);
