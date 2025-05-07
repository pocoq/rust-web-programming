use crate::entities::{initialize_task, task_status::TaskStatus};
use crate::models::{task_item::TaskItem, task_items::TaskItems};
use crate::processes::process_input;
use crate::state::read_file;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{value::Value, Map};


pub async fn edit(item: web::Json<TaskItem>) -> HttpResponse{
	let state: Map<String, Value> = read_file("./state.json");
	let status: TaskStatus;
	match state.get(&item.title){
		Some(result) => {
			status = TaskStatus::new(result.as_str().unwrap());
		},
		None => {
			return HttpResponse::NotFound().json(format!("{} not in state", &item.title));
		}
	}
	let existing_item = initialize_task(&item.title.as_str(), status.clone());
	if &status.stringify() == &TaskStatus::from_string(item.status.as_str().to_string()).stringify(){
		return HttpResponse::Ok().json(TaskItems::get_state());
	}
	process_input(existing_item, "edit".to_owned(), &state);
	return HttpResponse::Ok().json(TaskItems::get_state());
}