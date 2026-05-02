---
description: GitHub MCP server usage patterns and best practices for GitHub API operations
---

# GitHub MCP

## When to Use
- Interacting with GitHub repositories (issues, pull requests, code search)
- Creating or managing GitHub issues and pull requests
- Searching repositories, code, or issues across GitHub
- Retrieving repository metadata and file contents
- Managing pull request reviews and comments

## Key Patterns

### Repository Operations

#### Get Repository Details
```python
# Use mcp3_github_get_repository for basic repo info
# Parameters: owner, repo
```

#### Get File Content from Repository
```python
# Use mcp0_repository_get_file_content (GitKraken MCP)
# Parameters: provider, repository_name, repository_organization, ref, file_path
# Supports GitHub, GitLab, Bitbucket, Azure
```

### Issue Management

#### List Issues
```python
# Use mcp3_github_list_issues (GitHub MCP)
# Parameters: owner, repo, state (open/closed/all), limit, offset

# Use mcp0_issues_assigned_to_me (GitKraken MCP)
# Parameters: provider (github/gitlab/jira/azure/linear), page
# For fetching issues assigned to current user
```

#### Get Issue Details
```python
# Use mcp0_issues_get_detail (GitKraken MCP)
# Parameters: provider, issue_id, repository_name, repository_organization (for GitHub/GitLab)
# Supports GitHub/GitLab/Jira/Azure/Linear
```

#### Create Issue
```python
# Use mcp3_github_create_issue (GitHub MCP)
# Parameters: owner, repo, title, body
```

#### Add Comment to Issue
```python
# Use mcp0_issues_add_comment (GitKraken MCP)
# Parameters: provider, issue_id, comment, repository_name, repository_organization (for GitHub/GitLab)
```

### Pull Request Operations

#### List Pull Requests
```python
# Use mcp3_github_list_pull_requests (GitHub MCP)
# Parameters: owner, repo, state (open/closed/all), limit, offset

# Use mcp0_pull_request_assigned_to_me (GitKraken MCP)
# Parameters: provider, reviewer (include if reviewer), repository_name, repository_organization (for Azure/Bitbucket)
```

#### Get Pull Request Details
```python
# Use mcp3_github_get_pull_request (GitHub MCP)
# Parameters: owner, repo, pull_number

# Use mcp0_pull_request_get_detail (GitKraken MCP)
# Parameters: provider, pull_request_id, repository_name, repository_organization, pull_request_files (optional)
# Supports GitHub/GitLab/Bitbucket/Azure
```

#### Create Pull Request
```python
# Use mcp0_pull_request_create (GitKraken MCP)
# Parameters: provider, repository_name, repository_organization, title, source_branch, target_branch, body (optional), is_draft (optional)
# Supports GitHub/GitLab/Bitbucket/Azure
```

#### Create Pull Request Review
```python
# Use mcp0_pull_request_create_review (GitKraken MCP)
# Parameters: provider, pull_request_id, repository_name, repository_organization, review, approve (optional)
```

#### Get Pull Request Comments
```python
# Use mcp0_pull_request_get_comments (GitKraken MCP)
# Parameters: provider, pull_request_id, repository_name, repository_organization
```

### Search Operations

#### Search Code
```python
# Use mcp3_github_search_code (GitHub MCP)
# Parameters: query, limit, offset
# Query syntax: q=language:rust+example
```

#### Search Issues and Pull Requests
```python
# Use mcp3_github_search_issues_and_prs (GitHub MCP)
# Parameters: query, limit, offset
# Query syntax: q=repo:owner/name+is:issue+state:open
```

#### Search Repositories
```python
# Use mcp3_github_search_repositories (GitHub MCP)
# Parameters: query, limit, offset
# Query syntax: q=language:rust+stars:>100
```

## Common Pitfalls

- **Wrong MCP server**: Using GitHub MCP when GitKraken MCP has broader provider support (GitLab, Azure, Jira, Linear)
- **Missing parameters**: Forgetting repository_name and repository_organization for GitKraken MCP GitHub operations
- **Provider confusion**: Not specifying correct provider parameter in GitKraken MCP tools
- **Query syntax**: Not using proper GitHub search query syntax (q= parameter)
- **Pagination**: Not handling limit/offset parameters for large result sets

## Best Practices

- **Prefer GitKraken MCP** for multi-provider support (GitHub, GitLab, Bitbucket, Azure, Jira, Linear)
- **Use GitHub MCP** for GitHub-specific operations or when provider is always GitHub
- **Check provider support**: Verify the tool supports your git provider before using
- **Handle pagination**: Use limit and offset parameters for large datasets
- **Cache results**: Repository metadata rarely changes, consider caching
- **Error handling**: Always check for API rate limits and authentication errors
- **Use search wisely**: Construct specific queries to reduce result sets

## Tool Selection Guide

| Task | GitHub MCP | GitKraken MCP |
|------|------------|---------------|
| Basic repo info | ✅ get_repository | ❌ |
| File content | ❌ | ✅ repository_get_file_content |
| List issues | ✅ list_issues | ✅ issues_assigned_to_me |
| Issue details | ❌ | ✅ issues_get_detail |
| Create issue | ✅ create_issue | ❌ |
| Add comment | ❌ | ✅ issues_add_comment |
| List PRs | ✅ list_pull_requests | ✅ pull_request_assigned_to_me |
| PR details | ✅ get_pull_request | ✅ pull_request_get_detail |
| Create PR | ❌ | ✅ pull_request_create |
| PR review | ❌ | ✅ pull_request_create_review |
| PR comments | ❌ | ✅ pull_request_get_comments |
| Search code | ✅ search_code | ❌ |
| Search issues/PRs | ✅ search_issues_and_prs | ❌ |
| Search repos | ✅ search_repositories | ❌ |

## Related Workflows
- ../workflows/code-fix-loop.md
- ../workflows/implement.md

## Related Skills
- clean-code
- architecture
