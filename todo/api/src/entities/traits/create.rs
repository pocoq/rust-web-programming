use crate::state::write_to_file;
use serde_json::{json, value::Value, Map};

pub trait Create {
    fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(status));
        write_to_file("./state.json", state);
        println!("\n\n {} is being created", title);
    }
}
