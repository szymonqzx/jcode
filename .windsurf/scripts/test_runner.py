#!/usr/bin/env python3
"""
Test runner for FastRamdisk project with advanced filtering and reporting.

This script provides enhanced test running capabilities including test filtering,
parallel execution, and detailed result reporting.
"""

import argparse
import subprocess
import sys
import json
import re
from pathlib import Path
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from enum import Enum


class TestStatus(Enum):
    """Test execution status."""
    PASSED = "passed"
    FAILED = "failed"
    SKIPPED = "skipped"
    ERROR = "error"


@dataclass
class TestResult:
    """Individual test result."""
    name: str
    status: TestStatus
    duration: float
    output: str
    error: Optional[str] = None

    def to_dict(self) -> dict:
        return {
            'name': self.name,
            'status': self.status.value,
            'duration': self.duration,
            'output': self.output,
            'error': self.error
        }


@dataclass
class TestSuiteResult:
    """Complete test suite result."""
    tests: List[TestResult]
    total_duration: float

    @property
    def passed_count(self) -> int:
        return len([t for t in self.tests if t.status == TestStatus.PASSED])

    @property
    def failed_count(self) -> int:
        return len([t for t in self.tests if t.status == TestStatus.FAILED])

    @property
    def skipped_count(self) -> int:
        return len([t for t in self.tests if t.status == TestStatus.SKIPPED])

    @property
    def error_count(self) -> int:
        return len([t for t in self.tests if t.status == TestStatus.ERROR])

    @property
    def total_count(self) -> int:
        return len(self.tests)


def run_cargo_test(
    filter_pattern: Optional[str] = None,
    lib: bool = True,
    test_name: Optional[str] = None,
    verbose: bool = False,
    no_capture: bool = False
) -> Tuple[int, str, str]:
    """Run cargo test with specified options."""
    cmd = ['cargo', 'test']

    if lib:
        cmd.append('--lib')

    if test_name:
        cmd.append('--test')
        cmd.append(test_name)

    if filter_pattern:
        cmd.append('--')
        cmd.append(filter_pattern)

    if verbose or no_capture:
        cmd.append('--')
        cmd.append('--nocapture')

    result = subprocess.run(
        cmd,
        capture_output=not (verbose or no_capture),
        text=True,
        cwd=Path.cwd()
    )

    stdout = result.stdout if result.stdout else ""
    stderr = result.stderr if result.stderr else ""

    return result.returncode, stdout, stderr


def parse_cargo_test_output(output: str) -> List[TestResult]:
    """Parse cargo test output to extract individual test results."""
    results = []
    current_test = None
    test_output = []

    # Pattern to match test start
    test_start_pattern = re.compile(r'^test\s+(\w+)\s+\.\.\.')

    for line in output.split('\n'):
        match = test_start_pattern.match(line)
        if match:
            # Save previous test if exists
            if current_test:
                results.append(current_test)

            # Start new test
            test_name = match.group(1)
            current_test = TestResult(
                name=test_name,
                status=TestStatus.PASSED,
                duration=0.0,
                output=''
            )
            test_output = []
        elif current_test:
            # Check for test result
            if 'FAILED' in line and current_test.name in line:
                current_test.status = TestStatus.FAILED
            elif 'ok' in line and current_test.name in line:
                current_test.status = TestStatus.PASSED
            elif 'ignored' in line and current_test.name in line:
                current_test.status = TestStatus.SKIPPED
            else:
                test_output.append(line)

    # Add last test
    if current_test:
        current_test.output = '\n'.join(test_output)
        results.append(current_test)

    return results


def print_test_summary(result: TestSuiteResult):
    """Print test execution summary."""
    print("\n=== Test Summary ===")
    print(f"Total tests: {result.total_count}")
    print(f"Passed: {result.passed_count}")
    print(f"Failed: {result.failed_count}")
    print(f"Skipped: {result.skipped_count}")
    print(f"Errors: {result.error_count}")
    print(f"Duration: {result.total_duration:.2f}s")

    if result.failed_count > 0:
        print("\n=== Failed Tests ===")
        for test in [t for t in result.tests if t.status == TestStatus.FAILED]:
            print(f"  - {test.name}")
            if test.error:
                print(f"    Error: {test.error}")


def save_test_results(result: TestSuiteResult, output_path: Path):
    """Save test results to JSON file."""
    data = {
        'summary': {
            'total': result.total_count,
            'passed': result.passed_count,
            'failed': result.failed_count,
            'skipped': result.skipped_count,
            'errors': result.error_count,
            'duration': result.total_duration
        },
        'tests': [t.to_dict() for t in result.tests]
    }

    with open(output_path, 'w') as f:
        json.dump(data, f, indent=2)

    print(f"\nTest results saved to {output_path}")


def main():
    parser = argparse.ArgumentParser(description='Enhanced test runner for FastRamdisk')
    parser.add_argument('--filter', type=str, help='Filter tests by pattern')
    parser.add_argument('--lib', action='store_true', help='Run library tests')
    parser.add_argument('--test', type=str, help='Run specific integration test')
    parser.add_argument('--verbose', action='store_true', help='Verbose output')
    parser.add_argument('--no-capture', action='store_true', help='Disable output capture')
    parser.add_argument('--output', type=Path, help='Output results to JSON file')
    parser.add_argument('--fail-fast', action='store_true', help='Stop on first failure')

    args = parser.parse_args()

    import time
    start_time = time.time()

    returncode, stdout, stderr = run_cargo_test(
        filter_pattern=args.filter,
        lib=args.lib,
        test_name=args.test,
        verbose=args.verbose,
        no_capture=args.no_capture
    )

    duration = time.time() - start_time

    # Parse results
    test_results = parse_cargo_test_output(stdout + stderr)

    # Determine overall status
    if returncode != 0:
        for test in test_results:
            if test.status == TestStatus.PASSED:
                test.status = TestStatus.FAILED

    suite_result = TestSuiteResult(
        tests=test_results,
        total_duration=duration
    )

    # Print summary
    print_test_summary(suite_result)

    # Save results if requested
    if args.output:
        save_test_results(suite_result, args.output)

    # Exit with appropriate code
    if suite_result.failed_count > 0:
        sys.exit(1)
    elif suite_result.error_count > 0:
        sys.exit(1)
    else:
        sys.exit(0)


if __name__ == '__main__':
    main()
