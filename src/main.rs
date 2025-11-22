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
    notes: Vec<String>,
}

// Hardcoded sample data
fn get_sample_mnemons() -> Vec<Mnemon> {
    vec![
        Mnemon {
            id: 1,
            title: "Spirited Away".to_string(),
            year: 2001,
            work_type: WorkType::Movie,
            cover_url: "https://image.tmdb.org/t/p/w500/39wmItIWsg5sZMyRUHLkWBcuVCM.jpg"
                .to_string(),
            feelings: vec![
                "Nostalgic".to_string(),
                "Wholesome".to_string(),
                "Mysterious".to_string(),
            ],
            finished_date: Some("2023-08-15".to_string()),
            notes: vec![
                "Watched this again after 10 years. Still as magical as I remembered.".to_string(),
                "The scene with the train over the water hits different now.".to_string(),
                "Chihiro's growth throughout the story is so beautifully done.".to_string(),
                "No-Face represents loneliness in such a profound way.".to_string(),
            ],
        },
        Mnemon {
            id: 2,
            title: "The Legend of Zelda: Breath of the Wild".to_string(),
            year: 2017,
            work_type: WorkType::Game,
            cover_url: "https://images.igdb.com/igdb/image/upload/t_cover_big/co3p2d.png"
                .to_string(),
            feelings: vec![
                "Epic".to_string(),
                "Adventurous".to_string(),
                "Uplifting".to_string(),
            ],
            finished_date: Some("2024-01-20".to_string()),
            notes: vec![
                "120 hours of pure exploration. Every hill had something to discover.".to_string(),
                "The final battle felt earned after everything.".to_string(),
                "This game taught me that the journey matters more than the destination."
                    .to_string(),
                "Climbing in the rain became strangely meditative.".to_string(),
            ],
        },
        Mnemon {
            id: 3,
            title: "Cowboy Bebop".to_string(),
            year: 1998,
            work_type: WorkType::TvAnime,
            cover_url: "https://image.tmdb.org/t/p/w500/gZFHBd677gz8V5fyj8SZx5SrqTA.jpg"
                .to_string(),
            feelings: vec![
                "Melancholic".to_string(),
                "Chill".to_string(),
                "Bittersweet".to_string(),
            ],
            finished_date: Some("2023-12-03".to_string()),
            notes: vec![
                "See you, space cowboy. That ending still haunts me.".to_string(),
                "The jazz soundtrack is perfection. Each episode feels like a short film."
                    .to_string(),
                "Spike's past catching up to him felt inevitable and tragic.".to_string(),
            ],
        },
        Mnemon {
            id: 4,
            title: "The Grand Budapest Hotel".to_string(),
            year: 2014,
            work_type: WorkType::Movie,
            cover_url: "https://image.tmdb.org/t/p/w500/nX5XotM9yprCKarRH4fzOq1VM1J.jpg"
                .to_string(),
            feelings: vec![
                "Cozy".to_string(),
                "Wholesome".to_string(),
                "Nostalgic".to_string(),
            ],
            finished_date: Some("2024-02-14".to_string()),
            notes: vec![
                "Wes Anderson's masterpiece. Every frame is a painting.".to_string(),
                "The pastel colors and symmetry create a dreamlike world.".to_string(),
                "M. Gustave is unforgettable. Ralph Fiennes at his best.".to_string(),
            ],
        },
        Mnemon {
            id: 5,
            title: "Hollow Knight".to_string(),
            year: 2017,
            work_type: WorkType::Game,
            cover_url: "https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png"
                .to_string(),
            feelings: vec![
                "Somber".to_string(),
                "Epic".to_string(),
                "Mysterious".to_string(),
            ],
            finished_date: Some("2023-11-28".to_string()),
            notes: vec![
                "Defeated the Radiance after 60 hours. This game broke me and rebuilt me."
                    .to_string(),
                "The atmosphere is unmatched - haunting music, gorgeous hand-drawn art."
                    .to_string(),
                "Hallownest feels like a real place with its own history.".to_string(),
                "Every boss taught me something about patience.".to_string(),
            ],
        },
        Mnemon {
            id: 6,
            title: "Your Name".to_string(),
            year: 2016,
            work_type: WorkType::Movie,
            cover_url: "https://image.tmdb.org/t/p/w500/q719jXXEzOoYaps6babgKnONONX.jpg"
                .to_string(),
            feelings: vec![
                "Heartwarming".to_string(),
                "Bittersweet".to_string(),
                "Uplifting".to_string(),
            ],
            finished_date: Some("2024-03-01".to_string()),
            notes: vec![
                "Cried three times. The red string of fate made visual.".to_string(),
                "RADWIMPS soundtrack is on repeat. Pure emotion in every track.".to_string(),
                "That moment when they finally meet... perfect.".to_string(),
                "The way time and memory intertwine is beautifully handled.".to_string(),
            ],
        },
    ]
}

#[component]
fn App() -> Element {
    // All mnemons
    let mnemons = use_signal(|| get_sample_mnemons());

    // Current mnemon index
    let mut current_index = use_signal(|| 0);
    let mut is_transitioning = use_signal(|| false);

    let current_mnemon = use_memo(move || {
        let all = mnemons();
        let idx = current_index();
        all.get(idx).cloned()
    });

    // Auto-advance to next mnemon after 10 seconds (continuous cycle)
    // This effect only runs once on mount
    use_effect(use_reactive!(|| {
        let total = mnemons().len();

        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(10_000).await;

                // Start transition (slide out to left)
                is_transitioning.set(true);
                gloo_timers::future::TimeoutFuture::new(600).await;

                // Switch mnemon
                current_index.with_mut(|idx| *idx = (*idx + 1) % total);

                // End transition (slide in from right)
                gloo_timers::future::TimeoutFuture::new(50).await;
                is_transitioning.set(false);
            }
        });
    }));

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
                    is_transitioning: is_transitioning(),
                }
            }
        }
    }
}

// Calculate reading time in milliseconds based on word count
// Average reading speed: ~250 words per minute = ~4 words per second
// Minimum 3 seconds, maximum 8 seconds
fn calculate_reading_time(text: &str) -> u64 {
    let word_count = text.split_whitespace().count();
    let seconds = (word_count as f64 / 4.0).ceil() as u64;
    (seconds.max(3).min(8)) * 1000
}

#[component]
fn Hero(mnemon: ReadSignal<Mnemon>, is_transitioning: bool) -> Element {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    // Current note index
    let mut current_note_index = use_signal(|| 0);
    let mut note_visible = use_signal(|| true);

    // Selected notes - updates when mnemon changes
    let selected_notes = use_memo(move || {
        let mut rng = thread_rng();
        let mut notes = mnemon().notes.clone();
        notes.shuffle(&mut rng);
        current_note_index.set(0);
        notes.into_iter().take(2).collect::<Vec<String>>()
    });

    // Rotate through notes with fade animation
    use_effect(move || {
        let notes = selected_notes();
        if notes.is_empty() {
            return;
        }

        let idx = current_note_index();
        let current_text = notes.get(idx).cloned().unwrap_or_default();
        let duration = calculate_reading_time(&current_text);

        spawn(async move {
            // Wait for reading time
            gloo_timers::future::TimeoutFuture::new(duration as u32).await;

            // Fade out
            note_visible.set(false);
            gloo_timers::future::TimeoutFuture::new(500).await;

            // Switch to next note
            let next_idx = (idx + 1) % notes.len();
            current_note_index.set(next_idx);

            // Fade in
            note_visible.set(true);
        });
    });

    let current_note = use_memo(move || {
        let notes = selected_notes();
        let idx = current_note_index();
        notes.get(idx).cloned()
    });

    let transition_style = if is_transitioning {
        "transform: translateX(-100%); transition: transform 0.6s cubic-bezier(0.4, 0.0, 0.2, 1);"
    } else {
        "transform: translateX(0); transition: transform 0.6s cubic-bezier(0.4, 0.0, 0.2, 1);"
    };

    rsx! {
        div {
            class: "relative h-full w-full",
            style: "{transition_style}",

            // Background cover image with overlay
            div {
                class: "absolute inset-0 z-0",
                style: "background-image: url('{mnemon().cover_url}'); background-size: cover; background-position: center; background-repeat: no-repeat;",

                // Dark overlay for readability
                div {
                    class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/70 to-black/50"
                }
            }

            // Note display - lower left
            if let Some(note) = current_note() {
                div {
                    class: "absolute left-4 top-2/3 z-10 max-w-lg-1 max-w-80 transition-opacity duration-500",
                    style: if note_visible() { "opacity: 1;" } else { "opacity: 0;" },

                    p {
                        class: "text-white/90 text-lg leading-relaxed font-light italic",
                        "\"{note}\""
                    }
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
                            "{mnemon().work_type.icon()}"
                        }
                        h1 {
                            class: "text-2xl font-semibold text-white/95",
                            "{mnemon().title}"
                        }
                    }

                    // Feelings
                    if !mnemon().feelings.is_empty() {
                        div {
                            class: "flex flex-wrap gap-2",
                            for feeling in mnemon().feelings.iter() {
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
