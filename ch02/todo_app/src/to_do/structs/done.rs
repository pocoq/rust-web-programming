use super::super::{
    enums::TaskStatus, traits::delete::Delete, traits::edit::Edit, traits::get::Get,
};
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        return Done { super_struct: base };
    }
}
