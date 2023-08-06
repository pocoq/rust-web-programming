use serde::Serialize;

use super::super::task_status::TaskStatus;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
