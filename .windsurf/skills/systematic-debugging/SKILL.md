---
name: systematic-debugging
description: 4-phase systematic debugging methodology with root cause analysis and evidence-based verification. Use when debugging complex issues.
allowed-tools: Read, Glob, Grep
---

# Systematic Debugging

> Source: obra/superpowers

## When to Use
- Debugging complex issues that resist quick fixes
- Investigating production bugs with limited information
- Fixing intermittent or flaky failures
- Performing root cause analysis on problems
- Learning systematic debugging methodology
- When random guessing hasn't worked

## Overview
This skill provides a structured approach to debugging that prevents random guessing and ensures problems are properly understood before solving.

## 4-Phase Methodology

### Phase 1: Reproduce
- **Goal**: Create a minimal, reproducible test case
- **Actions**:
  - Write a failing test that captures the bug
  - Identify the exact conditions that trigger the issue
  - Isolate the problem from unrelated code
  - Document the reproduction steps

### Phase 2: Isolate
- **Goal**: Narrow down the location of the bug
- **Actions**:
  - Use binary search (comment out code, add back gradually)
  - Add strategic logging/print statements
  - Use debugger breakpoints
  - Check recent changes

### Phase 3: Understand (Root Cause Analysis)
- **Goal**: Understand WHY the bug occurs
- **Actions**:
  - Apply the "5 Whys" technique
  - Trace execution flow
  - Check assumptions about data/state
  - Review documentation/API specs

### Phase 4: Fix & Verify
Fix and verify it's truly fixed.

```markdown
## Fix Verification
- [ ] Bug no longer reproduces
- [ ] Related functionality still works
- [ ] No new issues introduced
- [ ] Test added to prevent regression
```

## Debugging Checklist

```markdown
## Before Starting
- [ ] Can reproduce consistently
- [ ] Have minimal reproduction case
- [ ] Understand expected behavior

## During Investigation
- [ ] Check recent changes (git log)
- [ ] Check logs for errors
- [ ] Add logging if needed
- [ ] Use debugger/breakpoints

## After Fix
- [ ] Root cause documented
- [ ] Fix verified
- [ ] Regression test added
- [ ] Similar code checked
```

## Common Debugging Commands

```bash
# Recent changes
git log --oneline -20
git diff HEAD~5

# Search for pattern
grep -r "errorPattern" --include="*.ts"

# Check logs
pm2 logs app-name --err --lines 100
```

## Anti-Patterns

❌ **Random changes** - "Maybe if I change this..."
❌ **Ignoring evidence** - "That can't be the cause"
❌ **Assuming** - "It must be X" without proof
❌ **Not reproducing first** - Fixing blindly
❌ **Stopping at symptoms** - Not finding root cause

---

## Common Pitfalls
- Random changes without understanding the problem
- Ignoring evidence that doesn't fit assumptions
- Making assumptions without proof
- Not reproducing the issue before fixing
- Stopping at symptoms instead of finding root cause
- Not adding tests to prevent regression
- Over-complicating the fix

## Best Practices
- Always reproduce the issue first
- Use binary search to isolate the problem location
- Apply "5 Whys" technique for root cause analysis
- Add regression tests after fixing
- Document the root cause and fix
- Check similar code for the same issue
- Use logging strategically, not excessively
