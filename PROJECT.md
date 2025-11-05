# Mnemon — MVP Spec (Surprise-first, Minimal, Local-first)

Mnemon is a private, nostalgia-focused app to quickly capture and resurface great memories from Movies, TV/Anime, and Games. MVP is intentionally minimal: all local, no sign-in, no sharing, no edits after save. The homepage is a header plus a full-page Surprise Me hero that auto-plays the related theme music.

## Principles and Scope
- Persona: Single private user journaling their favorite “nerd” memories for personal nostalgia.
- Platforms: Start with web; tech choice will later support native. Ignore tech details here.
- Privacy: Always private. No sharing or accounts in MVP.
- Data: Local-first and offline-capable. All user data and cached assets live on the device.
- Accessibility/Localization: Out of scope for MVP beyond basic semantics.
- Settings: Only theme toggle (Dark/Light).

## Core Concepts
- Work
  - A piece of media of Type: Movie, TV/Anime, or Game.
  - Source: From an external provider (preferred) or Manual Entry (fallback).
  - Goal: Provide title (English), year, cover art, and a theme music track if available.
- Mnemon
  - A user-created memory referencing exactly one Work.
  - Personal fields are optional; this is not a tracking app—no ratings/status.

## Homepage (Minimal, Surprise-first)
- Layout: Header + Full-page Hero (no other sections in MVP).
- Header:
  - Logo (left)
  - + Add button
  - Theme toggle (Dark/Light)
- Full-page Hero (Surprise Me mode):
  - Displays a single randomly selected existing mnemon (if any).
  - Visual: Large cover image background with subtle vignette/grain; overlaid title (English), year, and type icon.
  - Music: Plays the Work’s theme music automatically when allowed by the platform; otherwise shows a single-tap “Play” icon to initiate audio.
  - Optional visible fields: feelings (chips), finished date (if present), first ~2 lines of notes.
  - Actions:
    - Next Surprise: randomize to another mnemon.
    - Open Memory: navigate to read-only Memory Details.
- Empty State (no mnemons):
  - Centered CTA: “Add your first mnemon” with a large + Add button.
  - Subtext: “Capture a great movie, TV/anime, or game you loved. Nostalgia awaits.”

## Add Mnemon Flow (Two Steps, Minimal Required)
- Entry points:
  - Header: + Add
- Step 1 — Pick the Work
  - Search (English titles) across Movies, TV/Anime, Games via providers.
  - Result item shows: cover (if provided), Title (Year), Type.
  - Group results by Type.
  - Exact-ID Dedupe (MVP rule):
    - Dedupe results by exact provider identifier within the same provider (provider_source + provider_id).
    - If multiple results share the same provider ID, collapse into one.
    - If the selected provider item already exists locally (same provider_source + provider_id), reuse that Work instead of creating a new one.
    - No cross-provider dedupe in MVP.
  - Manual Entry (fallback) when no suitable results:
    - Fields (required unless noted): Type (Movie | TV/Anime | Game), Title (English), Release Year (optional)
    - No cover or theme music fields. Manual entries use placeholder image and no music in MVP.
- Step 2 — Personalize (all optional)
  - Finished date (single date)
  - Feelings (fixed taxonomy; choose up to 5):
    - Nostalgic, Cozy, Melancholic, Epic, Wholesome, Bittersweet, Heartwarming, Chill, Adventurous, Uplifting, Mysterious, Somber
  - Notes (rich text: Bold, Italic, Bulleted list, Quote)
- Save:
  - Creates the Mnemon linked to the selected Work (reusing an existing Work if matched by exact provider ID).
  - If Manual Entry, creates a new Work (no cover/music) and the Mnemon.
  - Post-save navigation: return to the homepage hero (Surprise Me) or optionally show the created Memory Details, then return to hero.
- Constraints:
  - No edits after save in MVP.
  - No delete in MVP.
  - No attachments/uploads.

## Memory Details (Read-only)
- Header: Back to Home
- Content:
  - Cover (provider-cached) or placeholder (manual).
  - Title (English), release year, Type.
  - Theme music player (auto-play if permitted; otherwise tap-to-play).
  - Feelings chips (if any).
  - Finished date (if provided).
  - Rich text notes (read-only).
- No edit/delete actions in MVP.

## Data Model (MVP)
- Work
  - id (local UUID)
  - type: enum {movie, tv_anime, game}
  - title_en: string (required)
  - release_year: number? (optional)
  - cover_image_local_uri: string? (optional; local cached path/URI)
  - theme_music_local_uri: string? (optional; local cached path/URI)
  - providers: array of ProviderRef? (optional, can be multiple in future)
    - ProviderRef = { provider_source: string, provider_id: string }
  - created_at: timestamp
  - origin: enum {provider, manual}
- Mnemon
  - id (local UUID)
  - work_id: Work.id (required)
  - finished_date: date? (optional)
  - feelings: string[] (0–5 from fixed taxonomy)
  - notes_richtext: string (stored in a simple portable format; e.g., HTML/JSON-markup) (optional)
  - created_at: timestamp

Notes:
- Exact-ID Dedupe:
  - Query-time: collapse suggestion rows by (provider_source + provider_id).
  - Save-time: if a Work with matching ProviderRef exists, reuse it (no new Work).
  - Cross-provider dedupe is not attempted in MVP.
- Manual entries:
  - No ProviderRef, no cover, no theme music.
  - Use a static placeholder image; hero will show no music controls for manual entries.

## Offline and Persistence
- All data and cached assets (covers, theme music) persist locally.
- The app must load and function fully offline after first-run caching.
- Providers are only used during search; if offline, only Manual Entry is
 available.
- Caching policy:
  - On selecting a provider result, cache the cover image and theme music locally before finalizing Save if possible; otherwise, save Work with missing assets and attempt background caching when online (optional).
  - Handle missing/failed asset caching gracefully (use placeholders).

## Acceptance Criteria
Add Mnemon
- I can search by English title; results appear grouped by Type.
- Dedupe by exact provider ID within a provider ensures no duplicate rows for the same item.
- Selecting a previously chosen provider item reuses the existing Work (no duplication).
- Manual Entry works when nothing is found; Type and Title are required, Year optional.
- Step 2 fields are all optional. Save works with Work only.
- After Save, the mnemon exists and can surface in Surprise Me.

Homepage Hero (Surprise Me)
- With at least one mnemon:
  - The hero shows a single mnemon with an image-forward presentation.
  - Next Surprise changes the surfaced mnemon.
  - If the mnemon’s Work has a cached theme music, it auto-plays when permitted; otherwise a single tap starts playback.
  - Open Memory navigates to read-only Memory Details.
- With zero mnemons:
  - The hero shows an empty state with a prominent + Add button.

Memory Details (Read-only)
- Shows Work (cover or placeholder), title, year, type.
- Plays theme music if present (auto-play if permitted; tap-to-play if not).
- Shows feelings, finished date, and notes when present.
- There is no edit or delete action available.

Data and Offline
- All data persists locally across sessions.
- Covers and music selected from providers are cached locally and render offline.
- Manual entries correctly show a placeholder cover and no music.

Settings
- Theme toggle exists and persists locally (Dark/Light only).

Out of Scope (MVP)
- Editing or deleting mnemons or works.
- Lists/grids of memories on the homepage (hero-only in MVP).
- Search/filter of saved items.
- Ratings, status, collections/shelves, people/location, attachments.
- Notifications, import/export, accounts/sharing.
- Advanced resurfacing logic (MVP uses simple randomization).
- Accessibility enhancements or additional locales beyond basics.

## Open Questions (Post-MVP)
- Should delete be added before broader usage?
- Enhanced resurfacing (anniversaries, seasonality, “nostalgia score”).
- Cross-provider dedupe/mapping for the same Work across different sources.
- Allow linking a manual Work to a provider later (to fetch cover/music).
- Additional feelings taxonomy or theming presets for visuals/sonics.
