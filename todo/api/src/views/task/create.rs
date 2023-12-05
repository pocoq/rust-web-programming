use actix_web::HttpRequest;
use serde_json::{value::Value, Map};

use crate::entities::{initialize_task, task_status::TaskStatus};
use crate::processes::process_input;
use crate::state::read_file;

pub async fn create(req: HttpRequest) -> String{
	let state: Map<String, Value> = read_file("./state.json");
	let title: String = req.match_info().get("title").unwrap().to_string();
	let item = initialize_task(&title.as_str(), TaskStatus::PENDING);
	process_input(item, "create".to_string(), &state);
	return format!("{} created", title);
}
