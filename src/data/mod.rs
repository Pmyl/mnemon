//! Data layer for Mnemon
//!
//! This module contains data access functions and fixtures for development.

pub mod fixtures;

use crate::constants::SEARCH_PAGE_SIZE;
use crate::models::{SearchResult, SearchResultsPage, WorkType};
use fixtures::get_fixture_search_results;

/// Search for works by query and type
///
/// This function will be replaced with real API calls in the future.
/// Currently uses fixture data for development and testing.
///
/// # Arguments
/// * `query` - Search query string (matches against title)
/// * `work_type` - Type of work to search for
/// * `page` - Page number (0-indexed)
///
/// # Returns
/// A `SearchResultsPage` containing matching results and pagination info
pub fn search_works(query: &str, work_type: WorkType, page: usize) -> SearchResultsPage {
    let fixtures = get_fixture_search_results();

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
