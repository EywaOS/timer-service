use crate::model::entities::time_entries;
use chrono::{DateTime, Utc};
use eywa_axum::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TimeEntryResponse {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub description: Option<String>,
    pub is_pomodoro: bool,
}

impl From<time_entries::Model> for TimeEntryResponse {
    fn from(model: time_entries::Model) -> Self {
        Self {
            id: model.id,
            project_id: model.project_id,
            tag_id: model.tag_id,
            start_time: model.start_time,
            end_time: model.end_time,
            duration_seconds: model.duration_seconds,
            description: model.description,
            is_pomodoro: model.is_pomodoro,
        }
    }
}
