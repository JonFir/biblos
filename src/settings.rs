use std::fs;

use anyhow::Context;
use serde::Deserialize;

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
