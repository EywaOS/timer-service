use eywa_axum::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
// We access TimeEntryResponse through the parent module's re-exports or sibling module
use super::time_entry_response::TimeEntryResponse;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TimerStatusResponse {
    pub active_entry: Option<TimeEntryResponse>,
}
