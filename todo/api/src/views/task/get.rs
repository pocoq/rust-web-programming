use actix_web::Responder;
use crate::models::task_items::TaskItems;

pub async fn get() -> impl Responder{
	return TaskItems::get_state();	
}