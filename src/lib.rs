use crate::with_func::{decoration::StyleDecoration, greetings::say_hello_with};
use crate::with_struct::{decoration::ColorDecoration, greetings::Greetings};

mod with_func;
mod with_struct;

pub fn say_hello_to(name: &String) {
    println!("{}", with_struct(name));
    println!("{}", with_func(name));
}

fn with_struct(name: &String) -> String {
    let decoration = Box::new(ColorDecoration);
    let greetings = Greetings::new(decoration);

    greetings.say_hello_to(String::from(name))
}

fn with_func(name: &String) -> String {
    let decoration = &StyleDecoration;
    let say_hello_to = say_hello_with(decoration);

    say_hello_to(String::from(name))
}
