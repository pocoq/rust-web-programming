use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaskItem{
	pub title: String,
	pub status: String
}