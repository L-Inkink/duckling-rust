#!/usr/bin/env python3
"""
Duckling Data Table Extractor
Extracts time-related data tables from Duckling for code generation.

This is the most valuable data for code generation:
- Day names
- Month names
- Season names
- Holiday patterns

Usage:
    python3 extract_data.py --lang ZH
"""

import re
import json
import os
import argparse
from pathlib import Path
from typing import Dict, List, Any, Optional


class DataTableExtractor:
    """Extracts data tables from Duckling Haskell files."""

    def __init__(self, duckling_path: str):
        self.duckling_path = Path(duckling_path)

    def extract_language(self, lang: str) -> Dict[str, Any]:
        """Extract all data tables for a language."""
        rules_path = self.duckling_path / "Duckling" / "Time" / lang / "Rules.hs"

        if not rules_path.exists():
            raise FileNotFoundError(f"Rules.hs not found for: {lang}")

        content = rules_path.read_text()

        return {
            "language": lang,
            "days_of_week": self._extract_days_of_week(content),
            "months": self._extract_months(content),
            "seasons": self._extract_seasons(content),
            "instants": self._extract_instants(content),
            "holidays": self._extract_holidays(content),
        }

    def _extract_days_of_week(self, content: str) -> List[Dict[str, str]]:
        """Extract day of week patterns."""
        # Pattern: ("Name", "regex|pattern")
        pattern = re.compile(
            r'ruleDaysOfWeek\s*=\s*mkRuleDaysOfWeek\s*\[(.*?)\]',
            re.DOTALL
        )

        match = pattern.search(content)
        if not match:
            return []

        data_block = match.group(1)
        days = []

        # Extract each entry
        entry_pattern = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')
        for entry in entry_pattern.finditer(data_block):
            days.append({
                "name": entry.group(1),
                "pattern": entry.group(2)
            })

        return days

    def _extract_months(self, content: str) -> List[Dict[str, str]]:
        """Extract month patterns."""
        pattern = re.compile(
            r'ruleMonths\s*=\s*mkRuleMonths\s*\[(.*?)\]',
            re.DOTALL
        )

        match = pattern.search(content)
        if not match:
            return []

        data_block = match.group(1)
        months = []

        entry_pattern = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')
        for entry in entry_pattern.finditer(data_block):
            months.append({
                "name": entry.group(1),
                "pattern": entry.group(2)
            })

        return months

    def _extract_seasons(self, content: str) -> List[Dict[str, str]]:
        """Extract season patterns."""
        pattern = re.compile(
            r'ruleSeasons\s*=\s*mkRuleSeasons\s*\[(.*?)\]',
            re.DOTALL
        )

        match = pattern.search(content)
        if not match:
            return []

        data_block = match.group(1)
        seasons = []

        entry_pattern = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')
        for entry in entry_pattern.finditer(data_block):
            seasons.append({
                "name": entry.group(1),
                "pattern": entry.group(2)
            })

        return seasons

    def _extract_instants(self, content: str) -> List[Dict[str, str]]:
        """Extract instant patterns (now, today, etc.)."""
        pattern = re.compile(
            r'ruleInstants\s*=\s*mkRuleInstants\s*\[(.*?)\]',
            re.DOTALL
        )

        match = pattern.search(content)
        if not match:
            return []

        data_block = match.group(1)
        instants = []

        entry_pattern = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')
        for entry in entry_pattern.finditer(data_block):
            instants.append({
                "name": entry.group(1),
                "pattern": entry.group(2)
            })

        return instants

    def _extract_holidays(self, content: str) -> List[Dict[str, str]]:
        """Extract holiday patterns."""
        pattern = re.compile(
            r'ruleHolidays\s*=\s*mkRuleHolidays\s*\[(.*?)\]',
            re.DOTALL
        )

        match = pattern.search(content)
        if not match:
            return []

        data_block = match.group(1)
        holidays = []

        entry_pattern = re.compile(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)')
        for entry in entry_pattern.finditer(data_block):
            holidays.append({
                "name": entry.group(1),
                "pattern": entry.group(2)
            })

        return holidays


def generate_rust_scaffolding(data: Dict[str, Any]) -> str:
    """Generate Rust scaffolding from extracted data."""

    lang = data['language']
    rust_code = f"""// {lang} Time Rules - Auto-generated from Duckling
// Generated: {__import__('datetime').datetime.now().isoformat()}

use crate::values::Value;
use rustling_core::time::{Grain, TimeContext, TimeValue};
use rustling_core::{{RuleSetBuilder, rustling_error}};
use chrono::{{Datelike, Duration, TimeZone, Utc, Weekday}};
use std::sync::Arc;

// ========================================
// Days of Week ({len(data['days_of_week'])} rules)
// ========================================

fn add_days_of_week_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {{
    let ctx_dow = Arc::clone(&ctx);
"""

    # Generate day rules
    day_patterns = {
        "Monday": "Mon",
        "Tuesday": "Tue",
        "Wednesday": "Wed",
        "Thursday": "Thu",
        "Friday": "Fri",
        "Saturday": "Sat",
        "Sunday": "Sun"
    }

    for day in data['days_of_week']:
        pattern = day.get('pattern', '')
        rust_code += f"""
    // {day['name']}
    let ctx_day = Arc::clone(&ctx_dow);
    b.rule_1_terminal(
        "time:dow:{day['name']}",
        b.reg(r"{pattern}").unwrap(),
        move |_| {{
            let ref_time = ctx_day.reference_local();
            let current_dow = ref_time.weekday();
            let target_dow = Weekday::{day_patterns.get(day['name'], 'Mon')};
            // TODO: Calculate days until target weekday
            Ok(Value::Time(TimeValue::instant(ref_time, Grain::Day)))
        }},
    );
"""

    # Generate month rules
    rust_code += """
}

// ========================================
// Months ({len(data['months'])} rules)
// ========================================

fn add_month_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {
    // TODO: Implement month rules
}
"""

    # Generate instant rules
    if data['instants']:
        rust_code += f"""
// ========================================
// Instants ({len(data['instants'])} rules)
// ========================================

fn add_instant_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {{
    let ctx_instant = Arc::clone(&ctx);
"""
        for instant in data['instants']:
            rust_code += f"""
    // {instant['name']}
    b.rule_1_terminal(
        "time:instant:{instant['name']}",
        b.reg(r"{instant['pattern']}").unwrap(),
        move |_| {{
            Ok(Value::Time(TimeValue::instant(ctx_instant.reference_utc(), Grain::Second)))
        }},
    );
"""
        rust_code += "}\n"

    return rust_code


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Extract data tables from Duckling")
    parser.add_argument("--lang", default="ZH", help="Language code")
    parser.add_argument("--duckling", default="~/Project/duckling", help="Duckling path")
    parser.add_argument("--output", help="Output Rust file")
    parser.add_argument("--json", action="store_true", help="Output JSON")

    args = parser.parse_args()

    duckling_path = os.path.expanduser(args.duckling)
    extractor = DataTableExtractor(duckling_path)

    try:
        data = extractor.extract_language(args.lang)

        if args.json:
            print(json.dumps(data, indent=2))
        else:
            print(f"=== {args.lang} Data Tables ===")
            print(f"Days of Week: {len(data['days_of_week'])}")
            print(f"Months: {len(data['months'])}")
            print(f"Seasons: {len(data['seasons'])}")
            print(f"Instants: {len(data['instants'])}")
            print(f"Holidays: {len(data['holidays'])}")

            if args.output:
                code = generate_rust_scaffolding(data)
                with open(args.output, 'w') as f:
                    f.write(code)
                print(f"\nWritten Rust scaffolding to {args.output}")

    except Exception as e:
        print(f"Error: {e}")
