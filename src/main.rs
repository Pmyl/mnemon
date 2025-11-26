use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

// Fixed feelings taxonomy
const FEELINGS: &[&str] = &[
    "Nostalgic",
    "Cozy",
    "Melancholic",
    "Epic",
    "Wholesome",
    "Bittersweet",
    "Heartwarming",
    "Chill",
    "Adventurous",
    "Uplifting",
    "Mysterious",
    "Somber",
];

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
    year: Option<u16>,
    work_type: WorkType,
    cover_url: Option<String>,
    feelings: Vec<String>,
    finished_date: Option<String>,
    notes: Vec<String>,
}

// Form data for adding a mnemon
#[derive(Clone, PartialEq, Debug, Default)]
struct AddMnemonForm {
    // Step 1 fields
    work_type: Option<WorkType>,
    title: String,
    year: String,

    // Step 2 fields
    finished_date: String,
    feelings: Vec<String>,
    notes: String,
}

impl AddMnemonForm {
    fn is_step1_valid(&self) -> bool {
        self.work_type.is_some() && !self.title.trim().is_empty()
    }
}

// Hardcoded sample data (kept for reference/testing)
#[allow(dead_code)]
fn get_sample_mnemons() -> Vec<Mnemon> {
    vec![
        Mnemon {
            id: 1,
            title: "Spirited Away".to_string(),
            year: Some(2001),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/39wmItIWsg5sZMyRUHLkWBcuVCM.jpg".to_string(),
            ),
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
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co3p2d.png".to_string(),
            ),
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
            year: Some(1998),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/gZFHBd677gz8V5fyj8SZx5SrqTA.jpg".to_string(),
            ),
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
            year: Some(2014),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/nX5XotM9yprCKarRH4fzOq1VM1J.jpg".to_string(),
            ),
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
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png".to_string(),
            ),
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
            year: Some(2016),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/q719jXXEzOoYaps6babgKnONONX.jpg".to_string(),
            ),
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
    // All mnemons - start with empty to show empty state
    let mut mnemons = use_signal(|| Vec::<Mnemon>::new());

    // Current mnemon index
    let mut current_index = use_signal(|| 0);
    let mut is_transitioning = use_signal(|| false);

    // Add flow state
    let mut show_add_flow = use_signal(|| false);

    let current_mnemon = use_memo(move || {
        let all = mnemons();
        if all.is_empty() {
            return None;
        }
        let idx = current_index();
        all.get(idx).cloned()
    });

    // Auto-advance to next mnemon after 10 seconds (continuous cycle)
    // This effect only runs once on mount to avoid creating multiple timers
    use_effect(move || {
        spawn(async move {
            loop {
                gloo_timers::future::TimeoutFuture::new(10_000).await;

                // Check if we have mnemons before cycling
                let total = mnemons.read().len();
                if total == 0 {
                    continue;
                }

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
    });

    let has_mnemons = !mnemons().is_empty();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            class: "h-screen w-screen overflow-hidden bg-gray-900",

            // Hero with current mnemon or empty state
            if has_mnemons {
                if let Some(mnemon) = current_mnemon() {
                    Hero {
                        mnemon: mnemon.clone(),
                        is_transitioning: is_transitioning(),
                        on_click: move |_| {
                            show_add_flow.set(true);
                        }
                    }
                }
            } else {
                EmptyState {
                    on_click: move |_| {
                        show_add_flow.set(true);
                    }
                }
            }

            // Add mnemon flow (modal overlay)
            if show_add_flow() {
                AddMnemonFlow {
                    on_save: move |form: AddMnemonForm| {
                        let mut all_mnemons = mnemons.write();
                        let new_id = all_mnemons.len() + 1;

                        // Parse year
                        let year = form.year.trim().parse::<u16>().ok();

                        // Split notes by newlines
                        let notes: Vec<String> = form.notes
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();

                        let new_mnemon = Mnemon {
                            id: new_id,
                            title: form.title.clone(),
                            year,
                            work_type: form.work_type.unwrap(),
                            cover_url: None, // Manual entries have no cover
                            feelings: form.feelings.clone(),
                            finished_date: if form.finished_date.is_empty() {
                                None
                            } else {
                                Some(form.finished_date.clone())
                            },
                            notes,
                        };

                        all_mnemons.push(new_mnemon);

                        // Set current index to the new mnemon
                        current_index.set(all_mnemons.len() - 1);

                        show_add_flow.set(false);
                    },
                    on_cancel: move |_| {
                        show_add_flow.set(false);
                    }
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
fn Hero(mnemon: ReadSignal<Mnemon>, is_transitioning: bool, on_click: EventHandler<()>) -> Element {
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
            class: "relative h-full w-full cursor-pointer",
            style: "{transition_style}",
            onclick: move |_| on_click.call(()),

            // Background cover image with overlay
            div {
                class: "absolute inset-0 z-0",
                style: if let Some(ref url) = mnemon().cover_url {
                    format!("background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat;", url)
                } else {
                    "background-color: #1a1a2e;".to_string()
                },

                // Dark overlay for readability
                div {
                    class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/70 to-black/50"
                }
            }

            // Note display - lower left
            if let Some(note) = current_note() {
                div {
                    class: "absolute left-4 top-2/3 z-10 max-w-lg max-w-80 transition-opacity duration-500",
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

#[component]
fn EmptyState(on_click: EventHandler<()>) -> Element {
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

#[component]
fn AddMnemonFlow(on_save: EventHandler<AddMnemonForm>, on_cancel: EventHandler<()>) -> Element {
    let mut form = use_signal(|| AddMnemonForm::default());
    let mut current_step = use_signal(|| 1);

    rsx! {
        // Modal overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm",
            onclick: move |_| on_cancel.call(()),

            // Modal content
            div {
                class: "bg-gray-800 rounded-lg shadow-2xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                onclick: move |e| e.stop_propagation(),

                if current_step() == 1 {
                    Step1ManualEntry {
                        form: form(),
                        on_next: move |updated_form| {
                            form.set(updated_form);
                            current_step.set(2);
                        },
                        on_cancel: move |_| on_cancel.call(())
                    }
                } else {
                    Step2Personalize {
                        form: form(),
                        on_save: move |updated_form| {
                            on_save.call(updated_form);
                        },
                        on_back: move |updated_form| {
                            form.set(updated_form);
                            current_step.set(1);
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Step1ManualEntry(
    form: AddMnemonForm,
    on_next: EventHandler<AddMnemonForm>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut local_form = use_signal(|| form);

    let is_valid = local_form().is_step1_valid();

    rsx! {
        div {
            class: "p-8",

            // Header
            div {
                class: "mb-6",
                h2 {
                    class: "text-3xl font-bold text-white mb-2",
                    "Add a mnemon"
                }
                p {
                    class: "text-gray-400",
                    "Step 1: Pick the Work"
                }
            }

            // Type selection
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-3",
                    "Type"
                    span { class: "text-red-400 ml-1", "*" }
                }
                div {
                    class: "flex gap-3",
                    for work_type in [WorkType::Movie, WorkType::TvAnime, WorkType::Game] {
                        button {
                            class: if local_form().work_type == Some(work_type.clone()) {
                                "flex items-center gap-2 px-4 py-3 bg-blue-600 text-white rounded-lg border-2 border-blue-500 font-medium"
                            } else {
                                "flex items-center gap-2 px-4 py-3 bg-gray-700 text-gray-300 rounded-lg border-2 border-gray-600 hover:border-gray-500 font-medium"
                            },
                            onclick: move |_| {
                                local_form.with_mut(|f| f.work_type = Some(work_type.clone()));
                            },
                            span { class: "text-xl", "{work_type.icon()}" }
                            span { "{work_type.label()}" }
                        }
                    }
                }
            }

            // Title input
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Title (English)"
                    span { class: "text-red-400 ml-1", "*" }
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "text",
                    placeholder: "Enter the title...",
                    value: "{local_form().title}",
                    oninput: move |e| {
                        local_form.with_mut(|f| f.title = e.value());
                    }
                }
            }

            // Year input
            div {
                class: "mb-8",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Release Year"
                    span { class: "text-gray-400 ml-1 text-xs", "(optional)" }
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "text",
                    placeholder: "YYYY",
                    value: "{local_form().year}",
                    maxlength: 4,
                    oninput: move |e| {
                        let value = e.value();
                        // Only allow digits
                        if value.chars().all(|c| c.is_ascii_digit()) {
                            local_form.with_mut(|f| f.year = value);
                        }
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-3 justify-end",
                button {
                    class: "px-6 py-3 text-gray-300 hover:text-white font-medium",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
                button {
                    class: if is_valid {
                        "px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium"
                    } else {
                        "px-6 py-3 bg-gray-600 text-gray-400 rounded-lg font-medium cursor-not-allowed"
                    },
                    disabled: !is_valid,
                    onclick: move |_| {
                        if is_valid {
                            on_next.call(local_form());
                        }
                    },
                    "Next →"
                }
            }
        }
    }
}

#[component]
fn Step2Personalize(
    form: AddMnemonForm,
    on_save: EventHandler<AddMnemonForm>,
    on_back: EventHandler<AddMnemonForm>,
) -> Element {
    let mut local_form = use_signal(|| form);

    rsx! {
        div {
            class: "p-8",

            // Header
            div {
                class: "mb-6",
                h2 {
                    class: "text-3xl font-bold text-white mb-2",
                    "Add a mnemon"
                }
                p {
                    class: "text-gray-400",
                    "Step 2: Personalize (all optional)"
                }
            }

            // Finished date
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Finished date"
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "date",
                    value: "{local_form().finished_date}",
                    oninput: move |e| {
                        local_form.with_mut(|f| f.finished_date = e.value());
                    }
                }
            }

            // Feelings
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-3",
                    "Feelings"
                    span { class: "text-gray-400 ml-2 text-xs", "(choose up to 5)" }
                }
                div {
                    class: "flex flex-wrap gap-2",
                    for feeling in FEELINGS {
                        {
                            let is_selected = local_form().feelings.contains(&feeling.to_string());
                            let feelings_count = local_form().feelings.len();
                            let can_add = feelings_count < 5;

                            rsx! {
                                button {
                                    class: if is_selected {
                                        "px-4 py-2 bg-blue-600 text-white rounded-full border-2 border-blue-500 text-sm font-medium"
                                    } else if can_add {
                                        "px-4 py-2 bg-gray-700 text-gray-300 rounded-full border-2 border-gray-600 hover:border-gray-500 text-sm font-medium"
                                    } else {
                                        "px-4 py-2 bg-gray-800 text-gray-500 rounded-full border-2 border-gray-700 text-sm font-medium cursor-not-allowed"
                                    },
                                    disabled: !is_selected && !can_add,
                                    onclick: move |_| {
                                        local_form.with_mut(|f| {
                                            if is_selected {
                                                f.feelings.retain(|s| s != feeling);
                                            } else if can_add {
                                                f.feelings.push(feeling.to_string());
                                            }
                                        });
                                    },
                                    "{feeling}"
                                }
                            }
                        }
                    }
                }
            }

            // Notes
            div {
                class: "mb-8",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Notes"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none min-h-[200px] resize-y",
                    placeholder: "Add your thoughts, memories, or reflections...",
                    value: "{local_form().notes}",
                    oninput: move |e| {
                        local_form.with_mut(|f| f.notes = e.value());
                    }
                }
                p {
                    class: "text-gray-500 text-xs mt-2",
                    "Plain text for now. Rich text formatting coming soon."
                }
            }

            // Actions
            div {
                class: "flex gap-3 justify-between",
                button {
                    class: "px-6 py-3 text-gray-300 hover:text-white font-medium",
                    onclick: move |_| on_back.call(local_form()),
                    "← Back"
                }
                button {
                    class: "px-6 py-3 bg-green-600 hover:bg-green-700 text-white rounded-lg font-medium",
                    onclick: move |_| {
                        on_save.call(local_form());
                    },
                    "Save Mnemon"
                }
            }
        }
    }
}
