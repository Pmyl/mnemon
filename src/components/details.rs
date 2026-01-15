//! Memory details component

use dioxus::prelude::*;
use uuid::Uuid;

use crate::app_state::MnemonWithWork;
use crate::components::EditIcon;

#[component]
pub fn MemoryDetails(
    mnemon_with_work: MnemonWithWork,
    on_edit: EventHandler<Uuid>,
    on_delete: EventHandler<Uuid>,
) -> Element {
    let work = &mnemon_with_work.work;
    let mnemon = &mnemon_with_work.mnemon;
    let mnemon_id = mnemon.id;

    // Stubbed audio state
    let mut is_playing = use_signal(|| false);

    rsx! {
        div {
            class: "h-full overflow-y-auto px-8 py-6 flex flex-col",

            // Header with Edit button
            div {
                class: "mb-6 flex items-center justify-between",

                h2 {
                    class: "text-2xl font-bold text-white",
                    "{work.title_en}"
                }

                button {
                    class: "px-4 py-2 bg-white/10 hover:bg-white/20 text-white rounded-lg transition-colors flex items-center gap-2",
                    onclick: move |_| on_edit.call(mnemon_id),

                    EditIcon {}
                    span { "Edit" }
                }
            }

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
                    class: "flex items-center justify-center flex-1 text-white/40 italic",
                    "No additional details for this memory"
                }
            }

            // Spacer to push delete button to bottom
            div { class: "flex-1" }

            // Delete button
            div {
                class: "pt-6 border-t border-white/10",

                button {
                    class: "w-full py-3 px-4 bg-red-500/20 hover:bg-red-500/30 text-red-400 rounded-lg transition-colors flex items-center justify-center gap-2",
                    onclick: move |_| on_delete.call(mnemon_id),

                    span { "🗑" }
                    span { "Delete this memory" }
                }
            }
        }
    }
}
