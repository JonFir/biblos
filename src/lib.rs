use app_state::AppState;

mod api;
mod app_state;

pub async fn run() -> anyhow::Result<()> {
    let app = AppState::new().await?;
    api::server::run(&app).await?;
    Ok(())
}
