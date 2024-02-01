use std::{error::Error, io, process};

fn main() {
    match ask_for_name() {
        Err(err) => exit_process_with(err),
        Ok(name) => hello_rust::say_hello_to(&name),
    }
}

fn ask_for_name() -> Result<String, Box<dyn Error>> {
    println!("What is your name ?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    Ok(String::from(name.trim()))
}

fn exit_process_with(err: Box<dyn Error>) {
    println!("{}", err);
    process::exit(1);
}
