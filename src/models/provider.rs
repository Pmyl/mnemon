//! Provider reference model

/// Reference to a work in an external provider (e.g., TMDB, AniList, IGDB)
///
/// Used for exact-ID deduplication: if two works have the same provider_source
/// and provider_id, they are considered the same work.
#[derive(Clone, PartialEq, Debug)]
pub struct ProviderRef {
    /// The source provider (e.g., "tmdb", "anilist", "igdb")
    pub provider_source: String,

    /// The unique identifier within that provider
    pub provider_id: String,
}

impl ProviderRef {
    /// Creates a new ProviderRef
    pub fn new(provider_source: impl Into<String>, provider_id: impl Into<String>) -> Self {
        Self {
            provider_source: provider_source.into(),
            provider_id: provider_id.into(),
        }
    }

    /// Checks if this provider ref matches another (same source and id)
    pub fn matches(&self, other: &ProviderRef) -> bool {
        self.provider_source == other.provider_source && self.provider_id == other.provider_id
    }
}
