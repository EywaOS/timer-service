use eywa_audit::{AuditLogger, PostgresAuditStorage};
use eywa_axum::axum::extract::FromRef;
use eywa_axum::eywa_authentication::JwtService as AuthJwtService;
use eywa_axum::{prelude::sea_orm::DatabaseConnection, JwtService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwt_service: Arc<JwtService>,
    pub audit: AuditLogger,
}

impl AppState {
    pub fn new(db: DatabaseConnection, secret: String, issuer: String) -> Self {
        let storage = Arc::new(PostgresAuditStorage::new(db.clone()));
        let audit = AuditLogger::with_postgres("timer-service", storage);

        Self {
            db: Arc::new(db),
            jwt_service: Arc::new(AuthJwtService::new(secret, issuer, 3600)),
            audit,
        }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn audit(&self) -> &AuditLogger {
        &self.audit
    }
}

impl FromRef<AppState> for Arc<JwtService> {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_service.clone()
    }
}

impl FromRef<AppState> for AuditLogger {
    fn from_ref(state: &AppState) -> Self {
        state.audit.clone()
    }
}
