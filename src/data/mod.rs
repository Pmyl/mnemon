//! Data layer for Mnemon
//!
//! This module provides unified search functionality that routes to the appropriate
//! provider based on work type:
//! - Movies: TMDB API
//! - TV/Anime: TMDB API
//! - Games: RAWG API

#![allow(dead_code)]

pub mod fixtures;

use crate::constants::SEARCH_PAGE_SIZE;
use crate::models::{SearchResult, SearchResultsPage, WorkType};
use crate::providers::rawg::RawgClient;
use crate::providers::tmdb::TmdbClient;
use crate::providers::{ProviderError, ProviderStatus};
use std::sync::Arc;
use tracing::info;

/// Search status returned alongside results
#[derive(Debug, Clone, PartialEq)]
pub enum SearchStatus {
    /// Search completed successfully using provider API
    Success,
    /// Provider not configured, using manual entry only
    ProviderNotConfigured,
    /// Network error, using fallback or manual entry
    NetworkError(String),
    /// API error (invalid key, rate limit, etc.)
    ApiError { status: u16, message: String },
    /// Using fixture data (for Games)
    UsingFixtures,
}

/// Result of a search operation
#[derive(Debug, Clone)]
pub struct SearchResponse {
    /// The search results
    pub results: Vec<SearchResult>,
    /// Status of the search operation
    pub status: SearchStatus,
    /// Total count (if known from API)
    pub total_count: Option<usize>,
}

impl SearchResponse {
    /// Create a successful response
    pub fn success(results: Vec<SearchResult>, total_count: Option<usize>) -> Self {
        Self {
            results,
            status: SearchStatus::Success,
            total_count,
        }
    }

    /// Create a response indicating provider not configured
    pub fn not_configured() -> Self {
        Self {
            results: Vec::new(),
            status: SearchStatus::ProviderNotConfigured,
            total_count: None,
        }
    }

    /// Create a response for network error
    pub fn network_error(message: String) -> Self {
        Self {
            results: Vec::new(),
            status: SearchStatus::NetworkError(message),
            total_count: None,
        }
    }

    /// Create a response for API error (invalid key, rate limit, etc.)
    pub fn api_error(status: u16, message: String) -> Self {
        Self {
            results: Vec::new(),
            status: SearchStatus::ApiError { status, message },
            total_count: None,
        }
    }

    /// Create a response using fixture data
    pub fn from_fixtures(results: Vec<SearchResult>, total_count: usize) -> Self {
        Self {
            results,
            status: SearchStatus::UsingFixtures,
            total_count: Some(total_count),
        }
    }
}

/// Unified search service that routes to appropriate providers
#[derive(Clone)]
pub struct SearchService {
    tmdb: Arc<TmdbClient>,
    rawg: Arc<RawgClient>,
}

impl SearchService {
    /// Create a new search service
    pub fn new() -> Self {
        Self {
            tmdb: Arc::new(TmdbClient::new()),
            rawg: Arc::new(RawgClient::new()),
        }
    }

    /// Check if TMDB is configured
    pub fn is_tmdb_configured(&self) -> bool {
        self.tmdb.status() == ProviderStatus::Available
    }

    /// Check if RAWG is configured
    pub fn is_rawg_configured(&self) -> bool {
        self.rawg.status() == ProviderStatus::Available
    }

    /// Search for works by query and type
    ///
    /// Routes to the appropriate provider:
    /// - Movies/TV: TMDB API (if configured)
    /// - Games: RAWG API (if configured)
    pub async fn search(&self, query: &str, work_type: WorkType, page: usize) -> SearchResponse {
        match work_type {
            WorkType::Movie | WorkType::TvAnime => self.search_tmdb(query, work_type, page).await,
            WorkType::Game => self.search_rawg(query, page).await,
        }
    }

    /// Search TMDB for movies or TV shows
    async fn search_tmdb(&self, query: &str, work_type: WorkType, page: usize) -> SearchResponse {
        // Check if TMDB is configured
        if !self.is_tmdb_configured() {
            info!("TMDB not configured, returning empty results");
            return SearchResponse::not_configured();
        }

        // Don't search with empty query
        if query.trim().is_empty() {
            return SearchResponse::success(Vec::new(), Some(0));
        }

        info!("TMDB is configured, performing search for '{}'", query);

        // Perform the search
        match self.tmdb.search(query, work_type, page).await {
            Ok(results) => {
                info!("TMDB search returned {} results", results.len());
                SearchResponse::success(results, None)
            }
            Err(e) => {
                info!(
                    "TMDB search error: {} (type: {:?})",
                    e,
                    std::mem::discriminant(&e)
                );
                match e {
                    ProviderError::NetworkError(msg) => {
                        info!("Returning NetworkError response for TMDB");
                        SearchResponse::network_error(msg)
                    }
                    ProviderError::AuthError(_) => {
                        info!("Returning NotConfigured response for TMDB");
                        SearchResponse::not_configured()
                    }
                    ProviderError::ApiError { status, message } => {
                        info!("Returning ApiError response for TMDB ({})", status);
                        SearchResponse::api_error(status, message)
                    }
                    _ => {
                        info!("Returning NetworkError (fallback) response for TMDB");
                        SearchResponse::network_error(e.to_string())
                    }
                }
            }
        }
    }

    /// Search games using RAWG API
    async fn search_rawg(&self, query: &str, page: usize) -> SearchResponse {
        // Check if RAWG is configured
        if !self.is_rawg_configured() {
            info!("RAWG not configured, returning empty results");
            return SearchResponse::not_configured();
        }

        // Don't search with empty query
        if query.trim().is_empty() {
            return SearchResponse::success(Vec::new(), Some(0));
        }

        info!("RAWG is configured, performing search for '{}'", query);

        // Perform the search
        match self.rawg.search(query, page).await {
            Ok(results) => {
                info!("RAWG search returned {} results", results.len());
                SearchResponse::success(results, None)
            }
            Err(e) => {
                info!(
                    "RAWG search error: {} (type: {:?})",
                    e,
                    std::mem::discriminant(&e)
                );
                match e {
                    ProviderError::NetworkError(msg) => {
                        info!("Returning NetworkError response for RAWG");
                        SearchResponse::network_error(msg)
                    }
                    ProviderError::AuthError(_) => {
                        info!("Returning NotConfigured response for RAWG");
                        SearchResponse::not_configured()
                    }
                    ProviderError::ApiError { status, message } => {
                        info!("Returning ApiError response for RAWG ({})", status);
                        SearchResponse::api_error(status, message)
                    }
                    _ => {
                        info!("Returning NetworkError (fallback) response for RAWG");
                        SearchResponse::network_error(e.to_string())
                    }
                }
            }
        }
    }
}

impl Default for SearchService {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Legacy function for backward compatibility
// =============================================================================

/// Search for works by query and type (synchronous, fixtures only)
///
/// This is kept for backward compatibility. Prefer using SearchService for
/// async provider-backed searches.
pub fn search_works(query: &str, work_type: WorkType, page: usize) -> SearchResultsPage {
    let fixtures = fixtures::get_fixture_search_results();

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
    let total_pages = (total_count as f32 / SEARCH_PAGE_SIZE as f32).ceil() as usize;
    let start_idx = page * SEARCH_PAGE_SIZE;
    let end_idx = (start_idx + SEARCH_PAGE_SIZE).min(total_count);

    let results = if start_idx < total_count {
        filtered[start_idx..end_idx].to_vec()
    } else {
        Vec::new()
    };

    SearchResultsPage::new(results, total_count, page, total_pages)
}
