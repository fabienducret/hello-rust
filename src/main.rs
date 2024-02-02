use crate::grettings::{decoration::StyleDecoration, greetings::say_hello_with};
use std::{error::Error, io, process};

mod grettings;

fn main() {
    let decoration = &StyleDecoration;
    let say_hello = say_hello_with(decoration);

    ask_for_name()
        .map(say_hello)
        .map_or_else(handle_error, print);
}

fn ask_for_name() -> Result<String, Box<dyn Error>> {
    println!("What is your name ?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    Ok(String::from(name.trim()))
}

fn handle_error(err: Box<dyn Error>) {
    println!("{}", err);
    process::exit(1);
}

fn print(to_display: String) {
    println!("{}", to_display);
}
