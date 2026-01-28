//! Mnemon - A nostalgia-focused app to capture and resurface memories from Movies, TV/Anime, and Games

use dioxus::prelude::*;
use tracing::info;
use uuid::Uuid;

mod app_state;
mod components;
mod constants;
mod data;
mod forms;
mod hooks;
mod models;
mod providers;
mod settings;
mod storage;
mod types;
mod utils;

use app_state::AppState;
use components::*;
use constants::*;
use forms::MnemonForm;
use models::*;
use types::Direction;

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
    let mut is_transitioning = use_signal(|| false);
    let mut next_index = use_signal(|| Option::<usize>::None);
    let mut transition_direction = use_signal(|| Direction::Forward);

    // Details view state
    let mut details_open = use_signal(|| false);

    // Auto-cycle pause state (controlled via Settings)
    let paused = use_signal(|| false);

    // Pause auto-cycle temporarily after manual navigation
    let mut pause_auto_cycle = use_signal(|| false);

    // Cycle version - increments to restart auto-cycle timer
    let mut cycle_version = use_signal(|| 0u32);

    // Detect if device is mobile for touch gestures
    let is_mobile = use_signal(utils::is_mobile_device);

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

    // Helper to get mnemon by shuffled index
    let get_mnemon_by_index = move |shuffled_pos: usize| -> Option<app_state::MnemonWithWork> {
        let all = mnemons_with_works();
        if all.is_empty() {
            return None;
        }
        let state = app_state.read();
        let actual_index = state.get_shuffled_index(shuffled_pos)?;
        all.get(actual_index).cloned()
    };

    let current_mnemon_with_work = use_memo(move || get_mnemon_by_index(current_index()));

    let next_mnemon_with_work = use_memo(move || next_index().and_then(get_mnemon_by_index));

    // Manual navigation function
    let mut navigate = move |direction: Direction| {
        if is_transitioning() {
            return; // Debounce - prevent navigation during transition
        }

        let total = app_state.peek().mnemons_count();
        if total == 0 {
            return;
        }

        let next_idx = match direction {
            Direction::Forward => (current_index() + 1) % total,
            Direction::Backward => {
                let current = current_index();
                if current == 0 {
                    total - 1
                } else {
                    current - 1
                }
            }
        };

        transition_direction.set(direction);
        next_index.set(Some(next_idx));
        is_transitioning.set(true);

        // Restart auto-cycle timer (use wrapping to handle overflow)
        let new_version = cycle_version.peek().wrapping_add(1);
        cycle_version.set(new_version);

        // Pause auto-cycle for 5 seconds after manual navigation
        pause_auto_cycle.set(true);

        spawn(async move {
            // Wait for transition animation
            gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_MS).await;

            // Switch to next mnemon
            current_index.set(next_idx);
            next_index.set(None);

            // End transition
            gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_SETTLE_MS).await;
            is_transitioning.set(false);

            // Resume auto-cycle after configured pause duration
            gloo_timers::future::TimeoutFuture::new(MANUAL_NAV_PAUSE_MS).await;
            pause_auto_cycle.set(false);
        });
    };

    // Track previous details state to detect transitions
    let mut prev_details_open = use_signal(|| false);

    // Restart timer when details close (transition from open to closed)
    use_effect(move || {
        let current = details_open();

        // Use with_mut to avoid reactive read causing infinite loop
        prev_details_open.with_mut(|previous| {
            // Only increment when transitioning from open to closed
            if *previous && !current {
                let new_version = cycle_version.peek().wrapping_add(1);
                cycle_version.set(new_version);
            }
            *previous = current;
        });
    });

    // Auto-advance to next mnemon after HERO_AUTO_CYCLE_MS (pauses when details open)
    use_effect(move || {
        spawn(async move {
            loop {
                // Capture version at start of wait cycle
                let version_at_start = cycle_version();

                gloo_timers::future::TimeoutFuture::new(HERO_AUTO_CYCLE_MS).await;

                // If version changed during wait (manual nav or details toggled), restart timer
                if cycle_version() != version_at_start {
                    continue;
                }

                // Skip cycling if details are open, paused, transitioning, or temporarily paused from manual nav
                if details_open() || paused() || pause_auto_cycle() || is_transitioning() {
                    continue;
                }

                let total = app_state.peek().mnemons_count();
                if total == 0 {
                    continue;
                }

                // Calculate next index and start transition
                let next = (current_index() + 1) % total;
                transition_direction.set(Direction::Forward);
                next_index.set(Some(next));
                is_transitioning.set(true);

                // Wait for transition animation
                gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_MS).await;

                // Switch to next mnemon
                current_index.set(next);
                next_index.set(None);

                // End transition
                gloo_timers::future::TimeoutFuture::new(HERO_TRANSITION_SETTLE_MS).await;
                is_transitioning.set(false);
            }
        });
    });

    let has_mnemons = !mnemons_with_works().is_empty();
    let is_loaded = app_state.peek().is_loaded();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "h-screen w-screen overflow-hidden bg-gray-900",

            // Show loading state until IndexedDB data is loaded
            if is_loaded && has_mnemons {
                // Positioning container for overlapping Heroes during transitions
                div {
                    class: "relative h-full w-full overflow-hidden",

                    // During transition, render both current and next mnemon
                    if is_transitioning() && next_mnemon_with_work().is_some() {
                        // Render current mnemon (exiting) - lower z-index
                        if let Some(mnemon_with_work) = current_mnemon_with_work() {
                            div {
                                class: "absolute inset-0 z-10",
                                Hero {
                                    key: "{mnemon_with_work.mnemon.id}-current",
                                    mnemon_with_work: mnemon_with_work.clone(),
                                    is_transitioning: true,
                                    transition_direction: transition_direction(),
                                    is_exiting: true,
                                    details_open: details_open(),
                                    is_mobile: is_mobile(),
                                    on_add_click: move |_| {},
                                    on_details_toggle: move |_| {},
                                    on_edit: move |_| {},
                                    on_delete: move |_| {},
                                }
                            }
                        }
                        // Render next mnemon (entering) - higher z-index
                        if let Some(mnemon_with_work) = next_mnemon_with_work() {
                            div {
                                class: "absolute inset-0 z-20",
                                Hero {
                                    key: "{mnemon_with_work.mnemon.id}-next",
                                    mnemon_with_work: mnemon_with_work.clone(),
                                    is_transitioning: true,
                                    transition_direction: transition_direction(),
                                    is_exiting: false,
                                    details_open: details_open(),
                                    is_mobile: is_mobile(),
                                    on_add_click: move |_| {},
                                    on_details_toggle: move |_| {},
                                    on_edit: move |_| {},
                                    on_delete: move |_| {},
                                }
                            }
                        }
                    } else if let Some(mnemon_with_work) = current_mnemon_with_work() {
                    // Normal rendering (no transition)
                    Hero {
                        key: "{mnemon_with_work.mnemon.id}",
                        mnemon_with_work: mnemon_with_work.clone(),
                        is_transitioning: false,
                        transition_direction: Direction::Forward,
                        is_exiting: false,
                        details_open: details_open(),
                        is_mobile: is_mobile(),
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
                            // Remove from memory and store for potential undo
                            let removed = app_state.write().remove_mnemon(mnemon_id);
                            if let Some((mnemon, original_idx)) = removed {
                                pending_delete.set(Some(PendingDelete { mnemon, original_idx }));
                                // Close details view
                                details_open.set(false);
                                // Adjust current index if needed
                                let total = app_state.peek().mnemons_count();
                                if total > 0 && current_index() >= total {
                                    current_index.set(total - 1);
                                }
                            }
                        },
                        on_navigate_prev: move |_| {
                            navigate(Direction::Backward);
                        },
                        on_navigate_next: move |_| {
                            navigate(Direction::Forward);
                        },
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
