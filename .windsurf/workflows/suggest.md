---
description: Brainstorm and suggest features or improvements with systematic analysis
---

# /suggest - Brainstorm Suggestions

$ARGUMENTS

---

## Purpose

Brainstorm and document new features or improvements with systematic analysis, feasibility assessment, and prioritization.

---

## Sub-commands

```
/suggest feature [topic]      - Suggest new features
/suggest improvement [area]   - Suggest improvements to existing code
```

---

## Feature Suggestions

### When to Use
- User requests new feature ideas
- Project needs direction for next development phase
- Exploring potential enhancements to existing functionality

### Brainstorming Process

**1. Context Analysis**
- Read README.md for project goals
- Review TODO.md for known issues
- Check recent team logs for context
- Examine codebase structure

**2. Idea Generation Categories**
- Performance optimizations - Speed, memory, efficiency
- User experience - CLI usability, error messages, documentation
- Integration - External tools, APIs, platforms
- Reliability - Error handling, testing, monitoring
- Maintainability - Code organization, refactoring, documentation
- Innovation - Novel approaches, new capabilities

**3. Feasibility Assessment**
- Technical feasibility - Can it be implemented with current stack?
- Complexity - Estimated effort and risk
- Impact - User value and project benefit
- Alignment - Does it fit project vision?
- Dependencies - External requirements or blockers

**4. Prioritization Matrix**
```
High Impact, Low Complexity    → Do First
High Impact, High Complexity   → Plan Carefully
Low Impact, Low Complexity     → Consider Later
Low Impact, High Complexity    → Discard
```

### Feature Proposal Template

```markdown
# Feature: [Name]

## Problem Statement
What problem does this solve? Why is it needed?

## Proposed Solution
High-level description of the solution.

## Benefits
- Benefit 1
- Benefit 2

## Implementation Approach
- Technical approach
- Key components
- Dependencies

## Complexity Assessment
- Estimated effort: [Low/Medium/High]
- Risk level: [Low/Medium/High]
- Breaking changes: [Yes/No]

## Alternatives Considered
- Alternative 1
- Alternative 2

## Open Questions
- Question 1
- Question 2
```

---

## Improvement Suggestions

### When to Use
- User requests improvement suggestions
- Code review reveals patterns that need enhancement
- Performance profiling identifies bottlenecks
- Technical debt accumulation needs addressing

### When NOT to Use
- Implementing improvements (use /code-fix or /enhance instead)
- Simple code formatting or style fixes
- Tasks requiring immediate refactoring
- Non-improvement requests

### Brainstorming Process

**1. Codebase Analysis**
- Review recent commits for patterns
- Check TODO.md for known issues
- Examine error handling consistency
- Look for code duplication
- Identify performance bottlenecks
- Check test coverage gaps

**2. Improvement Categories**
- Code quality - Readability, maintainability, duplication
- Performance - Speed, memory usage, efficiency
- Error handling - Consistency, clarity, recovery
- Testing - Coverage, test quality, flakiness
- Documentation - Comments, README, API docs
- Architecture - Modularity, coupling, design patterns
- Security - Vulnerabilities, best practices
- Developer experience - Build times, error messages, debugging

**3. Impact Assessment**
- Current pain point - What problem does this solve?
- Benefit - What value does this provide?
- Effort - Estimated implementation time
- Risk - Potential for introducing bugs
- Scope - Files/modules affected
- Priority - Urgency and importance

**4. Prioritization Matrix**
```
High Benefit, Low Effort     → Quick Wins
High Benefit, High Effort    → Strategic Investments
Low Benefit, Low Effort      → Cleanup Tasks
Low Benefit, High Effort     → Defer or Discard
```

### Improvement Proposal Template

```markdown
# Improvement: [Name]

## Current State
Description of the current situation/problem.

## Proposed Improvement
High-level description of the improvement.

## Benefits
- Benefit 1
- Benefit 2

## Implementation Approach
- Technical approach
- Files/modules affected
- Breaking changes (if any)

## Effort Assessment
- Estimated effort: [Low/Medium/High]
- Risk level: [Low/Medium/High]
- Test coverage impact: [Positive/Negative/Neutral]

## Alternatives Considered
- Alternative 1
- Alternative 2

## Open Questions
- Question 1
- Question 2
```

---

## Output Format

Create suggestions in `.questions/` directory:
- `TEAM_XXX_feature_[name].md` for team-specific features
- `feature_[name].md` for general features
- `TEAM_XXX_improvement_[name].md` for team-specific improvements
- `improvement_[name].md` for general improvements

---

## Guardrails (Non-Negotiable)

1. **Context first** - Never suggest without understanding the project/code
2. **Feasibility check** - Only suggest technically feasible changes
3. **User value** - Every suggestion must provide clear benefit
4. **No scope creep** - Keep suggestions focused and realistic
5. **Document assumptions** - Clearly state any assumptions made
6. **Measure impact** - Only suggest improvements with clear benefits
7. **Consider risk** - Assess potential for introducing bugs

---

## Examples

```
/suggest feature monitoring dashboard
/suggest improvement error handling consistency
/suggest feature support for multiple storage backends
/suggest improvement reduce memory footprint
```

---

## Related Skills

- Replace with project-relevant skills from `.windsurf/skills/`
- Examples: error-handling, architecture, database-design, performance-profiling
- Reference skills that match your project's technology stack

---

## Related Workflows

- `/implement-feature` - For implementing approved features
- `/code-fix` - For implementing approved improvements

## Edge Case Handling
- **No clear direction**: When project has no obvious next steps - analyze README, TODO, and recent commits for context
- **Overwhelming options**: Too many potential suggestions - prioritize by impact and feasibility matrix
- **Technical debt vs features**: Balance between fixing issues and adding new capabilities - assess both categories
- **Resource constraints**: Limited development time or team size - focus on quick wins and high-impact items
- **Conflicting suggestions**: Multiple improvements address same area - consolidate or rank by benefit
- **Domain expertise gaps**: Suggestions require unfamiliar tech - flag for research or recommend alternatives
- **User preference unclear**: User doesn't specify feature vs improvement - ask for clarification or provide both

## Failure Modes
- **Irrelevant suggestions**: Ideas don't match project context or goals - always analyze project first
- **Over-engineering**: Suggesting complex solutions for simple problems - prioritize simplicity
- **Missing dependencies**: Suggestions require unavailable resources - validate feasibility before proposing
- **Low-value ideas**: Suggestions with minimal impact - use prioritization matrix to filter
- **Implementation blockers**: Suggestions require major refactoring - note complexity and risk
- **Duplicate suggestions**: Repeating existing ideas - check TODO.md and team logs first
- **Scope creep**: Individual suggestions too large - break down into smaller, actionable items

## Performance Considerations
- **Analysis speed**: Complete suggestion generation within 5-10 minutes
- **Context loading**: Read key files efficiently (README, TODO, project config) - avoid exhaustive codebase scan
- **Suggestion quality**: Balance quantity with relevance - 5-10 high-quality suggestions better than 50 poor ones
- **Prioritization time**: Use matrix-based prioritization for quick decision-making
- **Output format**: Use structured templates for consistent, scannable output
- **Follow-up efficiency**: Store suggestions in .questions/ for easy reference and tracking
- **Review overhead**: Keep proposals concise but complete - avoid excessive detail

## Security Notes
- **Feature security**: New features should include security considerations in proposal
- **Dependency audit**: Suggested dependencies should be audited for vulnerabilities
- **Access control**: New features should consider authentication/authorization requirements
- **Data privacy**: Improvements should not expose sensitive data or weaken privacy
- **Input validation**: Suggested code should include proper input validation
- **Secrets handling**: Proposals should not introduce hardcoded secrets
- **Compliance**: Consider regulatory requirements (GDPR, SOC2, etc.) in suggestions
