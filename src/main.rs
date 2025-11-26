use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

// Fixed feelings taxonomy with emojis
const FEELINGS: &[(&str, &str)] = &[
    ("Nostalgic", "🌅"),
    ("Cozy", "☕"),
    ("Melancholic", "🌧️"),
    ("Epic", "⚔️"),
    ("Wholesome", "💚"),
    ("Bittersweet", "🍃"),
    ("Heartwarming", "💝"),
    ("Chill", "😎"),
    ("Adventurous", "🗺️"),
    ("Uplifting", "🎈"),
    ("Mysterious", "🔮"),
    ("Somber", "🌑"),
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
    provider_ref: Option<ProviderRef>,
    feelings: Vec<String>,
    finished_date: Option<String>,
    notes: Vec<String>,
}

// Provider reference
#[derive(Clone, PartialEq, Debug)]
struct ProviderRef {
    provider_source: String,
    provider_id: String,
}

// Search result from provider
#[derive(Clone, PartialEq, Debug)]
struct SearchResult {
    provider_ref: ProviderRef,
    title: String,
    year: Option<u16>,
    work_type: WorkType,
    cover_url: Option<String>,
    theme_music_url: Option<String>,
}

// Fixture search data (simulating a large dataset)
fn get_fixture_search_results() -> Vec<SearchResult> {
    vec![
        // Movies
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "129".to_string(),
            },
            title: "Spirited Away".to_string(),
            year: Some(2001),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/39wmItIWsg5sZMyRUHLkWBcuVCM.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "146216".to_string(),
            },
            title: "Your Name".to_string(),
            year: Some(2016),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/q719jXXEzOoYaps6babgKnONONX.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "120467".to_string(),
            },
            title: "The Grand Budapest Hotel".to_string(),
            year: Some(2014),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/nX5XotM9yprCKarRH4fzOq1VM1J.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "550".to_string(),
            },
            title: "Fight Club".to_string(),
            year: Some(1999),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "13".to_string(),
            },
            title: "Forrest Gump".to_string(),
            year: Some(1994),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/arw2vcBveWOVZr6pxd9XTd1TdQa.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "278".to_string(),
            },
            title: "The Shawshank Redemption".to_string(),
            year: Some(1994),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/q6y0Go1tsGEsmtFryDOJo3dEmqu.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "238".to_string(),
            },
            title: "The Godfather".to_string(),
            year: Some(1972),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/3bhkrj58Vtu7enYsRolD1fZdja1.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "424".to_string(),
            },
            title: "Schindler's List".to_string(),
            year: Some(1993),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/sF1U4EUQS8YHUYjNl3pMGNIQyr0.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "389".to_string(),
            },
            title: "12 Angry Men".to_string(),
            year: Some(1957),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/ow3wq89wM8qd5X7hWKxiRfsFf9C.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "155".to_string(),
            },
            title: "The Dark Knight".to_string(),
            year: Some(2008),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/qJ2tW6WMUDux911r6m7haRef0WH.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "497".to_string(),
            },
            title: "The Green Mile".to_string(),
            year: Some(1999),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/velWPhVMQeQKcxggNEU8YmIo52R.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "680".to_string(),
            },
            title: "Pulp Fiction".to_string(),
            year: Some(1994),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/d5iIlFn5s0ImszYzBPb8JPIfbXD.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "769".to_string(),
            },
            title: "GoodFellas".to_string(),
            year: Some(1990),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/aKuFiU82s5ISJpGZp7YkIr3kCUd.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "27205".to_string(),
            },
            title: "Inception".to_string(),
            year: Some(2010),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/9gk7adHYeDvHkCSEqAvQNLV5Uge.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "98".to_string(),
            },
            title: "Gladiator".to_string(),
            year: Some(2000),
            work_type: WorkType::Movie,
            cover_url: Some("https://image.tmdb.org/t/p/w500/ty8TGRuvJLPUmAR1H1nRIsgwvim.jpg".to_string()),
            theme_music_url: None,
        },
        // TV/Anime
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "1355".to_string(),
            },
            title: "Cowboy Bebop".to_string(),
            year: Some(1998),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://image.tmdb.org/t/p/w500/gZFHBd677gz8V5fyj8SZx5SrqTA.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "11061".to_string(),
            },
            title: "Hunter x Hunter".to_string(),
            year: Some(2011),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx11061-sIpercRKikfh.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "5114".to_string(),
            },
            title: "Fullmetal Alchemist: Brotherhood".to_string(),
            year: Some(2009),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx5114-qg9GGO3c8zqF.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "21".to_string(),
            },
            title: "One Piece".to_string(),
            year: Some(1999),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx21-YCDoj1EkAxFn.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "1535".to_string(),
            },
            title: "Death Note".to_string(),
            year: Some(2006),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx1535-4r88a1tsBEIz.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "16498".to_string(),
            },
            title: "Attack on Titan".to_string(),
            year: Some(2013),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx16498-C6FPmWm59CyP.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "11757".to_string(),
            },
            title: "Sword Art Online".to_string(),
            year: Some(2012),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx11757-QlamRgbmYlbv.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "20958".to_string(),
            },
            title: "My Hero Academia".to_string(),
            year: Some(2016),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx20958-UMb6Cr4l8YJ8.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "9253".to_string(),
            },
            title: "Steins;Gate".to_string(),
            year: Some(2011),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx9253-7pdcVzQSkKxT.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "anilist".to_string(),
                provider_id: "101922".to_string(),
            },
            title: "Demon Slayer".to_string(),
            year: Some(2019),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx101922-PEn1CTc93blC.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "1396".to_string(),
            },
            title: "Breaking Bad".to_string(),
            year: Some(2008),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://image.tmdb.org/t/p/w500/ggFHVNu6YYI5L9pCfOacjizRGt.jpg".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "tmdb".to_string(),
                provider_id: "1399".to_string(),
            },
            title: "Game of Thrones".to_string(),
            year: Some(2011),
            work_type: WorkType::TvAnime,
            cover_url: Some("https://image.tmdb.org/t/p/w500/1XS1oqL89opfnbLl8WnZY1O1uJx.jpg".to_string()),
            theme_music_url: None,
        },
        // Games
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "7346".to_string(),
            },
            title: "The Legend of Zelda: Breath of the Wild".to_string(),
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co3p2d.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "26844".to_string(),
            },
            title: "Hollow Knight".to_string(),
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "1942".to_string(),
            },
            title: "The Witcher 3: Wild Hunt".to_string(),
            year: Some(2015),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co1wyy.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "1020".to_string(),
            },
            title: "Grand Theft Auto V".to_string(),
            year: Some(2013),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co2lbd.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "1074".to_string(),
            },
            title: "Red Dead Redemption 2".to_string(),
            year: Some(2018),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co1q1f.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "11208".to_string(),
            },
            title: "Elden Ring".to_string(),
            year: Some(2022),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co4jni.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "119171".to_string(),
            },
            title: "Baldur's Gate 3".to_string(),
            year: Some(2023),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co5s5v.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "1877".to_string(),
            },
            title: "Cyberpunk 2077".to_string(),
            year: Some(2020),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co2vt0.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "11156".to_string(),
            },
            title: "Sekiro: Shadows Die Twice".to_string(),
            year: Some(2019),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co1ixg.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "113285".to_string(),
            },
            title: "Hades".to_string(),
            year: Some(2020),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co2i0u.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "26192".to_string(),
            },
            title: "Celeste".to_string(),
            year: Some(2018),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co1tnq.png".to_string()),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef {
                provider_source: "igdb".to_string(),
                provider_id: "25076".to_string(),
            },
            title: "Stardew Valley".to_string(),
            year: Some(2016),
            work_type: WorkType::Game,
            cover_url: Some("https://images.igdb.com/igdb/image/upload/t_cover_big/co5qkw.png".to_string()),
            theme_music_url: None,
        },
    ]
}

// Search results with pagination
#[allow(dead_code)]
struct SearchResultsPage {
    results: Vec<SearchResult>,
    total_count: usize,
    page: usize,
    total_pages: usize,
}

// Extracted search function (will be replaced with API call in the future)
fn search_works(query: &str, work_type: WorkType, page: usize) -> SearchResultsPage {
    let page_size = 10;
    let fixtures = get_fixture_search_results();

    // Filter by work type and query
    let filtered: Vec<SearchResult> = fixtures
        .into_iter()
        .filter(|r| {
            if r.work_type != work_type {
                return false;
            }
            if query.is_empty() {
                true
            } else {
                r.title.to_lowercase().contains(&query.to_lowercase())
            }
        })
        .collect();

    let total_count = filtered.len();
    let total_pages = (total_count as f32 / page_size as f32).ceil() as usize;
    let start_idx = page * page_size;
    let end_idx = (start_idx + page_size).min(total_count);

    let results = if start_idx < total_count {
        filtered[start_idx..end_idx].to_vec()
    } else {
        Vec::new()
    };

    SearchResultsPage {
        results,
        total_count,
        page,
        total_pages,
    }
}

// Form data for adding a mnemon
#[derive(Clone, PartialEq, Debug, Default)]
struct AddMnemonForm {
    // Step 1 fields
    work_type: Option<WorkType>,
    title: String,
    year: String,

    // Provider fields (if selected from search)
    provider_ref: Option<ProviderRef>,
    cover_url: Option<String>,
    theme_music_url: Option<String>,

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
            provider_ref: None,
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
            provider_ref: None,
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
            provider_ref: None,
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
            provider_ref: None,
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
            provider_ref: None,
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
            provider_ref: None,
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

    // Provide mnemons to child components
    use_context_provider(|| mnemons);

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
                            cover_url: form.cover_url.clone(),
                            provider_ref: form.provider_ref.clone(),
                            feelings: form.feelings.clone(),
                            finished_date: if form.finished_date.is_empty() {
                                None
                            } else {
                                Some(form.finished_date.clone())
                            },
                            notes,
                        };

                        all_mnemons.push(new_mnemon);
                        let new_index = all_mnemons.len() - 1;
                        drop(all_mnemons);

                        // Set current index to the new mnemon
                        current_index.set(new_index);

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
    let mut search_results = use_signal(|| Vec::<SearchResult>::new());
    let mut show_results = use_signal(|| false);
    let mut existing_work_error = use_signal(|| false);
    let mnemons = use_context::<Signal<Vec<Mnemon>>>();

    let is_valid = local_form().is_step1_valid() && !existing_work_error();
    let mut current_page = use_signal(|| 0usize);

    // Search function (will call API in the future)
    let mut perform_search =
        move |query: String, work_type: Option<WorkType>, force: bool, page: usize| {
            if let Some(wt) = work_type {
                if force || query.len() >= 3 || query.is_empty() {
                    let results_page = search_works(&query, wt, page);
                    search_results.set(results_page.results);
                    show_results.set(true);
                } else {
                    search_results.set(Vec::new());
                    show_results.set(false);
                }
            }
        };

    // Check if provider ref already exists
    let check_existing_work = move |provider_ref: &ProviderRef| -> bool {
        mnemons.read().iter().any(|m| {
            if let Some(ref pr) = m.provider_ref {
                pr.provider_source == provider_ref.provider_source
                    && pr.provider_id == provider_ref.provider_id
            } else {
                false
            }
        })
    };

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
                                local_form.with_mut(|f| {
                                    f.work_type = Some(work_type.clone());
                                    // Clear provider data when changing type
                                    f.provider_ref = None;
                                    f.cover_url = None;
                                    f.theme_music_url = None;
                                });
                                existing_work_error.set(false);
                                current_page.set(0);
                                // Don't trigger search on type selection
                            },
                            span { class: "text-xl", "{work_type.icon()}" }
                            span { "{work_type.label()}" }
                        }
                    }
                }
            }

            // Title input with search
            div {
                class: "mb-6 relative",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Title (English)"
                    span { class: "text-red-400 ml-1", "*" }
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "text",
                    placeholder: if local_form().work_type.is_some() {
                        "Search or enter title..."
                    } else {
                        "Select a type first..."
                    },
                    value: "{local_form().title}",
                    disabled: local_form().work_type.is_none(),
                    onfocus: move |_| {
                        // Trigger search when field is focused
                        if local_form().work_type.is_some() {
                            current_page.set(0);
                            perform_search(local_form().title.clone(), local_form().work_type.clone(), false, 0);
                        }
                    },
                    oninput: move |e| {
                        let value = e.value();
                        local_form.with_mut(|f| {
                            f.title = value.clone();
                            // Clear provider data when typing
                            f.provider_ref = None;
                            f.cover_url = None;
                            f.theme_music_url = None;
                            f.year = String::new();
                        });
                        existing_work_error.set(false);
                        current_page.set(0);
                        perform_search(value, local_form().work_type.clone(), false, 0);
                    },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            // Force search on Enter regardless of length
                            current_page.set(0);
                            perform_search(local_form().title.clone(), local_form().work_type.clone(), true, 0);
                        }
                    }
                }

                // Search results dropdown
                if show_results() && !search_results().is_empty() {
                    div {
                        class: "absolute z-10 w-full mt-1 bg-gray-700 border-2 border-gray-600 rounded-lg shadow-lg max-h-64 overflow-y-auto",
                        for result in search_results().iter() {
                            button {
                                class: "w-full px-4 py-3 flex items-center gap-3 hover:bg-gray-600 border-b border-gray-600 last:border-b-0 text-left",
                                onclick: {
                                    let result_clone = result.clone();
                                    move |_| {
                                        // Check if work already exists
                                        if check_existing_work(&result_clone.provider_ref) {
                                            existing_work_error.set(true);
                                            show_results.set(false);
                                        } else {
                                            // Autofill form with provider result
                                            local_form.with_mut(|f| {
                                                f.title = result_clone.title.clone();
                                                f.year = result_clone.year.map(|y| y.to_string()).unwrap_or_default();
                                                f.provider_ref = Some(result_clone.provider_ref.clone());
                                                f.cover_url = result_clone.cover_url.clone();
                                                f.theme_music_url = result_clone.theme_music_url.clone();
                                            });
                                            existing_work_error.set(false);
                                            show_results.set(false);
                                        }
                                    }
                                },

                                // Cover thumbnail
                                if let Some(ref cover) = result.cover_url {
                                    img {
                                        class: "w-12 h-16 object-cover rounded",
                                        src: "{cover}",
                                        alt: "{result.title}"
                                    }
                                } else {
                                    div {
                                        class: "w-12 h-16 bg-gray-800 rounded flex items-center justify-center",
                                        span { class: "text-2xl", "{result.work_type.icon()}" }
                                    }
                                }

                                div {
                                    class: "flex-1",
                                    div {
                                        class: "text-white font-medium",
                                        "{result.title}"
                                    }
                                    if let Some(year) = result.year {
                                        div {
                                            class: "text-gray-400 text-sm",
                                            "{year}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Existing work error message
                if existing_work_error() {
                    div {
                        class: "mt-2 px-3 py-2 bg-red-900/50 border border-red-700 rounded text-red-200 text-sm",
                        "⚠️ This work already exists in your collection. Please search for a different title."
                    }
                }
            }

            // Notes
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Notes"
                }
                textarea {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none min-h-[120px] resize-y",
                    placeholder: "Add your thoughts, memories, or reflections...",
                    value: "{local_form().notes}",
                    oninput: move |e| {
                        local_form.with_mut(|f| f.notes = e.value());
                    }
                }
            }

            // Feelings
            div {
                class: "mb-8",
                label {
                    class: "block text-white text-sm font-semibold mb-3",
                    "Feelings"
                    span { class: "text-gray-400 ml-2 text-xs", "(choose up to 5)" }
                }
                div {
                    class: "flex flex-wrap gap-2",
                    for (feeling_name, feeling_emoji) in FEELINGS {
                        {
                            let is_selected = local_form().feelings.contains(&feeling_name.to_string());
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
                    "Step 2: Optional dates"
                }
            }

            // Release Year (autofilled from search or manual)
            div {
                class: "mb-6",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Release Year"
                }
                input {
                    class: "w-full px-4 py-3 bg-gray-700 text-white rounded-lg border-2 border-gray-600 focus:border-blue-500 focus:outline-none",
                    r#type: "text",
                    placeholder: "YYYY",
                    value: "{local_form().year}",
                    readonly: local_form().provider_ref.is_some(),
                    maxlength: 4,
                    oninput: move |e| {
                        let value = e.value();
                        // Only allow digits
                        if value.chars().all(|c| c.is_ascii_digit()) {
                            local_form.with_mut(|f| f.year = value);
                        }
                    }
                }
                if local_form().provider_ref.is_some() {
                    p {
                        class: "text-gray-500 text-xs mt-1",
                        "Autofilled from search result"
                    }
                }
            }

            // Finished date (when you finished/completed it)
            div {
                class: "mb-8",
                label {
                    class: "block text-white text-sm font-semibold mb-2",
                    "Finished date"
                    span { class: "text-gray-400 ml-1 text-xs", "(when you completed it)" }
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
