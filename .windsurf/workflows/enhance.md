---
description: Add or update features in existing application. Used for iterative development with systematic planning and validation.
---

# /enhance - Update Application

$ARGUMENTS

---

## Overview

Systematic workflow for adding features or making updates to existing applications. Emphasizes understanding current state, planning changes, user approval for major work, incremental implementation, and thorough testing. Designed for iterative development with rollback safety.

---

## When to Use
- Adding new features to existing applications
- Updating or modifying existing functionality
- Iterative development and improvements
- Refactoring existing code with architectural considerations
- Integrating new libraries or dependencies
- Performance optimization of existing features
- Security enhancements to current codebase

## When NOT to Use
- Creating new applications from scratch (use /create instead)
- Simple bug fixes (use /debug instead)
- Non-application tasks
- Breaking changes without user approval
- Conflicting with existing architecture without warning
- Emergency hotfixes (use direct editing instead)

---

## Pre-flight Checks

```powershell
# Verify project state - adapt to your project's configuration file
# Examples: Cargo.toml, package.json, pom.xml, build.gradle, requirements.txt
if (-not (Test-Path "<your-config-file>")) {
    Write-Error "No recognized project configuration found"
    exit 1
}

# Check for uncommitted changes
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Warning "Uncommitted changes detected. Consider committing before enhancement."
}

# Verify build passes - adapt to your project's build command
# Examples: cargo check, npm run build, python -m py_compile, mvn compile
<build-check-command> 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Error "Project does not build. Fix errors before enhancement."
    exit 1
}

# List existing tests - adapt to your project's test file pattern
# Examples: *test*.rs, *.test.ts, *_test.py, *Test.java
$testFiles = Get-ChildItem -Recurse -Filter "<your-test-pattern>" -ErrorAction SilentlyContinue
Write-Host "Found $($testFiles.Count) test files"
```

---

## Loop Configuration

```powershell
$MAX_ITERS = 8
$KILLSWITCH = "$env:USERPROFILE\.enhance-stop"
$LOGDIR = ".enhance-logs/$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null
```

---

## The Enhancement Process

### Phase 1: Understand Current State

```powershell
# Load project context
python .agent/scripts/session_manager.py info
```

**Actions:**
- Read main configuration files (adapt to your project: Cargo.toml, package.json, pom.xml, etc.)
- Understand existing features and tech stack
- Review current architecture (ARCHITECTURE.md if exists)
- Identify entry points and core modules
- Check existing test coverage

**Deliverable:** Project context summary with tech stack, key files, and architectural patterns.

---

### Phase 2: Plan Changes

**Actions:**
- Use `/plan` workflow for systematic planning with files
- Register team per global rules Rule 2
- Determine what will be added/changed
- Detect affected files using dependency analysis
- Check compatibility with existing dependencies
- Identify potential breaking changes
- Estimate implementation effort

**Questions to Answer:**
- Which files need modification?
- New files required?
- Dependencies to add?
- Migration path needed?
- Testing strategy?

**Deliverable:** Change plan in `docs/PLAN-{slug}.md` with file list, dependency changes, and risk assessment.

---

### Phase 3: Present Plan to User

For major changes (threshold: >5 files or breaking changes):

```
"To add [feature]:
- I'll create X new files: [list]
- Update Y existing files: [list]
- Add Z dependencies: [list]
- Estimated effort: ~[time]
- Risk level: [low/medium/high]
- Breaking changes: [yes/no]

Should I proceed?"
```

For minor changes:
- Proceed directly but log changes
- Still check for breaking changes

---

### Phase 4: Apply Changes

**Actions:**
- Call relevant agents based on domain (frontend, backend, database)
- Make changes incrementally
- Test each change before proceeding
- Commit after each logical unit

**Order of Operations:**
1. Update dependencies (if any)
2. Create new files
3. Modify existing files
4. Update configuration
5. Add tests
6. Run test suite

**Deliverable:** Applied changes with passing tests.

---

### Phase 5: Update and Verify

**Actions:**
- Hot reload if supported (dev server)
- Restart application if needed
- Manual verification of new functionality
- Regression testing of affected areas
- Performance baseline comparison (if applicable)

**Deliverable:** Verified working enhancement with test results.

---

## Usage Examples

### Feature Addition
```
/enhance add dark mode support
/enhance build admin panel with RBAC
/enhance integrate Stripe payment system
/enhance add full-text search with Elasticsearch
```

### Functionality Updates
```
/enhance edit profile page to include avatar upload
/enhance make dashboard responsive for mobile
/enhance optimize database queries for user list
/enhance add rate limiting to API endpoints
```

### Refactoring
```
/enhance refactor authentication to use JWT
/enhance migrate from Redux to Zustand state management
/enhance extract common components to design system
```

### Security Enhancements
```
/enhance add CSRF protection to forms
/enhance implement input sanitization for all user inputs
/enhance add audit logging for sensitive operations
```

---

## Real-World Scenarios

### Scenario 1: Adding User Authentication
**Context:** Existing API has no authentication
**Plan:**
- Add JWT dependency
- Create auth middleware
- Add login/register endpoints
- Update existing endpoints to require auth
- Add token refresh logic
- Write integration tests

**Risk:** Breaking change for existing API consumers
**Mitigation:** Add optional auth mode, document deprecation timeline

### Scenario 2: Performance Optimization
**Context:** Slow database queries on dashboard
**Plan:**
- Profile current performance
- Identify slow queries
- Add database indexes
- Implement query caching
- Add pagination
- Benchmark before/after

**Risk:** Cache invalidation bugs
**Mitigation:** Add cache monitoring, implement cache warming

### Scenario 3: Third-Party Integration
**Context:** Add email notifications via SendGrid
**Plan:**
- Add SendGrid SDK dependency
- Create email service module
- Configure API keys (environment variables)
- Add email templates
- Implement retry logic
- Add error handling and logging

**Risk:** API key exposure, rate limiting
**Mitigation:** Use secrets management, implement exponential backoff

---

## Edge Case Handling

### Breaking Changes
**Scenario:** Changes affect existing functionality
**Procedure:**
1. Identify all affected code paths
2. Document breaking changes clearly
3. Provide migration guide if needed
4. Get explicit user approval
5. Implement feature flags if gradual rollout needed
6. Add deprecation warnings for old behavior

### Conflicting Patterns
**Scenario:** New code conflicts with existing architecture
**Procedure:**
1. Document the conflict
2. Highlight trade-offs of both approaches
3. Propose architectural refactoring if justified
4. Get user decision on direction
5. Document decision rationale in ARCHITECTURE.md

### Dependency Conflicts
**Scenario:** New dependencies break existing code
**Procedure:**
1. Validate dependencies in isolation (new project)
2. Check version compatibility matrix
3. Look for alternative libraries if conflicts exist
4. Consider dependency vendoring if critical
5. Update all dependent code if version bump required

### Performance Regression
**Scenario:** Changes slow down existing features
**Procedure:**
1. Establish baseline metrics before changes
2. Profile critical paths after implementation
3. Identify regression source
4. Optimize or rollback if significant degradation
5. Add performance regression tests to CI

### Migration Complexity
**Scenario:** Large changes require data migration
**Procedure:**
1. Design migration strategy (big bang vs incremental)
2. Create migration scripts with rollback capability
3. Test migration on staging data
4. Plan maintenance window if needed
5. Document migration procedure
6. Verify data integrity post-migration

---

## Failure Modes

### Incomplete Understanding
**Symptom:** Missing context leads to wrong changes
**Prevention:**
- Read existing code thoroughly before editing
- Review documentation and comments
- Understand data flow and dependencies
- Ask clarifying questions if unclear
- Review git history for recent changes

### Breaking Existing Features
**Symptom:** Changes break working code
**Prevention:**
- Run full test suite before changes
- Run test suite after each major change
- Add regression tests for affected areas
- Use feature flags for risky changes
- Test in isolation before integration

### Dependency Hell
**Symptom:** New dependencies cause version conflicts
**Prevention:**
- Validate dependencies in isolation first
- Check transitive dependencies
- Use dependency lock files
- Prefer well-maintained libraries
- Document dependency versions in requirements

### Poor Integration
**Symptom:** New code doesn't fit existing patterns
**Prevention:**
- Review architecture before coding
- Follow existing code style and patterns
- Consult with team on architectural decisions
- Refactor existing code if patterns are outdated
- Document architectural decisions

### Rollback Difficulty
**Symptom:** Changes hard to undo
**Prevention:**
- Use git for version control
- Commit changes incrementally
- Write atomic commits with clear messages
- Test rollback procedure before deployment
- Keep database migrations reversible
- Maintain feature flags for quick disabling

---

## Performance Considerations

### Build Impact
- Minimize build time increases from new dependencies
- Prefer tree-shakeable libraries for frontend
- Use conditional compilation for optional features
- Monitor build times in CI

### Runtime Performance
- Benchmark critical paths before and after changes
- Profile hot paths with tools (perf, flamegraphs)
- Consider algorithmic complexity of new code
- Add performance regression tests
- Monitor production metrics post-deployment

### Bundle Size (Web)
- Monitor bundle growth for web applications
- Use code splitting for large features
- Lazy load non-critical components
- Analyze bundle with webpack-bundle-analyzer
- Set bundle size budgets in CI

### Database Queries
- Check for N+1 queries when adding features
- Add appropriate indexes for new queries
- Use query optimization tools (EXPLAIN ANALYZE)
- Implement query result caching where appropriate
- Monitor query performance in production

### Memory Usage
- Verify changes don't introduce memory leaks
- Profile memory usage for long-running processes
- Check for unintended object retention
- Use weak references for caches
- Monitor memory metrics in production

---

## Security Notes

### Input Validation
- Add validation for any new user inputs
- Use whitelist validation over blacklist
- Sanitize data before storage or display
- Validate data types and ranges
- Handle malformed input gracefully

### Authentication
- Ensure new endpoints respect existing auth rules
- Use strong authentication mechanisms (JWT, OAuth)
- Implement proper session management
- Handle token expiration and refresh
- Secure authentication endpoints

### Authorization
- Verify permission checks for new features
- Implement principle of least privilege
- Use role-based access control (RBAC)
- Check authorization on both client and server
- Audit authorization failures

### Data Sanitization
- Sanitize user data before storage or display
- Use parameterized queries to prevent SQL injection
- Escape output to prevent XSS
- Validate file uploads (type, size, content)
- Handle sensitive data encryption

### Dependency Security
- Audit new dependencies for vulnerabilities
- Use tools like `cargo-audit`, `npm audit`
- Keep dependencies updated
- Review dependency maintenance status
- Prefer libraries with security track record

### Secrets Management
- Never hardcode API keys or secrets
- Use environment variables for configuration
- Implement secrets rotation
- Use vault services for production secrets
- Audit secret access logs

---

## Guardrails

1. **Understand current state** - Read existing code, architecture, and tests before making changes
2. **User approval** - Get explicit approval for major or breaking changes
3. **Conflict warnings** - Warn on conflicting architectural decisions
4. **Incremental commits** - Commit changes incrementally with meaningful messages
5. **Test before deploy** - Test changes before updating preview or deploying
6. **Rollback planning** - Ensure rollback procedure is tested before deployment
7. **Documentation** - Update documentation alongside code changes
8. **Performance monitoring** - Establish baselines and monitor post-deployment
9. **Security review** - Review security implications of all changes
10. **Killswitch** - Create `~/.enhance-stop` to stop enhancement loop

---

## Success Criteria

- [ ] All tests pass (unit, integration, e2e)
- [ ] No regression in existing functionality
- [ ] Performance metrics within acceptable range
- [ ] Security review completed
- [ ] Documentation updated
- [ ] Code reviewed and approved
- [ ] Rollback procedure tested
- [ ] Feature flagged if needed for gradual rollout

---

## Rollback Procedure

```powershell
# If enhancement fails or causes issues:
# 1. Identify last good commit
git log --oneline -10

# 2. Rollback to last good state
git revert <commit-hash>

# 3. If revert fails, hard reset (use with caution)
git reset --hard <commit-hash>

# 4. Rebuild and test - adapt to your project
<build-command>
<test-command>

# 5. Document the rollback
git log --format="%H %s" -1 > rollback-log.txt
```

---

## Related Skills

- `architecture/skill.md` - Architectural decision-making
- `brainstorming/skill.md` - Socratic questioning for requirements
- `clean-code/skill.md` - Pragmatic coding standards
- `error-handling/skill.md` - Error handling patterns
- `plan-writing/skill.md` - Structured task planning
- `systematic-debugging/skill.md` - Debugging methodology

---

## Related Workflows

- `implement.md` - Systematic implementation with planning
- `debug.md` - Debugging workflow for issues
- `code-fix-loop.md` - Iterative code fixing
- `test.md` - Test generation and execution
- `suggest.md` - Feature brainstorming

---

## Credits

Universal AI Team Rulebook, Planning With Files pattern, systematic debugging methodology, clean code principles, test-driven development practices
