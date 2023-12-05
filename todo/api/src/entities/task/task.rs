// pub mod task_items;

use serde::Serialize;
use super::super::task_status::TaskStatus;

#[derive(Serialize)]
pub struct Task {
    pub title: String,
    pub status: TaskStatus,
}

