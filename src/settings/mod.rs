//! Settings module for managing user configuration
//!
//! This module provides functions for storing and retrieving API tokens
//! from localStorage, allowing users to configure their own API keys
//! for TMDB and RAWG providers.

#![allow(dead_code)]

use tracing::info;
use web_sys::window;

/// LocalStorage key for TMDB access token
const TMDB_TOKEN_KEY: &str = "mnemon_tmdb_access_token";

/// LocalStorage key for RAWG API key
const RAWG_API_KEY_KEY: &str = "mnemon_rawg_api_key";

/// Get the localStorage object
fn get_local_storage() -> Option<web_sys::Storage> {
    window()?.local_storage().ok()?
}

/// Save the TMDB access token to localStorage
pub fn save_tmdb_token(token: &str) -> bool {
    if let Some(storage) = get_local_storage() {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            // Remove the key if empty
            if storage.remove_item(TMDB_TOKEN_KEY).is_ok() {
                info!("Removed TMDB token from localStorage");
                return true;
            }
        } else if storage.set_item(TMDB_TOKEN_KEY, trimmed).is_ok() {
            info!("Saved TMDB token to localStorage");
            return true;
        }
    }
    false
}

/// Load the TMDB access token from localStorage
pub fn load_tmdb_token() -> Option<String> {
    let storage = get_local_storage()?;
    match storage.get_item(TMDB_TOKEN_KEY) {
        Ok(Some(token)) if !token.is_empty() => {
            info!("Loaded TMDB token from localStorage");
            Some(token)
        }
        _ => None,
    }
}

/// Save the RAWG API key to localStorage
pub fn save_rawg_api_key(api_key: &str) -> bool {
    if let Some(storage) = get_local_storage() {
        let trimmed = api_key.trim();
        if trimmed.is_empty() {
            // Remove the key if empty
            if storage.remove_item(RAWG_API_KEY_KEY).is_ok() {
                info!("Removed RAWG API key from localStorage");
                return true;
            }
        } else if storage.set_item(RAWG_API_KEY_KEY, trimmed).is_ok() {
            info!("Saved RAWG API key to localStorage");
            return true;
        }
    }
    false
}

/// Load the RAWG API key from localStorage
pub fn load_rawg_api_key() -> Option<String> {
    let storage = get_local_storage()?;
    match storage.get_item(RAWG_API_KEY_KEY) {
        Ok(Some(api_key)) if !api_key.is_empty() => {
            info!("Loaded RAWG API key from localStorage");
            Some(api_key)
        }
        _ => None,
    }
}

/// Check if TMDB token is configured in localStorage
pub fn is_tmdb_configured() -> bool {
    load_tmdb_token().is_some()
}

/// Check if RAWG API key is configured in localStorage
pub fn is_rawg_configured() -> bool {
    load_rawg_api_key().is_some()
}

/// Get the TMDB token from localStorage
pub fn get_tmdb_token() -> Option<String> {
    load_tmdb_token()
}

/// Get the RAWG API key from localStorage
pub fn get_rawg_api_key() -> Option<String> {
    load_rawg_api_key()
}

/// Clear all stored settings
pub fn clear_all_settings() -> bool {
    if let Some(storage) = get_local_storage() {
        let tmdb_cleared = storage.remove_item(TMDB_TOKEN_KEY).is_ok();
        let rawg_cleared = storage.remove_item(RAWG_API_KEY_KEY).is_ok();
        if tmdb_cleared && rawg_cleared {
            info!("Cleared all settings from localStorage");
            return true;
        }
    }
    false
}

/// Settings state for UI display
#[derive(Clone, Debug, Default)]
pub struct ApiTokenSettings {
    pub tmdb_token: String,
    pub rawg_api_key: String,
}

impl ApiTokenSettings {
    /// Load current settings from localStorage
    pub fn load() -> Self {
        Self {
            tmdb_token: load_tmdb_token().unwrap_or_default(),
            rawg_api_key: load_rawg_api_key().unwrap_or_default(),
        }
    }

    /// Save current settings to localStorage
    pub fn save(&self) -> bool {
        let tmdb_saved = save_tmdb_token(&self.tmdb_token);
        let rawg_saved = save_rawg_api_key(&self.rawg_api_key);
        tmdb_saved && rawg_saved
    }

    /// Check if TMDB is configured (has a value)
    pub fn has_tmdb(&self) -> bool {
        !self.tmdb_token.trim().is_empty()
    }

    /// Check if RAWG is configured (has a value)
    pub fn has_rawg(&self) -> bool {
        !self.rawg_api_key.trim().is_empty()
    }
}
