use eywa_axum::axum::extract::FromRef;
use eywa_axum::eywa_authentication::JwtService as AuthJwtService; // Renamed to avoid conflict
use eywa_axum::{JwtService, prelude::sea_orm::DatabaseConnection}; // Assuming eywa_axum::JwtService is the one used for the field type
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub jwt_service: Arc<JwtService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, secret: String, issuer: String) -> Self {
        Self {
            db: Arc::new(db),
            jwt_service: Arc::new(AuthJwtService::new(secret, issuer, 3600)), // Using AuthJwtService for construction
        }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl FromRef<AppState> for Arc<JwtService> {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_service.clone()
    }
}
