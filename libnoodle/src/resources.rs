use serde::{Deserialize, Serialize};

pub mod branding;
pub mod content_section;
pub mod course;
pub mod file;
pub mod template;

#[derive(Serialize, Deserialize, sqlx::Type, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    User,
    Role,
    Group,
    File,
}

impl Type {
    use const_concat;
    pub fn table_name(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Role => "role",
            Self::Group => "group",
            Self::File => "file",
        }
    }

    pub fn permission_id_query(&self) -> &'static str {
        const START: &str = "SELECT 1 FROM ";
        const END: &str = " _permissions WHERE user_id = $1 AND ($2::bit(16) & permission) <> B'0'::bit(16) AND (resource_id = NULL OR resource_id = $3)";
        match self {
            Self::User => {
                const_concat!(START, "user", END)
            }
            Self::Role => {
                const_concat!(START, "role", END)
            }
            Self::Group => {
                const_concat!(START, "group", END)
            }
            Self::File => {
                const_concat!(START, "file", END)
            }
        }
    }
}
