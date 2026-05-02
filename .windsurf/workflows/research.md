---
description: Autonomous deep web research workflow that self-generates questions, iteratively searches, and compiles findings into a markdown report
---

# Deep Research

Autonomous workflow for conducting comprehensive web research on any topic. Self-generates clarifying questions, performs iterative web searches, validates completeness, and compiles findings into a structured markdown report.

## Overview

This workflow enables deep, autonomous research by:

1. Analyzing the user's prompt to generate targeted research questions
2. Iteratively searching the web to answer each question
3. Identifying gaps and follow-up questions from search results
4. Continuing until all questions are answered with high confidence
5. Compiling findings into a well-structured markdown report in the workspace root

## When to Use
- User requests research on a topic you need to investigate thoroughly
- You need comprehensive information before making technical decisions
- Exploring new technologies, frameworks, or domains
- Gathering competitive intelligence or market research
- Investigating best practices or architectural patterns
- Understanding complex problems that require multiple perspectives

## When NOT to Use
- For simple factual queries (use direct web search instead)
- When the answer is already in the codebase or documentation
- For time-critical tasks requiring immediate action
- When the user wants a quick summary, not deep research
- For questions that require proprietary or internal knowledge only

## Pre-flight Checks

```powershell
# Verify workspace is accessible
if (-not (Test-Path .)) {
    Write-Error "Cannot access workspace directory"
    exit 1
}

# Check if research output already exists to avoid overwriting
$existingResearch = Get-ChildItem -Filter "*_research.md" -ErrorAction SilentlyContinue
if ($existingResearch) {
    Write-Host "Found existing research files:"
    $existingResearch | ForEach-Object { Write-Host "  - $($_.Name)" }
    $overwrite = Read-Host "Overwrite existing research? (Y/N)"
    if ($overwrite -ne "Y") {
        exit 0
    }
}

# Initialize research context
$RESEARCH_TOPIC = $args[0]
if (-not $RESEARCH_TOPIC) {
    Write-Error "Usage: deep-research <topic>"
    exit 1
}
```

## Loop Configuration

```powershell
$MAX_ITERS = 15
$KILLSWITCH = "$env:USERPROFILE\.workflow-stop"
$LOGDIR = ".workflow-logs/$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $LOGDIR | Out-Null

$researchQuestions = @()
$answeredQuestions = @{}
$followUpQuestions = @()
$researchLog = @()
$confidenceThreshold = 0.8
```

## The Loop

```powershell
for ($i = 1; $i -le $MAX_ITERS; $i++) {
    Write-Host "── Iteration $i/$MAX_ITERS ──"

    if (Test-Path $KILLSWITCH) {
        Write-Host "Killswitch tripped — bailing."
        Remove-Item $KILLSWITCH
        exit 2
    }

    # Generate or use follow-up questions
    if ($i -eq 1 -or $followUpQuestions.Count -gt 0) {
        if ($i -eq 1) {
            Write-Host "Generating initial research questions..."
            $researchQuestions = Generate-InitialQuestions -Topic $RESEARCH_TOPIC
        } else {
            Write-Host "Processing follow-up questions..."
            $researchQuestions = $followUpQuestions
            $followUpQuestions = @()
        }
        Write-Host "Questions to research: $($researchQuestions.Count)"
    }

    # Research each unanswered question
    $unansweredQuestions = $researchQuestions | Where-Object { -not $answeredQuestions.ContainsKey($_) }
    foreach ($question in $unansweredQuestions) {
        Write-Host "Researching: $question"
        $searchResults = Search-Web -Query $question
        $analysis = Analyze-SearchResults -Results $searchResults -Question $question

        if ($analysis.Confidence -ge $confidenceThreshold) {
            $answeredQuestions[$question] = @{
                Answer = $analysis.Answer
                Sources = $analysis.Sources
                Confidence = $analysis.Confidence
                Timestamp = Get-Date
            }
            $newQuestions = Extract-FollowUpQuestions -Analysis $analysis
            if ($newQuestions) { $followUpQuestions += $newQuestions }
        } else {
            $followUpQuestions += Refine-Query -Question $question -Analysis $analysis
        }
    }

    # Check completion
    $allAnswered = ($researchQuestions | Where-Object { -not $answeredQuestions.ContainsKey($_) }).Count -eq 0
    if ($allAnswered -and $followUpQuestions.Count -eq 0) {
        Write-Host "✓ All questions answered"
        break
    }
}

$reportPath = Generate-ResearchReport -Topic $RESEARCH_TOPIC -Answers $answeredQuestions
Write-Host "✓ Report saved to: $reportPath"
```

## Question Generation Logic

Generate questions systematically covering: definition, mechanism, benefits, limitations, alternatives, best practices, current state, and ecosystem.

```powershell
function Generate-InitialQuestions {
    param([string]$Topic)
    $questions = @(
        "What is $Topic and what are its core concepts?"
        "How does $Topic work technically?"
        "What problems does $Topic solve?"
        "What are the main benefits of using $Topic?"
        "What are the limitations or drawbacks of $Topic?"
        "When should $Topic NOT be used?"
        "What are the main alternatives to $Topic?"
        "How does $Topic compare to its alternatives?"
        "What are the best practices for using $Topic?"
        "What are common pitfalls or mistakes when using $Topic?"
        "What is the current state and maturity of $Topic?"
        "What are recent developments or trends in $Topic?"
        "What is the community sentiment around $Topic?"
        "What tools, libraries, or resources exist for $Topic?"
    )
    return $questions
}
```

## Report Structure

```markdown
# Research Report: [Topic]

**Generated:** [Date] | **Iterations:** [N] | **Questions:** [N] | **Confidence:** [X%]

## Executive Summary
[2-3 paragraph summary]

## Core Concepts
[What is X? with sources]

## Technical Overview
[How does X work? with sources]

## Benefits and Use Cases
[Why use X? with sources]

## Limitations and Trade-offs
[When NOT to use X? with sources]

## Comparison with Alternatives
[Alternatives with sources]

## Best Practices
[Recommendations with sources]

## Common Pitfalls
[Mistakes to avoid with sources]

## Current State and Trends
[Latest developments with sources]

## Community and Ecosystem
[Tools, resources, adoption with sources]

## Sources
[Cited sources list]

## Research Log
[Optional: detailed search log]
```

## Guardrails (Non-Negotiable)

1. Always cite sources - Every claim must reference source material
2. Confidence threshold - Don't consider answered below 80% confidence
3. Source diversity - Seek multiple sources to avoid bias
4. Recency check - Prioritize recent sources (1-2 years) for fast-moving topics
5. Killswitch file - Create `~/.workflow-stop` to stop
6. Iteration cap - Default 15 iterations
7. No hallucination - Only report what was found in search results
8. Clear uncertainty - Explicitly mark uncertain/conflicting information
9. Workspace root - Save report to workspace root
10. Unique filenames - Use timestamp or topic hash

## Research Quality Criteria

- Completeness - Addresses all aspects of the question
- Accuracy - Multiple sources agree
- Recency - Current information for fast-moving topics
- Authority - Credible and authoritative sources
- Specificity - Specific to question, not generic
- Evidence - Examples, data, or concrete evidence

## Related Skills

- `../skills/windsurf-memory/SKILL.md` - Persist research findings

## Related Workflows

- `brainstorm.md` - Brainstorm options with pros/cons
- `suggest.md` - Brainstorm features/improvements based on research

## Edge Case Handling
- **Vague topics**: When research topic is too broad, narrow down to specific aspects
- **Conflicting sources**: Multiple sources disagree - present conflicting views with context
- **Outdated information**: Sources are old - prioritize recent sources and note recency
- **Sparse results**: Limited search results - broaden query or try alternative terms
- **Technical depth**: Topic too technical for general understanding - include explanatory context
- **Language barriers**: Sources in non-English languages - use translation or find English equivalents
- **Paywalled content**: Sources behind paywalls - find free alternatives or note paywall limitation

## Failure Modes
- **Insufficient confidence**: Questions remain below confidence threshold after max iterations - document uncertainty and suggest manual review
- **Search API failures**: Web search tool unavailable or errors - retry with exponential backoff, escalate if persistent
- **Hallucination risk**: AI generates content not found in sources - strict source citation requirement prevents this
- **Report generation failure**: Cannot write report to workspace - verify write permissions and disk space
- **Topic drift**: Research veers off-topic - periodically check alignment with original question
- **Source bias**: Over-reliance on single source - enforce source diversity requirement
- **Infinite loop**: Follow-up questions never resolve - iteration cap prevents runaway

## Performance Considerations
- **Research velocity**: Complete within 5-15 minutes for most topics depending on complexity
- **Question generation**: Start with 10-15 initial questions, add follow-ups as needed
- **Search efficiency**: Batch related questions to reduce API calls
- **Source caching**: Cache search results to avoid redundant queries
- **Report size**: Keep reports concise but comprehensive - aim for 2000-5000 words
- **Iteration balance**: Balance thoroughness with time - confidence threshold prevents over-researching
- **Context management**: Research logs can grow large - use rolling logs or summarize periodically

## Security Notes
- **Source validation**: Verify sources are credible and not malicious sites
- **Content sanitization**: Ensure no malicious code or scripts in research output
- **Privacy**: Don't include PII or sensitive data in research reports
- **API key protection**: If using paid search APIs, never log or expose keys
- **Phishing awareness**: Be cautious of sources that may be phishing attempts
- **Information disclosure**: Research reports may be shared - ensure no sensitive project details
- **Compliance**: Consider regulatory requirements when researching (GDPR, etc.)
