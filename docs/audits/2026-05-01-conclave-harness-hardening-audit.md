# QA Audit Result: Conclave Harness Hardening

Date: 2026-05-01
Status: Complete
Source audit prompt: https://rentry.co/4gaw6c6y
Repo: `/home/sacred/code/conclave`
Commit under audit: `145e7b62`

## Verdict

PASS WITH RISKS.

The implementation is not fake-complete: durable run records, context packets,
budget checks, gate result records, artifact reviews, task records, swarm bridge
records, idempotency replay, projection repair, TUI inspector views, and
hash-chain replay all have real storage and smoke-tested behavior.

The remaining risks are mostly scope-completeness and operator-truth issues:
task decomposition has only create/list/show/assign-run commands despite a
larger status model, gates record manual outcomes but do not execute their
stored commands or ship built-in reusable templates, and worktree support records
an existing git target rather than creating/cleaning an isolated run worktree.

## Executive Summary

- The existing room model was preserved. Scratch `CONCLAVE_HOME` runs created
  file-backed rooms, legacy `jcode conclave ...` commands still worked, and
  `replay-inspect --json` kept reporting a valid event order/hash chain.
- New durable surfaces are mostly real. The full smoke room wrote
  `runs.jsonl`, per-run `run.json`, `context-packets.jsonl`, context markdown,
  `gates.jsonl`, `gate-runs.jsonl`, per-run `gate-results.jsonl`,
  `artifact-reviews.jsonl`, `artifact-index.json`, `tasks.jsonl`,
  `swarm-bridges.jsonl`, and `idempotency.jsonl`.
- Budget enforcement works before run creation for `max_runs` and
  `max_parallel_runs`; the second run was blocked before a second run record was
  written.
- The TUI side has explicit protocol-backed views for runs, gates, budget,
  failures, next actions, context packets, swarm bridge records, and reviews.
- The biggest PRD gap is lifecycle depth: runs have a workable lifecycle, but
  tasks, gates, and worktrees are narrower than the audit prompt expects.

## Major Gaps

### P0

- None found.

### P1

- Task lifecycle is only partially implemented.
  - Evidence: `ConclaveTaskStatus` defines `running`, `blocked`, `completed`,
    and `cancelled` states in `src/conclave/task.rs`, but the CLI only exposes
    `add`, `list`, `show`, and `assign-run` in `src/cli/args.rs`.
  - Behavioral result: the audit prompt's `task start`, `task block`, and
    `task complete` flows cannot be executed. The only state transition tested
    was `open` to `assigned` through `task assign-run`.
  - Recommended fix: add task lifecycle commands and event types for start,
    block, complete, and cancel, or document the current task model as
    assignment-only and remove lifecycle claims.

### P2

- Gate definitions are durable, but command gates are not executable gates.
  - Evidence: `gate define --command` stores command text, and `gate run`
    records a supplied status/summary/limitation. I found no execution path that
    runs the stored command and captures its exit code/output.
  - Behavioral result: a `qa_gate` with command `cargo check` was recorded, then
    a passing result was manually recorded with limitation `did not execute
    command`.
  - Recommended fix: either add an explicit executable-gate runner with
    truncation/capability policy, or rename/docs-scope this as manual gate result
    recording.

- No built-in reusable gate registry was visible from a fresh room.
  - Evidence: `gate list` on a fresh scratch room returned no definitions; the
    only registry behavior verified was operator-defined gates.
  - Recommended fix: seed documented templates such as `cargo_check`,
    `cargo_fmt_check`, `replay_hash_chain`, `dirty_git_state`, and
    `secret_scan_basic`, or explicitly defer built-ins.

- Worktree execution target records an existing git repo/worktree, but does not
  create or clean a run-scoped worktree.
  - Evidence: `run create --worktree <path>` captured `base_commit`,
    `base_branch`, and `dirty_at_start`; `run worktree-close` recorded a final
    diff hash. The command requires an existing directory and there is no
    `run diff` or cleanup command beyond closeout status.
  - Behavioral result: the smoke confirmed base commit and diff hash capture,
    and active runs cannot share the same path. It did not create an isolated
    path or clean it.
  - Recommended fix: either implement managed worktree creation/diff/cleanup or
    document this as "external worktree target capture".

- Artifact review transitions are append-only latest-status updates without a
  transition guard.
  - Evidence: an accepted artifact could later be rejected; `artifact-index.json`
    simply reflected the latest status.
  - Recommended fix: define allowed status transitions, or document "latest
    review wins" as the intended review model.

- Budget visibility has TUI/protocol coverage but no standalone CLI `budget`
  command.
  - Evidence: `conclave budget --help` fails, while the server inspector has a
    `Budget` control view derived from durable run records and contract limits.
  - Recommended fix: add `conclave budget show/usage` if CLI budget visibility
    is part of the PRD, or keep the requirement TUI-only in docs.

### P3

- Standalone `conclave run --help` is still the generic one-shot run help, while
  durable subcommands such as `conclave run create --help` are normalized into
  Conclave room-run commands.
  - Evidence: `conclave run --help` says "Run a single message and exit";
    `conclave run create --help` shows "Create a durable run record".
  - Recommended fix: add a short hint in generic `run --help` pointing durable
    operators to `conclave run create` or `conclave room run`.

## PRD Alignment Matrix

- Backward compatibility: PASS.
  - Evidence: scratch `conclave room init/state/doctor/inspect/replay-inspect`
    worked; legacy `jcode conclave init/state` worked.

- Event journal and hash chain: PASS WITH RISKS.
  - Evidence: new durable operations appended event types including
    `run_queued`, `context_packet_recorded`, `run_context_linked`,
    `artifact_review_recorded`, `gate_defined`, `run_gate_passed`,
    `task_recorded`, `task_assigned`, `swarm_bridge_bound`, `run_summarized`,
    `run_retried`, and `replay_inspected`.
  - Risk: some run side-effects use generic `projection_written` rather than
    more semantic event names.

- Run records: PASS.
  - Evidence: `run create/list/show/context/summarize/retry/cancel` worked in
    scratch rooms. Idempotency replay returned the original run id.

- Context packets: PASS.
  - Evidence: `run context` wrote `context-packets.jsonl`,
    `context/<id>.json`, and `context/<id>.md`, linked the packet back onto the
    run, included source event range and budget usage, and used artifact review
    status to prefer accepted artifacts.

- Budgets: PASS WITH RISKS.
  - Evidence: a contract with `max_runs: 1` and `max_parallel_runs: 1` blocked
    a second run before any second run record was written.
  - Risk: token/cost enforcement is limited to recorded usage and some fields
    explicitly warn that they cannot be enforced before provider usage exists.

- Gates: PARTIAL.
  - Evidence: definitions and run results are durable and evented.
  - Risk: no executable command runner or built-in gate templates were verified.

- Artifact review: PASS WITH RISKS.
  - Evidence: reviews are durable, evented, indexed, and context packets respect
    accepted/rejected/superseded status.
  - Risk: no transition guard beyond latest-status projection.

- Worktree isolation: PARTIAL.
  - Evidence: existing git target capture records base commit, branch,
    dirty-at-start, final diff hash, and blocks simultaneous active reuse.
  - Risk: no managed worktree creation/cleanup.

- Task decomposition: PARTIAL.
  - Evidence: durable tasks include expected artifacts, gates, path bounds,
    dependencies, reviewer, assigned run ids, duplicate fingerprint, and
    assignment behavior.
  - Risk: no start/block/complete/cancel lifecycle commands.

- Room to swarm bridge: PASS.
  - Evidence: `swarm bind` wrote a bridge with session ids, role mapping, active
    state, and `record_raw_messages: false` by default.

- TUI inspector: PASS.
  - Evidence: slash commands and protocol/server handlers exist for room runs,
    run detail, gates, budget, failures, next actions, context, swarm, and
    reviews; server budget view derives usage from durable runs.

- Idempotency and repair: PASS.
  - Evidence: duplicate `run create` with the same idempotency key returned the
    original result id without creating a second run. Repair dry-run left a
    corrupted `tasks.jsonl` unchanged; write mode created a backup and repaired
    the JSONL file.

- Security/privacy: PASS WITH RISKS.
  - Evidence: context packets link ids and summaries rather than dumping raw room
    history, and swarm bridge defaults to not recording raw messages.
  - Risk: executable gates are not implemented yet, so their future capability
    policy still needs a dedicated review.

- Docs/operator truth: PASS WITH RISKS.
  - Evidence: docs describe the current command set reasonably well, including
    the side-panel inspector being intentionally not a full tabbed UI.
  - Risk: `conclave run --help` by itself points at the generic agent runner,
    which can confuse operators looking for durable run-record help.

- Tests: PASS.
  - Evidence: `cargo fmt --check`, `git diff --check`, and
    `cargo check -q --no-default-features` passed before the deeper test pass.
    `cargo test conclave`, `cargo test conclave_inspector`, and
    `bash scripts/conclave_qa_smoke.sh` passed.

## Command Results

- `node /home/sacred/code/apex-workflow/scripts/check-config.mjs --config=apex.workflow.json --target=.`: PASS.
- `node /home/sacred/code/apex-workflow/scripts/apex-doctor.mjs --config=apex.workflow.json --target=.`: READY with two non-blocking warnings.
- `cargo fmt --check`: PASS.
- `git diff --check`: PASS.
- `CARGO_BUILD_JOBS=1 cargo check -q --no-default-features`: PASS.
- `cargo test conclave`: PASS, 178 matching tests passed.
- `cargo test conclave_inspector`: PASS, 24 matching tests passed.
- `bash scripts/conclave_qa_smoke.sh`: PASS.
- `conclave --version`: `v0.10.571-dev (145e7b62)`.
- `jcode --version`: `v0.10.571-dev (145e7b62)`.
- Scratch full durable-surface smoke: PASS.
  - Room: `qa-full`.
  - Run id: `run-0ab83d01-1c5e-4e5f-af2d-c7a431b3d8f7`.
  - Retry id: `run-6f1c6d3a-067f-4855-af09-cdf62e323b35`.
  - Context packet id: `context-28d47832-1034-41d3-af03-a43af025c3f8`.
  - Event types:
    `room_created,run_queued,context_packet_recorded,run_context_linked,artifact_recorded,artifact_review_recorded,gate_defined,run_gate_passed,projection_written,task_recorded,projection_written,task_assigned,swarm_bridge_bound,run_summarized,run_retried,replay_inspected`.
- Budget smoke: PASS.
  - First run succeeded.
  - Second run failed before creation with:
    `Conclave run budget exceeded: max_runs=1, current_runs=1; Conclave run budget exceeded: max_parallel_runs=1, active_runs=1`.
- Worktree smoke: PASS WITH RISKS.
  - Recorded base commit and final diff hash for an existing git repo path.
  - Blocked a second active run from reusing the same path.
- Repair smoke: PASS.
  - Dry-run reported the bad JSONL line and did not mutate the file.
  - Write mode created `tasks.jsonl.repair.bak` and repaired the projection.
- Large-room smoke: PASS WITH RISKS.
  - 1500 `claim add` operations wrote 1500 claim records and 1502 events in
    45.298s.
  - `room inspect` completed in 358ms.
  - `room replay-inspect` completed in 146ms.
  - `packet --type audit` refused in 79ms before required gates passed:
    `contract`.

## File-Level Findings

- `src/conclave/event.rs`: new event types exist for runs, context packets,
  gates, artifact reviews, tasks, and swarm bridge records.
- `src/conclave/storage.rs`: durable record helpers append both room events and
  projection JSONL files for runs, context packets, gates, gate runs, artifact
  reviews, tasks, and swarm bridge records.
- `src/conclave/budget.rs`: budget usage is derived from latest run records;
  `max_runs`, `max_parallel_runs`, total token budget, spend budget, retry
  budget, and repeated-failure stop checks exist.
- `src/cli/commands/conclave.rs`: run creation enforces budget before writing,
  idempotency is checked before durable mutation, context packets are linked to
  runs, and worktree closeout records final diff hash.
- `src/cli/args.rs`: task commands stop at add/list/show/assign-run, while the
  task model includes additional lifecycle states.
- `src/server/conclave_room_inspector.rs` and `src/tui/conclave_inspector.rs`:
  TUI/protocol visibility exists for the new run/gate/budget/failure/context
  surfaces.

## Behavioral Findings

- Idempotency replay is real for run creation: the second request returned the
  stored idempotency record instead of appending a duplicate run.
- Projection repair is conservative: dry-run does not mutate, write mode backs up
  before rewriting, and corrupt JSONL warnings include file and line number.
- Context packets do not dump full artifacts or room history by default; they
  include ids, counts, provenance range, warnings, and budget usage.
- Swarm bridge defaults avoid raw live-chat persistence.

## Security/Safety Findings

- No raw hidden reasoning path was found in the new context/surface records.
- No destructive default cleanup was found for worktrees; the risk is the
  opposite: cleanup is only a recorded status, not actual managed cleanup.
- Future executable gates need a separate safety/capability review if implemented
  because command execution is currently only metadata plus manual result
  recording.

## Performance Findings

- The 1500-claim command-loop performance check completed successfully. The
  write loop was dominated by process startup across 1500 CLI invocations
  (45.298s total), but inspector/replay reads stayed responsive:
  `room inspect` 358ms and `room replay-inspect` 146ms.
- Context/audit packet creation did not create an invalid large packet; it
  refused quickly because the room lacked passing required gates.
- Existing inspector/protocol code uses lossy JSONL reads and latest-run
  reduction for several views, which is a reasonable current-scale pattern.

## Recommended Fix Plan

1. Add or explicitly defer task lifecycle commands and events:
   `task start`, `task block`, `task complete`, `task cancel`.
2. Decide whether gate definitions are meant to be executable. If yes, implement
   a safe command runner with truncation, receipts, and capability policy. If no,
   rename/docs-scope them as manual gate-result records.
3. Decide whether worktree support must manage isolated worktrees. If yes, add
   create/diff/cleanup commands with safe refusal on unrecorded diffs. If no,
   document the feature as existing worktree target capture.
4. Add CLI budget visibility or document that budget visibility is TUI-only.
5. Add a durable-run hint to generic `conclave run --help`.
6. Add artifact review transition rules if accepted-to-rejected latest-wins is
   not intentional.

## Final Recommendation

Do not revert. Keep the implementation, but do not call the hardening PRD fully
complete until the task lifecycle, gate execution/template scope, and worktree
scope are explicitly resolved. The current implementation is useful and
operator-testable, but it is narrower than the full audit prompt.
