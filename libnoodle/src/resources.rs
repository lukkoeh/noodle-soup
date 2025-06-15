use serde::{Deserialize, Serialize};

pub mod branding;
pub mod file;
pub mod course;
pub mod template;
pub mod content_section;

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    User,
    Role,
    Group,
    File,
}
