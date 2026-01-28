use crate::constants::*;
use web_sys::window;

pub fn calculate_reading_time(text: &str) -> u64 {
    let word_count = text.split_whitespace().count();
    let seconds = (word_count as f64 / WORDS_PER_SECOND).ceil() as u64;
    let ms = seconds * 1000;
    ms.clamp(NOTE_MIN_READING_TIME_MS, NOTE_MAX_READING_TIME_MS)
}

/// Detects if the device is mobile based on viewport width and touch capability
pub fn is_mobile_device() -> bool {
    let window = match window() {
        Some(w) => w,
        None => return false,
    };

    // Check viewport width (768px is the mobile breakpoint)
    let width = window.inner_width().ok()
        .and_then(|w| w.as_f64())
        .unwrap_or(1920.0);

    if width >= 768.0 {
        return false;
    }

    // Also check if touch is supported
    let navigator = window.navigator();
    let max_touch_points = navigator.max_touch_points();

    width < 768.0 && max_touch_points > 0
}
