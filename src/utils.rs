//! Utility functions

use crate::constants::*;

/// Calculate reading time in milliseconds based on word count
/// Average reading speed: ~4 words per second
/// Returns a value between NOTE_MIN_READING_TIME_MS and NOTE_MAX_READING_TIME_MS
pub fn calculate_reading_time(text: &str) -> u64 {
    let word_count = text.split_whitespace().count();
    let seconds = (word_count as f64 / WORDS_PER_SECOND).ceil() as u64;
    let ms = seconds * 1000;
    ms.clamp(NOTE_MIN_READING_TIME_MS, NOTE_MAX_READING_TIME_MS)
}
