//! Hero component - main display for a mnemon with cover art and details

use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use uuid::Uuid;

use crate::app_state::MnemonWithWork;
use crate::components::MemoryDetails;
use crate::constants::*;
use crate::hooks::touch_gesture::{use_touch_gesture, SwipeDirection};
use crate::types::Direction;
use crate::utils::calculate_reading_time;

#[component]
pub fn Hero(
    mnemon_with_work: ReadSignal<MnemonWithWork>,
    is_transitioning: bool,
    #[props(default = Direction::Forward)]
    transition_direction: Direction,
    #[props(default = false)]
    is_exiting: bool,
    details_open: bool,
    on_add_click: EventHandler<()>,
    on_details_toggle: EventHandler<()>,
    on_edit: EventHandler<Uuid>,
    on_delete: EventHandler<Uuid>,
    #[props(default = None)]
    on_navigate_prev: Option<EventHandler<()>>,
    #[props(default = None)]
    on_navigate_next: Option<EventHandler<()>>,
    #[props(default = false)]
    is_mobile: bool,
) -> Element {
    let work = mnemon_with_work().work;
    let mnemon = mnemon_with_work().mnemon;

    // Measured height of the title bar content
    let mut title_bar_height = use_signal(|| 150.0f64); // Default fallback

    // Touch gesture handling (for mobile)
    let mut touch_gesture = use_touch_gesture();

    // Mounted state for entering animation trigger
    let mut is_mounted = use_signal(|| false);
    // Started state for exiting animation trigger
    let mut is_exiting_started = use_signal(|| false);

    // Effect to trigger animations after mount/start
    use_effect(move || {
        if is_transitioning {
            if !is_exiting {
                // Entering element: start off-screen, animate in after frame
                is_mounted.set(false);
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(16).await;
                    // Only set if still in entering state
                    if is_transitioning && !is_exiting {
                        is_mounted.set(true);
                    }
                });
            } else {
                // Exiting element: start centered, animate out after frame
                is_exiting_started.set(false);
                spawn(async move {
                    gloo_timers::future::TimeoutFuture::new(16).await;
                    // Only set if still in exiting state
                    if is_transitioning && is_exiting {
                        is_exiting_started.set(true);
                    }
                });
            }
        } else {
            // Reset states when not transitioning
            is_mounted.set(false);
            is_exiting_started.set(false);
        }
    });

    // Current note index
    let mut current_note_index = use_signal(|| 0usize);
    let mut note_visible = use_signal(|| true);

    // Selected notes - updates when mnemon changes
    let selected_notes = use_memo(move || {
        let mut rng = thread_rng();
        let mut notes = mnemon_with_work().mnemon.notes.clone();
        notes.shuffle(&mut rng);
        current_note_index.set(0);
        notes.into_iter().collect::<Vec<String>>()
    });

    // Rotate through notes with fade animation (only when details closed)
    use_effect(move || {
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

    // Horizontal transition for slideshow with direction support
    let horizontal_transition = if is_transitioning {
        if is_exiting {
            // Exiting element: start centered, slide out when started
            let translate = if is_exiting_started() {
                match transition_direction {
                    Direction::Forward => "-100%",
                    Direction::Backward => "100%",
                }
            } else {
                "0"
            };
            format!(
                "transform: translateX({}); transition: transform {}ms cubic-bezier(0.4, 0.0, 0.2, 1);",
                translate, HERO_TRANSITION_MS
            )
        } else {
            // Entering element: start off-screen, animate to center when mounted
            let translate = if is_mounted() {
                "0"
            } else {
                match transition_direction {
                    Direction::Forward => "100%",
                    Direction::Backward => "-100%",
                }
            };
            format!(
                "transform: translateX({}); transition: transform {}ms cubic-bezier(0.4, 0.0, 0.2, 1);",
                translate, HERO_TRANSITION_MS
            )
        }
    } else {
        "transform: translateX(0);".to_string()
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

                    // Touch handlers for mobile gestures (both horizontal and vertical)
                    ontouchstart: move |evt| {
                        if !is_mobile {
                            return;
                        }
                        if let Some(touch) = evt.touches().first() {
                            let coords = touch.page_coordinates();
                            touch_gesture.on_touch_start(coords.x, coords.y);
                        }
                    },

                    ontouchmove: move |evt| {
                        if !is_mobile || !touch_gesture.is_touching() {
                            return;
                        }
                        if let Some(touch) = evt.touches().first() {
                            let coords = touch.page_coordinates();
                            touch_gesture.on_touch_move(coords.x, coords.y, 20.0);
                        }
                    },

                    ontouchcancel: move |_| {
                        touch_gesture.reset();
                    },

                    ontouchend: move |_| {
                        if !is_mobile {
                            return;
                        }

                        const SWIPE_THRESHOLD: f64 = 50.0;

                        if let Some(direction) = touch_gesture.on_touch_end(SWIPE_THRESHOLD) {
                            match direction {
                                SwipeDirection::Up => {
                                    if !details_open {
                                        on_details_toggle.call(());
                                    }
                                }
                                SwipeDirection::Down => {
                                    if details_open {
                                        on_details_toggle.call(());
                                    }
                                }
                                SwipeDirection::Left => {
                                    if let Some(on_next) = on_navigate_next {
                                        on_next.call(());
                                    }
                                }
                                SwipeDirection::Right => {
                                    if let Some(on_prev) = on_navigate_prev {
                                        on_prev.call(());
                                    }
                                }
                            }
                        }
                    },

                    // Background cover image with overlay - overflow container for Ken Burns effect
                    div {
                        class: "absolute inset-0 z-0 overflow-hidden",

                        // Animated background layer
                        div {
                            class: format!(
                                "absolute inset-0 ken-burns-{} bg-yellow-300",
                                (mnemon.id.as_u128() % 6) + 1
                            ),
                            style: if let Some(ref url) = work.cover_image_local_uri {
                                format!("background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat;", url)
                            } else {
                                "background-color: #1a1a2e;".to_string()
                            },
                        }

                        // Dark overlay for readability
                        div {
                            class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/70 to-black/50"
                        }
                    }

                    // Note display - lower left (only visible when details closed)
                    if !details_open {
                        if let Some(note) = current_note() {
                            div {
                                class: "absolute left-4 top-2/3 z-10 max-w-80 transition-opacity duration-500",
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

                    // Left navigation zone (25% width) - only when callback provided
                    if let Some(on_prev) = on_navigate_prev {
                        div {
                            class: "absolute left-0 top-0 bottom-0 z-20 w-1/4 cursor-w-resize hover:bg-white/5 transition-colors",
                            onclick: move |e| {
                                e.stop_propagation();
                                on_prev.call(());
                            },
                        }
                    }

                    // Right navigation zone (25% width) - only when callback provided
                    if let Some(on_next) = on_navigate_next {
                        div {
                            class: "absolute right-0 top-0 bottom-0 z-20 w-1/4 cursor-e-resize hover:bg-white/5 transition-colors",
                            onclick: move |e| {
                                e.stop_propagation();
                                on_next.call(());
                            },
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
                        mnemon_with_work: mnemon_with_work.read().clone(),
                        on_edit: on_edit,
                        on_delete: on_delete,
                    }
                }
            }
        }
    }
}
