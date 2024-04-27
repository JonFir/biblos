use std::sync::Arc;

use axum::Router;

use crate::app_state::AppState;

pub async fn run(app: Arc<AppState>) -> anyhow::Result<()> {
    let router = Router::<Arc<AppState>>::new()
        .nest("/user", crate::auth::make_routes())
        .with_state(Arc::clone(&app));
    let listener = tokio::net::TcpListener::bind(&app.settings.server.address).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
