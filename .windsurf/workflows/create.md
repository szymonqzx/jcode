---
description: Create new application command. Triggers App Builder skill and starts interactive dialogue with user.
---

# /create - Create Application

$ARGUMENTS

---

## Task

This command starts a new application creation process.

### Steps:

1. **Request Analysis**
   - Understand what the user wants
   - If information is missing, use `conversation-manager` skill to ask

2. **Project Planning**
   - Use `/plan` workflow for systematic planning with files
   - Register team per global rules Rule 2
   - Determine tech stack
   - Plan file structure
   - Create plan file in `docs/PLAN-{slug}.md` and proceed to building

3. **Application Building (After Approval)**
   - Orchestrate with `app-builder` skill
   - Coordinate expert agents:
     - `database-architect` → Schema
     - `backend-specialist` → API
     - `frontend-specialist` → UI

4. **Preview**
   - Start with `auto_preview.py` when complete
   - Present URL to user

---

## Usage Examples

```
/create blog site
/create e-commerce app with product listing and cart
/create todo app
/create Instagram clone
/create crm system with customer management
```

---

## Before Starting

If request is unclear, ask these questions:
- What type of application?
- What are the basic features?
- Who will use it?

Use defaults, add details later.

---

## When to Use
- Creating a new application from scratch
- Starting a new project with no existing codebase
- Building MVP or prototype applications
- Setting up initial project structure and tech stack

## When NOT to Use
- Modifying existing applications (use /enhance instead)
- Simple bug fixes or small changes
- Adding features to existing codebase
- Non-application tasks (scripts, utilities)

## Edge Case Handling
- **Vague requirements**: When request lacks detail, ask 3 minimum questions (purpose, users, features)
- **Conflicting constraints**: Highlight trade-offs when requirements conflict (e.g., speed vs. cost)
- **Tech stack uncertainty**: Recommend stack based on project type, get user approval before proceeding
- **Scope creep**: Define MVP boundaries clearly, defer non-essential features to later
- **Resource constraints**: Consider development time, team size, budget when recommending approaches

## Failure Modes
- **Wrong tech stack**: Choosing inappropriate stack leads to rework - validate against requirements
- **Over-engineering**: Building too much complexity for MVP - start simple, iterate
- **Missing dependencies**: Incomplete setup causes runtime errors - verify all dependencies install
- **Broken build**: Code doesn't run immediately - test build before marking complete
- **Poor structure**: Disorganized codebase hinders maintenance - follow established patterns

## Performance Considerations
- Setup time: Complete initial project structure within 5-10 minutes
- Build time: Optimize for fast initial builds (avoid heavy dependencies initially)
- Development velocity: Choose stack with good tooling and hot reload for rapid iteration
- Bundle size: Consider initial bundle size impact for web projects

## Security Notes
- **Dependency security**: Audit initial dependencies for known vulnerabilities
- **Secrets management**: Never commit API keys, use environment variables from start
- **Authentication defaults**: Don't include hardcoded credentials or default passwords
- **HTTPS enforcement**: Configure SSL/TLS from the start for web applications
- **Input validation**: Include basic validation patterns in initial scaffolding

## Guardrails
- Always clarify requirements before starting
- Get user approval for tech stack choices
- Use established project patterns when available
- Create proper project structure from the start
- Ensure code is immediately runnable
