pub mod task_status;
pub mod structs;
pub mod traits;

use task_status::TaskStatus;
use structs::{done::Done, pending::Pending};

pub enum ItemTypes {
    Pending(Pending),
	Done(Done)
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
