use std::{error::Error, io, process};

use crate::func::{decoration::StyleDecoration, greetings::say_hello_with};
use crate::oop::{decoration::ColorDecoration, greetings::Greetings};

mod func;
mod oop;

fn main() {
    let name = ask_for_name().unwrap_or_else(|err| {
        println!("Problem asking for name: {}", err);
        process::exit(1);
    });

    display_oop_way(&name);
    display_func_way(&name);
}

fn ask_for_name() -> Result<String, Box<dyn Error>> {
    println!("What is your name ?");

    let mut raw_name = String::new();
    io::stdin().read_line(&mut raw_name)?;

    Ok(String::from(raw_name.trim()))
}

fn display_oop_way(name: &String) {
    let decoration = Box::new(ColorDecoration);
    let greetings = Greetings::new(decoration);

    let to_display = greetings.say_hello_to(String::from(name));

    println!("{}", to_display);
}

fn display_func_way(name: &String) {
    let decoration = &StyleDecoration;
    let say_hello_to = say_hello_with(decoration);

    let to_display = say_hello_to(String::from(name));

    println!("{}", to_display);
}
