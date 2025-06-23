use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    #[sqlx(rename = "uid")]
    pub course_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDescription {
    pub name: String,
}

pub mod http {
    use crate::{
        auth::{self, permission::Operations},
        resources,
    };

    use super::{Course, CourseDescription};
    use axum::{
        Json,
        extract::{Path as UrlPath, State},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use axum_login::AuthSession;

    pub async fn get_all(
        auth_session: AuthSession<auth::Backend>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        let courses = match auth::user_has_permissions_all(
            resources::Type::Course,
            Operations::READ,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Ok(true) => match sqlx::query_as::<_, Course>("SELECT uid, name FROM \"course\"")
                .fetch_all(&state.db)
                .await
            {
                Ok(c) => c,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
            Ok(false) => match sqlx::query_as::<_, Course>(
                "SELECT c.uid, c.name \
FROM course c \
JOIN course_permissions cp ON c.uid = cp.resource_id \
WHERE (cp.user_id = $1 OR EXISTS(\
SELECT 1 FROM user_has_role ur \
WHERE ur.user_id = $1 AND ur.role_id = cp.role_id)) \
AND (cp.permission & $2::int::bit(16)) <> B'0'::bit(16)",
            )
            .bind(s_user.user_id)
            .bind(Operations::READ)
            .fetch_all(&state.db)
            .await
            {
                Ok(c) => c,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        Json(courses).into_response()
    }

    pub async fn get_by_uid(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(id): UrlPath<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &id,
            Operations::READ,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        match sqlx::query_as::<_, Course>("SELECT uid, name FROM \"course\" WHERE uid = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(course)) => Json(course).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn create(
        auth_session: AuthSession<auth::Backend>,
        State(state): State<crate::AppState>,
        Json(desc): Json<CourseDescription>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::can_create(resources::Type::Course, s_user.user_id, &state.db).await {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        }

        if desc.name.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match sqlx::query_scalar::<_, i64>("INSERT INTO \"course\"(name) VALUES ($1) RETURNING uid")
            .bind(&desc.name)
            .fetch_one(&state.db)
            .await
        {
            Ok(id) => Json(Course {
                course_id: id,
                name: desc.name,
            })
            .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn update(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(desc): Json<CourseDescription>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        }

        if desc.name.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match sqlx::query("UPDATE \"course\" SET name = $1 WHERE uid = $2")
            .bind(&desc.name)
            .bind(id)
            .execute(&state.db)
            .await
        {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return StatusCode::NOT_FOUND.into_response();
                }
                Json(Course {
                    course_id: id,
                    name: desc.name,
                })
                .into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn delete(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(id): UrlPath<i64>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::can_delete(resources::Type::Course, s_user.user_id, &state.db).await {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        match sqlx::query("DELETE FROM \"course\" WHERE uid = $1")
            .bind(id)
            .execute(&state.db)
            .await
        {
            Ok(r) => {
                if r.rows_affected() == 0 {
                    StatusCode::NOT_FOUND
                } else {
                    StatusCode::OK
                }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
