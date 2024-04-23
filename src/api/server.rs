use axum::Router;

use crate::settings::Settings;

pub async fn run(settings: &Settings) -> anyhow::Result<()> {
    let app = Router::new();
    let listener = tokio::net::TcpListener::bind(&settings.server.address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
