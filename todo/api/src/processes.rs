use serde_json::{value::Value, Map};

use crate::entities::task::{done::Done, pending::Pending};
use crate::entities::traits::{create::Create, delete::Delete, edit::Edit, get::Get};
use crate::entities::ItemTypes;

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.task_base.title, &state),
        "create" => item.create(
            &item.task_base.title,
            &item.task_base.status.stringify(),
            &mut state,
        ),
        "edit" => item.set_to_done(&item.task_base.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}
fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.task_base.title, &state),
        "delete" => item.delete(&item.task_base.title, &mut state),
        "edit" => item.set_to_pending(&item.task_base.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state),
    }
}
