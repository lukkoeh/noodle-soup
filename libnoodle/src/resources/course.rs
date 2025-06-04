#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    id: i64,
    name: String,
}

pub mod http {
    use super::Course;
    use crate::resources::http::Response;

    pub async fn get_by_uid(uid: i64) -> Response<Course> {
        Response::Ok(Course {
            uid,
            name: "Course 1".to_string(),
        })
    }

    pub async fn get_all(name: String) -> Response<Vec<Course>> {
        Response::Ok(vec![
            Course {
                id: 0,
                name: "Course 1".to_string(),
            },
            Course {
                id: 1,
                name: "Course 2".to_string(),
            },
        ])
    }

    pub async fn create(course: Course) -> Response<Course> {
        Response::Ok(course)
    }

    pub async fn update(course: Course) -> Response<Course> {
        Response::Ok(course)
    }

    pub async fn delete(id: i64) -> Response<()> {
        Response::Ok(())
    }
}
