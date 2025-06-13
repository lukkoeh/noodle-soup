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
    use super::{Course, CourseDescription};
    use axum::{extract::{Path as UrlPath, State}, Json, http::StatusCode, response::{IntoResponse, Response}};

    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, Course>("SELECT uid, name FROM \"course\"")
            .fetch_all(&state.db)
            .await
        {
            Ok(courses) => Json(courses).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn get_by_uid(UrlPath(id): UrlPath<i64>, State(state): State<crate::AppState>) -> Response {
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

    pub async fn create(State(state): State<crate::AppState>, Json(desc): Json<CourseDescription>) -> Response {
        if desc.name.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match sqlx::query_scalar::<_, i64>("INSERT INTO \"course\"(name) VALUES ($1) RETURNING uid")
            .bind(&desc.name)
            .fetch_one(&state.db)
            .await
        {
            Ok(id) => {
                Json(Course { course_id: id, name: desc.name }).into_response()
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn update(
        UrlPath(id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(desc): Json<CourseDescription>,
    ) -> Response {
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
                Json(Course { course_id: id, name: desc.name }).into_response()
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn delete(UrlPath(id): UrlPath<i64>, State(state): State<crate::AppState>) -> StatusCode {
        match sqlx::query("DELETE FROM \"course\" WHERE uid = $1")
            .bind(id)
            .execute(&state.db)
            .await
        {
            Ok(r) => if r.rows_affected() == 0 { StatusCode::NOT_FOUND } else { StatusCode::OK },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
