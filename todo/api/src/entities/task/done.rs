use super::super::task_status::TaskStatus;
use super::super::traits::{delete::Delete, edit::Edit, get::Get};
use super::task::Task;

pub struct Done {
    pub task_base: Task,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let task = Task {
            title: title.to_string(),
            status: TaskStatus::DONE,
        };
        return Done { task_base: task };
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
