use dioxus::document::eval;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Theme state: true = dark, false = light
    let theme = use_signal(|| true);

    // Apply dark mode class to document element whenever theme changes
    use_effect(move || {
        if theme() {
            eval("document.documentElement.classList.add('dark');");
        } else {
            eval("document.documentElement.classList.remove('dark');");
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 transition-colors",

            // Header
            Header { theme }

            // Hero (Empty State)
            EmptyHero {}
        }
    }
}

#[component]
fn Header(mut theme: Signal<bool>) -> Element {
    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 z-50 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800",
            div {
                class: "container mx-auto px-4 py-4 flex items-center justify-between",

                // Logo
                div {
                    class: "text-2xl font-bold text-gray-900 dark:text-gray-100",
                    "Mnemon"
                }

                // Theme Toggle
                button {
                    class: "p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors",
                    onclick: move |_| {
                        theme.set(!theme());
                    },
                    span {
                        class: "text-2xl",
                        if theme() {
                            "🌙"
                        } else {
                            "☀️"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyHero() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen pt-16",
            div {
                class: "text-center px-4",

                // Title
                h1 {
                    class: "text-4xl md:text-5xl font-bold mb-4 text-gray-900 dark:text-gray-100",
                    "Add your first mnemon"
                }

                // Subtitle
                p {
                    class: "text-lg md:text-xl text-gray-600 dark:text-gray-400",
                    "Capture a great movie, TV/anime, or game you loved. Nostalgia awaits."
                }
            }
        }
    }
}
