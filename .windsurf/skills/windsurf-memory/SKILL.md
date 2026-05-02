---
name: windsurf-memory
description: Built-in Windsurf memory system for persistent context and knowledge management
---

# Windsurf Memory Usage

## When to Use
- Storing project-specific context for future sessions
- Saving user preferences and configuration patterns
- Preserving important code snippets and patterns
- Tracking architectural decisions and project structure
- Managing team workflows and coordination
- Creating persistent knowledge bases across conversations

## Key Patterns

### Creating Memories
```markdown
Use the create_memory tool to save context:
- Title: Short descriptive title
- Content: Detailed information to persist
- Tags: Array of snake_case tags for filtering
- CorpusNames: Workspace identifiers for scoping
- Action: "create", "update", or "delete"
```

### Memory Types
- **Global rules**: System-wide rules that always apply
- **User-provided memories**: Explicit context provided by user
- **System-retrieved memories**: Auto-retrieved from previous conversations

### Memory Lifecycle
1. Create memory when encountering important context
2. Update existing memories instead of creating duplicates
3. Check relevance before using retrieved memories
4. Delete incorrect or outdated memories

### Best Practices for Memory Content
- Be specific and actionable
- Include file paths, function names, and concrete examples
- Use code blocks for technical patterns
- Structure with clear headings and sections
- Add tags for easy retrieval

### Memory Search and Retrieval
- Memories are automatically retrieved based on relevance
- Always verify relevance before using
- System may retrieve stale or incorrect information
- Cross-reference with current workspace state

## Common Pitfalls
- Creating duplicate memories instead of updating existing ones
- Storing stale or outdated information without verification
- Overloading memory with trivial details
- Not verifying relevance of auto-retrieved memories
- Forgetting to scope memories to correct CorpusNames
- Not tagging memories properly for retrieval

## Best Practices
- Check for semantically related memories before creating new ones
- Use descriptive titles and comprehensive tags
- Scope memories to specific workspaces using CorpusNames
- Update memories when information changes
- Delete memories that are no longer accurate
- Use memories for cross-session context preservation
- Keep memory content focused and actionable

## Memory Management Workflow
1. **Before creating**: Search for existing related memories
2. **When creating**: Use clear titles, comprehensive tags, proper scoping
3. **When updating**: Modify existing memory instead of creating duplicate
4. **When using**: Verify relevance and accuracy with current state
5. **When deleting**: Remove outdated or incorrect memories

## Related Workflows
- `../workflows/evolve.md` - For creating, refactoring, and syncing instruction files

## Related Skills
- `../skills/async-tokio-patterns/SKILL.md` - For async patterns in memory operations
- `../skills/error-handling/SKILL.md` - For handling memory operation errors
