pub mod calculate;
mod japanese;

use japanese::{Japanese, Konnitiwa};

struct Person {
    name: String,
}

impl Japanese for Person {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn run() {
    let tanaka = Person {
        name: String::from("Tanaka"),
    };
    println!("{}", tanaka.konnitiwa());
    println!("{}", tanaka.name());
}
