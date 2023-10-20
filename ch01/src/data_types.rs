use std::collections::HashMap;

fn print(message: String){
	println!("{}", message);
}

pub enum SomeValue{
	StringValue(String),
	IntValue(i32)
}
#[derive(Debug)]
pub enum CharacterValue{
	Name(String),
	Age(i32),
	Items(Vec<String>)
}

fn error_check(check: bool) -> Result<i8, &'static str>{
	if check{
		Err("This is an error")
	}
	else{
		Ok(1)
	}
}
pub fn main() {
    let message = String::from("Hello, world");
	print(message);
	
	let number1 = 255i16;
	let number2 = 5i8;
	let result = number1 + number2 as i16;
	println!("{}", result);

	let mut mutable_array :[i32; 3] = [1, 2, 0];
	mutable_array[2] = 3;
	println!("{:?}", mutable_array);
	println!("{}", mutable_array[2]);

	let sliced_array :[i32; 100] = [0; 100];
	println!("Len of sliced array: {}", sliced_array.len());
	println!("Item 5 to 8 of array: {:?}", &sliced_array[5 .. 8]);

	let multi_array: [SomeValue; 4] = [
		SomeValue::StringValue(String::from("one")),
		SomeValue::IntValue(2),
		SomeValue::StringValue(String::from("three")),
		SomeValue::IntValue(4)
	];

	for i in multi_array{
		match i {
			SomeValue::StringValue(data) => {
				println!("The string is: {}", data);
			},
			SomeValue::IntValue(data) => {
				println!("The int is: {}", data);
			}
		}
	}

	let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
	println!("{:?}", string_vector);
	string_vector.push("four");
	println!("{:?}", string_vector);

	let mut profile: HashMap<&str, CharacterValue> = HashMap::new();
	profile.insert("name", CharacterValue::Name("pocoq".to_string()));
	profile.insert("age", CharacterValue::Age(32));
	profile.insert("items", CharacterValue::Items(vec!["laptop".to_string(), "book".to_string(), "coat".to_string()]));
	println!("{:?}", profile);

	println!("{:?}", error_check(false));
	println!("{:?}", error_check(false).is_err());
	println!("{:?}", error_check(true));
	println!("{:?}", error_check(true).is_err());

	let result = error_check(true).expect("This has been caught");
	println!("{:?}", result);
}
