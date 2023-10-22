
mod cli;
mod config;
mod todo;
mod db;
mod run;
mod setup;

use cli::Args;
use clap::Parser;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = config::get_config();
    let pool = db::create_pool(&config).await?; // PgPool::connect(config.database_url.as_str()).await?;

    match args {
        Args::Setup => setup::setup_app(&config, &pool).await?,

        a => run::app(a, &config, &pool).await?
    }

    Ok(())
}
