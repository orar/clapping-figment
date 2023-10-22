use std::str::FromStr;
use std::time::Duration;
use sqlx::{Error, PgPool};
use sqlx::{ConnectOptions, postgres::{PgConnectOptions, PgPoolOptions}};
use sqlx::migrate::MigrateError;
use crate::config::{AppConfig};

pub async fn create_pool (config: &AppConfig) -> Result<PgPool, Error> {
    let pg_conf = &config.database.postgres;
    let conn_opts = PgConnectOptions::from_str(config.database.postgres.connection_url.as_str())?
        .log_statements(log::LevelFilter::Debug)
        .log_slow_statements(log::LevelFilter::Debug, Default::default());

    PgPoolOptions::default()
        .acquire_timeout(Duration::from_secs(pg_conf.acquire_timeout.unwrap_or(10)))
        .idle_timeout(Duration::from_secs(pg_conf.idle_timeout.unwrap_or(300)))
        .max_connections(pg_conf.max_connections.unwrap_or(10))
        .min_connections(pg_conf.min_connections.unwrap_or(0))
        .test_before_acquire(false)
        .connect_with(conn_opts)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!().run(pool).await
}