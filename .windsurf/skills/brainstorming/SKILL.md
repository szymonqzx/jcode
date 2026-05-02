---
name: brainstorming
description: Socratic questioning protocol + user communication. MANDATORY for complex requests, new features, or unclear requirements. Includes progress reporting and error handling.
allowed-tools: Read, Glob, Grep
---

# Brainstorming & Communication Protocol

> **MANDATORY:** Use for complex/vague requests, new features, updates.

## When to Use
- Complex or vague user requests requiring clarification
- New feature development without clear specifications
- Architecture decisions with multiple options
- Updates or changes where scope is unclear
- Any request where requirements are ambiguous
- When user says "build/create/make" without details

---

## SOCRATIC GATE (ENFORCEMENT)

### When to Trigger

| Pattern | Action |
|---------|--------|
| "Build/Create/Make [thing]" without details | ASK 3 questions |
| Complex feature or architecture | Clarify before implementing |
| Update/change request | Confirm scope |
| Vague requirements | Ask purpose, users, constraints |

### MANDATORY: 3 Questions Before Implementation

1. **STOP** - Do NOT start coding
2. **ASK** - Minimum 3 questions:
   - Purpose: What problem are you solving?
   - Users: Who will use this?
   - Scope: Must-have vs nice-to-have?
3. **WAIT** - Get response before proceeding

---

## Dynamic Question Generation

**NEVER use static templates.** Read `dynamic-questioning.md` for principles.

### Core Principles

| Principle | Meaning |
|-----------|---------|
| **Questions Reveal Consequences** | Each question connects to an architectural decision |
| **Context Before Content** | Understand greenfield/feature/refactor/debug context first |
| **Minimum Viable Questions** | Each question must eliminate implementation paths |
| **Generate Data, Not Assumptions** | Don't guess—ask with trade-offs |

### Question Generation Process

```
1. Parse request → Extract domain, features, scale indicators
2. Identify decision points → Blocking vs. deferable
3. Generate questions → Priority: P0 (blocking) > P1 (high-leverage) > P2 (nice-to-have)
4. Format with trade-offs → What, Why, Options, Default
```

### Question Format (MANDATORY)

```markdown
### [PRIORITY] **[DECISION POINT]**

**Question:** [Clear question]

**Why This Matters:**
- [Architectural consequence]
- [Affects: cost/complexity/timeline/scale]

**Options:**
| Option | Pros | Cons | Best For |
|--------|------|------|----------|
| A | [+] | [-] | [Use case] |

**If Not Specified:** [Default + rationale]
```

**For detailed domain-specific question banks and algorithms**, see: `dynamic-questioning.md`

---

## Progress Reporting (PRINCIPLE-BASED)

**PRINCIPLE:** Transparency builds trust. Status must be visible and actionable.

### Status Board Format

| Agent | Status | Current Task | Progress |
|-------|--------|--------------|----------|
| [Agent Name] | ✅🔄⏳❌⚠️ | [Task description] | [% or count] |

### Status Icons

| Icon | Meaning | Usage |
|------|---------|-------|
| ✅ | Completed | Task finished successfully |
| 🔄 | Running | Currently executing |
| ⏳ | Waiting | Blocked, waiting for dependency |
| ❌ | Error | Failed, needs attention |
| ⚠️ | Warning | Potential issue, not blocking |

---

## Error Handling (PRINCIPLE-BASED)

**PRINCIPLE:** Errors are opportunities for clear communication.

### Error Response Pattern

```
1. Acknowledge the error
2. Explain what happened (user-friendly)
3. Offer specific solutions with trade-offs
4. Ask user to choose or provide alternative
```

### Error Categories

| Category | Response Strategy |
|----------|-------------------|
| **Port Conflict** | Offer alternative port or close existing |
| **Dependency Missing** | Auto-install or ask permission |
| **Build Failure** | Show specific error + suggested fix |
| **Unclear Error** | Ask for specifics: screenshot, console output |

---

## Completion Message (PRINCIPLE-BASED)

**PRINCIPLE:** Celebrate success, guide next steps.

### Completion Structure

```
1. Success confirmation (celebrate briefly)
2. Summary of what was done (concrete)
3. How to verify/test (actionable)
4. Next steps suggestion (proactive)
```

---

## Communication Principles

| Principle | Implementation |
|-----------|----------------|
| **Concise** | No unnecessary details, get to point |
| **Visual** | Use emojis (✅🔄⏳❌) for quick scanning |
| **Specific** | "~2 minutes" not "wait a bit" |
| **Alternatives** | Offer multiple paths when stuck |
| **Proactive** | Suggest next step after completion |

---

## Anti-Patterns (AVOID)

| Anti-Pattern | Why |
|--------------|-----|
| Jumping to solutions before understanding | Wastes time on wrong problem |
| Assuming requirements without asking | Creates wrong output |
| Over-engineering first version | Delays value delivery |
| Ignoring constraints | Creates unusable solutions |
| "I think" phrases | Uncertainty → Ask instead |

---

## Edge Case Handling
- **Overwhelming options**: Too many choices confuse user - cap at 5 options, prioritize relevance
- **Analysis paralysis**: User can't decide - provide clear recommendation with reasoning
- **Domain expertise gap**: User lacks technical context - explain trade-offs in accessible language
- **Time pressure**: Quick decision needed - offer quick wins vs. comprehensive solutions
- **Conflicting constraints**: Requirements conflict - highlight trade-offs explicitly

## Failure Modes
- **Insufficient context**: Generating options without understanding - ask 3 minimum questions first
- **Biased recommendations**: Favoring one option unfairly - ensure balanced pros/cons
- **Implementation leakage**: Getting into code details instead of concepts - stay at architectural level
- **Missing options**: Forgetting viable alternatives - systematically explore solution space
- **Wrong problem**: Solving the wrong issue - validate understanding before brainstorming

## Performance Considerations
- Response time: Generate options within 10-15 seconds for most requests
- Cognitive load: Present options in scannable format with clear structure
- Decision friction: Provide clear recommendation to reduce decision time
- Follow-up clarity: Ensure selected option has clear next steps

## Security Notes
- **Input validation**: Validate brainstorm topic doesn't contain malicious patterns
- **Output sanitization**: Ensure options don't include harmful code or commands
- **Privacy**: Avoid solutions that expose sensitive data or credentials
- **Compliance**: Consider regulatory requirements in options (GDPR, SOC2, etc.)
- **Supply chain**: Evaluate third-party solutions for security posture

## Common Pitfalls
- Asking too many questions at once (overwhelming user)
- Not waiting for user response before proceeding
- Using static/template questions instead of dynamic ones
- Focusing on implementation details instead of architectural decisions
- Not offering trade-offs with options
- Making assumptions about user preferences

## Best Practices
- Ask minimum 3 strategic questions before implementation
- Use dynamic question generation based on context
- Always provide trade-offs with options
- Wait for user response before proceeding
- Format questions with clear "What, Why, Options, Default" structure
- Prioritize blocking questions (P0) over nice-to-haves (P2)

---
