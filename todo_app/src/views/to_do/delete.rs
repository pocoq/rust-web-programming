use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    processes::process_input,
    state::read_file,
    to_do::{task_status::TaskStatus, to_do_factory},
	jwt::JwToken
};
use actix_web::{web, HttpResponse};
use serde_json::{value::Value, Map};

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
	println!("Here is the message from token: {}", token.message);
	
    let state: Map<String, Value> = read_file("./state.json");
    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound().json(format!("{} not in state", &to_do_item.title))
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
    process_input(existing_item, "delete".to_owned(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
