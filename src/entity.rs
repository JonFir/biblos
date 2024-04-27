use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub boosty_email: String,
    pub boosty_level: String,
    pub github_nikname: String,
    pub password: String,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_active: bool,
}
