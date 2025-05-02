use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod role {
    use crate::auth::permission::{Role, RoleDescription, RoleRow};

    use super::*;

    pub async fn create(
        State(state): State<crate::AppState>,
        Json(role): Json<RoleDescription>,
    ) -> Response {
        let existing_role = sqlx::query_as::<_, (i32,)>("SELECT 1 FROM role WHERE \"name\" = $1")
            .bind(&role.name)
            .fetch_optional(&state.db)
            .await;

        match existing_role {
            Ok(Some(_)) => {
                return StatusCode::CONFLICT.into_response();
            }
            Err(e) => {
                println!("{}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            _ => {}
        }

        let last_id = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO role (\"name\", permissions) VALUES ($1, $2) RETURNING id",
        )
        .bind(&role.name)
        .bind(sqlx::types::Json(&role.permissions))
        .fetch_one(&state.db)
        .await;

        match last_id {
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Ok(id) => {
                return Json(Role {
                    id: id.0,
                    name: role.name,
                    permissions: role.permissions,
                })
                .into_response();
            }
        }
    }
    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        let roles = sqlx::query_as::<_, RoleRow>("SELECT * FROM role LIMIT 1024")
            .fetch_all(&state.db)
            .await;

        match roles {
            Ok(r) => Json(r).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
