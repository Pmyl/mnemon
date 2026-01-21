use dioxus::prelude::*;

#[component]
pub fn EmptyState(on_click: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "h-full w-full flex items-center justify-center cursor-pointer",
            onclick: move |_| on_click.call(()),

            div {
                class: "text-center px-8 max-w-md",

                h1 {
                    class: "text-4xl font-semibold mb-4 text-white",
                    "Add your first mnemon"
                }

                p {
                    class: "text-lg opacity-70 mb-8 text-white",
                    "Capture a great movie, TV/anime, or game you loved. Nostalgia awaits."
                }

                p {
                    class: "text-sm opacity-50 italic text-white",
                    "Tap anywhere to begin"
                }
            }
        }
    }
}
