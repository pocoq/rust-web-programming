use super::super::traits::{create::Create, edit::Edit, get::Get};
use super::super::task_status::TaskStatus;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Pending { super_struct: base };
    }
}
