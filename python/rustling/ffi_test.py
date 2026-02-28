#!/usr/bin/env python3
"""Desktop validation: verify Python ctypes wrapper calls librustling correctly."""
import sys
import os

# Add python/ directory to module search path
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))

# Import from the python/ subdirectory (not the symlinked .dylib location)
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

from rustling import parse


def test_integer():
    result = parse("42", "en")
    assert len(result) > 0, f"Expected at least 1 result, got: {result}"
    value = result[0]["value"]
    assert isinstance(value, dict), f"value should be a dict, got: {type(value)}"
    print(f"  ✓ integer: {value}")


def test_duration():
    result = parse("5 minutes", "en")
    assert len(result) > 0, f"Expected at least 1 result, got: {result}"
    # Find the Duration result (there may also be an Integer result for "5")
    duration_results = [r for r in result if "Duration" in r["value"]]
    assert len(duration_results) > 0, f"Expected Duration in results, got: {result}"
    value = duration_results[0]["value"]
    dur = value["Duration"]
    assert dur["amount"] == 5, f"Expected amount=5, got: {dur['amount']}"
    assert dur["unit"] == "Minute", f"Expected unit=Minute, got: {dur['unit']}"
    print(f"  ✓ duration: {value}")


def test_empty():
    result = parse("", "en")
    assert result == [], f"Empty text should return [], got: {result}"
    print("  ✓ empty text -> []")


def test_unsupported_locale():
    try:
        parse("42", "xx")
        assert False, "Expected RuntimeError for unsupported locale"
    except RuntimeError as e:
        print(f"  ✓ unsupported locale -> RuntimeError: {e}")


if __name__ == "__main__":
    print("=== Python ctypes wrapper validation ===")
    test_integer()
    test_duration()
    test_empty()
    test_unsupported_locale()
    print("\nAll tests passed.")
