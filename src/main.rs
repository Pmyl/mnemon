//! Mnemon - A nostalgia-focused app to capture and resurface memories from Movies, TV/Anime, and Games

use dioxus::prelude::*;
use rand::seq::SliceRandom;
use tracing::info;
use uuid::Uuid;

mod constants;
mod data;
mod models;
mod providers;
mod storage;

use constants::*;
use data::{SearchService, SearchStatus};
use models::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    AssetOptions::css().with_static_head(true)
);
const TAILWIND_CSS: Asset = asset!(
    "/assets/tailwind.css",
    AssetOptions::css().with_static_head(true)
);

fn main() {
    dioxus::launch(App);
}

// =============================================================================
// APPLICATION STATE
// =============================================================================

/// Combined view of a Mnemon with its associated Work data
/// Used for displaying in the UI without needing to look up Work separately
#[derive(Clone, PartialEq, Debug)]
struct MnemonWithWork {
    mnemon: Mnemon,
    work: Work,
}

/// Application state container
#[derive(Clone)]
struct AppState {
    works: Signal<Vec<Work>>,
    mnemons: Signal<Vec<Mnemon>>,
    /// Shuffled indices for randomized display order
    shuffled_indices: Signal<Vec<usize>>,
    /// Whether initial data has been loaded from storage
    loaded: Signal<bool>,
}

impl AppState {
    fn new() -> Self {
        Self {
            works: Signal::new(Vec::new()),
            mnemons: Signal::new(Vec::new()),
            shuffled_indices: Signal::new(Vec::new()),
            loaded: Signal::new(false),
        }
    }

    /// Load data from IndexedDB storage (async)
    async fn load_from_storage(&mut self) {
        let persisted = storage::load_all_async().await;
        info!(
            "Loaded {} works and {} mnemons from IndexedDB",
            persisted.works.len(),
            persisted.mnemons.len()
        );

        // Create shuffled indices for randomized display order
        let mut indices: Vec<usize> = (0..persisted.mnemons.len()).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        self.works.set(persisted.works);
        self.mnemons.set(persisted.mnemons);
        self.shuffled_indices.set(indices);
        self.loaded.set(true);
    }

    /// Check if data has been loaded from storage
    fn is_loaded(&self) -> bool {
        *self.loaded.read()
    }

    /// Get all mnemons with their associated works (in original order)
    fn get_mnemons_with_works(&self) -> Vec<MnemonWithWork> {
        let works = self.works.read();
        let mnemons = self.mnemons.read();

        mnemons
            .iter()
            .filter_map(|m| {
                works
                    .iter()
                    .find(|w| w.id == m.work_id)
                    .map(|w| MnemonWithWork {
                        mnemon: m.clone(),
                        work: w.clone(),
                    })
            })
            .collect()
    }

    /// Get the mnemon index for a given position in the shuffled order
    fn get_shuffled_index(&self, shuffled_position: usize) -> Option<usize> {
        self.shuffled_indices.read().get(shuffled_position).copied()
    }

    /// Get the total number of mnemons
    fn mnemons_count(&self) -> usize {
        self.shuffled_indices.read().len()
    }

    /// Find a work by provider reference
    fn find_work_by_provider_ref(&self, provider_ref: &ProviderRef) -> Option<Work> {
        self.works
            .read()
            .iter()
            .find(|w| {
                w.provider_ref
                    .as_ref()
                    .map(|pr| pr.matches(provider_ref))
                    .unwrap_or(false)
            })
            .cloned()
    }

    /// Check if a work with the given provider ref already has a mnemon
    fn has_mnemon_for_provider_ref(&self, provider_ref: &ProviderRef) -> bool {
        if let Some(work) = self.find_work_by_provider_ref(provider_ref) {
            self.mnemons.read().iter().any(|m| m.work_id == work.id)
        } else {
            false
        }
    }

    /// Add a new work and return its ID
    fn add_work(&mut self, work: Work) -> Uuid {
        let id = work.id;
        let work_clone = work.clone();
        self.works.write().push(work);

        // Persist asynchronously
        spawn(async move {
            if let Err(e) = storage::save_work(&work_clone).await {
                info!("Failed to persist work: {}", e);
            }
        });

        id
    }

    /// Add a new mnemon, returns the shuffled position of the new mnemon
    fn add_mnemon(&mut self, mnemon: Mnemon) -> usize {
        let mnemon_clone = mnemon.clone();
        let new_index = self.mnemons.read().len();
        self.mnemons.write().push(mnemon);

        // Reshuffle existing indices, then append the new mnemon's index at the end
        let mut indices = self.shuffled_indices.write();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);
        indices.push(new_index);
        let shuffled_position = indices.len() - 1;
        drop(indices);

        // Persist asynchronously
        spawn(async move {
            if let Err(e) = storage::save_mnemon(&mnemon_clone).await {
                info!("Failed to persist mnemon: {}", e);
            }
        });

        shuffled_position
    }
}

// =============================================================================
// FORM DATA
// =============================================================================

/// Form data for adding a mnemon
#[derive(Clone, PartialEq, Debug, Default)]
struct AddMnemonForm {
    // Step 1 fields
    work_type: Option<WorkType>,
    title: String,
    year: String,

    // Provider fields (if selected from search)
    provider_ref: Option<ProviderRef>,
    cover_url: Option<String>,
    theme_music_url: Option<String>,

    // Step 2 fields
    finished_date: String,
    feelings: Vec<String>,
    notes: String,
}

impl AddMnemonForm {
    fn is_step1_valid(&self) -> bool {
        self.work_type.is_some() && !self.title.trim().is_empty()
    }
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Calculate reading time in milliseconds based on word count
/// Average reading speed: ~4 words per second
/// Returns a value between NOTE_MIN_READING_TIME_MS and NOTE_MAX_READING_TIME_MS
fn calculate_reading_time(text: &str) -> u64 {
    let word_count = text.split_whitespace().count();
    let seconds = (word_count as f64 / WORDS_PER_SECOND).ceil() as u64;
    let ms = seconds * 1000;
    ms.clamp(NOTE_MIN_READING_TIME_MS, NOTE_MAX_READING_TIME_MS)
}

// =============================================================================
// MAIN APP COMPONENT
// =============================================================================

#[component]
fn App() -> Element {
    // Initialize application state
    let mut app_state = use_signal(AppState::new);

    // Provide state to child components
    use_context_provider(|| app_state);

    // Load data from IndexedDB on mount
    use_effect(move || {
        let is_loaded = app_state.peek().is_loaded();
        if !is_loaded {
            spawn(async move {
                app_state.write().load_from_storage().await;
            });
        }
    });

    // Current mnemon index for hero display
    let mut current_index = use_signal(|| 0usize);
    let mut is_transitioning = use_signal(|| false);

    // Details view state
    let mut details_open = use_signal(|| false);

    // Debug: pause auto-cycle with S key
    let mut paused = use_signal(|| false);

    // Add flow state
    let mut show_add_flow = use_signal(|| false);

    // Get mnemons with works for display (in original order, indexed by shuffled_indices)
    let mnemons_with_works = use_memo(move || app_state.read().get_mnemons_with_works());

    let current_mnemon_with_work = use_memo(move || {
        let all = mnemons_with_works();
        if all.is_empty() {
            return None;
        }
        // current_index indexes into shuffled_indices, which gives us the actual mnemon index
        let shuffled_pos = current_index();
        let state = app_state.read();
        let actual_index = state.get_shuffled_index(shuffled_pos)?;
        all.get(actual_index).cloned()
    });

    // Auto-advance to next mnemon after HERO_AUTO_CYCLE_MS (pauses when details open)
    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(HERO_AUTO_CYCLE_MS).await;

                // Skip cycling if details are open or paused
                if details_open() || paused() {
                    continue;
                }

                let total = app_state.peek().mnemons_count();
                if total == 0 {
                    continue;
                }

                // Start transition (slide out to left)
                is_transitioning.set(true);
                gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_MS).await;

                // Switch mnemon
                current_index.with_mut(|idx| *idx = (*idx + 1) % total);

                // End transition
                gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_SETTLE_MS).await;
                is_transitioning.set(false);
            }
        });
    });

    let has_mnemons = !mnemons_with_works().is_empty();
    let is_loaded = app_state.read().is_loaded();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Global key handler for debug pause
        div {
            tabindex: 0,
            autofocus: true,
            onkeydown: move |e| {
                if e.key() == Key::Character("s".to_string()) {
                    paused.toggle();
                    info!("Auto-cycle paused: {}", paused());
                }
            },

        div {
            class: "h-screen w-screen overflow-hidden bg-gray-900",

            // Show loading state until IndexedDB data is loaded
            if is_loaded && has_mnemons {
                if let Some(mnemon_with_work) = current_mnemon_with_work() {
                    Hero {
                        key: "{mnemon_with_work.mnemon.id}",
                        mnemon_with_work: mnemon_with_work.clone(),
                        is_transitioning: is_transitioning(),
                        details_open: details_open(),
                        on_add_click: move |_| {
                            show_add_flow.set(true);
                        },
                        on_details_toggle: move |_| {
                            details_open.toggle();
                        }
                    }
                }
            } else if is_loaded {
                EmptyState {
                    on_click: move |_| {
                        show_add_flow.set(true);
                    }
                }
            }

            if show_add_flow() {
                AddMnemonFlow {
                    on_save: move |form: AddMnemonForm| {
                        // Parse year
                        let year = form.year.trim().parse::<u16>().ok();

                        // First, do reads without holding a write lock
                        let existing_work_id = if let Some(ref provider_ref) = form.provider_ref {
                            app_state.peek().find_work_by_provider_ref(provider_ref).map(|w| {
                                info!("Reusing existing work: {}", w.title_en);
                                w.id
                            })
                        } else {
                            None
                        };

                        // Now do writes
                        let work_id = if let Some(id) = existing_work_id {
                            id
                        } else if let Some(ref provider_ref) = form.provider_ref {
                            // Create new work from provider
                            let work = Work::from_provider(
                                form.work_type.clone().unwrap(),
                                form.title.clone(),
                                year,
                                form.cover_url.clone(),
                                form.theme_music_url.clone(),
                                provider_ref.clone(),
                            );
                            info!("Created new work from provider: {}", work.title_en);
                            app_state.write().add_work(work)
                        } else {
                            // Create manual work
                            let work = Work::from_manual(
                                form.work_type.clone().unwrap(),
                                form.title.clone(),
                                year,
                            );
                            info!("Created manual work: {}", work.title_en);
                            app_state.write().add_work(work)
                        };

                        // Split notes by newlines
                        let notes: Vec<String> = form
                            .notes
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();

                        // Create mnemon
                        let finished_date = if form.finished_date.is_empty() {
                            None
                        } else {
                            Some(form.finished_date.clone())
                        };

                        let mnemon = Mnemon::new(work_id, finished_date, form.feelings.clone(), notes);
                        info!("Created new mnemon for work_id: {}", work_id);
                        let shuffled_position = app_state.write().add_mnemon(mnemon);

                        // Set current index to the shuffled position of the new mnemon
                        current_index.set(shuffled_position);

                        show_add_flow.set(false);
                    },
                    on_cancel: move |_| {
                        show_add_flow.set(false);
                    }
                }
            }
            }
        }
    }
}

// =============================================================================
// HERO COMPONENT
// =============================================================================

#[component]
fn Hero(
    mnemon_with_work: MnemonWithWork,
    is_transitioning: bool,
    details_open: bool,
    on_add_click: EventHandler<()>,
    on_details_toggle: EventHandler<()>,
) -> Element {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let work = mnemon_with_work.work.clone();
    let mnemon = mnemon_with_work.mnemon.clone();

    // Measured height of the title bar content
    let mut title_bar_height = use_signal(|| 150.0f64); // Default fallback

    // Current note index
    let mut current_note_index = use_signal(|| 0usize);
    let mut note_visible = use_signal(|| true);

    // Selected notes - updates when mnemon changes
    let mnemon_notes = mnemon.notes.clone();
    let selected_notes = use_memo(move || {
        let mut rng = thread_rng();
        let mut notes = mnemon_notes.clone();
        notes.shuffle(&mut rng);
        current_note_index.set(0);
        notes
            .into_iter()
            .take(HERO_NOTES_TO_DISPLAY)
            .collect::<Vec<String>>()
    });

    // Rotate through notes with fade animation (only when details closed)
    use_effect(move || {
        let notes = selected_notes();
        if notes.is_empty() || details_open {
            return;
        }

        let idx = current_note_index();
        let current_text = notes.get(idx).cloned().unwrap_or_default();
        let duration = calculate_reading_time(&current_text);

        spawn(async move {
            // Wait for reading time
            gloo_timers::future::TimeoutFuture::new(duration as u32).await;

            // Fade out
            note_visible.set(false);
            gloo_timers::future::TimeoutFuture::new(NOTE_FADE_TRANSITION_MS).await;

            // Switch to next note
            let next_idx = (idx + 1) % notes.len();
            current_note_index.set(next_idx);

            // Fade in
            note_visible.set(true);
        });
    });

    let current_note = use_memo(move || {
        let notes = selected_notes();
        let idx = current_note_index();
        notes.get(idx).cloned()
    });

    // Horizontal transition for slideshow
    let horizontal_transition = if is_transitioning {
        format!(
            "transform: translateX(-100%); transition: transform {}ms cubic-bezier(0.4, 0.0, 0.2, 1);",
            HERO_TRANSITION_MS
        )
    } else {
        format!(
            "transform: translateX(0); transition: transform {}ms cubic-bezier(0.4, 0.0, 0.2, 1);",
            HERO_TRANSITION_MS
        )
    };

    // Vertical slide for details reveal - slides up to show only title bar (using measured height)
    // Add 32px top padding so title doesn't sit flush at top of viewport
    let measured_height = title_bar_height();
    let visible_height = measured_height + 32.0;
    let vertical_slide = if details_open {
        format!("transform: translateY(calc(-100% + {}px)); transition: transform {}ms cubic-bezier(0.4, 0.0, 0.2, 1);", visible_height, DETAILS_TRANSITION_MS)
    } else {
        format!(
            "transform: translateY(0); transition: transform {}ms cubic-bezier(0.4, 0.0, 0.2, 1);",
            DETAILS_TRANSITION_MS
        )
    };

    rsx! {
        // Container for hero + details (full viewport height, with hero stacked above details)
        div {
            class: "relative h-full w-full",
            style: "{horizontal_transition}",

            // Sliding container (hero + details stacked vertically)
            div {
                class: "absolute inset-0",
                style: "{vertical_slide}",

                // Hero section (full viewport height) - original layout preserved
                div {
                    class: "relative h-screen w-full cursor-pointer",
                    onclick: move |_| on_add_click.call(()),

                    // Background cover image with overlay
                    div {
                        class: "absolute inset-0 z-0",
                        style: if let Some(ref url) = work.cover_image_local_uri {
                            format!("background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat;", url)
                        } else {
                            "background-color: #1a1a2e;".to_string()
                        },

                        // Dark overlay for readability
                        div {
                            class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/70 to-black/50"
                        }
                    }

                    // Note display - lower left (only visible when details closed)
                    if !details_open {
                        if let Some(note) = current_note() {
                            div {
                                class: "absolute left-4 top-2/3 z-10 max-w-lg max-w-80 transition-opacity duration-500",
                                style: if note_visible() { "opacity: 1;" } else { "opacity: 0;" },

                                p {
                                    class: "text-white/90 text-lg leading-relaxed font-light italic",
                                    "\"{note}\""
                                }
                            }
                        }
                    }

                    // Content overlay - footnote style at bottom right (original layout)
                    div {
                        class: "absolute inset-0 z-10 px-8 pb-8 flex items-end justify-end pointer-events-none",

                        div {
                            class: "max-w-md",
                            onmounted: move |evt| {
                                let data = evt.data();
                                spawn(async move {
                                    if let Ok(rect) = data.get_client_rect().await {
                                        // Add padding (pb-8 = 32px) to the measured content height
                                        let height = rect.height() + 32.0;
                                        title_bar_height.set(height);
                                    }
                                });
                            },

                            // Icon and Title
                            div {
                                class: "flex items-center gap-3 mb-3",
                                span {
                                    class: "text-2xl opacity-90",
                                    "{work.work_type.icon()}"
                                }
                                h1 {
                                    class: "text-2xl font-semibold text-white/95",
                                    "{work.title_en}"
                                }
                            }

                            // Feelings
                            if !mnemon.feelings.is_empty() {
                                div {
                                    class: "flex flex-wrap gap-2",
                                    for feeling in mnemon.feelings.iter() {
                                        span {
                                            class: "px-3 py-1 bg-white/15 backdrop-blur-sm text-white/90 text-sm rounded-full border border-white/20",
                                            "{feeling}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Bottom click zone - toggles details (title bar area, uses measured height + top padding)
                    div {
                        class: "absolute bottom-0 left-0 right-0 z-30 cursor-pointer",
                        style: "height: {visible_height}px;",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_details_toggle.call(());
                        },
                    }
                }

                // Details section (below hero, same height as viewport minus visible title area)
                div {
                    class: "relative w-full bg-gray-900",
                    style: "height: calc(100vh - {visible_height}px);",

                    MemoryDetails {
                        mnemon_with_work: mnemon_with_work.clone(),
                    }
                }
            }
        }
    }
}

// =============================================================================
// MEMORY DETAILS COMPONENT
// =============================================================================

#[component]
fn MemoryDetails(mnemon_with_work: MnemonWithWork) -> Element {
    let work = &mnemon_with_work.work;
    let mnemon = &mnemon_with_work.mnemon;

    // Stubbed audio state
    let mut is_playing = use_signal(|| false);

    rsx! {
        div {
            class: "h-full overflow-y-auto px-8 py-6",

            // Audio player stub (only if theme music exists)
            if work.theme_music_local_uri.is_some() {
                div {
                    class: "mb-6 flex items-center gap-4",

                    button {
                        class: "w-14 h-14 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center transition-colors",
                        onclick: move |_| is_playing.toggle(),

                        span {
                            class: "text-2xl text-white",
                            if is_playing() { "⏸" } else { "▶" }
                        }
                    }

                    div {
                        class: "flex-1",

                        // Progress bar stub
                        div {
                            class: "h-1 bg-white/20 rounded-full overflow-hidden",
                            div {
                                class: "h-full bg-white/60 rounded-full",
                                style: "width: 0%;",
                            }
                        }

                        p {
                            class: "text-sm text-white/40 mt-1",
                            "Theme music"
                        }
                    }
                }
            }

            // Feelings chips
            if !mnemon.feelings.is_empty() {
                div {
                    class: "mb-6",

                    h3 {
                        class: "text-sm text-white/50 uppercase tracking-wide mb-3",
                        "Feelings"
                    }

                    div {
                        class: "flex flex-wrap gap-2",
                        for feeling in mnemon.feelings.iter() {
                            span {
                                class: "px-4 py-2 bg-white/10 text-white/90 rounded-full border border-white/20",
                                "{feeling}"
                            }
                        }
                    }
                }
            }

            // Finished date
            if let Some(ref finished_date) = mnemon.finished_date {
                div {
                    class: "mb-6",

                    h3 {
                        class: "text-sm text-white/50 uppercase tracking-wide mb-2",
                        "Finished"
                    }

                    p {
                        class: "text-white/80",
                        "{finished_date}"
                    }
                }
            }

            // Notes
            if !mnemon.notes.is_empty() {
                div {
                    class: "mb-6",

                    h3 {
                        class: "text-sm text-white/50 uppercase tracking-wide mb-3",
                        "Notes"
                    }

                    div {
                        class: "space-y-3",
                        for note in mnemon.notes.iter() {
                            p {
                                class: "text-white/80 leading-relaxed",
                                "{note}"
                            }
                        }
                    }
                }
            }

            // Empty state if no details
            if mnemon.feelings.is_empty() && mnemon.finished_date.is_none() && mnemon.notes.is_empty() && work.theme_music_local_uri.is_none() {
                div {
                    class: "flex items-center justify-center h-full text-white/40 italic",
                    "No additional details for this memory"
                }
            }
        }
    }
}

// =============================================================================
// EMPTY STATE COMPONENT
// =============================================================================

#[component]
fn EmptyState(on_click: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "h-full w-full flex items-center justify-center cursor-pointer",
            onclick: move |_| on_click.call(()),

            div {
                class: "text-center px-8 max-w-md",

                h1 {
                    class: "text-4xl font-semibold mb-4 text-white",
                    "Add your first mnemon"
                }

                p {
                    class: "text-lg opacity-70 mb-8 text-white",
                    "Capture a great movie, TV/anime, or game you loved. Nostalgia awaits."
                }

                p {
                    class: "text-sm opacity-50 italic text-white",
                    "Tap anywhere to begin"
                }
            }
        }
    }
}

// =============================================================================
// ADD MNEMON FLOW
// =============================================================================

#[component]
fn AddMnemonFlow(on_save: EventHandler<AddMnemonForm>, on_cancel: EventHandler<()>) -> Element {
    let mut form = use_signal(AddMnemonForm::default);
    let mut current_step = use_signal(|| 1);

    rsx! {
        // Modal overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm",
            onclick: move |_| on_cancel.call(()),

            // Modal content
            div {
                class: "bg-gray-800 rounded-lg shadow-2xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),

                if current_step() == 1 {
                    Step1ManualEntry {
                        form: form(),
                        on_next: move |updated_form| {
                            form.set(updated_form);
                            current_step.set(2);
                        },
                        on_cancel: move |_| on_cancel.call(())
                    }
                } else {
                    Step2Personalize {
                        form: form(),
                        on_save: move |updated_form| {
                            on_save.call(updated_form);
                        },
                        on_back: move |updated_form| {
                            form.set(updated_form);
                            current_step.set(1);
                        }
                    }
                }
            }
        }
    }
}

// =============================================================================
// STEP 1: PICK THE WORK
// =============================================================================

#[component]
fn Step1ManualEntry(
    form: AddMnemonForm,
    on_next: EventHandler<AddMnemonForm>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut local_form = use_signal(|| form);
    let mut search_results = use_signal(Vec::<SearchResult>::new);
    let mut show_results = use_signal(|| false);
    let mut existing_work_error = use_signal(|| false);
    let mut is_searching = use_signal(|| false);
    let mut search_status = use_signal(|| Option::<SearchStatus>::None);

    // Search trigger signals - when these change, a search is triggered
    let mut search_query = use_signal(String::new);
    let mut search_type = use_signal(|| Option::<WorkType>::None);
    let mut search_force = use_signal(|| false);
    let mut search_version = use_signal(|| 0u32);

    let app_state = use_context::<Signal<AppState>>();

    // Create search service once
    let search_service = use_hook(SearchService::new);

    let is_valid = local_form().is_step1_valid() && !existing_work_error();
    let is_tmdb_configured = search_service.is_tmdb_configured();
    let is_rawg_configured = search_service.is_rawg_configured();

    // Check if provider ref already exists
    let check_existing_work = move |provider_ref: &ProviderRef| -> bool {
        app_state().has_mnemon_for_provider_ref(provider_ref)
    };

    // Effect-based async search with debouncing
    {
        let service = search_service.clone();
        use_effect(move || {
            let query = search_query();
            let work_type = search_type();
            let force = search_force();
            let version = search_version();

            // Reset force flag
            if force {
                search_force.set(false);
            }

            // Don't search if no work type selected
            let Some(wt) = work_type else {
                return;
            };

            // Don't search if query is too short (unless forced or empty)
            if !force && !query.is_empty() && query.len() < SEARCH_MIN_CHARS {
                search_results.set(Vec::new());
                show_results.set(false);
                return;
            }

            is_searching.set(true);
            search_status.set(None);

            let service = service.clone();
            spawn(async move {
                // Debounce delay (skip if forced search)
                if !force {
                    gloo_timers::future::TimeoutFuture::new(SEARCH_DEBOUNCE_MS).await;
                }

                // Check if this search is still valid
                if search_version() != version {
                    info!("Search cancelled (superseded)");
                    return;
                }

                info!("Executing search for '{}' ({:?})", query, wt);
                let response = service.search(&query, wt, 0).await;

                // Check again after async operation
                if search_version() != version {
                    info!("Search results discarded (superseded)");
                    return;
                }

                is_searching.set(false);
                search_status.set(Some(response.status.clone()));

                match response.status {
                    SearchStatus::Success | SearchStatus::UsingFixtures => {
                        info!("Search returned {} results", response.results.len());
                        search_results.set(response.results);
                        show_results.set(true);
                    }
                    SearchStatus::ProviderNotConfigured => {
                        info!("Provider not configured");
                        search_results.set(Vec::new());
                        show_results.set(true);
                    }
                    SearchStatus::NetworkError(ref msg) => {
                        info!("Network error: {}", msg);
                        search_results.set(Vec::new());
                        show_results.set(true);
                    }
                }
            });
        });
    }

    rsx! {
        div {
            class: "p-8",

            // Header
            div {
                class: "mb-6",
                h2 {
                    class: "text-3xl font-bold text-white mb-2",
                    "Add a mnemon"
                }
                p {
                    class: "text-gray-400",
                    "Step 1: Pick the Work"
                }
            }

            // Provider status indicator (when not configured)
            if !is_tmdb_configured {
                div {
                    class: "mb-4 px-4 py-3 bg-yellow-900/30 border border-yellow-700/50 rounded-lg",
                    div {
                        class: "flex items-center gap-2 text-yellow-200",
                        span { "⚠️" }
                        span { class: "font-medium", "TMDB API not configured" }
                    }
                    p {
                        class: "text-yellow-200/70 text-sm mt-1",
                        "Set TMDB_ACCESS_TOKEN environment variable to enable movie/TV search. You can still add entries manually."
                    }
                }
            }

            if !is_rawg_configured {
                div {
                    class: "mb-4 px-4 py-3 bg-yellow-900/30 border border-yellow-700/50 rounded-lg",
                    div {
                        class: "flex items-center gap-2 text-yellow-200",
                        span { "⚠️" }
                        span { class: "font-medium", "RAWG API not configured" }
                    }
                    p {
                        class: "text-yellow-200/70 text-sm mt-1",
                        "Set RAWG_API_KEY environment variable to enable game search. You can still add entries manually."
                    }
                }
            }

            // Type selection
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-3",
                    "Type"
                    span { class: "text-red-400 ml-1", "*" }
                }
                div {
                    class: "flex gap-3",
                    for work_type in [WorkType::Movie, WorkType::TvAnime, WorkType::Game] {
                        button {
                            class: if local_form().work_type == Some(work_type.clone()) {
                                "flex items-center gap-2 px-4 py-3 bg-blue-600 text-white rounded-lg border-2 border-blue-500 font-medium"
                            } else {
                                "flex items-center gap-2 px-4 py-3 bg-gray-700 text-gray-300 rounded-lg border-2 border-gray-600 hover:border-gray-500 font-medium"
                            },
                            onclick: move |_| {
                                info!("Type selected: {}", work_type.label());
                                local_form.with_mut(|f| {
                                    f.work_type = Some(work_type.clone());
                                    // Clear provider data when changing type
                                    f.provider_ref = None;
                                    f.cover_url = None;
                                    f.theme_music_url = None;
                                });
                                existing_work_error.set(false);
                                search_results.set(Vec::new());
                                show_results.set(false);
                                search_status.set(None);
                                search_type.set(Some(work_type.clone()));
                            },
                            span { class: "text-xl", "{work_type.icon()}" }
                            span { "{work_type.label()}" }
                        }
                    }
                }
            }

            // Title input with search
            div {
                class: "mb-6 relative",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Title (English)"
                    span { class: "text-red-400 ml-1", "*" }
                }
                div {
                    class: "relative",
                    input {
                        class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none pr-10",
                        r#type: "text",
                        placeholder: if local_form().work_type.is_some() {
                            "Search or enter title..."
                        } else {
                            "Select a type first..."
                        },
                        value: "{local_form().title}",
                        disabled: local_form().work_type.is_none(),
                        onfocus: move |_| {
                            info!("Title field focused");
                            if local_form().work_type.is_some() {
                                search_version.set(search_version() + 1);
                                search_query.set(local_form().title.clone());
                                search_type.set(local_form().work_type.clone());
                            }
                        },
                        onblur: move |_| {
                            // Delay hiding to allow click on results
                            let current_version = search_version();
                            spawn(async move {
                                gloo_timers::future::TimeoutFuture::new(150).await;
                                // Only hide if no new search was triggered
                                if search_version() == current_version {
                                    show_results.set(false);
                                }
                            });
                        },
                        oninput: move |e| {
                            let value = e.value();
                            local_form.with_mut(|f| {
                                f.title = value.clone();
                                // Clear provider data when typing
                                f.provider_ref = None;
                                f.cover_url = None;
                                f.theme_music_url = None;
                                f.year = String::new();
                            });
                            existing_work_error.set(false);
                            // Trigger debounced search
                            search_version.set(search_version() + 1);
                            search_query.set(value);
                            search_type.set(local_form().work_type.clone());
                        },
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                e.prevent_default();
                                // Force search on Enter regardless of query length
                                if local_form().work_type.is_some() {
                                    search_version.set(search_version() + 1);
                                    search_query.set(local_form().title.clone());
                                    search_type.set(local_form().work_type.clone());
                                    search_force.set(true);
                                }
                            }
                        }
                    }

                    // Loading spinner
                    if is_searching() {
                        div {
                            class: "absolute right-3 top-1/2 -translate-y-1/2",
                            div {
                                class: "w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"
                            }
                        }
                    }
                }

                // Search results dropdown
                if show_results() {
                    div {
                        class: "absolute z-10 w-full mt-1 bg-gray-700 border-2 border-gray-600 rounded-lg shadow-lg max-h-64 overflow-y-auto",

                        // Status messages
                        match search_status() {
                            Some(SearchStatus::ProviderNotConfigured) => rsx! {
                                div {
                                    class: "px-4 py-3 text-gray-400 text-sm",
                                    "Provider not configured. Enter title manually below."
                                }
                            },
                            Some(SearchStatus::NetworkError(msg)) => rsx! {
                                div {
                                    class: "px-4 py-3 text-yellow-400 text-sm",
                                    "Network error: {msg}. You can enter the title manually."
                                }
                            },
                            _ if search_results().is_empty() && !is_searching() && !local_form().title.is_empty() => rsx! {
                                div {
                                    class: "px-4 py-3 text-gray-400 text-sm",
                                    "No results found for \"{local_form().title}\". You can enter it manually."
                                }
                            },
                            _ if search_results().is_empty() && !is_searching() => rsx! {
                                div {
                                    class: "px-4 py-3 text-gray-400 text-sm",
                                    "Type to search or enter title manually."
                                }
                            },
                            _ => rsx! {}
                        }

                        // Results list
                        for result in search_results().iter() {
                            button {
                                class: "w-full px-4 py-3 flex items-center gap-3 hover:bg-gray-600 border-b border-gray-600 last:border-b-0 text-left",
                                onmousedown: {
                                    let result_clone = result.clone();
                                    move |e: MouseEvent| {
                                        e.prevent_default();
                                        info!("Result selected: {}", result_clone.title);

                                        if check_existing_work(&result_clone.provider_ref) {
                                            info!("Work already has a mnemon - showing error");
                                            existing_work_error.set(true);
                                            show_results.set(false);
                                        } else {
                                            info!("Autofilling form with result");
                                            local_form.with_mut(|f| {
                                                f.title = result_clone.title.clone();
                                                f.year = result_clone.year.map(|y| y.to_string()).unwrap_or_default();
                                                f.provider_ref = Some(result_clone.provider_ref.clone());
                                                f.cover_url = result_clone.cover_url.clone();
                                                f.theme_music_url = result_clone.theme_music_url.clone();
                                            });
                                            existing_work_error.set(false);
                                            show_results.set(false);
                                        }
                                    }
                                },

                                // Cover thumbnail
                                if let Some(ref cover) = result.cover_url {
                                    img {
                                        class: "w-12 h-16 object-cover rounded",
                                        src: "{cover}",
                                        alt: "{result.title}"
                                    }
                                } else {
                                    div {
                                        class: "w-12 h-16 bg-gray-800 rounded flex items-center justify-center",
                                        span { class: "text-2xl", "{result.work_type.icon()}" }
                                    }
                                }

                                div {
                                    class: "flex-1",
                                    div {
                                        class: "text-white font-medium",
                                        "{result.title}"
                                    }
                                    if let Some(year) = result.year {
                                        div {
                                            class: "text-gray-400 text-sm",
                                            "{year}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Existing work error message
                if existing_work_error() {
                    div {
                        class: "mt-2 px-3 py-2 bg-red-900/50 border border-red-700 rounded text-red-200 text-sm",
                        "⚠️ This work already exists in your collection. Please search for a different title."
                    }
                }

                // Manual entry hint when provider selected result
                if local_form().provider_ref.is_some() {
                    div {
                        class: "mt-2 px-3 py-2 bg-green-900/30 border border-green-700/50 rounded text-green-200 text-sm flex items-center gap-2",
                        span { "✓" }
                        span { "Selected from search results" }
                    }
                }
            }

            // Notes
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Notes"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none min-h-[120px] resize-y",
                    placeholder: "Add your thoughts, memories, or reflections...",
                    value: "{local_form().notes}",
                    oninput: move |e| {
                        local_form.with_mut(|f| f.notes = e.value());
                    }
                }
            }

            // Feelings
            div {
                class: "mb-8",
                label {
                    class: "block text-white text-sm font-semibold mb-3",
                    "Feelings"
                    span { class: "text-gray-400 ml-2 text-xs", "(choose up to {MAX_FEELINGS})" }
                }
                div {
                    class: "flex flex-wrap gap-2",
                    for (feeling_name, feeling_emoji) in FEELINGS {
                        {
                            let is_selected = local_form().feelings.contains(&feeling_name.to_string());
                            let feelings_count = local_form().feelings.len();
                            let can_add = feelings_count < MAX_FEELINGS;

                            rsx! {
                                button {
                                    class: if is_selected {
                                        "px-4 py-2 bg-blue-600 text-white rounded-full border-2 border-blue-500 text-sm font-medium"
                                    } else if can_add {
                                        "px-4 py-2 bg-gray-700 text-gray-300 rounded-full border-2 border-gray-600 hover:border-gray-500 text-sm font-medium"
                                    } else {
                                        "px-4 py-2 bg-gray-800 text-gray-500 rounded-full border-2 border-gray-700 text-sm font-medium cursor-not-allowed"
                                    },
                                    disabled: !is_selected && !can_add,
                                    onclick: move |_| {
                                        local_form.with_mut(|f| {
                                            if is_selected {
                                                f.feelings.retain(|s| s != feeling_name);
                                            } else if can_add {
                                                f.feelings.push(feeling_name.to_string());
                                            }
                                        });
                                    },
                                    span { class: "mr-1", "{feeling_emoji}" }
                                    span { "{feeling_name}" }
                                }
                            }
                        }
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-3 justify-end",
                button {
                    class: "px-6 py-3 text-gray-300 hover:text-white font-medium",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
                button {
                    class: if is_valid {
                        "px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium"
                    } else {
                        "px-6 py-3 bg-gray-600 text-gray-400 rounded-lg font-medium cursor-not-allowed"
                    },
                    disabled: !is_valid,
                    onclick: move |_| {
                        if is_valid {
                            on_next.call(local_form());
                        }
                    },
                    "Next →"
                }
            }
        }
    }
}

// =============================================================================
// STEP 2: PERSONALIZE
// =============================================================================

#[component]
fn Step2Personalize(
    form: AddMnemonForm,
    on_save: EventHandler<AddMnemonForm>,
    on_back: EventHandler<AddMnemonForm>,
) -> Element {
    let mut local_form = use_signal(|| form);

    rsx! {
        div {
            class: "p-8",

            // Header
            div {
                class: "mb-6",
                h2 {
                    class: "text-3xl font-bold text-white mb-2",
                    "Add a mnemon"
                }
                p {
                    class: "text-gray-400",
                    "Step 2: Optional dates"
                }
            }

            // Release Year (autofilled from search or manual)
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Release Year"
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "text",
                    placeholder: "YYYY",
                    value: "{local_form().year}",
                    readonly: local_form().provider_ref.is_some(),
                    maxlength: 4,
                    oninput: move |e| {
                        let value = e.value();
                        // Only allow digits
                        if value.chars().all(|c| c.is_ascii_digit()) {
                            local_form.with_mut(|f| f.year = value);
                        }
                    }
                }
                if local_form().provider_ref.is_some() {
                    p {
                        class: "text-gray-500 text-xs mt-1",
                        "Autofilled from search result"
                    }
                }
            }

            // Finished date
            div {
                class: "mb-8",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Finished date"
                    span { class: "text-gray-400 ml-1 text-xs", "(when you completed it)" }
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "date",
                    value: "{local_form().finished_date}",
                    oninput: move |e| {
                        local_form.with_mut(|f| f.finished_date = e.value());
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-3 justify-between",
                button {
                    class: "px-6 py-3 text-gray-300 hover:text-white font-medium",
                    onclick: move |_| on_back.call(local_form()),
                    "← Back"
                }
                button {
                    class: "px-6 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg font-medium",
                    onclick: move |_| {
                        on_save.call(local_form());
                    },
                    "Save Mnemon"
                }
            }
        }
    }
}
