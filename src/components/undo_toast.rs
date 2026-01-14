//! Undo toast component for deletions

use dioxus::prelude::*;

use crate::constants::*;
use crate::models::Mnemon;

/// Data for a pending deletion that can be undone
#[derive(Clone, PartialEq, Debug)]
pub struct PendingDelete {
    pub mnemon: Mnemon,
    pub original_idx: usize,
}

/// Update interval for undo toast progress bar (ms)
const UNDO_PROGRESS_INTERVAL_MS: u32 = 50;

#[component]
pub fn UndoToast(message: String, on_undo: EventHandler<()>, on_timeout: EventHandler<()>) -> Element {
    let mut progress = use_signal(|| 100.0f64);

    // Animate progress bar and handle timeout using gloo_timers
    use_effect(move || {
        spawn(async move {
            let steps = UNDO_TIMEOUT_MS / UNDO_PROGRESS_INTERVAL_MS;
            let decrement = 100.0 / steps as f64;

            for _ in 0..steps {
                gloo_timers::future::TimeoutFuture::new(UNDO_PROGRESS_INTERVAL_MS).await;
                progress.with_mut(|p| *p -= decrement);
            }

            on_timeout.call(());
        });
    });

    let progress_width = progress();

    rsx! {
        div {
            class: "fixed bottom-8 left-1/2 transform -translate-x-1/2 z-50",

            div {
                class: "bg-gray-800 border border-white/20 rounded-lg shadow-2xl overflow-hidden min-w-80",

                // Content
                div {
                    class: "px-4 py-3 flex items-center justify-between gap-4",

                    span {
                        class: "text-white/90",
                        "{message}"
                    }

                    button {
                        class: "px-4 py-1 bg-white/20 hover:bg-white/30 text-white rounded transition-colors font-medium",
                        onclick: move |_| on_undo.call(()),
                        "Undo"
                    }
                }

                // Progress bar (depletes from right to left)
                div {
                    class: "h-1 bg-white/10",

                    div {
                        class: "h-full bg-red-500",
                        style: "width: {progress_width}%;",
                    }
                }
            }
        }
    }
}
