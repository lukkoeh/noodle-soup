use axum::routing::post;
use axum::{Router, routing::get};
use axum_login::tower_sessions::{ExpiredDeletion, SessionManagerLayer};
use axum_login::{AuthManagerLayerBuilder, login_required};
use dotenv::dotenv;
use libnoodle::AppState;
use libnoodle::{auth, resources, user};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::task::AbortHandle;
use tower_sessions_sqlx_store::PostgresStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<_> = std::env::args().collect();

    let pool_options = PgPoolOptions::new().max_connections(5);

    let db_pool;
    if args.len() > 1 && args[1] == "test" {
        db_pool = pool_options
            .connect(&env::var("PG_TEST_URL").unwrap())
            .await?;
    } else {
        db_pool = pool_options.connect(&env::var("PG_URL").unwrap()).await?;
    }

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
        media_path: env::var("MEDIA_PATH").unwrap(),
    };

    //for testing only
    sqlx::raw_sql("DELETE FROM \"user\"")
        .execute(&db_pool)
        .await?;

    //for testing only
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

    use auth::permission;
    let app = Router::new()
        .route("/user", get(user::http::get_self).post(user::http::create))
        .route("/user/groups", get(user::http::get_self_groups))
        .route("/user/roles", get(user::http::get_self_roles))
        .route(
            "/users/{id}",
            get(user::http::get)
                .patch(user::http::update)
                .delete(user::http::delete),
        )
        .route(
            "/users/{id}/groups",
            get(user::http::get_groups)
                .put(user::http::replace_groups)
                .post(user::http::add_to_groups)
                .delete(user::http::remove_from_groups),
        )
        .route(
            "/users/{id}/roles",
            get(user::http::get_roles)
                .put(user::http::replace_roles) //TODO: Auch in entsprechender Gruppe
                .post(user::http::assign_roles)
                .delete(user::http::unassign_roles),
        )
        .route(
            "/roles",
            get(permission::http::role::get_all).post(permission::http::role::create),
        )
        .route(
            "/roles/{id}",
            get(permission::http::role::get_by_id)
                .patch(permission::http::role::update)
                .delete(permission::http::role::delete),
        )
        .route(
            "/roles/{id}/users",
            get(permission::http::role::get_users)
                .put(permission::http::role::replace_users)
                .post(permission::http::role::add_users)
                .delete(permission::http::role::delete_users),
        )
        .route(
            "/groups",
            get(permission::http::group::get_all).post(permission::http::group::create),
        )
        .route(
            "/groups/{id}",
            get(permission::http::group::get_by_id)
                .patch(permission::http::group::update)
                .delete(permission::http::group::delete),
        )
        .route(
            "/groups/{id}/users",
            get(permission::http::group::get_users)
                .put(permission::http::group::replace_users)
                .post(permission::http::group::add_users)
                .delete(permission::http::group::delete_users),
        )
        .route(
            "/files",
            get(resources::file::http::get_all).post(resources::file::http::create),
        )
        .route(
            "/file/{uid}",
            get(resources::file::http::get_by_uid)
                .put(resources::file::http::update)
                .delete(resources::file::http::delete),
        )
        .route(
            "/design",
            get(resources::branding::http::get).post(resources::branding::http::create_default),
        )
        .route(
            "/courses",
            get(resources::course::http::get_all).post(resources::course::http::create),
        )
        .route(
            "/course/{id}",
            get(resources::course::http::get_by_uid)
                .put(resources::course::http::update)
                .delete(resources::course::http::delete),
        )
        .route(
            "/course/{courseId}/sections",
            get(resources::content_section::http::get_sections)
                .post(resources::content_section::http::create_section),
        )
        .route(
            "/course/{courseId}/section/{sectionId}",
            get(resources::content_section::http::get_section)
                .put(resources::content_section::http::update_section)
                .delete(resources::content_section::http::delete_section),
        )
        .route(
            "/course/{courseId}/section/{sectionId}/content",
            get(resources::content_section::http::get_content)
                .post(resources::content_section::http::create_content)
                .put(resources::content_section::http::update_content)
                .delete(resources::content_section::http::delete_content),
        )
        .route(
            "/templates",
            get(resources::template::http::get_all).post(resources::template::http::create),
        )
        .route(
            "/template/{id}",
            get(resources::template::http::get_by_uid)
                .put(resources::template::http::update)
                .delete(resources::template::http::delete),
        )
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
