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

pub mod group {
    use axum::extract::Path;

    use super::*;
    use crate::{
        auth::permission::{GroupDescription, GroupRow, add_user_to_group_query},
        user,
    };

    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        let groups = sqlx::query_as::<_, GroupRow>("SELECT * FROM \"group\" LIMIT 1024")
            .fetch_all(&state.db)
            .await;

        match groups {
            Ok(r) => Json(r).into_response(),
            Err(e) => {
                println!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    pub async fn get_by_id(Path(id): Path<i64>, State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, GroupRow>("SELECT * FROM \"group\" WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(g)) => Json(g).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn get_users(Path(id): Path<i64>, State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, user::Profile>("SELECT \"user\".id, firstname, lastname, email FROM \"user\" LEFT JOIN \"user_in_group\" ON id = user_id WHERE group_id = $1").bind(id).fetch_all(&state.db).await
        {
            Ok(u) => Json(u).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    pub async fn create(
        State(state): State<crate::AppState>,
        Json(group): Json<GroupDescription>,
    ) -> Response {
        let existing_group =
            sqlx::query_as::<_, (i32,)>("SELECT 1 FROM \"group\" WHERE \"name\" = $1")
                .bind(&group.name)
                .fetch_optional(&state.db)
                .await;

        match existing_group {
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
            "INSERT INTO \"group\" (\"name\", kind, parent) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(&group.name)
        .bind(&group.kind)
        .bind(group.parent)
        .fetch_one(&state.db)
        .await;

        match last_id {
            Err(e) => {
                println!("{}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Ok(id) => {
                return Json(GroupRow {
                    id: id.0,
                    name: group.name,
                    kind: group.kind,
                    parent: group.parent,
                })
                .into_response();
            }
        }
    }

    pub async fn add_users(
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(ids): Json<Vec<i64>>,
    ) -> StatusCode {
        match sqlx::query_as::<_, (i32,)>("SELECT 1 FROM \"group\" WHERE id = $1")
            .bind(group_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                if ids.len() < 1 {
                    return StatusCode::BAD_REQUEST;
                }
                if ids.len() == 1 {
                    if let Err(_) = sqlx::query(
                        "INSERT INTO \"user_in_group\"(user_id, group_id) VALUES ($1, $2)",
                    )
                    .bind(ids[0])
                    .bind(group_id)
                    .execute(&state.db)
                    .await
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }
                    return StatusCode::CREATED;
                }

                let mut group_ids = Vec::with_capacity(ids.len());
                group_ids.fill(group_id);
                if let Err(_) = add_user_to_group_query()
                    .bind(&ids)
                    .bind(group_ids)
                    .execute(&state.db)
                    .await
                {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
                return StatusCode::CREATED;
            }
            Ok(None) => StatusCode::NOT_FOUND,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn replace_users(
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(ids): Json<Vec<i64>>,
    ) -> Response {
        match sqlx::query_as::<_, (i32,)>("SELECT 1 FROM \"group\" WHERE id = $1")
            .bind(group_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                if ids.len() == 0 {
                    return StatusCode::BAD_REQUEST.into_response();
                } else {
                    let Ok(mut transaction) = state.db.begin().await else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    if let Err(_) = sqlx::query("DELETE FROM \"user_in_group\" WHERE group_id = $1")
                        .bind(ids[0])
                        .execute(&mut *transaction)
                        .await
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                    if ids.len() == 1 {
                        if let Err(_) = sqlx::query(
                            "INSERT INTO \"user_in_group\"(user_id, group_id) VALUES($1, $2)",
                        )
                        .bind(ids[0])
                        .bind(group_id)
                        .execute(&mut *transaction)
                        .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                        Json(ids).into_response()
                    } else {
                        let mut group_ids = Vec::with_capacity(ids.len());
                        group_ids.fill(group_id);
                        if let Err(_) = add_user_to_group_query()
                            .bind(&ids)
                            .bind(&group_ids)
                            .execute(&mut *transaction)
                            .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }

                        if let Err(_) = transaction.commit().await {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                        Json(ids).into_response()
                    }
                }
            }
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn delete(
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        match sqlx::query("DELETE FROM \"group\" WHERE id = $1")
            .bind(group_id)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn delete_users(
        Path(id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        match sqlx::query("DELETE FROM \"user_in_group\" WHERE group_id = $1 AND user_id = ANY($2)")
            .bind(id)
            .bind(user_ids)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
