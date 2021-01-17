#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: String::from(name),
            age,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut people = vec![
        Person::new("Zoe", 25),
        Person::new("Al", 60),
        Person::new("John", 1),
    ];

    people.sort();
    println!("{:?}", people);

    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("{:?}", people);
}
