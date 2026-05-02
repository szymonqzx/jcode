---
name: plan-writing
description: Structured task planning with clear breakdowns, dependencies, and verification criteria. Use when implementing features, refactoring, or any multi-step work.
allowed-tools: Read, Glob, Grep
---

# Plan Writing

> Source: obra/superpowers

## When to Use
- Planning complex features (>20 LOC or >1 file)
- Breaking down large tasks into manageable steps
- Creating implementation plans for features
- Documenting architecture decisions
- Coordinating team work with clear dependencies
- When task complexity requires structured approach

## Plan Template

### 1. Overview
- **Goal**: What are we trying to achieve?
- **Scope**: What's in and what's out?
- **Dependencies**: What do we need first?

### 2. Architecture
- **Components**: What are the major pieces?
- **Interfaces**: How do they interact?
- **Data Flow**: How does data move through the system?

### 3. Implementation Tasks
- **Task List**: Ordered list of tasks with dependencies
- **Verification**: How do we know each task is done?
- **Testing**: What tests do we need?

### 4. Risks & Mitigations
- **Risks**: What could go wrong?
- **Mitigations**: How do we handle them?
- **Rollback**: How do we undo changes if needed?

## Planning Principles (NOT Templates!)

> 🔴 **NO fixed templates. Each plan is UNIQUE to the task.**

### Principle 1: Keep It SHORT

| ❌ Wrong | ✅ Right |
|----------|----------|
| 50 tasks with sub-sub-tasks | 5-10 clear tasks max |
| Every micro-step listed | Only actionable items |
| Verbose descriptions | One-line per task |

> **Rule:** If plan is longer than 1 page, it's too long. Simplify.

---

### Principle 2: Be SPECIFIC, Not Generic

| ❌ Wrong | ✅ Right |
|----------|----------|
| "Set up project" | "Run `npx create-next-app`" |
| "Add authentication" | "Install next-auth, create `/api/auth/[...nextauth].ts`" |
| "Style the UI" | "Add Tailwind classes to `Header.tsx`" |

> **Rule:** Each task should have a clear, verifiable outcome.

---

### Principle 3: Dynamic Content Based on Project Type

**For NEW PROJECT:**
- What tech stack? (decide first)
- What's the MVP? (minimal features)
- What's the file structure?

**For FEATURE ADDITION:**
- Which files are affected?
- What dependencies needed?
- How to verify it works?

**For BUG FIX:**
- What's the root cause?
- What file/line to change?
- How to test the fix?

---

### Principle 4: Scripts Are Project-Specific

> 🔴 **DO NOT copy-paste script commands. Choose based on project type.**

| Project Type | Relevant Scripts |
|--------------|------------------|
| Frontend/React | `ux_audit.py`, `accessibility_checker.py` |
| Backend/API | `api_validator.py`, `security_scan.py` |
| Mobile | `mobile_audit.py` |
| Database | `schema_validator.py` |
| Full-stack | Mix of above based on what you touched |

**Wrong:** Adding all scripts to every plan
**Right:** Only scripts relevant to THIS task

---

### Principle 5: Verification is Simple

| ❌ Wrong | ✅ Right |
|----------|----------|
| "Verify the component works correctly" | "Run `npm run dev`, click button, see toast" |
| "Test the API" | "curl localhost:3000/api/users returns 200" |
| "Check styles" | "Open browser, verify dark mode toggle works" |

---

## Plan Structure (Flexible, Not Fixed!)

```
# [Task Name]

## Goal
One sentence: What are we building/fixing?

## Tasks
- [ ] Task 1: [Specific action] → Verify: [How to check]
- [ ] Task 2: [Specific action] → Verify: [How to check]
- [ ] Task 3: [Specific action] → Verify: [How to check]

## Done When
- [ ] [Main success criteria]
```

> **That's it.** No phases, no sub-sections unless truly needed.
> Keep it minimal. Add complexity only when required.

## Notes
[Any important considerations]
```

---

## Best Practices (Quick Reference)

1. **Start with goal** - What are we building/fixing?
2. **Max 10 tasks** - If more, break into multiple plans
3. **Each task verifiable** - Clear "done" criteria
4. **Project-specific** - No copy-paste templates
5. **Update as you go** - Mark `[x]` when complete

---

## When to Use

- New project from scratch
- Adding a feature
- Fixing a bug (if complex)
- Refactoring multiple files

---

## Edge Case Handling
- **Task dependencies**: Tasks blocked by others - identify critical path and parallel opportunities
- **Unknown complexity**: Can't estimate task effort - break into smaller tasks or add research spike
- **Changing requirements**: Plan becomes outdated - plan for change and update as needed
- **Resource constraints**: Limited time/people - prioritize tasks and cut scope if needed
- **External blockers**: Waiting on external dependencies - identify and plan workarounds

## Failure Modes
- **Too granular**: Micro-management level detail - consolidate related tasks
- **Too vague**: Unclear what "done" means - add specific verification criteria
- **Missing tasks**: Forgot important steps - review with checklist or peer review
- **Wrong dependencies**: Task order doesn't work - validate dependencies before starting
- **Unrealistic estimates**: Tasks take longer than planned - add buffer and track actual vs estimated

## Performance Considerations
- Planning time: Complete plan within 15-20 minutes for most tasks
- Task size: Balance between too small (overhead) and too large (untrackable)
- Parallelization: Identify tasks that can be done simultaneously
- Review efficiency: Keep plan reviews focused on blockers and risks

## Security Notes
- **Access control**: Consider who needs access to plan documents
- **Sensitive information**: Don't include secrets, credentials, or PII in plans
- **Compliance**: Ensure plan addresses regulatory requirements if applicable
- **Security tasks**: Include security considerations (penetration testing, security review)
- **Data handling**: Identify and plan for handling of sensitive data

## Common Pitfalls
- Writing plans longer than 1 page (too complex)
- Being generic instead of specific ("Set up project" vs "Run npx create-next-app")
- Including every micro-step instead of actionable items
- Copy-pasting templates instead of customizing for context
- Not making tasks verifiable
- Adding irrelevant scripts to every plan

## Best Practices
- Keep plans short (max 10 tasks, 1 page)
- Be specific with exact commands and file names
- Each task should have clear verification criteria
- Customize scripts based on project type (frontend vs backend vs mobile)
- Update plan as you go (mark [x] when complete)
- Start with goal: what are we building/fixing?
