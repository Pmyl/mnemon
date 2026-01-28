pub const HERO_AUTO_CYCLE_MS: u32 = 10_000;

pub const HERO_TRANSITION_MS: u32 = 600;

pub const DETAILS_TRANSITION_MS: u32 = 400;

pub const UNDO_TIMEOUT_MS: u32 = 5000;

pub const HERO_TRANSITION_SETTLE_MS: u32 = 50;

pub const MANUAL_NAV_PAUSE_MS: u32 = 5000;

pub const NOTE_MIN_READING_TIME_MS: u64 = 3_000;

pub const NOTE_MAX_READING_TIME_MS: u64 = 8_000;

pub const NOTE_FADE_TRANSITION_MS: u32 = 500;

pub const WORDS_PER_SECOND: f64 = 4.0;

pub const SEARCH_MIN_CHARS: usize = 3;

pub const SEARCH_DEBOUNCE_MS: u32 = 300;

pub const SEARCH_PAGE_SIZE: usize = 10;

pub const MAX_FEELINGS: usize = 5;

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
