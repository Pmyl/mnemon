//! TMDB (The Movie Database) API client
//!
//! Provides search functionality for Movies and TV shows.
//! API Documentation: https://developer.themoviedb.org/reference/search-movie

#![allow(dead_code)]

use crate::models::{ProviderRef, SearchResult, WorkType};
use crate::providers::{ProviderError, ProviderResult, ProviderStatus};
use serde::Deserialize;
use tracing::info;

/// TMDB API base URL
const TMDB_API_BASE: &str = "https://api.themoviedb.org/3";

/// TMDB image base URL (w500 size for covers)
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p/w500";

/// Environment variable name for the API token
const TMDB_TOKEN_ENV: &str = "TMDB_ACCESS_TOKEN";

/// TMDB API client
#[derive(Clone)]
pub struct TmdbClient {
    /// Bearer token for API authentication
    token: Option<String>,
    /// HTTP client
    client: reqwest::Client,
}

// =============================================================================
// API Response Types
// =============================================================================

/// TMDB search response wrapper
#[derive(Debug, Deserialize)]
struct TmdbSearchResponse {
    page: u32,
    results: Vec<TmdbSearchResult>,
    total_pages: u32,
    total_results: u32,
}

/// Individual search result from TMDB
#[derive(Debug, Deserialize)]
struct TmdbSearchResult {
    id: u64,
    // Movie fields
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    release_date: Option<String>,
    // TV fields
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    first_air_date: Option<String>,
    // Common fields
    #[serde(default)]
    poster_path: Option<String>,
    #[serde(default)]
    overview: Option<String>,
}

impl TmdbSearchResult {
    /// Convert to our SearchResult type
    fn to_search_result(&self, work_type: WorkType) -> SearchResult {
        let (title, date) = match work_type {
            WorkType::Movie => (
                self.title.clone().unwrap_or_default(),
                self.release_date.clone(),
            ),
            WorkType::TvAnime => (
                self.name.clone().unwrap_or_default(),
                self.first_air_date.clone(),
            ),
            WorkType::Game => unreachable!("TMDB does not support games"),
        };

        // Extract year from date string (format: "YYYY-MM-DD")
        let year = date
            .as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse::<u16>().ok());

        // Build cover URL if poster_path exists
        let cover_url = self
            .poster_path
            .as_ref()
            .map(|path| format!("{}{}", TMDB_IMAGE_BASE, path));

        SearchResult {
            provider_ref: ProviderRef::new("tmdb", self.id.to_string()),
            title,
            year,
            work_type,
            cover_url,
            theme_music_url: None, // TMDB doesn't provide music
        }
    }
}

// =============================================================================
// Client Implementation
// =============================================================================

impl TmdbClient {
    /// Create a new TMDB client
    ///
    /// Reads the API token from the TMDB_ACCESS_TOKEN environment variable.
    /// If the token is not set, the client will be in "not configured" state.
    pub fn new() -> Self {
        let token = get_tmdb_token();

        if token.is_some() {
            info!("TMDB client initialized with token");
        } else {
            info!("TMDB client initialized without token (not configured)");
        }

        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    /// Check if the client has a valid token configured
    pub fn is_configured(&self) -> bool {
        self.token.is_some()
    }

    /// Get the provider status
    pub fn status(&self) -> ProviderStatus {
        if self.token.is_some() {
            ProviderStatus::Available
        } else {
            ProviderStatus::NotConfigured
        }
    }

    /// Search for movies
    pub async fn search_movies(
        &self,
        query: &str,
        page: usize,
    ) -> ProviderResult<Vec<SearchResult>> {
        self.search_internal(query, WorkType::Movie, page).await
    }

    /// Search for TV shows
    pub async fn search_tv(&self, query: &str, page: usize) -> ProviderResult<Vec<SearchResult>> {
        self.search_internal(query, WorkType::TvAnime, page).await
    }

    /// Search for both movies and TV shows
    pub async fn search(
        &self,
        query: &str,
        work_type: WorkType,
        page: usize,
    ) -> ProviderResult<Vec<SearchResult>> {
        match work_type {
            WorkType::Movie => self.search_movies(query, page).await,
            WorkType::TvAnime => self.search_tv(query, page).await,
            WorkType::Game => Err(ProviderError::Unavailable(
                "TMDB does not support games".to_string(),
            )),
        }
    }

    /// Internal search implementation
    async fn search_internal(
        &self,
        query: &str,
        work_type: WorkType,
        page: usize,
    ) -> ProviderResult<Vec<SearchResult>> {
        let token = self
            .token
            .as_ref()
            .ok_or_else(|| ProviderError::AuthError("TMDB API token not configured".to_string()))?;

        // TMDB uses 1-indexed pages
        let tmdb_page = page + 1;

        let endpoint = match work_type {
            WorkType::Movie => "search/movie",
            WorkType::TvAnime => "search/tv",
            WorkType::Game => unreachable!(),
        };

        let url = format!(
            "{}/{}?query={}&page={}&include_adult=false&language=en-US",
            TMDB_API_BASE,
            endpoint,
            urlencoding::encode(query),
            tmdb_page
        );

        info!("TMDB search: {} (page {})", query, tmdb_page);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let search_response: TmdbSearchResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        info!(
            "TMDB returned {} results (page {}/{})",
            search_response.results.len(),
            search_response.page,
            search_response.total_pages
        );

        let results = search_response
            .results
            .into_iter()
            .map(|r| r.to_search_result(work_type.clone()))
            .collect();

        Ok(results)
    }
}

impl Default for TmdbClient {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Get the TMDB token from environment
///
/// In WASM context, we use a compile-time environment variable.
/// In native context, we could read from runtime environment.
fn get_tmdb_token() -> Option<String> {
    // For WASM, we need to use option_env! at compile time
    // For native builds, we could use std::env::var at runtime
    option_env!("TMDB_ACCESS_TOKEN").map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_extraction() {
        let result = TmdbSearchResult {
            id: 123,
            title: Some("Test Movie".to_string()),
            release_date: Some("2024-03-15".to_string()),
            name: None,
            first_air_date: None,
            poster_path: Some("/abc123.jpg".to_string()),
            overview: None,
        };

        let search_result = result.to_search_result(WorkType::Movie);
        assert_eq!(search_result.year, Some(2024));
        assert_eq!(search_result.title, "Test Movie");
        assert_eq!(
            search_result.cover_url,
            Some("https://image.tmdb.org/t/p/w500/abc123.jpg".to_string())
        );
    }
}
