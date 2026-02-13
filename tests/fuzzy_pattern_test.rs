use rustling::fuzzy::PatternNormalizer;

#[test]
fn test_normalize_chinese_tomorrow_morning() {
    let normalizer = PatternNormalizer::new();

    // Abbreviation -> full form
    assert_eq!(normalizer.normalize("明早"), "明天早上");
    // Already normalized -> same
    assert_eq!(normalizer.normalize("明天早上"), "明天早上");
}

#[test]
fn test_normalize_chinese_tomorrow_evening() {
    let normalizer = PatternNormalizer::new();

    assert_eq!(normalizer.normalize("明晚"), "明天晚上");
    assert_eq!(normalizer.normalize("明天晚上"), "明天晚上");
}

#[test]
fn test_normalize_chinese_today_morning() {
    let normalizer = PatternNormalizer::new();

    assert_eq!(normalizer.normalize("今早"), "今天早上");
    assert_eq!(normalizer.normalize("今天早上"), "今天早上");
}

#[test]
fn test_normalize_no_change() {
    let normalizer = PatternNormalizer::new();

    // No pattern match, should return original input
    assert_eq!(normalizer.normalize("hello world"), "hello world");
    assert_eq!(normalizer.normalize("12345"), "12345");
}
