use std::sync::Arc;

use app_state::AppState;

mod api;
mod app_state;
pub(crate) mod auth;
mod entity;

pub async fn run() -> anyhow::Result<()> {
    let app = AppState::new().await?;
    let app = Arc::new(app);
    api::server::run(app).await?;
    Ok(())
}
