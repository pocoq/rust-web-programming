use super::super::task_status::TaskStatus;
use super::super::traits::{create::Create, edit::Edit, get::Get};
use super::task::Task;

pub struct Pending {
    pub task_base: Task,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let task = Task {
            title: title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Pending { task_base: task };
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
