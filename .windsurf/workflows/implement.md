---
description: Systematically implement new features using Planning With Files and team registration
---

# Implement Feature

A systematic workflow for implementing new features using the Planning With Files pattern, team registration, and iterative development with validation.

## Overview

This workflow combines several powerful patterns:

1. **Planning With Files**: Persistent plan.md that survives context compaction
2. **Team Registration**: Every team gets a permanent ID and log file
3. **Single Source of Truth**: All planning happens in designated locations
4. **Iterative Development**: Build, test, and refine in cycles
5. **Regression Protection**: Baseline tests before and after changes

## When to Use
- Implementing new features from scratch
- Adding new capabilities to existing projects
- Multi-file changes requiring coordination
- Features that need systematic planning and tracking
- Any implementation where the scope is >20 LOC or >1 file

## When NOT to Use
- Simple bug fixes (use review-fix-loop instead)
- Refactoring existing code (use refactor-fix-loop instead)
- Trivial one-off changes (<20 LOC, single file)
- Documentation-only changes (edit directly)

## Pre-flight Checks

```powershell
# Build command - adapt to your project's build system
# Examples: cargo build, npm run build, python -m build, mvn compile
<build-command>
if ($LASTEXITCODE -ne 0) {
    Write-Error "Project must build before starting feature implementation"
    exit 1
}

# Test command - adapt to your project's test framework
# Examples: cargo test, npm test, pytest, mvn test
<test-command>
if ($LASTEXITCODE -ne 0) {
    Write-Error "Tests must pass before starting feature implementation"
    exit 1
}

$existingTeams = Get-ChildItem .teams -Filter "TEAM_*.md" -ErrorAction SilentlyContinue
if ($existingTeams) {
    $highestTeamNum = $existingTeams | ForEach-Object {
        [int]($_.Name -replace "TEAM_(\d+)_.*", '$1')
    } | Measure-Object -Maximum | Select-Object -ExpandProperty Maximum
    $nextTeamNum = $highestTeamNum + 1
} else {
    $nextTeamNum = 1
}
Write-Host "Next team number: $nextTeamNum"
```

## Phase 1: Team Registration

```powershell
$TEAM_NUM = $nextTeamNum
$TEAM_FILE = ".teams/TEAM_${TEAM_NUM}_feature_summary.md"

$teamContent = @"
# TEAM_${TEAM_NUM} - Feature Implementation

**Started:** $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
**Status:** In Progress

## Feature Description
[Description]

## Team Members
- Cascade (AI Assistant)
- USER (Human)

## Progress Log
- $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') - Team created

## Decisions Made
[Architectural decisions]

## Questions Raised
[Questions]

## Handoff Notes
[Notes]
"@

$teamContent | Out-File -FilePath $TEAM_FILE -Encoding UTF8
Write-Host "✓ Created team file: $TEAM_FILE"
```

## Phase 2: Planning With Files

**Note:** For systematic planning with team registration, use `/plan` workflow instead. This workflow is for implementation after planning is complete.

```powershell
$PLAN_DIR = ".teams/TEAM_${TEAM_NUM}"
New-Item -ItemType Directory -Force -Path $PLAN_DIR | Out-Null
$PLAN_FILE = "$PLAN_DIR/plan.md"

$planContent = @"
# Feature Implementation Plan - TEAM_${TEAM_NUM}

## Feature Overview
[Description]

## Requirements
- [Requirement 1]
- [Requirement 2]

## Design Decisions
- [Decision 1]
- [Decision 2]

## Implementation Tasks
1. [Task 1]
2. [Task 2]

## Testing Strategy
- [Test approach]

## Dependencies
- [External]
- [Internal]

## Success Criteria
- [Criterion 1]
- [Criterion 2]
"@

$planContent | Out-File -FilePath $PLAN_FILE -Encoding UTF8
Write-Host "✓ Created plan file: $PLAN_FILE"
```

## Phase 3: Implementation Loop

```powershell
$MAX_ITERS = 15
$KILLSWITCH = "$env:USERPROFILE\.implement-feature-stop"
$LOGDIR = ".implement-feature-logs/TEAM_${TEAM_NUM}/$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null

for ($i = 1; $i -le $MAX_ITERS; $i++) {
    Write-Host "── Iteration $i/$MAX_ITERS ──"

    if (Test-Path $KILLSWITCH) {
        Write-Host "Killswitch tripped — bailing."
        Remove-Item $KILLSWITCH
        exit 2
    }

    $logFile = "$LOGDIR/build-$i.log"
    cargo check *>&1 | Tee-Object -FilePath $logFile

    $testLogFile = "$LOGDIR/test-$i.log"
    cargo test *>&1 | Tee-Object -FilePath $testLogFile

    if ($LASTEXITCODE -eq 0) { break }

    $progressEntry = "- $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') - Iteration $i completed"
    Add-Content -Path $TEAM_FILE -Value $progressEntry
}
```

## Phase 4: Regression Protection

```powershell
# Baseline test command - adapt to your project
$BASELINE_TEST_CMD = "<your-integration-test-command>"

$baselineLog = "$LOGDIR/baseline-before.log"
Invoke-Expression $BASELINE_TEST_CMD *>&1 | Tee-Object -FilePath $baselineLog

$baselineAfterLog = "$LOGDIR/baseline-after.log"
Invoke-Expression $BASELINE_TEST_CMD *>&1 | Tee-Object -FilePath $baselineAfterLog

# Compare results - if different, this is a regression → fix it
```

## Guardrails (Non-Negotiable)

1. Team registration required - Every feature must have a TEAM_XXX file
2. Killswitch file - Create `~/.implement-feature-stop` to stop
3. Iteration cap - Default 15, reassess if not complete
4. Baseline tests - Run before and after behavior-critical changes
5. Plan persistence - plan.md must survive context compaction
6. Code comments - Add `// TEAM_XXX: Reason` to all modified code
7. No dead code - Remove unused functions, imports, commented code
8. Ask questions early - Create .questions/TEAM_XXX_* for ambiguous decisions
9. Update TODO.md - Track incomplete work globally
10. Single Source of Truth - All planning in plan.md

## Phase 5: Completion Checklist

Before marking the feature complete, ensure:

- [ ] Project builds cleanly (`cargo build --release`)
- [ ] All tests pass (`cargo test`)
- [ ] Integration tests pass (`cargo test --test integration_test`)
- [ ] Baseline regression tests pass (if applicable)
- [ ] Team file updated with final progress
- [ ] Plan.md reflects actual implementation
- [ ] All modified code has `// TEAM_XXX:` comments
- [ ] No dead code left behind
- [ ] TODO.md updated with any remaining work
- [ ] Handoff notes written in team file
- [ ] Questions resolved or documented

## Project-Specific Notes

Replace this section with project-specific guidance relevant to your project:
- External library integrations and compatibility requirements
- Platform-specific APIs and error handling patterns
- Resource management and cleanup requirements
- Performance constraints and limits
- Concurrency patterns and threading considerations
- Error handling conventions (library vs application-level)
- Configuration schema updates if adding new options
- IPC or inter-service communication protocols
- Testing frameworks and platform-specific test considerations

---

## Edge Case Handling
- **Ambiguous requirements**: When feature description is unclear, ask clarifying questions before planning
- **Scope creep**: Feature grows during implementation - define clear boundaries and change process
- **Dependency conflicts**: New dependencies break existing code - validate in isolation first
- **Team number conflicts**: Multiple teams working simultaneously - use unique team numbers and coordinate
- **Plan drift**: Implementation diverges from plan.md - update plan.md to reflect actual implementation
- **Baseline test failures**: Tests don't pass before starting - fix tests or document known issues
- **Context loss**: Long implementation exceeds context window - rely on persistent plan.md and team file

## Failure Modes
- **Incomplete implementation**: Feature partially implemented but marked complete - verify against checklist
- **Broken build**: Code doesn't compile - stop and fix before proceeding
- **Test failures**: New code breaks existing tests - fix before marking complete
- **Regression bugs**: Changes break existing functionality - run baseline tests before and after
- **Dead code left behind**: Unused code not removed - violates Rule 6, must clean up
- **Missing documentation**: Changes not documented in team file - update team file continuously
- **Handoff incomplete**: Next team cannot understand changes - write comprehensive handoff notes

## Performance Considerations
- **Implementation velocity**: Balance speed with quality - don't rush at expense of correctness
- **Build time impact**: New dependencies may slow builds - measure before and after
- **Test execution time**: New tests increase suite duration - optimize or categorize as integration tests
- **Memory usage**: New features may increase memory footprint - profile for RAM disk impact
- **I/O patterns**: File operations affect RAM disk performance - consider junction/symlink overhead
- **Async overhead**: Tokio async patterns add complexity - ensure proper async/await usage
- **Resource cleanup**: Ensure RAM disks and processes are cleaned up to prevent leaks

## Security Notes
- **Unsafe code review**: All unsafe blocks must be reviewed for memory safety and correctness
- **Windows API security**: Win32 API calls must handle HRESULT errors properly and validate parameters
- **Input validation**: New features must validate all inputs, especially from external sources
- **Secrets scanning**: Ensure no secrets are introduced (use secret-scrubber skill)
- **Resource isolation**: Ensure RAM disk isolation between projects using hash-based subdirectories
- **Process security**: aim_ll.exe runs as user process - verify no privilege escalation
- **Error disclosure**: Error messages should not leak sensitive information (paths, internal details)

## Guardrails
- Always register a team before starting implementation
- Follow Planning With Files pattern with persistent plan.md
- Run baseline tests before making changes
- Add TEAM_XXX comments to all modified code
- Update team file with progress and decisions
- Ensure project builds and tests pass before finishing
- Document remaining work in TODO.md
- Write handoff notes before completion

## Example Usage

```powershell
# Build and test commands - adapt to your project
<build-command>
<test-command>
# Team registration and planning automated by workflow
# Implementation loop handles iterative development
<build-command> && <test-command> && <integration-test-command>
```

## Related Skills

- Replace with project-relevant skills from `.windsurf/skills/`
- Examples: error-handling, testing-strategies, architecture, database-design
- Reference skills that match your project's technology stack

## Related Workflows

- `code-fix-loop.md` - For refactoring and fixing code
- `suggest.md` - For brainstorming features

## Credits

Planning With Files pattern, Universal AI Team Rulebook, Ralph Safe Loop
