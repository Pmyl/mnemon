use dioxus::document::eval;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

// Mock data structures
#[derive(Clone, PartialEq, Debug)]
enum WorkType {
    Movie,
    TvAnime,
    Game,
}

impl WorkType {
    fn icon(&self) -> &'static str {
        match self {
            WorkType::Movie => "🎬",
            WorkType::TvAnime => "📺",
            WorkType::Game => "🎮",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            WorkType::Movie => "Movie",
            WorkType::TvAnime => "TV/Anime",
            WorkType::Game => "Game",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Mnemon {
    id: usize,
    title: String,
    year: u16,
    work_type: WorkType,
    cover_url: String,
    feelings: Vec<String>,
    finished_date: Option<String>,
    notes: Option<String>,
}

// Hardcoded sample data
fn get_sample_mnemons() -> Vec<Mnemon> {
    vec![
        Mnemon {
            id: 1,
            title: "Spirited Away".to_string(),
            year: 2001,
            work_type: WorkType::Movie,
            cover_url: "https://image.tmdb.org/t/p/w500/39wmItIWsg5sZMyRUHLkWBcuVCM.jpg".to_string(),
            feelings: vec!["Nostalgic".to_string(), "Wholesome".to_string(), "Mysterious".to_string()],
            finished_date: Some("2023-08-15".to_string()),
            notes: Some("Watched this again after 10 years. Still as magical as I remembered. The scene with the train over the water hits different now. Chihiro's growth throughout the story is so beautifully done.".to_string()),
        },
        Mnemon {
            id: 2,
            title: "The Legend of Zelda: Breath of the Wild".to_string(),
            year: 2017,
            work_type: WorkType::Game,
            cover_url: "https://images.igdb.com/igdb/image/upload/t_cover_big/co3p2d.png".to_string(),
            feelings: vec!["Epic".to_string(), "Adventurous".to_string(), "Uplifting".to_string()],
            finished_date: Some("2024-01-20".to_string()),
            notes: Some("120 hours of pure exploration. Every hill had something to discover. The final battle felt earned after everything. This game taught me that the journey matters more than the destination.".to_string()),
        },
        Mnemon {
            id: 3,
            title: "Cowboy Bebop".to_string(),
            year: 1998,
            work_type: WorkType::TvAnime,
            cover_url: "https://image.tmdb.org/t/p/w500/gZFHBd677gz8V5fyj8SZx5SrqTA.jpg".to_string(),
            feelings: vec!["Melancholic".to_string(), "Chill".to_string(), "Bittersweet".to_string()],
            finished_date: Some("2023-12-03".to_string()),
            notes: Some("See you, space cowboy. That ending still haunts me. The jazz soundtrack is perfection. Each episode feels like a short film.".to_string()),
        },
        Mnemon {
            id: 4,
            title: "The Grand Budapest Hotel".to_string(),
            year: 2014,
            work_type: WorkType::Movie,
            cover_url: "https://image.tmdb.org/t/p/w500/nX5XotM9yprCKarRH4fzOq1VM1J.jpg".to_string(),
            feelings: vec!["Cozy".to_string(), "Wholesome".to_string(), "Nostalgic".to_string()],
            finished_date: Some("2024-02-14".to_string()),
            notes: Some("Wes Anderson's masterpiece. Every frame is a painting. The pastel colors, the symmetry, the story within a story. M. Gustave is unforgettable.".to_string()),
        },
        Mnemon {
            id: 5,
            title: "Hollow Knight".to_string(),
            year: 2017,
            work_type: WorkType::Game,
            cover_url: "https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png".to_string(),
            feelings: vec!["Somber".to_string(), "Epic".to_string(), "Mysterious".to_string()],
            finished_date: Some("2023-11-28".to_string()),
            notes: Some("Defeated the Radiance after 60 hours. This game broke me and rebuilt me. The atmosphere is unmatched - haunting music, gorgeous hand-drawn art. Hallownest feels like a real place with its own history.".to_string()),
        },
        Mnemon {
            id: 6,
            title: "Your Name".to_string(),
            year: 2016,
            work_type: WorkType::Movie,
            cover_url: "https://image.tmdb.org/t/p/w500/q719jXXEzOoYaps6babgKnONONX.jpg".to_string(),
            feelings: vec!["Heartwarming".to_string(), "Bittersweet".to_string(), "Uplifting".to_string()],
            finished_date: Some("2024-03-01".to_string()),
            notes: Some("Cried three times. The red string of fate made visual. RADWIMPS soundtrack is on repeat. That moment when they finally meet... perfect.".to_string()),
        },
    ]
}

#[component]
fn App() -> Element {
    // Theme state: true = dark, false = light
    let mut theme = use_signal(|| true);

    // All mnemons
    let mnemons = use_signal(|| get_sample_mnemons());

    // Current mnemon index
    let mut current_index = use_signal(|| 0);

    // Apply dark mode class to document element whenever theme changes
    use_effect(move || {
        if theme() {
            eval("document.documentElement.classList.add('dark');");
        } else {
            eval("document.documentElement.classList.remove('dark');");
        }
    });

    let current_mnemon = use_memo(move || {
        let all = mnemons();
        let idx = current_index();
        all.get(idx).cloned()
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 transition-colors",

            // Header
            Header { theme }

            // Hero with current mnemon
            if let Some(mnemon) = current_mnemon() {
                Hero {
                    mnemon: mnemon.clone(),
                    on_next: move |_| {
                        let total = mnemons().len();
                        current_index.set((current_index() + 1) % total);
                    }
                }
            }
        }
    }
}

#[component]
fn Header(mut theme: Signal<bool>) -> Element {
    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 z-50 bg-white/95 dark:bg-gray-900/95 backdrop-blur-sm border-b border-gray-200 dark:border-gray-800",
            div {
                class: "container mx-auto px-4 py-4 flex items-center justify-between",

                // Logo
                div {
                    class: "text-2xl font-bold text-gray-900 dark:text-gray-100",
                    "Mnemon"
                }

                div {
                    class: "flex items-center gap-4",

                    // Add button
                    button {
                        class: "px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors font-medium",
                        "＋ Add"
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
}

#[component]
fn Hero(mnemon: Mnemon, on_next: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "relative min-h-screen w-full",

            // Background cover image with overlay
            div {
                class: "absolute inset-0 z-0",
                style: "background-image: url('{mnemon.cover_url}'); background-size: cover; background-position: center;",

                // Dark overlay for readability
                div {
                    class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/70 to-black/50"
                }
            }

            // Content overlay
            div {
                class: "relative z-10 container mx-auto px-6 pt-32 pb-16 flex items-center min-h-screen",

                div {
                    class: "max-w-3xl",

                    // Type and Year
                    div {
                        class: "flex items-center gap-2 mb-4 text-gray-200",
                        span {
                            class: "text-2xl",
                            "{mnemon.work_type.icon()}"
                        }
                        span {
                            class: "text-lg font-medium",
                            "{mnemon.year} • {mnemon.work_type.label()}"
                        }
                    }

                    // Title
                    h1 {
                        class: "text-5xl md:text-6xl font-bold mb-6 text-white leading-tight",
                        "{mnemon.title}"
                    }

                    // Feelings
                    if !mnemon.feelings.is_empty() {
                        div {
                            class: "flex flex-wrap gap-2 mb-6",
                            for feeling in mnemon.feelings.iter() {
                                span {
                                    class: "px-3 py-1 bg-white/20 backdrop-blur-sm text-white text-sm rounded-full border border-white/30",
                                    "{feeling}"
                                }
                            }
                        }
                    }

                    // Finished date
                    if let Some(date) = &mnemon.finished_date {
                        div {
                            class: "text-gray-300 mb-4",
                            "Finished on: {date}"
                        }
                    }

                    // Notes preview
                    if let Some(notes) = &mnemon.notes {
                        div {
                            class: "text-gray-200 text-lg mb-8 leading-relaxed line-clamp-3",
                            style: "display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;",
                            "{notes}"
                        }
                    }

                    // Action buttons
                    div {
                        class: "flex flex-wrap gap-4",

                        button {
                            class: "px-6 py-3 bg-white/20 backdrop-blur-sm hover:bg-white/30 text-white rounded-lg transition-all font-medium border border-white/30 flex items-center gap-2",
                            onclick: move |_| on_next.call(()),
                            span { "🔀" }
                            "Next Surprise"
                        }

                        button {
                            class: "px-6 py-3 bg-white hover:bg-gray-100 text-gray-900 rounded-lg transition-colors font-medium",
                            "Open Memory"
                        }
                    }
                }
            }
        }
    }
}
