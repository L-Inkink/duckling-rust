use std::collections::HashMap;
use std::sync::Arc;

use rustling_core::time::TimeContext;
use rustling_core::{BoundariesChecker, RuleSet, RuleSetBuilder};

use crate::languages;
use crate::rules;
use crate::values::Value;

pub struct LocaleRegistry {
    rule_sets: HashMap<String, Arc<RuleSet<Value>>>,
}

impl LocaleRegistry {
    /// Build rule sets for all 28 supported locales.
    /// Called once at server startup.
    pub fn build_all() -> Self {
        let mut map = HashMap::new();

        let language_rules: &[(&str, fn(&RuleSetBuilder<Value>, Option<Arc<TimeContext>>))] = &[
            ("ar", languages::ar::time::rules),
            ("bg", languages::bg::time::rules),
            ("ca", languages::ca::time::rules),
            ("da", languages::da::time::rules),
            ("de", languages::de::time::rules),
            ("el", languages::el::time::rules),
            ("en", languages::en::time::rules),
            ("es", languages::es::time::rules),
            ("fr", languages::fr::time::rules),
            ("ga", languages::ga::time::rules),
            ("he", languages::he::time::rules),
            ("hr", languages::hr::time::rules),
            ("hu", languages::hu::time::rules),
            ("it", languages::it::time::rules),
            ("ja", languages::ja::time::rules),
            ("ka", languages::ka::time::rules),
            ("ko", languages::ko::time::rules),
            ("nb", languages::nb::time::rules),
            ("nl", languages::nl::time::rules),
            ("pl", languages::pl::time::rules),
            ("pt", languages::pt::time::rules),
            ("ro", languages::ro::time::rules),
            ("ru", languages::ru::time::rules),
            ("sv", languages::sv::time::rules),
            ("tr", languages::tr::time::rules),
            ("uk", languages::uk::time::rules),
            ("vi", languages::vi::time::rules),
            ("zh", languages::zh::time::rules),
        ];

        let ctx = Arc::new(TimeContext::default());

        for (code, register_fn) in language_rules {
            let b = RuleSetBuilder::new(
                BoundariesChecker::detailed(),
                BoundariesChecker::separated_alphanumeric_word(),
            );
            rules::integer::rules(&b);
            rules::duration::rules(&b);
            register_fn(&b, Some(Arc::clone(&ctx)));
            map.insert(code.to_string(), Arc::new(b.build()));
        }

        Self { rule_sets: map }
    }

    /// Look up a rule set by locale code (e.g. "fr", "zh").
    /// Returns None if the locale is not supported.
    pub fn get(&self, locale: &str) -> Option<&Arc<RuleSet<Value>>> {
        self.rule_sets.get(locale)
    }

    /// List all supported locale codes, sorted.
    pub fn supported_locales(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.rule_sets.keys().map(|s| s.as_str()).collect();
        v.sort();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_supports_known_locales() {
        let registry = LocaleRegistry::build_all();
        assert!(registry.get("en").is_some());
        assert!(registry.get("fr").is_some());
        assert!(registry.get("zh").is_some());
        assert!(registry.get("xx").is_none());
    }

    #[test]
    fn test_supported_locales_count() {
        let registry = LocaleRegistry::build_all();
        let locales = registry.supported_locales();
        assert!(locales.len() >= 28, "expected 28 locales, got {}", locales.len());
    }
}
