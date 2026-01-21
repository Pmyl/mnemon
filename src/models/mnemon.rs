use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Mnemon {
    pub id: Uuid,

    pub work_id: Uuid,

    pub finished_date: Option<String>,

    pub feelings: Vec<String>,

    pub notes: Vec<String>,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Mnemon {
    pub fn new(
        work_id: Uuid,
        finished_date: Option<String>,
        feelings: Vec<String>,
        notes: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            work_id,
            finished_date,
            feelings,
            notes,
            created_at: chrono::Utc::now(),
        }
    }
}
