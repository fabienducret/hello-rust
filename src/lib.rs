use crate::func::{decoration::StyleDecoration, greetings::say_hello_with};
use crate::oop::{decoration::ColorDecoration, greetings::Greetings};

mod func;
mod oop;

pub fn display_oop_way(name: &String) {
    let decoration = Box::new(ColorDecoration);
    let greetings = Greetings::new(decoration);

    let to_display = greetings.say_hello_to(String::from(name));

    println!("{}", to_display);
}

pub fn display_func_way(name: &String) {
    let decoration = &StyleDecoration;
    let say_hello_to = say_hello_with(decoration);

    let to_display = say_hello_to(String::from(name));

    println!("{}", to_display);
}
