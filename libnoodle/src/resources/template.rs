use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    #[sqlx(rename = "uid")]
    pub template_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateDescription {
    pub name: String,
}

pub mod http {
    use crate::{
        auth::{self, permission::Operations},
        resources,
    };

    use super::{Template, TemplateDescription};
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
        let templates = match auth::user_has_permissions_all(
            resources::Type::Template,
            Operations::READ,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Ok(true) => match sqlx::query_as::<_, Template>("SELECT uid, name FROM \"template\"")
                .fetch_all(&state.db)
                .await
            {
                Ok(templates) => templates,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
            Ok(false) => match sqlx::query_as::<_, Template>(
                "SELECT t.uid, t.name \
FROM template t \
JOIN template_permissions tp ON t.uid = tp.resource_id \
WHERE (tp.user_id = $1 OR EXISTS(\
SELECT 1 FROM user_has_role ur \
WHERE ur.user_id = $1 AND ur.role_id = tp.role_id)) \
AND (tp.permission & $2::int::bit(16)) <> B'0'::bit(16)",
            )
            .bind(s_user.user_id)
            .bind(Operations::READ)
            .fetch_all(&state.db)
            .await
            {
                Ok(templates) => templates,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        Json(templates).into_response()
    }

    pub async fn get_by_uid(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(id): UrlPath<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
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

        match sqlx::query_as::<_, Template>("SELECT uid, name FROM \"template\" WHERE uid = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(tpl)) => Json(tpl).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn create(
        auth_session: AuthSession<auth::Backend>,
        State(state): State<crate::AppState>,
        Json(desc): Json<TemplateDescription>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::can_create(resources::Type::Template, s_user.user_id, &state.db).await {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        }

        if desc.name.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match sqlx::query_scalar::<_, i64>(
            "INSERT INTO \"template\"(name) VALUES ($1) RETURNING uid",
        )
        .bind(&desc.name)
        .fetch_one(&state.db)
        .await
        {
            Ok(id) => Json(Template {
                template_id: id,
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
        Json(desc): Json<TemplateDescription>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
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
        match sqlx::query("UPDATE \"template\" SET name = $1 WHERE uid = $2")
            .bind(&desc.name)
            .bind(id)
            .execute(&state.db)
            .await
        {
            Ok(res) => {
                if res.rows_affected() == 0 {
                    StatusCode::NOT_FOUND.into_response()
                } else {
                    Json(Template {
                        template_id: id,
                        name: desc.name,
                    })
                    .into_response()
                }
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
        match auth::can_delete(resources::Type::Template, s_user.user_id, &state.db).await {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
            Ok(false) => return StatusCode::UNAUTHORIZED,
            Ok(true) => {}
        }

        match sqlx::query("DELETE FROM \"template\" WHERE uid = $1")
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
