---
trigger: always_on
---

# Windsurf Rules

Defines AI behavior in this workspace.

---

## Agent & Skill Protocol

**MANDATORY:** Read appropriate agent file and its skills BEFORE any implementation. Highest priority rule.

### Skill Loading
Agent activated → Check frontmatter "skills:" → Read SKILL.md → Read specific sections.

- Selective reading: DO NOT READ ALL files. Read applicable SKILL.md first, then only sections matching request
- Rule priority: P0 (.windsurfrules.md) > P1 (Agent .md) > P2 (SKILL.md). All binding.

### Enforcement
When agent activated: Read Rules → Check Frontmatter → Load SKILL.md → Apply All.
Never skip reading agent rules or skill instructions.

### Memory Protocol
When agent activated: Check memory → Use memory tools if applicable → Apply memory context.
Always verify memory relevance before using.
Never create duplicate memories - update existing ones instead.
Maintain memory consistency across sessions.
When ending task: Save final state to memory for continuity.

---

## Request Classifier

Before any action, classify the request:

| Type | Keywords | Tiers | Result |
|------|----------|-------|--------|
| QUESTION | "what is", "how does", "explain" | TIER 0 | Text Response |
| SURVEY/INTEL | "analyze", "list files", "overview" | TIER 0 + Explorer | Session Intel |
| SIMPLE CODE | "fix", "add", "change" (single file) | TIER 0 + TIER 1 (lite) | Inline Edit |
| COMPLEX CODE | "build", "create", "implement", "refactor" | TIER 0 + TIER 1 (full) | {task-slug}.md Required |
| DESIGN/UI | "design", "UI", "page", "dashboard" | TIER 0 + TIER 1 | {task-slug}.md Required |
| SLASH CMD | /create, /debug, /plan, /review, /test | Command-specific | Variable |

---

## Intelligent Agent Routing

**ALWAYS ACTIVE:** Before responding, automatically analyze and select best agent(s).

Follow protocol in `@[skills/intelligent-routing]`.

### Auto-Selection
1. Analyze silently: Detect domains from request
2. Select agent(s): Choose appropriate specialist(s)
3. Inform user: State which expertise applied
4. Apply: Generate response using agent's persona/rules

### Response Format
When auto-applying agent:
```
🤖 Applying knowledge of @[agent-name]...
[Continue with specialized response]
```

### Rules
- Silent analysis (no meta-commentary)
- Respect overrides if user mentions @agent
- Multi-domain: use orchestrator + Socratic questions

### Agent Routing Checklist (Before code/design)
1. Identified correct agent for domain?
2. Read agent's .md file (or recalled rules)?
3. Announced "🤖 Applying knowledge of @[agent]..."?
4. Loaded required skills from agent frontmatter?

**Failures:** Writing code without agent = violation. Skipping announcement = user can't verify. Ignoring agent rules = quality failure.

---

## Universal Rules (Always Active)

### Clean Code
ALL code MUST follow `@[skills/clean-code]` rules.
- Code: Concise, direct, self-documenting
- Testing: Mandatory (Unit > Int > E2E) + AAA Pattern
- Performance: Measure first, adhere to 2025/2026 standards
- Infra/Safety: 5-Phase Deployment, verify secrets

### File Dependency Awareness
Before modifying any file:
1. Check project documentation for file dependencies (CODEBASE.md, ARCHITECTURE.md, or equivalent)
2. Identify dependent files
3. Update ALL affected files together

### System Map
MANDATORY: Read ARCHITECTURE.md at session start.
Paths: Agents (.windsurf/agents/), Skills (.windsurf/skills/), Scripts (.windsurf/scripts/)

### Read → Understand → Apply
WRONG: Read agent → Start coding
CORRECT: Read → Understand WHY → Apply PRINCIPLES → Code

Before coding: What is GOAL? What PRINCIPLES apply? How does this DIFFER from generic?

### Automatic Git Commits [P0]

**MANDATORY:** All agents MUST automatically git commit when finishing a task.

### When to Commit
- After completing each feature, fix, or refactoring
- After passing all relevant tests
- Before marking a task as complete in todo list
- After any substantial code change (>30 lines OR 3+ files modified)
- Small changes (≤30 lines and <3 files) should not be committed unless completing a task

### Commit Protocol
1. Stage all relevant changes: `git add <files>`
2. Commit with descriptive message: `git commit -m "<message>"`
3. Use conventional commit format when applicable
4. Reference related issues/PRs when available

### Push Protocol
- Push commits to remote when finishing a task or session
- If git state is not clean or other agents are working in parallel, still commit your work
- Use `git push` after final commit

### Commit Message Guidelines
- Be concise but descriptive (50-72 chars for subject line)
- Explain WHAT and WHY, not HOW
- Use imperative mood ("Add feature" not "Added feature")
- Reference team IDs when applicable: `// TEAM_XXX: <reason>`

---

## Code Rules (When Writing Code)

### Project Type Routing
| Type | Agent | Skills |
|------|-------|--------|
| MOBILE | mobile-developer | mobile-design |
| WEB | frontend-specialist | frontend-design |
| BACKEND | backend-specialist | api-patterns, database-design |

Mobile + frontend-specialist = WRONG. Mobile = mobile-developer ONLY.

### Socratic Gate (TIER 0)
MANDATORY: Every request must pass Socratic Gate before tool use/implementation.

| Type | Strategy | Action |
|------|----------|--------|
| New Feature/Build | Deep Discovery | ASK minimum 3 strategic questions |
| Code Edit/Bug Fix | Context Check | Confirm + ask impact questions |
| Vague/Simple | Clarification | Ask Purpose, Users, Scope |
| Direct "Proceed" | Validation | STOP → ask 2 Edge Case questions |

**Protocol:**
- Never assume: If 1% unclear, ASK
- Spec-heavy: Ask trade-offs/edge cases before starting
- Wait: No subagents/code until user clears gate
- Reference: Full protocol in `@[skills/brainstorming]`

### Final Checklist Protocol
Trigger: "final checks", "run all tests", "pre-deployment checks"

| Stage | Command | Purpose |
|-------|---------|---------|
| Manual Audit | python .windsurf/scripts/checklist.py . | Project audit |
| Pre-Deploy | python .windsurf/scripts/checklist.py . --url <URL> | Full Suite + Performance + E2E |

**Priority:** Security → Lint → Schema → Tests → UX → SEO → Lighthouse/E2E
**Rules:** Task not finished until checklist.py returns success. Fix Critical blockers first (Security/Lint).

**Scripts:** security_scan.py, dependency_analyzer.py, lint_runner.py, test_runner.py, schema_validator.py, ux_audit.py, accessibility_checker.py, seo_checker.py, bundle_analyzer.py, mobile_audit.py, lighthouse_audit.py, playwright_runner.py
Invoke via: python .windsurf/scripts/<script>.py or python .windsurf/skills/<skill>/scripts/<script>.py

### Agent Mapping
| Mode | Agent | Behavior |
|------|-------|----------|
| plan | project-planner | Use `/plan` workflow for systematic planning with files and team registration |
| ask | - | Focus on understanding, ask questions |
| edit | applicable from request | Execute, check {task-slug}.md first |

**Planning:** Use `/plan` workflow for all planning tasks. Creates `docs/PLAN-{slug}.md` and registers team in `.teams/active/` per global rules Rule 2.
**Edit Mode:** Multi-file/structural → offer to create {task-slug}.md. Single-file fixes → proceed directly.

---

## Design Rules (Reference)

Design rules are in specialist agents, NOT here.

| Task | Read |
|------|------|
| Web UI/UX | .windsurf/agents/frontend-specialist.md |
| Mobile UI/UX | .windsurf/agents/mobile-developer.md |

Agents contain: Purple Ban (no violet/purple), Template Ban (no standard layouts), Anti-cliché rules, Deep Design Thinking.

For design work: Open and READ the agent file.

---

## Quick Reference

**Master Agents:** orchestrator, project-planner, security-auditor, backend-specialist, frontend-specialist, mobile-developer, debugger, game-developer

**Key Skills:** clean-code, brainstorming, app-builder, frontend-design, mobile-design, plan-writing, behavioral-modes

**Key Scripts:** verify_all.py, checklist.py, security_scan.py, dependency_analyzer.py, ux_audit.py, mobile_audit.py, lighthouse_audit.py, seo_checker.py, playwright_runner.py, test_runner.py

---
