use std::sync::Arc;

use anyhow::Ok;
use axum::{extract::State, routing::post, Router};

use crate::{app_state::AppState, entity::User};

async fn store_user(state: &AppState, user: User) -> anyhow::Result<User> {
    let result = sqlx::query!(
        "
        INSERT INTO users (username, email, boosty_email, boosty_level, github_nikname, password, last_login, date_joined, is_superuser, is_staff, is_active)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ", user.username, user.email, user.boosty_email, user.boosty_level, user.github_nikname, user.password, user.last_login, user.date_joined, user.is_superuser, user.is_staff, user.is_active
    ).execute(&state.db_pool).await?;
    let id = result.last_insert_rowid();
    let user = User {
        id: Some(id),
        ..user
    };
    Ok(user)
}

pub fn make_routes() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register_handler))
}

async fn register_handler(State(state): State<Arc<AppState>>) {
    let user = User {
        id: None,
        username: "test".to_owned(),
        email: "test@test.ru".to_owned(),
        boosty_email: "boo@dd.ru".to_owned(),
        boosty_level: "Мидл".to_owned(),
        github_nikname: "gi".to_owned(),
        password: "1234567".to_owned(),
        last_login: None,
        date_joined: chrono::offset::Utc::now(),
        is_superuser: false,
        is_staff: false,
        is_active: false,
    };
    let user = store_user(&state, user).await.unwrap();
}
