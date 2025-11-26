# WIREFRAMES — Mnemon (MVP)

Scope
- Persona: Single private user. No accounts, no sharing.
- MVP homepage: header + full-page Surprise Me hero (only). The homepage is Surprise Me.
- Add flow: two steps (Pick Work → Personalize), with manual fallback.
- Memory Details: read-only view.
- Local-first, offline-capable. Assets (covers/music) cached locally.

Global UI
- Header: none in MVP (full immersive experience).
- Footer: none in MVP (clean, minimal).
- Theme: Dark mode only in MVP (Light mode and theme toggle deferred to future settings page).
- Typography/visual style:
  - Nostalgic, soft grain allowed on imagery.
  - Image-forward; minimal text lines, subtle overlays.
- Interaction:
  - Click/tap anywhere on the page to add a new mnemon.
  - Hero auto-cycles through memories every 10 seconds.
  - Notes rotate with adaptive reading time based on length.
- Audio behavior (global rules):
  - Auto-play theme music when the platform permits. If not permitted, show a single prominent Play control (one user gesture starts audio).
  - Switching surfaced memory (via Next Surprise or navigation) stops any currently playing track before starting the new one.

Feelings taxonomy (fixed, up to 5 per mnemon)
Nostalgic, Cozy, Melancholic, Epic, Wholesome, Bittersweet, Heartwarming, Chill, Adventurous, Uplifting, Mysterious, Somber


1) Homepage — Full-page Surprise Me Hero
- The homepage itself is Surprise Me. Full immersive, headerless experience.

- Layout:
  HERO (fills entire viewport)
  - Background: Cover image (if available), full-bleed, subtle vignette/grain overlay.
  - Foreground overlay:
    - Lower left: Rotating note quotes (italic, white text, fades between notes)
      - Displays 2 randomly selected notes from current mnemon
      - Each note shows for adaptive duration based on reading time (3-8 seconds)
      - Smooth fade transitions between notes
    - Bottom right: Memory metadata (footnote style)
      - Type icon (🎬/📺/🎮) + Title
      - Feelings chips (subtle, semi-transparent)
  - Auto-cycle: Automatically advances to next mnemon every 10 seconds
    - Smooth slide transition (current slides left, next slides in from right)
  - Tap/click behavior:
    - Click anywhere on the page to add a new mnemon (launches Add flow)
  - Audio (future):
    - If theme available and auto-play allowed: music starts on hero render
    - Else: [Play ▶] button within hero overlay
  - If theme not available: omit audio controls quietly

- Empty state (no mnemons):
  HERO (centered content; no background cover)
  - Title: "Add your first mnemon"
  - Subtitle: "Capture a great movie, TV/anime, or game you loved. Nostalgia awaits."
  - Tap/click behavior: Click anywhere to add first mnemon

- Keyboard/navigation:
  - Escape or Back from Add flow or details returns to hero.
  - Spacebar: pause/resume auto-cycle (future)

- Cycling behavior:
  - Auto-advances through all mnemons in sequence
  - Wraps around to first after reaching last
  - Continuous loop while on homepage


2) Add Mnemon — Two-step flow (minimal required)

Entry points
- Click/tap anywhere on the homepage hero.

Step 1: Pick the Work

- Layout:
  HEADER
  ─────────────────────────────────────────────────────────
  CONTENT
  - Title: “Add a mnemon”
  - Search box (placeholder: “Search movies, TV/anime, and games in English…”)
  - Results area (live suggestions; grouped by Type):
    - Movies
      - [Cover] Title (Year)
    - TV/Anime
      - [Cover] Title (Year)
    - Games
      - [Cover] Title (Year)
  - Dedupe:
    - Collapse results by exact provider identity: (provider_source + provider_id).
    - If the chosen provider item already exists locally, selecting it reuses that Work (no duplicates).
    - No cross-provider dedupe in MVP.
  - No results:
    - Inline message: “Didn’t find it?”
    - [Create Manual Entry] — leads to Manual form (below)
  - Offline:
    - Show tip: “You’re offline. Use Manual Entry.”
    - [Create Manual Entry]

- Manual Entry (fallback)
  - Fields:
    - Type (required): [Movie | TV/Anime | Game]
    - Title (English) (required): [text]
    - Release Year (optional): [YYYY]
  - Note: no cover or theme music in MVP for manual entries (hero shows placeholder, no audio).

- Selecting a result or completing Manual Entry → Continue to Step 2.

Step 2: Personalize (all optional)
- Layout:
  HEADER
  ─────────────────────────────────────────────────────────
  CONTENT
  - Finished date: [date]
  - Feelings (choose up to 5; chips/toggles):
    - Nostalgic, Cozy, Melancholic, Epic, Wholesome,
      Bittersweet, Heartwarming, Chill, Adventurous,
      Uplifting, Mysterious, Somber
  - Notes (rich text):
    - Toolbar: [B] [I] [• List] [Quote]
    - Multiline editor
  - Actions:
    - [Save] (primary)
    - [Back] (secondary)

- Save outcomes:
  - Provider result:
    - If provider ID already exists locally: link mnemon to existing Work.
    - Else: create Work; cache cover/music locally when possible.
  - Manual entry:
    - Create Work with placeholder cover and no music.
  - Navigate:
    - Return to Homepage hero (recommended); the new mnemon can surface immediately.
    - Optional toast: “Mnemon added.”


3) Memory Details — Read-only (future)

- Layout:
  CONTENT (with back gesture/button)
  - Top section:
    - Cover image (cached) or placeholder (manual Work)
    - Title (English) (prominent)
    - Year • Type icon
  - Audio:
    - If theme music exists:
      - Auto-play on entry if allowed; else show [Play ▶]
    - Simple scrubber and pause; no playlist or queue in MVP
  - Meta:
    - Feelings chips (if any)
    - Finished on: YYYY-MM-DD (if provided)
  - Notes:
    - Rich text, read-only
  - No actions for edit/delete in MVP.

- Navigation:
  - [Back] returns to Homepage hero
  - Audio stops on back navigation


Microcopy (examples)
- Empty state CTA: "Add your first mnemon" / "Tap anywhere to begin"
- Search placeholder: "Search movies, TV/anime, and games in English…"
- No results: "Didn't find it? Create a Manual Entry."
- Manual entry labels: "Type", "Title (English)", "Release year"
- Save confirmation (non-blocking toast): "Mnemon added."


States and Behaviors Summary
- Homepage
  - No mnemons → empty state with [+ Add]
  - Has mnemons → hero shows one memory; auto-play if permitted; Next Surprise randomizes
- Add flow
  - Online search with exact-ID dedupe per provider
  - Offline/manual fallback always available
  - Step 2 fields optional; Save works with Work only
- Details
  - Read-only; no edit/delete
  - Audio behavior consistent with homepage rules
- Offline
  - Search disabled (manual only)
  - Existing cached covers/music still display/play


Out of Scope (MVP)
- Editing or deleting mnemons
- Lists/grids on homepage (hero-only)
- Ratings, status, shelves/collections, people/location, attachments
- Notifications, import/export, accounts/sharing
- Advanced resurfacing logic (random only)
- Accessibility localization beyond basics