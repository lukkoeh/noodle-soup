use serde::{Deserialize, Serialize};

pub mod file;

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    User,
    Role,
    Group,
    File,
}
