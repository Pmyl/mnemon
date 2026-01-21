use crate::models::*;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct MnemonForm {
    pub work_type: Option<WorkType>,
    pub title: String,
    pub year: String,

    pub provider_ref: Option<ProviderRef>,
    pub cover_url: Option<String>,
    pub theme_music_url: Option<String>,

    pub finished_date: String,
    pub feelings: Vec<String>,
    pub notes: String,
}

impl MnemonForm {
    pub fn is_step1_valid(&self) -> bool {
        self.work_type.is_some() && !self.title.trim().is_empty()
    }

    pub fn from_mnemon_for_edit(mnemon: &Mnemon, work: &Work) -> Self {
        Self {
            work_type: Some(work.work_type.clone()),
            title: work.title_en.clone(),
            year: work.release_year.map(|y| y.to_string()).unwrap_or_default(),
            provider_ref: work.provider_ref.clone(),
            cover_url: work.cover_image_local_uri.clone(),
            theme_music_url: work.theme_music_local_uri.clone(),
            finished_date: mnemon.finished_date.clone().unwrap_or_default(),
            feelings: mnemon.feelings.clone(),
            notes: mnemon.notes.join("\n"),
        }
    }

    pub fn parse_notes(&self) -> Vec<String> {
        self.notes
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn parse_finished_date(&self) -> Option<String> {
        if self.finished_date.is_empty() {
            None
        } else {
            Some(self.finished_date.clone())
        }
    }
}
