# TECHNOLOGIES — Rust + Dioxus stack for Mnemon (MVP)

## Migration and de-duplication policy (for the coding agent)
To avoid stale docs and duplication, treat the codebase as the single source of truth for implementation details.

- What moves into code (and should be removed from this file as you implement it):
  - Concrete API/trait signatures, type definitions, and module/service boundaries.
  - Platform-specific implementation notes (e.g., which crate or adapter does what).
  - Build/run instructions that are target-specific.
  - Storage schemas, audio wiring, provider adapter specifics, and fetch/caching flows.
  - Any examples or usage snippets that pertain to a specific crate/module.

- Where to put it in code:
  - Crate-level docs using //! in each crate’s lib.rs (e.g., app_core, app_ui, platform_*).
  - Module/function/trait docs using /// rustdoc comments.
  - crate-local README.md files (e.g., crates/app_core/README.md) for longer explanations.
  - Comments only when they explain non-obvious algorithms or decisions.

- What stays in this TECHNOLOGIES.md:
  - High-level stack overview and architecture intent.
  - Cross-cutting principles and constraints (local-first, exact-ID dedupe, offline).
  - Pointers/links to the canonical code docs (prefer permanent paths/anchors).

- Workflow:
  - When you land a feature, migrate the corresponding implementation details into code docs and delete them here in the same PR.
  - If duplication is unavoidable temporarily, add a short pointer here and remove the duplicated text.
  - Prefer linking to code docs over copying text.

This policy ensures this document remains a stable, high-level guide while details live next to the code that implements them.

This document guides contributors and agents through the chosen stack and patterns for building Mnemon’s MVP:
- Rust + Dioxus UI across Web (WASM), Desktop, and Mobile
- Local-first storage and offline assets (covers + music previews)
- Provider integrations for Movies / TV/Anime / Games with exact-ID dedupe
- Audio playback abstraction with autoplay fallback
- Conventions to keep code agent-friendly

Note: This file intentionally provides “how to build” directions and glue patterns that are proven to help automation agents succeed with Rust + Dioxus. It’s not a full tutorial; it’s a blueprint.


## Targets and project layout

We will keep a modular, layered layout. The coding agent should implement separate crates/modules to keep platform concerns isolated from domain logic.

Recommended workspace structure:
- crates/app_core: Pure Rust. Domain models, validation, storage traits, provider traits, audio traits, serialization. No platform-specific dependencies.
- crates/app_ui: Dioxus components, routing, state, and platform feature gates. This crate depends on `app_core` traits and interacts with injected implementations.
- crates/platform_web: WASM bindings for storage (IndexedDB), audio (HTMLAudioElement), asset caching, HTTP fetch, and provider adapters suitable for web. Builds with Dioxus Web renderer.
- crates/platform_desktop: Native bindings for storage (SQLite + files), audio (rodio), asset caching, HTTP, and provider adapters. Builds with Dioxus Desktop renderer.
- crates/platform_mobile: (Optional post-MVP) Similar to desktop; iOS/Android packaging via `dioxus-mobile` or an equivalent path.
- crates/providers: Concrete provider adapters (TMDB, AniList/MAL/Jikan, IGDB, etc.), plus a “MusicResolver” that can find 30s previews where possible.

Top-level binaries (per target):
- web: app entry for Dioxus Web (WASM)
- desktop: app entry for Dioxus Desktop
- mobile: app entry for Dioxus Mobile (post-MVP if desired)


## Rust + Dioxus stack

- Language/toolchain: stable Rust
- UI framework: Dioxus v0.7.0
- Renderers:
  - Web: Dioxus Web (WASM)
  - Desktop: Dioxus Desktop (tao/wry based)
  - Mobile (post-MVP): Dioxus Mobile
- Routing: dioxus-router
- State management:
  - Prefer Dioxus Signals or a single global AppStore (signals under the hood) that owns AppState and services (storage, providers, audio).
- Logging: dioxus-logger or tracing; route to console/logcat where supported
- Serialization: serde (+ serde_json)
- HTTP: reqwest for native, fetch bindings for web (a thin adapter that wraps web_sys fetch)
- Time: chrono or time crate for parsing and formatting dates


## UI structure (agent-friendly)

- App root sets up:
  - Theme provider (light/dark toggle)
  - AppStore injection (storage + providers + audio implementations)
  - Router:
    - “/” → Homepage (Surprise Me hero)
    - “/add” → Add flow: Step 1 (Pick Work) → Step 2 (Personalize)
    - “/m/:id” → Read-only Memory Details
- Homepage: a single full-viewport hero that surfaces one mnemon, with:
  - Title/Year/Type, optional feelings/date/notes preview
  - Play/Pause if autoplay disallowed by platform policies
  - “Next Surprise” and “Open Memory”
- Add: two small forms; do not overcomplicate validation—only Work is required
- Details: read-only panel with cover, music, text



## Data model (matches PROJECT.md)

Core entities (in app_core):
- Work
  - id: uuid (local)
  - type: enum { movie, tv_anime, game }
  - title_en: String
  - release_year: Option<u16>
  - cover_image_local_uri: Option<String>  // stored local URL/path
  - theme_music_local_uri: Option<String>  // stored local URL/path
  - providers: Vec<ProviderRef>            // exact-ID dedupe keys
  - origin: enum { provider, manual }
  - created_at: DateTime
- ProviderRef
  - provider_source: String  // e.g., "tmdb", "anilist", "igdb"
  - provider_id: String
- Mnemon
  - id: uuid (local)
  - work_id: Work.id
  - finished_date: Option<NaiveDate>
  - feelings: Vec<Feeling>  // up to 5 from fixed taxonomy
  - notes_richtext: Option<String> // HTML or JSON-richtext
  - created_at: DateTime

Feelings (fixed list): Nostalgic, Cozy, Melancholic, Epic, Wholesome, Bittersweet, Heartwarming, Chill, Adventurous, Uplifting, Mysterious, Somber


## Storage and asset caching

We use a trait-based abstraction in app_core. Platform crates implement the traits.

Traits (conceptual signatures):
- DataStore: CRUD for Work and Mnemon; atomic save on create
  - get_work_by_provider_ref, get_work, put_work, list_mnemon_ids, get_mnemon, put_mnemon
- AssetStore: binary assets for cover/music
  - put_asset(bytes, kind) -> local_uri
  - get_asset_uri(asset_id) -> local_uri
  - remove_asset(asset_id) (not used in MVP)
- Tx/Unit-of-Work optional; MVP can do simple sequential saves

Web implementation:
- Records (Work/Mnemon): IndexedDB via a Rust binding such as rexie or idb
- Assets (covers/music): Store as Blobs in IndexedDB and expose blob: URLs; persist blob URL or a logical key and regenerate URL on load
- Optional SW: A service worker can be added post-MVP; not required—IndexedDB is sufficient

Desktop implementation:
- Records (Work/Mnemon): SQLite (rusqlite or sqlx with SQLite feature)
- Assets: Files on disk under an app data directory (use `directories`/`dirs`), store file paths in DB
- Fetch assets and write to disk on save

Mobile (post-MVP) mirrors desktop:
- Records: SQLite
- Assets: Files in app sandbox

Schema suggestion (SQLite):
- works(id TEXT PK, type TEXT, title_en TEXT, release_year INTEGER NULL,
  cover_uri TEXT NULL, music_uri TEXT NULL, origin TEXT, created_at TEXT)
- work_providers(work_id TEXT, source TEXT, provider_id TEXT, PRIMARY KEY(work_id, source, provider_id))
- mnemons(id TEXT PK, work_id TEXT, finished_date TEXT NULL, feelings TEXT, notes_richtext TEXT NULL, created_at TEXT)


## Audio playback

Abstract through an `AudioPlayer` trait:

- load(source: AudioSource) -> AudioHandle
- play(handle), pause(handle), stop(handle)
- can_autoplay() -> bool  // returns platform hint; on web often false until user gesture
- on_end(handle, callback)

Implementations:
- Web: HTMLAudioElement via web_sys; use object URLs for cached blobs; respect autoplay policies (start playing only after a user click if blocked)
- Desktop/Mobile: rodio (cpal-backed). Load from local file path; simple play/pause/stop; one track at a time in MVP

Autoplay behavior:
- Attempt to auto-play in hero if `can_autoplay()` is true; else show a single-tap Play button
- Always stop the current track before switching to “Next Surprise”


## Provider architecture and exact-ID dedupe

We unify provider results behind traits in app_core.

Concepts:
- ProviderKind: { Movies, TvAnime, Games }
- SearchQuery { text: String, lang: "en" }
- ProviderResult { provider_source, provider_id, title_en, release_year, cover_http_url?, music_http_url? }

Traits:
- Provider: fn search(kind, query) -> Vec<ProviderResult>
- AssetFetcher: fn fetch_cover(url) -> bytes, fn fetch_music(url) -> bytes

Exact-ID dedupe rules:
- Dedupe within a single provider by (provider_source + provider_id)
- On save, if a Work already exists with the same ProviderRef, reuse it (do not create new)
- Cross-provider mapping is out of scope for MVP

Notes on specific providers:
- Movies/TV: TMDB is commonly used; requires API key and attribution
- Anime: AniList GraphQL or Jikan (unofficial) for MAL; requirements differ
- Games: IGDB requires OAuth via Twitch developer creds
- Music preview: Many content providers don’t ship audio. For a pragmatic MVP, use a “MusicResolver” that can:
  - Try iTunes/Apple Search API (no key) to find a 30s preview by title + “soundtrack” or similar query
  - Fallback to “no music” if nothing is found or CORS disallows caching
- Important: Do not hardcode secret keys in the client. If a key is required (TMDB/IGDB), use a tiny proxy service that injects the key server-side, or store the key only in local dev env. If no proxy configured, the UI must gracefully fall back to Manual Entry and/or partial results.

Caching flow on Save (provider-backed work):
1) Store minimal Work record immediately (optimistic), origin=provider
2) Fetch cover/music bytes in background
3) Put assets in AssetStore → update Work.cover/music URIs
4) UI: If assets unavailable yet, show placeholders; on update, hero/details should reactively refresh

Manual Entry:
- Create Work with origin=manual
- No provider refs, no cover/music
- Uses placeholders in UI


## Offline strategy

- App must run and show data offline from first launch after a successful Save
- Web: IndexedDB-based persistence for records and binary assets
- Native: SQLite + filesystem
- When offline:
  - Search falls back to Manual Entry only
  - Adding a mnemon that references a provider requires connectivity; the UI should clearly state the offline limitation and suggest Manual Entry
- No service worker is required in MVP; can be added later for shell caching


## Using dx (Dioxus 0.7) to run/build

- Dev (web): use dx to serve the app targeting the web renderer (see Dioxus 0.7 docs).
- Dev (desktop): use dx to run with the desktop renderer.
- Build (web, release): use dx to build a release bundle for web (WASM).
- Build (desktop, release): use dx to build a release bundle for desktop.
- Mobile: post-MVP; do not block MVP on packaging.

Notes:
- The exact flags for selecting the platform (e.g., --platform or -p) may vary; follow the Dioxus 0.7 CLI docs.
- Ensure wasm32-unknown-unknown target is installed for web builds.

Agent hints:
- Do not introduce long-running commands in CI steps (no watch/serve loops).
- Keep build steps explicit and terminating.
- Use conditional compilation to avoid pulling native-only crates into the wasm target.


## Conventions for agents and contributors

- No comments unless needed to explain complex algorithms or non-obvious decisions
- Only add code that is exercised by the current app flow; remove or defer anything unused so reviews focus on observable behavior
- Keep functions small and pure when possible; isolate side effects (I/O, HTTP, audio) behind traits
- Prefer constructor functions or small service structs for dependency injection (AppStore builds platform services and passes them to UI)
- Avoid global mutable state; if needed, wrap in Signals or an explicit store
- Validate only what the product requires (Work required; everything else optional)
- Strictly follow the MVP feature scope; avoid implementing edit/delete, search filters, or collections
- When a provider is not configured (no keys/proxy), present Manual Entry
- Always dedupe by exact provider ID at suggestion/result time and on save
- Cache assets locally and handle failures gracefully (show placeholders; never crash)


## Minimal type outlines (for orientation)

These are orientation-only; the coding agent can adapt exact signatures.

- Domain (app_core)
  - Models: Work, ProviderRef, Mnemon, Feeling
  - Traits:
    - DataStore
    - AssetStore
    - AudioPlayer
    - Provider (search)
    - AssetFetcher (HTTP fetch for cover/music)
  - Services:
    - AddMnemonService { fn add_from_provider(result, personalize) -> MnemonId; fn add_manual(...)}
    - SurpriseService { fn random_mnemon_id() -> Option<MnemonId> }

- UI (app_ui)
  - AppStore: holds implementations of DataStore, AssetStore, AudioPlayer, Provider registry
  - Routes: Home, AddStep1, AddStep2, Details
  - Components: Hero, AddSearch, AddPersonalize, DetailsView
  - State: Selected suggestion, Pending personalization, Current hero mnemon

- Platform crates
  - platform_web: IndexedDBStore, WebAssetStore (blob URLs), WebAudioPlayer (HTMLAudioElement), WebProviderAdapters, WebFetcher
  - platform_desktop: SqliteStore, FsAssetStore, RodioAudioPlayer, NativeProviderAdapters, NativeFetcher


## Legal and content notes

- Respect provider ToS and attribution (e.g., TMDB’s attribution requirements)
- Do not redistribute copyrighted media beyond fair use; for music, use short official previews (where licenses allow) and cache locally for offline if permitted
- If a source forbids local caching, skip caching and rely on runtime fetch; still respect offline-first by gracefully disabling the audio in offline mode


## What success looks like (technical)

- The desktop and web targets can:
  - Add a mnemon via provider search (when configured) or via manual entry (offline or no-key)
  - Cache cover/music locally and render/play offline
  - Show the Surprise Me hero that auto-plays music when allowed, otherwise offers a one-tap play
  - Navigate to read-only details
- The codebase compiles cleanly for wasm32-unknown-unknown and native desktop
- Platform-specific code is isolated; app_core contains no platform imports


## Open choices the coding agent can decide (within scope)

- IndexedDB binding crate choice on web (rexie/idb)
- SQLite binding crate choice on native (rusqlite/sqlx)
- Rich-text representation (HTML string vs JSON delta) — keep it simple for MVP
- How to locate short music previews (e.g., iTunes Search API). If unavailable, omit audio for that Work.


---
This document is intended to be an “AGENTS.md-style” guide tailored for Rust + Dioxus. It encodes decisions and safe defaults so agents can implement the MVP without deep framework spelunking.
