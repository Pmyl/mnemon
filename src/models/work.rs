//! Work model - represents a piece of media (Movie, TV/Anime, or Game)

use super::ProviderRef;
use uuid::Uuid;

/// The type of media work
#[derive(Clone, PartialEq, Debug)]
pub enum WorkType {
    Movie,
    TvAnime,
    Game,
}

impl WorkType {
    /// Returns the emoji icon for this work type
    pub fn icon(&self) -> &'static str {
        match self {
            WorkType::Movie => "🎬",
            WorkType::TvAnime => "📺",
            WorkType::Game => "🎮",
        }
    }

    /// Returns the human-readable label for this work type
    pub fn label(&self) -> &'static str {
        match self {
            WorkType::Movie => "Movie",
            WorkType::TvAnime => "TV/Anime",
            WorkType::Game => "Game",
        }
    }
}

/// Origin of the work data
#[derive(Clone, PartialEq, Debug)]
pub enum WorkOrigin {
    /// Work was created from a provider search result
    Provider,
    /// Work was manually entered by the user
    Manual,
}

/// A piece of media (Movie, TV/Anime, or Game)
///
/// Works are the media entities that Mnemons reference. A Work can be shared
/// across multiple Mnemons (e.g., if you watch the same movie twice).
#[derive(Clone, PartialEq, Debug)]
pub struct Work {
    /// Unique identifier (UUID)
    pub id: Uuid,

    /// Type of media
    pub work_type: WorkType,

    /// Title in English (required)
    pub title_en: String,

    /// Release year (optional)
    pub release_year: Option<u16>,

    /// Local URI/path to cached cover image (optional)
    pub cover_image_local_uri: Option<String>,

    /// Local URI/path to cached theme music (optional)
    pub theme_music_local_uri: Option<String>,

    /// Provider reference if this work came from a provider search
    pub provider_ref: Option<ProviderRef>,

    /// Origin of this work (provider or manual entry)
    pub origin: WorkOrigin,

    /// Timestamp when this work was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Work {
    /// Creates a new Work from a provider search result
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
            cover_image_local_uri: cover_url, // For now, using URL directly; will be local path after caching
            theme_music_local_uri: theme_music_url,
            provider_ref: Some(provider_ref),
            origin: WorkOrigin::Provider,
            created_at: chrono::Utc::now(),
        }
    }

    /// Creates a new Work from manual entry
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
