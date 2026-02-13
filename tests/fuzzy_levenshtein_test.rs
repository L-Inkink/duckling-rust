use rustling::fuzzy::LevenshteinMatcher;

#[test]
fn test_levenshtein_distance_zero() {
    let matcher = LevenshteinMatcher::new(0.85);

    assert_eq!(matcher.distance("hello", "hello"), 0);
    assert_eq!(matcher.distance("world", "world"), 0);
}

#[test]
fn test_levenshtein_distance_one() {
    let matcher = LevenshteinMatcher::new(0.85);

    // Insert one character
    assert_eq!(matcher.distance("cat", "cats"), 1);

    // Delete one character
    assert_eq!(matcher.distance("cats", "cat"), 1);

    // Replace one character
    assert_eq!(matcher.distance("cat", "bat"), 1);
}

#[test]
fn test_levenshtein_similarity() {
    let matcher = LevenshteinMatcher::new(0.85);

    // Completely same
    assert_eq!(matcher.similarity("hello", "hello"), 1.0);

    // One character difference, 5 characters length
    // similarity = 1 - (1 / 5) = 0.8
    assert_eq!(matcher.similarity("hello", "hallo"), 0.8);
}

#[test]
fn test_correct_tomorrow_typo() {
    let matcher = LevenshteinMatcher::new(0.85);

    // tomorow → tomorrow (similarity = 0.875 > 0.85)
    let corrected = matcher.correct("tomorow", &["tomorrow", "today", "yesterday"]);
    assert_eq!(corrected, Some("tomorrow".to_string()));
}

#[test]
fn test_no_correction_needed() {
    let matcher = LevenshteinMatcher::new(0.85);

    // Already correct
    let corrected = matcher.correct("tomorrow", &["tomorrow", "today"]);
    assert_eq!(corrected, Some("tomorrow".to_string()));
}

#[test]
fn test_no_good_match() {
    let matcher = LevenshteinMatcher::new(0.85);

    // xyz has too low similarity with tomorrow
    let corrected = matcher.correct("xyz", &["tomorrow", "today"]);
    assert_eq!(corrected, None);
}
