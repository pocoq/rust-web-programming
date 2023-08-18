use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::vec::Vec;
use diesel::prelude::*;

use crate::{
	diesel,
    database::establish_connection,
    models::item::item::Item,
    schema::to_do,
    to_do::{structs::base::Base, task_status::TaskStatus, to_do_factory, ItemTypes},
};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_arrays = Vec::new();
        let mut done_arrays = Vec::new();
        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_arrays.push(packed.super_struct),
                ItemTypes::Done(packed) => done_arrays.push(packed.super_struct),
            }
        }
        let pending_count: i8 = pending_arrays.len() as i8;
        let done_count: i8 = done_arrays.len() as i8;

        return ToDoItems {
            pending_items: pending_arrays,
            pending_item_count: pending_count,
            done_items: done_arrays,
            done_item_count: done_count,
        };
    }

    pub fn get_state() -> ToDoItems {
        let connection = establish_connection();

        let mut array_buffer = Vec::new();
        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&connection)
            .unwrap();
		for item in items{
			let status = TaskStatus::from_string(item.status.as_str().to_string());
			let item = to_do_factory(&item.title, status);
			array_buffer.push(item);
		}
		return ToDoItems::new(array_buffer);
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
