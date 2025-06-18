use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub mod role {
    use axum::extract::Path;
    use sqlx::error::ErrorKind;

    use crate::auth::permission::{Role, RoleDescription, RoleRow, add_users_to_role_group_query};
    use crate::auth::permission::{add_users_to_groups_query, add_users_to_roles_query};
    use crate::user;

    use super::*;

    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        let roles = sqlx::query_as::<_, RoleRow>("SELECT * FROM role LIMIT 1024")
            .fetch_all(&state.db)
            .await;

        match roles {
            Ok(r) => Json(r).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

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
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            _ => {}
        }

        match sqlx::query_scalar::<_, i64>(
            "WITH inserted_group AS (INSERT INTO \"group\" (\"name\", kind, parent) VALUES ($1, 'role', $2) RETURNING id) \
INSERT INTO \"role\"(name, permissions, \"group\") \
VALUES ($3, $4, (SELECT id FROM inserted_group))\
RETURNING id",
        )
        .bind(&role.name)
        .bind::<Option<i64>>(None)
        .bind(&role.name)
        .bind(sqlx::types::Json(&role.permissions))
        .fetch_one(&state.db)
        .await
        {
            Ok(id) => (StatusCode::CREATED, Json(Role {
                role_id: id,
                name: role.name,
                permissions: role.permissions,
            }))
            .into_response(),
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    if e.kind() == ErrorKind::UniqueViolation {
                        StatusCode::CONFLICT.into_response()
                    } else {
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response()

            },
        }

        // let group_id =
        //     match GroupRow::create_in_db(&state.db, &role.name, &GroupKind::Organization, None)
        //         .await
        //     {
        //         Ok(id) => id,
        //         Err(e) => match e {
        //             ResourceCreateError::Conflict => return StatusCode::CONFLICT.into_response(),
        //             ResourceCreateError::BadParam => {
        //                 return StatusCode::BAD_REQUEST.into_response();
        //             }
        //             _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        //         },
        //     };

        // match RoleRow::create_in_db(&state.db, &role.name, &role.permissions, Some(group_id)).await
        // {
        //     Ok(id) => Json(Role {
        //         id,
        //         name: role.name,
        //         permissions: role.permissions,
        //     })
        //     .into_response(),
        //     Err(e) => match e {
        //         ResourceCreateError::Conflict => StatusCode::CONFLICT.into_response(),
        //         ResourceCreateError::BadParam => StatusCode::BAD_REQUEST.into_response(),
        //         _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        //     },
        // }
    }

    pub async fn get_by_id(Path(id): Path<i64>, State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, RoleRow>("SELECT * FROM \"role\" WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(r)) => Json(r).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn update(
        Path(role_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(role): Json<RoleDescription>,
    ) -> Response {
        match sqlx::query_scalar::<_, i32>("SELECT 1 FROM \"role\" WHERE id = $1")
            .bind(role_id)
            .fetch_optional(&state.db)
            .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(None) => return StatusCode::NOT_FOUND.into_response(),
            Ok(Some(_)) => {}
        }

        //TODO: Update role group aswell
        match sqlx::query_scalar::<_, i64>(
            "WITH updated_role AS (UPDATE \"role\" SET \"name\" = $1, permissions = $2 WHERE id = $3 RETURNING \"group\") \
UPDATE \"group\" SET \"name\" = $1 WHERE id = (SELECT \"group\" FROM updated_role)",
        )
        .bind(&role.name)
        .bind(sqlx::types::Json(&role.permissions))
        .bind(role_id)
        .fetch_optional(&state.db)
        .await
        {
            Ok(g) => Json(RoleRow {
                role_id,
                name: role.name,
                permissions: sqlx::types::Json(role.permissions),
                group: g,
            })
            .into_response(),
            Err(e) => match e {
                sqlx::Error::Database(f) => match f.kind() {
                    ErrorKind::UniqueViolation => StatusCode::CONFLICT.into_response(),
                    _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    }

    pub async fn delete(
        Path(role_id): Path<i64>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        let group_to_delete = match sqlx::query_as::<_, (i64,)>(
            "DELETE FROM \"role\" WHERE id = $1 RETURNING \"group\"",
        )
        .bind(role_id)
        .fetch_optional(&state.db)
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(None) => return StatusCode::NOT_FOUND,
            Ok(Some(g)) => g.0,
        };

        match sqlx::query("DELETE FROM \"group\" WHERE id = $1")
            .bind(group_to_delete)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn get_users(
        Path(role_id): Path<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        match sqlx::query_as::<_, user::Profile>(
            "SELECT \"user\".id, firstname, lastname, email FROM \"user\" \
LEFT JOIN \"user_has_role\" ON id = user_id \
WHERE role_id = $1",
        )
        .bind(role_id)
        .fetch_all(&state.db)
        .await
        {
            Ok(u) => Json(u).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn replace_users(
        Path(role_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> Response {
        match sqlx::query_scalar::<_, i32>("SELECT 1 FROM \"role\" WHERE id = $1")
            .bind(role_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                if user_ids.len() == 0 {
                    return StatusCode::BAD_REQUEST.into_response();
                } else {
                    let Ok(mut transaction) = state.db.begin().await else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };

                    let Ok(users_to_remove) = sqlx::query_scalar::<_, i64>(
                        "DELETE FROM \"user_has_role\" WHERE role_id = $1 RETURNING user_id",
                    )
                    .bind(role_id)
                    .fetch_all(&mut *transaction)
                    .await
                    else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };

                    if sqlx::query("DELETE FROM \"user_in_group\" WHERE user_id = ANY($1)")
                        .bind(&users_to_remove)
                        .execute(&mut *transaction)
                        .await
                        .is_err()
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }

                    if user_ids.len() == 1 {
                        if let Err(_) = sqlx::query(
                            "INSERT INTO \"user_has_role\"(user_id, role_id) VALUES($1, $2)",
                        )
                        .bind(user_ids[0])
                        .bind(role_id)
                        .execute(&mut *transaction)
                        .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }

                        if let Err(_) = sqlx::query("INSERT INTO \"user_in_group\"(user_id, group_id) VALUES ($1, (SELECT \"group\" FROM \"role\" WHERE id = $2))")
                            .bind(user_ids[0]).bind(role_id).execute(&mut *transaction).await {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    } else {
                        let role_ids = vec![role_id; user_ids.len()];

                        if let Err(_) = add_users_to_roles_query()
                            .bind(&user_ids)
                            .bind(&role_ids)
                            .execute(&mut *transaction)
                            .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }

                        match add_users_to_role_group_query()
                            .bind(&user_ids)
                            .bind(role_id)
                            .execute(&mut *transaction)
                            .await
                        {
                            Err(_) => {
                                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                            }
                            _ => {}
                        }
                    }

                    if let Err(_) = transaction.commit().await {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                    Json(user_ids).into_response()
                }
            }
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn add_users(
        Path(role_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        if user_ids.len() < 1 {
            return StatusCode::BAD_REQUEST;
        }
        match sqlx::query_scalar::<_, i32>("SELECT 1 FROM \"role\" WHERE id = $1")
            .bind(role_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                let Ok(mut transaction) = state.db.begin().await else {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                };
                if user_ids.len() == 1 {
                    if sqlx::query(
                        "INSERT INTO \"user_has_role\"(user_id, role_id) VALUES ($1, $2)",
                    )
                    .bind(user_ids[0])
                    .bind(role_id)
                    .execute(&mut *transaction)
                    .await
                    .is_err()
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }

                    if sqlx::query(
                        "INSERT INTO \"user_in_group\"(user_id, group_id) VALUES ($1, (SELECT \"group\" FROM \"role\" WHERE id = $2))",
                    )
                    .bind(user_ids[0])
                    .bind(role_id)
                    .execute(&mut *transaction)
                    .await.is_err()
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }

                    if transaction.commit().await.is_err() {
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }
                    return StatusCode::CREATED;
                }

                let role_ids = vec![role_id; user_ids.len()];

                if add_users_to_groups_query()
                    .bind(&user_ids)
                    .bind(&role_ids)
                    .execute(&mut *transaction)
                    .await
                    .is_err()
                {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }

                if add_users_to_role_group_query()
                    .bind(user_ids)
                    .bind(role_id)
                    .execute(&mut *transaction)
                    .await
                    .is_err()
                {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }

                if transaction.commit().await.is_err() {
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
                StatusCode::CREATED
            }
            Ok(None) => StatusCode::NOT_FOUND,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn delete_users(
        Path(role_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        let Ok(mut transaction) = state.db.begin().await else {
            return StatusCode::INTERNAL_SERVER_ERROR;
        };

        if sqlx::query("DELETE FROM \"user_has_role\" WHERE role_id = $1 AND user_id = ANY($2)")
            .bind(role_id)
            .bind(&user_ids)
            .execute(&state.db)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }

        if sqlx::query("DELETE FROM \"user_in_group\" WHERE user_id = ANY($1)")
            .bind(user_ids)
            .execute(&mut *transaction)
            .await
            .is_err()
        {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        match transaction.commit().await {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub mod group {
    use axum::extract::Path;
    use axum_login::AuthSession;

    use super::*;
    use crate::{
        auth::{
            self,
            permission::{GroupDescription, GroupRow, Operations, add_users_to_groups_query},
        },
        resources, user,
    };

    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        let groups = sqlx::query_as::<_, GroupRow>("SELECT * FROM \"group\" LIMIT 1024")
            .fetch_all(&state.db)
            .await;

        match groups {
            Ok(r) => Json(r).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn create(
        auth_session: AuthSession<auth::Backend>,
        State(state): State<crate::AppState>,
        Json(group): Json<GroupDescription>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::can_create(resources::Type::Group, s_user.user_id, &state.db).await {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        }

        let existing_group =
            sqlx::query_as::<_, (i32,)>("SELECT 1 FROM \"group\" WHERE \"name\" = $1")
                .bind(&group.name)
                .fetch_optional(&state.db)
                .await;

        match existing_group {
            Ok(Some(_)) => {
                return StatusCode::CONFLICT.into_response();
            }
            Err(_) => {
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
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Ok(id) => {
                return (
                    StatusCode::CREATED,
                    Json(GroupRow {
                        group_id: id.0,
                        name: group.name,
                        kind: group.kind,
                        parent: group.parent,
                    }),
                )
                    .into_response();
            }
        }
    }

    pub async fn get_by_id(
        auth_session: AuthSession<auth::Backend>,
        Path(id): Path<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Group,
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

    pub async fn update(
        auth_session: AuthSession<auth::Backend>,
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(group): Json<GroupDescription>,
    ) -> Response {
        match sqlx::query("SELECT 1 FROM \"group\" WHERE id = $1")
            .bind(group.parent)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {}
            Ok(None) => return StatusCode::BAD_REQUEST.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Group,
            &group_id,
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

        match sqlx::query(
            "UPDATE \"group\" SET \"name\" = $1, kind = $2, parent = $3 WHERE id = $4",
        )
        .bind(&group.name)
        .bind(&group.kind)
        .bind(&group.parent)
        .bind(group_id)
        .execute(&state.db)
        .await
        {
            Ok(r) => match r.rows_affected() {
                0 => StatusCode::NOT_FOUND.into_response(),
                1 => Json(group).into_response(),
                _ => StatusCode::BAD_REQUEST.into_response(),
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn delete(
        auth_session: AuthSession<auth::Backend>,
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::can_delete(resources::Type::Group, s_user.user_id, &state.db).await {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        match sqlx::query("DELETE FROM \"group\" WHERE id = $1")
            .bind(group_id)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn get_users(Path(id): Path<i64>, State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, user::Profile>(
            "SELECT \"user\".id, firstname, lastname, email \
FROM \"user\" LEFT JOIN \"user_in_group\" ON id = user_id
WHERE group_id = $1",
        )
        .bind(id)
        .fetch_all(&state.db)
        .await
        {
            Ok(u) => Json(u).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn replace_users(
        auth_session: AuthSession<auth::Backend>,
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> Response {
        match sqlx::query_as::<_, (i32,)>("SELECT 1 FROM \"group\" WHERE id = $1")
            .bind(group_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                if user_ids.len() == 0 {
                    return StatusCode::BAD_REQUEST.into_response();
                } else {
                    let s_user = auth_session.user.unwrap();
                    match auth::user_has_permissions_id(
                        resources::Type::Group,
                        &group_id,
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

                    let Ok(mut transaction) = state.db.begin().await else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    if sqlx::query("DELETE FROM \"user_in_group\" WHERE group_id = $1")
                        .bind(group_id)
                        .execute(&mut *transaction)
                        .await
                        .is_err()
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                    if user_ids.len() == 1 {
                        if let Err(_) = sqlx::query(
                            "INSERT INTO \"user_in_group\"(user_id, group_id) VALUES($1, $2)",
                        )
                        .bind(user_ids[0])
                        .bind(group_id)
                        .execute(&mut *transaction)
                        .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    } else {
                        let mut group_ids = Vec::with_capacity(user_ids.len());
                        group_ids.fill(group_id);
                        if let Err(_) = add_users_to_groups_query()
                            .bind(&user_ids)
                            .bind(&group_ids)
                            .execute(&mut *transaction)
                            .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    }

                    if let Err(_) = transaction.commit().await {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                    Json(user_ids).into_response()
                }
            }
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn add_users(
        auth_session: AuthSession<auth::Backend>,
        Path(group_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(user_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        if user_ids.len() < 1 {
            return StatusCode::BAD_REQUEST;
        }

        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Group,
            &group_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        };

        match sqlx::query_scalar::<_, i32>("SELECT 1 FROM \"group\" WHERE id = $1")
            .bind(group_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                if user_ids.len() == 1 {
                    if let Err(_) = sqlx::query(
                        "INSERT INTO \"user_in_group\"(user_id, group_id) VALUES ($1, $2)",
                    )
                    .bind(user_ids[0])
                    .bind(group_id)
                    .execute(&state.db)
                    .await
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR;
                    }
                    return StatusCode::CREATED;
                }

                let group_ids = vec![group_id; user_ids.len()];
                if let Err(_) = add_users_to_groups_query()
                    .bind(&user_ids)
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
