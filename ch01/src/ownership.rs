pub fn main() {
    let one: i8 = 5;
    let outcome: &i8;
    {
        let two: i8 = 1;
        outcome = filter(&one, &two);
    }
    println!("{}", outcome);

    let dev2 = Person::new("Michell", 45);

    let dev = Person::new("Sam", 35)
        .with_friend(Box::new(dev2))
        .with_thought("it's rainning");
    println!("{:?}", dev);
}

fn filter<'a, 'b>(one: &'a i8, two: &'b i8) -> &'a i8 {
    if one > two {
        one
    } else {
        &0
    }
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friends: Friend,
}

#[derive(Debug)]
pub enum Friend {
    Person(Box<Person>),
    NIL,
}

impl Person {
    fn new(name: &str, age: i8) -> Person {
        Person {
            name: name.to_string(),
            age: age,
            current_thought: None,
            friends: Friend::NIL,
        }
    }

    fn with_thought(mut self, thought: &str) -> Person {
        self.current_thought = Some(thought.to_string());
        self
    }

    fn with_friend(mut self, friend: Box<Person>) -> Person {
        self.friends = Friend::Person(friend);
        self
    }
}
