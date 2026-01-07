//! RAWG (Video Games Database) API client
//!
//! Provides search functionality for video games.
//! RAWG supports CORS and works from browser contexts.
//! API Documentation: https://rawg.io/apidocs

#![allow(dead_code)]

use crate::models::{ProviderRef, SearchResult, WorkType};
use crate::providers::{ProviderError, ProviderResult, ProviderStatus};
use crate::settings;
use serde::Deserialize;
use tracing::info;

/// RAWG API base URL
const RAWG_API_BASE: &str = "https://api.rawg.io/api";

/// RAWG API client
#[derive(Clone)]
pub struct RawgClient {
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
    pub fn new() -> Self {
        info!("RAWG client initialized");
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Check if the client has a valid API key configured (checks localStorage)
    pub fn is_configured(&self) -> bool {
        settings::is_rawg_configured()
    }

    /// Get the provider status (checks localStorage)
    pub fn status(&self) -> ProviderStatus {
        if settings::is_rawg_configured() {
            ProviderStatus::Available
        } else {
            ProviderStatus::NotConfigured
        }
    }

    /// Search for games
    pub async fn search(&self, query: &str, page: usize) -> ProviderResult<Vec<SearchResult>> {
        let api_key = settings::get_rawg_api_key().ok_or_else(|| {
            ProviderError::AuthError(
                "RAWG API key not configured. Add your key in Settings.".to_string(),
            )
        })?;

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
            &api_key,
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
            .map_err(|e| {
                let err_msg = e.to_string();
                info!("RAWG network error during request: {}", err_msg);

                // In browser/WASM, when RAWG returns 401 for invalid API key,
                // the browser's CORS policy blocks the response and reqwest
                // reports it as "error sending request" before we can check status.
                // We need to treat this as an auth error, not a network error.
                if err_msg.contains("error sending request") {
                    info!("Detected 'error sending request' - likely CORS-blocked 401 from RAWG");
                    ProviderError::ApiError {
                        status: 401,
                        message: "Invalid or missing API key".to_string(),
                    }
                } else if err_msg.contains("401") || err_msg.contains("Unauthorized") {
                    ProviderError::ApiError {
                        status: 401,
                        message: "Invalid API key".to_string(),
                    }
                } else {
                    ProviderError::NetworkError(err_msg)
                }
            })?;

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
