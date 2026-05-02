#!/usr/bin/env python3
"""
Benchmark analyzer for FastRamdisk project.

This script analyzes benchmark results, compares performance across runs,
and generates performance trend reports.
"""

import argparse
import json
import re
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict
from datetime import datetime


@dataclass
class BenchmarkResult:
    """Single benchmark result."""
    name: str
    duration_ms: float
    timestamp: str
    metadata: Dict[str, str]

    def to_dict(self) -> dict:
        return asdict(self)


@dataclass
class BenchmarkComparison:
    """Comparison between two benchmark runs."""
    name: str
    old_duration: float
    new_duration: float
    change_percent: float
    is_improvement: bool

    @property
    def change_ms(self) -> float:
        return self.new_duration - self.old_duration


def parse_benchmark_output(output: str, timestamp: Optional[str] = None) -> List[BenchmarkResult]:
    """Parse benchmark output from cargo bench or custom benchmark script."""
    results = []
    if timestamp is None:
        timestamp = datetime.now().isoformat()

    # Try to parse cargo bench output format
    # Example: "test bench_name ... bench: 100.23 ns/iter (+/- 5.00)"
    pattern = r'test\s+(\w+)\s+.*?bench:\s+([\d.]+)\s+(\w+)/iter'

    for line in output.split('\n'):
        match = re.search(pattern, line)
        if match:
            name = match.group(1)
            value = float(match.group(2))
            unit = match.group(3)

            # Convert to milliseconds
            if unit == 'ns':
                duration_ms = value / 1_000_000
            elif unit == 'us':
                duration_ms = value / 1_000
            elif unit == 'ms':
                duration_ms = value
            elif unit == 's':
                duration_ms = value * 1_000
            else:
                continue

            results.append(BenchmarkResult(
                name=name,
                duration_ms=duration_ms,
                timestamp=timestamp,
                metadata={'unit': unit}
            ))

    return results


def parse_benchmark_json(json_path: Path) -> List[BenchmarkResult]:
    """Parse benchmark results from JSON file."""
    with open(json_path, 'r') as f:
        data = json.load(f)

    results = []
    timestamp = datetime.now().isoformat()

    if isinstance(data, list):
        for item in data:
            results.append(BenchmarkResult(
                name=item.get('name', 'unknown'),
                duration_ms=item.get('duration_ms', 0),
                timestamp=item.get('timestamp', timestamp),
                metadata=item.get('metadata', {})
            ))
    elif isinstance(data, dict):
        for name, value in data.items():
            results.append(BenchmarkResult(
                name=name,
                duration_ms=value.get('duration_ms', 0),
                timestamp=value.get('timestamp', timestamp),
                metadata=value.get('metadata', {})
            ))

    return results


def compare_benchmarks(old_results: List[BenchmarkResult],
                      new_results: List[BenchmarkResult]) -> List[BenchmarkComparison]:
    """Compare two sets of benchmark results."""
    old_dict = {r.name: r for r in old_results}
    new_dict = {r.name: r for r in new_results}

    comparisons = []
    all_names = set(old_dict.keys()) | set(new_dict.keys())

    for name in sorted(all_names):
        if name in old_dict and name in new_dict:
            old_duration = old_dict[name].duration_ms
            new_duration = new_dict[name].duration_ms
            change_percent = ((new_duration - old_duration) / old_duration) * 100
            is_improvement = change_percent < 0

            comparisons.append(BenchmarkComparison(
                name=name,
                old_duration=old_duration,
                new_duration=new_duration,
                change_percent=change_percent,
                is_improvement=is_improvement
            ))

    return comparisons


def print_benchmark_summary(results: List[BenchmarkResult]):
    """Print summary of benchmark results."""
    if not results:
        print("No benchmark results found.")
        return

    print("\n=== Benchmark Summary ===")
    print(f"Total benchmarks: {len(results)}")

    total_duration = sum(r.duration_ms for r in results)
    print(f"Total duration: {total_duration:.2f} ms")

    avg_duration = total_duration / len(results)
    print(f"Average duration: {avg_duration:.2f} ms")

    print("\n=== Individual Results ===")
    for result in sorted(results, key=lambda r: r.duration_ms, reverse=True):
        print(f"{result.name:30s} {result.duration_ms:10.2f} ms")


def print_comparison_summary(comparisons: List[BenchmarkComparison],
                           threshold: float = 10.0):
    """Print summary of benchmark comparisons."""
    if not comparisons:
        print("No comparisons available.")
        return

    print("\n=== Benchmark Comparison ===")

    improvements = [c for c in comparisons if c.is_improvement]
    regressions = [c for c in comparisons if not c.is_improvement]

    print(f"Total comparisons: {len(comparisons)}")
    print(f"Improvements: {len(improvements)}")
    print(f"Regressions: {len(regressions)}")

    significant_changes = [c for c in comparisons if abs(c.change_percent) > threshold]

    if significant_changes:
        print(f"\n=== Significant Changes (>{threshold}%) ===")
        for comp in sorted(significant_changes, key=lambda c: abs(c.change_percent), reverse=True):
            direction = "↓" if comp.is_improvement else "↑"
            print(f"{direction} {comp.name:30s} {comp.old_duration:10.2f} ms → {comp.new_duration:10.2f} ms ({comp.change_percent:+.2f}%)")
    else:
        print(f"\n✓ No significant changes (threshold: {threshold}%)")


def save_results(results: List[BenchmarkResult], output_path: Path):
    """Save benchmark results to JSON file."""
    data = [r.to_dict() for r in results]
    with open(output_path, 'w') as f:
        json.dump(data, f, indent=2)
    print(f"\nResults saved to {output_path}")


def main():
    parser = argparse.ArgumentParser(description='Analyze benchmark results')
    parser.add_argument('--input', type=Path, help='Input file (JSON or text output)')
    parser.add_argument('--compare', type=Path, help='Previous benchmark results to compare against')
    parser.add_argument('--output', type=Path, help='Output JSON file for results')
    parser.add_argument('--threshold', type=float, default=10.0,
                        help='Significant change threshold percentage (default: 10)')
    parser.add_argument('--stdin', action='store_true', help='Read benchmark output from stdin')

    args = parser.parse_args()

    # Parse input
    if args.stdin:
        import sys
        output = sys.stdin.read()
        results = parse_benchmark_output(output)
    elif args.input:
        if args.input.suffix == '.json':
            results = parse_benchmark_json(args.input)
        else:
            with open(args.input, 'r') as f:
                output = f.read()
            results = parse_benchmark_output(output)
    else:
        parser.error("Must provide --input or --stdin")

    print_benchmark_summary(results)

    # Save results if requested
    if args.output:
        save_results(results, args.output)

    # Compare if requested
    if args.compare:
        if args.compare.suffix == '.json':
            old_results = parse_benchmark_json(args.compare)
        else:
            with open(args.compare, 'r') as f:
                output = f.read()
            old_results = parse_benchmark_output(output)

        comparisons = compare_benchmarks(old_results, results)
        print_comparison_summary(comparisons, args.threshold)


if __name__ == '__main__':
    main()
