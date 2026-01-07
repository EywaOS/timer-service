use eywa_axum::prelude::*;
use eywa_validation::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema, Validate)]
pub struct ToggleTimerRequest {
    pub project_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,

    #[validate(length(max = 255, message = "Description too long (max 255 chars)"))]
    pub description: Option<String>,

    pub is_pomodoro: Option<bool>,
}
