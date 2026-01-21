use super::ProviderRef;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum WorkType {
    Movie,
    TvAnime,
    Game,
}

impl WorkType {
    pub fn icon(&self) -> &'static str {
        match self {
            WorkType::Movie => "🎬",
            WorkType::TvAnime => "📺",
            WorkType::Game => "🎮",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WorkType::Movie => "Movie",
            WorkType::TvAnime => "TV/Anime",
            WorkType::Game => "Game",
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum WorkOrigin {
    Provider,
    Manual,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Work {
    pub id: Uuid,

    pub work_type: WorkType,

    pub title_en: String,

    pub release_year: Option<u16>,

    pub cover_image_local_uri: Option<String>,

    pub theme_music_local_uri: Option<String>,

    pub provider_ref: Option<ProviderRef>,

    pub origin: WorkOrigin,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Work {
    pub fn from_provider(
        work_type: WorkType,
        title: String,
        release_year: Option<u16>,
        cover_url: Option<String>,
        theme_music_url: Option<String>,
        provider_ref: ProviderRef,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            work_type,
            title_en: title,
            release_year,
            cover_image_local_uri: cover_url,
            theme_music_local_uri: theme_music_url,
            provider_ref: Some(provider_ref),
            origin: WorkOrigin::Provider,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn from_manual(work_type: WorkType, title: String, release_year: Option<u16>) -> Self {
        Self {
            id: Uuid::new_v4(),
            work_type,
            title_en: title,
            release_year,
            cover_image_local_uri: None,
            theme_music_local_uri: None,
            provider_ref: None,
            origin: WorkOrigin::Manual,
            created_at: chrono::Utc::now(),
        }
    }
}
