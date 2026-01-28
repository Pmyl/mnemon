# Coding Style Guidelines

## ⚠️ CRITICAL: File Size Limits

**MANDATORY - Check after EVERY code modification:**

### Hard Limits
- **Typical target:** 200-400 lines per file
- **Warning threshold:** 400 lines (start considering extraction)
- **Absolute maximum:** 800 lines (must be refactored)

### Enforcement
When a file exceeds 400 lines:
1. **Immediately** identify extraction opportunities
2. Extract into focused, cohesive modules
3. Create subdirectories for related files (e.g., `components/add_mnemon/`)
4. Update imports and module declarations

### What to Extract
- **Component files:** Separate step components, form sections, complex UI
- **Logic files:** Extract business logic into services/utilities
- **State management:** Move complex state to custom hooks
- **Type definitions:** Extract shared types to types.rs or dedicated type files

## Architecture Principles

### Many Small Files > Few Large Files
- **High cohesion:** Related code stays together
- **Low coupling:** Minimal dependencies between modules
- **Single responsibility:** Each file has one clear purpose
- **Easy navigation:** Find code quickly by feature/domain

### Organization
- **By feature/domain:** `auth/`, `search/`, `navigation/`
- **NOT by type:** Avoid `components/`, `utils/`, `services/` at root
- **Exception:** Truly shared utilities can be in `utils/` or `helpers/`

## Examples

### ❌ BAD: 626-line add_mnemon.rs
```
src/components/add_mnemon.rs  (626 lines - way too big!)
```

### ✅ GOOD: Split into focused modules
```
src/components/add_mnemon/
  mod.rs              (60 lines - main flow)
  step1_work.rs       (300 lines - work selection)
  step2_personalize.rs (80 lines - personalization)
  search_ui.rs        (150 lines - search interface)
```

## Practical Guidelines

### When Writing New Code
1. Start with small files
2. If approaching 400 lines, pause and refactor
3. Don't wait until hitting 800 lines

### When Modifying Existing Code
1. Check file line count: `wc -l filename`
2. If over 400 lines, extract while you're there
3. Use refactor-cleaner agent to identify opportunities

### How to Check
```bash
# Check specific files
wc -l src/components/*.rs

# Find large files
find src -name "*.rs" -exec wc -l {} \; | sort -rn | head -20
```

---

Read DIOXUS.md for Dioxus-specific rules.
