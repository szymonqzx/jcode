---
status: completed
created: 2026-05-03
completed: 2026-05-03
---

# TEAM_001 - Build Time Optimization

## Task
Implement crate seam refactoring to improve jcode project build times through:
1. Types crate consolidation (13→3 crates)
2. Parallel feature profiles (light/standard/full)
3. Build tooling recommendations (sccache)
4. Optional binary split architecture evaluation

## Progress
- [x] Team registration
- [x] Baseline build verification
- [x] Phase 1: Document sccache + optimize fast-dev profile
- [x] Phase 2: Consolidate types crates (13→4)
- [x] Phase 3: Create feature unions (light/standard/full)
- [x] Phase 4: Evaluate binary split (documented)
- [x] Final verification and commit
- [x] All consolidated crates build successfully

## Final Crate Consolidation (13→4)

**New consolidated crates:**
- `jcode-base-types`: message, auth, config, gateway, background, side-panel, usage
- `jcode-session-types`: session, memory (expanded existing crate)
- `jcode-workflow-types`: task, ambient, batch
- `jcode-selfdev-types`: kept separate (requires chrono + anyhow)

**Feature profiles added:**
- `light`: Core only, fastest builds for testing business logic
- `standard`: Core + essential TUI, good for UI work
- `full`: Everything including embeddings/PDF (default)

**Build tooling:**
- Updated `.cargo/config.toml` with clearer sccache documentation
- `fast-dev` profile already exists with 512 codegen units

## Handoff Notes
Task completed. Legacy types crates remain in workspace for backward compatibility during migration period. Future work could migrate dependent crates to use consolidated types directly, then remove legacy crates.

**Commits:**
- d4ffdb5b: Consolidate types crates
- d4fdbd75: Add feature profiles
