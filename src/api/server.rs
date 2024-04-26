use axum::Router;

use crate::app_state::AppState;

pub async fn run(app: &AppState) -> anyhow::Result<()> {
    let router = Router::new();
    let listener = tokio::net::TcpListener::bind(&app.settings.server.address).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
