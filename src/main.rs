use std::{error::Error, io, process};

fn main() {
    let name = ask_for_name().unwrap_or_else(|err| {
        println!("Problem asking for name: {}", err);
        process::exit(1);
    });

    hello_rust::say_hello_to(&name);
}

fn ask_for_name() -> Result<String, Box<dyn Error>> {
    println!("What is your name ?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    Ok(String::from(name.trim()))
}
