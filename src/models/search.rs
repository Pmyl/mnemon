use super::{ProviderRef, WorkType};

#[derive(Clone, PartialEq, Debug)]
pub struct SearchResult {
    pub provider_ref: ProviderRef,

    pub title: String,

    pub year: Option<u16>,

    pub work_type: WorkType,

    pub cover_url: Option<String>,

    pub theme_music_url: Option<String>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct SearchResultsPage {
    pub results: Vec<SearchResult>,

    pub total_count: usize,

    pub page: usize,

    pub total_pages: usize,
}

#[allow(dead_code)]
impl SearchResultsPage {
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

    pub fn has_next_page(&self) -> bool {
        self.page + 1 < self.total_pages
    }

    pub fn has_previous_page(&self) -> bool {
        self.page > 0
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}
