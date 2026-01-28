//! Settings modal component

use dioxus::prelude::*;
use tracing::info;

use crate::settings::ApiTokenSettings;

#[component]
pub fn SettingsModal(paused: Signal<bool>, on_close: EventHandler<()>) -> Element {
    // Load current settings into local state
    let mut local_settings = use_signal(ApiTokenSettings::load);
    let mut save_status = use_signal(|| Option::<bool>::None);

    // Check configuration status
    let tmdb_configured = local_settings().has_tmdb();
    let rawg_configured = local_settings().has_rawg();

    rsx! {
        // Modal overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm",
            onclick: move |_| on_close.call(()),

            // Modal content
            div {
                class: "bg-gray-800 rounded-lg shadow-2xl w-full max-w-lg mx-2 sm:mx-4 md:mx-auto max-h-[90vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "px-6 py-4 border-b border-gray-700 flex items-center justify-between",

                    h2 {
                        class: "text-xl font-semibold text-white",
                        "Settings"
                    }

                    button {
                        class: "text-gray-400 hover:text-white transition-colors",
                        onclick: move |_| on_close.call(()),
                        // X icon
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M6 18L18 6M6 6l12 12"
                            }
                        }
                    }
                }

                // Content
                div {
                    class: "px-6 py-4",

                    // Info text
                    p {
                        class: "text-gray-400 text-sm mb-6",
                        "Configure your API keys to enable search for movies, TV shows, and games. "
                        "Keys are stored locally in your browser."
                    }

                    // Auto-cycle pause toggle
                    div {
                        class: "mb-6 p-4 bg-gray-700/50 rounded-lg",

                        div {
                            class: "flex items-center justify-between gap-4",

                            div {
                                class: "flex-1",
                                label {
                                    class: "block text-white text-sm font-semibold mb-1",
                                    "Auto-cycle Playback"
                                }
                                p {
                                    class: "text-gray-400 text-xs mb-1",
                                    "Automatically rotate through your mnemons"
                                }
                                // Current status indicator
                                div {
                                    class: "flex items-center gap-1 mt-2",
                                    div {
                                        class: if !paused() {
                                            "w-2 h-2 bg-green-500 rounded-full animate-pulse"
                                        } else {
                                            "w-2 h-2 bg-gray-500 rounded-full"
                                        }
                                    }
                                    span {
                                        class: if !paused() {
                                            "text-xs font-medium text-green-400"
                                        } else {
                                            "text-xs font-medium text-gray-500"
                                        },
                                        if !paused() {
                                            "Playing"
                                        } else {
                                            "Paused"
                                        }
                                    }
                                }
                            }

                            button {
                                class: if !paused() {
                                    "relative w-14 h-7 bg-green-600 hover:bg-green-700 rounded-full transition-all focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                                } else {
                                    "relative w-14 h-7 bg-gray-600 hover:bg-gray-500 rounded-full transition-all focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 focus:ring-offset-gray-800"
                                },
                                onclick: move |_| {
                                    paused.toggle();
                                    info!("Auto-cycle paused: {}", paused());
                                },
                                "aria-label": if !paused() { "Pause auto-cycle" } else { "Resume auto-cycle" },
                                "aria-pressed": if !paused() { "true" } else { "false" },

                                div {
                                    class: if !paused() {
                                        "absolute left-8 top-1 w-5 h-5 bg-white rounded-full shadow-md transition-transform duration-200 ease-in-out"
                                    } else {
                                        "absolute left-1 top-1 w-5 h-5 bg-white rounded-full shadow-md transition-transform duration-200 ease-in-out"
                                    }
                                }
                            }
                        }
                    }

                    // TMDB Token
                    div {
                        class: "mb-6",

                        div {
                            class: "flex items-center justify-between mb-2",
                            label {
                                class: "block text-white text-sm font-semibold",
                                "TMDB Access Token"
                            }
                            // Status indicator
                            if tmdb_configured {
                                span {
                                    class: "text-xs px-2 py-1 bg-green-500/20 text-green-400 rounded-full",
                                    "✓ Configured"
                                }
                            } else {
                                span {
                                    class: "text-xs px-2 py-1 bg-yellow-500/20 text-yellow-400 rounded-full",
                                    "Not configured"
                                }
                            }
                        }

                        p {
                            class: "text-gray-500 text-xs mb-2",
                            "For Movies and TV/Anime search. Get one at "
                            a {
                                class: "text-blue-400 hover:underline",
                                href: "https://www.themoviedb.org/settings/api",
                                target: "_blank",
                                "themoviedb.org"
                            }
                        }

                        input {
                            class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none font-mono text-sm",
                            r#type: "password",
                            placeholder: "eyJhbGciOiJIUzI1NiJ9...",
                            value: "{local_settings().tmdb_token}",
                            oninput: move |e| {
                                local_settings.with_mut(|s| s.tmdb_token = e.value());
                                save_status.set(None);
                            }
                        }
                    }

                    // RAWG API Key
                    div {
                        class: "mb-6",

                        div {
                            class: "flex items-center justify-between mb-2",
                            label {
                                class: "block text-white text-sm font-semibold",
                                "RAWG API Key"
                            }
                            // Status indicator
                            if rawg_configured {
                                span {
                                    class: "text-xs px-2 py-1 bg-green-500/20 text-green-400 rounded-full",
                                    "✓ Configured"
                                }
                            } else {
                                span {
                                    class: "text-xs px-2 py-1 bg-yellow-500/20 text-yellow-400 rounded-full",
                                    "Not configured"
                                }
                            }
                        }

                        p {
                            class: "text-gray-500 text-xs mb-2",
                            "For Games search. Get one at "
                            a {
                                class: "text-blue-400 hover:underline",
                                href: "https://rawg.io/apidocs",
                                target: "_blank",
                                "rawg.io"
                            }
                        }

                        input {
                            class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none font-mono text-sm",
                            r#type: "password",
                            placeholder: "abc123def456...",
                            value: "{local_settings().rawg_api_key}",
                            oninput: move |e| {
                                local_settings.with_mut(|s| s.rawg_api_key = e.value());
                                save_status.set(None);
                            }
                        }
                    }

                    // Save status message
                    if let Some(success) = save_status() {
                        div {
                            class: if success {
                                "mb-4 px-4 py-2 bg-green-500/20 text-green-400 rounded-lg text-sm"
                            } else {
                                "mb-4 px-4 py-2 bg-red-500/20 text-red-400 rounded-lg text-sm"
                            },
                            if success {
                                "Settings saved!"
                            } else {
                                "Failed to save settings. Please try again."
                            }
                        }
                    }
                }

                // Footer with actions
                div {
                    class: "px-6 py-4 border-t border-gray-700 flex justify-end gap-3",

                    button {
                        class: "px-4 py-2 text-gray-400 hover:text-white transition-colors",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }

                    button {
                        class: "px-6 py-2 bg-transparent border-2 border-white hover:bg-white/10 text-white rounded-lg font-medium transition-colors",
                        onclick: move |_| {
                            let success = local_settings().save();
                            save_status.set(Some(success));
                            if success {
                                info!("Settings saved successfully");
                            }
                        },
                        "Save"
                    }
                }
            }
        }
    }
}
