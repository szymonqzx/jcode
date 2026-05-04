---
status: active
created: 2026-05-03
---

# TEAM_001 - Code Fix Loop

## Task
Fix all issues identified in the comprehensive code review: duplicate Cargo.toml entry,
unchecked setsid() return, bare std::env calls in tests, unreachable!() panics, silenced errors.

## Progress
- [ ] Fix duplicate jcode-task-types in Cargo.toml
- [ ] Fix setsid() return not checked in server/socket.rs
- [ ] Fix bare std::env::set_var/remove_var in test
- [ ] Fix unreachable!() on AuthMethod::Unknown (2 sites)
- [ ] Fix unreachable!() in auth_account_picker.rs
- [ ] Fix silenced session.save() errors in crash.rs (3 sites)
- [ ] cargo check passes

## Decisions
- Replace unreachable!() with graceful no-ops (return/continue) rather than panics
- Use crate::env::* for all env mutation per project convention
- Log silenced save errors via crate::logging::warn

## Handoff Notes
All fixes are minimal, targeted, and non-breaking.
