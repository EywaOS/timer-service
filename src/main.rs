//! Timer Service - Main Entry Point
//!
//! Service for managing time tracking (Tsaheylu - The Bond).
//!
//! This service provides:
//! - Timer toggle (start/stop)
//! - Timer status queries
//! - Time entry management

use eywa_axum::prelude::*;
use migration::MigratorTrait;

mod app_state;
mod controller;
mod handler;
mod model;

use controller::TimerController;

#[derive(Debug, Deserialize)]
struct TimerConfig {
    database: DatabaseConfig,
    auth: AuthConfigInner,
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug, Deserialize)]
struct AuthConfigInner {
    secret: String,
    issuer: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LoggingConfig {
    level: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load Configuration
    let config: TimerConfig = EywaConfig::load()?;

    // 1.1 Initialize Tracing
    let log_level = config.logging.level.unwrap_or_else(|| "info".to_string());
    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_level)),
        )
        .init();

    info!("Starting Timer Service (Tsaheylu)...");
    info!("Log level: {}", log_level);

    let jwt_issuer = config
        .auth
        .issuer
        .clone()
        .unwrap_or_else(|| "eywa-auth-service".to_string());

    // 2. Connect to Database
    info!("Connecting to database...");
    let db = Database::connect(&config.database.url).await?;
    info!("Database connected successfully");

    // 3. Run Migrations
    info!("Running migrations...");
    migration::Migrator::up(&db, None)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    info!("Migrations completed");

    // 4. Create State
    let state = app_state::AppState::new(db, config.auth.secret, jwt_issuer);

    // 5. Build and Run App - Super Clean!
    EywaApp::new(state)
        .info(
            "TSAHEYLU - Timer Service API",
            "1.0.0",
            "Time tracking service for the EYWA ecosystem. Named after Tsaheylu (The Bond).",
        )
        .tag("Timer", "Timer management endpoints")
        .mount::<TimerController>()
        .health_checks()
        .request_context()
        .request_logging()
        .compression()
        .serve("0.0.0.0:3004")
        .await
}
