# Agent Orchestration

## Available Agents

Located in `~/.claude/agents/`:

| Agent | Purpose | When to Use |
|-------|---------|-------------|
| planner | Implementation planning | Complex features, refactoring |
| architect | System design | Architectural decisions |
| code-reviewer | Code review | After writing code |
| refactor-cleaner | Dead code cleanup | Code maintenance |

## ⚠️ MANDATORY Agent Usage

**These are REQUIREMENTS, not suggestions. You MUST follow these rules automatically.**

### 1. After Writing/Modifying Code
**ALWAYS run code-reviewer agent immediately after making code changes.**
- ✅ DO: Run code-reviewer as soon as code is written
- ❌ DON'T: Wait for user to ask for code review
- ❌ DON'T: Skip code review because "changes are small"

### 2. After ANY Code Changes
**ALWAYS run refactor-cleaner agent to check for:**
- File size violations (files over 400 lines)
- Dead code and unused imports
- Duplication and cleanup opportunities
- Clippy warnings

### 3. Before Marking Tasks Complete
**MANDATORY checklist before completing implementation:**
- [ ] Run code-reviewer agent
- [ ] Run refactor-cleaner agent
- [ ] Check all modified files are under 400 lines
- [ ] Fix all clippy warnings
- [ ] If any file > 400 lines, extract logic into smaller files

### 4. For Complex Features
**Use planner agent proactively for:**
- Multi-file changes
- New features with multiple approaches
- Architectural decisions
- Refactoring that affects multiple components

### 5. For Architectural Decisions
**Use architect agent when:**
- Choosing between patterns or technologies
- Designing system interactions
- Making decisions that affect multiple modules

## Post-Implementation Protocol

**REQUIRED STEPS - Execute automatically:**

1. **Immediate:** Run code-reviewer (security, quality, best practices)
2. **Immediate:** Run refactor-cleaner (file sizes, dead code, duplication)
3. **If issues found:** Fix them before marking complete
4. **Always:** Verify all files under 400 lines (800 absolute max)
5. **Always:** Run `cargo clippy` and fix all warnings

## Parallel Task Execution

ALWAYS use parallel Task execution for independent operations:

```markdown
# GOOD: Parallel execution
Launch 3 agents in parallel:
1. Agent 1: Security analysis of auth.ts
2. Agent 2: Performance review of cache system
3. Agent 3: Type checking of utils.ts

# BAD: Sequential when unnecessary
First agent 1, then agent 2, then agent 3
```

## Multi-Perspective Analysis

For complex problems, use split role sub-agents:
- Factual reviewer
- Senior engineer
- Security expert
- Consistency reviewer
- Redundancy checker

---

## Enforcement

If you (Claude) skip these mandatory steps:
1. The user will point it out (like they just did)
2. You'll need to run the agents retroactively
3. You'll waste time fixing what should have been caught immediately

**Treat these rules as hard requirements, not optional suggestions.**
