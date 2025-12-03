//! Search-related models for provider queries

use super::{ProviderRef, WorkType};

/// A search result from a provider query
///
/// Represents a potential Work that can be selected from search results.
/// Contains all the information needed to create a Work if selected.
#[derive(Clone, PartialEq, Debug)]
pub struct SearchResult {
    /// Provider reference for deduplication
    pub provider_ref: ProviderRef,

    /// Title of the work
    pub title: String,

    /// Release year (if known)
    pub year: Option<u16>,

    /// Type of work
    pub work_type: WorkType,

    /// URL to cover image (will be cached locally on selection)
    pub cover_url: Option<String>,

    /// URL to theme music preview (will be cached locally on selection)
    pub theme_music_url: Option<String>,
}

/// Paginated search results
///
/// Contains a page of search results along with pagination metadata.
/// Used for implementing infinite scroll or pagination UI in the future.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SearchResultsPage {
    /// The search results for this page
    pub results: Vec<SearchResult>,

    /// Total number of results across all pages
    pub total_count: usize,

    /// Current page number (0-indexed)
    pub page: usize,

    /// Total number of pages available
    pub total_pages: usize,
}

#[allow(dead_code)]
impl SearchResultsPage {
    /// Creates a new SearchResultsPage
    pub fn new(
        results: Vec<SearchResult>,
        total_count: usize,
        page: usize,
        total_pages: usize,
    ) -> Self {
        Self {
            results,
            total_count,
            page,
            total_pages,
        }
    }

    /// Returns true if there are more pages after this one
    pub fn has_next_page(&self) -> bool {
        self.page + 1 < self.total_pages
    }

    /// Returns true if there are pages before this one
    pub fn has_previous_page(&self) -> bool {
        self.page > 0
    }

    /// Returns true if there are no results
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}
