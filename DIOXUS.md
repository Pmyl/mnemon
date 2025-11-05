# DIOXUS.md — Agent playbook for Dioxus 0.7

Purpose
- Give you a compact, copy-pasteable mental model to build Mnemon with Dioxus 0.7 without constantly jumping to the docs.
- Focus on: components/pages, signals/state, routing, assets/audio, and build/run/test with the Dioxus CLI.
- Single codebase, multiple targets. Use the CLI to pick web or desktop as needed.

De-duplication
- Keep this file high-level and process-focused.
- Move concrete API/trait signatures and module details into code docs next to the implementation and remove duplication here (see TECHNOLOGIES.md policy).


1) Components and Pages

Anatomy
- Components are Rust functions that return `Element`. Prefer `#[component]` for typed props and cleaner signatures.
- Markup is declared with `rsx! { ... }`.
- Events like `onclick` receive closures. Update state via signals (see next section).

Typical component shape
- Signature: `#[component] fn Xyz(props: Props) -> Element { ... }`
- State: `let mut count = use_signal(|| 0);`
- Markup: `rsx! { div { "Count: {count}" button { onclick: move |_| count += 1, "Inc" } } }`

Pages
- Treat pages as components routed to by the router:
  - Home (Surprise hero): shows one mnemon and its audio, with “Next Surprise” and “Open Memory”.
  - Add Step 1: provider search/manual entry.
  - Add Step 2: personalize.
  - Details: read-only memory view.

Styling
- Use standard HTML-like tags in `rsx!` with `class`/`style` attributes.
- You can bring your own CSS approach (utility classes, embedded styles, etc.). Keep it simple for MVP.
- Images are just `<img>` or `<div style="background-image: url(...)">` depending on the desired effect.

Tips
- Keep components small and pure. Move I/O behind injected services and call async tasks from controlled points (e.g., “Save” button).
- Prefer composition over large monolith components.


2) State and Signals

Signals
- `use_signal` provides reactive state. Reading a signal in `rsx!` subscribes the component to updates. Writing to a signal re-renders dependents.
- Derive computed values via `use_memo` if you want memoized transformations.

App-wide state
- Create a lightweight “AppStore” that holds:
  - Storage, asset, provider, and audio service handles.
  - The current theme, and any global flags (e.g., autoplay allowed).
- Provide it via context at the app root and read it with `use_context`. Keep it immutable; mutate through well-defined methods on services.

Async
- Use task spawns from Dioxus utilities where appropriate to run async operations (e.g., provider search, caching assets). Keep UI responsive and show placeholders while loading.


3) Routing (Typesafe)

Pages and routes for Mnemon MVP
- “/” → Home (Surprise hero)
- “/add” → Add Step 1 (Pick Work)
- “/add/personalize” → Add Step 2 (Personalize)
- “/m/:id” → Details (read-only)

Patterns
- Define a router with a central enum of routes and a switch that renders a page component for each route.
- For navigation from code, call the router’s navigation API with the route (e.g., navigate to details after selecting a memory).

Guidelines
- Keep route params typed (e.g., `id` as string/UUID).
- Avoid global mutable navigation state; let the router drive what’s visible.


4) Assets and Audio

Images (covers)
- Source of truth: local URIs (offline-first).
- Web:
  - Covers cached as Blobs in IndexedDB; use `blob:` URLs for `<img>`.
- Desktop:
  - Covers cached as files on disk; use file paths or file:// URIs.

Audio (theme music)
- Web:
  - Use the platform audio element (e.g., an HTML audio element via bindings).
  - Respect autoplay policies: try to autoplay; if blocked, show a single-tap Play control.
- Desktop:
  - Use a simple audio backend (e.g., a native audio player crate).
  - Single track at a time for MVP; stop current track before playing the next.
- Always stop audio when leaving the hero/details or when switching to the next surprise.

Placeholders
- If a Work has no cover: render a static placeholder.
- If a Work has no theme: hide audio controls.

Caching flow (provider-backed save)
- Save minimal record → start background fetch for cover/music → persist asset(s) locally → update record with local URIs → UI refreshes reactively.
- If offline or fetch fails: keep placeholders; do not block saving the mnemon.


5) Build, Run, and Test (Dioxus 0.7 CLI)

Install and verify
- Rust stable + `wasm32-unknown-unknown` target for web builds.
- Dioxus CLI installed and matches the major version you’re targeting.
- Verify with `dx --version` and inspect available commands with `dx help`.

Common commands (use the CLI’s help for exact flags)
- Develop (web): run the dev server that builds, watches, and serves the app for the web renderer. Hot reload speeds iteration.
- Develop (desktop): run a native window using the desktop renderer.
- Build (web): produce a release bundle (WASM + assets) for deployment.
- Build (desktop): produce a release binary/bundle.
- Format RSX: run the CLI’s formatter for RSX markup (keep it consistent).
- Check: run CLI checks on the project to catch common issues.
- Clean: remove output artifacts.

Notes
- Ensure the WASM target is installed prior to web builds.
- Keep long-running dev servers out of CI. In CI, prefer deterministic, terminating steps (checks, test, build).
- Use conditional compilation for platform-specific glue to avoid pulling native-only crates into the web target.

Testing approach for Mnemon MVP
- Core/domain (pure Rust):
  - Unit test the data model, dedupe logic, and service algorithms with `cargo test`.
- UI:
  - Add lightweight “smoke” tests where feasible (e.g., render-to-string on the server side if you choose to bring a renderer suitable for tests).
  - Keep most behavior verifiable through small, pure functions and services. Wire them from components with minimal glue.
- Tooling:
  - Run `dx check` as a fast sanity pass.
  - Run `cargo clippy` and `cargo fmt` where applicable.


6) Minimal implementation map (Mnemon)

Pages/components
- App root:
  - Theme provider (dark/light)
  - AppStore/context wiring (storage, providers, audio)
  - Router with routes: “/”, “/add”, “/add/personalize”, “/m/:id”
- Home (Surprise hero):
  - Reads a random mnemon; renders cover/title/year/type
  - Tries to autoplay audio; else shows Play
  - Buttons: Next Surprise, Open Memory
- Add Step 1 (Pick Work):
  - Search: provider-backed results (dedupe by exact provider ID). Manual fallback.
- Add Step 2 (Personalize):
  - Finished date, feelings (chips; up to 5), rich text notes
  - Save → creates/links Work + Mnemon, then returns to the hero
- Details (read-only):
  - Cover, audio player, feelings/date, read-only notes

Services (behind traits; injected via AppStore)
- Storage: get/put Works & Mnemons, list IDs, fetch by ID/provider ref
- Asset store: put/get local URIs for covers/music
- Providers: search Movies / TV/Anime / Games (English titles). Exact-ID dedupe in suggestions and on save.
- Audio: play/stop and handle autoplay policy


7) Agent guardrails

- Single codebase: share components across targets. Only add platform cfg glue for storage, assets, providers, and audio.
- Keep functions small and pure where possible. Effects live in services.
- Validate only what MVP requires (Work required; others optional).
- Implement Empty State first (no mnemons).
- Implement Manual Entry path; never block the user on missing provider keys.
- Audio must never lock up UI; always stop before playing the next track.
- Migrate concrete APIs into rustdoc next to their definitions and remove duplication from TECHNOLOGIES.md/DIOXUS.md when implemented.


Cheat sheet (inline examples without full code blocks)
- Component with state: `#[component] fn Counter() -> Element { let mut c = use_signal(|| 0); rsx!{ button{ onclick: move |_| c += 1, "Inc" } span{"{c}"} } }`
- Route navigation (conceptual): `let nav = use_navigator(); nav.push("/m/123");`
- Signal read/write: `let mut s = use_signal(|| false); let t = *s.read(); s.set(!t);`
- Button handler: `onclick: move |_| do_something()`
- Show image: `img { src: "{cover_uri}", alt: "Cover" }`
- Show audio control on web: `button { onclick: move |_| play_audio_for(work_id), "Play" }`


Appendix: When in doubt
- Keep UI declarative and side-effects outside components.
- If a feature forces platform-specific APIs, hide it behind a small trait and provide per-target implementations via conditional compilation.
- Prefer simple, reliable patterns that are easy to test for core logic; accept that rich UI affordances can remain thinly tested in MVP.
