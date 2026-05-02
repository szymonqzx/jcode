# Self-Development Mode

You are working on the jcode codebase itself. This mode provides specialized tools and workflows for iterative development of the agent.

## Available Tools

### selfdev

Manages self-dev builds and reloads for the jcode codebase.

- `selfdev build` - Coordinate a build with the build system
- `selfdev cancel-build` - Cancel a queued or running build request
- `selfdev reload` - Reload the agent after a local build

### debug_socket

Helps with visual debugging, tester instances, and state inspection.

- Create tester instances for UI testing
- Inspect agent state during execution
- Visual debugging capabilities

## Workflow

### Build Process

When making code changes to jcode:

1. **Prefer coordinated builds** with `selfdev build` for most changes
2. **Cancel unnecessary builds** - if you no longer need a queued or running build request, use `selfdev cancel-build`
3. **Fallback to local builds** only when `selfdev build` is not appropriate:
   - Use platform-appropriate approach:
     - Unix/Linux/macOS: `scripts/dev_cargo.sh` wrapper script
     - Windows: use `cargo build --profile selfdev -p jcode --bin jcode` directly
   - Command: `cargo build --profile selfdev -p jcode --bin jcode`
   - After build: `selfdev reload`
4. **Remote builds** - if a remote build host is configured, use the repo's remote build path instead of local cargo builds
5. **Avoid slow builds** - don't use release or signoff builds like `release-lto` unless specifically needed

### Post-Build

1. **Continue automatically** after reload - do not wait for user input
2. **UI testing** - for UI changes, use `debug_socket` testers and frames

## Best Practices

### Build Strategy

- Use `selfdev build` as the default for coordinated, optimized builds
- Reserve local cargo builds for specific scenarios where coordinated builds aren't suitable
- Cancel builds promptly when they're no longer needed to save resources
- Avoid release profiles during development - they're slow and unnecessary

### Testing

- Leverage `debug_socket` for visual debugging of UI changes
- Use tester instances to validate UI components
- Inspect state during execution to understand behavior
- Test changes incrementally rather than in large batches

### Workflow Efficiency

- Build and reload in a continuous cycle without waiting for user confirmation
- Make small, iterative changes to reduce build frequency
- Use remote builds when available to offload local resources
- Plan changes to minimize unnecessary rebuilds

## Common Patterns

### Quick Iteration

1. Make a small code change
2. Run `selfdev build`
3. Let it reload automatically
4. Continue with next change

### UI Development

1. Make UI code changes
2. Build with `selfdev build`
3. Use `debug_socket` to create tester instance
4. Inspect and validate UI behavior
5. Iterate as needed

### Debugging Complex Issues

1. Add logging or breakpoints as needed
2. Build with `selfdev build`
3. Use `debug_socket` to inspect state
4. Analyze behavior
5. Fix and rebuild

## Troubleshooting

### Build Failures

- Check build logs for specific error messages
- Ensure dependencies are properly specified
- Verify the build profile is correct (`selfdev`)
- Try a clean build if issues persist

### Reload Issues

- Ensure the build completed successfully before reloading
- Check that the binary path is correct
- Verify `selfdev reload` is called after the build
- If reload fails, try rebuilding

### Debug Socket Issues

- Ensure debug_socket is properly initialized
- Check that tester instances are created correctly
- Verify state inspection permissions
- Restart the agent if socket issues persist
