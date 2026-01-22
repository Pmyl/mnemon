//! Mnemon - A nostalgia-focused app to capture and resurface memories from Movies, TV/Anime, and Games

use dioxus::prelude::*;
use tracing::info;
use uuid::Uuid;

mod app_state;
mod components;
mod constants;
mod data;
mod forms;
mod models;
mod providers;
mod settings;
mod storage;
mod utils;

use app_state::AppState;
use components::*;
use constants::*;
use forms::MnemonForm;
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
    let mut is_animating = use_signal(|| false);
    let mut user_interacting = use_signal(|| false);

    // Details view state
    let mut details_open = use_signal(|| false);

    // Auto-cycle pause state (controlled via Settings)
    let paused = use_signal(|| false);

    // Add flow state
    let mut show_add_flow = use_signal(|| false);

    // Edit flow state
    let mut edit_mnemon_id = use_signal(|| Option::<Uuid>::None);

    // Settings modal state
    let mut show_settings = use_signal(|| false);

    // Pending delete state for undo functionality
    let mut pending_delete: Signal<Option<PendingDelete>> = use_signal(|| None);

    // Get mnemons with works for display (in original order, indexed by shuffled_indices)
    let mnemons_with_works = use_memo(move || app_state.read().get_mnemons_with_works());

    // Carousel indices: prev, current, next
    let carousel_indices = use_memo(move || {
        let state = app_state.read();
        let total = state.mnemons_count();
        if total == 0 {
            return (0, 0, 0);
        }

        let current = current_index();
        let prev = if current == 0 { total - 1 } else { current - 1 };
        let next = (current + 1) % total;
        (prev, current, next)
    });

    // Get three mnemons for carousel
    let carousel_mnemons = use_memo(move || {
        let all = mnemons_with_works();
        if all.is_empty() {
            return (None, None, None);
        }

        let (prev_idx, curr_idx, next_idx) = carousel_indices();
        let state = app_state.read();

        let get_mnemon = |idx: usize| {
            state
                .get_shuffled_index(idx)
                .and_then(|i| all.get(i))
                .cloned()
        };

        (
            get_mnemon(prev_idx),
            get_mnemon(curr_idx),
            get_mnemon(next_idx),
        )
    });

    // Navigation functions
    let mut navigate_next = move |_| {
        if is_animating() {
            return;
        }

        is_animating.set(true);
        let total = app_state.read().mnemons_count();

        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_MS).await;

            current_index.with_mut(|idx| {
                *idx = (*idx + 1) % total;
            });

            gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_SETTLE_MS).await;
            is_animating.set(false);
        });
    };

    let navigate_prev = move |_| {
        if is_animating() {
            return;
        }

        is_animating.set(true);
        let total = app_state.read().mnemons_count();

        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_MS).await;

            current_index.with_mut(|idx| {
                *idx = if *idx == 0 { total - 1 } else { *idx - 1 };
            });

            gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_SETTLE_MS).await;
            is_animating.set(false);
        });
    };

    // Auto-advance to next mnemon after HERO_AUTO_CYCLE_MS (pauses when details open, paused, or user interacting)
    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(HERO_AUTO_CYCLE_MS).await;

                // Skip cycling if details open, paused, or user is interacting
                if details_open() || paused() || user_interacting() {
                    continue;
                }

                let total = app_state.peek().mnemons_count();
                if total == 0 {
                    continue;
                }

                // Trigger navigation if not already animating
                if !is_animating() {
                    navigate_next(());
                }
            }
        });
    });

    let has_mnemons = !mnemons_with_works().is_empty();
    let is_loaded = app_state.peek().is_loaded();

    rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }

            // Global key handler for debug pause
            div {

            div {
                class: "h-screen w-screen overflow-hidden bg-gray-900",

                // Show loading state until IndexedDB data is loaded
                if is_loaded && has_mnemons {
    {
                        let (prev, curr, next) = carousel_mnemons();
                        if let Some(current_mnemon) = curr {
                            rsx! {
                                Carousel {
                                    key: "{current_mnemon.mnemon.id}",
                                    prev_mnemon: prev,
                                    current_mnemon: current_mnemon,
                                    next_mnemon: next,
                                    details_open: details_open(),
                                    is_animating: is_animating(),
                                    on_navigate_next: navigate_next,
                                    on_navigate_prev: navigate_prev,
                                    on_user_interaction_start: move |_| user_interacting.set(true),
                                    on_user_interaction_end: move |_| user_interacting.set(false),
                                    on_add_click: move |_| {
                                        show_add_flow.set(true);
                                    },
                                    on_details_toggle: move |_| {
                                        details_open.toggle();
                                    },
                                    on_edit: move |mnemon_id: Uuid| {
                                        edit_mnemon_id.set(Some(mnemon_id));
                                    },
                                    on_delete: move |mnemon_id: Uuid| {
                                        let removed = app_state.write().remove_mnemon(mnemon_id);
                                        if let Some((mnemon, original_idx)) = removed {
                                            pending_delete.set(Some(PendingDelete { mnemon, original_idx }));
                                            details_open.set(false);
                                            let total = app_state.peek().mnemons_count();
                                            if total > 0 && current_index() >= total {
                                                current_index.set(total - 1);
                                            }
                                        }
                                    },
                                }
                            }
                        } else {
                            rsx! {}
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
                        on_settings: move |_| {
                            show_settings.set(true);
                        },
                        on_save: move |form: MnemonForm| {
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

                            // Parse form data using helper methods
                            let notes = form.parse_notes();
                            let finished_date = form.parse_finished_date();

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

                // Edit flow
                if let Some(editing_id) = edit_mnemon_id() {
                    {
                        // Find the mnemon with work to pre-fill the form
                        let all_mnemons_with_works = mnemons_with_works();
                        if let Some(mnemon_with_work) = all_mnemons_with_works.iter().find(|mw| mw.mnemon.id == editing_id) {
                            let initial_form = MnemonForm::from_mnemon_for_edit(&mnemon_with_work.mnemon, &mnemon_with_work.work);
                            rsx! {
                                EditMnemonFlow {
                                    initial_form: initial_form,
                                    on_save: move |form: MnemonForm| {
                                        // Parse form data using helper methods
                                        let notes = form.parse_notes();
                                        let finished_date = form.parse_finished_date();

                                        // Update the mnemon
                                        app_state.write().edit_mnemon(
                                            editing_id,
                                            finished_date,
                                            form.feelings.clone(),
                                            notes,
                                        );

                                        info!("Updated mnemon: {}", editing_id);
                                        edit_mnemon_id.set(None);
                                    },
                                    on_cancel: move |_| {
                                        edit_mnemon_id.set(None);
                                    }
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                }

                // Undo toast for pending deletions
                if let Some(_pending) = pending_delete() {
                    UndoToast {
                        message: "Memory deleted".to_string(),
                        on_undo: move |_| {
                            if let Some(pending) = pending_delete.take() {
                                app_state.write().restore_mnemon(pending.mnemon, pending.original_idx);
                                // Set index to 0 to show restored mnemon
                                current_index.set(0);
                            }
                        },
                        on_timeout: move |_| {
                            if let Some(pending) = pending_delete.take() {
                                // Permanently delete from storage
                                AppState::delete_mnemon_from_storage(pending.mnemon.id);
                            }
                        }
                    }
                }

                // Settings modal
                if show_settings() {
                    SettingsModal {
                        paused: paused,
                        on_close: move |_| {
                            show_settings.set(false);
                        }
                    }
                }
            }
            }
        }
}
