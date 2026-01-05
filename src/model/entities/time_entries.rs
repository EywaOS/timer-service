use eywa_axum::prelude::sea_orm;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "time_entries", schema_name = "tsaheylu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub tag_id: Option<Uuid>,
    pub start_time: DateTimeUtc,
    pub end_time: Option<DateTimeUtc>,
    pub duration_seconds: Option<i64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub is_pomodoro: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
