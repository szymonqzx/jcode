---
description: Create, refactor, sync, and refine instruction files (skills, workflows, rules) to evolve the instruction ecosystem
---

# Evolve Instructions

Comprehensive workflow for creating new instruction files and evolving existing ones. Combines template generation, quality refactoring, cross-reference syncing, and excellence refinement.

## Overview

This workflow provides four modes for instruction evolution:

1. **Create mode** - Generate new skills/workflows with proper templates and structure
2. **Refactor mode** - Fix quality issues (missing sections, structure, clarity)
3. **Sync mode** - Ensure cross-references are valid and ecosystem is consistent
4. **Refine mode** - Elevate workflows from functional to excellent

## Sub-commands

```text
/evolve create     - Create new skills or workflows with templates
/evolve refactor    - Fix quality issues in existing instructions
/evolve sync        - Sync cross-references and ecosystem consistency
/evolve refine      - Elevate workflows to excellence (premium quality)
```

## When to Use

- Creating new workflows for recurring tasks
- Adding new skills for domain-specific knowledge
- Improving instruction file quality and clarity
- Restructuring skills for better organization
- Consolidating redundant instruction content
- Updating outdated instruction patterns
- Syncing cross-references and navigation

## When NOT to Use

- For code refactoring (use code-fix-loop instead)
- For code documentation sync (use documentation-sync-loop instead)
- For trivial one-off tasks (use inline comments instead)

## Configuration Enforcement

This workflow respects your current configuration:

- **Instruction directories**: Uses your project's instruction directory structure (e.g., `.windsurf/`, `.agents/`, etc.)
- **Rules file**: Respects your project's rules file location
- **Naming conventions**: kebab-case for all filenames
- **Structure**: Follows your established template patterns

**Note**: This workflow is designed to be directory-agnostic. It will work with any instruction directory structure by detecting the appropriate paths automatically.

## Mode 1: Create

### Pre-flight Checks

```powershell
# Detect instruction directory structure
$instructionDirs = @(".windsurf", ".agents")
$baseDir = $instructionDirs | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $baseDir) {
    Write-Error "No instruction directory found (.windsurf or .agents)"
    exit 1
}

Write-Host "Using instruction directory: $baseDir"

# Ensure subdirectories exist
if (-not (Test-Path "$baseDir/workflows")) {
    New-Item -ItemType Directory -Force -Path "$baseDir/workflows"
}
if (-not (Test-Path "$baseDir/skills")) {
    New-Item -ItemType Directory -Force -Path "$baseDir/skills"
}

# List existing instruction files
$existingWorkflows = Get-ChildItem "$baseDir/workflows" -Filter *.md -ErrorAction SilentlyContinue
$existingSkills = Get-ChildItem "$baseDir/skills" -Filter *.md -Recurse -ErrorAction SilentlyContinue

Write-Host "Found $($existingWorkflows.Count) existing workflows"
Write-Host "Found $($existingSkills.Count) existing skills"
```

### Workflow Template

```powershell
function New-WorkflowTemplate {
    param(
        [string]$Name,
        [string]$Description,
        [string]$Overview,
        [string[]]$RelatedSkills = @()
    )

    $skillReferences = if ($RelatedSkills.Count -gt 0) {
        $RelatedSkills | ForEach-Object { "  - `"$_`"" } | Out-String
    } else {
        "  # Add related skills here"
    }

    $template = @"
---
description: $Description
---

# $Name

$Overview

## Overview

[Brief description]

## When to Use
- [Condition 1]
- [Condition 2]

## When NOT to Use
- [Condition 1]
- [Condition 2]

## Pre-flight Checks

\`\`\`powershell
# Add checks
\`\`\`

## Loop Configuration

\`\`\`powershell
\$MAX_ITERS = 10
\$KILLSWITCH = "\$env:USERPROFILE\.workflow-stop"
\$LOGDIR = ".workflow-logs/\$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path \$LOGDIR | Out-Null
\`\`\`

## The Loop

\`\`\`powershell
for (\$i = 1; \$i -le \$MAX_ITERS; \$i++) {
    if (Test-Path \$KILLSWITCH) { exit 2 }
    # Add workflow logic
}
\`\`\`

## Guardrails
1. Guardrail 1 - Description
2. Killswitch file - Create \`~/.workflow-stop\`
3. Iteration cap - Default 10

## Related Skills
$skillReferences

## Credits
- Reference 1
- Reference 2

## Related Workflows
- [List]
"@
    return $template
}
```

### Skill Template

```powershell
function New-SkillTemplate {
    param(
        [string]$Name,
        [string]$Description,
        [string]$Topic,
        [string[]]$RelatedWorkflows = @()
    )

    $workflowReferences = if ($RelatedWorkflows.Count -gt 0) {
        $RelatedWorkflows | ForEach-Object { "  - \`"../workflows/$_\`"" } | Out-String
    } else {
        "  # Add related workflows here"
    }

    $template = @"
---
description: $Description
---

# $Name

## When to Use
- [Condition 1]
- [Condition 2]

## Key Patterns

### Pattern 1
\`\`\`rust
// Example
\`\`\`

## Common Pitfalls
- [Pitfall 1]

## Best Practices
- [Best practice 1]

## Related Workflows
$workflowReferences

## Related Skills
- [List]
"@
    return $template
}
```

### Interactive Creation

```powershell
function Add-NewInstruction {
    $type = Read-Host "Create (W)orkflow or (S)kill?"
    if ($type -notmatch "^[WS]$") { return }

    $name = Read-Host "Name (kebab-case)"
    if ($name -notmatch "^[a-z0-9-]+$") { return }

    $description = Read-Host "Short description"
    $overview = Read-Host "Overview/purpose"

    if ($type -eq "W") {
        $filename = "$baseDir/workflows/$name.md"
        if (Test-Path $filename) { return }
        $template = New-WorkflowTemplate -Name $name -Description $description -Overview $overview
        $template | Out-File -FilePath $filename -Encoding UTF8
    } else {
        $skillDir = Read-Host "Skill directory name"
        $skillPath = "$baseDir/skills/$skillDir"
        $filename = "$skillPath/skill.md"
        if (Test-Path $filename) { return }
        New-Item -ItemType Directory -Force -Path $skillPath | Out-Null
        $template = New-SkillTemplate -Name $name -Description $description -Topic $overview
        $template | Out-File -FilePath $filename -Encoding UTF8
    }

    if ((Read-Host "Add cross-references? (Y/N)") -eq "Y") {
        Update-CrossReferences -Type $type -Name $name
    }
}
```

### Cross-Reference Management

```powershell
function Update-CrossReferences {
    param([string]$Type, [string]$Name)

    if ($type -eq "W") {
        $skills = Get-ChildItem "$baseDir/skills" -Filter *.md -Recurse
        $selectedSkills = Read-Host "Skill names (comma-separated)"
        if ($selectedSkills) {
            $workflowFile = "$baseDir/workflows/$Name.md"
            $content = Get-Content $workflowFile -Raw
            $skillList = $selectedSkills -split ',' | ForEach-Object { "  - `"$_`"" } | Out-String
            $content = $content -replace "  # Add related skills here", $skillList.Trim()
            $content | Out-File -FilePath $workflowFile -Encoding UTF8
        }
    } else {
        $workflows = Get-ChildItem "$baseDir/workflows" -Filter *.md
        $selectedWorkflows = Read-Host "Workflow names (comma-separated)"
        if ($selectedWorkflows) {
            $skillFile = "$baseDir/skills/*/skill.md" | Where-Object { (Get-Content $_ -Raw) -match "# $Name" }
            if ($skillFile) {
                $content = Get-Content $skillFile -Raw
                $workflowList = $selectedWorkflows -split ',' | ForEach-Object { "  - \`"../workflows/$_.md\`"" } | Out-String
                $content = $content -replace "  # Add related workflows here", $workflowList.Trim()
                $content | Out-File -FilePath $skillFile -Encoding UTF8
            }
        }
    }
}
```

## Mode 2: Refactor

### Pre-flight Safety Checks

```powershell
# Detect instruction directory
$instructionDirs = @(".windsurf", ".agents")
$baseDir = $instructionDirs | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $baseDir) {
    Write-Error "No instruction directory found (.windsurf or .agents)"
    exit 1
}

# Optional: Create backup before running
$BACKUP_DIR = ".evolve-backup/$(Get-Date -Format 'yyyyMMddHHmmss')"
Copy-Item -Recurse -Force $baseDir $BACKUP_DIR
Write-Host "Backup created at: $BACKUP_DIR"
```

### Loop Configuration

```powershell
$MAX_ITERS = 15
$KILLSWITCH = "$env:USERPROFILE\.evolve-stop"
$LOGDIR = ".evolve-logs/$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null
```

### Instruction Quality Checks

```powershell
function Test-SkillQuality {
    $issues = @()
    Get-ChildItem "$baseDir/skills" -Filter *.md -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if (-not ($content -match "^---")) { $issues += "$($_.FullName): Missing YAML frontmatter" }
        if (-not ($content -match "description:")) { $issues += "$($_.FullName): Missing description" }
        if (-not ($content -match "## When to Use")) { $issues += "$($_.FullName): Missing 'When to Use'" }
        if (-not ($content -match "## (Common Pitfalls|Best Practices)")) { $issues += "$($_.FullName): Should include pitfalls or practices" }
        if (-not ($content -match "```")) { $issues += "$($_.FullName): Missing code examples" }
    }
    return $issues
}

function Test-WorkflowQuality {
    $issues = @()
    Get-ChildItem "$baseDir/workflows" -Filter *.md | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if (-not ($content -match "^---")) { $issues += "$($_.FullName): Missing YAML frontmatter" }
        if (-not ($content -match "description:")) { $issues += "$($_.FullName): Missing description" }
        if (-not ($content -match "## When to Use")) { $issues += "$($_.FullName): Missing 'When to Use'" }
        if (-not ($content -match "## When NOT to Use")) { $issues += "$($_.FullName): Missing 'When NOT to Use'" }
        if (-not ($content -match "## Guardrails")) { $issues += "$($_.FullName): Missing 'Guardrails'" }
    }
    return $issues
}

function Test-CrossReferences {
    $issues = @()
    $skillDirs = Get-ChildItem "$baseDir/skills" -Directory | Select-Object -ExpandProperty Name
    $workflowNames = Get-ChildItem "$baseDir/workflows" -Filter *.md | ForEach-Object { $_.Name -replace '\.md$', '' }

    Get-ChildItem "$baseDir/workflows" -Filter *.md | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if ($content -match "## Related Skills") {
            $matches = [regex]::Matches($content, 'skills/([^/]+)/')
            foreach ($match in $matches) {
                if ($match.Groups[1].Value -notin $skillDirs) {
                    $issues += "$($_.FullName): References non-existent skill"
                }
            }
        }
    }
    return $issues
}
```

### The Loop

```powershell
for ($i = 1; $i -le $MAX_ITERS; $i++) {
    Write-Host "── Iteration $i/$MAX_ITERS ──"

    if (Test-Path $KILLSWITCH) {
        Remove-Item $KILLSWITCH
        exit 2
    }

    $logFile = "$LOGDIR/quality-$i.log"
    $results = @()

    $skillIssues = Test-SkillQuality
    if ($skillIssues) { $results += "FAIL: Skill quality issues"; $results += $skillIssues }
    else { $results += "PASS: All skills meet quality standards" }

    $workflowIssues = Test-WorkflowQuality
    if ($workflowIssues) { $results += "FAIL: Workflow quality issues"; $results += $workflowIssues }
    else { $results += "PASS: All workflows meet quality standards" }

    $xrefIssues = Test-CrossReferences
    if ($xrefIssues) { $results += "FAIL: Cross-reference issues"; $results += $xrefIssues }
    else { $results += "PASS: All cross-references valid" }

    $results | Out-File -FilePath $logFile

    $failures = $results | Where-Object { $_ -like "*FAIL*" }
    if (-not $failures) {
        Write-Host "✓ All quality checks passed at iteration $i"
        exit 0
    }
}
```

## Mode 3: Sync

Sync mode focuses on cross-reference validation and ecosystem consistency:

```powershell
function Sync-InstructionEcosystem {
    # Detect instruction directory
    $instructionDirs = @(".windsurf", ".agents")
    $baseDir = $instructionDirs | Where-Object { Test-Path $_ } | Select-Object -First 1

    if (-not $baseDir) {
        Write-Error "No instruction directory found"
        exit 1
    }

    # Validate all cross-references
    $xrefIssues = Test-CrossReferences


    # Check for orphaned references
    # Update related skills/workflows sections
    # Ensure consistency across instruction files

    if ($xrefIssues) {
        Write-Host "Found cross-reference issues, fixing..."
        # Auto-fix or prompt for manual fixes
    }
}
```

## Mode 4: Refine

Refine mode elevates workflows to excellence by ensuring they meet premium quality standards.

### Excellence Criteria

A workflow is excellent when it includes:

### Comprehensive Examples

- Real-world usage scenarios
- Input/output examples
- Common patterns demonstrated
- Edge case examples

### Clear Edge Case Handling

- Ambiguous requests handled
- Conflicting requirements addressed
- Resource constraints considered
- Environment-specific scenarios covered

### Robust Guardrails

- Killswitch mechanism
- Iteration caps
- Backup requirements
- Logging strategy
- Manual review checkpoints

### Well-Documented Failure Modes

- Common failure scenarios
- Prevention strategies
- Recovery procedures
- Escalation paths

### Performance Considerations

- Execution time expectations
- Resource usage guidelines
- Optimization strategies
- Bottleneck identification

### Security Notes (where applicable)

- Input validation requirements
- Secrets handling
- Access control
- Audit logging
- Vulnerability scanning

### Refinement Process

```powershell
function Refine-Workflows {
    # Detect instruction directory
    $instructionDirs = @(".windsurf", ".agents")
    $baseDir = $instructionDirs | Where-Object { Test-Path $_ } | Select-Object -First 1

    if (-not $baseDir) {
        Write-Error "No instruction directory found"
        exit 1
    }

    $LOGDIR = ".evolve-logs/$(Get-Date -Format 'yyyyMMddHHmmss')"
    New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null

    $workflows = Get-ChildItem "$baseDir/workflows" -Filter *.md
    $refinementReport = @()

    foreach ($workflow in $workflows) {
        $content = Get-Content $workflow.FullName -Raw
        $issues = @()

        # Check for comprehensive examples
        if (-not ($content -match "## Examples")) {
            $issues += "Missing Examples section"
        }

        # Check for edge case handling
        if (-not ($content -match "## Edge Case Handling")) {
            $issues += "Missing Edge Case Handling section"
        }

        # Check for failure modes
        if (-not ($content -match "## Failure Modes")) {
            $issues += "Missing Failure Modes section"
        }

        # Check for performance considerations
        if (-not ($content -match "## Performance Considerations")) {
            $issues += "Missing Performance Considerations section"
        }

        # Check for security notes (if applicable)
        if (-not ($content -match "## Security Notes")) {
            $issues += "Missing Security Notes section"
        }

        # Check for guardrails
        if (-not ($content -match "## Guardrails")) {
            $issues += "Missing Guardrails section"
        }

        if ($issues) {
            $refinementReport += "$($workflow.Name): $($issues -join ', ')"
        } else {
            $refinementReport += "$($workflow.Name): ✓ Excellent"
        }
    }

    $refinementReport | Out-File -FilePath "$LOGDIR/refinement-report.txt"
    $refinementReport | Write-Host
}
```

### Pre-flight Checks for Refine Mode

```powershell
# Detect instruction directory
$instructionDirs = @(".windsurf", ".agents")
$baseDir = $instructionDirs | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $baseDir) {
    Write-Error "No instruction directory found (.windsurf or .agents)"
    exit 1
}

# Create backup before refining
$BACKUP_DIR = ".evolve-backup/$(Get-Date -Format 'yyyyMMddHHmmss')"
Copy-Item -Recurse -Force $baseDir $BACKUP_DIR
Write-Host "Backup created at: $BACKUP_DIR"
```

### Loop Configuration for Refine Mode

```powershell
$MAX_ITERS = 10
$KILLSWITCH = "$env:USERPROFILE\.evolve-stop"
$LOGDIR = ".evolve-logs/$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null
```

### The Loop for Refine Mode

```powershell
for ($i = 1; $i -le $MAX_ITERS; $i++) {
    Write-Host "── Refinement Iteration $i/$MAX_ITERS ──"

    if (Test-Path $KILLSWITCH) {
        Write-Host "Killswitch tripped — bailing."
        Remove-Item $KILLSWITCH
        exit 2
    }

    # Run refinement check
    $results = Refine-Workflows

    # Check if all workflows are excellent
    $needsRefinement = $results | Where-Object { $_ -notlike "*✓ Excellent*" }
    if (-not $needsRefinement) {
        Write-Host "✓ All workflows meet excellence standards"
        exit 0
    }

    # Prompt for manual refinement or auto-fix
    Write-Host "Workflows needing refinement:"
    $needsRefinement | Write-Host

    # Manual intervention required for content generation
    # AI should review each workflow and add missing sections
}
```

### Manual Refinement Guidelines

When adding missing sections to workflows:

#### Edge Case Handling

- Identify 5-7 common edge cases
- Provide specific handling strategies
- Consider ambiguous inputs, conflicts, constraints

#### Failure Modes

- List 5-7 common failure scenarios
- Provide prevention strategies
- Include recovery procedures

#### Adding Performance Considerations

- Identify performance bottlenecks
- Provide optimization strategies
- Set reasonable time/resource expectations

#### Adding Security Notes (if applicable)

- Identify security risks
- Provide mitigation strategies
- Include validation requirements

### Quality Standards

A refined workflow must:

- Have all 6 excellence sections (or justification for omission)
- Provide actionable guidance in each section
- Use consistent formatting and structure
- Include specific examples where helpful
- Reference related skills/workflows appropriately

## Guardrails

1. Killswitch file - Create `~/.evolve-stop` to stop
2. Iteration cap - Default 15
3. Auto-detect instruction directory - Works with .windsurf, .agents, or custom paths
4. Backup required - Always create backup before refactor/sync/refine
5. Never delete working content - Only improve
6. Preserve cross-references - Ensure refactoring doesn't break navigation
7. Log everything - Results go to `.evolve-logs/<timestamp>/`
8. Manual review required - Instruction quality is subjective
9. Maintain consistency - Follow existing patterns
10. Validate structure - Run validation after creation

## Naming Conventions

### Workflows

- Use kebab-case: `my-new-workflow.md`
- End with `-loop` if iterative: `review-fix-loop.md`
- Focus on action: `evolve.md`

### Skills

- Directory structure: `<instruction-dir>/skills/topic-name/skill.md`
- Directory name in kebab-case
- Focus on domain/topic: `windows-api-validation`
- File name typically `skill.md` (project-specific conventions may vary)

## After Creation/Evolution

- Validate structure - Run validation checks
- Add content - Fill in template sections
- Add cross-references - Link to related skills/workflows
- Test integration - Ensure it works with instruction ecosystem
- Update rules if needed - Update project-level guidance
- Commit the change - Save to version control

## Example Usage

```powershell
# Create new workflow
/evolve create

# Refactor existing instructions
/evolve refactor

# Sync cross-references
/evolve sync

# Refine to excellence
/evolve refine
```

## Related Skills

- Replace with project-relevant skills from your instruction directory
- Examples: error-handling, testing-strategies, architecture, database-design
- Reference skills that match your project's technology stack

## Related Workflows

- `code-fix-loop.md` - For refactoring and fixing code
- `documentation-sync-loop.md` - Sync code documentation
- `suggest.md` - For brainstorming features

## Credits

Planning With Files pattern, Universal AI Team Rulebook, Ralph Safe Loop, technical writing best practices
