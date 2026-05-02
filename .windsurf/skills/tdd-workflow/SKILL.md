---
name: tdd-workflow
description: Test-Driven Development workflow principles. RED-GREEN-REFACTOR cycle.
allowed-tools: Read, Write, Edit, Glob, Grep, Bash
---

# TDD Workflow

> Write tests first, code second.

## When to Use
- Implementing new features with test-driven development
- Refactoring existing code with safety net
- Writing tests for untested codebases
- Teaching TDD methodology
- Improving code quality through test coverage
- When requirements are clear enough to write tests first

## RED-GREEN-REFACTOR Cycle

### RED: Write a Failing Test
- Write a test that fails
- The test should be minimal and focused
- Run the test to confirm it fails
- The test should fail for the right reason

### GREEN: Make the Test Pass
- Write the minimal code to make the test pass
- The code should be simple and direct
- Run the test to confirm it passes
- Do not worry about code quality yet

### REFACTOR: Improve the Code
- Improve the code structure and quality
- Ensure the test still passes
- Remove duplication
- Improve readability

## The Three Laws of TDD
1. You are not allowed to write any production code unless it is to make a failing unit test pass.
2. You are not allowed to write more of a unit test than is sufficient to fail, and not compiling is failing.
3. You are not allowed to write more production code than is sufficient to pass the currently failing test.

## RED Phase Principles

### What to Write

| Focus | Example |
|-------|---------|
| Behavior | "should add two numbers" |
| Edge cases | "should handle empty input" |
| Error states | "should throw for invalid data" |

### RED Phase Rules

- Test must fail first
- Test name describes expected behavior
- One assertion per test (ideally)

---

## 4. GREEN Phase Principles

### Minimum Code

| Principle | Meaning |
|-----------|---------|
| **YAGNI** | You Aren't Gonna Need It |
| **Simplest thing** | Write the minimum to pass |
| **No optimization** | Just make it work |

### GREEN Phase Rules

- Don't write unneeded code
- Don't optimize yet
- Pass the test, nothing more

---

## 5. REFACTOR Phase Principles

### What to Improve

| Area | Action |
|------|--------|
| Duplication | Extract common code |
| Naming | Make intent clear |
| Structure | Improve organization |
| Complexity | Simplify logic |

### REFACTOR Rules

- All tests must stay green
- Small incremental changes
- Commit after each refactor

---

## 6. AAA Pattern

Every test follows:

| Step | Purpose |
|------|---------|
| **Arrange** | Set up test data |
| **Act** | Execute code under test |
| **Assert** | Verify expected outcome |

---

## 7. When to Use TDD

| Scenario | TDD Value |
|----------|-----------|
| New feature | High |
| Bug fix | High (write test first) |
| Complex logic | High |
| Exploratory | Low (spike, then TDD) |
| UI layout | Low |

---

## 8. Test Prioritization

| Priority | Test Type |
|----------|-----------|
| 1 | Happy path |
| 2 | Error cases |
| 3 | Edge cases |
| 4 | Performance |

---

## 9. Anti-Patterns

| ❌ Don't | ✅ Do |
|----------|-------|
| Skip the RED phase | Watch test fail first |
| Write tests after | Write tests before |
| Over-engineer initial | Keep it simple |
| Multiple asserts | One behavior per test |
| Test implementation | Test behavior |

---

## 10. AI-Augmented TDD

### Multi-Agent Pattern

| Agent | Role |
|-------|------|
| Agent A | Write failing tests (RED) |
| Agent B | Implement to pass (GREEN) |
| Agent C | Optimize (REFACTOR) |

---

> **Remember:** The test is the specification. If you can't write a test, you don't understand the requirement.

---

## Common Pitfalls
- Skipping the RED phase (not watching test fail first)
- Writing tests after code instead of before
- Over-engineering initial implementation in GREEN phase
- Multiple assertions in one test
- Testing implementation details instead of behavior
- Not refactoring in REFACTOR phase
- Writing tests that are too brittle

## Best Practices
- Always watch test fail before implementing (RED)
- Write minimal code to pass test (GREEN)
- Improve code structure while tests stay green (REFACTOR)
- Use AAA pattern (Arrange, Act, Assert)
- One behavior per test
- Test behavior, not implementation
- Keep tests simple and focused
