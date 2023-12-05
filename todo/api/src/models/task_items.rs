use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json::{value::Value, Map};
use std::vec::Vec;

use crate::entities::{initialize_task, task::task::Task, task_status::TaskStatus, ItemTypes};
use crate::state::read_file;

#[derive(Serialize)]
pub struct TaskItems {
    pub pending_items: Vec<Task>,
    pub done_items: Vec<Task>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl TaskItems {
    pub fn new(input_items: Vec<ItemTypes>) -> TaskItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending.push(packed.task_base),
                ItemTypes::Done(packed) => done.push(packed.task_base),
            }
        }

        let done_count: i8 = done.len() as i8;
        let pending_count: i8 = pending.len() as i8;

        return TaskItems {
            pending_items: pending,
            pending_item_count: pending_count,
            done_items: done,
            done_item_count: done_count,
        };
    }

    pub fn get_state() -> TaskItems {
        let state: Map<String, Value> = read_file("./state.json");
        let mut tasks = Vec::new();

        for (key, value) in state {
            let status = TaskStatus::from_string(value.as_str().unwrap().to_string());
            let item: ItemTypes = initialize_task(&key, status);
            tasks.push(item);
        }
        return TaskItems::new(tasks);
    }
}

impl Responder for TaskItems{
	type Body = BoxBody;
	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body>{
		let body = serde_json::to_string(&self).unwrap();
		HttpResponse::Ok().content_type(ContentType::json()).body(body)
	}
}
