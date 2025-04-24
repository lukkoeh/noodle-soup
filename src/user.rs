use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_login::AuthSession;
use serde::Deserialize;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl Profile {
    pub fn new(user_id: i64, firstname: String, lastname: String, email: String) -> Self {
        Profile {
            user_id,
            firstname,
            lastname,
            email,
        }
    }
}

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub(crate) user_id: i64,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) email: String,
    pub(crate) password: Vec<u8>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct New {
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) email: String,
    pub(crate) password: String,
}

pub async fn delete_user_handler(
    Path(id): Path<i64>,
    State(state): State<crate::AppState>,
) -> StatusCode {
    match sqlx::query("DELETE FROM \"user\" WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
    {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        Ok(num) => {
            if num.rows_affected() < 1 {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::OK
            }
        }
    }
}

pub async fn change_user_handler(
    Path(id): Path<i64>,
    State(state): State<crate::AppState>,
    Json(profile): Json<Profile>,
) -> Response {
    //TODO: validation
    if let Err(_) =
        //TODO: Checken, ob neue E-mail bereits vergeben
        sqlx::query(
            "UPDATE \"user\" SET firstname = $1, lastname = $2, email = $3 WHERE id = $4",
        )
        .bind(&profile.firstname)
        .bind(&profile.lastname)
        .bind(&profile.email)
        .bind(id)
        .execute(&state.db)
        .await
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    return Json(Profile::new(
        id,
        profile.firstname,
        profile.lastname,
        profile.email,
    ))
    .into_response();
}

pub async fn get_user_handler(
    Path(id): Path<i64>,
    State(state): State<crate::AppState>,
) -> Response {
    let user: Result<Option<(i64, String, String, String)>, _> =
        sqlx::query_as("SELECT id, firstname, lastname, email FROM \"user\" WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await;

    if let Err(_) = user {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    if let Some(u) = user.unwrap() {
        Json(Profile::new(u.0, u.1, u.2, u.3)).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn create_user_handler(
    State(state): State<crate::AppState>,
    Json(user): Json<New>,
) -> Response {
    //TODO: validation
    let result = sqlx::query("SELECT 1 FROM \"user\" WHERE email = $1")
        .bind(&user.email)
        .fetch_optional(&state.db)
        .await;
    match result {
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(Some(_)) => StatusCode::CONFLICT.into_response(),
        Ok(None) => {
            let hashed =
                tokio::task::spawn_blocking(|| bcrypt::hash(user.password, bcrypt::DEFAULT_COST))
                    .await;

            if let Ok(pw) = hashed {
                if let Err(_) = pw {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                };

                let result =sqlx::query("INSERT INTO \"user\" (firstname, lastname, email, password) VALUES ($1, $2, $3, $4)")
                    .bind(&user.firstname)
                    .bind(&user.lastname)
                    .bind(&user.email)
                    .bind(pw.unwrap())
                    .execute(&state.db).await;
                if let Err(_) = result {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                let result: Result<Option<(i64,)>, _> =
                    sqlx::query_as("SELECT id FROM \"user\" WHERE email = $1")
                        .bind(&user.email)
                        .fetch_optional(&state.db)
                        .await;
                if let Err(_) | Ok(None) = result {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                (
                    StatusCode::CREATED,
                    Json(Profile::new(
                        result.unwrap().unwrap().0,
                        user.firstname,
                        user.lastname,
                        user.email,
                    )),
                )
                    .into_response()
            } else {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn get_self_handler(auth_session: AuthSession<crate::auth::Backend>) -> Json<Profile> {
    let user = auth_session.user.unwrap();
    Json(Profile::new(
        user.user_id,
        user.firstname,
        user.lastname,
        user.email,
    ))
}
