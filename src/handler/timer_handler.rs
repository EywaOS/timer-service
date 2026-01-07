use crate::app_state::AppState;
use crate::model::entities::time_entries::{self, Entity as TimeEntries};
use crate::model::requests::ToggleTimerRequest;
use crate::model::responses::{TimeEntryResponse, TimerStatusResponse};
use chrono::Utc;
use eywa_audit::{AuditAction, AuditEvent};
use eywa_axum::prelude::sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
};
use eywa_axum::prelude::*;
use eywa_validation::ValidatedJson;

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
    ValidatedJson(req): ValidatedJson<ToggleTimerRequest>,
) -> Result<Json<TimerStatusResponse>> {
    let user_id = ext.0;
    let db = state.db();
    let audit = state.audit();

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

        let updated_timer = active.update(db).await.map_err(AppError::from)?;

        // AUDIT: Log stop event
        audit.log(
            AuditEvent::new(AuditAction::Update, "timer")
                .resource_id(updated_timer.id.to_string())
                .user_id(user_id.as_uuid())
                .changes(&timer, &updated_timer) // Log changes
                .metadata(serde_json::json!({ "action": "stop_timer", "duration": duration })),
        );
    }

    // 3. Start new timer if details provided
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

        // Needs to be converted to Model for auditing (after insert)
        // Or we can construct "after" state manually.
        // Let's insert first.
        let created_timer = new_timer.insert(db).await.map_err(AppError::from)?;

        // AUDIT: Log create event
        audit.log(
            AuditEvent::new(AuditAction::Create, "timer")
                .resource_id(created_timer.id.to_string())
                .user_id(user_id.as_uuid())
                .changes(&(), &created_timer) // Before is empty
                .metadata(serde_json::json!({ "action": "start_timer" })),
        );
    }

    get_timer_status(state, ext).await
}
