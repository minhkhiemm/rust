use std::fmt;
use std::string;

#[derive(Debug)]
enum AnimalType {
    Dog,
}

#[derive(Debug)]
struct Dog {
    animal_type: AnimalType,
    name: String,
    leg: i32,
}

impl Dog {
    fn new() -> Dog {
        Dog {
            animal_type: AnimalType::Dog,
            name: String::from("doggy"),
            leg: 4,
        }
    }
}

impl fmt::Display for AnimalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let animal_type = match *self {
            AnimalType::Dog => "Dog",
        };
        write!(f, "{}", animal_type)
    }
}

trait Introducer {
    fn introduce(&self) -> String;
}

impl Introducer for string::String {
    fn introduce(&self) -> String {
        format!("{}", self)
    }
}

impl Introducer for Dog {
    fn introduce(&self) -> String {
        format!(
            "Hi, i'm: {}, i'm a: {}, i have {} legs",
            self.name, self.animal_type, self.leg
        )
    }
}

// trait as parameters
fn say(introducer: impl Introducer) {
    println!("let me introduce my self: {}", introducer.introduce())
}

fn main() {
    let dog = Dog::new();
    let introduce = dog.introduce();
    say(introduce);
}
