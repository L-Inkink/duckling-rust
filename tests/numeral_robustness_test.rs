use rustling::values::Value;
use rustling::languages::en::numeral::rules;
use rustling_core::{RuleSetBuilder, BoundariesChecker};

#[test]
fn test_spaced_numbers() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "8 : 00" - should recognize "8" and "00" separately
    let results = ruleset.apply_all("8 : 00").unwrap();
    eprintln!("\n=== Testing '8 : 00' ===");
    eprintln!("Total matches: {}", results.len());
    for (i, r) in results.iter().enumerate() {
        eprintln!("  Match {}: {:?}", i, r.value);
    }

    let has_8 = results.iter().any(|r| r.value == Value::Integer(8));
    let has_0 = results.iter().any(|r| r.value == Value::Integer(0));
    assert!(has_8, "Should recognize '8'");
    assert!(has_0, "Should recognize '0' or '00'");

    // Test "1 234" (space instead of comma)
    let results = ruleset.apply_all("1 234").unwrap();
    eprintln!("\n=== Testing '1 234' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_1 = results.iter().any(|r| r.value == Value::Integer(1));
    let has_234 = results.iter().any(|r| r.value == Value::Integer(234));
    assert!(has_1 && has_234, "Should recognize '1' and '234' separately");

    // Test "twenty - three" (with spaces around hyphen)
    let results = ruleset.apply_all("twenty - three").unwrap();
    eprintln!("\n=== Testing 'twenty - three' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    // Should recognize "twenty" and "three" separately, may also have composite
    let has_20 = results.iter().any(|r| r.value == Value::Integer(20));
    let has_3 = results.iter().any(|r| r.value == Value::Integer(3));
    assert!(has_20 && has_3, "Should recognize 'twenty' and 'three'");
}

#[test]
fn test_mixed_case() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "TWENTY THREE" (uppercase)
    let results = ruleset.apply_all("TWENTY THREE").unwrap();
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should handle uppercase 'TWENTY THREE'");

    // Test "Twenty Three" (title case)
    let results = ruleset.apply_all("Twenty Three").unwrap();
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should handle title case 'Twenty Three'");

    // Test "tWeNtY tHrEe" (random case)
    let results = ruleset.apply_all("tWeNtY tHrEe").unwrap();
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should handle mixed case 'tWeNtY tHrEe'");
}

#[test]
fn test_extra_whitespace() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "  twenty   three  " (extra spaces)
    let results = ruleset.apply_all("  twenty   three  ").unwrap();
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should handle extra whitespace");

    // Test "five\thundred" (tab character)
    let results = ruleset.apply_all("five\thundred").unwrap();
    eprintln!("\n=== Testing 'five\\thundred' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_500 = results.iter().any(|r| r.value == Value::Integer(500));
    assert!(has_500, "Should handle tab separator");
}

#[test]
fn test_punctuation_around_numbers() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "(23)" (parentheses)
    let results = ruleset.apply_all("(23)").unwrap();
    eprintln!("\n=== Testing '(23)' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should recognize '23' in parentheses");

    // Test "[100]" (brackets)
    let results = ruleset.apply_all("[100]").unwrap();
    let has_100 = results.iter().any(|r| r.value == Value::Integer(100));
    assert!(has_100, "Should recognize '100' in brackets");

    // Test "23." (period at end)
    let results = ruleset.apply_all("23.").unwrap();
    eprintln!("\n=== Testing '23.' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should recognize '23' before period");

    // Test "23," (comma at end)
    let results = ruleset.apply_all("23,").unwrap();
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should recognize '23' before comma");
}

#[test]
fn test_decimal_variations() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test ".5" (no leading zero)
    let results = ruleset.apply_all(".5").unwrap();
    let has_05 = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 0.5).abs() < 0.001)
    });
    assert!(has_05, "Should recognize '.5' as 0.5");

    // Test "0.5" (with leading zero)
    let results = ruleset.apply_all("0.5").unwrap();
    let has_05 = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 0.5).abs() < 0.001)
    });
    assert!(has_05, "Should recognize '0.5'");

    // Test "5.0" (trailing zero)
    let results = ruleset.apply_all("5.0").unwrap();
    eprintln!("\n=== Testing '5.0' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_50 = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 5.0).abs() < 0.001)
    });
    assert!(has_50, "Should recognize '5.0'");
}

#[test]
fn test_large_numbers_with_commas() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "1,234,567" (multiple commas)
    let results = ruleset.apply_all("1,234,567").unwrap();
    let has_million = results.iter().any(|r| r.value == Value::Integer(1234567));
    assert!(has_million, "Should recognize '1,234,567'");

    // Test "1,234,567.89" (decimal with commas)
    let results = ruleset.apply_all("1,234,567.89").unwrap();
    eprintln!("\n=== Testing '1,234,567.89' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_million_decimal = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 1234567.89).abs() < 0.01)
    });
    assert!(has_million_decimal, "Should recognize '1,234,567.89'");
}

#[test]
fn test_edge_cases() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "0" (zero)
    let results = ruleset.apply_all("0").unwrap();
    let has_0 = results.iter().any(|r| r.value == Value::Integer(0));
    assert!(has_0, "Should recognize '0'");

    // Test "00" (double zero)
    let results = ruleset.apply_all("00").unwrap();
    eprintln!("\n=== Testing '00' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_0 = results.iter().any(|r| r.value == Value::Integer(0));
    assert!(has_0, "Should recognize '00' as 0");

    // Test "000" (triple zero)
    let results = ruleset.apply_all("000").unwrap();
    let has_0 = results.iter().any(|r| r.value == Value::Integer(0));
    assert!(has_0, "Should recognize '000' as 0");

    // Test very large number
    let results = ruleset.apply_all("999999999").unwrap();
    let has_large = results.iter().any(|r| r.value == Value::Integer(999999999));
    assert!(has_large, "Should recognize large number '999999999'");
}

#[test]
fn test_word_boundaries() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "twenty3" (word + digit, should NOT combine)
    let results = ruleset.apply_all("twenty3").unwrap();
    eprintln!("\n=== Testing 'twenty3' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    // Should recognize them separately due to boundary checker

    // Test "3twenty" (digit + word, should NOT combine)
    let results = ruleset.apply_all("3twenty").unwrap();
    eprintln!("\n=== Testing '3twenty' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
}

#[test]
fn test_negative_variations() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "- 5" (space after minus)
    let results = ruleset.apply_all("- 5").unwrap();
    eprintln!("\n=== Testing '- 5' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_neg5 = results.iter().any(|r| r.value == Value::Integer(-5));
    assert!(has_neg5, "Should recognize '- 5' as -5");

    // Test "NEGATIVE TEN" (uppercase)
    let results = ruleset.apply_all("NEGATIVE TEN").unwrap();
    let has_neg10 = results.iter().any(|r| r.value == Value::Integer(-10));
    assert!(has_neg10, "Should handle uppercase 'NEGATIVE TEN'");

    // Test "minus  5" (double space)
    let results = ruleset.apply_all("minus  5").unwrap();
    let has_neg5 = results.iter().any(|r| r.value == Value::Integer(-5));
    assert!(has_neg5, "Should handle extra space in 'minus  5'");
}

#[test]
fn test_complex_expressions() {
    let b = RuleSetBuilder::new(
        BoundariesChecker::detailed(),
        BoundariesChecker::separated_alphanumeric_word(),
    );
    rules(&b);
    let ruleset = b.build();

    // Test "I have 23 apples" (number in sentence)
    let results = ruleset.apply_all("I have 23 apples").unwrap();
    eprintln!("\n=== Testing 'I have 23 apples' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_23 = results.iter().any(|r| r.value == Value::Integer(23));
    assert!(has_23, "Should extract '23' from sentence");

    // Test "The price is $1,234.56" (number with currency)
    let results = ruleset.apply_all("The price is $1,234.56").unwrap();
    eprintln!("\n=== Testing 'The price is $1,234.56' ===");
    eprintln!("Total matches: {}", results.len());
    for r in &results {
        eprintln!("  {:?}", r.value);
    }
    let has_price = results.iter().any(|r| {
        matches!(r.value, Value::Float(f) if (f - 1234.56).abs() < 0.01)
    });
    assert!(has_price, "Should extract '1,234.56' from price");

    // Test "Year 2024" (number after word)
    let results = ruleset.apply_all("Year 2024").unwrap();
    let has_year = results.iter().any(|r| r.value == Value::Integer(2024));
    assert!(has_year, "Should extract '2024' from 'Year 2024'");
}
