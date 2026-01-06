use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Create schema 'tsaheylu'
        db.execute_unprepared("CREATE SCHEMA IF NOT EXISTS tsaheylu")
            .await?;

        // Create time_entries table
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS tsaheylu.time_entries (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL,
                project_id UUID,
                tag_id UUID,
                start_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                end_time TIMESTAMPTZ,
                duration_seconds BIGINT,
                description TEXT,
                is_pomodoro BOOLEAN DEFAULT FALSE,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            )
            "#,
        )
        .await?;

        // Create index for active timer check (where end_time is NULL)
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_active_timer ON tsaheylu.time_entries (user_id) WHERE end_time IS NULL"
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP TABLE IF EXISTS tsaheylu.time_entries")
            .await?;
        db.execute_unprepared("DROP SCHEMA IF EXISTS tsaheylu")
            .await?;

        Ok(())
    }
}
