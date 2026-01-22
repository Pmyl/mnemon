//! Hero component - main display for a mnemon with cover art and details

use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use uuid::Uuid;

use crate::app_state::MnemonWithWork;
use crate::components::{CarouselPosition, MemoryDetails};
use crate::constants::*;
use crate::utils::calculate_reading_time;

#[component]
pub fn Hero(
    mnemon_with_work: MnemonWithWork,
    #[props(default = CarouselPosition::Current)] carousel_position: CarouselPosition,
    details_open: bool,
    #[props(default)] on_add_click: Option<EventHandler<()>>,
    #[props(default)] on_details_toggle: Option<EventHandler<()>>,
    #[props(default)] on_edit: Option<EventHandler<Uuid>>,
    #[props(default)] on_delete: Option<EventHandler<Uuid>>,
) -> Element {
    let work = mnemon_with_work.work.clone();
    let mnemon = mnemon_with_work.mnemon.clone();
    let mnemon_with_work_for_memo = mnemon_with_work.clone();
    let mnemon_with_work_for_details = mnemon_with_work.clone();
    let is_current = carousel_position == CarouselPosition::Current;

    // Measured height of the title bar content
    let mut title_bar_height = use_signal(|| 150.0f64); // Default fallback

    // Current note index
    let mut current_note_index = use_signal(|| 0usize);
    let mut note_visible = use_signal(|| true);

    // Selected notes - updates when mnemon changes
    let selected_notes = use_memo(move || {
        let mut rng = thread_rng();
        let mut notes = mnemon_with_work_for_memo.mnemon.notes.clone();
        notes.shuffle(&mut rng);
        current_note_index.set(0);
        notes.into_iter().collect::<Vec<String>>()
    });

    // Rotate through notes with fade animation (only when details closed and current position)
    use_effect(move || {
        if !is_current {
            return;
        }

        let notes = selected_notes();
        if notes.is_empty() {
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

            // Re-read notes to ensure consistency after async operations
            let current_notes = selected_notes();
            if current_notes.is_empty() {
                return;
            }

            // Switch to next note with bounds checking
            let next_idx = (idx + 1) % current_notes.len();
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

    // Vertical slide for details reveal - slides up to show only title bar (using measured height)
    // Add 32px top padding so title doesn't sit flush at top of viewport
    // Only apply to current carousel position
    let measured_height = title_bar_height();
    let visible_height = measured_height + 32.0;
    let vertical_slide = if is_current && details_open {
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

            // Sliding container (hero + details stacked vertically)
            div {
                class: "absolute inset-0",
                style: "{vertical_slide}",

                // Hero section (full viewport height) - original layout preserved
                div {
                    class: if is_current { "relative h-screen w-full cursor-pointer" } else { "relative h-screen w-full" },
                    onclick: move |_| {
                        if is_current {
                            if let Some(handler) = on_add_click {
                                handler.call(());
                            }
                        }
                    },

                    // Background cover image with overlay
                    div {
                        class: "absolute inset-0 z-0 bg-yellow-300",
                        style: if let Some(ref url) = work.cover_image_local_uri {
                            format!("background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat; animation: kenBurns 45s ease-in-out infinite; will-change: transform;", url)
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
                    // Only active for current carousel position
                    if is_current {
                        div {
                            class: "absolute bottom-0 left-0 right-0 z-30 cursor-pointer",
                            style: "height: {visible_height}px;",
                            onclick: move |e| {
                                e.stop_propagation();
                                if let Some(handler) = on_details_toggle {
                                    handler.call(());
                                }
                            },
                        }
                    }
                }

                // Details section (below hero, same height as viewport minus visible title area)
                // Only render for current carousel position
                if is_current {
                    if let (Some(edit_handler), Some(delete_handler)) = (on_edit, on_delete) {
                        div {
                            class: "relative w-full bg-gray-900",
                            style: "height: calc(100vh - {visible_height}px);",

                            MemoryDetails {
                                mnemon_with_work: mnemon_with_work_for_details.clone(),
                                on_edit: edit_handler,
                                on_delete: delete_handler,
                            }
                        }
                    }
                }
            }
        }
    }
}
