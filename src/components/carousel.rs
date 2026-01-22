use dioxus::prelude::*;
use uuid::Uuid;

use crate::app_state::MnemonWithWork;
use crate::components::hero::Hero;
use crate::utils::gestures::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CarouselPosition {
    Prev,
    Current,
    Next,
}

#[component]
pub fn Carousel(
    prev_mnemon: Option<MnemonWithWork>,
    current_mnemon: MnemonWithWork,
    next_mnemon: Option<MnemonWithWork>,
    details_open: bool,
    is_animating: bool,
    on_navigate_next: EventHandler<()>,
    on_navigate_prev: EventHandler<()>,
    on_user_interaction_start: EventHandler<()>,
    on_user_interaction_end: EventHandler<()>,
    #[props(default)] on_add_click: Option<EventHandler<()>>,
    #[props(default)] on_details_toggle: Option<EventHandler<()>>,
    #[props(default)] on_edit: Option<EventHandler<Uuid>>,
    #[props(default)] on_delete: Option<EventHandler<Uuid>>,
) -> Element {
    let mut gesture_state = use_signal(GestureState::default);
    let mut drag_offset = use_signal(|| 0.0f64);

    rsx! {
        div {
            class: "relative h-screen w-screen overflow-hidden",
            role: "region",
            "aria-label": "Memory carousel",
            tabindex: "0",

            onkeydown: move |evt| {
                match evt.key() {
                    Key::ArrowLeft => on_navigate_prev.call(()),
                    Key::ArrowRight => on_navigate_next.call(()),
                    _ => {}
                }
            },

            onpointerdown: move |evt| {
                let x = evt.data().page_coordinates().x as f64;
                let y = evt.data().page_coordinates().y as f64;
                gesture_state.write().start(x, y);
                on_user_interaction_start.call(());
            },

            onpointermove: move |evt| {
                let mut state = gesture_state.write();
                if !state.is_tracking {
                    return;
                }

                let x = evt.data().page_coordinates().x as f64;
                let y = evt.data().page_coordinates().y as f64;
                state.update(x, y);

                if state.locked_direction == Some(GestureDirection::Horizontal) {
                    let (dx, _) = state.get_delta();
                    drag_offset.set(dx);
                }
            },

            onpointerup: move |_| {
                let gesture_result = {
                    let state = gesture_state.read();
                    state.complete()
                };

                match gesture_result {
                    CompletedGesture::SwipeLeft => on_navigate_next.call(()),
                    CompletedGesture::SwipeRight => on_navigate_prev.call(()),
                    _ => {}
                }

                drag_offset.set(0.0);
                gesture_state.write().reset();
                on_user_interaction_end.call(());
            },

            div {
                class: "absolute inset-0 flex",
                style: {
                    if is_animating {
                        "transform: translateX(-100%); transition: transform 600ms cubic-bezier(0.4, 0.0, 0.2, 1);".to_string()
                    } else {
                        let drag_px = drag_offset();
                        format!("transform: translateX(calc(-100% + {}px));", drag_px)
                    }
                },

                if let Some(mnemon) = prev_mnemon {
                    div {
                        class: "w-screen h-screen flex-shrink-0",
                        Hero {
                            key: "{mnemon.mnemon.id}-prev",
                            mnemon_with_work: mnemon,
                            carousel_position: CarouselPosition::Prev,
                            details_open: false,
                        }
                    }
                }

                div {
                    class: "w-screen h-screen flex-shrink-0",
                    Hero {
                        key: "{current_mnemon.mnemon.id}-curr",
                        mnemon_with_work: current_mnemon.clone(),
                        carousel_position: CarouselPosition::Current,
                        details_open: details_open,
                        on_add_click: on_add_click,
                        on_details_toggle: on_details_toggle,
                        on_edit: on_edit,
                        on_delete: on_delete,
                    }
                }

                if let Some(mnemon) = next_mnemon {
                    div {
                        class: "w-screen h-screen flex-shrink-0",
                        Hero {
                            key: "{mnemon.mnemon.id}-next",
                            mnemon_with_work: mnemon,
                            carousel_position: CarouselPosition::Next,
                            details_open: false,
                        }
                    }
                }
            }

            div {
                class: "hidden md:block absolute left-0 top-0 bottom-0 w-1/3 z-10 cursor-pointer hover:bg-white/5 transition-colors",
                onclick: move |_| on_navigate_prev.call(()),
            }
            div {
                class: "hidden md:block absolute right-0 top-0 bottom-0 w-1/3 z-10 cursor-pointer hover:bg-white/5 transition-colors",
                onclick: move |_| on_navigate_next.call(()),
            }
        }
    }
}
