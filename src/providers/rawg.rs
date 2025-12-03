//! RAWG (Video Games Database) API client
//!
//! Provides search functionality for video games.
//! RAWG supports CORS and works from browser contexts.
//! API Documentation: https://rawg.io/apidocs

#![allow(dead_code)]

use crate::models::{ProviderRef, SearchResult, WorkType};
use crate::providers::{ProviderError, ProviderResult, ProviderStatus};
use serde::Deserialize;
use tracing::info;

/// RAWG API base URL
const RAWG_API_BASE: &str = "https://api.rawg.io/api";

/// Environment variable name for the API key
const RAWG_API_KEY_ENV: &str = "RAWG_API_KEY";

/// RAWG API client
#[derive(Clone)]
pub struct RawgClient {
    /// API key for authentication
    api_key: Option<String>,
    /// HTTP client
    client: reqwest::Client,
}

// =============================================================================
// API Response Types
// =============================================================================

/// RAWG search response wrapper
#[derive(Debug, Deserialize)]
struct RawgSearchResponse {
    count: u32,
    results: Vec<RawgGame>,
}

/// Individual game result from RAWG
#[derive(Debug, Deserialize)]
struct RawgGame {
    id: u64,
    name: String,
    #[serde(default)]
    released: Option<String>,
    #[serde(default)]
    background_image: Option<String>,
}

impl RawgGame {
    /// Convert to our SearchResult type
    fn to_search_result(&self) -> SearchResult {
        // Extract year from released date (format: "YYYY-MM-DD")
        let year = self
            .released
            .as_ref()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse::<u16>().ok());

        SearchResult {
            provider_ref: ProviderRef::new("rawg", self.id.to_string()),
            title: self.name.clone(),
            year,
            work_type: WorkType::Game,
            cover_url: self.background_image.clone(),
            theme_music_url: None,
        }
    }
}

// =============================================================================
// Client Implementation
// =============================================================================

impl RawgClient {
    /// Create a new RAWG client
    ///
    /// Reads the API key from the RAWG_API_KEY environment variable.
    /// If the key is not set, the client will be in "not configured" state.
    pub fn new() -> Self {
        let api_key = get_rawg_api_key();

        if api_key.is_some() {
            info!("RAWG client initialized with API key");
        } else {
            info!("RAWG client initialized without API key (not configured)");
        }

        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Check if the client has a valid API key configured
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }

    /// Get the provider status
    pub fn status(&self) -> ProviderStatus {
        if self.api_key.is_some() {
            ProviderStatus::Available
        } else {
            ProviderStatus::NotConfigured
        }
    }

    /// Search for games
    pub async fn search(&self, query: &str, page: usize) -> ProviderResult<Vec<SearchResult>> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| ProviderError::AuthError("RAWG API key not configured".to_string()))?;

        // Don't search with empty query
        if query.trim().is_empty() {
            return Ok(Vec::new());
        }

        // RAWG uses 1-indexed pages
        let rawg_page = page + 1;
        let page_size = 20;

        let url = format!(
            "{}/games?key={}&search={}&page={}&page_size={}",
            RAWG_API_BASE,
            api_key,
            urlencoding::encode(query),
            rawg_page,
            page_size
        );

        info!("RAWG search: {} (page {})", query, rawg_page);

        let response = self
            .client
            .get(&url)
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

        let search_response: RawgSearchResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::ParseError(e.to_string()))?;

        info!(
            "RAWG returned {} results (total: {})",
            search_response.results.len(),
            search_response.count
        );

        let results = search_response
            .results
            .into_iter()
            .map(|g| g.to_search_result())
            .collect();

        Ok(results)
    }
}

impl Default for RawgClient {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Get the RAWG API key from environment
///
/// In WASM context, we use a compile-time environment variable.
fn get_rawg_api_key() -> Option<String> {
    option_env!("RAWG_API_KEY").map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_extraction() {
        let game = RawgGame {
            id: 123,
            name: "Test Game".to_string(),
            released: Some("2021-03-15".to_string()),
            background_image: Some("https://example.com/image.jpg".to_string()),
        };

        let search_result = game.to_search_result();
        assert_eq!(search_result.year, Some(2021));
        assert_eq!(search_result.title, "Test Game");
        assert_eq!(
            search_result.cover_url,
            Some("https://example.com/image.jpg".to_string())
        );
    }

    #[test]
    fn test_no_release_date() {
        let game = RawgGame {
            id: 456,
            name: "Unknown Date Game".to_string(),
            released: None,
            background_image: None,
        };

        let search_result = game.to_search_result();
        assert_eq!(search_result.year, None);
        assert_eq!(search_result.cover_url, None);
    }
}
