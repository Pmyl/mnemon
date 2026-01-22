//! Add mnemon flow - multi-step wizard for creating a new mnemon

use dioxus::prelude::*;
use tracing::info;

use crate::app_state::AppState;
use crate::components::{FeelingsSelector, FinishedDateInput, NotesInput};
use crate::constants::*;
use crate::data::{SearchService, SearchStatus};
use crate::forms::MnemonForm;
use crate::models::*;
use crate::settings;

#[component]
pub fn AddMnemonFlow(
    on_save: EventHandler<MnemonForm>,
    on_cancel: EventHandler<()>,
    on_settings: EventHandler<()>,
) -> Element {
    let mut form = use_signal(MnemonForm::default);
    let mut current_step = use_signal(|| 1);

    rsx! {
        // Modal overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm",
            onclick: move |_| on_cancel.call(()),

            // Modal content
            div {
                class: "bg-gray-800 rounded-lg shadow-2xl max-w-full sm:max-w-2xl w-full mx-2 sm:mx-4 max-h-[95vh] sm:max-h-[90vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),

                if current_step() == 1 {
                    Step1ManualEntry {
                        form: form(),
                        on_next: move |updated_form| {
                            form.set(updated_form);
                            current_step.set(2);
                        },
                        on_cancel: move |_| on_cancel.call(()),
                        on_settings: move |_| on_settings.call(()),
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
    form: MnemonForm,
    on_next: EventHandler<MnemonForm>,
    on_cancel: EventHandler<()>,
    on_settings: EventHandler<()>,
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

    // Check if APIs are configured (from localStorage or compile-time env)
    let is_tmdb_configured = settings::is_tmdb_configured();
    let is_rawg_configured = settings::is_rawg_configured();

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
                    SearchStatus::ApiError {
                        status,
                        ref message,
                    } => {
                        info!("API error ({}): {}", status, message);
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

            // Header with settings button
            div {
                class: "mb-6 flex items-start justify-between",
                div {
                    h2 {
                        class: "text-3xl font-bold text-white mb-2",
                        "Add a mnemon"
                    }
                    p {
                        class: "text-gray-400",
                        "Step 1: Pick the Work"
                    }
                }
                // Settings button (gear icon)
                button {
                    class: "p-2 rounded-full bg-white/10 hover:bg-white/20 transition-colors text-white/70 hover:text-white",
                    onclick: move |_| on_settings.call(()),
                    title: "Settings",
                    svg {
                        class: "w-5 h-5",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.204-.107-.397.165-.71.505-.78.929l-.15.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z"
                        }
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                        }
                    }
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
                        "Click the ⚙️ Settings button above to add your TMDB token for movie/TV search. You can still add entries manually."
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
                        "Click the ⚙️ Settings button above to add your RAWG API key for game search. You can still add entries manually."
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
                                "flex items-center gap-2 px-4 py-3 bg-transparent text-white rounded-lg border-2 border-white font-medium"
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
                        class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-white focus:outline-none pr-10",
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
                                // Only hide if no new search was triggered and there's no error/status to show
                                if search_version() == current_version {
                                    // Don't hide if there's an error or status message to display
                                    let should_keep_visible = match search_status() {
                                        Some(SearchStatus::NetworkError(_)) => true,
                                        Some(SearchStatus::ApiError { .. }) => true,
                                        Some(SearchStatus::ProviderNotConfigured) => true,
                                        _ => false,
                                    };

                                    if !should_keep_visible {
                                        show_results.set(false);
                                    }
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
                                class: "w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin"
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
                            Some(SearchStatus::ApiError { status, .. }) => rsx! {
                                div {
                                    class: "px-4 py-3 text-yellow-400 text-sm",
                                    if status == 401 {
                                        "Invalid API key. Please check your API key in Settings (⚙️)."
                                    } else if status == 429 {
                                        "Rate limit exceeded. Please try again later."
                                    } else {
                                        "API error ({status}). Please check your API key in Settings (⚙️)."
                                    }
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
            NotesInput { form: local_form }

            // Feelings
            FeelingsSelector { form: local_form }

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
                        "px-6 py-3 bg-transparent border-2 border-white hover:bg-white/10 text-white rounded-lg font-medium flex items-center gap-2"
                    } else {
                        "px-6 py-3 bg-gray-700 text-gray-500 rounded-lg font-medium cursor-not-allowed flex items-center gap-2"
                    },
                    disabled: !is_valid,
                    onclick: move |_| {
                        if is_valid {
                            on_next.call(local_form());
                        }
                    },
                    span { "Next" }
                    span { "→" }
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
    form: MnemonForm,
    on_save: EventHandler<MnemonForm>,
    on_back: EventHandler<MnemonForm>,
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
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-white focus:outline-none",
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
            FinishedDateInput { form: local_form }

            // Actions
            div {
                class: "flex gap-3 justify-between",
                button {
                    class: "px-6 py-3 text-gray-300 hover:text-white font-medium flex items-center gap-2",
                    onclick: move |_| on_back.call(local_form()),
                    span { "←" }
                    span { "Back" }
                }
                button {
                    class: "px-6 py-3 bg-transparent border-2 border-white hover:bg-white/10 text-white rounded-lg font-medium",
                    onclick: move |_| {
                        on_save.call(local_form());
                    },
                    "Save Mnemon"
                }
            }
        }
    }
}
