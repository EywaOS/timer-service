use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditLog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AuditLog::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(AuditLog::Timestamp).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(AuditLog::Action).string().not_null())
                    .col(ColumnDef::new(AuditLog::ResourceType).string().not_null())
                    .col(ColumnDef::new(AuditLog::ResourceId).string())
                    .col(ColumnDef::new(AuditLog::UserId).uuid())
                    .col(ColumnDef::new(AuditLog::UserEmail).string())
                    .col(ColumnDef::new(AuditLog::IpAddress).string())
                    .col(ColumnDef::new(AuditLog::UserAgent).string())
                    .col(ColumnDef::new(AuditLog::CorrelationId).uuid())
                    .col(ColumnDef::new(AuditLog::ServiceName).string())
                    .col(ColumnDef::new(AuditLog::Changes).json_binary())
                    .col(ColumnDef::new(AuditLog::Metadata).json_binary())
                    .to_owned(),
            )
            .await?;

        // Add indexes for faster querying
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_audit_log_timestamp")
                    .table(AuditLog::Table)
                    .col(AuditLog::Timestamp)
                    .to_owned(),
            )
            .await?;
            
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_audit_log_resource")
                    .table(AuditLog::Table)
                    .col(AuditLog::ResourceType)
                    .col(AuditLog::ResourceId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AuditLog {
    Table,
    Id,
    Timestamp,
    Action,
    ResourceType,
    ResourceId,
    UserId,
    UserEmail,
    IpAddress,
    UserAgent,
    CorrelationId,
    ServiceName,
    Changes,
    Metadata,
}
