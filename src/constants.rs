//! Application-wide constants for Mnemon

// =============================================================================
// TIMING CONSTANTS
// =============================================================================

/// Duration in milliseconds before auto-advancing to the next mnemon in the hero
pub const HERO_AUTO_CYCLE_MS: u32 = 10_000;

/// Duration in milliseconds for the slide transition animation
pub const HERO_TRANSITION_MS: u32 = 600;

/// Duration in milliseconds for the details slide transition
pub const DETAILS_TRANSITION_MS: u32 = 400;

/// Duration in milliseconds for the undo toast before permanent deletion
pub const UNDO_TIMEOUT_MS: u32 = 5000;

/// Small delay after transition before resetting state
pub const HERO_TRANSITION_SETTLE_MS: u32 = 50;

/// Minimum reading time for notes in milliseconds
pub const NOTE_MIN_READING_TIME_MS: u64 = 3_000;

/// Maximum reading time for notes in milliseconds
pub const NOTE_MAX_READING_TIME_MS: u64 = 8_000;

/// Duration in milliseconds for note fade transition
pub const NOTE_FADE_TRANSITION_MS: u32 = 500;

/// Average words per second for reading time calculation
pub const WORDS_PER_SECOND: f64 = 4.0;

// =============================================================================
// SEARCH CONSTANTS
// =============================================================================

/// Minimum characters required to trigger auto-search (can be bypassed with Enter)
pub const SEARCH_MIN_CHARS: usize = 3;

/// Debounce delay in milliseconds before triggering search after typing stops
pub const SEARCH_DEBOUNCE_MS: u32 = 300;

/// Number of search results per page
pub const SEARCH_PAGE_SIZE: usize = 10;

// =============================================================================
// UI CONSTANTS
// =============================================================================

/// Maximum number of feelings that can be selected per mnemon
pub const MAX_FEELINGS: usize = 5;

/// Number of notes to display in the hero rotation
pub const HERO_NOTES_TO_DISPLAY: usize = 2;

// =============================================================================
// FEELINGS TAXONOMY
// =============================================================================

/// Fixed feelings taxonomy with emoji representations
pub const FEELINGS: &[(&str, &str)] = &[
    ("Nostalgic", "🌅"),
    ("Cozy", "☕"),
    ("Melancholic", "🌧️"),
    ("Epic", "⚔️"),
    ("Wholesome", "💚"),
    ("Bittersweet", "🍃"),
    ("Heartwarming", "💝"),
    ("Chill", "😎"),
    ("Adventurous", "🗺️"),
    ("Uplifting", "🎈"),
    ("Mysterious", "🔮"),
    ("Somber", "🌑"),
];
