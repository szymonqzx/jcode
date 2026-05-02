---
name: architecture
description: Architectural decision-making framework. Requirements analysis, trade-off evaluation, ADR documentation. Use when making architecture decisions or analyzing system design.
allowed-tools: Read, Glob, Grep
---

# Architecture Decision Framework

"Requirements drive architecture. Trade-offs inform decisions. ADRs capture rationale."

## When to Use
- Making architecture decisions for new features or systems
- Analyzing system design and architectural patterns
- Documenting architectural trade-offs with ADRs
- Evaluating technology choices and patterns
- Designing system structure and component relationships

## Selective Reading Rule

**Read ONLY files relevant to the request!**

| File | Description | When to Read |
|------|-------------|--------------|
| `context-discovery.md` | Questions to ask, project classification | Starting architecture design |
| `trade-off-analysis.md` | ADR templates, trade-off framework | Documenting decisions |
| `pattern-selection.md` | Decision trees, anti-patterns | Choosing patterns |
| `examples.md` | MVP, SaaS, Enterprise examples | Reference implementations |
| `patterns-reference.md` | Quick lookup for patterns | Pattern comparison |

---

## Related Skills

| Skill | Use For |
|-------|---------|
| `@[skills/database-design]` | Database schema design |
| `@[skills/api-patterns]` | API design patterns |
| `@[skills/deployment-procedures]` | Deployment architecture |

---

## Core Principle

"Simplicity is the ultimate sophistication."

- Start simple
- Add complexity ONLY when proven necessary
- You can always add patterns later
- Removing complexity is MUCH harder than adding it

---

## Validation Checklist

Before finalizing architecture:

- [ ] Requirements clearly understood
- [ ] Constraints identified
- [ ] Each decision has trade-off analysis
- [ ] Simpler alternatives considered
- [ ] ADRs written for significant decisions
- [ ] Team expertise matches chosen patterns

## Edge Case Handling
- **Evolving requirements**: Architecture must accommodate change - design for extensibility
- **Scale uncertainty**: Unknown future scale - design for horizontal scaling when possible
- **Team size changes**: Architecture for small team may not scale - consider team coordination overhead
- **Technology shifts**: New tech may invalidate choices - design for modularity and replaceability
- **Budget constraints**: Ideal architecture too expensive - prioritize based on ROI

## Failure Modes
- **Over-engineering**: Building too much complexity for current needs - start simple, iterate
- **Under-engineering**: Not enough structure for growth - identify minimum viable architecture
- **Wrong abstractions**: Leaky abstractions cause pain everywhere - validate abstractions with real usage
- **Tight coupling**: Changes ripple through system - design clear boundaries and interfaces
- **Ignoring constraints**: Architecture doesn't fit team/scale/budget - validate against real constraints

## Performance Considerations
- Latency targets: Design architecture to meet SLA requirements
- Throughput capacity: Plan for peak load, not average load
- Data locality: Consider where data lives vs. where it's processed
- Caching strategy: Design caching at appropriate layers (CDN, application, database)
- Database performance: Design schema and queries for expected query patterns

## Security Notes
- **Defense in depth**: Security at multiple layers (network, application, data)
- **Least privilege**: Minimize permissions for all components and services
- **Data classification**: Design based on data sensitivity (public, internal, confidential)
- **Audit trails**: Log access and modifications to critical resources
- **Secure defaults**: Default to secure configurations, opt-in to insecure features

## Common Pitfalls
- Over-engineering for current requirements
- Choosing patterns without understanding trade-offs
- Not documenting architectural decisions
- Ignoring team expertise and constraints
- Premature optimization before understanding scale
- Copying architectures without context adaptation
