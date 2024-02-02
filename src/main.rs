use crate::grettings::{decoration::StyleDecoration, greetings::say_hello_with};
use std::{io, process};

mod grettings;

fn main() {
    let say_hello = say_hello_with(&StyleDecoration);

    ask_for_name()
        .and_then(say_hello)
        .map_or_else(handle_error, print_hello);
}

fn ask_for_name() -> Result<String, io::Error> {
    println!("What is your name ?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    Ok(String::from(name.trim()))
}

fn handle_error(err: io::Error) {
    println!("Error: {}", err);
    process::exit(1);
}

fn print_hello(hello: String) {
    println!("{}", hello);
}
