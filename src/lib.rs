use sqlx::PgPool;

pub mod auth;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(serde::Serialize)]
pub struct Message {
    message: String,
}
