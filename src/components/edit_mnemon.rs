//! Edit mnemon flow component

use dioxus::prelude::*;

use crate::components::{FeelingsSelector, FinishedDateInput, NotesInput};
use crate::forms::MnemonForm;

#[component]
pub fn EditMnemonFlow(
    initial_form: MnemonForm,
    on_save: EventHandler<MnemonForm>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut form = use_signal(|| initial_form.clone());
    
    // Reset form whenever initial_form changes (e.g., when editing a different mnemon
    // or reopening the modal for the same mnemon after saving)
    use_effect(use_reactive((&initial_form,), move |(initial_form,)| {
        form.set(initial_form.clone());
    }));

    rsx! {
        // Modal overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "edit-modal-title",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    on_cancel.call(());
                }
            },

            // Modal content
            div {
                class: "bg-gray-800 rounded-lg shadow-2xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto",

                div {
                    class: "p-8",

                    // Header
                    div {
                        class: "mb-6",
                        h2 {
                            id: "edit-modal-title",
                            class: "text-3xl font-bold text-white mb-2",
                            "Edit mnemon"
                        }
                        p {
                            class: "text-gray-400",
                            "Update your feelings, notes, and finished date"
                        }
                    }

                    // Work info (read-only display)
                    div {
                        class: "mb-6 p-4 bg-gray-700/50 rounded-lg",
                        div {
                            class: "flex items-center gap-2 mb-2",
                            span {
                                class: "text-xl",
                                "{form().work_type.as_ref().map(|wt| wt.icon()).unwrap_or(\"📄\")}"
                            }
                            h3 {
                                class: "text-lg font-semibold text-white",
                                "{form().title}"
                            }
                        }
                        if !form().year.is_empty() {
                            p {
                                class: "text-gray-400 text-sm",
                                "Released: {form().year}"
                            }
                        }
                    }

                    // Notes
                    NotesInput { form: form }

                    // Feelings
                    FeelingsSelector { form: form }

                    // Finished date
                    FinishedDateInput { form: form }

                    // Actions
                    div {
                        class: "flex gap-3 justify-end",
                        button {
                            class: "px-6 py-3 text-gray-300 hover:text-white font-medium",
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                        button {
                            class: "px-6 py-3 bg-transparent border-2 border-white hover:bg-white/10 text-white rounded-lg font-medium",
                            onclick: move |_| {
                                on_save.call(form());
                            },
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}
