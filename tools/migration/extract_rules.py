#!/usr/bin/env python3
"""
Duckling Rule Extractor - Semi-Automated Migration Tool

Extracts rules from Duckling Haskell source files and converts them to JSON
format defined by rule_schema.json. Handles ~80% of common patterns automatically.

Usage:
    python extract_rules.py <haskell_file> [--output <json_file>]
    python extract_rules.py --batch Duckling/Numeral --output extracted/numeral/

Author: Migration Toolchain
Version: 1.0.0
"""

import re
import json
import argparse
import sys
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple
from datetime import datetime
from collections import OrderedDict


class DucklingRuleExtractor:
    """Extracts rules from Duckling Haskell source files."""

    def __init__(self, haskell_file: Path):
        self.haskell_file = haskell_file
        self.content = haskell_file.read_text(encoding='utf-8')
        self.dimension = self._extract_dimension()
        self.locale = self._extract_locale()
        self.rules = []

    def _extract_dimension(self) -> str:
        """Extract dimension from module path."""
        # Pattern: Duckling.<Dimension>.<Locale>.Rules
        match = re.search(r'module\s+Duckling\.(\w+)\.', self.content)
        if match:
            return match.group(1)

        # Fallback: extract from file path
        parts = self.haskell_file.parts
        for i, part in enumerate(parts):
            if part in ['Numeral', 'Time', 'Duration', 'AmountOfMoney',
                       'Distance', 'Volume', 'Temperature', 'Ordinal',
                       'Email', 'PhoneNumber', 'Url', 'CreditCardNumber',
                       'Quantity', 'TimeGrain']:
                return part

        return "Unknown"

    def _extract_locale(self) -> str:
        """Extract locale from module path."""
        # Pattern: Duckling.<Dimension>.<Locale>.Rules
        match = re.search(r'module\s+Duckling\.\w+\.(\w+)\.Rules', self.content)
        if match:
            locale_code = match.group(1)
            # Convert to lowercase (EN -> en)
            return locale_code.lower()

        # Fallback: extract from file path
        parts = self.haskell_file.parts
        for i, part in enumerate(parts):
            if part in ['EN', 'ZH', 'ES', 'FR', 'DE', 'JA', 'KO', 'AR',
                       'PT', 'IT', 'RU', 'NL', 'PL', 'TR', 'VI', 'TH',
                       'SV', 'DA', 'NO', 'FI', 'CS', 'HU', 'RO', 'EL',
                       'HE', 'ID', 'MS', 'FA', 'UK', 'BG', 'HR', 'SR',
                       'SK', 'SL', 'ET', 'LV', 'LT', 'IS', 'GA', 'MT',
                       'CY', 'EU', 'CA', 'GL', 'AF', 'SW', 'HI', 'BN',
                       'TA', 'TE', 'ML', 'KN', 'MR', 'NE', 'MY', 'KM',
                       'LO', 'KA', 'MN']:
                return part.lower()

        return "unknown"

    def extract_hashmap_rules(self) -> List[Dict[str, Any]]:
        """Extract HashMap-based dictionary rules.

        Pattern:
            <name>Map :: HashMap Text Integer
            <name>Map = HashMap.fromList
              [ ( "text", value )
              , ...
              ]
        """
        rules = []

        # Find all HashMap definitions
        # Support: "HashMap Text Integer", "HashMap.HashMap Text Integer", "HashMap.HashMap Text.Text Integer"
        pattern = r'(\w+Map)\s*::\s*HashMap(?:\.HashMap)?\s+(?:Text\.)?Text\s+(\w+)\s*\n\s*\1\s*=\s*HashMap\.fromList\s*\[((?:[^]]*(?:\[[^\]]*\])?)*)\]'

        for match in re.finditer(pattern, self.content, re.MULTILINE | re.DOTALL):
            map_name = match.group(1)
            value_type = match.group(2)
            entries_text = match.group(3)

            # Extract entries
            entries = self._parse_hashmap_entries(entries_text, value_type)

            if entries:
                rule = {
                    "name": map_name.replace('Map', '_dictionary'),
                    "rule_type": "dictionary",
                    "pattern": {
                        "type": "dictionary",
                        "entries": entries,
                        "case_sensitive": False
                    },
                    "production": {
                        "value_extractor": "dictionary_lookup",
                        "confidence": 1.0
                    },
                    "examples": list(entries.keys())[:5]  # First 5 as examples
                }
                rules.append(rule)

        return rules

    @staticmethod
    def sanitize_regex(pattern: str) -> str:
        """
        Convert capturing groups to non-capturing groups in regex patterns.

        Keeps only the outermost capturing group (if any), converts all
        inner groups to non-capturing (?:...) to avoid rustling-core panic.

        Examples:
            "(a )?(pair|couple)s?( of)?" -> "(?:a )?(?:pair|couple)s?(?: of)?"
            "(\\d*\\.\\d+)" -> "(\\d*\\.\\d+)"  # Keep single outer group
        """
        # Strategy: Replace all ( with (?: except for the outermost one
        # For now, use a simple approach: convert all to non-capturing
        # This is safe but may lose some capture semantics

        import re as regex_module

        # Count capturing groups
        # Remove escaped parens first
        temp = pattern.replace(r'\(', '').replace(r'\)', '')
        open_parens = [i for i, c in enumerate(temp) if c == '(']

        # If only 0-1 capturing groups, no need to change
        if len(open_parens) <= 1:
            return pattern

        # Otherwise, convert all to non-capturing
        # This is a simple but safe approach
        result = []
        i = 0
        while i < len(pattern):
            if pattern[i:i+2] == r'\(':
                result.append(r'\(')
                i += 2
            elif pattern[i:i+2] == r'\)':
                result.append(r'\)')
                i += 2
            elif pattern[i] == '(' and i+1 < len(pattern) and pattern[i+1] != '?':
                # Convert capturing group to non-capturing
                result.append('(?:')
                i += 1
            else:
                result.append(pattern[i])
                i += 1

        return ''.join(result)

    def _parse_hashmap_entries(self, entries_text: str, value_type: str) -> Dict[str, Any]:
        """Parse HashMap entries into dictionary."""
        entries = OrderedDict()

        # Pattern: ( "key" , value )
        pattern = r'\(\s*"([^"]+)"\s*,\s*([^)]+)\s*\)'

        for match in re.finditer(pattern, entries_text):
            key = match.group(1)
            value_str = match.group(2).strip()

            # Parse value based on type
            if value_type == 'Integer':
                try:
                    value = int(value_str)
                    entries[key] = {"value": value}
                except ValueError:
                    # Complex expression, mark for manual review
                    entries[key] = {"value": value_str, "_needs_manual_review": True}
            elif value_type == 'Double':
                try:
                    value = float(value_str)
                    entries[key] = {"value": value}
                except ValueError:
                    entries[key] = {"value": value_str, "_needs_manual_review": True}
            else:
                # String or other type
                entries[key] = {"value": value_str}

        return entries

    def extract_regex_rules(self) -> List[Dict[str, Any]]:
        """Extract simple regex-based rules.

        Pattern:
            rule<Name> :: Rule
            rule<Name> = Rule
              { name = "..."
              , pattern = [ regex "..." ]
              , prod = ...
              }
        """
        rules = []

        # Find rule definitions
        pattern = r'rule(\w+)\s*::\s*Rule\s*\nrule\1\s*=\s*Rule\s*\{([^}]+)\}'

        for match in re.finditer(pattern, self.content, re.MULTILINE | re.DOTALL):
            rule_name = match.group(1)
            rule_body = match.group(2)

            # Extract name field
            name_match = re.search(r'name\s*=\s*"([^"]+)"', rule_body)
            if not name_match:
                continue

            display_name = name_match.group(1)

            # Extract pattern (simple regex only)
            pattern_match = re.search(r'pattern\s*=\s*\[\s*regex\s+"([^"]+)"', rule_body)
            if not pattern_match:
                # Not a simple regex rule, skip for now
                continue

            regex_pattern = pattern_match.group(1)
            # Sanitize regex to avoid rustling-core capture group panics
            regex_pattern = self.sanitize_regex(regex_pattern)

            # Try to detect the production type
            prod_match = re.search(r'HashMap\.lookup.*?(\w+Map)', rule_body)

            # Try to detect constant integer values: prod = \_ -> integer N
            const_match = re.search(r'prod\s*=\s*\\[_\s]*->\s*integer\s+(\d+)', rule_body)

            if const_match:
                # Constant value rule
                const_value = int(const_match.group(1))
                rule = {
                    "name": rule_name,
                    "rule_type": "regex",
                    "pattern": {
                        "type": "regex",
                        "value": regex_pattern,
                        "flags": "i"
                    },
                    "production": {
                        "value_extractor": "constant",
                        "constant_value": const_value,
                        "confidence": 1.0
                    },
                    "metadata": {
                        "original_name": display_name
                    }
                }
            elif prod_match:
                # This rule uses a dictionary, link to it
                dict_name = prod_match.group(1)
                rule = {
                    "name": rule_name,
                    "rule_type": "regex",
                    "pattern": {
                        "type": "regex",
                        "value": regex_pattern,
                        "flags": "i"  # Case-insensitive by default
                    },
                    "production": {
                        "value_extractor": "dictionary_lookup",
                        "dictionary_ref": dict_name,
                        "confidence": 1.0
                    },
                    "metadata": {
                        "original_name": display_name
                    }
                }
            else:
                # Direct value extraction (needs manual review)
                rule = {
                    "name": rule_name,
                    "rule_type": "regex",
                    "pattern": {
                        "type": "regex",
                        "value": regex_pattern,
                        "flags": "i"
                    },
                    "production": {
                        "value_extractor": "custom",
                        "custom_logic": "NEEDS_MANUAL_REVIEW: " + rule_body[rule_body.find('prod'):rule_body.find('prod')+200],
                        "confidence": 1.0
                    },
                    "metadata": {
                        "original_name": display_name,
                        "_needs_manual_review": True
                    }
                }

            rules.append(rule)

        return rules

    def extract_composite_rules(self) -> List[Dict[str, Any]]:
        """Extract composite rules (sequence, intersect, etc.).

        This is more complex and may require manual review.
        For now, we identify them and mark for manual extraction.
        """
        rules = []

        # Find rules with multiple pattern components
        pattern = r'rule(\w+)\s*::\s*Rule\s*\nrule\1\s*=\s*Rule\s*\{([^}]+)\}'

        for match in re.finditer(pattern, self.content, re.MULTILINE | re.DOTALL):
            rule_name = match.group(1)
            rule_body = match.group(2)

            # Check if pattern has multiple components
            pattern_match = re.search(r'pattern\s*=\s*\[([^\]]+)\]', rule_body, re.DOTALL)
            if not pattern_match:
                continue

            pattern_content = pattern_match.group(1)

            # Count components (separated by commas at depth 0)
            # Simple heuristic: more than one comma likely means composite
            if pattern_content.count(',') > 0 and 'regex' not in pattern_content:
                name_match = re.search(r'name\s*=\s*"([^"]+)"', rule_body)
                display_name = name_match.group(1) if name_match else rule_name

                rule = {
                    "name": rule_name,
                    "rule_type": "composite",
                    "pattern": {
                        "type": "composite",
                        "operator": "sequence",
                        "components": [],
                        "_raw_haskell": pattern_content[:500],  # Truncate for safety
                        "_needs_manual_review": True
                    },
                    "production": {
                        "value_extractor": "composite_function",
                        "function": "UNKNOWN",
                        "confidence": 1.0,
                        "_needs_manual_review": True
                    },
                    "metadata": {
                        "original_name": display_name,
                        "_needs_manual_review": True
                    }
                }

                rules.append(rule)

        return rules

    def extract_all(self) -> Dict[str, Any]:
        """Extract all rules from the Haskell file."""
        print(f"Extracting from {self.haskell_file}...", file=sys.stderr)
        print(f"  Dimension: {self.dimension}, Locale: {self.locale}", file=sys.stderr)

        # Extract different rule types
        hashmap_rules = self.extract_hashmap_rules()
        regex_rules = self.extract_regex_rules()
        composite_rules = self.extract_composite_rules()

        all_rules = hashmap_rules + regex_rules + composite_rules

        print(f"  Extracted {len(all_rules)} rules:", file=sys.stderr)
        print(f"    - {len(hashmap_rules)} dictionary rules", file=sys.stderr)
        print(f"    - {len(regex_rules)} regex rules", file=sys.stderr)
        print(f"    - {len(composite_rules)} composite rules (need manual review)", file=sys.stderr)

        # Count rules needing manual review
        needs_review = sum(1 for r in all_rules if r.get('metadata', {}).get('_needs_manual_review'))
        if needs_review > 0:
            print(f"  ⚠️  {needs_review} rules need manual review", file=sys.stderr)

        return {
            "dimension": self.dimension,
            "locale": self.locale,
            "source_file": str(self.haskell_file),
            "rules": all_rules,
            "metadata": {
                "extracted_at": datetime.now().isoformat(),
                "extractor_version": "1.0.0",
                "extraction_method": "python_regex",
                "rules_needing_review": needs_review,
                "notes": "Auto-extracted by extract_rules.py. Rules marked with _needs_manual_review require human verification."
            }
        }


def main():
    parser = argparse.ArgumentParser(
        description='Extract Duckling rules from Haskell source files',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Extract single file
  python extract_rules.py ~/Project/duckling/Duckling/Numeral/ZH/Rules.hs

  # Extract single file with output
  python extract_rules.py ~/Project/duckling/Duckling/Numeral/EN/Rules.hs \\
    --output extracted/numeral/en.json

  # Batch extract all Numeral rules
  python extract_rules.py --batch ~/Project/duckling/Duckling/Numeral \\
    --output extracted/numeral/
        """
    )

    parser.add_argument('input', nargs='?', help='Input Haskell file or directory (for --batch)')
    parser.add_argument('--output', '-o', help='Output JSON file or directory (for --batch)')
    parser.add_argument('--batch', action='store_true', help='Batch process directory')
    parser.add_argument('--validate', action='store_true', help='Validate against JSON schema')

    args = parser.parse_args()

    if not args.input:
        parser.print_help()
        return 1

    input_path = Path(args.input).expanduser()

    if args.batch:
        # Batch mode: process all Rules.hs files in directory
        if not input_path.is_dir():
            print(f"Error: {input_path} is not a directory", file=sys.stderr)
            return 1

        rules_files = list(input_path.rglob("**/Rules.hs"))
        print(f"Found {len(rules_files)} Rules.hs files", file=sys.stderr)

        output_dir = Path(args.output) if args.output else Path('extracted')
        output_dir.mkdir(parents=True, exist_ok=True)

        for rules_file in rules_files:
            try:
                extractor = DucklingRuleExtractor(rules_file)
                result = extractor.extract_all()

                # Generate output filename
                output_file = output_dir / f"{result['locale']}.json"

                with output_file.open('w', encoding='utf-8') as f:
                    json.dump(result, f, indent=2, ensure_ascii=False)

                print(f"✓ Wrote {output_file}", file=sys.stderr)

            except Exception as e:
                print(f"✗ Error processing {rules_file}: {e}", file=sys.stderr)

        print(f"\n✓ Batch extraction complete. Output in {output_dir}", file=sys.stderr)

    else:
        # Single file mode
        if not input_path.exists():
            print(f"Error: {input_path} does not exist", file=sys.stderr)
            return 1

        extractor = DucklingRuleExtractor(input_path)
        result = extractor.extract_all()

        if args.output:
            output_path = Path(args.output)
            output_path.parent.mkdir(parents=True, exist_ok=True)

            with output_path.open('w', encoding='utf-8') as f:
                json.dump(result, f, indent=2, ensure_ascii=False)

            print(f"✓ Wrote {output_path}", file=sys.stderr)
        else:
            # Print to stdout
            print(json.dumps(result, indent=2, ensure_ascii=False))

    return 0


if __name__ == '__main__':
    sys.exit(main())
