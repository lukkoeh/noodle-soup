use sqlx::PgPool;

pub mod auth;
pub mod resources;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub media_path: String,
}
