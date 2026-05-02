---
description: Debugging command. Activates DEBUG mode for systematic problem investigation.
---

# /debug - Systematic Problem Investigation

$ARGUMENTS

---

## Purpose

This command activates DEBUG mode for systematic investigation of issues, errors, or unexpected behavior.

---

## Behavior

When `/debug` is triggered:

1. **Gather information**
   - Error message
   - Reproduction steps
   - Expected vs actual behavior
   - Recent changes

2. **Form hypotheses**
   - List possible causes
   - Order by likelihood

3. **Investigate systematically**
   - Test each hypothesis
   - Check logs, data flow
   - Use elimination method

4. **Fix and prevent**
   - Apply fix
   - Explain root cause
   - Add prevention measures

---

## Output Format

```markdown
## 🔍 Debug: [Issue]

### 1. Symptom
[What's happening]

### 2. Information Gathered
- Error: `[error message]`
- File: `[filepath]`
- Line: [line number]

### 3. Hypotheses
1. ❓ [Most likely cause]
2. ❓ [Second possibility]
3. ❓ [Less likely cause]

### 4. Investigation

**Testing hypothesis 1:**
[What I checked] → [Result]

**Testing hypothesis 2:**
[What I checked] → [Result]

### 5. Root Cause
🎯 **[Explanation of why this happened]**

### 6. Fix
```[language]
// Before
[broken code]

// After
[fixed code]
```

### 7. Prevention
🛡️ [How to prevent this in the future]
```

---

## Examples

```
/debug login not working
/debug API returns 500
/debug form doesn't submit
/debug data not saving
```

---

## Key Principles

- **Ask before assuming** - get full error context
- **Test hypotheses** - don't guess randomly
- **Explain why** - not just what to fix
- **Prevent recurrence** - add tests, validation

---

## When to Use
- Investigating bugs or unexpected behavior
- Debugging compilation errors or runtime failures
- Analyzing performance issues
- Troubleshooting integration problems
- Understanding why code isn't working as expected

## When NOT to Use
- Simple syntax errors with clear error messages
- Well-documented issues with known solutions
- Tasks requiring feature implementation (use /enhance instead)
- Non-technical problems or user education
- Production emergencies requiring immediate hotfix

## Edge Case Handling
- **Missing error details**: When error message is incomplete, ask for full stack trace, reproduction steps, and environment details
- **Intermittent bugs**: For flaky issues, add logging, reproduce multiple times, check timing/race conditions
- **Environment-specific**: Debug locally vs. staging vs. production differences (OS, versions, dependencies)
- **Multiple error sources**: When several errors appear, prioritize by impact and fix sequentially
- **Silent failures**: Add logging to identify where execution stops without explicit error

## Failure Modes
- **Insufficient information**: Cannot debug without error context - ask for logs, screenshots, reproduction steps
- **Wrong hypothesis**: Testing incorrect root cause leads to wasted time - use elimination method systematically
- **Fix without understanding**: Patching symptoms without addressing root cause leads to recurrence
- **Breaking changes**: Fix introduces new bugs - run tests after each fix
- **Incomplete fix**: Resolves immediate issue but misses edge cases - add regression tests

## Performance Considerations
- Debugging time: Set timeout for investigation (e.g., 30 minutes) before escalating
- Log analysis: Use grep/filter to focus on relevant log entries, avoid information overload
- Reproduction speed: Automate reproduction when possible to speed up iteration
- Tool selection: Use appropriate debugging tools (debugger, logging, profiling) based on issue type

## Security Notes
- **Sensitive data**: Never log or expose passwords, tokens, PII in debug output
- **Error messages**: Sanitize error messages before displaying to users (hide stack traces in production)
- **Debug access**: Restrict debug endpoints and tools in production environments
- **Log retention**: Securely store and rotate debug logs containing sensitive information
- **Information disclosure**: Avoid revealing system internals in error messages that could aid attackers

## Guardrails
- Always gather full error context before proposing fixes
- Test hypotheses systematically, don't guess
- Explain root cause, not just the fix
- Add prevention measures (tests, validation)
- Verify fix actually resolves the issue
