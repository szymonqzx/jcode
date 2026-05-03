---
status: active
created: 2026-05-03
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
- [ ] Phase 1: Document sccache + optimize fast-dev profile
- [ ] Phase 2: Consolidate types crates
- [ ] Phase 3: Create feature unions
- [ ] Phase 4: Evaluate binary split (if needed)
- [ ] Final verification and commit

## Decisions
- **Types crate grouping strategy:**
  - `jcode-base-types`: Core types (message, session, memory, config)
  - `jcode-task-types`: Task/plan/batch related types
  - `jcode-ui-types`: UI/TUI/side-panel related types
  
- **Feature profiles:**
  - `light`: Core functionality, no TUI/embeddings/PDF
  - `standard`: Core + essential TUI (default for dev)
  - `full`: Everything (current default)

- **Keep jcode-core separate**: It's already a lean foundation crate

## Handoff Notes
N/A - active task
