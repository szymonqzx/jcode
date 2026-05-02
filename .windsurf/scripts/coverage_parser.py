#!/usr/bin/env python3
"""
Coverage report parser and analyzer for FastRamdisk project.

This script parses coverage reports (lcov, cobertura, json) and provides
analysis including coverage gaps, trends, and summary statistics.
"""

import argparse
import json
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass


@dataclass
class FileCoverage:
    """Coverage data for a single file."""
    path: str
    lines_total: int
    lines_covered: int
    branches_total: int
    branches_covered: int
    functions_total: int
    functions_covered: int

    @property
    def line_coverage(self) -> float:
        """Line coverage percentage."""
        if self.lines_total == 0:
            return 0.0
        return (self.lines_covered / self.lines_total) * 100

    @property
    def branch_coverage(self) -> float:
        """Branch coverage percentage."""
        if self.branches_total == 0:
            return 0.0
        return (self.branches_covered / self.branches_total) * 100

    @property
    def function_coverage(self) -> float:
        """Function coverage percentage."""
        if self.functions_total == 0:
            return 0.0
        return (self.functions_covered / self.functions_total) * 100


@dataclass
class CoverageReport:
    """Complete coverage report."""
    files: List[FileCoverage]

    @property
    def total_lines(self) -> int:
        return sum(f.lines_total for f in self.files)

    @property
    def total_lines_covered(self) -> int:
        return sum(f.lines_covered for f in self.files)

    @property
    def total_branches(self) -> int:
        return sum(f.branches_total for f in self.files)

    @property
    def total_branches_covered(self) -> int:
        return sum(f.branches_covered for f in self.files)

    @property
    def total_functions(self) -> int:
        return sum(f.functions_total for f in self.files)

    @property
    def total_functions_covered(self) -> int:
        return sum(f.functions_covered for f in self.files)

    @property
    def line_coverage(self) -> float:
        if self.total_lines == 0:
            return 0.0
        return (self.total_lines_covered / self.total_lines) * 100

    @property
    def branch_coverage(self) -> float:
        if self.total_branches == 0:
            return 0.0
        return (self.total_branches_covered / self.total_branches) * 100

    @property
    def function_coverage(self) -> float:
        if self.total_functions == 0:
            return 0.0
        return (self.total_functions_covered / self.total_functions) * 100

    def get_low_coverage_files(self, threshold: float = 50.0) -> List[FileCoverage]:
        """Get files with coverage below threshold."""
        return [f for f in self.files if f.line_coverage < threshold]


def parse_lcov(lcov_path: Path) -> CoverageReport:
    """Parse lcov format coverage report."""
    files = []
    current_file = None
    current_lines = []

    with open(lcov_path, 'r') as f:
        for line in f:
            line = line.strip()
            if line.startswith('SF:'):
                if current_file and current_lines:
                    files.append(current_file)
                current_file = FileCoverage(
                    path=line[3:],
                    lines_total=0,
                    lines_covered=0,
                    branches_total=0,
                    branches_covered=0,
                    functions_total=0,
                    functions_covered=0
                )
                current_lines = []
            elif line.startswith('LF:'):
                if current_file:
                    current_file.lines_total = int(line[3:])
            elif line.startswith('LH:'):
                if current_file:
                    current_file.lines_covered = int(line[3:])
            elif line.startswith('BRF:'):
                if current_file:
                    current_file.branches_total = int(line[4:])
            elif line.startswith('BRH:'):
                if current_file:
                    current_file.branches_covered = int(line[4:])
            elif line.startswith('FNF:'):
                if current_file:
                    current_file.functions_total = int(line[4:])
            elif line.startswith('FNH:'):
                if current_file:
                    current_file.functions_covered = int(line[4:])
            elif line == 'end_of_record':
                if current_file:
                    files.append(current_file)
                current_file = None

    return CoverageReport(files=files)


def parse_cobertura(xml_path: Path) -> CoverageReport:
    """Parse Cobertura XML format coverage report."""
    tree = ET.parse(xml_path)
    root = tree.getroot()

    files = []
    for class_elem in root.findall('.//class'):
        filename = class_elem.get('filename', 'unknown')
        lines_total = 0
        lines_covered = 0
        branches_total = 0
        branches_covered = 0
        functions_total = 0
        functions_covered = 0

        for line_elem in class_elem.findall('.//line'):
            lines_total += 1
            hits = int(line_elem.get('hits', 0))
            if hits > 0:
                lines_covered += 1

        file_cov = FileCoverage(
            path=filename,
            lines_total=lines_total,
            lines_covered=lines_covered,
            branches_total=branches_total,
            branches_covered=branches_covered,
            functions_total=functions_total,
            functions_covered=functions_covered
        )
        files.append(file_cov)

    return CoverageReport(files=files)


def parse_json(json_path: Path) -> CoverageReport:
    """Parse JSON format coverage report."""
    with open(json_path, 'r') as f:
        data = json.load(f)

    files = []
    # Adjust based on actual JSON structure from tarpaulin/grcov
    if 'files' in data:
        for file_data in data['files']:
            file_cov = FileCoverage(
                path=file_data.get('file', 'unknown'),
                lines_total=file_data.get('lines_total', 0),
                lines_covered=file_data.get('lines_covered', 0),
                branches_total=file_data.get('branches_total', 0),
                branches_covered=file_data.get('branches_covered', 0),
                functions_total=file_data.get('functions_total', 0),
                functions_covered=file_data.get('functions_covered', 0)
            )
            files.append(file_cov)

    return CoverageReport(files=files)


def print_summary(report: CoverageReport, threshold: float = 80.0):
    """Print coverage summary."""
    print("\n=== Coverage Summary ===")
    print(f"Files analyzed: {len(report.files)}")
    print(f"Line coverage: {report.line_coverage:.2f}% ({report.total_lines_covered}/{report.total_lines})")
    print(f"Branch coverage: {report.branch_coverage:.2f}% ({report.total_branches_covered}/{report.total_branches})")
    print(f"Function coverage: {report.function_coverage:.2f}% ({report.total_functions_covered}/{report.total_functions})")

    if report.line_coverage < threshold:
        print(f"\n⚠ WARNING: Coverage ({report.line_coverage:.2f}%) is below threshold ({threshold}%)")
    else:
        print(f"\n✓ Coverage meets threshold ({threshold}%)")


def print_low_coverage_files(report: CoverageReport, threshold: float = 50.0):
    """Print files with low coverage."""
    low_cov = report.get_low_coverage_files(threshold)
    if low_cov:
        print(f"\n=== Files with coverage < {threshold}% ===")
        for file_cov in sorted(low_cov, key=lambda f: f.line_coverage):
            print(f"{file_cov.line_coverage:.2f}% - {file_cov.path}")
    else:
        print(f"\n✓ All files have coverage >= {threshold}%")


def main():
    parser = argparse.ArgumentParser(description='Parse and analyze coverage reports')
    parser.add_argument('report', type=Path, help='Coverage report file path')
    parser.add_argument('--format', choices=['lcov', 'cobertura', 'json', 'auto'],
                        default='auto', help='Report format (default: auto-detect)')
    parser.add_argument('--threshold', type=float, default=80.0,
                        help='Coverage threshold percentage (default: 80)')
    parser.add_argument('--low-threshold', type=float, default=50.0,
                        help='Low coverage threshold (default: 50)')
    parser.add_argument('--output', type=Path, help='Output analysis to JSON file')

    args = parser.parse_args()

    # Auto-detect format
    if args.format == 'auto':
        suffix = args.report.suffix.lower()
        if suffix == '.lcov' or args.report.name == 'lcov.info':
            args.format = 'lcov'
        elif suffix == '.xml':
            args.format = 'cobertura'
        elif suffix == '.json':
            args.format = 'json'
        else:
            print(f"Cannot auto-detect format from {args.report.suffix}, assuming lcov")
            args.format = 'lcov'

    # Parse report
    if args.format == 'lcov':
        report = parse_lcov(args.report)
    elif args.format == 'cobertura':
        report = parse_cobertura(args.report)
    elif args.format == 'json':
        report = parse_json(args.report)
    else:
        parser.error(f"Unsupported format: {args.format}")

    # Print summary
    print_summary(report, args.threshold)
    print_low_coverage_files(report, args.low_threshold)

    # Output to JSON if requested
    if args.output:
        output_data = {
            'summary': {
                'line_coverage': report.line_coverage,
                'branch_coverage': report.branch_coverage,
                'function_coverage': report.function_coverage,
                'files_count': len(report.files)
            },
            'files': [
                {
                    'path': f.path,
                    'line_coverage': f.line_coverage,
                    'branch_coverage': f.branch_coverage,
                    'function_coverage': f.function_coverage
                }
                for f in report.files
            ]
        }
        with open(args.output, 'w') as f:
            json.dump(output_data, f, indent=2)
        print(f"\nAnalysis saved to {args.output}")


if __name__ == '__main__':
    main()
