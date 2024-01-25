use crate::func::{decoration::StyleDecoration, greetings::say_hello_to_factory};
use crate::oop::{decoration::ColorDecoration, greetings::Greetings};

mod func;
mod oop;

fn main() {
    oop_way();
    func_way();
}

fn oop_way() {
    let decoration = Box::new(ColorDecoration);
    let greetings = Greetings::new(decoration);

    let name = String::from("OOP");
    let to_display = greetings.say_hello_to(name);

    println!("{}", to_display);
}

fn func_way() {
    let decoration = &StyleDecoration;
    let say_hello_to = say_hello_to_factory(decoration);

    let name = String::from("Functional programming");
    let to_display = say_hello_to(name);

    println!("{}", to_display);
}
