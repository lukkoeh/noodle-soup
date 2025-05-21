use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    uid: i64,
    #[serde(rename = "type")]
    mime_type: String,
    data: String,
}

pub mod http {
    use super::*;
    use axum::Json;
    use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};

    pub async fn get_all() -> Json<Vec<File>> {
        Json(vec![
            File {
                uid: 1,
                mime_type: mime::IMAGE_JPEG.to_string(),
                data: STANDARD_NO_PAD.encode("hello")
            };
            1
        ])
    }
}
