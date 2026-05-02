# FastRamdisk Scripts

This directory contains utility scripts for building, testing, and maintaining the FastRamdisk project.

## PowerShell Scripts

### coverage.ps1

Run code coverage analysis for the FastRamdisk project.

```powershell
# Run coverage with tarpaulin (default)
.\scripts\coverage.ps1

# Run with grcov
.\scripts\coverage.ps1 -Tool grcov

# Generate all report formats
.\scripts\coverage.ps1 -OutputFormat all

# Set custom coverage threshold
.\scripts\coverage.ps1 -Threshold 90

# Clean previous coverage data
.\scripts\coverage.ps1 -Clean
```

### build.ps1

Build automation for FastRamdisk.

```powershell
# Build debug (default)
.\scripts\build.ps1

# Build release
.\scripts\build.ps1 -Release

# Build with crando wrapper
.\scripts\build.ps1 -WithWrapper

# Clean before building
.\scripts\build.ps1 -Clean

# Verify build after completion
.\scripts\build.ps1 -Verify
```

### test.ps1

Test automation for FastRamdisk.

```powershell
# Run all tests
.\scripts\test.ps1

# Run only unit tests
.\scripts\test.ps1 -Unit

# Run only integration tests
.\scripts\test.ps1 -Integration

# Run with verbose output
.\scripts\test.ps1 -Verbose

# Filter tests by pattern
.\scripts\test.ps1 -TestFilter "ramdisk"
```

### clean.ps1

Clean build artifacts and temporary files.

```powershell
# Clean target directory
.\scripts\clean.ps1 -Target

# Clean coverage reports
.\scripts\clean.ps1 -Coverage

# Clean temporary files
.\scripts\clean.ps1 -Temp

# Clean everything
.\scripts\clean.ps1 -All

# Dry run (show what would be deleted)
.\scripts\clean.ps1 -All -DryRun
```

### deps.ps1

Dependency management for FastRamdisk.

```powershell
# Check all dependencies
.\scripts\deps.ps1

# Install missing dependencies
.\scripts\deps.ps1 -Install

# Update all dependencies
.\scripts\deps.ps1 -Update

# Check only Rust toolchain
.\scripts\deps.ps1 -Rust

# Check only WinFsp
.\scripts\deps.ps1 -WinFsp
```

## Python Scripts

### coverage_parser.py

Parse and analyze coverage reports (lcov, cobertura, json).

```bash
# Auto-detect format and analyze
python scripts/coverage_parser.py coverage/reports/lcov.info

# Specify format explicitly
python scripts/coverage_parser.py coverage/reports/report.xml --format cobertura

# Set custom threshold
python scripts/coverage_parser.py coverage/reports/lcov.info --threshold 90

# Output analysis to JSON
python scripts/coverage_parser.py coverage/reports/lcov.info --output analysis.json
```

### benchmark_analyzer.py

Analyze benchmark results and compare performance.

```bash
# Analyze benchmark output from stdin
cargo bench | python scripts/benchmark_analyzer.py --stdin

# Analyze from JSON file
python scripts/benchmark_analyzer.py --input benchmark_results.json

# Compare with previous results
python scripts/benchmark_analyzer.py --input new_results.json --compare old_results.json

# Set significance threshold
python scripts/benchmark_analyzer.py --input results.json --threshold 5
```

### config_validator.py

Validate crando.toml configuration files.

```bash
# Validate configuration
python scripts/config_validator.py crando.toml

# Print configuration summary
python scripts/config_validator.py crando.toml --summary

# Treat warnings as errors
python scripts/config_validator.py crando.toml --strict
```

### test_runner.py

Enhanced test runner with filtering and reporting.

```bash
# Run all tests
python scripts/test_runner.py --lib

# Filter tests by pattern
python scripts/test_runner.py --lib --filter "ramdisk"

# Run specific integration test
python scripts/test_runner.py --test integration_test

# Verbose output
python scripts/test_runner.py --lib --verbose

# Save results to JSON
python scripts/test_runner.py --lib --output test_results.json
```

## Usage Notes

- All PowerShell scripts should be run from the project root directory
- Python scripts require Python 3.8+
- Some Python scripts may require additional dependencies (e.g., tomli for config_validator.py)
- Install Python dependencies: `pip install tomli`

## For Agents

These scripts are designed to be used by AI agents for:

- Automated build and test workflows
- Coverage analysis and reporting
- Benchmark comparison and trend analysis
- Configuration validation
- Dependency management and installation
- Project cleanup and maintenance
