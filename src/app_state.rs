//! Application state management

use dioxus::prelude::*;
use rand::seq::SliceRandom;
use tracing::info;
use uuid::Uuid;

use crate::models::*;
use crate::storage;

/// Combined view of a Mnemon with its associated Work data
/// Used for displaying in the UI without needing to look up Work separately
#[derive(Clone, PartialEq, Debug)]
pub struct MnemonWithWork {
    pub mnemon: Mnemon,
    pub work: Work,
}

/// Application state container
#[derive(Clone)]
pub struct AppState {
    pub works: Signal<Vec<Work>>,
    pub mnemons: Signal<Vec<Mnemon>>,
    /// Shuffled indices for randomized display order
    pub shuffled_indices: Signal<Vec<usize>>,
    /// Whether initial data has been loaded from storage
    pub loaded: Signal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            works: Signal::new(Vec::new()),
            mnemons: Signal::new(Vec::new()),
            shuffled_indices: Signal::new(Vec::new()),
            loaded: Signal::new(false),
        }
    }

    /// Load data from IndexedDB storage (async)
    pub async fn load_from_storage(&mut self) {
        let persisted = storage::load_all_async().await;
        info!(
            "Loaded {} works and {} mnemons from IndexedDB",
            persisted.works.len(),
            persisted.mnemons.len()
        );

        // Create shuffled indices for randomized display order
        let mut indices: Vec<usize> = (0..persisted.mnemons.len()).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        self.works.set(persisted.works);
        self.mnemons.set(persisted.mnemons);
        self.shuffled_indices.set(indices);
        self.loaded.set(true);
    }

    /// Check if data has been loaded from storage
    pub fn is_loaded(&self) -> bool {
        *self.loaded.read()
    }

    /// Get all mnemons with their associated works (in original order)
    pub fn get_mnemons_with_works(&self) -> Vec<MnemonWithWork> {
        let works = self.works.read();
        let mnemons = self.mnemons.read();

        mnemons
            .iter()
            .filter_map(|m| {
                works
                    .iter()
                    .find(|w| w.id == m.work_id)
                    .map(|w| MnemonWithWork {
                        mnemon: m.clone(),
                        work: w.clone(),
                    })
            })
            .collect()
    }

    /// Get the mnemon index for a given position in the shuffled order
    pub fn get_shuffled_index(&self, shuffled_position: usize) -> Option<usize> {
        self.shuffled_indices.read().get(shuffled_position).copied()
    }

    /// Get the total number of mnemons
    pub fn mnemons_count(&self) -> usize {
        self.shuffled_indices.read().len()
    }

    /// Find a work by provider reference
    pub fn find_work_by_provider_ref(&self, provider_ref: &ProviderRef) -> Option<Work> {
        self.works
            .read()
            .iter()
            .find(|w| {
                w.provider_ref
                    .as_ref()
                    .map(|pr| pr.matches(provider_ref))
                    .unwrap_or(false)
            })
            .cloned()
    }

    /// Check if a work with the given provider ref already has a mnemon
    pub fn has_mnemon_for_provider_ref(&self, provider_ref: &ProviderRef) -> bool {
        if let Some(work) = self.find_work_by_provider_ref(provider_ref) {
            self.mnemons.read().iter().any(|m| m.work_id == work.id)
        } else {
            false
        }
    }

    /// Add a new work and return its ID
    pub fn add_work(&mut self, work: Work) -> Uuid {
        let id = work.id;
        let work_clone = work.clone();
        self.works.write().push(work);

        // Persist asynchronously
        spawn(async move {
            if let Err(e) = storage::save_work(&work_clone).await {
                info!("Failed to persist work: {}", e);
            }
        });

        id
    }

    /// Add a new mnemon, returns the shuffled position of the new mnemon
    pub fn add_mnemon(&mut self, mnemon: Mnemon) -> usize {
        let mnemon_clone = mnemon.clone();
        let new_index = self.mnemons.read().len();
        self.mnemons.write().push(mnemon);

        // Reshuffle existing indices, then append the new mnemon's index at the end
        let mut indices = self.shuffled_indices.write();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);
        indices.push(new_index);
        let shuffled_position = indices.len() - 1;
        drop(indices);

        // Persist asynchronously
        spawn(async move {
            if let Err(e) = storage::save_mnemon(&mnemon_clone).await {
                info!("Failed to persist mnemon: {}", e);
            }
        });

        shuffled_position
    }

    /// Remove a mnemon from memory (does not delete from storage yet)
    /// Returns the removed mnemon and its original index for potential restore
    pub fn remove_mnemon(&mut self, mnemon_id: Uuid) -> Option<(Mnemon, usize)> {
        let mut mnemons = self.mnemons.write();
        let mut indices = self.shuffled_indices.write();

        // Find the mnemon's position in the mnemons vec
        let mnemon_idx = mnemons.iter().position(|m| m.id == mnemon_id)?;
        let mnemon = mnemons.remove(mnemon_idx);

        // Remove from shuffled_indices and update indices that pointed to higher positions
        let shuffled_pos = indices.iter().position(|&i| i == mnemon_idx)?;
        indices.remove(shuffled_pos);

        // Update all indices that were greater than the removed index
        for idx in indices.iter_mut() {
            if *idx > mnemon_idx {
                *idx -= 1;
            }
        }

        info!("Removed mnemon {} from memory", mnemon_id);
        Some((mnemon, mnemon_idx))
    }

    /// Restore a previously removed mnemon
    pub fn restore_mnemon(&mut self, mnemon: Mnemon, original_idx: usize) {
        let mut mnemons = self.mnemons.write();
        let mut indices = self.shuffled_indices.write();

        // Update all indices that are >= original_idx
        for idx in indices.iter_mut() {
            if *idx >= original_idx {
                *idx += 1;
            }
        }

        // Insert mnemon back at original position
        mnemons.insert(original_idx, mnemon.clone());

        // Add the index back to shuffled_indices (at the beginning so user sees it)
        indices.insert(0, original_idx);

        info!("Restored mnemon {}", mnemon.id);
    }

    /// Permanently delete a mnemon from storage
    pub fn delete_mnemon_from_storage(mnemon_id: Uuid) {
        spawn(async move {
            if let Err(e) = storage::delete_mnemon(&mnemon_id).await {
                info!("Failed to delete mnemon from storage: {}", e);
            }
        });
    }

    /// Update an existing mnemon (only feelings, notes, and finished_date can be edited)
    pub fn edit_mnemon(
        &mut self,
        mnemon_id: Uuid,
        finished_date: Option<String>,
        feelings: Vec<String>,
        notes: Vec<String>,
    ) {
        let mut mnemons = self.mnemons.write();
        if let Some(mnemon) = mnemons.iter_mut().find(|m| m.id == mnemon_id) {
            mnemon.finished_date = finished_date;
            mnemon.feelings = feelings;
            mnemon.notes = notes;

            let mnemon_clone = mnemon.clone();
            info!("Updated mnemon {} in memory", mnemon_id);

            // Persist asynchronously
            spawn(async move {
                if let Err(e) = storage::save_mnemon(&mnemon_clone).await {
                    tracing::error!("Failed to persist updated mnemon: {}", e);
                }
            });
        }
    }
}
