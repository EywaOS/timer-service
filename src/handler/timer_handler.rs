use crate::app_state::AppState;
use crate::model::entities::time_entries::{self, Entity as TimeEntries};
use crate::model::requests::ToggleTimerRequest;
use crate::model::responses::{TimeEntryResponse, TimerStatusResponse};
use chrono::Utc;
use eywa_axum::prelude::sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use eywa_axum::prelude::*;

/// Get current timer status
pub async fn get_timer_status(
    state: State<AppState>,
    ext: Extension<UserId>,
) -> Result<Json<TimerStatusResponse>> {
    let user_id = ext.0;

    let active_timer = TimeEntries::find()
        .filter(time_entries::Column::UserId.eq(user_id.as_uuid()))
        .filter(time_entries::Column::EndTime.is_null())
        .one(state.db())
        .await
        .map_err(AppError::from)?;

    let active_entry = active_timer.map(TimeEntryResponse::from);

    Ok(Json(TimerStatusResponse { active_entry }))
}

/// Unified toggle endpoint
/// - Stops any active timer.
/// - If request has content (project/tag/desc), starts a new timer.
pub async fn toggle_timer(
    state: State<AppState>,
    ext: Extension<UserId>,
    req: Json<ToggleTimerRequest>,
) -> Result<Json<TimerStatusResponse>> {
    let user_id = ext.0;
    let db = state.db();

    // 1. Find active timer
    let active_timer = TimeEntries::find()
        .filter(time_entries::Column::UserId.eq(user_id.as_uuid()))
        .filter(time_entries::Column::EndTime.is_null())
        .one(db)
        .await
        .map_err(AppError::from)?;

    // 2. Stop active timer if exists
    if let Some(timer) = active_timer {
        let mut active: time_entries::ActiveModel = timer.clone().into();
        let end_time = Utc::now();
        active.end_time = Set(Some(end_time.into()));

        let start_time = timer.start_time;
        let duration = (end_time - start_time).num_seconds();
        active.duration_seconds = Set(Some(duration));
        active.updated_at = Set(end_time.into());

        active.update(db).await.map_err(AppError::from)?;
    }

    // 3. Start new timer if details provided
    // We consider "details provided" if at least one field is Some.
    // However, user might want to start an empty timer ("Just tracking time").
    // Let's decide: If request is COMPLETELY default (all Nones), do we start?
    // If request is default, we assume it's just a "Stop" command if there was an active timer.
    // If there was NO active timer, and request is default -> Start default timer?
    // Let's assume if any field is provided OR it's explicit start intention.
    // But `ToggleTimerRequest` makes intent implicit.
    // Logic:
    // - If params match active timer -> Stop. (Already done by "Stop active")
    // - If params provided -> Start new.
    // - If NO params provided -> Just Stop.

    // Correction: Frontend usually sends specific "Stop" action or "Start X".
    // "Toggle" implies if I hit the SAME button, it stops.
    // So if req body matches active timer, we just stop.
    // If req body differs (or is new), we start.

    // Simplified logic for MVP:
    // If `project_id` or `tag_id` or `description` is present -> Start new.
    // If all None -> Do not start new (effectively "Stop").

    let should_start =
        req.project_id.is_some() || req.tag_id.is_some() || req.description.is_some();

    if should_start {
        let id = Uuid::new_v4();
        let new_timer = time_entries::ActiveModel {
            id: Set(id),
            user_id: Set(user_id.as_uuid()),
            project_id: Set(req.project_id),
            tag_id: Set(req.tag_id),
            description: Set(req.description.clone()),
            is_pomodoro: Set(req.is_pomodoro.unwrap_or(false)),
            start_time: Set(Utc::now().into()),
            ..Default::default()
        };

        new_timer.insert(db).await.map_err(AppError::from)?;
    }

    // Return new status
    // Re-fetch logic or construct response? Re-fetch is safer but slower.
    // We can just construct.

    // Actually, calling get_timer_status is easiest to reuse logic.
    get_timer_status(state, ext).await
}
