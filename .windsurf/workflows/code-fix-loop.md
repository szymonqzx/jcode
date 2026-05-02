---
description: Review, refactor, and iteratively fix code until all checks pass
---

# Code Fix Loop

A comprehensive workflow that combines code review, refactoring, and automated fixing with reflection loops to systematically identify and resolve issues in the codebase.

## Overview

This workflow merges two powerful patterns:

1. **Review Mode**: Thorough analysis for bugs, edge cases, security issues, and code quality
2. **Refactor Mode**: Systematic code refactoring with quality improvements
3. **Automated Loop**: Iterate until build, lint, and tests all pass
4. **Guardrails**: Killswitch, iteration caps, backup recommendations, and comprehensive logging

## Sub-commands

```
/code-fix review    - Review code for bugs/quality, then fix
/code-fix refactor  - Refactor code and fix issues
```

---

## Review Mode

### When to Use
- Systematic code quality improvement across multiple files
- Fixing 10+ failing tests with clear error messages
- Resolving compile errors in a large refactor
- Making a linter pass (clippy, ESLint, ruff)
- Closing out a type-check sweep
- Addressing security vulnerabilities or resource leaks
- Removing dead code and unused functions
- Improving error handling and adding proper context
- Cleaning up silent error swallowing (unwrap_or, ignore patterns)
- Any task where the checkable outcome is well-defined and machine-verifiable

### When NOT to Use
- UI flows that need human judgment
- Feature implementation from scratch (use Planning With Files instead)
- Anything involving money, destructive data operations, or production infrastructure
- Tasks where passing depends on subjective quality
- On production systems or critical infrastructure

### Review Focus Areas

**Bugs and Logic Errors:**
- Logic errors and incorrect behavior
- Edge cases that aren't handled
- Null/undefined reference issues
- Race conditions or concurrency issues
- Security vulnerabilities
- Improper resource management or resource leaks
- API contract violations
- Incorrect caching behavior (cache staleness, cache key bugs, invalidation issues)
- Violations of existing code patterns or conventions

**Code Quality Issues:**
- Dead code - unused functions, unused imports, commented-out code
- Silent error swallowing - unwrap_or, ignore patterns without logging
- Poor error handling - missing error context, generic error types
- Inconsistent error patterns - mixing Result types, inconsistent anyhow/thiserror usage
- Missing or incomplete documentation - undocumented public APIs
- Code duplication - repeated logic that should be extracted
- Type safety issues - unnecessary string parsing, weak typing
- Resource cleanup issues - missing drop guards, improper cleanup ordering

### Review Guidelines
- Explore the codebase efficiently using parallel tool calls
- Report both new issues and pre-existing bugs
- Only report high-confidence issues (no speculative findings)
- Base conclusions on complete understanding of the codebase

---

## Refactor Mode

### When to Use
- Systematic code refactoring across multiple files
- Resolving clippy warnings and linting issues
- Fixing compilation errors during refactoring
- Improving code organization and structure
- Any refactoring where the outcome is machine-verifiable

### When NOT to Use
- New feature implementation (use Planning With Files instead)
- UI flows requiring human judgment
- Production infrastructure changes
- Tasks where correct depends on subjective quality

### Refactoring Focus Areas
1. Clippy warnings - Address all linting suggestions systematically
2. Compilation errors - Fix type mismatches, missing imports, etc.
3. Code organization - Improve module structure and file organization
4. Error handling - Ensure proper error propagation and context
5. Resource management - Verify cleanup and RAII patterns
6. Documentation - Add missing docs and clarify complex logic
7. Performance - Address obvious performance issues

---

## Pre-flight Safety Checks

```powershell
# Optional: Create backup before running
$BACKUP_DIR = ".code-fix-backup/$(Get-Date -Format 'yyyyMMddHHmmss')"
Copy-Item -Recurse -Force . $BACKUP_DIR

# Baseline: ensure tests pass before starting - adapt to your project
<test-command>
if ($LASTEXITCODE -ne 0) {
    Write-Error "Tests must pass before starting code fix loop"
    exit 1
}
```

---

## Loop Configuration

```powershell
$MAX_ITERS = 20
$COST_CAP_USD = 5.00
$KILLSWITCH = "$env:USERPROFILE\.code-fix-stop"
$LOGDIR = ".code-fix-logs/$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null

# Check command - adapt to your project's build, lint, and test commands
# Examples:
# Rust: cargo check && cargo clippy -- -D warnings && cargo test
# Node: npm run build && npm run lint && npm test
# Python: python -m py_compile src && flake8 src && pytest
$CHECK_CMD = "<your-build-command> && <your-lint-command> && <your-test-command>"
```

---

## The Loop

```powershell
for ($i = 1; $i -le $MAX_ITERS; $i++) {
    Write-Host "── Iteration $i/$MAX_ITERS ──"

    # 1. Killswitch check
    if (Test-Path $KILLSWITCH) {
        Write-Host "Killswitch tripped — bailing."
        Remove-Item $KILLSWITCH
        exit 2
    }

    # 2. Format code first (refactor mode only) - adapt to your project
    if ($MODE -eq "refactor") {
        <format-command>
    }

    # 3. Check: are we already green?
    $logFile = "$LOGDIR/check-$i.log"
    $checkResult = Invoke-Expression $CHECK_CMD *>&1 | Tee-Object -FilePath $logFile
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Green at iteration $i"
        exit 0
    }

    # 4. Not green — analyze and fix
    $failure = Get-Content $logFile -Tail 50

    # Hand the failure to Cascade for fixes
    # Include review context if in review mode
}
```

---

## Guardrails (Non-Negotiable)

1. Killswitch file - Create `~/.code-fix-stop` from any terminal to stop the loop
2. Iteration cap - Default 20. If it hasn't gone green by then, something is structurally wrong
3. Cost cap - Default $5. Sanity bound on runaway spend (if using paid AI services)
4. Backup recommended - Consider creating a backup before running if the project is important
5. Never weakens tests - Explicitly forbid modifying test assertions unless tests are wrong
6. Log everything - Every iteration's check output and AI response goes to `.code-fix-logs/<timestamp>/`
7. Baseline required - Tests must pass before starting (review mode)

---

## After It Goes Green

### Review Mode
1. Review the changes - Manually inspect modified files. Understand what was changed
2. Re-run the check manually - Sometimes caches lie
3. Check for suspicious weakenings - Search for skip/todo/xtest/only patterns in changed files
4. Verify review findings - Ensure issues identified are actually fixed
5. Commit or backup - If using git, commit the changes. Otherwise, create a backup

### Refactor Mode
1. Review the changes - Manually inspect modified files. Understand what was changed
2. Re-run the check manually - Sometimes caches lie
3. Run integration tests - adapt to your project: <integration-test-command>
4. Check for suspicious weakenings - Search for skip/todo/xtest/only patterns
5. Verify no regressions - Ensure all original functionality still works
6. Commit or backup - If using git, commit the changes. Otherwise, create a backup

---

## Project-Specific Notes

Replace this section with project-specific guidance relevant to your project:
- Platform-specific APIs and their correct usage
- External library integrations and compatibility requirements
- Resource management and cleanup requirements
- Concurrency patterns and threading considerations
- Performance constraints and limits
- Error handling conventions (library vs application-level)
- Security considerations specific to your domain
- Testing frameworks and platform-specific test considerations

---

## Edge Case Handling
- **Circular dependencies**: Fixing one file breaks another - identify dependency chains and fix in correct order
- **Test flakiness**: Intermittent test failures mask real issues - run tests multiple times to confirm stability
- **Multiple error types**: Compiler, linter, and test errors together - prioritize by severity (compiler > linter > tests)
- **Silent failures**: Code passes checks but still has bugs - add targeted tests for suspected issues
- **Large refactor scope**: Too many files to fix at once - break into smaller batches with intermediate verification
- **Unsafe code blocks**: Windows API or raw pointer code requires extra caution - review changes thoroughly
- **Resource leaks**: Fixed code might introduce new leaks - verify RAII patterns and cleanup paths

## Failure Modes
- **Infinite loop**: Check never goes green due to fundamental issue - stop at iteration cap and escalate
- **Weakened tests**: Fixing code by modifying test assertions instead of fixing logic - explicitly forbidden
- **New bugs introduced**: Fix resolves one issue but creates others - run full test suite after each batch
- **Incomplete fixes**: Addressing symptoms instead of root cause - ensure fix addresses underlying problem
- **Wrong guardrail**: Killswitch or iteration cap prevents completion - adjust parameters if justified
- **Cache issues**: Stale build cache causes false failures - clear cache and retry before escalating
- **Context loss**: Long-running loop exceeds context window - use persistent logs and checkpoint files

## Performance Considerations
- **Iteration speed**: Each iteration should complete within 30-60 seconds for most projects
- **Parallel checks**: Run multiple check commands in parallel when independent (e.g., build + lint)
- **Incremental builds**: Use your build system's incremental compilation to speed up repeated checks
- **Cache strategy**: Leverage build cache aggressively but clear when suspected stale
- **Target selection**: For large projects, focus on specific modules/packages rather than full workspace
- **Test parallelization**: Use parallel test flags appropriate for your framework
- **Log size**: Keep logs concise to avoid context bloat, summarize results instead of raw output

## Security Notes
- **Unsafe code review**: Review any unsafe/low-level code for memory safety and correctness
- **Secrets scanning**: Ensure no secrets are introduced during fixes (use secret-scrubber skill if available)
- **Input validation**: Fixed code should properly validate all inputs, especially from external sources
- **Error disclosure**: Error messages should not leak sensitive information (paths, internal details)
- **API security**: Platform-specific API calls must handle errors properly and validate parameters
- **Resource cleanup**: Ensure all handles, file descriptors, and resources are properly closed
- **Privilege checks**: Verify code doesn't inadvertently require elevated privileges

## Guardrails
- Killswitch file - Create `~/.workflow-stop` to stop
- Iteration cap - Default 15 iterations
- Backup before changes - Always create backup before refactoring
- Verify checks pass - Ensure check command passes before finishing
- Log everything - Results go to `.workflow-logs/<timestamp>/`
- Manual review required - Code quality is subjective
- Preserve functionality - Don't break existing behavior

---

## Example Usage

```powershell
# Review mode with default checks
/code-fix review

# Refactor mode with default checks
/code-fix refactor

# Custom check command
$CHECK_CMD = "cargo test --test integration_test" ./code-fix-loop.ps1

# Multi-check scenario
$CHECK_CMD = "cargo check && cargo clippy && cargo test" ./code-fix-loop.ps1
```

---

## Credits

Combines concepts from:

- Ralph Safe Loop by Steve Kinney
- Reflection Loop pattern from HumanEval research
- Code Review best practices for systematic quality improvement
- Rust refactoring best practices

---

## Related Skills

- Replace with project-relevant skills from `.windsurf/skills/`
- Examples: error-handling, testing-strategies, architecture
- Reference skills that match your project's technology stack
