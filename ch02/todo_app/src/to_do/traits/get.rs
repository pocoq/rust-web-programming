use serde_json::{Map, value::Value};

pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let item: Option<&Value> = state.get(title);
        match item {
            Some(result) => {
                println!("\n\nItem: {}", title);
                println!("Statue: {} \n\n", result);
            }
            None => println!("item: {} was not found", title),
        }
    }
}
