use super::time_entry_response::TimeEntryResponse;
use eywa_axum::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TimerStatusResponse {
    pub active_entry: Option<TimeEntryResponse>,
}
