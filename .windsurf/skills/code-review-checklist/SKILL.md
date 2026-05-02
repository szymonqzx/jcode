---
name: code-review-checklist
description: Code review guidelines covering code quality, security, and best practices.
allowed-tools: Read, Glob, Grep
---

# Code Review Checklist

## When to Use
- Reviewing code for correctness, security, performance, quality, testing, and documentation
- Conducting formal code reviews before merge
- Checking AI-generated code for common issues
- Reviewing pull requests
- Auditing codebases for anti-patterns
- Providing structured feedback on code changes

## Review Categories

### Correctness
- Logic errors and edge cases
- Off-by-one errors
- Null/undefined handling
- Type safety violations
- Incorrect algorithm implementation
- Code does what it's supposed to do
- Edge cases handled
- Error handling in place
- No obvious bugs

### Security
- SQL injection vulnerabilities
- XSS vulnerabilities
- Authentication/authorization flaws
- Sensitive data exposure
- Dependency vulnerabilities
- Input validated and sanitized
- No SQL/NoSQL injection vulnerabilities
- No XSS or CSRF vulnerabilities
- No hardcoded secrets or sensitive credentials
- **AI-Specific:** Protection against Prompt Injection (if applicable)
- **AI-Specific:** Outputs are sanitized before being used in critical sinks

### Performance
- Inefficient algorithms
- Unnecessary computations
- Memory leaks
- N+1 queries
- Blocking operations in async code
- No N+1 queries
- No unnecessary loops
- Appropriate caching
- Bundle size impact considered

### Code Quality
- Naming conventions
- Code duplication
- Function/method length
- Cyclomatic complexity
- Dead code
- Clear naming
- DRY - no duplicate code
- SOLID principles followed
- Appropriate abstraction level

### Testing
- Missing test coverage
- Test quality issues
- Flaky tests
- Missing edge case tests
- Test isolation problems
- Unit tests for new code
- Edge cases tested
- Tests readable and maintainable

### Documentation
- Missing or outdated comments
- Unclear API documentation
- Missing README
- Inconsistent documentation
- Missing examples
- Complex logic commented
- Public APIs documented
- README updated if needed

## AI & LLM Review Patterns (2025)

### Logic & Hallucinations
- [ ] **Chain of Thought:** Does the logic follow a verifiable path?
- [ ] **Edge Cases:** Did the AI account for empty states, timeouts, and partial failures?
- [ ] **External State:** Is the code making safe assumptions about file systems or networks?

### Prompt Engineering Review
```markdown
// ❌ Vague prompt in code
const response = await ai.generate(userInput);

// ✅ Structured & Safe prompt
const response = await ai.generate({
  system: "You are a specialized parser...",
  input: sanitize(userInput),
  schema: ResponseSchema
});
```

## Anti-Patterns to Flag

```typescript
// ❌ Magic numbers
if (status === 3) { ... }

// ✅ Named constants
if (status === Status.ACTIVE) { ... }

// ❌ Deep nesting
if (a) { if (b) { if (c) { ... } } }

// ✅ Early returns
if (!a) return;
if (!b) return;
if (!c) return;
// do work

// ❌ Long functions (100+ lines)
// ✅ Small, focused functions

// ❌ any type
const data: any = ...

// ✅ Proper types
const data: UserData = ...
```

## Review Comments Guide

```
// Blocking issues use 🔴
🔴 BLOCKING: SQL injection vulnerability here

// Important suggestions use 🟡
🟡 SUGGESTION: Consider using useMemo for performance

// Minor nits use 🟢
🟢 NIT: Prefer const over let for immutable variable

// Questions use ❓
❓ QUESTION: What happens if user is null here?

---

## Common Pitfalls
- Not checking for SQL injection vulnerabilities
- Missing XSS vulnerabilities in web apps
- Not validating input at boundaries
- Using SELECT * in production
- Not reviewing error handling
- Ignoring performance implications
- Not checking test coverage
- Missing or outdated documentation

## Best Practices
- Review for correctness (logic errors, edge cases)
- Check security (SQL injection, XSS, auth flaws)
- Verify performance (N+1 queries, inefficient algorithms)
- Assess code quality (naming, duplication, complexity)
- Ensure testing coverage (unit, integration, edge cases)
- Validate documentation (comments, API docs, README)
- Use structured review format (Critical/Improvements/Good)
- Provide specific, actionable feedback with examples
