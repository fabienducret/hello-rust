use std::io;

use crate::func::{decoration::StyleDecoration, greetings::say_hello_to_factory};
use crate::oop::{decoration::ColorDecoration, greetings::Greetings};

mod func;
mod oop;

fn main() {
    let name = ask_for_name();

    display_oop_way(&name);
    display_func_way(&name);
}

fn ask_for_name() -> String {
    println!("What is your name ?");

    let mut raw_name = String::new();

    io::stdin()
        .read_line(&mut raw_name)
        .expect("Failed to read line");

    String::from(raw_name.trim())
}

fn display_oop_way(name: &String) {
    let decoration = Box::new(ColorDecoration);
    let greetings = Greetings::new(decoration);

    let to_display = greetings.say_hello_to(String::from(name));

    println!("{}", to_display);
}

fn display_func_way(name: &String) {
    let decoration = &StyleDecoration;
    let say_hello_to = say_hello_to_factory(decoration);

    let to_display = say_hello_to(String::from(name));

    println!("{}", to_display);
}
