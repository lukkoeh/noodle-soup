// use std::os::unix::ffi::OsStrExt;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use sqlx::PgPool;
use uuid::Uuid;

use crate::resources::file::Path;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branding {
    color: u32,
    #[serde(with = "uuid::serde::simple")]
    #[serde(rename = "targetUid")]
    logo_file_uuid: uuid::Uuid,
}

#[derive(thiserror::Error, Debug)]
pub enum BrandingCreateError {
    #[error("Database query failed!")]
    Sqlx(#[from] sqlx::Error),
    #[error("I/O operation failed!")]
    Io(#[from] std::io::Error),
    #[error("Serialization invalid")]
    Serde(serde_json::Error),
    #[error("File type is invalid")]
    InvalidFileType,
}

const CONFIG_FILE_NAME: &'static str = "branding.json";
const DEFAULT_LOGO_NAME: &'static str = "default.png";

impl Branding {
    pub async fn create_in_db(
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
        logo: std::path::PathBuf,
        dest_base_path: &str,
        db: &PgPool,
    ) -> Result<Self, BrandingCreateError> {
        let file_name = logo.file_name();
        let Some(filename) = file_name else {
            return Err(BrandingCreateError::InvalidFileType);
        };

        let mime_type = match logo.extension() {
            None => return Err(BrandingCreateError::InvalidFileType),
            Some(ext) => match ext.to_string_lossy().as_ref() {
                "jpg" | "JPG" | "jpeg" | "JPEG" => mime::IMAGE_JPEG,
                "png" | "PNG" => mime::IMAGE_PNG,
                "svg" | "SVG" => mime::IMAGE_SVG,
                _ => return Err(BrandingCreateError::InvalidFileType),
            },
        };

        let color = 0u32
            | ((red as u32) << 24)
            | ((green as u32) << 16)
            | ((blue as u32) << 8)
            | alpha as u32;

        let current_time = Utc::now();
        let mut hasher = Sha1::new();
        hasher.update(&logo.file_name().unwrap().as_encoded_bytes());
        hasher.update(current_time.to_rfc2822());
        let dir_hash = hasher.finalize();
        let dest_path = Path::from_hash(dest_base_path, &dir_hash);

        tokio::fs::create_dir(&dest_path.0.parent().unwrap())
            .await
            .unwrap();
        tokio::fs::copy(&logo, &dest_path).await?;

        let file_uuid = Uuid::new_v4();
        let dest_path_str = dest_path.0.to_string_lossy();
        match sqlx::query(
            "INSERT INTO \"file\" (uid, filename, \"type\", location) VALUES ($1, $2, $3, $4)",
        )
        .bind(file_uuid)
        .bind(filename.to_string_lossy())
        .bind(mime_type.to_string())
        .bind(&dest_path_str[dest_path_str.len() - dir_hash.len() * 2 - 1..])
        .execute(db)
        .await
        {
            Err(e) => return Err(BrandingCreateError::Sqlx(e)),
            _ => {}
        };

        let branding_dir = std::env::var("BRANDING_PATH").unwrap();
        let mut branding_conf_path =
            std::path::PathBuf::with_capacity(branding_dir.len() + CONFIG_FILE_NAME.len() + 2);
        branding_conf_path.push(branding_dir);
        branding_conf_path.push(CONFIG_FILE_NAME);
        let result = Branding {
            color,
            logo_file_uuid: file_uuid,
        };
        let result_string = match serde_json::to_string(&result) {
            Ok(res) => res,
            Err(e) => return Err(BrandingCreateError::Serde(e)),
        };
        if let Err(e) = tokio::fs::write(branding_conf_path, result_string).await {
            return Err(BrandingCreateError::Io(e));
        };
        Ok(result)
    }
}

pub mod http {
    use super::*;
    use axum::{
        Json,
        extract::State,
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    pub async fn get() -> Response {
        let branding_dir = std::env::var("BRANDING_PATH").unwrap();
        let mut path =
            std::path::PathBuf::with_capacity(branding_dir.len() + CONFIG_FILE_NAME.len() + 2);
        path.push(branding_dir);
        path.push(CONFIG_FILE_NAME);
        let branding_file = match tokio::fs::read(path).await {
            Ok(b) => b,
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        match serde_json::from_slice::<Branding>(&branding_file) {
            Ok(val) => Json(val).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn create_default(State(state): State<crate::AppState>) -> Response {
        let branding_path = std::env::var("BRANDING_PATH").unwrap();
        let mut logo_path =
            std::path::PathBuf::with_capacity(branding_path.len() + DEFAULT_LOGO_NAME.len() + 2);
        logo_path.push(branding_path);
        logo_path.push(DEFAULT_LOGO_NAME);
        match Branding::create_in_db(255, 0, 0, 255, logo_path, &state.media_path, &state.db).await
        {
            Ok(b) => (StatusCode::CREATED, Json(b)).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
