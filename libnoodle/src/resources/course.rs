use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    #[sqlx(rename = "uid")]
    pub course_id: i64,
    pub name: String,
    pub shortname: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDescription {
    pub name: String,
    pub shortname: String,
}

pub mod http {
    use crate::{
        auth::{
            self,
            permission::{Operations, PermissionQueryParam},
        },
        resources,
        user::Profile,
    };

    use super::{Course, CourseDescription};
    use axum::{
        Json,
        extract::{Path as UrlPath, Query, State},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use axum_login::AuthSession;

    pub async fn get_all(
        auth_session: AuthSession<auth::Backend>,
        State(state): State<crate::AppState>,
        Query(perm): Query<PermissionQueryParam>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        let ops = if let Some(true) = perm.edit {
            Operations::UPDATE
        } else {
            Operations::READ
        };
        let courses = match auth::user_has_permissions_all(
            resources::Type::Course,
            ops,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Ok(true) => {
                match sqlx::query_as::<_, Course>("SELECT uid, name, shortname FROM \"course\"")
                    .fetch_all(&state.db)
                    .await
                {
                    Ok(c) => c,
                    Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                }
            }
            Ok(false) => match sqlx::query_as::<_, Course>(
                "SELECT c.uid, c.name, c.shortname \
FROM course c \
JOIN course_permissions cp ON c.uid = cp.resource_id \
WHERE (cp.user_id = $1 OR EXISTS(\
SELECT 1 FROM user_has_role ur \
WHERE ur.user_id = $1 AND ur.role_id = cp.role_id)) \
AND (cp.permission & $2::int::bit(16)) <> B'0'::bit(16)",
            )
            .bind(s_user.user_id)
            .bind(ops)
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

        match sqlx::query_as::<_, Course>(
            "SELECT uid, name, shortname FROM \"course\" WHERE uid = $1",
        )
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
        match sqlx::query_scalar::<_, i64>(
            "INSERT INTO \"course\"(name, shortname) VALUES ($1, $2) RETURNING uid",
        )
        .bind(&desc.name)
        .bind(&desc.shortname)
        .fetch_one(&state.db)
        .await
        {
            Ok(id) => Json(Course {
                course_id: id,
                name: desc.name,
                shortname: desc.shortname,
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
        match sqlx::query("UPDATE \"course\" SET \"name\" = $1, shortname = $2 WHERE uid = $3")
            .bind(&desc.name)
            .bind(&desc.shortname)
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
                    shortname: desc.shortname,
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

    pub async fn get_lecturers(
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
        }

        match sqlx::query_as::<_, (String, String)>("SELECT u.firstname, u.lastname FROM \"user\" AS u JOIN course_lecturer cl ON cl.user_id = u.id WHERE cl.course_id = $1")
            .bind(id)
            .fetch_all(&state.db)
            .await
        {
            Ok(n) => Json(n.iter().map(|v| {format!("{} {}", v.0, v.1)}).collect::<Vec<String>>()).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn add_lecturers(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        match sqlx::query("INSERT INTO course_lecturer SELECT * FROM UNNEST(array_fill($1, ARRAY[array_length($2::int8[], 1)]), $2::int8[])")
            .bind(course_id)
            .bind(user_ids)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn set_lecturers(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        let Ok(mut tx) = state.db.begin().await else {
            return StatusCode::INTERNAL_SERVER_ERROR;
        };

        match sqlx::query("DELETE FROM course_lecturer WHERE course_id = $1")
            .bind(course_id)
            .execute(&mut *tx)
            .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(_) => {}
        };

        match sqlx::query("INSERT INTO course_lecturer(course_id, user_id) SELECT * FROM UNNEST(array_fill($1, ARRAY[array_length($2::int8[], 1)]), $2::int8[])")
            .bind(course_id)
            .bind(user_ids)
            .execute(&mut *tx).await {
            Ok(_) => match tx.commit().await {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn get_groups(
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
        }

        match sqlx::query_scalar::<_, String>("SELECT g.name FROM \"group\" AS g JOIN course_group cg ON cg.group_id = g.id WHERE cg.course_id = $1")
            .bind(id)
            .fetch_all(&state.db)
            .await
        {
            Ok(n) => Json(n).into_response(),
            Err(e) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn add_groups(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(group_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        match sqlx::query("INSERT INTO course_group(course_id, group_id) SELECT * FROM UNNEST(array_fill($1, ARRAY[array_length($2::int8[], 1)]), $2::int8[])")
            .bind(course_id)
            .bind(group_ids)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn set_groups(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(group_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        let Ok(mut tx) = state.db.begin().await else {
            return StatusCode::INTERNAL_SERVER_ERROR;
        };

        match sqlx::query("DELETE FROM course_group WHERE course_id = $1")
            .bind(course_id)
            .execute(&mut *tx)
            .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(_) => {}
        };

        match sqlx::query("INSERT INTO course_group(course_id, group_id) SELECT * FROM UNNEST(array_fill($1, ARRAY[array_length($2::int8[], 1)]), $2::int8[])")
            .bind(course_id)
            .bind(group_ids)
            .execute(&mut *tx).await {
            Ok(_) => match tx.commit().await {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn get_users(
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
        }

        match sqlx::query_as::<_, Profile>(
            "SELECT u.id, u.firstname, u.lastname, u.title, u.email FROM \"user\" u \
LEFT JOIN course_user cu ON cu.user_id = u.id \
WHERE cu.course_id = $1
OR u.id IN (SELECT user_id FROM user_in_group uig JOIN course_group cg ON cg.group_id = uig.group_id WHERE cg.course_id = $1)",
        )
        .bind(id)
        .bind(Operations::READ)
        .fetch_all(&state.db)
        .await
        {
            Ok(p) => Json(p).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
            
        }
    }

    pub async fn add_users(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        match sqlx::query(
            "INSERT INTO course_user(user_id, course_id) \
SELECT * FROM UNNEST($1::int8[], \
array_fill($2, ARRAY[array_length($1::int8[], 1)]))",
        )
        .bind(user_ids)
        .bind(course_id)
        .bind(Operations::READ)
        .execute(&state.db)
        .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    pub async fn set_users(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        let Ok(mut tx) = state.db.begin().await else {
            return StatusCode::INTERNAL_SERVER_ERROR;
        };

        match sqlx::query(
            "DELETE FROM course_permissions WHERE course_id = $1 AND permission = $2::int::bit(16)",
        )
        .bind(course_id)
        .bind(Operations::READ)
        .execute(&mut *tx)
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(_) => {}
        };

        match sqlx::query(
            "INSERT INTO course_user(user_id, course_id) \
SELECT * FROM UNNEST($1::int8[], \
array_fill($2::int::bit(16), ARRAY[array_length($1::int8[], 1)]))",
        )
        .bind(course_id)
        .bind(user_ids)
        .execute(&mut *tx)
        .await
        {
            Ok(_) => match tx.commit().await {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
