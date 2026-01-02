//! Storage module for persisting Mnemon data
//!
//! This module provides web persistence using IndexedDB via the rexie crate.
//! Data is stored in object stores and survives page reloads.
//! Supports storing both structured data (JSON) and binary blobs (images, audio).

use crate::models::{Mnemon, Work};
use rexie::{ObjectStore, Rexie, TransactionMode};
use tracing::info;

/// Database name
const DB_NAME: &str = "mnemon_db";

/// Database version - increment when schema changes
const DB_VERSION: u32 = 1;

/// Object store names
const WORKS_STORE: &str = "works";
const MNEMONS_STORE: &str = "mnemons";
const ASSETS_STORE: &str = "assets";

/// Storage error type
#[derive(Debug)]
pub enum StorageError {
    /// Failed to build/open database
    DatabaseError(String),
    /// Failed to serialize data
    SerializeError(String),
    /// Failed to deserialize data
    DeserializeError(String),
    /// Transaction failed
    TransactionError(String),
    /// Store operation failed
    StoreError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            StorageError::SerializeError(msg) => write!(f, "Serialize error: {}", msg),
            StorageError::DeserializeError(msg) => write!(f, "Deserialize error: {}", msg),
            StorageError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
            StorageError::StoreError(msg) => write!(f, "Store error: {}", msg),
        }
    }
}

impl From<rexie::Error> for StorageError {
    fn from(e: rexie::Error) -> Self {
        StorageError::DatabaseError(e.to_string())
    }
}

impl From<serde_wasm_bindgen::Error> for StorageError {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        StorageError::SerializeError(e.to_string())
    }
}

/// Result type for storage operations
pub type StorageResult<T> = Result<T, StorageError>;

/// Build and open the IndexedDB database
async fn open_database() -> StorageResult<Rexie> {
    let rexie = Rexie::builder(DB_NAME)
        .version(DB_VERSION)
        .add_object_store(ObjectStore::new(WORKS_STORE).key_path("id"))
        .add_object_store(ObjectStore::new(MNEMONS_STORE).key_path("id"))
        .add_object_store(ObjectStore::new(ASSETS_STORE).key_path("id"))
        .build()
        .await?;

    Ok(rexie)
}

/// Save a single work to IndexedDB
pub async fn save_work(work: &Work) -> StorageResult<()> {
    let db = open_database().await?;

    let transaction = db
        .transaction(&[WORKS_STORE], TransactionMode::ReadWrite)
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let store = transaction
        .store(WORKS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let js_value = serde_wasm_bindgen::to_value(work)?;
    store
        .put(&js_value, None)
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    transaction
        .done()
        .await
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    info!("Saved work '{}' to IndexedDB", work.title_en);
    Ok(())
}

/// Save a single mnemon to IndexedDB
pub async fn save_mnemon(mnemon: &Mnemon) -> StorageResult<()> {
    let db = open_database().await?;

    let transaction = db
        .transaction(&[MNEMONS_STORE], TransactionMode::ReadWrite)
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let store = transaction
        .store(MNEMONS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let js_value = serde_wasm_bindgen::to_value(mnemon)?;
    store
        .put(&js_value, None)
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    transaction
        .done()
        .await
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    info!("Saved mnemon {} to IndexedDB", mnemon.id);
    Ok(())
}

/// Load all works from IndexedDB
pub async fn load_works() -> StorageResult<Vec<Work>> {
    let db = open_database().await?;

    let transaction = db
        .transaction(&[WORKS_STORE], TransactionMode::ReadOnly)
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let store = transaction
        .store(WORKS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let js_values = store
        .get_all(None, None)
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let mut works = Vec::new();
    for js_value in js_values {
        let work: Work = serde_wasm_bindgen::from_value(js_value)
            .map_err(|e| StorageError::DeserializeError(e.to_string()))?;
        works.push(work);
    }

    info!("Loaded {} works from IndexedDB", works.len());
    Ok(works)
}

/// Load all mnemons from IndexedDB
pub async fn load_mnemons() -> StorageResult<Vec<Mnemon>> {
    let db = open_database().await?;

    let transaction = db
        .transaction(&[MNEMONS_STORE], TransactionMode::ReadOnly)
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let store = transaction
        .store(MNEMONS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let js_values = store
        .get_all(None, None)
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let mut mnemons = Vec::new();
    for js_value in js_values {
        let mnemon: Mnemon = serde_wasm_bindgen::from_value(js_value)
            .map_err(|e| StorageError::DeserializeError(e.to_string()))?;
        mnemons.push(mnemon);
    }

    info!("Loaded {} mnemons from IndexedDB", mnemons.len());
    Ok(mnemons)
}

/// Persisted data container for loading both works and mnemons together
#[derive(Debug, Default)]
pub struct PersistedData {
    pub works: Vec<Work>,
    pub mnemons: Vec<Mnemon>,
}

/// Load all persisted data (async version)
pub async fn load_all_async() -> PersistedData {
    let works = load_works().await.unwrap_or_else(|e| {
        info!("Failed to load works: {}", e);
        Vec::new()
    });

    let mnemons = load_mnemons().await.unwrap_or_else(|e| {
        info!("Failed to load mnemons: {}", e);
        Vec::new()
    });

    PersistedData { works, mnemons }
}

/// Asset metadata for storing binary data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredAsset {
    /// Unique identifier (usually the URL or a hash)
    pub id: String,
    /// MIME type of the asset
    pub mime_type: String,
    /// The binary data as a base64-encoded string
    /// (IndexedDB can store blobs directly, but base64 is simpler for now)
    pub data_base64: String,
}

/// Save an asset (image/audio) to IndexedDB
#[allow(dead_code)]
pub async fn save_asset(asset: &StoredAsset) -> StorageResult<()> {
    let db = open_database().await?;

    let transaction = db
        .transaction(&[ASSETS_STORE], TransactionMode::ReadWrite)
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let store = transaction
        .store(ASSETS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let js_value = serde_wasm_bindgen::to_value(asset)?;
    store
        .put(&js_value, None)
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    transaction
        .done()
        .await
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    info!("Saved asset '{}' to IndexedDB", asset.id);
    Ok(())
}

/// Load an asset by ID from IndexedDB
#[allow(dead_code)]
pub async fn load_asset(id: &str) -> StorageResult<Option<StoredAsset>> {
    let db = open_database().await?;

    let transaction = db
        .transaction(&[ASSETS_STORE], TransactionMode::ReadOnly)
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let store = transaction
        .store(ASSETS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let js_key = serde_wasm_bindgen::to_value(id)?;
    let result = store
        .get(js_key)
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    match result {
        Some(js_value) => {
            let asset: StoredAsset = serde_wasm_bindgen::from_value(js_value)
                .map_err(|e| StorageError::DeserializeError(e.to_string()))?;
            info!("Loaded asset '{}' from IndexedDB", id);
            Ok(Some(asset))
        }
        None => {
            info!("Asset '{}' not found in IndexedDB", id);
            Ok(None)
        }
    }
}

/// Clear all stored data (useful for testing/reset)
#[allow(dead_code)]
pub async fn clear_all() -> StorageResult<()> {
    let db = open_database().await?;

    let transaction = db
        .transaction(
            &[WORKS_STORE, MNEMONS_STORE, ASSETS_STORE],
            TransactionMode::ReadWrite,
        )
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    let works_store = transaction
        .store(WORKS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;
    works_store
        .clear()
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let mnemons_store = transaction
        .store(MNEMONS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;
    mnemons_store
        .clear()
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    let assets_store = transaction
        .store(ASSETS_STORE)
        .map_err(|e| StorageError::StoreError(e.to_string()))?;
    assets_store
        .clear()
        .await
        .map_err(|e| StorageError::StoreError(e.to_string()))?;

    transaction
        .done()
        .await
        .map_err(|e| StorageError::TransactionError(e.to_string()))?;

    info!("Cleared all IndexedDB storage");
    Ok(())
}
