use crate::func::{formatter::DefaultFormatter as FuncFormatter, greeter::say_hello_to_factory};
use crate::oop::{formatter::DefaultFormatter as OOPFormatter, greeter::Greeter};

mod func;
mod oop;

fn main() {
    oop_way();
    func_way();
}

fn oop_way() {
    let formatter = Box::new(OOPFormatter);
    let greeter = Greeter::new(formatter);

    let name = String::from("OOP");
    let to_display = greeter.say_hello_to(name);

    println!("{}", to_display);
}

fn func_way() {
    let formatter = &FuncFormatter;
    let say_hello_to = say_hello_to_factory(formatter);

    let name = String::from("Functional programming");
    let to_display = say_hello_to(name);

    println!("{}", to_display);
}
