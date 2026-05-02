---
description: Create project plan using project-planner agent. No code writing - only plan file generation.
---

# /plan - Project Planning Mode

$ARGUMENTS

---

## 🔴 CRITICAL RULES

1. **NO CODE WRITING** - This command creates plan file only
2. **Use project-planner agent** - NOT Antigravity Agent's native Plan mode
3. **Socratic Gate** - Ask clarifying questions before planning
4. **Dynamic Naming** - Plan file named based on task
5. **Team Registration** - Create team file in `.teams/active/` per global rules

---

## Task

Use the `project-planner` agent with this context:

```text
CONTEXT:
- User Request: $ARGUMENTS
- Mode: PLANNING ONLY (no code)
- Output: docs/PLAN-{task-slug}.md (dynamic naming)
- Team Registration: Create team file in .teams/active/ per global rules Rule 2

NAMING RULES:
1. Extract 2-3 key words from request
2. Lowercase, hyphen-separated
3. Max 30 characters
4. Example: "e-commerce cart" → PLAN-ecommerce-cart.md

TEAM REGISTRATION (per global rules Rule 2):
1. Determine highest existing team number from .teams/
2. Your team number = highest + 1
3. Create .teams/active/TEAM_XXX_{summary}.md with frontmatter:
   - status: active
   - created: YYYY-MM-DD
   - completed: (leave blank until done)
4. Include task description, progress checklist, decisions section
5. Update team file throughout planning process

RULES:
1. Follow project-planner.md Phase -1 (Context Check)
2. Follow project-planner.md Phase 0 (Socratic Gate)
3. Create PLAN-{slug}.md with task breakdown
4. Register team in .teams/active/ with proper frontmatter
5. DO NOT write any code files
6. REPORT the exact file names created (plan + team file)
```

---

## Expected Output

| Deliverable | Location |
|-------------|----------|
| Project Plan | `docs/PLAN-{task-slug}.md` |
| Team File | `.teams/active/TEAM_XXX_{summary}.md` |
| Task Breakdown | Inside plan file |
| Agent Assignments | Inside plan file |
| Verification Checklist | Phase X in plan file |

---

## After Planning

Tell user:

```text
[OK] Plan created: docs/PLAN-{slug}.md
[OK] Team registered: .teams/active/TEAM_XXX_{summary}.md

Next steps:
- Review the plan and team file
- Run `/create` to start implementation
- Or modify plan manually
- Team will move to .teams/completed/ when finished
```

---

## Naming Examples

| Request | Plan File |
|---------|-----------|
| `/plan e-commerce site with cart` | `docs/PLAN-ecommerce-cart.md` |
| `/plan mobile app for fitness` | `docs/PLAN-fitness-app.md` |
| `/plan add dark mode feature` | `docs/PLAN-dark-mode.md` |
| `/plan fix authentication bug` | `docs/PLAN-auth-fix.md` |
| `/plan SaaS dashboard` | `docs/PLAN-saas-dashboard.md` |

---

## Usage

```
/plan e-commerce site with cart
/plan mobile app for fitness tracking
/plan SaaS dashboard with analytics
```

---

## When to Use
- Planning new projects or features before implementation
- Breaking down complex tasks into manageable steps
- Creating structured project plans with agent assignments
- Documenting architecture and implementation approach

## When NOT to Use
- Quick prototyping or MVP building (use /create instead)
- Simple tasks with clear implementation paths
- Tasks requiring immediate code implementation
- Non-project planning tasks

## Edge Case Handling
- **Unclear task scope**: When request is ambiguous, ask clarifying questions before planning
- **Overly complex plans**: If plan exceeds 10 tasks, break into multiple phases
- **Missing dependencies**: Identify blockers and prerequisites before starting implementation
- **Conflicting requirements**: Highlight trade-offs when goals conflict
- **Resource constraints**: Consider time, team, and budget limitations in task breakdown

## Failure Modes
- **Planning paralysis**: Spending too much time planning instead of acting - limit planning to 15-20 minutes
- **Incomplete plans**: Missing critical tasks leads to blocked implementation - validate plan completeness
- **Unrealistic estimates**: Tasks take longer than planned - add buffer time and identify risks
- **Wrong assumptions**: Planning based on incorrect understanding - validate with user before finalizing
- **Scope creep**: Plan grows during implementation - define clear boundaries and change process

## Performance Considerations
- Planning velocity: Complete plan within 15-20 minutes for most tasks
- Task granularity: Balance between too granular (micro-management) and too coarse (untrackable)
- Dependency optimization: Sequence tasks to maximize parallel work where possible
- Review efficiency: Keep plan review focused on blockers and risks, not every detail

## Security Notes
- **Access control**: Consider who needs access to plan documents (team, stakeholders)
- **Sensitive information**: Don't include secrets, credentials, or PII in plan files
- **Compliance**: Ensure plan addresses regulatory requirements if applicable (GDPR, HIPAA, etc.)
- **Security testing**: Include security tasks in plan (penetration testing, security review)
- **Data classification**: Identify and plan for handling of sensitive data

## Guardrails
- NO code writing - plan files only
- Always use Socratic Gate before planning
- Ask clarifying questions if requirements are unclear
- Use dynamic naming based on task content
- Follow project-planner agent methodology
