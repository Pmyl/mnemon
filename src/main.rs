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
    // All mnemons
    let mnemons = use_signal(|| get_sample_mnemons());

    // Current mnemon index
    let mut current_index = use_signal(|| 0);

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
            class: "h-screen w-screen overflow-hidden bg-gray-900",

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
fn Hero(mnemon: Mnemon, on_next: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "relative h-full w-full",

            // Background cover image with overlay
            div {
                class: "absolute inset-0 z-0",
                style: "background-image: url('{mnemon.cover_url}'); background-size: cover; background-position: center; background-repeat: no-repeat;",

                // Dark overlay for readability
                div {
                    class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/70 to-black/50"
                }
            }

            // Content overlay - footnote style at bottom right
            div {
                class: "relative z-10 px-8 pb-8 flex items-end justify-end h-full",

                div {
                    class: "max-w-md",

                    // Icon and Title
                    div {
                        class: "flex items-center gap-3 mb-3",
                        span {
                            class: "text-2xl opacity-90",
                            "{mnemon.work_type.icon()}"
                        }
                        h1 {
                            class: "text-2xl font-semibold text-white/95",
                            "{mnemon.title}"
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
        }
    }
}
