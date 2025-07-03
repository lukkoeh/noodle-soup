use serde::{Deserialize, Serialize};
use sqlx::{Decode, PgPool, Postgres, error::ErrorKind};
use thiserror::Error;

use crate::resources;

#[derive(Serialize, Deserialize, sqlx::Type, Hash, PartialEq, Eq)]
pub struct Permission {
    pub(super) subject: resources::Type,
    pub(super) ops: Operations,
    pub(super) ids: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct PermissionQueryParam {
    pub edit: Option<bool>,
    pub view: Option<bool>,
}

#[derive(Serialize, Deserialize, sqlx::Type, Hash, PartialEq, Eq, Clone, Copy)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Operations(i16);

impl From<i16> for Operations {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

#[allow(dead_code)]
impl Operations {
    pub const CREATE: Operations = Self(0b00000001);
    pub const READ: Operations = Self(0b00000010);
    pub const UPDATE: Operations = Self(0b00000100);
    pub const DELETE: Operations = Self(0b00001000);

    pub fn new(create: bool, read: bool, update: bool, delete: bool) -> Self {
        Self(create as i16 | (read as i16) << 1 | (update as i16) << 2 | (delete as i16) << 3)
    }

    pub fn can_create(&self) -> bool {
        (Self::CREATE.0 & self.0) != 0
    }
    pub fn can_read(&self) -> bool {
        (Self::READ.0 & self.0) != 0
    }
    pub fn can_update(&self) -> bool {
        (Self::UPDATE.0 & self.0) != 0
    }
    pub fn can_delete(&self) -> bool {
        (Self::DELETE.0 & self.0) != 0
    }
}

#[derive(Debug, Error)]
pub enum ResourceCreateError {
    #[error("SQLX Error")]
    Sqlx(#[from] sqlx::Error),
    #[error("conflicting parameters")]
    Conflict,
    #[error("malformed parameters")]
    BadParam,
    #[error("access denied")]
    AccessDenied,
}

#[derive(Serialize, Deserialize, Decode, sqlx::FromRow)]
#[sqlx(no_pg_array)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    role_id: i64,
    name: String,
    permissions: Vec<Permission>,
}

#[derive(Serialize, sqlx::FromRow)]
#[sqlx(no_pg_array)]
#[serde(rename_all = "camelCase")]
pub struct RoleRow {
    #[sqlx(rename = "id")]
    role_id: i64,
    name: String,
    permissions: sqlx::types::Json<Vec<Permission>>,
    group: Option<i64>,
}

impl RoleRow {
    pub async fn from_user_id(
        db: &PgPool,
        requesting_user_id: i64,
        target_user_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        match sqlx::query_as::<_, RoleRow>(
            "SELECT id, \"name\", permissions, \"group\" FROM \"role\" r \
JOIN user_has_role ON role_id = r.id \
WHERE EXISTS (\
SELECT 1 FROM role_permissions rp \
WHERE (rp.resource_id = r.id OR rp.resource_id IS NULL) \
AND (rp.permission & $3::int::bit(16)) <> B'0'::bit(16) \
AND (rp.user_id = $2 OR rp.role_id IN (SELECT role_id FROM user_has_role WHERE user_id = $2)))",
        )
        .bind(target_user_id)
        .bind(requesting_user_id)
        .bind(Operations::READ)
        .fetch_all(db)
        .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }

    pub async fn create_in_db(
        db: &PgPool,
        name: &str,
        permissions: &[Permission],
        group: Option<i64>,
        user_id: i64,
    ) -> Result<i64, ResourceCreateError> {
        match super::can_create(resources::Type::Role, user_id, db).await {
            Err(e) => return Err(ResourceCreateError::Sqlx(e)),
            Ok(false) => return Err(ResourceCreateError::AccessDenied),
            Ok(true) => {}
        };

        match sqlx::query_scalar::<_, i64>(
            "INSERT INTO role (\"name\", permissions, \"group\") VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(name)
        .bind(sqlx::types::Json(permissions))
        .bind(group)
        .fetch_one(db)
        .await
        {
            Ok(id) => Ok(id),
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    if e.kind() == ErrorKind::UniqueViolation {
                        Err(ResourceCreateError::Conflict)
                    } else {
                        Err(ResourceCreateError::Sqlx(sqlx::Error::Database(e)))
                    }
                }
                _ => Err(ResourceCreateError::Sqlx(e)),
            },
        }
    }
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
    Role,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    group_id: i64,
    name: String,
    shortname: String,
    kind: GroupKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<Box<Group>>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct GroupRow {
    #[sqlx(rename = "id")]
    group_id: i64,
    name: String,
    shortname: String,
    kind: GroupKind,
    parent: Option<i64>,
}

impl GroupRow {
    pub async fn from_user_id(
        db: &PgPool,
        requesting_user_id: i64,
        target_user_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        match sqlx::query_as::<_, GroupRow>(
            "SELECT g.id, g.\"name\", g.shortname, g.kind, g.parent FROM \"group\" g \
JOIN user_in_group uig ON uig.group_id = g.id \
WHERE EXISTS (\
SELECT 1 FROM group_permissions gp \
WHERE (gp.resource_id = g.id OR gp.resource_id IS NULL) \
AND (gp.permission & $3::int::bit(16)) <> B'0'::bit(16) \
AND (gp.user_id = $2 OR gp.role_id IN (SELECT role_id FROM user_has_role WHERE user_id = $2)))",
        )
        .bind(target_user_id)
        .bind(requesting_user_id)
        .bind(Operations::READ)
        .fetch_all(db)
        .await
        {
            Ok(g) => Ok(g),
            Err(e) => Err(e),
        }
    }

    pub async fn create_in_db(
        db: &PgPool,
        name: &str,
        shortname: &str,
        kind: &GroupKind,
        parent: Option<i64>,
        user_id: i64,
    ) -> Result<i64, ResourceCreateError> {
        match super::can_create(resources::Type::Group, user_id, db).await {
            Err(e) => return Err(ResourceCreateError::Sqlx(e)),
            Ok(false) => return Err(ResourceCreateError::AccessDenied),
            Ok(true) => {}
        };

        match sqlx::query_scalar::<_, i64>(
            "INSERT INTO \"group\"(\"name\", shortname, kind, parent) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(name)
        .bind(shortname)
        .bind(kind)
        .bind(parent)
        .fetch_one(db)
        .await
        {
            Ok(id) => Ok(id),
            Err(e) => match e {
                sqlx::Error::Database(e) => {
                    if e.kind() == ErrorKind::UniqueViolation {
                        Err(ResourceCreateError::Conflict)
                    } else {
                        Err(ResourceCreateError::Sqlx(sqlx::Error::Database(e)))
                    }
                }
                _ => Err(ResourceCreateError::Sqlx(e)),
            },
        }
    }
}

pub fn add_users_to_groups_query<'a>()
-> sqlx::query::Query<'a, Postgres, <Postgres as sqlx::Database>::Arguments<'a>> {
    sqlx::query(
        "INSERT INTO \"user_in_group\"(user_id, group_id) SELECT * FROM UNNEST($1::int8[], $2::int8[])",
    )
}

pub fn remove_user_from_role_groups_query<'a>()
-> sqlx::query::Query<'a, Postgres, <Postgres as sqlx::Database>::Arguments<'a>> {
    sqlx::query(
        "DELETE FROM \"user_in_group\"\
WHERE user_id = $1 AND group_id IN \
(SELECT \"role\".\"group\"::int8 FROM \"role\" WHERE id = ANY($2))",
    )
}

pub fn add_users_to_role_group_query<'a>()
-> sqlx::query::Query<'a, Postgres, <Postgres as sqlx::Database>::Arguments<'a>> {
    sqlx::query(
        "INSERT INTO \"user_in_group\"(user_id, group_id) \
SELECT uid, \"group\"::int8 FROM UNNEST($1::int8[]) AS u(uid) CROSS JOIN (SELECT \"group\" FROM \"role\" WHERE id = $2)",
    )
}

pub fn add_user_to_role_groups_query<'a>()
-> sqlx::query::Query<'a, Postgres, <Postgres as sqlx::Database>::Arguments<'a>> {
    sqlx::query(
        "INSERT INTO \"user_in_group\"(user_id, group_id) \
SELECT $1::int8, \"role\".\"group\"::int8 FROM \"role\" WHERE id = ANY($2)",
    )
}

pub fn add_users_to_roles_query<'a>()
-> sqlx::query::Query<'a, Postgres, <Postgres as sqlx::Database>::Arguments<'a>> {
    sqlx::query(
        "INSERT INTO \"user_has_role\"(user_id, role_id) SELECT * FROM UNNEST($1::int8[], $2::int8[])",
    )
}

#[derive(Serialize, Deserialize)]
pub struct GroupDescription {
    name: String,
    shortname: String,
    kind: GroupKind,
    parent: Option<i64>,
}

pub mod http;
