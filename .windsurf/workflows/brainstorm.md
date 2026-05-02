---
description: Structured brainstorming for projects and features. Explores multiple options before implementation.
---

# /brainstorm - Structured Idea Exploration

$ARGUMENTS

---

## Purpose

This command activates BRAINSTORM mode for structured idea exploration. Use when you need to explore options before committing to an implementation.

---

## Behavior

When `/brainstorm` is triggered:

1. **Understand the goal**
   - What problem are we solving?
   - Who is the user?
   - What constraints exist?

2. **Generate options**
   - Provide at least 3 different approaches
   - Each with pros and cons
   - Consider unconventional solutions

3. **Compare and recommend**
   - Summarize tradeoffs
   - Give a recommendation with reasoning

---

## Output Format

```markdown
## 🧠 Brainstorm: [Topic]

### Context
[Brief problem statement]

---

### Option A: [Name]
[Description]

✅ **Pros:**
- [benefit 1]
- [benefit 2]

❌ **Cons:**
- [drawback 1]

📊 **Effort:** Low | Medium | High

---

### Option B: [Name]
[Description]

✅ **Pros:**
- [benefit 1]

❌ **Cons:**
- [drawback 1]
- [drawback 2]

📊 **Effort:** Low | Medium | High

---

### Option C: [Name]
[Description]

✅ **Pros:**
- [benefit 1]

❌ **Cons:**
- [drawback 1]

📊 **Effort:** Low | Medium | High

---

## 💡 Recommendation

**Option [X]** because [reasoning].

What direction would you like to explore?
```

---

## Examples

```
/brainstorm authentication system
/brainstorm state management for complex form
/brainstorm database schema for social app
/brainstorm caching strategy
```

---

## When to Use
- Exploring multiple implementation approaches before coding
- Comparing architectural patterns or technology choices
- Designing new features from scratch
- Evaluating trade-offs for significant decisions
- Brainstorming solutions to complex problems

## When NOT to Use
- Simple, obvious implementations with no alternatives
- Bug fixes with clear solutions
- Tasks requiring immediate code implementation
- Well-defined patterns with no decision needed
- Production emergencies requiring quick action

## Edge Case Handling
- **Ambiguous requests**: Ask clarifying questions before generating options
- **Overwhelming options**: Cap at 5 options, prioritize most relevant
- **Conflicting requirements**: Highlight trade-offs explicitly
- **Domain expertise gaps**: Flag when options require specialist knowledge
- **Time constraints**: Offer quick wins vs. comprehensive solutions

## Failure Modes
- **Insufficient context**: When request lacks detail, ask 3 minimum questions before proceeding
- **Analysis paralysis**: If too many options emerge, group by approach and recommend top 3
- **Biased recommendations**: Ensure pros/cons are balanced, not favoring one option
- **Implementation leakage**: Keep options at conceptual level, avoid implementation details
- **User indecision**: Provide clear recommendation with reasoning if user can't decide

## Performance Considerations
- Response time: Generate options within 10-15 seconds for most requests
- Cognitive load: Present options in scannable format with clear structure
- Decision friction: Provide clear recommendation to reduce decision time
- Follow-up complexity: Ensure selected option has clear next steps

## Security Notes
- **Input validation**: Validate brainstorm topic doesn't contain malicious patterns
- **Output sanitization**: Ensure generated options don't include harmful code or commands
- **Privacy**: Avoid suggesting solutions that expose sensitive data or credentials
- **Compliance**: Consider regulatory requirements in options (GDPR, SOC2, etc.)

## Guardrails
- Always provide at least 3 options
- Include honest pros/cons for each option
- Don't hide complexity or risks
- Defer final decision to user
- Keep options at idea level, not implementation

## Key Principles

- **No code** - this is about ideas, not implementation
- **Visual when helpful** - use diagrams for architecture
- **Honest tradeoffs** - don't hide complexity
- **Defer to user** - present options, let them decide
