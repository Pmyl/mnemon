//! Provider integrations for fetching work metadata from external APIs
//!
//! This module contains clients for various media databases:
//! - TMDB (The Movie Database) - Movies and TV shows
//! - RAWG (Video Games Database) - Video games

#![allow(dead_code)]

pub mod rawg;
pub mod tmdb;

use crate::models::{SearchResult, WorkType};
use std::future::Future;
use std::pin::Pin;

/// Error type for provider operations
#[derive(Debug, Clone)]
pub enum ProviderError {
    /// Network request failed
    NetworkError(String),
    /// API returned an error response
    ApiError { status: u16, message: String },
    /// Failed to parse API response
    ParseError(String),
    /// API token is missing or invalid
    AuthError(String),
    /// Provider is not available (offline, rate limited, etc.)
    Unavailable(String),
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ProviderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProviderError::AuthError(msg) => write!(f, "Auth error: {}", msg),
            ProviderError::Unavailable(msg) => write!(f, "Provider unavailable: {}", msg),
        }
    }
}

/// Result type for provider operations
pub type ProviderResult<T> = Result<T, ProviderError>;

/// Status of a provider (for UI feedback)
#[derive(Debug, Clone, PartialEq)]
pub enum ProviderStatus {
    /// Provider is ready to use
    Available,
    /// Provider is not configured (missing API key)
    NotConfigured,
    /// Provider is offline or unreachable
    Offline,
    /// Provider is rate limited
    RateLimited,
}

/// Trait for search providers
///
/// Note: This trait uses async_trait pattern manually since we're in WASM context
pub trait SearchProvider {
    /// Search for works matching the query
    fn search(
        &self,
        query: &str,
        work_type: WorkType,
        page: usize,
    ) -> Pin<Box<dyn Future<Output = ProviderResult<Vec<SearchResult>>> + '_>>;

    /// Check if the provider is available
    fn status(&self) -> ProviderStatus;

    /// Get the provider name for display
    fn name(&self) -> &'static str;
}
