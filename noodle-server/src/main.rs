use axum::routing::{delete, post};
use axum::{Router, routing::get};
use axum_login::tower_sessions::{ExpiredDeletion, SessionManagerLayer};
use axum_login::{AuthManagerLayerBuilder, login_required};
use dotenv::dotenv;
use libnoodle::AppState;
use libnoodle::{auth, resources, user};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::task::AbortHandle;
use tower_http::cors::{Any, CorsLayer};
use tower_sessions_sqlx_store::PostgresStore;

async fn migrate_test(db_pool: &PgPool) {
    sqlx::raw_sql("DELETE FROM \"user\"")
        .execute(db_pool)
        .await
        .unwrap();

    let main_user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO \"user\" (firstname, lastname, title, email, password) VALUES ($1, $2, $3, $4, $5) RETURNING id",
    )
    .bind(&env::var("ADMIN_FIRSTNAME").unwrap())
    .bind(&env::var("ADMIN_LASTNAME").unwrap())
    .bind(&env::var("ADMIN_TITLE").unwrap())
    .bind(&env::var("ADMIN_MAIL").unwrap())
    .bind(&bcrypt::hash(&env::var("ADMIN_PASSWORD").unwrap(), bcrypt::DEFAULT_COST).unwrap())
    .fetch_one(db_pool)
    .await
    .unwrap();

    sqlx::raw_sql(&format!(
        "INSERT INTO user_has_role (user_id, role_id) VALUES ('{}', (SELECT id FROM \"role\" WHERE \"role\".name = 'admin'));",
        main_user_id
    ))
    .execute(db_pool)
    .await
    .unwrap();
}

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

    //testing only
    migrate_test(&db_pool).await;

    let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 3000))).await?;

    use auth::permission;
    let app = Router::new()
        .route("/user", get(user::http::get_self).post(user::http::create))
        .route("/user/groups", get(user::http::get_self_groups))
        .route("/user/roles", get(user::http::get_self_roles))
        .route("/users", get(user::http::get_all))
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
                .put(user::http::replace_roles)
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
            "/course/{courseId}/lecturers",
            get(resources::course::http::get_lecturers)
                .post(resources::course::http::add_lecturers)
                .put(resources::course::http::set_lecturers),
        )
        .route(
            "/course/{courseId}/groups",
            get(resources::course::http::get_groups)
                .post(resources::course::http::add_groups)
                .put(resources::course::http::set_groups),
        )
        .route(
            "/course/{courseId}/users",
            get(resources::course::http::get_users)
                .post(resources::course::http::add_users)
                .put(resources::course::http::set_users),
        )
        .route(
            "/course/{courseId}/users/{userId}",
            delete(resources::course::http::delete_user),
        )
        .route(
            "/course/{courseId}/sections",
            get(resources::content_section::http::get_all_for_course)
                .post(resources::content_section::http::create_for_course),
        )
        .route(
            "/course/{courseId}/section/{sectionId}",
            get(resources::content_section::http::get_for_course)
                .put(resources::content_section::http::update_for_course)
                .delete(resources::content_section::http::delete_for_course),
        )
        .route(
            "/course/{courseId}/section/{sectionId}/content",
            get(resources::content_section::http::get_course_content)
                .post(resources::content_section::http::create_course_content)
                .put(resources::content_section::http::update_course_content)
                .delete(resources::content_section::http::delete_course_content),
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
        .route(
            "/template/{templateId}/sections",
            get(resources::content_section::http::get_all_for_template)
                .post(resources::content_section::http::create_for_template),
        )
        .route(
            "/template/{templateId}/section/{sectionId}",
            get(resources::content_section::http::get_for_template)
                .put(resources::content_section::http::update_for_template)
                .delete(resources::content_section::http::delete_for_template),
        )
        .route(
            "/template/{templateId}/section/{sectionId}/content",
            get(resources::content_section::http::get_template_content)
                .post(resources::content_section::http::create_template_content)
                .put(resources::content_section::http::update_template_content)
                .delete(resources::content_section::http::delete_template_content),
        )
        .route_layer(login_required!(auth::Backend))
        //NOTE: potentially temporary
        .route("/login", post(auth::create_session_handler))
        .layer(
            tower::ServiceBuilder::new().layer(auth_layer).layer(
                CorsLayer::new()
                    .allow_methods(Any)
                    .allow_origin(Any)
                    .allow_headers(Any),
            ),
        )
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
