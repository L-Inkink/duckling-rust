#!/usr/bin/env python3
"""
Duckling Corpus Test Converter

Converts Duckling Corpus YAML files to Rust test cases.

Usage:
    python corpus_converter.py --dimension Numeral --output languages/tests/
    python corpus_converter.py --batch ~/Project/duckling/Duckling/Numeral --output tests/

Author: Migration Toolchain
Version: 1.0.0
"""

import argparse
import sys
from pathlib import Path
from typing import Dict, List, Any
import yaml
import json


class CorpusConverter:
    """Converts Duckling Corpus YAML to Rust test cases."""

    def __init__(self, corpus_file: Path):
        self.corpus_file = corpus_file
        self.dimension = self._extract_dimension()
        self.locale = self._extract_locale()

    def _extract_dimension(self) -> str:
        """Extract dimension from file path."""
        parts = self.corpus_file.parts
        for part in parts:
            if part in ['Numeral', 'Time', 'Duration', 'AmountOfMoney',
                       'Distance', 'Volume', 'Temperature', 'Ordinal',
                       'Email', 'PhoneNumber', 'Url', 'CreditCardNumber',
                       'Quantity', 'TimeGrain']:
                return part
        return "Unknown"

    def _extract_locale(self) -> str:
        """Extract locale from file path."""
        parts = self.corpus_file.parts
        # Locale is usually the directory before Corpus.hs
        for i, part in enumerate(parts):
            if 'Corpus' in part and i > 0:
                locale = parts[i - 1]
                return locale.lower()
        return "unknown"

    def parse_corpus(self) -> List[Dict[str, Any]]:
        """Parse Corpus.hs file.

        Duckling Corpus files are actually Haskell, not YAML,
        so we'll do simple text parsing.
        """
        content = self.corpus_file.read_text(encoding='utf-8')

        test_cases = []

        # Try to find test patterns
        # Pattern: "text" -> expected value
        # This is simplified - real parsing would be more complex

        # Look for examples in comments or test definitions
        import re

        # Pattern for simple test cases in corpus
        # ( "text", value )
        pattern = r'\(\s*"([^"]+)"\s*,\s*(\d+)\s*\)'

        for match in re.finditer(pattern, content):
            text = match.group(1)
            value = match.group(2)

            test_cases.append({
                'text': text,
                'expected_value': value,
                'dimension': self.dimension
            })

        print(f"  Extracted {len(test_cases)} test cases from {self.corpus_file.name}", file=sys.stderr)

        return test_cases

    def generate_rust_tests(self, test_cases: List[Dict[str, Any]]) -> str:
        """Generate Rust test code from test cases."""

        dimension_lower = self.dimension.lower()

        rust_code = f"""// Auto-generated Corpus tests for {self.dimension} / {self.locale}
// Source: {self.corpus_file}

#[cfg(test)]
mod {self.locale}_{dimension_lower}_corpus_tests {{
    use rustling_core::*;
    use super::*;

    #[test]
    fn test_{self.locale}_{dimension_lower}_corpus() {{
        let mut builder = RuleSetBuilder::new();
        build_{self.locale}_{dimension_lower}_rules(&mut builder);
        let ruleset = builder.build();

        let test_cases = vec![
"""

        for case in test_cases:
            text = case['text'].replace('"', '\\"')
            expected = case['expected_value']
            rust_code += f'            ("{text}", {expected}),\n'

        rust_code += """        ];

        for (text, expected_value) in test_cases {
            let results = ruleset.parse(text);

            // Check that we got at least one result
            assert!(
                !results.is_empty(),
                "Failed to parse '{}', expected value {}",
                text,
                expected_value
            );

            // Check that at least one result matches the expected value
            let has_match = results.iter().any(|r| {
                if let Token::Integer(int_val) = &r.value {
                    int_val.value == expected_value
                } else {
                    false
                }
            });

            assert!(
                has_match,
                "Parsed '{}' but none of the results matched expected value {}",
                text,
                expected_value
            );
        }
    }
}
"""

        return rust_code

    def convert(self, output_file: Path) -> None:
        """Convert corpus to Rust tests and write to file."""
        print(f"Converting {self.corpus_file}...", file=sys.stderr)
        print(f"  Dimension: {self.dimension}, Locale: {self.locale}", file=sys.stderr)

        test_cases = self.parse_corpus()

        if not test_cases:
            print(f"  ⚠️  No test cases extracted, skipping", file=sys.stderr)
            return

        rust_code = self.generate_rust_tests(test_cases)

        output_file.parent.mkdir(parents=True, exist_ok=True)
        output_file.write_text(rust_code, encoding='utf-8')

        print(f"  ✓ Wrote {output_file}", file=sys.stderr)


def main():
    parser = argparse.ArgumentParser(
        description='Convert Duckling Corpus to Rust test cases',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Convert single corpus file
  python corpus_converter.py \\
    ~/Project/duckling/Duckling/Numeral/EN/Corpus.hs \\
    --output tests/en_numeral_corpus.rs

  # Batch convert all Numeral corpus files
  python corpus_converter.py --batch ~/Project/duckling/Duckling/Numeral \\
    --output tests/numeral/
        """
    )

    parser.add_argument('input', nargs='?', help='Input Corpus.hs file or directory (for --batch)')
    parser.add_argument('--output', '-o', required=True, help='Output Rust test file or directory')
    parser.add_argument('--batch', action='store_true', help='Batch process directory')
    parser.add_argument('--dimension', help='Dimension filter for batch mode')

    args = parser.parse_args()

    if not args.input:
        parser.print_help()
        return 1

    input_path = Path(args.input).expanduser()
    output_path = Path(args.output)

    if args.batch:
        # Batch mode: process all Corpus.hs files in directory
        if not input_path.is_dir():
            print(f"Error: {input_path} is not a directory", file=sys.stderr)
            return 1

        corpus_files = list(input_path.rglob("**/Corpus.hs"))
        print(f"Found {len(corpus_files)} Corpus.hs files", file=sys.stderr)

        for corpus_file in corpus_files:
            try:
                converter = CorpusConverter(corpus_file)

                # Filter by dimension if specified
                if args.dimension and converter.dimension != args.dimension:
                    continue

                # Generate output filename
                output_file = output_path / f"{converter.locale}_{converter.dimension.lower()}_corpus.rs"

                converter.convert(output_file)

            except Exception as e:
                print(f"✗ Error processing {corpus_file}: {e}", file=sys.stderr)
                import traceback
                traceback.print_exc()

        print(f"\n✓ Batch conversion complete. Output in {output_path}", file=sys.stderr)

    else:
        # Single file mode
        if not input_path.exists():
            print(f"Error: {input_path} does not exist", file=sys.stderr)
            return 1

        converter = CorpusConverter(input_path)
        converter.convert(output_path)

    return 0


if __name__ == '__main__':
    sys.exit(main())
