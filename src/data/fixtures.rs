//! Fixture data for development and testing
//!
//! This module contains hardcoded search results that simulate provider API responses.
//! Games still use fixtures until IGDB integration is complete.
//! Movies and TV/Anime now use TMDB API.

#![allow(dead_code)]

use crate::models::{ProviderRef, SearchResult, WorkType};

/// Returns all fixture search results (legacy, kept for backward compatibility)
///
/// Contains titles across Movies, TV/Anime, and Games with realistic
/// provider metadata (TMDB, AniList, IGDB).
pub fn get_fixture_search_results() -> Vec<SearchResult> {
    vec![
        // =============================================================================
        // MOVIES (TMDB)
        // =============================================================================
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "129"),
            title: "Spirited Away".to_string(),
            year: Some(2001),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/39wmItIWsg5sZMyRUHLkWBcuVCM.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "146216"),
            title: "Your Name".to_string(),
            year: Some(2016),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/q719jXXEzOoYaps6babgKnONONX.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "120467"),
            title: "The Grand Budapest Hotel".to_string(),
            year: Some(2014),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/nX5XotM9yprCKarRH4fzOq1VM1J.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "550"),
            title: "Fight Club".to_string(),
            year: Some(1999),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "13"),
            title: "Forrest Gump".to_string(),
            year: Some(1994),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/arw2vcBveWOVZr6pxd9XTd1TdQa.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "278"),
            title: "The Shawshank Redemption".to_string(),
            year: Some(1994),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/q6y0Go1tsGEsmtFryDOJo3dEmqu.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "238"),
            title: "The Godfather".to_string(),
            year: Some(1972),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/3bhkrj58Vtu7enYsRolD1fZdja1.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "424"),
            title: "Schindler's List".to_string(),
            year: Some(1993),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/sF1U4EUQS8YHUYjNl3pMGNIQyr0.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "389"),
            title: "12 Angry Men".to_string(),
            year: Some(1957),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/ow3wq89wM8qd5X7hWKxiRfsFf9C.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "155"),
            title: "The Dark Knight".to_string(),
            year: Some(2008),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/qJ2tW6WMUDux911r6m7haRef0WH.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "497"),
            title: "The Green Mile".to_string(),
            year: Some(1999),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/velWPhVMQeQKcxggNEU8YmIo52R.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "680"),
            title: "Pulp Fiction".to_string(),
            year: Some(1994),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/d5iIlFn5s0ImszYzBPb8JPIfbXD.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "769"),
            title: "GoodFellas".to_string(),
            year: Some(1990),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/aKuFiU82s5ISJpGZp7YkIr3kCUd.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "27205"),
            title: "Inception".to_string(),
            year: Some(2010),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/9gk7adHYeDvHkCSEqAvQNLV5Uge.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "98"),
            title: "Gladiator".to_string(),
            year: Some(2000),
            work_type: WorkType::Movie,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/ty8TGRuvJLPUmAR1H1nRIsgwvim.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        // =============================================================================
        // TV/ANIME (TMDB + AniList)
        // =============================================================================
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "1355"),
            title: "Cowboy Bebop".to_string(),
            year: Some(1998),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/gZFHBd677gz8V5fyj8SZx5SrqTA.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "11061"),
            title: "Hunter x Hunter".to_string(),
            year: Some(2011),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx11061-sIpercRKikfh.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "5114"),
            title: "Fullmetal Alchemist: Brotherhood".to_string(),
            year: Some(2009),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx5114-qg9GGO3c8zqF.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "21"),
            title: "One Piece".to_string(),
            year: Some(1999),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx21-YCDoj1EkAxFn.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "1535"),
            title: "Death Note".to_string(),
            year: Some(2006),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx1535-4r88a1tsBEIz.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "16498"),
            title: "Attack on Titan".to_string(),
            year: Some(2013),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx16498-C6FPmWm59CyP.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "11757"),
            title: "Sword Art Online".to_string(),
            year: Some(2012),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx11757-QlamRgbmYlbv.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "20958"),
            title: "My Hero Academia".to_string(),
            year: Some(2016),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx20958-UMb6Cr4l8YJ8.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "9253"),
            title: "Steins;Gate".to_string(),
            year: Some(2011),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx9253-7pdcVzQSkKxT.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("anilist", "101922"),
            title: "Demon Slayer".to_string(),
            year: Some(2019),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx101922-PEn1CTc93blC.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "1396"),
            title: "Breaking Bad".to_string(),
            year: Some(2008),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/ggFHVNu6YYI5L9pCfOacjizRGt.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("tmdb", "1399"),
            title: "Game of Thrones".to_string(),
            year: Some(2011),
            work_type: WorkType::TvAnime,
            cover_url: Some(
                "https://image.tmdb.org/t/p/w500/1XS1oqL89opfnbLl8WnZY1O1uJx.jpg".to_string(),
            ),
            theme_music_url: None,
        },
        // =============================================================================
        // GAMES (IGDB)
        // =============================================================================
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "7346"),
            title: "The Legend of Zelda: Breath of the Wild".to_string(),
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co3p2d.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "26844"),
            title: "Hollow Knight".to_string(),
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1942"),
            title: "The Witcher 3: Wild Hunt".to_string(),
            year: Some(2015),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1wyy.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1020"),
            title: "Grand Theft Auto V".to_string(),
            year: Some(2013),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2lbd.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1074"),
            title: "Red Dead Redemption 2".to_string(),
            year: Some(2018),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1q1f.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "11208"),
            title: "Elden Ring".to_string(),
            year: Some(2022),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co4jni.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "119171"),
            title: "Baldur's Gate 3".to_string(),
            year: Some(2023),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co5s5v.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1877"),
            title: "Cyberpunk 2077".to_string(),
            year: Some(2020),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2vt0.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "11156"),
            title: "Sekiro: Shadows Die Twice".to_string(),
            year: Some(2019),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1ixg.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "113285"),
            title: "Hades".to_string(),
            year: Some(2020),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2i0u.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "26192"),
            title: "Celeste".to_string(),
            year: Some(2018),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1tnq.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "25076"),
            title: "Stardew Valley".to_string(),
            year: Some(2016),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co5qkw.png".to_string(),
            ),
            theme_music_url: None,
        },
    ]
}

/// Returns only game fixture data
///
/// Used for Games search until IGDB integration is complete.
/// Movies and TV/Anime use TMDB API instead.
pub fn get_game_fixtures() -> Vec<SearchResult> {
    vec![
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "7346"),
            title: "The Legend of Zelda: Breath of the Wild".to_string(),
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co3p2d.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "26844"),
            title: "Hollow Knight".to_string(),
            year: Some(2017),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1rgi.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1942"),
            title: "The Witcher 3: Wild Hunt".to_string(),
            year: Some(2015),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1wyy.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1020"),
            title: "Grand Theft Auto V".to_string(),
            year: Some(2013),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2lbd.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1074"),
            title: "Red Dead Redemption 2".to_string(),
            year: Some(2018),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1q1f.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "11208"),
            title: "Elden Ring".to_string(),
            year: Some(2022),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co4jni.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "119171"),
            title: "Baldur's Gate 3".to_string(),
            year: Some(2023),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co5s5v.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "1877"),
            title: "Cyberpunk 2077".to_string(),
            year: Some(2020),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2vt0.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "11156"),
            title: "Sekiro: Shadows Die Twice".to_string(),
            year: Some(2019),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1ixg.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "113285"),
            title: "Hades".to_string(),
            year: Some(2020),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co2i0u.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "26192"),
            title: "Celeste".to_string(),
            year: Some(2018),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co1tnq.png".to_string(),
            ),
            theme_music_url: None,
        },
        SearchResult {
            provider_ref: ProviderRef::new("igdb", "25076"),
            title: "Stardew Valley".to_string(),
            year: Some(2016),
            work_type: WorkType::Game,
            cover_url: Some(
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co5qkw.png".to_string(),
            ),
            theme_music_url: None,
        },
    ]
}
