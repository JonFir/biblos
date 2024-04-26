use std::{env, fs};

use anyhow::{Context, Ok};
use serde::Deserialize;
use sqlx::SqlitePool;

pub struct AppState {
    pub settings: Settings,
    pub db_pool: SqlitePool,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let settings = Settings::load()?;
        let db_pool = Self::open_db_pool().await?;
        let app = AppState { settings, db_pool };
        Ok(app)
    }

    async fn open_db_pool() -> anyhow::Result<SqlitePool> {
        let db_url =
            env::var("DATABASE_URL").context("Fail to get database url from environment")?;
        let db_pool = SqlitePool::connect(&db_url)
            .await
            .context("Failt to conect sqlite db")?;
        Ok(db_pool)
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let path = std::env::var("SETTINGS_FILE")
            .context("Fail to get settings file path from environment")?;
        let settings = fs::read_to_string(path).context("Fail to read settings file")?;
        let settings = toml::from_str(&settings).context("Fail parse settings toml string")?;
        Ok(settings)
    }
}
