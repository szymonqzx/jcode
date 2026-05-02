---
name: database-design
description: Database design principles and decision-making. Schema design, indexing strategy, ORM selection, serverless databases.
allowed-tools: Read, Write, Edit, Glob, Grep
---

# Database Design

"Learn to THINK, not copy SQL patterns."

## When to Use
- Designing database schemas for new projects
- Selecting database technology (PostgreSQL, SQLite, Neon, Turso, etc.)
- Choosing ORM frameworks (Prisma, Drizzle, SQLAlchemy, TypeORM)
- Planning indexing strategies for performance
- Designing database migrations
- Optimizing query performance and N+1 issues

## Selective Reading Rule

**Read ONLY files relevant to the request!**

| File | Description | When to Read |
|------|-------------|--------------|
| `database-selection.md` | PostgreSQL vs Neon vs Turso vs SQLite | Choosing database |
| `orm-selection.md` | Drizzle vs Prisma vs Kysely | Choosing ORM |
| `schema-design.md` | Normalization, PKs, relationships | Designing schema |
| `indexing.md` | Index types, composite indexes | Performance tuning |
| `optimization.md` | N+1, EXPLAIN ANALYZE | Query optimization |
| `migrations.md` | Safe migrations, serverless DBs | Schema changes |

---

## Core Principle

- ASK user for database preferences when unclear
- Choose database/ORM based on CONTEXT
- Don't default to PostgreSQL for everything

---

## Decision Checklist

Before designing schema:

- [ ] Asked user about database preference?
- [ ] Chosen database for THIS context?
- [ ] Considered deployment environment?
- [ ] Planned index strategy?
- [ ] Defined relationship types?

---

## Anti-Patterns

- Default to PostgreSQL for simple apps (SQLite may suffice)
- Skip indexing
- Use SELECT * in production
- Store JSON when structured data is better
- Ignore N+1 queries

## Common Pitfalls
- Defaulting to PostgreSQL for simple apps (SQLite may suffice)
- Skipping indexing on frequently queried columns
- Using SELECT * in production queries
- Not planning for migration strategies
- Ignoring N+1 query problems
- Choosing complex ORMs when simple queries suffice
- Not considering serverless database constraints

## Best Practices
- ASK user about database preferences when unclear
- Choose database based on deployment context (edge/serverless/traditional)
- Plan indexing strategy early in design
- Use parameterized queries to prevent injection
- Consider migration safety and rollback strategies
- Normalize appropriately but avoid over-normalization
- Test query performance with realistic data volumes
