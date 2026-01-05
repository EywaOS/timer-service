use eywa_axum::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct ToggleTimerRequest {
    pub project_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub description: Option<String>,
    pub is_pomodoro: Option<bool>,
}
