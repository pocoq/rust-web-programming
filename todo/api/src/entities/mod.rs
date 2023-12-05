pub mod task;
pub mod task_status;
pub mod traits;

use task::{done::Done, pending::Pending};
use task_status::TaskStatus;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn initialize_task(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
