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
    Course,
    Template,
}

use const_format::concatcp;
const USER_TABLE: &str = "user";
const ROLE_TABLE: &str = "role";
const GROUP_TABLE: &str = "group";
const FILE_TABLE: &str = "file";
const COURSE_TABLE: &str = "course";
const TEMPLATE_TABLE: &str = "template";
impl Type {
    pub fn table_name(&self) -> &'static str {
        match self {
            Self::User => USER_TABLE,
            Self::Role => ROLE_TABLE,
            Self::Group => GROUP_TABLE,
            Self::File => FILE_TABLE,
            Self::Course => COURSE_TABLE,
            Self::Template => TEMPLATE_TABLE,
        }
    }

    pub fn permission_id_query(&self) -> &'static str {
        // "SELECT 1 FROM {table_name}_permissions \
        // LEFT JOIN user_has_role on user_has_role.role_id = {table_name}_permissions.role_id \
        // WHERE (user_has_role.user_id = $1 OR {table_name}_permissions.user_id = $1) \
        // AND ($2::bit(16) & permission) <> B'0'::bit(16) AND resource_id = $3",
        const START: &str = "SELECT 1 FROM ";
        const SECOND: &str = "_permissions LEFT JOIN user_has_role ON user_has_role.role_id = ";
        const THIRD: &str = "_permissions.role_id WHERE (user_has_role.user_id = $1 OR ";
        const END: &str = "_permissions.user_id = $1) AND ($2::int::bit(16) & permission) <> B'0'::bit(16) AND resource_id = $3";

        match self {
            Self::User => {
                concatcp!(
                    START, USER_TABLE, SECOND, USER_TABLE, THIRD, USER_TABLE, END
                )
            }
            Self::Role => {
                concatcp!(
                    START, ROLE_TABLE, SECOND, ROLE_TABLE, THIRD, ROLE_TABLE, END
                )
            }
            Self::Group => {
                concatcp!(
                    START,
                    GROUP_TABLE,
                    SECOND,
                    GROUP_TABLE,
                    THIRD,
                    GROUP_TABLE,
                    END
                )
            }
            Self::File => {
                concatcp!(
                    START, FILE_TABLE, SECOND, FILE_TABLE, THIRD, FILE_TABLE, END
                )
            }
            Self::Course => {
                concatcp!(
                    START,
                    COURSE_TABLE,
                    SECOND,
                    COURSE_TABLE,
                    THIRD,
                    COURSE_TABLE,
                    END
                )
            }
            Self::Template => {
                concatcp!(
                    START,
                    TEMPLATE_TABLE,
                    SECOND,
                    TEMPLATE_TABLE,
                    THIRD,
                    TEMPLATE_TABLE,
                    END
                )
            }
        }
    }
}
