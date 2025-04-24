use axum::routing::{delete, patch, post};
use axum::{Router, routing::get};
use axum_login::tower_sessions::{ExpiredDeletion, SessionManagerLayer};
use axum_login::{AuthManagerLayerBuilder, login_required};
use dotenv::dotenv;
use noodle_soup::AppState;
use noodle_soup::{auth, user};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::task::AbortHandle;
use tower_sessions_sqlx_store::PostgresStore;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("PG_URL").unwrap())
        .await?;

    let session_store = PostgresStore::new(db_pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("BLOODLESSNESS")
        .with_expiry(axum_login::tower_sessions::Expiry::OnInactivity(
            time::Duration::days(1),
        ));
    let auth_backend = auth::Backend::new(db_pool.clone()).await;
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    let app_state = AppState {
        db: db_pool.clone(),
    };

    sqlx::raw_sql("DELETE FROM \"user\"")
        .execute(&db_pool)
        .await?;

    sqlx::raw_sql(&format!(
        "INSERT INTO \"user\" (firstname, lastname, email, password) VALUES ('{}', '{}', '{}', '{}');",
        &env::var("ADMIN_FIRSTNAME").unwrap(),
        &env::var("ADMIN_LASTNAME").unwrap(),
        &env::var("ADMIN_MAIL").unwrap(),
        &bcrypt::hash(&env::var("ADMIN_PASSWORD").unwrap(), bcrypt::DEFAULT_COST).unwrap()
    ))
    .execute(&db_pool)
    .await?;

    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).await?;

    let app = Router::new()
        .route("/user", get(user::get_self_handler))
        .route("/user", post(user::create_user_handler))
        .route("/user/{id}", get(user::get_user_handler))
        .route("/user/{id}", patch(user::change_user_handler))
        .route("/user/{id}", delete(user::delete_user_handler))
        .route_layer(login_required!(auth::Backend))
        //NOTE: potentially temporary
        .route("/login", post(auth::create_session_handler))
        .layer(auth_layer)
        .with_state(app_state);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install ctrl-c handler.")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler.")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
