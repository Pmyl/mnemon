use dioxus::prelude::*;

use crate::constants::*;
use crate::forms::MnemonForm;

#[component]
pub fn EditIcon() -> Element {
    rsx! {
        svg {
            class: "w-4 h-4",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            view_box: "0 0 24 24",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10"
            }
        }
    }
}

#[component]
pub fn NotesInput(form: Signal<MnemonForm>) -> Element {
    rsx! {
        div {
            class: "mb-6",
            label {
                class: "block text-white text-sm font-semibold mb-2",
                "Notes"
            }
            textarea {
                class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-white focus:outline-none min-h-[120px] resize-y",
                placeholder: "Add your thoughts, memories, or reflections...",
                value: "{form().notes}",
                oninput: move |e| {
                    form.with_mut(|f| f.notes = e.value());
                }
            }
        }
    }
}

#[component]
pub fn FeelingsSelector(form: Signal<MnemonForm>) -> Element {
    rsx! {
        div {
            class: "mb-6",
            label {
                class: "block text-white text-sm font-semibold mb-3",
                "Feelings"
                span { class: "text-gray-400 ml-2 text-xs", "(choose up to {MAX_FEELINGS})" }
            }
            div {
                class: "flex flex-wrap gap-2",
                for (feeling_name, feeling_emoji) in FEELINGS {
                    {
                        let is_selected = form().feelings.contains(&feeling_name.to_string());
                        let feelings_count = form().feelings.len();
                        let can_add = feelings_count < MAX_FEELINGS;

                        rsx! {
                            button {
                                class: if is_selected {
                                    "px-4 py-2 bg-transparent text-white rounded-full border-2 border-white text-sm font-medium"
                                } else if can_add {
                                    "px-4 py-2 bg-gray-700 text-gray-300 rounded-full border-2 border-gray-600 hover:border-gray-500 text-sm font-medium cursor-pointer"
                                } else {
                                    "px-4 py-2 bg-gray-800 text-gray-500 rounded-full border-2 border-gray-700 text-sm font-medium cursor-not-allowed opacity-50"
                                },
                                disabled: !is_selected && !can_add,
                                onclick: move |_| {
                                    form.with_mut(|f| {
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
    }
}

#[component]
pub fn FinishedDateInput(form: Signal<MnemonForm>) -> Element {
    rsx! {
        div {
            class: "mb-8",
            label {
                class: "block text-white text-sm font-semibold mb-2",
                "Finished date"
                span { class: "text-gray-400 ml-1 text-xs", "(when you completed it)" }
            }
            input {
                class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-white focus:outline-none",
                r#type: "date",
                value: "{form().finished_date}",
                oninput: move |e| {
                    form.with_mut(|f| f.finished_date = e.value());
                }
            }
        }
    }
}
