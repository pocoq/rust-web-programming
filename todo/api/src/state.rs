use serde_json::{json, value::Value, Map};
use std::{fs, fs::File, io::Read};

pub fn read_to_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name.to_string()).unwrap();
    let mut data = String::new();
    file.read_to_file(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file testing");
}
