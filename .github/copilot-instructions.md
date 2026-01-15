# Copilot Instructions for Mnemon

This repository contains **Mnemon**, a private, nostalgia-focused app built with Dioxus 0.7 for capturing and resurfacing memories from Movies, TV/Anime, and Games.

## Essential Documentation

Before working on any task, familiarize yourself with these key documents:

- **PROJECT.md** - Complete MVP specification with core concepts, data model, and acceptance criteria
- **LIVING_PLAN.md** - Phase-by-phase implementation plan with current progress and review log
- **WIREFRAMES.md** - UI/UX specifications, layouts, and interaction patterns
- **DIOXUS.md** - Dioxus 0.7 API reference and patterns (Dioxus 0.7 has breaking changes from earlier versions)
- **AGENTS.md** - General agent guidelines for this repository

## Technology Stack

- **Framework**: Dioxus 0.7.2 with router
- **Target Platforms**: Web (primary), Desktop, Mobile
- **Styling**: Tailwind CSS (automatic compilation via `dx serve`)
- **Storage**: IndexedDB (via rexie) for web persistence
- **API Clients**: TMDB (Movies/TV), RAWG (Games)
- **Build Tool**: Dioxus CLI (`dx`)

## Development Workflow

### Running the App

```bash
dx serve
```

The app runs with hot reload. Tailwind CSS is automatically compiled by Dioxus CLI.

### Building

```bash
# Web build
dx bundle --release --platform web --base-path mnemon

# Desktop build
dx build --release --platform desktop
```

### Testing and Validation

```bash
# Check code compiles
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests (when available)
cargo test
```

**Important**: Assume `dx serve` is always running. No need to run `dx build` for validation—`cargo check` is sufficient.

## Project Structure

```
mnemon/
├── .github/
│   ├── workflows/
│   │   ├── deploy.yml              # GitHub Pages deployment
│   │   └── copilot-setup-steps.yml # Copilot environment setup
│   └── copilot-instructions.md     # This file
├── assets/                         # Static assets (images, fonts)
├── src/
│   ├── main.rs                     # Entry point and main app component
│   ├── models/                     # Data models (Work, Mnemon)
│   ├── storage/                    # IndexedDB persistence layer
│   ├── providers/                  # API clients (TMDB, RAWG)
│   ├── settings/                   # User settings (API tokens)
│   └── components/                 # Reusable UI components
├── Cargo.toml                      # Rust dependencies
├── Dioxus.toml                     # Dioxus configuration
├── tailwind.css                    # Tailwind input
└── package.json                    # Node.js dependencies (Tailwind)
```

## Critical Coding Guidelines

### Dioxus 0.7 Specifics

⚠️ **Dioxus 0.7 has breaking API changes**. Do NOT use patterns from earlier versions:

- **NO** `cx`, `Scope`, or `use_state` (removed in 0.7)
- **YES** `use_signal`, `use_memo`, `use_resource` for state management
- Components use `#[component]` macro
- Props must be owned values (use `String`, not `&str`)
- Props must implement `PartialEq` and `Clone`
- Call signals like functions: `my_signal()` to read, `my_signal.write()` to mutate

Example:
```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        button { onclick: move |_| *count.write() += 1, "Count: {count}" }
    }
}
```

### RSX Patterns

```rust
rsx! {
    div {
        class: "container",
        color: "red",
        // Prefer loops over iterators
        for item in items {
            span { "{item}" }
        }
        // Conditionals
        if show {
            p { "Visible" }
        }
    }
}
```

### Asset References

Always use the `asset!()` macro for local files:

```rust
rsx! {
    img { src: asset!("/assets/image.png") }
    document::Stylesheet { href: asset!("/assets/styles.css") }
}
```

### Async and State Management

- Use `use_resource` for async operations (API calls)
- Use `use_signal` for local component state
- Use `use_memo` for derived/computed values
- Use Context API (`use_context_provider`/`use_context`) for shared state

### API Token Management

- API tokens (TMDB, RAWG) are stored in localStorage, NOT environment variables
- Tokens are loaded at runtime via `settings::get_tmdb_token()` and `settings::get_rawg_api_key()`
- Fallback to manual entry when tokens are missing or API calls fail

## MVP Scope and Constraints

### In Scope

- Homepage with full-page "Surprise Me" hero (auto-cycling memories)
- Add Mnemon flow: search providers OR manual entry → optional personalization
- Memory Details (read-only, slide-up panel)
- Local persistence (IndexedDB for web)
- Offline-capable with cached assets
- Dark mode only
- Delete with undo (5-second toast with progress bar)

### Out of Scope for MVP

- Edit functionality (mnemons are immutable after save)
- Light mode / theme toggle
- User accounts, sharing, or social features
- Lists/grids view (hero-only)
- Advanced search/filter of saved items
- Ratings, status tracking, collections
- Import/export
- Accessibility beyond basic semantics

### Data Constraints

- **Exact-ID dedupe**: Works are deduplicated by `(provider_source, provider_id)`
- **No cross-provider dedupe** in MVP
- **Manual entries**: No cover image or theme music (show placeholders)
- **No edits**: Once saved, mnemons cannot be edited (delete-only)

## Phase-Based Development

The project follows a phased approach documented in **LIVING_PLAN.md**. Always check the current phase and completed steps before starting work:

- **Phase 0-6**: ✅ Complete (Bootstrap → GitHub Pages deployment)
- **Phase 7**: 🚧 In progress (Asset caching & audio playback)
- **Phase 8**: Offline guarantees & refinement
- **Phase 9**: Mobile persistence

When implementing features, ensure they align with the current phase and don't introduce functionality from future phases.

## UI/UX Patterns

### Design System

- **Theme**: Dark mode only (charcoal/gray background, white text)
- **Typography**: Nostalgic, minimal, image-forward
- **Interactions**: Click anywhere on hero to add mnemon
- **Transitions**: Smooth slides (hero auto-cycles every 10s)
- **Feelings**: Fixed taxonomy (12 emotions), max 5 per mnemon
- **Notes**: Rich text with adaptive reading time in hero

### Key Interactions

1. **Hero auto-cycle**: Advances every 10s, wraps around, smooth slide transitions
2. **Click zones**:
   - Upper hero area → Opens Add Mnemon flow
   - Bottom title bar → Toggles Memory Details slide-up
3. **Add flow**: Modal overlay, 2-step wizard (Pick Work → Personalize)
4. **Delete**: Immediate removal with 5s undo toast (animated progress bar)

### Responsive Behavior

- Hero always fills viewport
- Details slide up from bottom (not a separate route)
- Mobile-first design with touch-friendly tap targets

## Common Tasks

### Adding a New Feature

1. Check **LIVING_PLAN.md** to ensure feature is in current phase
2. Review **PROJECT.md** for requirements and acceptance criteria
3. Check **WIREFRAMES.md** for UI specifications
4. Implement minimal changes to existing components
5. Test with `cargo check` and `dx serve`
6. Update **LIVING_PLAN.md** review log when phase step completes

### Adding API Integration

1. Create client in `src/providers/`
2. Implement async methods returning `Result<T, ApiError>`
3. Add graceful fallback to manual entry on error
4. Handle offline state (no API calls when offline)
5. Cache results in IndexedDB when appropriate

### Adding Storage Operations

1. Use `rexie` for IndexedDB operations
2. Define models with `Serialize`/`Deserialize` traits
3. Add async functions to `src/storage/mod.rs`
4. Update `AppState` signals after mutations
5. Handle errors gracefully (log and show user-friendly messages)

### Styling Components

1. Use Tailwind utility classes
2. Follow dark mode color scheme (gray/charcoal backgrounds, white text)
3. Prefer `class` over inline styles
4. Use consistent spacing (Tailwind scale: 2, 4, 6, 8)
5. Test responsive behavior (mobile, tablet, desktop)

## Error Handling

- **API errors**: Show error message, fallback to manual entry
- **Storage errors**: Log to console, show user-friendly toast
- **Asset loading**: Use placeholders for missing images/audio
- **Network offline**: Detect and show appropriate UI state

## Testing Strategy

- Manual testing via `dx serve` (primary validation method)
- No unit tests in MVP (add later if needed)
- Verify each change in the running app before committing
- Test offline behavior by disabling network in DevTools

## Deployment

- **GitHub Pages**: Automatic deployment on push to `main`
- **Workflow**: `.github/workflows/deploy.yml`
- **Base path**: `/mnemon/` (configured in Dioxus.toml)
- **Build output**: `dist/public/`

## Common Pitfalls

❌ **Don't:**
- Use Dioxus 0.6 or earlier patterns (`cx`, `Scope`, `use_state`)
- Add light mode or theme toggle (out of scope for MVP)
- Implement edit functionality (mnemons are immutable)
- Create lists/grids view (hero-only in MVP)
- Use compile-time env vars for API tokens (use localStorage)
- Commit `.env` files (tokens are user-configured)
- Modify existing working tests or linting rules

✅ **Do:**
- Reference Dioxus 0.7 documentation in **DIOXUS.md**
- Check **LIVING_PLAN.md** before starting work
- Follow dark mode design system
- Handle offline gracefully
- Use placeholders for missing assets
- Add minimal changes to accomplish goals
- Test in the running app with `dx serve`

## Security Considerations

- Never commit API tokens to the repository
- Use `.env.example` for documenting required env vars
- Store user tokens in localStorage (client-side only)
- Validate and sanitize user inputs
- Use HTTPS for all API calls

## Questions or Unclear Requirements?

If you encounter ambiguous requirements or need clarification:

1. Check the relevant documentation files first (PROJECT.md, WIREFRAMES.md, LIVING_PLAN.md)
2. Look for similar patterns in existing code
3. Ask the user for clarification before proceeding
4. Document decisions in code comments when appropriate

## Additional Resources

- [Dioxus 0.7 Documentation](https://dioxuslabs.com/learn/0.7)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [TMDB API Documentation](https://developer.themoviedb.org/docs)
- [RAWG API Documentation](https://rawg.io/apidocs)

---

**Remember**: This is an MVP focused on core functionality. Keep changes minimal, test thoroughly, and always respect the scope defined in the specification documents.
