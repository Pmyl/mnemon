# Mnemon — Living Implementation Plan


This document is the evolving build plan for Mnemon's MVP, grounded in `PROJECT.md`, `WIREFRAMES.md`, `TECHNOLOGIES.md`, and `DIOXUS.md`. After every review cycle, we revise this plan so it always reflects the next actionable steps.

## Ground Rules
- **Scope guardrails:** Respect MVP scope and exclusions exactly as defined in the specification files.
- **Technology guardrails:** Follow the workspace layout, Dioxus 0.7 patterns, and "no unused code" policy documented in `TECHNOLOGIES.md`.
- **Review cadence:** Every numbered step must deliver a reviewable change in the running app (UI behavior, data flow, or platform capability that you can see or verify).
- **Documentation hygiene:** When implementation details land in code, migrate duplicate guidance from the MD playbooks into rustdoc and update references here accordingly.

## Phase 0 — Project Bootstrap
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 1 — Scaffold the workspace** | Create the  Dioxus entry points. | application that runs "Hello mnemon" screen | None |

## Phase 1 — Hero Shell & Empty State
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 2 — App shell with empty Surprise hero** ✅ | Hero layout with empty state using static placeholder data. | Running app shows hero empty state per wireframe. Click anywhere to load sample data and see hero auto-cycle through memories with smooth transitions. | Step 1 |

## Phase 2 — Manual Mnemon Vertical Slice
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 3 — Manual Add flow creates surfacing mnemons** ✅ | Implement Step 1 (manual-only entry), Step 2 personalization, and in-memory storage used by the hero. | You can add a manual mnemon, return to hero, and see it surfaced (with feelings/notes preview). Auto-cycle continues through all session mnemons. | Step 2 |

## Phase 3 — Provider Search Experience
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 4 — Provider search with deterministic fixtures** ✅ | Introduce provider abstractions plus fixture-backed results hooked into Step 1 UI. | Searching shows grouped provider results with exact-ID dedupe; selecting a fixture Work populates hero/details using cached metadata. | Step 3 |
| **Step 5 — Real provider integrations** ✅ | Connect to live APIs (TMDB for Movies/TV, RAWG for Games) with graceful fallback. | Live search returns real titles when API keys configured; manual entry path activates automatically when offline or keys missing. | Step 4 |

## Phase 4 — Persistence Foundations
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 6 — Web persistence (IndexedDB)** ✅ | Persist Works/Mnemons/assets metadata for the web target. | In web build, mnemons survive page reloads; empty state returns only when storage cleared. Manual entries still surface in hero and details after reload. | Step 5 |

## Phase 5 — Read-only Details & Audio Stub
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 7 — Memory Details view with audio controls (stub)** ✅ | Render Details view as slide-up panel, share components for feelings/notes, wire a stubbed audio controller, and add delete with undo. | Clicking bottom title bar slides hero up to reveal details below; clicking title bar again slides back down. Audio player stub with play/pause button (no actual audio yet). Delete button with undo toast that shows animated progress bar before permanent deletion. | Step 6 |

## Phase 6 — GitHub Pages Deployment & User Token Configuration
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 8 — GitHub Actions workflow for static deployment** ✅ | Create CI/CD pipeline to build and deploy the web app to GitHub Pages. | Push to main triggers automated build; site is live at `https://<user>.github.io/mnemon/` with all static assets served correctly. | Step 7 |
| **Step 9 — Settings UI for API token configuration** ✅ | Add a Settings page/modal where users can input their own TMDB and RAWG API keys, stored in localStorage. | Users can open Settings, enter API tokens, save them; tokens persist across sessions and are used for provider searches instead of compile-time env vars. | Step 8 |
| **Step 10 — Runtime token loading** | Refactor provider clients to read tokens from localStorage at runtime instead of compile-time env vars. | App works on GitHub Pages with user-provided tokens; shows configuration prompts when tokens missing; seamless fallback to manual entry. | Step 9 |

## Phase 7 — Asset Caching & Audio Playback
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 11 — Cover and theme caching pipeline** | Fetch and persist cover/music assets, updating Works with local URIs. | After saving a provider-backed mnemon, cover art appears in hero/details; audio play button streams cached preview when available; placeholders show when assets missing. | Step 10 |
| **Step 12 — Surprise hero audio behavior** | Enforce autoplay policy, stop-on-switch, and Play fallback per MVP rules. | Hero auto-plays when platform allows, otherwise surfaces a Play control; switching memories stops previous audio and (re)autoplays subject to policy. | Step 11 |

## Phase 8 — Offline Guarantees & Refinement
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 13 — Offline resilience audit** | Validate offline flows, add UX cues, and document limitations. | Demonstrated walkthrough (web & desktop) showing manual entry offline, existing mnemons surfacing with cached assets, and copy guiding offline behavior. | Step 12 |
| **Step 14 — Quality hardening & release playbook** | Polish code/tests/docs and produce release artifacts. | Clean `cargo fmt/clippy/test` runs, migration of duplicated docs into rustdoc, generated web bundle & desktop binaries with deployment checklist. | Step 13 |

## Phase 9 — Mobile Persistence
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 15 — Mobile persistence (iOS/Android)** | Add mobile storage adapters using native filesystem APIs for iPhone and Android apps. | Mobile builds persist mnemons across app restarts; data stored in app-specific storage directories. | Step 14 |

## Review & Feedback Log
| Revision | Reviewer Notes | Plan Adjustments |
| --- | --- | --- |
| rev. 0 | Initial plan | Replaced with rev. 1 per feedback (ensure each step produces reviewable behavior). |
| rev. 1 | Step 2 complete | Implemented headerless design with: (1) Empty state with "tap anywhere to begin", (2) Dark mode styling, (3) Click interaction loads sample data for demo, (4) Auto-cycling hero with smooth transitions, (5) Rotating notes with adaptive reading time. Updated WIREFRAMES.md and PROJECT.md to reflect simplified design without header. Theme toggle deferred to future settings page. |
| rev. 2 | Step 3 complete + Phase 2 improvements | Implemented complete Add Mnemon flow with: (1) Modal overlay UI with two-step wizard, (2) Step 1: Manual entry form (Type, Title, Notes, Feelings with emojis), (3) Step 2: Optional dates (Release Year, Finished Date), (4) Feelings display as emoji+name chips with 5-max selection, (5) In-memory storage using signals, (6) Click-anywhere interaction opens Add flow, (7) New mnemons immediately surface in hero with auto-cycle, (8) Manual entries show placeholder background when no cover image, (9) Fixed timer bug where multiple timers were spawned per mnemon added. Full vertical slice working end-to-end. |
| rev. 3 | Step 4 complete | Implemented provider search with fixtures: (1) Type-first flow - select type before searching, (2) Search-in-title - title field doubles as search box, triggers on field focus, 3+ chars (debounced) or Enter key forces search, (3) Empty search returns all results for selected type, (4) Expanded fixture data - 50+ titles across Movies/TV-Anime/Games with provider metadata (tmdb/anilist/igdb), (5) Extracted search logic into separate `search_works()` function ready for future API replacement, (6) Pagination implemented - returns 10 results per page (infrastructure for future UI), (7) Search results dropdown with cover thumbnails and year display, (8) Click result to autofill form (title, cover_url, provider_ref, and release year for Step 2), (9) Duplicate detection - prevents selecting works that already exist (same provider_source + provider_id), shows error message, (10) Unified form - manual and provider search use same UI, provider results just autofill fields, (11) Step 1 now only has Type, Title, Notes, and Feelings. Step 2 has Release Year (autofilled from search or manual entry) and Finished Date (user input date for when they completed it). Search and manual entry seamlessly coexist in one flow. |
| rev. 4 | Step 5 complete | Implemented TMDB integration for Movies and TV/Anime, RAWG integration for Games: (1) **TMDB API Client** - Async HTTP client using reqwest with Bearer token auth, (2) **RAWG API Client** - Simple REST API with key-based auth for games (replaced IGDB due to CORS restrictions), (3) **Compile-time tokens** - Reads `TMDB_ACCESS_TOKEN` and `RAWG_API_KEY` env vars at compile time via `option_env!()`, (4) **Debounced search** - 300ms delay after typing stops before API request fires (SEARCH_DEBOUNCE_MS constant), (5) **Loading state** - Spinner shows while search is in progress, (6) **Graceful fallback** - When providers not configured, shows warning banner and allows manual entry, (7) **Network error handling** - Displays error message and allows manual entry on failures, (8) **Search versioning** - Cancels stale searches when user types faster than debounce, (9) **New module structure** - `src/providers/` with `mod.rs` (traits/errors), `tmdb.rs`, and `rawg.rs`, (10) **SearchService** - Unified search routing to appropriate provider based on WorkType, (11) **SearchStatus enum** - Tracks Success/ProviderNotConfigured/NetworkError/UsingFixtures for UI feedback. To use locally: `export TMDB_ACCESS_TOKEN="..." RAWG_API_KEY="..."` before `dx serve`. |
| rev. 5 | Step 6 complete | Implemented IndexedDB persistence for web: (1) **IndexedDB via rexie** - Async database wrapper for WASM with three object stores (works, mnemons, assets), (2) **Serde serialization** - Added Serialize/Deserialize to all models (Work, Mnemon, ProviderRef, WorkType, WorkOrigin), (3) **Async storage module** - `src/storage/mod.rs` with `save_work()`, `save_mnemon()`, `load_works()`, `load_mnemons()`, `load_all_async()`, (4) **Asset storage ready** - `StoredAsset` struct and `save_asset()`/`load_asset()` functions prepared for Phase 7 image/audio caching, (5) **AppState integration** - Loads from IndexedDB on app mount via `use_effect`, saves individual items asynchronously on add, (6) **Loading state** - `AppState.loaded` signal tracks whether initial data load is complete, (7) **Dependencies added** - `rexie = "0.6"`, `serde-wasm-bindgen = "0.6"`, enabled `serde` features on `uuid` and `chrono`. Data survives page reloads; empty state only shows when IndexedDB is cleared. |
| rev. 6 | Step 7 complete | Implemented Memory Details as slide-up panel (not separate route): (1) **Slide-up interaction** - Hero slides up to reveal details below when clicking bottom title bar area, (2) **Dynamic title bar height** - Title bar height measured dynamically using onmounted + get_client_rect to accommodate multi-line titles and feelings, (3) **Two click zones** - Upper area opens Add flow, bottom title bar toggles details, (4) **Auto-cycle pauses** - Slideshow pauses when details are open, resumes when closed (also S key to toggle pause for debugging), (5) **MemoryDetails component** - Shows feelings chips, finished date, and notes in scrollable area, (6) **Stubbed audio player** - Play/pause button toggles state (no actual audio), progress bar placeholder, only shown when theme_music_local_uri exists, (7) **Delete with undo** - Delete button in details view removes mnemon immediately, shows toast with animated progress bar (depletes right-to-left over 5s), undo restores mnemon, timeout permanently deletes from IndexedDB, (8) **Storage delete** - Added delete_mnemon function to storage module, (9) **AppState methods** - Added remove_mnemon (removes from memory, returns data for restore), restore_mnemon (re-inserts at original position), delete_mnemon_from_storage (async permanent delete). |
| rev. 7 | Step 8 complete | Implemented GitHub Actions workflow for GitHub Pages deployment: (1) **Workflow file** - `.github/workflows/deploy.yml` triggers on push to `main` and manual dispatch, (2) **Build pipeline** - Installs Rust with `wasm32-unknown-unknown` target, Dioxus CLI, Node.js, and Tailwind CSS, (3) **Production build** - Runs `dx build --release --platform web --base-path mnemon` for correct subdirectory hosting, (4) **Comprehensive caching** - Caches Cargo registry/index/build, Dioxus CLI binary, and node_modules for faster CI runs, (5) **GitHub Pages integration** - Uses `actions/upload-pages-artifact` and `actions/deploy-pages` with proper permissions, (6) **Jekyll bypass** - Creates `.nojekyll` file in dist to prevent GitHub Pages Jekyll processing, (7) **Gitignore updated** - Added `/dist` to exclude build artifacts from repo. To enable: push to GitHub, set Pages source to "GitHub Actions" in repo settings. |
| rev. 8 | Step 9 complete | Implemented Settings UI for API token configuration: (1) **Settings module** - `src/settings/mod.rs` with localStorage functions for save/load TMDB and RAWG tokens, (2) **SettingsModal component** - Modal overlay with input fields for TMDB Access Token and RAWG API Key, status indicators (configured/not configured), links to get API keys, (3) **Entry points** - Gear icon button fixed in top-right corner, comma key shortcut to open settings, (4) **localStorage persistence** - Tokens stored with keys `mnemon_tmdb_access_token` and `mnemon_rawg_api_key`, (5) **ApiTokenSettings struct** - Helper for loading/saving settings with has_tmdb/has_rawg checks that also consider compile-time env vars, (6) **Password inputs** - Token fields use password type for security, (7) **Save feedback** - Status message shows success/failure after save with note about page reload, (8) **web-sys Storage feature** - Added to Cargo.toml for localStorage access. Settings UI ready; Step 10 will wire tokens to provider clients. |
| rev. 9 | _Pending_ | _TBD_ |

When you review a step's implementation, we will capture your notes here and adjust downstream steps before moving forward.