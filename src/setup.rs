
use sqlx::PgPool;
use crate::config::AppConfig;
use crate::db::run_migrations;

pub async fn setup_app(_config: &AppConfig, pool: &PgPool) -> anyhow::Result<()> {
    //
    run_migrations(pool).await?;
    println!("Running migrations: Completed");


    Ok(())
}