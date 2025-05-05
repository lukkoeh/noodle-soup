use serde::Deserialize;

#[derive(serde::Serialize, serde::Deserialize, sqlx::types::Type, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

impl Profile {
    pub fn new(user_id: i64, firstname: String, lastname: String, email: String) -> Self {
        Profile {
            id: user_id,
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

pub mod http {
    use crate::auth::permission::{GroupRow, add_users_to_groups_query};

    use super::{New, Profile, validation};
    use axum::{
        Json,
        extract::{Path, State},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use axum_login::AuthSession;

    pub async fn delete(Path(id): Path<i64>, State(state): State<crate::AppState>) -> StatusCode {
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

    pub async fn update(
        Path(id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(profile): Json<Profile>,
    ) -> Response {
        let mail_validation = validation::email(&profile.email);
        if !mail_validation.is_valid() {
            return (StatusCode::BAD_REQUEST, Json(mail_validation)).into_response();
        }
        match sqlx::query_as::<_, (i64,)>("SELECT id FROM \"user\" WHERE email = $1")
            .bind(&profile.email)
            .fetch_optional(&state.db)
            .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(Some(i)) => {
                if i.0 != id {
                    return StatusCode::CONFLICT.into_response();
                }
            }
            _ => {}
        }
        if let Err(_) = sqlx::query(
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

    pub async fn get(Path(id): Path<i64>, State(state): State<crate::AppState>) -> Response {
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

    pub async fn create(State(state): State<crate::AppState>, Json(user): Json<New>) -> Response {
        let result = sqlx::query("SELECT 1 FROM \"user\" WHERE email = $1")
            .bind(&user.email)
            .fetch_optional(&state.db)
            .await;
        match result {
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(Some(_)) => StatusCode::CONFLICT.into_response(),
            Ok(None) => {
                let validation = validation::credentials(&user.email, &user.password);
                if !validation.is_valid() {
                    return (StatusCode::BAD_REQUEST, Json(validation)).into_response();
                }

                let hashed = tokio::task::spawn_blocking(|| {
                    bcrypt::hash(user.password, bcrypt::DEFAULT_COST)
                })
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

    pub async fn add_to_groups(
        Path(user_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(group_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        if group_ids.len() < 1 {
            return StatusCode::BAD_REQUEST;
        }
        match sqlx::query("SELECT 1 FROM \"user\" WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(None) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(Some(_)) => {}
        };
        if group_ids.len() == 1 {
            if let Err(_) =
                sqlx::query("INSERT INTO \"user_in_group\"(user_id, group_id) VALUES ($1, $2)")
                    .bind(user_id)
                    .bind(group_ids[0])
                    .execute(&state.db)
                    .await
            {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
            return StatusCode::CREATED;
        }
        let mut user_ids = Vec::with_capacity(group_ids.len());
        user_ids.fill(user_id);
        match add_users_to_groups_query()
            .bind(user_ids)
            .bind(group_ids)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub async fn get_self(auth_session: AuthSession<crate::auth::Backend>) -> Json<Profile> {
        let user = auth_session.user.unwrap();
        Json(Profile::new(
            user.user_id,
            user.firstname,
            user.lastname,
            user.email,
        ))
    }

    pub async fn get_groups(
        Path(user_id): Path<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        match GroupRow::from_user_id(&state.db, user_id).await {
            Ok(g) => Json(g).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn get_self_groups(
        State(state): State<crate::AppState>,
        auth_session: AuthSession<crate::auth::Backend>,
    ) -> Response {
        let user = auth_session.user.unwrap();
        match GroupRow::from_user_id(&state.db, user.user_id).await {
            Ok(g) => Json(g).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn replace_groups(
        Path(user_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(group_ids): Json<Vec<i64>>,
    ) -> Response {
        match sqlx::query_as::<_, (i32,)>("SELECT 1 FROM \"user\" WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(_)) => {
                if group_ids.len() == 0 {
                    return StatusCode::BAD_REQUEST.into_response();
                } else {
                    let Ok(mut transaction) = state.db.begin().await else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };
                    if let Err(_) = sqlx::query("DELETE FROM \"user_in_group\" WHERE user_id = $1")
                        .bind(group_ids[0])
                        .execute(&mut *transaction)
                        .await
                    {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                    if group_ids.len() == 1 {
                        if let Err(_) = sqlx::query(
                            "INSERT INTO \"user_in_group\"(user_id, group_id) VALUES($1, $2)",
                        )
                        .bind(user_id)
                        .bind(group_ids[0])
                        .execute(&mut *transaction)
                        .await
                        {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    } else {
                        let mut user_ids = Vec::with_capacity(group_ids.len());
                        user_ids.fill(user_id);
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
                    Json(group_ids).into_response()
                }
            }
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn delete_groups(
        Path(user_id): Path<i64>,
        State(state): State<crate::AppState>,
        Json(group_ids): Json<Vec<i64>>,
    ) -> StatusCode {
        match sqlx::query("DELETE FROM \"user_in_group\" WHERE group_id = ANY($1) AND user_id = $2")
            .bind(group_ids)
            .bind(user_id)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

mod validation {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct CredentialsErrors {
        email: EmailErrors,
        password: PasswordErrors,
    }

    impl CredentialsErrors {
        pub fn is_valid(&self) -> bool {
            self.email.is_valid() && self.password.is_valid()
        }
    }

    pub fn credentials(email: &str, password: &str) -> CredentialsErrors {
        CredentialsErrors {
            email: self::email(email),
            password: self::password(password),
        }
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PasswordErrors {
        too_short: bool,
        uppercase_missing: bool,
        lowercase_missing: bool,
        digit_missing: bool,
        special_missing: bool,
    }

    impl PasswordErrors {
        pub fn is_valid(&self) -> bool {
            !self.too_short
                && !self.uppercase_missing
                && !self.lowercase_missing
                && !self.digit_missing
                && !self.special_missing
        }
    }

    pub fn password(password: &str) -> PasswordErrors {
        PasswordErrors {
            too_short: password.len() < 8,
            uppercase_missing: !password.contains(|c: char| c.is_ascii_uppercase()),
            lowercase_missing: !password.contains(|c: char| c.is_ascii_lowercase()),
            digit_missing: !password.contains(|c: char| c.is_ascii_digit()),
            special_missing: !password.contains(|c: char| {
                !c.is_ascii_uppercase() && !c.is_ascii_lowercase() && !c.is_ascii_digit()
            }),
        }
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EmailErrors {
        too_short: bool,
        too_long: bool,
        illegal_char: bool,
        invalid_format: bool,
    }

    impl EmailErrors {
        pub fn is_valid(&self) -> bool {
            !self.too_short && !self.too_long && !self.illegal_char && !self.invalid_format
        }
    }

    impl Default for EmailErrors {
        fn default() -> Self {
            Self {
                too_short: false,
                too_long: false,
                illegal_char: false,
                invalid_format: false,
            }
        }
    }

    pub fn email(email: &str) -> EmailErrors {
        let mut status = EmailErrors::default();
        if email.len() < 3 {
            status.too_short = true
        }
        if email.len() > 255 {
            status.too_long = true
        }
        if email.contains([' ', '\t', '\n']) {
            status.illegal_char = true
        }
        match email.find('@') {
            Some(at) => {
                status.invalid_format =
                    at == 0 || at == (email.len() - 1) || email[at + 1..].contains('@');
                status
            }
            None => {
                status.invalid_format = true;
                status
            }
        }
    }
}
