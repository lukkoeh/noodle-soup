use std::{
    env::{self, VarError},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Path(pub std::path::PathBuf);
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};

impl Path {
    pub fn from_relative_path(base_path: &str, path: &str) -> Self {
        let mut path_buf = std::path::PathBuf::with_capacity(base_path.len() + path.len() + 2);
        path_buf.push(base_path);
        path_buf.push(path);
        Self(path_buf)
    }

    pub fn from_file_row(base_path: &str, file_row: &FileRow) -> Self {
        Self::from_relative_path(base_path, &file_row.location)
    }

    pub fn from_hash(base_path: &str, dir_hash: &[u8]) -> Self {
        let filename = hex::encode(&dir_hash[1..]);
        let relative_path = format!("{}/{}", &hex::encode(&dir_hash[..1]), &filename);
        Path::from_relative_path(base_path, &relative_path)
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

        tokio::task::spawn_blocking(|| STANDARD_NO_PAD.encode(file_contents))
            .await
            .unwrap()
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
    // TODO: proper error handling for failed I/O operations
    // Files are not streamed. They are directly put in memory and written to the file system at once.
    // This is definetly not optimal (Someone uploads an 1GB file -> Ram usage goes up by 1GB).
    // If we want to change that, the API spec needs to be adjusted accordingly.
    use std::sync::Arc;

    use super::*;
    use axum::{
        Json,
        extract::{Path as UrlPath, State},
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use axum_login::AuthSession;
    use sha1::{Digest, Sha1};

    const TEMP_SUFFIX: &'static str = "tmp";

    fn get_temp_file_name(orig_name: &str) -> String {
        format!("{}.{}", orig_name, TEMP_SUFFIX)
    }

    fn strip_tmp_suffix(temp_file_name: &str) -> &str {
        &temp_file_name[..temp_file_name.len() - (TEMP_SUFFIX.len() + 1)]
    }

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

        let mut path = Path::from_hash(&state.media_path, &dir_hash);
        tokio::fs::create_dir(&path.0.parent().unwrap())
            .await
            .unwrap();

        path.0.set_extension(TEMP_SUFFIX);

        let shared_contents = Arc::new(file.data);
        let contents = {
            let data_for_decode = Arc::clone(&shared_contents);
            tokio::task::spawn_blocking(move || {
                STANDARD_NO_PAD.decode(data_for_decode.as_bytes()).unwrap()
            })
            .await
            .unwrap()
        };

        if let Err(_) = tokio::fs::write(&path, contents).await {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        let file_id = Uuid::new_v4();
        match sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(
            "INSERT INTO \"file\" (uid, filename, \"type\", location) VALUES ($1, $2, $3, $4) RETURNING created_at",
        )
        .bind(&file_id)
        .bind(&file.filename)
        .bind(&file.mime_type)
        .bind(&path.0.strip_prefix(&state.media_path).unwrap().with_extension("").to_str().unwrap())
        .fetch_one(&state.db)
        .await
        {
            Ok(created_at) => {
                let data = match Arc::try_unwrap(shared_contents) {
                    Ok(s) => s,
                    Err(arc) => (*arc).clone(),
                };
                let path_stripped = strip_tmp_suffix(path.0.to_str().unwrap());
                tokio::fs::rename(&path, path_stripped).await.unwrap();
                (StatusCode::CREATED, Json(File{
                    uid: file_id,
                    filename: file.filename,
                    mime_type: file.mime_type,
                    data,
                    last_modified: created_at,
                })).into_response()
            },
            Err(_) => {
                tokio::fs::remove_file(path).await.unwrap();
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    pub async fn get_by_uid(
        UrlPath(uid): UrlPath<Uuid>,
        State(state): State<crate::AppState>,
    ) -> Response {
        //TODO: Caching - return NOT MODIFIED if file was not changed.
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

        let mut path = Path::from_relative_path(&state.media_path, &old_file.location);
        path.0.set_extension(TEMP_SUFFIX);

        tokio::fs::write(&path, new_contents).await.unwrap();

        match sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(
            "UPDATE \"file\" SET filename = $1, \"type\" = $2, updated_at = CURRENT_TIMESTAMP WHERE uid = $3 RETURNING updated_at",
        )
        .bind(&new_file.filename)
        .bind(&new_file.mime_type)
        .bind(&uid)
        .fetch_one(&state.db)
        .await
        {
            Ok(last_modified) => {
                let data = match Arc::try_unwrap(shared_contents) {
                    Ok(s) => s,
                    Err(arc) => (*arc).clone(),
                };
                tokio::fs::rename(&path, strip_tmp_suffix(path.0.to_str().unwrap())).await.unwrap();
                Json(File {
                    uid,
                    filename: new_file.filename,
                    mime_type: new_file.mime_type,
                    data,
                    last_modified,
                }).into_response()
            },
            Err(_) => {
                tokio::fs::remove_file(&path).await.unwrap();
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
        }
    }

    pub async fn delete(
        UrlPath(uid): UrlPath<Uuid>,
        State(state): State<crate::AppState>,
    ) -> StatusCode {
        let file_to_delete =
            match sqlx::query_as::<_, FileRow>("SELECT * FROM \"file\" WHERE uid = $1")
                .bind(&uid)
                .fetch_optional(&state.db)
                .await
            {
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
                Ok(None) => return StatusCode::NOT_FOUND,
                Ok(Some(file)) => file,
            };

        match sqlx::query("DELETE FROM \"file\" WHERE uid = $1")
            .bind(&uid)
            .execute(&state.db)
            .await
        {
            Ok(_) => {
                let path = Path::from_relative_path(&state.media_path, &file_to_delete.location);
                match tokio::fs::remove_file(&path).await {
                    Ok(_) => StatusCode::OK,
                    Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
