use std::{error, fmt::Display};

use async_trait::async_trait;
use axum::{Json, http::StatusCode};
use axum_login::{AuthSession, AuthUser, AuthnBackend, UserId};
use sqlx::{PgPool, Postgres, postgres::PgRow};
use tokio::task;

use crate::{
    resources::Type as ResourceType,
    user::{self, User},
};

pub mod permission;
use permission::Operations;

impl AuthUser for user::User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password
    }
}

#[derive(Clone)]
pub struct Backend {
    db: PgPool,
}

impl Backend {
    pub async fn new(db: PgPool) -> Self {
        Backend { db }
    }
}

#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
    TaskJoin(task::JoinError),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Sqlx(..) => write!(f, "Sql error occurred"),
            Error::TaskJoin(..) => write!(f, "TaskJoin Error occurred"),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Sqlx(ref e) => Some(e),
            Error::TaskJoin(ref e) => Some(e),
        }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = user::User;
    type Credentials = user::Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Result<Option<(i64, String, String, String, String, String)>, _> =
            sqlx::query_as("SELECT * FROM \"user\" WHERE email = $1")
                .bind(credentials.email)
                .fetch_optional(&self.db)
                .await;
        if let Err(e) = user {
            return Err(Self::Error::Sqlx(e));
        }
        let user = user.unwrap();
        match task::spawn_blocking(|| {
            user.filter(|u| {
                if let Ok(true) = bcrypt::verify(credentials.password, &u.4) {
                    true
                } else {
                    false
                }
            })
        })
        .await
        {
            Err(e) => return Err(Self::Error::TaskJoin(e)),
            Ok(Some(u)) => Ok(Some(Self::User {
                user_id: u.0,
                firstname: u.1,
                lastname: u.2,
                title: u.3,
                email: u.4,
                password: u.5.into(),
            })),
            _ => Ok(None),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user: Result<Option<(i64, String, String, String, String, String)>, _> =
            sqlx::query_as("SELECT * FROM \"user\" WHERE id = $1")
                .bind(user_id)
                .fetch_optional(&self.db)
                .await;
        if let Err(e) = user {
            return Err(Self::Error::Sqlx(e));
        }

        if let Some(u) = user.unwrap() {
            Ok(Some(Self::User {
                user_id: u.0,
                firstname: u.1,
                lastname: u.2,
                title: u.3,
                email: u.4,
                password: u.5.into(),
            }))
        } else {
            Ok(None)
        }
    }
}

pub async fn create_session_handler(
    mut auth_session: AuthSession<Backend>,
    Json(credentials): Json<user::Credentials>,
) -> StatusCode {
    if let Ok(Some(u)) = auth_session.authenticate(credentials).await {
        if let Err(_) = auth_session.login(&u).await {
            return StatusCode::UNAUTHORIZED;
        };
        StatusCode::CREATED
    } else {
        StatusCode::UNAUTHORIZED
    }
}

pub async fn get_permitted_ids<'a, T>(
    resource_type: ResourceType,
    operations: Operations,
    user: &User,
    db: &PgPool,
) -> Result<Vec<T>, sqlx::Error>
where
    for<'r> (T,): sqlx::FromRow<'r, PgRow>,
    for<'r> T: sqlx::Decode<'r, Postgres> + sqlx::Type<Postgres>,
    T: Send + Unpin,
{
    let table_name = resource_type.table_name();
    sqlx::query_scalar::<_, T>(&format!(
        "SELECT DISTINCT resource_id FROM {table_name}_permissions \
LEFT JOIN user_has_role on user_has_role.role_id = {table_name}_permissions.role_id \
WHERE (user_has_role.user_id = $1 OR {table_name}_permissions.user_id = $1) \
AND ($2::bit(16) & permission) <> B'0'::bit(16)",
    ))
    .bind(user.user_id)
    .bind(operations)
    .fetch_all(db)
    .await
}

pub async fn can_create(
    resource_type: ResourceType,
    user_id: i64,
    db: &PgPool,
) -> Result<bool, sqlx::Error> {
    user_has_permissions_all(resource_type, Operations::CREATE, user_id, db).await
}

pub async fn can_delete(
    resource_type: ResourceType,
    user_id: i64,
    db: &PgPool,
) -> Result<bool, sqlx::Error> {
    user_has_permissions_all(resource_type, Operations::DELETE, user_id, db).await
}

pub async fn can_update(
    resource_type: ResourceType,
    user_id: i64,
    db: &PgPool,
) -> Result<bool, sqlx::Error> {
    user_has_permissions_all(resource_type, Operations::UPDATE, user_id, db).await
}

pub async fn user_has_permissions_all(
    resource_type: ResourceType,
    operations: Operations,
    user_id: i64,
    db: &PgPool,
) -> Result<bool, sqlx::Error> {
    let table_name = resource_type.table_name();
    match sqlx::query_scalar::<_, i64>(&format!(
        "SELECT 1 FROM {table_name}_permissions \
LEFT JOIN user_has_role ON user_has_role.role_id = {table_name}_permissions.role_id \
WHERE (user_has_role.user_id = $1 OR {table_name}_permissions.user_id = $1) \
AND ($2::int::bit(16) & permission) <> B'0'::bit(16) AND resource_id IS NULL",
    ))
    .bind(user_id)
    .bind(operations)
    .fetch_optional(db)
    .await
    {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e),
    }
}

pub async fn user_has_permissions_id<
    'a,
    T: sqlx::Type<sqlx::Postgres> + sqlx::Encode<'a, sqlx::Postgres>,
>(
    resource_type: ResourceType,
    resource_id: &'a T,
    operations: Operations,
    user_id: i64,
    db: &PgPool,
) -> Result<bool, sqlx::Error> {
    match sqlx::query_scalar::<_, i64>(resource_type.permission_id_query())
        .bind(user_id)
        .bind(operations)
        .bind(resource_id)
        .fetch_optional(db)
        .await
    {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e),
    }
}
