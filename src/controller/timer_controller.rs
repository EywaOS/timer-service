//! Timer Controller
//!
//! API endpoints for timer management.

use crate::app_state::AppState;
use crate::handler::timer_handler;
use crate::model::requests::ToggleTimerRequest;
use crate::model::responses::{TimeEntryResponse, TimerStatusResponse};
use eywa_axum::{auth_middleware, prelude::*};

pub struct TimerController;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct ErrorResponseExample {
    message: String,
}

#[controller(
    version = "v1",
    prefix = "/timer",
    state = AppState,
    tag = "Timer",
    middleware = auth_middleware,
    security,
    schemas(ToggleTimerRequest, TimerStatusResponse, TimeEntryResponse, ErrorResponseExample, Link)
)]
impl TimerController {
    /// Get current timer status
    #[route(
        GET "/status",
        summary = "Get current timer status",
        responses(
            (status = 200, description = "Current status", body = TimerStatusResponse)
        ),
        links(
            (rel = "self", href = "/timer/status"),
        )
    )]
    pub async fn status(
        state: State<AppState>,
        ext: Extension<UserId>,
    ) -> Result<Json<TimerStatusResponse>> {
        timer_handler::get_timer_status(state, ext).await
    }

    /// Toggle timer (start/stop)
    #[route(POST "/toggle",
        summary = "Start or stop timer",
        description = "Unified toggle endpoint. Stops active timer and optionally starts a new one.",
        links(
            (rel = "self", href = "/timer/status"),
            (rel = "toggle", href = "/timer/toggle", method = "POST")
        ),
        responses(
            (status = 200, description = "Current status", body = TimerStatusResponse),
            (status = 400, description = "Invalid request", body = ErrorResponseExample)
        )
    )]
    pub async fn toggle(
        state: State<AppState>,
        ext: Extension<UserId>,
        req: Json<ToggleTimerRequest>,
    ) -> Result<Json<TimerStatusResponse>> {
        timer_handler::toggle_timer(state, ext, req).await
    }
}
