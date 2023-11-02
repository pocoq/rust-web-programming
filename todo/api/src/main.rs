mod entities;
mod state;

use serde_json::{json, value::Value, Map};
use std::env;

use entities::initialize;
use entities::task_status::TaskStatus;
use entities::ItemTypes;
use state::{read_to_file, write_to_file};

use crate::entities::traits::{delete::Delete, edit::Edit, get::Get};

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_to_file("./state.json", &mut state);
}
