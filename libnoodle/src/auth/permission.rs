use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};

use crate::resources;

#[derive(Serialize, Deserialize, sqlx::Type)]
pub struct Permission {
    subject: resources::Type,
    ops: Operations,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(transparent)]
struct Operations(i8);

impl From<i8> for Operations {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

#[allow(dead_code)]
impl Operations {
    pub fn new(create: bool, read: bool, update: bool, delete: bool) -> Self {
        Self(create as i8 | (read as i8) << 1 | (update as i8) << 2 | (delete as i8) << 3)
    }

    pub fn can_create(&self) -> bool {
        (0b00000001 & self.0) != 0
    }
    pub fn can_read(&self) -> bool {
        (0b00000010 & self.0) != 0
    }
    pub fn can_update(&self) -> bool {
        (0b00000100 & self.0) != 0
    }
    pub fn can_delete(&self) -> bool {
        (0b00001000 & self.0) != 0
    }
}

#[derive(Serialize, Deserialize, Decode, sqlx::FromRow)]
#[sqlx(no_pg_array)]
pub struct Role {
    id: i64,
    name: String,
    permissions: Vec<Permission>,
}

#[derive(Serialize, sqlx::FromRow)]
#[sqlx(no_pg_array)]
pub struct RoleRow {
    id: i64,
    name: String,
    permissions: sqlx::types::Json<Vec<Permission>>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleDescription {
    name: String,
    permissions: Vec<Permission>,
}

impl From<Role> for RoleDescription {
    fn from(value: Role) -> Self {
        Self {
            name: value.name,
            permissions: value.permissions,
        }
    }
}

#[derive(Serialize, Deserialize, sqlx::types::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "group_kind", rename_all = "lowercase")]
pub enum GroupKind {
    Organization,
    Learning,
    Contact,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    id: i64,
    name: String,
    kind: GroupKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<Box<Group>>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupRow {
    id: i64,
    name: String,
    kind: GroupKind,
    parent: Option<i64>,
}

pub fn add_user_to_group_query<'a>()
-> sqlx::query::Query<'a, Postgres, <Postgres as sqlx::Database>::Arguments<'a>> {
    sqlx::query(
        "INSERT INTO \"user_in_group\"(user_id, group_id) SELECT * FROM UNNEST($1::bigserial[], $2::bigserial[])",
    )
}

#[derive(Serialize, Deserialize)]
pub struct GroupDescription {
    name: String,
    kind: GroupKind,
    parent: Option<i64>,
}

pub mod http;
