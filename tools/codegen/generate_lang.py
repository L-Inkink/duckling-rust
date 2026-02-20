#!/usr/bin/env python3
"""
Duckling to Rust Code Generator

Generates Rust time rules scaffolding from Duckling Haskell data.

Usage:
    python3 generate_lang.py --lang ES --output languages/es/time.rs
"""

import re
import os
import argparse
import json
from pathlib import Path
from datetime import datetime


class RustCodeGenerator:
    """Generates Rust code from Duckling data."""

    # Mapping of English day names to Rust Weekday
    DAY_MAP = {
        "Monday": "Mon",
        "Tuesday": "Tue",
        "Wednesday": "Wed",
        "Thursday": "Thu",
        "Friday": "Fri",
        "Saturday": "Sat",
        "Sunday": "Sun",
    }

    # Mapping of English month names to numbers
    MONTH_MAP = {
        "January": 1,
        "February": 2,
        "March": 3,
        "April": 4,
        "May": 5,
        "June": 6,
        "July": 7,
        "August": 8,
        "September": 9,
        "October": 10,
        "November": 11,
        "December": 12,
    }

    def __init__(self, lang_code: str, lang_name: str):
        self.lang_code = lang_code
        self.lang_name = lang_name

    def generate(self, data: dict) -> str:
        """Generate complete Rust module."""

        return f"""// {self.lang_name} Time rules
// Auto-generated from Duckling
// Generated: {datetime.now().isoformat()}

use crate::values::Value;
use rustling_core::time::{{Form, Grain, TimeContext, TimeData, TimeValue}};
use rustling_core::{{RuleSetBuilder, rustling_error}};
use chrono::{{Datelike, Duration, TimeZone, Timelike, Utc, Weekday}};
use std::sync::Arc;

/// Build {self.lang_name} Time rules
///
/// # Parameters
/// - `b`: RuleSetBuilder for registering rules
/// - `context`: Optional TimeContext for reference time
pub fn rules(b: &RuleSetBuilder<Value>, context: Option<Arc<TimeContext>>) {{
    let ctx = context.unwrap_or_else(|| Arc::new(TimeContext::default()));

    // ========================================
    // Instants (now, today, etc.)
    // ========================================
    self._generate_instant_rules(b, ctx);

    // ========================================
    // Days of Week
    // ========================================
    self._generate_dow_rules(b, ctx);

    // ========================================
    // Months
    // ========================================
    self._generate_month_rules(b, ctx);

    // ========================================
    // Simple Time References
    // ========================================
    self._generate_simple_rules(b, ctx);
}}

fn _generate_instant_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {{
    // TODO: Add instant patterns (now, today, tomorrow, etc.)
    // Based on Duckling patterns

    // Example pattern:
    // let ctx_now = Arc::clone(&ctx);
    // b.rule_1_terminal(
    //     "{self.lang_code.lower()}:time:now",
    //     b.reg(r"now|.today").unwrap(),
    //     move |_| Ok(Value::Time(TimeValue::instant(ctx_now.reference_utc(), Grain::Second)))
    // );
}}

fn _generate_dow_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {{
    // Day of week patterns from Duckling:
{self._format_dow_comments(data.get('days_of_week', []))}
{self._generate_dow_patterns(data.get('days_of_week', []))}
}}

fn _generate_month_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {{
    // Month patterns from Duckling:
{self._format_month_comments(data.get('months', []))}
{self._generate_month_patterns(data.get('months', []))}
}}

fn _generate_simple_rules(b: &RuleSetBuilder<Value>, ctx: Arc<TimeContext>) {{
    // TODO: Add simple time reference patterns
    // Examples: "morning", "afternoon", "evening", etc.
}}

// ========================================
// Helper Functions
// ========================================

{self._generate_helper_docs()}

/// Parse day of week from English name
fn parse_dow(name: &str) -> Option<Weekday> {{
    match name {{
{self._generate_dow_match()}
        _ => None,
    }}
}}

/// Parse month from English name
fn parse_month(name: &str) -> Option<u32> {{
    match name {{
{self._generate_month_match()}
        _ => None,
    }}
}}
"""

    def _format_dow_comments(self, days: list) -> str:
        if not days:
            return "    // (No days of week data found)"
        lines = []
        for day in days[:7]:
            lines.append(f"    // - {day['name']}: {day['pattern'][:50]}...")
        return "\n".join(lines)

    def _format_month_comments(self, months: list) -> str:
        if not months:
            return "    // (No month data found)"
        lines = []
        for month in months[:12]:
            lines.append(f"    // - {month['name']}: {month['pattern'][:50]}...")
        return "\n".join(lines)

    def _generate_dow_patterns(self, days: list) -> str:
        if not days:
            return """
    // Add day patterns here
"""

        code = []
        for day in days:
            rust_day = self.DAY_MAP.get(day['name'], 'Mon')
            code.append(f"""
    // {day['name']}
    let ctx_{day['name'].lower()} = Arc::clone(&ctx);
    b.rule_1_terminal(
        "{self.lang_code.lower()}:time:dow:{day['name'].lower()}",
        b.reg(r"{day['pattern']}").unwrap(),
        move |_| {{
            let local_ref = ctx_{day['name'].lower()}.reference_local();
            // Get current weekday and calculate offset to target
            let current = local_ref.weekday();
            let target = Weekday::{rust_day};
            let days_diff = ((target.num_days_from_monday() as i64)
                - (current.num_days_from_monday() as i64) + 7) % 7;
            let result = local_ref + Duration::days(days_diff);
            let result_utc = result.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Day)))
        }},
    );""")

        return "\n".join(code)

    def _generate_month_patterns(self, months: list) -> str:
        if not months:
            return """
    // Add month patterns here
"""

        code = []
        for month in months:
            month_num = self.MONTH_MAP.get(month['name'], 1)
            code.append(f"""
    // {month['name']} (month {month_num})
    let ctx_{month['name'].lower()} = Arc::clone(&ctx);
    b.rule_1_terminal(
        "{self.lang_code.lower()}:time:month:{month['name'].lower()}",
        b.reg(r"{month['pattern']}").unwrap(),
        move |_| {{
            let ref_time = ctx_{month['name'].lower()}.reference_local();
            let target = ref_time.with_month({month_num}).unwrap();
            let result_utc = target.with_timezone(&Utc);
            Ok(Value::Time(TimeValue::instant(result_utc, Grain::Month)))
        }},
    );""")

        return "\n".join(code)

    def _generate_helper_docs(self) -> str:
        return """/// Helper: intersect two time values
fn intersect(time1: TimeData, time2: TimeData) -> Value {
    // TODO: Implement intersection logic
    Value::Time(TimeValue::instant(time1.datetime, time1.grain))
}

/// Helper: shift time by duration
fn shift(time: TimeData, grain: Grain, offset: i64) -> Value {
    // TODO: Implement shift logic
    let new_time = time.datetime + Duration::days(offset);
    Value::Time(TimeValue::instant(new_time, grain))
}"""

    def _generate_dow_match(self) -> str:
        lines = []
        for eng, rust in self.DAY_MAP.items():
            lines.append(f'        "{eng}" => Some(Weekday::{rust}),')
        return "\n".join(lines)

    def _generate_month_match(self) -> str:
        lines = []
        for eng, num in self.MONTH_MAP.items():
            lines.append(f'        "{eng}" => Some({num}),')
        return "\n".join(lines)


def main():
    import sys

    parser = argparse.ArgumentParser(description="Generate Rust time rules")
    parser.add_argument("--lang", default="ES", help="Language code (e.g., ES, FR, DE)")
    parser.add_argument("--name", default="Spanish", help="Language name")
    parser.add_argument("--data", help="JSON data file (from extract_data.py)")
    parser.add_argument("--output", help="Output file")
    parser.add_argument("--duckling", default="~/Project/duckling", help="Duckling path")

    args = parser.parse_args()

    # Load data
    if args.data:
        with open(args.data) as f:
            data = json.load(f)
    else:
        # Extract from Duckling
        sys.path.insert(0, os.path.dirname(__file__))
        from extract_data import DataTableExtractor

        extractor = DataTableExtractor(os.path.expanduser(args.duckling))
        data = extractor.extract_language(args.lang)

    # Generate code
    generator = RustCodeGenerator(args.lang, args.name)
    code = generator.generate(data)

    if args.output:
        with open(args.output, 'w') as f:
            f.write(code)
        print(f"Generated: {args.output}")
    else:
        print(code)


if __name__ == "__main__":
    main()
