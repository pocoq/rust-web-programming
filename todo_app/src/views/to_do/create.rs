use actix_web::{HttpRequest, HttpResponse};
use serde_json::{value::Value, Map};

use crate::{
    processes::process_input,
    state::read_file,
    to_do::{task_status::TaskStatus, to_do_factory},
	json_serialization::to_do_items::ToDoItems
};

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let title: String = req.match_info().get("title").unwrap().to_string();
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING);
    process_input(item, "create".to_string(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state())
}