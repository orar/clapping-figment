use serde::Deserialize;

use figment::{Figment, providers::{Format, Toml, Env}};

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub package: AppPackage,
    pub database: Database,
}

#[derive(Deserialize, Debug)]
pub struct AppPackage {
    name: String,

    authors: Vec<String>,
    publish: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub postgres: PostgresDBConfig
}

#[derive(Deserialize, Debug)]
pub struct PostgresDBConfig {
    pub connection_url: String,
    pub min_connections: Option<u32>,
    pub max_connections: Option<u32>,
    pub idle_timeout: Option<u64>,
    pub acquire_timeout: Option<u64>,
}


pub fn get_config() -> AppConfig {
    Figment::new()
        .merge(Toml::file("Cargo.toml"))
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("CARGO_"))
        .merge(Env::raw().only(&["RUSTC", "RUSTDOC"]))
        .extract()
        .unwrap()

}
