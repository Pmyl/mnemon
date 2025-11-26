# Mnemon — Living Implementation Plan


This document is the evolving build plan for Mnemon’s MVP, grounded in `PROJECT.md`, `WIREFRAMES.md`, `TECHNOLOGIES.md`, and `DIOXUS.md`. After every review cycle, we revise this plan so it always reflects the next actionable steps.

## Ground Rules
- **Scope guardrails:** Respect MVP scope and exclusions exactly as defined in the specification files.
- **Technology guardrails:** Follow the workspace layout, Dioxus 0.7 patterns, and “no unused code” policy documented in `TECHNOLOGIES.md`.
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

## Phase 3 — Read-only Details & Audio Stub
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 4 — Memory Details view with audio controls (stub)** | Render Details route, share components for feelings/notes, and wire a stubbed audio controller. | Selecting “Open Memory” shows read-only detail screen with placeholder cover/audio player that reacts to play/stop in UI (no actual audio yet). | Step 3 |

## Phase 4 — Persistence Foundations
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 5 — Web persistence (IndexedDB)** | Persist Works/Mnemons/assets metadata for the web target. | In web build, mnemons survive page reloads; empty state returns only when storage cleared. Manual entries still surface in hero and details after reload. | Step 4 |
| **Step 6 — Desktop persistence (SQLite + filesystem)** | Add desktop storage adapters mirroring web behavior. | Desktop build persists mnemons across restarts and stores placeholder assets on disk. | Step 5 |

## Phase 5 — Provider Search Experience
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 7 — Provider search with deterministic fixtures** | Introduce provider abstractions plus fixture-backed results hooked into Step 1 UI. | Searching shows grouped provider results with exact-ID dedupe; selecting a fixture Work populates hero/details using cached metadata. | Step 5 |
| **Step 8 — Real provider integrations** | Connect to live APIs (TMDB, AniList/Jikan, IGDB, iTunes preview) with graceful fallback. | Live search returns real titles when API keys/configured; manual entry path activates automatically when offline or keys missing. | Step 7 |

## Phase 6 — Asset Caching & Audio Playback
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 9 — Cover and theme caching pipeline** | Fetch and persist cover/music assets, updating Works with local URIs. | After saving a provider-backed mnemon, cover art appears in hero/details; audio play button streams cached preview when available; placeholders show when assets missing. | Step 8 |
| **Step 10 — Surprise hero audio behavior** | Enforce autoplay policy, stop-on-switch, and Play fallback per MVP rules. | Hero auto-plays when platform allows, otherwise surfaces a Play control; switching memories stops previous audio and (re)autoplays subject to policy. | Step 9 |

## Phase 7 — Offline Guarantees & Refinement
| Step | Goal | Reviewable Outcome | Dependencies |
| --- | --- | --- | --- |
| **Step 11 — Offline resilience audit** | Validate offline flows, add UX cues, and document limitations. | Demonstrated walkthrough (web & desktop) showing manual entry offline, existing mnemons surfacing with cached assets, and copy guiding offline behavior. | Step 10 |
| **Step 12 — Quality hardening & release playbook** | Polish code/tests/docs and produce release artifacts. | Clean `cargo fmt/clippy/test` runs, migration of duplicated docs into rustdoc, generated web bundle & desktop binaries with deployment checklist. | Step 11 |

## Review & Feedback Log
| Revision | Reviewer Notes | Plan Adjustments |
| --- | --- | --- |
| rev. 0 | Initial plan | Replaced with rev. 1 per feedback (ensure each step produces reviewable behavior). |
| rev. 1 | Step 2 complete | Implemented headerless design with: (1) Empty state with "tap anywhere to begin", (2) Dark mode styling, (3) Click interaction loads sample data for demo, (4) Auto-cycling hero with smooth transitions, (5) Rotating notes with adaptive reading time. Updated WIREFRAMES.md and PROJECT.md to reflect simplified design without header. Theme toggle deferred to future settings page. |
| rev. 2 | Step 3 complete + Phase 2 improvements | Implemented complete Add Mnemon flow with: (1) Modal overlay UI with two-step wizard, (2) Step 1: Manual entry form (Type, Title, Notes, Feelings with emojis), (3) Step 2: Optional dates (Release Year, Finished Date), (4) Feelings display as emoji+name chips with 5-max selection, (5) In-memory storage using signals, (6) Click-anywhere interaction opens Add flow, (7) New mnemons immediately surface in hero with auto-cycle, (8) Manual entries show placeholder background when no cover image, (9) Fixed timer bug where multiple timers were spawned per mnemon added. Full vertical slice working end-to-end. |
| rev. 3 | _Pending_ | _TBD_ |

When you review a step’s implementation, we will capture your notes here and adjust downstream steps before moving forward.
