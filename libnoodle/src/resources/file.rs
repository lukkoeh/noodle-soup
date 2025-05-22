use std::{
    env::{self, VarError},
    ops::RangeToInclusive,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    #[serde(with = "uuid::serde::simple")]
    uid: Uuid,
    #[serde(rename = "type")]
    mime_type: String,
    data: String,
    last_modified: chrono::DateTime<chrono::Utc>,
}

pub struct Path(std::path::PathBuf);
impl Path {
    pub fn from_relative_path(base_path: &str, path: &str) -> Self {
        let mut path_buf = std::path::PathBuf::with_capacity(base_path.len() + path.len() + 10);
        path_buf.push(base_path);
        path_buf.push(path);
        Self(path_buf)
    }
}

impl FromStr for Path {
    type Err = VarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base_path = &env::var("MEDIA_PATH")?;
        let mut path_buf = std::path::PathBuf::with_capacity(base_path.len() + s.len() + 10);
        path_buf.push(base_path);
        path_buf.push(s);
        Ok(Self(path_buf))
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}

#[derive(sqlx::FromRow)]
pub struct FileRow {
    uid: Uuid,
    mime_type: String,
    location: String,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub mod http {
    use super::*;
    use axum::{
        Json,
        extract::State,
        http::StatusCode,
        response::{IntoResponse, Response},
    };
    use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};

    pub async fn get_all(State(state): State<crate::AppState>) -> Response {
        match sqlx::query_as::<_, FileRow>("SELECT * FROM \"file\"")
            .fetch_all(&state.db)
            .await
        {
            Ok(file_rows) => {
                let mut files = Vec::new();
                for file in file_rows {
                    let file_contents =
                        std::fs::read(Path::from_relative_path(&state.media_path, &file.location))
                            .unwrap();
                    files.push(File {
                        uid: file.uid,
                        mime_type: file.mime_type,
                        data: STANDARD_NO_PAD.encode(file_contents),
                        last_modified: file.updated_at,
                    })
                }
                Json(files).into_response()
            }
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
