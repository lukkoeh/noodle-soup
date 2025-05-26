use std::{
    env::{self, VarError},
    str::FromStr,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Path(std::path::PathBuf);
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
impl Path {
    pub fn from_relative_path(base_path: &str, path: &str) -> Self {
        let mut path_buf = std::path::PathBuf::with_capacity(base_path.len() + path.len() + 2);
        path_buf.push(base_path);
        path_buf.push(path);
        Self(path_buf)
    }

    pub fn from_file_row(base_path: &str, file_row: &FileRow) -> Self {
        Self::from_file_location_and_name(base_path, &file_row.location, &file_row.filename)
    }

    pub fn from_file_location_and_name(base_path: &str, location: &str, name: &str) -> Self {
        let mut path_buf =
            std::path::PathBuf::with_capacity(base_path.len() + location.len() + name.len() + 3);
        path_buf.push(base_path);
        path_buf.push(location);
        path_buf.push(name);
        Self(path_buf)
    }
}

impl FromStr for Path {
    type Err = VarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base_path = &env::var("MEDIA_PATH")?;
        Ok(Path::from_relative_path(base_path, s))
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    #[serde(with = "uuid::serde::simple")]
    uid: Uuid,
    filename: String,
    #[serde(rename = "type")]
    mime_type: String,
    data: String,
    last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
pub struct FileRow {
    uid: Uuid,
    filename: String,
    #[sqlx(rename = "type")]
    mime_type: String,
    location: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl FileRow {
    pub async fn get_contents_b64(&self, media_path: &str) -> String {
        let path = Path::from_file_row(media_path, self);

        let file_contents = tokio::fs::read(&path).await.unwrap();
        let file_contents_encoded =
            tokio::task::spawn_blocking(|| STANDARD_NO_PAD.encode(file_contents))
                .await
                .unwrap();
        file_contents_encoded
    }
}

#[derive(Deserialize)]
pub struct FileDescription {
    filename: String,
    #[serde(rename = "type")]
    mime_type: String,
    data: String,
}

pub mod http {
    //TODO: proper error handling for failed I/O operations
    use std::{fmt::Write, sync::Arc};

    use super::*;
    use axum::{
        Json,
        extract::{Path as UrlPath, State},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use axum_login::AuthSession;
    use sha1::{Digest, Sha1};

    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, FileRow>("SELECT * FROM \"file\"")
            .fetch_all(&state.db)
            .await
        {
            Ok(file_rows) => {
                let mut files = Vec::with_capacity(file_rows.len());
                for file in file_rows {
                    let file_contents_encoded = file.get_contents_b64(&state.media_path).await;

                    files.push(File {
                        uid: file.uid,
                        mime_type: file.mime_type,
                        filename: file.filename,
                        data: file_contents_encoded,
                        last_modified: file.updated_at,
                    })
                }
                Json(files).into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn create(
        State(state): State<crate::AppState>,
        auth_session: AuthSession<crate::auth::Backend>,
        Json(file): Json<FileDescription>,
    ) -> Response {
        if file.filename.len() < 1 {
            return StatusCode::BAD_REQUEST.into_response();
        }
        let current_time = chrono::Utc::now();
        let mut hasher = Sha1::new();
        hasher.update(current_time.to_rfc2822());
        hasher.update(&file.filename);
        hasher.update(&auth_session.user.unwrap().user_id.to_le_bytes());
        let dir_hash = hasher.finalize();
        let mut path_str = String::with_capacity(2 * dir_hash.len() + 2);
        write!(
            &mut path_str,
            "{}/{}",
            hex::encode(&dir_hash[..1]),
            hex::encode(&dir_hash[1..])
        )
        .unwrap();
        let mut path = Path::from_relative_path(&state.media_path, &path_str);
        std::fs::create_dir_all(&path).unwrap();
        path.0.push(&file.filename);

        let shared_contents = Arc::new(file.data);
        let contents = {
            let data_for_decode = Arc::clone(&shared_contents);
            tokio::task::spawn_blocking(move || {
                STANDARD_NO_PAD.decode(data_for_decode.as_bytes()).unwrap()
            })
            .await
            .unwrap()
        };

        if let Err(_) = tokio::fs::write(path, contents).await {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        let file_id = Uuid::new_v4();
        match sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(
            "INSERT INTO \"file\" (uid, filename, \"type\", location) VALUES ($1, $2, $3, $4) RETURNING created_at",
        )
        .bind(&file_id)
        .bind(&file.filename)
        .bind(&file.mime_type)
        .bind(&path_str)
        .fetch_one(&state.db)
        .await
        {
            Ok(created_at) => {
                let data = match Arc::try_unwrap(shared_contents) {
                    Ok(s) => s,
                    Err(arc) => (*arc).clone(),
                };

                (StatusCode::CREATED, Json(File{
                    uid: file_id,
                    filename: file.filename,
                    mime_type: file.mime_type,
                    data,
                    last_modified: created_at,
                })).into_response()
            },
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    pub async fn get_by_uid(
        UrlPath(uid): UrlPath<Uuid>,
        State(state): State<crate::AppState>,
    ) -> Response {
        match sqlx::query_as::<_, FileRow>("SELECT * FROM \"file\" WHERE uid = $1")
            .bind(&uid)
            .fetch_optional(&state.db)
            .await
        {
            Ok(Some(file)) => {
                let encoded_contents = file.get_contents_b64(&state.media_path).await;
                Json(File {
                    uid,
                    filename: file.filename,
                    mime_type: file.mime_type,
                    data: encoded_contents,
                    last_modified: file.updated_at,
                })
                .into_response()
            }
            Ok(None) => StatusCode::NOT_FOUND.into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn update(
        UrlPath(uid): UrlPath<Uuid>,
        State(state): State<crate::AppState>,
        Json(new_file): Json<FileDescription>,
    ) -> Response {
        if new_file.filename.len() < 1 {
            return StatusCode::BAD_REQUEST.into_response();
        }
        let old_file = match sqlx::query_as::<_, FileRow>("SELECT * FROM \"file\" WHERE uid = $1")
            .bind(&uid)
            .fetch_optional(&state.db)
            .await
        {
            Ok(None) => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Ok(Some(file)) => file,
        };

        let shared_contents = Arc::new(new_file.data);
        let new_contents = {
            let data_for_decode = Arc::clone(&shared_contents);
            tokio::task::spawn_blocking(move || {
                STANDARD_NO_PAD.decode(data_for_decode.as_bytes()).unwrap()
            })
            .await
            .unwrap()
        };

        tokio::fs::write(
            &Path::from_file_location_and_name(
                &state.media_path,
                &old_file.location,
                &new_file.filename,
            ),
            new_contents,
        )
        .await
        .unwrap();

        if old_file.filename != new_file.filename {
            tokio::fs::remove_file(&Path::from_file_row(&state.media_path, &old_file))
                .await
                .unwrap();
        }
        //TODO: Update DB, return response
    }
}
