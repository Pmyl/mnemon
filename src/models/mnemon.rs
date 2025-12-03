//! Mnemon model - a user-created memory referencing a Work

use uuid::Uuid;

/// A user-created memory referencing exactly one Work
///
/// Mnemons capture personal memories and feelings about a piece of media.
/// All personal fields are optional - this is not a tracking app.
#[derive(Clone, PartialEq, Debug)]
pub struct Mnemon {
    /// Unique identifier (UUID)
    pub id: Uuid,

    /// Reference to the Work this mnemon is about
    pub work_id: Uuid,

    /// Date when the user finished/completed the work (optional)
    pub finished_date: Option<String>,

    /// Feelings associated with this memory (0-5 from fixed taxonomy)
    pub feelings: Vec<String>,

    /// Rich text notes about this memory (stored as lines for now)
    pub notes: Vec<String>,

    /// Timestamp when this mnemon was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Mnemon {
    /// Creates a new Mnemon for a given Work
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
