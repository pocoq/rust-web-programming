use crate::entities::{initialize_task, task_status::TaskStatus};
use crate::models::{task_item::TaskItem, task_items::TaskItems};
use crate::processes::process_input;
use crate::state::read_file;
use actix_web::{web, HttpResponse};
use serde_json::{value::Value, Map};


pub async fn edit(req: HttpRequest) -> impl Responder{
	let state: Map<String, Value> = read_file("./state.json");
	let title: String = req.match_info().get("title").unwrap().to_string();
	let item = state.get(title);
	let notify = format!("{} updated", title);
	match item{
		Some(result) => {
			println!("Status: {} \n\n", result);
			let status = req.match_info().get("status").unwrap().to_string();
			if status != result{
				let updated_status: TaskStatus = match result{
					"DONE" => TaskStatus::PENDING,
					"PENDING" => TaskStatus::DONE
				}
				let updated_item = initialize_task(&title.as_str(), updated_status);
				process_input(updated_item, "edit".to_string(), &state);
			}
		}
		None => notify = format!("item: {} was not found", title),
	}
	return notify;
}