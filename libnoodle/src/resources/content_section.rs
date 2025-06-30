use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ContentSection {
    #[sqlx(rename = "uid")]
    pub section_id: i64,
    #[serde(rename = "parentCourseId")]
    pub course_id: Option<i64>,
    pub template_id: Option<i64>,
    pub headline: String,
    pub order_index: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSectionCreationRequest {
    pub headline: String,
    pub order_index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ContentElement {
    #[sqlx(rename = "uid")]
    pub content_id: i64,
    #[sqlx(rename = "section_id")]
    #[serde(rename = "parentSectionId")]
    pub section_id: i64,
    pub order_index: i32,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub element_type: String,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentElementCreationRequest {
    #[serde(rename = "type")]
    pub element_type: String,
    pub content: Option<String>,
    pub order_index: Option<i32>,
}

pub mod http {
    //NOTE: evtl. einzelne Content Sections -/ Elements versteckbar machen -> Permissions anpassen
    use crate::{
        auth::{self, permission::Operations},
        resources,
    };

    use super::*;
    use axum::{
        Json,
        extract::{Path as UrlPath, State},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use axum_login::AuthSession;

    /* -------- Sections ---------- */

    //GET /course/{courseId}/sections
    pub async fn get_all_for_course(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
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

        match sqlx::query_as::<_, ContentSection>("SELECT uid, course_id, template_id, headline, order_index FROM content_section WHERE course_id = $1 ORDER BY order_index")
            .bind(course_id)
            .fetch_all(&state.db)
            .await
        {
            Ok(sections) => Json(sections).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //POST /course/{courseId}/sections
    pub async fn create_for_course(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(course_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(req): Json<ContentSectionCreationRequest>,
    ) -> Response {
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
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        if req.headline.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        let idx = req.order_index.unwrap_or(0);
        match sqlx::query_scalar::<_, i64>("INSERT INTO content_section(course_id, headline, order_index) VALUES ($1,$2,$3) RETURNING uid")
            .bind(course_id)
            .bind(&req.headline)
            .bind(idx)
            .fetch_one(&state.db)
            .await
        {
            Ok(uid) => Json(
                    ContentSection { section_id: uid, course_id: Some(course_id), template_id: None, headline: req.headline, order_index: idx
                }).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //GET /course/{courseId}/section/{sectionId}
    pub async fn get_for_course(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
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

        match sqlx::query_as::<_, ContentSection>("SELECT uid, course_id, headline, order_index FROM content_section WHERE uid = $1 AND course_id = $2")
            .bind(section_id)
            .bind(course_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(sec)) => Json(sec).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //PUT /course/{courseId}/section/{sectionId}
    pub async fn update_for_course(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
        Json(req): Json<ContentSectionCreationRequest>,
    ) -> Response {
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
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        if req.headline.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        let idx = req.order_index.unwrap_or(0);
        match sqlx::query("UPDATE content_section SET headline = $1, order_index = $2 WHERE uid = $3 AND course_id = $4")
            .bind(&req.headline)
            .bind(idx)
            .bind(section_id)
            .bind(course_id)
            .execute(&state.db)
            .await
        {
            Ok(res) => {
                if res.rows_affected()==0 { StatusCode::NOT_FOUND.into_response() } else { Json(ContentSection{section_id, course_id: Some(course_id), template_id: None, headline:req.headline,order_index:idx}).into_response() }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //DELETE /course/{courseId}/section/{sectionId}
    pub async fn delete_for_course(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
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
        };

        match sqlx::query("DELETE FROM content_section WHERE uid = $1 AND course_id = $2")
            .bind(section_id)
            .bind(course_id)
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

    /* -------- Content Elements ---------- */

    //GET /course/{courseId}/section/{sectionId}/content
    pub async fn get_course_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Course,
            &course_id,
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

        let query = "SELECT uid, section_id, order_index, type, content FROM content_element WHERE section_id = $1 AND section_id IN (SELECT uid FROM content_section WHERE course_id = $2) ORDER BY order_index";
        match sqlx::query_as::<_, ContentElement>(query)
            .bind(section_id)
            .bind(course_id)
            .fetch_all(&state.db)
            .await
        {
            Ok(elements) => Json(elements).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //POST /course/{courseId}/section/{sectionId}/content
    pub async fn create_course_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
        Json(req): Json<ContentElementCreationRequest>,
    ) -> Response {
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
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        // verify section
        match sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM content_section WHERE uid=$1 AND course_id=$2",
        )
        .bind(section_id)
        .bind(course_id)
        .fetch_optional(&state.db)
        .await
        {
            Ok(None) => return StatusCode::BAD_REQUEST.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            _ => {}
        }
        let idx = req.order_index.unwrap_or(0);
        match sqlx::query_scalar::<_, i64>("INSERT INTO content_element(section_id, order_index, type, content) VALUES ($1,$2,$3,$4) RETURNING uid")
            .bind(section_id)
            .bind(idx)
            .bind(&req.element_type)
            .bind(&req.content)
            .fetch_one(&state.db)
            .await
        {
            Ok(uid) => Json(ContentElement{content_id:uid,section_id,order_index:idx,element_type:req.element_type,content:req.content}).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //PUT /course/{courseId}/section/{sectionId}/content
    pub async fn update_course_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
        Json(elem): Json<ContentElement>,
    ) -> Response {
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
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        match sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM content_section WHERE uid=$1 AND course_id=$2",
        )
        .bind(section_id)
        .bind(course_id)
        .fetch_optional(&state.db)
        .await
        {
            Ok(None) => return StatusCode::BAD_REQUEST.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            _ => {}
        }
        let query = "UPDATE content_element SET order_index=$1, type=$2, content=$3, updated_at=CURRENT_TIMESTAMP WHERE uid=$4 AND section_id=$5";
        match sqlx::query(query)
            .bind(elem.order_index)
            .bind(&elem.element_type)
            .bind(&elem.content)
            .bind(elem.content_id)
            .bind(section_id)
            .execute(&state.db)
            .await
        {
            Ok(r) => {
                if r.rows_affected() == 0 {
                    StatusCode::NOT_FOUND.into_response()
                } else {
                    Json(elem).into_response()
                }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //DELETE /course/{courseId}/section/{sectionId}/content
    pub async fn delete_course_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((course_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
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
        };

        let query = "DELETE FROM content_element WHERE section_id=$1 AND section_id IN (SELECT uid FROM content_section WHERE course_id=$2)";
        match sqlx::query(query)
            .bind(section_id)
            .bind(course_id)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    //GET /template/{templateId}/sections
    pub async fn get_all_for_template(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(template_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
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

        match sqlx::query_as::<_, ContentSection>("SELECT uid, course_id, template_id, headline, order_index FROM content_section WHERE template_id = $1 ORDER BY order_index")
            .bind(template_id)
            .fetch_all(&state.db)
            .await
        {
            Ok(sections) => Json(sections).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //POST /template/{templateId}/sections
    pub async fn create_for_template(
        auth_session: AuthSession<auth::Backend>,
        UrlPath(template_id): UrlPath<i64>,
        State(state): State<crate::AppState>,
        Json(req): Json<ContentSectionCreationRequest>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        if req.headline.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        let idx = req.order_index.unwrap_or(0);
        match sqlx::query_scalar::<_, i64>("INSERT INTO content_section(template_id, headline, order_index) VALUES ($1,$2,$3) RETURNING uid")
            .bind(template_id)
            .bind(&req.headline)
            .bind(idx)
            .fetch_one(&state.db)
            .await
        {
            Ok(uid) => Json(
                    ContentSection { section_id: uid, course_id: None, template_id: Some(template_id), headline: req.headline, order_index: idx
                }).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //GET /template/{templateId}/section/{sectionId}
    pub async fn get_for_template(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
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

        match sqlx::query_as::<_, ContentSection>("SELECT uid, course_id, template_id, headline, order_index FROM content_section WHERE uid = $1 AND template_id = $2")
            .bind(section_id)
            .bind(template_id)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(sec)) => Json(sec).into_response(),
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //PUT /template/{templateId}/section/{sectionId}
    pub async fn update_for_template(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
        Json(req): Json<ContentSectionCreationRequest>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        if req.headline.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }
        let idx = req.order_index.unwrap_or(0);
        match sqlx::query("UPDATE content_section SET headline = $1, order_index = $2 WHERE uid = $3 AND template_id = $4")
            .bind(&req.headline)
            .bind(idx)
            .bind(section_id)
            .bind(template_id)
            .execute(&state.db)
            .await
        {
            Ok(res) => {
                if res.rows_affected()==0 { StatusCode::NOT_FOUND.into_response() } else { Json(ContentSection{section_id, course_id: None, template_id: Some(template_id), headline:req.headline,order_index:idx}).into_response() }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //DELETE /template/{templateId}/section/{sectionId}
    pub async fn delete_for_template(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
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

        match sqlx::query("DELETE FROM content_section WHERE uid = $1 AND template_id = $2")
            .bind(section_id)
            .bind(template_id)
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
    //GET /template/{templateId}/section/{sectionId}/content
    pub async fn get_template_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
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

        let query = "SELECT uid, section_id, order_index, type, content FROM content_element WHERE section_id = $1 AND section_id IN (SELECT uid FROM content_section WHERE template_id = $2) ORDER BY order_index";
        match sqlx::query_as::<_, ContentElement>(query)
            .bind(section_id)
            .bind(template_id)
            .fetch_all(&state.db)
            .await
        {
            Ok(elements) => Json(elements).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //POST /template/{templateId}/section/{sectionId}/content
    pub async fn create_template_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
        Json(req): Json<ContentElementCreationRequest>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };

        // verify section
        match sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM content_section WHERE uid=$1 AND template_id=$2",
        )
        .bind(section_id)
        .bind(template_id)
        .fetch_optional(&state.db)
        .await
        {
            Ok(None) => return StatusCode::BAD_REQUEST.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            _ => {}
        }
        let idx = req.order_index.unwrap_or(0);
        match sqlx::query_scalar::<_, i64>("INSERT INTO content_element(section_id, order_index, type, content) VALUES ($1,$2,$3,$4) RETURNING uid")
            .bind(section_id)
            .bind(idx)
            .bind(&req.element_type)
            .bind(&req.content)
            .fetch_one(&state.db)
            .await
        {
            Ok(uid) => Json(ContentElement{content_id:uid,section_id,order_index:idx,element_type:req.element_type,content:req.content}).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //PUT /template/{templateId}/section/{sectionId}/content
    pub async fn update_template_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
        Json(elem): Json<ContentElement>,
    ) -> Response {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
            Operations::UPDATE,
            s_user.user_id,
            &state.db,
        )
        .await
        {
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(false) => return StatusCode::UNAUTHORIZED.into_response(),
            Ok(true) => {}
        };
        // sanity checks
        if elem.section_id != section_id {
            return StatusCode::BAD_REQUEST.into_response();
        }
        match sqlx::query_scalar::<_, i32>(
            "SELECT 1 FROM content_section WHERE uid=$1 AND template_id=$2",
        )
        .bind(section_id)
        .bind(template_id)
        .fetch_optional(&state.db)
        .await
        {
            Ok(None) => return StatusCode::BAD_REQUEST.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            _ => {}
        }
        let query = "UPDATE content_element SET order_index=$1, type=$2, content=$3, updated_at=CURRENT_TIMESTAMP WHERE uid=$4 AND section_id=$5";
        match sqlx::query(query)
            .bind(elem.order_index)
            .bind(&elem.element_type)
            .bind(&elem.content)
            .bind(elem.content_id)
            .bind(section_id)
            .execute(&state.db)
            .await
        {
            Ok(r) => {
                if r.rows_affected() == 0 {
                    StatusCode::NOT_FOUND.into_response()
                } else {
                    Json(elem).into_response()
                }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    //DELETE /template/{templateId}/section/{sectionId}/content
    pub async fn delete_template_content(
        auth_session: AuthSession<auth::Backend>,
        UrlPath((template_id, section_id)): UrlPath<(i64, i64)>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        let s_user = auth_session.user.unwrap();
        match auth::user_has_permissions_id(
            resources::Type::Template,
            &template_id,
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

        let query = "DELETE FROM content_element WHERE section_id=$1 AND section_id IN (SELECT uid FROM content_section WHERE template_id=$2)";
        match sqlx::query(query)
            .bind(section_id)
            .bind(template_id)
            .execute(&state.db)
            .await
        {
            Ok(_) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
