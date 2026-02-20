#!/usr/bin/env python3
"""
Duckling Rule Extractor
Extracts rule metadata from Duckling Haskell files for code generation.

Usage:
    python3 extract_rules.py --lang ES --output es_rules.json

This extracts rule patterns that can be used to generate Rust scaffolding.
"""

import re
import json
import argparse
import os
from pathlib import Path
from typing import Dict, List, Any, Optional


class DucklingRuleExtractor:
    """Extracts rule metadata from Duckling Haskell files."""

    def __init__(self, duckling_path: str):
        self.duckling_path = Path(duckling_path)

    def extract_language(self, lang: str) -> Dict[str, Any]:
        """Extract all rules for a given language."""
        rules_path = self.duckling_path / "Duckling" / "Time" / lang / "Rules.hs"

        if not rules_path.exists():
            raise FileNotFoundError(f"Rules.hs not found for language: {lang}")

        content = rules_path.read_text()

        return {
            "language": lang,
            "rules": self._extract_rules(content),
            "helpers": self._extract_helpers(content),
            "rule_groups": self._extract_rule_groups(content),
        }

    def _extract_rules(self, content: str) -> List[Dict[str, str]]:
        """Extract individual rule definitions."""
        rules = []

        # Pattern for rule definitions
        # ruleName :: Rule
        # ruleName = Rule { ... }
        rule_pattern = re.compile(
            r'rule(\w+)::\s*Rule\s*\n\s*rule\1\s*=\s*Rule\s*\{([^}]+)\}',
            re.MULTILINE | re.DOTALL
        )

        for match in rule_pattern.finditer(content):
            name = match.group(1)
            body = match.group(2)

            # Extract regex pattern
            pattern_match = re.search(r'pattern\s*=\s*\[(.*?)\]', body, re.DOTALL)
            pattern = pattern_match.group(1) if pattern_match else None

            # Clean up pattern
            if pattern:
                pattern = self._clean_pattern(pattern)

            rules.append({
                "name": name,
                "pattern": pattern,
                "type": "simple"
            })

        return rules

    def _clean_pattern(self, pattern: str) -> str:
        """Clean up regex pattern for cross-platform use."""
        # Remove "regex " wrapper
        pattern = re.sub(r'regex\s+', '', pattern)
        # Remove quotes
        pattern = pattern.strip('"')
        return pattern

    def _extract_helpers(self, content: str) -> List[Dict[str, Any]]:
        """Extract helper function definitions."""
        helpers = []

        # Pattern for mkRule* helper calls
        helper_pattern = re.compile(
            r'mkRule(\w+)\s+(\[.*?\]|\([^)]+\))',
            re.MULTILINE | re.DOTALL
        )

        for match in helper_pattern.finditer(content):
            helper_name = match.group(1)
            helper_args = match.group(2)

            helpers.append({
                "name": f"mkRule{helper_name}",
                "args": helper_args[:100]  # Truncate for preview
            })

        return helpers

    def _extract_rule_groups(self, content: str) -> List[str]:
        """Extract rule group names (e.g., ruleDaysOfWeek, ruleMonths)."""
        groups = []

        # Pattern for ruleGroupName :: [Rule]
        group_pattern = re.compile(r'^rule(\w+) :: \[Rule\]$', re.MULTILINE)

        for match in group_pattern.finditer(content):
            groups.append(match.group(1))

        return groups


def analyze_common_patterns(duckling_path: str, languages: List[str]) -> Dict[str, Any]:
    """Analyze common patterns across multiple languages."""
    extractor = DucklingRuleExtractor(duckling_path)

    all_rules = {}
    common_groups = {}

    for lang in languages:
        try:
            data = extractor.extract_language(lang)
            all_rules[lang] = len(data['rules'])
            common_groups[lang] = data['rule_groups']
        except Exception as e:
            print(f"Error extracting {lang}: {e}")

    return {
        "total_languages": len(all_rules),
        "rules_per_language": all_rules,
        "common_groups": _find_common_groups(common_groups),
    }


def _find_common_groups(groups: Dict[str, List[str]]) -> List[str]:
    """Find groups that appear in multiple languages."""
    from collections import Counter

    all_groups = []
    for lang, group_list in groups.items():
        all_groups.extend(group_list)

    counts = Counter(all_groups)
    return [g for g, count in counts.items() if count >= 3]


# Example usage and testing
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Extract Duckling rules")
    parser.add_argument("--lang", default="ES", help="Language code")
    parser.add_argument("--duckling", default="~/Project/duckling", help="Duckling path")
    parser.add_argument("--output", help="Output JSON file")
    parser.add_argument("--analyze", action="store_true", help="Analyze common patterns")

    args = parser.parse_args()

    duckling_path = os.path.expanduser(args.duckling)
    extractor = DucklingRuleExtractor(duckling_path)

    if args.analyze:
        # Analyze common patterns across major languages
        languages = ["EN", "ES", "FR", "DE", "IT", "PT", "ZH"]
        result = analyze_common_patterns(duckling_path, languages)
        print(json.dumps(result, indent=2))
    else:
        # Extract specific language
        result = extractor.extract_language(args.lang)

        # Summary
        print(f"=== {args.lang} Rules Summary ===")
        print(f"Total rules: {len(result['rules'])}")
        print(f"Rule groups: {len(result['rule_groups'])}")
        print(f"Helpers: {len(result['helpers'])}")

        # Show sample rules
        print("\n=== Sample Rules ===")
        for rule in result['rules'][:5]:
            if rule['pattern']:
                print(f"  {rule['name']}: {rule['pattern'][:50]}...")

        # Show rule groups
        print("\n=== Rule Groups ===")
        for group in result['rule_groups']:
            print(f"  - {group}")

        if args.output:
            with open(args.output, 'w') as f:
                json.dump(result, f, indent=2)
            print(f"\nWritten to {args.output}")
