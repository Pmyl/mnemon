use dioxus::prelude::*;
use rand::seq::SliceRandom;
use tracing::info;
use uuid::Uuid;

use crate::models::*;
use crate::storage;

#[derive(Clone, PartialEq, Debug)]
pub struct MnemonWithWork {
    pub mnemon: Mnemon,
    pub work: Work,
}

#[derive(Clone)]
pub struct AppState {
    pub works: Signal<Vec<Work>>,
    pub mnemons: Signal<Vec<Mnemon>>,
    pub shuffled_indices: Signal<Vec<usize>>,
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

    pub async fn load_from_storage(&mut self) {
        let persisted = storage::load_all_async().await;
        info!(
            "Loaded {} works and {} mnemons from IndexedDB",
            persisted.works.len(),
            persisted.mnemons.len()
        );

        let mut indices: Vec<usize> = (0..persisted.mnemons.len()).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        self.works.set(persisted.works);
        self.mnemons.set(persisted.mnemons);
        self.shuffled_indices.set(indices);
        self.loaded.set(true);
    }

    pub fn is_loaded(&self) -> bool {
        *self.loaded.read()
    }

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

    pub fn get_shuffled_index(&self, shuffled_position: usize) -> Option<usize> {
        self.shuffled_indices.read().get(shuffled_position).copied()
    }

    pub fn mnemons_count(&self) -> usize {
        self.shuffled_indices.read().len()
    }

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

    pub fn has_mnemon_for_provider_ref(&self, provider_ref: &ProviderRef) -> bool {
        if let Some(work) = self.find_work_by_provider_ref(provider_ref) {
            self.mnemons.read().iter().any(|m| m.work_id == work.id)
        } else {
            false
        }
    }

    pub fn add_work(&mut self, work: Work) -> Uuid {
        let id = work.id;
        let work_clone = work.clone();
        self.works.write().push(work);

        spawn(async move {
            if let Err(e) = storage::save_work(&work_clone).await {
                info!("Failed to persist work: {}", e);
            }
        });

        id
    }

    pub fn add_mnemon(&mut self, mnemon: Mnemon) -> usize {
        let mnemon_clone = mnemon.clone();
        let new_index = self.mnemons.read().len();
        self.mnemons.write().push(mnemon);

        let mut indices = self.shuffled_indices.write();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);
        indices.push(new_index);
        let shuffled_position = indices.len() - 1;
        drop(indices);

        spawn(async move {
            if let Err(e) = storage::save_mnemon(&mnemon_clone).await {
                info!("Failed to persist mnemon: {}", e);
            }
        });

        shuffled_position
    }

    pub fn remove_mnemon(&mut self, mnemon_id: Uuid) -> Option<(Mnemon, usize)> {
        let mut mnemons = self.mnemons.write();
        let mut indices = self.shuffled_indices.write();

        let mnemon_idx = mnemons.iter().position(|m| m.id == mnemon_id)?;
        let mnemon = mnemons.remove(mnemon_idx);

        let shuffled_pos = indices.iter().position(|&i| i == mnemon_idx)?;
        indices.remove(shuffled_pos);

        for idx in indices.iter_mut() {
            if *idx > mnemon_idx {
                *idx -= 1;
            }
        }

        info!("Removed mnemon {} from memory", mnemon_id);
        Some((mnemon, mnemon_idx))
    }

    pub fn restore_mnemon(&mut self, mnemon: Mnemon, original_idx: usize) {
        let mut mnemons = self.mnemons.write();
        let mut indices = self.shuffled_indices.write();

        for idx in indices.iter_mut() {
            if *idx >= original_idx {
                *idx += 1;
            }
        }

        mnemons.insert(original_idx, mnemon.clone());

        indices.insert(0, original_idx);

        info!("Restored mnemon {}", mnemon.id);
    }

    pub fn delete_mnemon_from_storage(mnemon_id: Uuid) {
        spawn(async move {
            if let Err(e) = storage::delete_mnemon(&mnemon_id).await {
                info!("Failed to delete mnemon from storage: {}", e);
            }
        });
    }

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

            spawn(async move {
                if let Err(e) = storage::save_mnemon(&mnemon_clone).await {
                    tracing::error!("Failed to persist updated mnemon: {}", e);
                }
            });
        }
    }
}
