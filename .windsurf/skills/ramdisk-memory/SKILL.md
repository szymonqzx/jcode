---
name: ramdisk-memory
description: Memory management patterns, allocation strategies, and memory-backed storage integration
---

# Memory Management

## When to Use
- Working with RAM disks or memory-backed file systems
- Managing dynamic memory allocation
- Integrating with memory-mapped storage or FSDs
- Monitoring memory usage and limits
- Implementing resource cleanup for memory-intensive operations

## Key Patterns

### Memory Allocation
- Track actual memory usage vs allocated memory
- Implement total memory limits (not per-file)
- Handle allocation failures gracefully
- Use dynamic allocation for unpreallocated writes

### External Process Integration
```rust
// Spawn external process with correct parameters
// Monitor process health
// Handle process cleanup on exit
// Use PID files for process tracking
// Example: memefs.exe for WinFsp, or any memory-backed filesystem
```

### Memory Monitoring
- Implement periodic memory usage checks
- Alert when approaching limits
- Provide graceful degradation
- Log allocation patterns for debugging

### Cleanup Strategies
- Always cleanup on process exit (use scopeguard)
- Handle abnormal termination
- Remove junction points before unmounting
- Verify cleanup succeeded

## Common Pitfalls
- Memory leaks from uncleaned resources or handles
- Orphaned external processes
- Incorrect size calculations
- Not handling out-of-memory scenarios
- Forgetting to clean up on abnormal termination

## Testing
- Test with various memory limits
- Simulate out-of-memory conditions
- Verify cleanup in all exit scenarios
- Test with large file operations

## Related Workflows
- `../workflows/debug.md` - For detecting memory leaks in RAM disk operations
- `../skills/code-review-checklist/SKILL.md` - For reviewing memory management code
