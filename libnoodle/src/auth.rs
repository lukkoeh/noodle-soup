use std::{error, fmt::Display};

use async_trait::async_trait;
use axum::{Json, http::StatusCode};
use axum_login::{AuthSession, AuthUser, AuthnBackend, UserId};
use sqlx::PgPool;
use tokio::task;

use crate::user;

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
        let user: Result<Option<(i64, String, String, String, String)>, _> =
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
                email: u.3,
                password: u.4.into(),
            })),
            _ => Ok(None),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user: Result<Option<(i64, String, String, String, String)>, _> =
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
                email: u.3,
                password: u.4.into(),
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
