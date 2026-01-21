use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ProviderRef {
    pub provider_source: String,

    pub provider_id: String,
}

impl ProviderRef {
    pub fn new(provider_source: impl Into<String>, provider_id: impl Into<String>) -> Self {
        Self {
            provider_source: provider_source.into(),
            provider_id: provider_id.into(),
        }
    }

    pub fn matches(&self, other: &ProviderRef) -> bool {
        self.provider_source == other.provider_source && self.provider_id == other.provider_id
    }
}
