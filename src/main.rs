use crate::hello_func::{formatter::DefaultFormatter as FuncFormatter, hello::hello_factory};
use crate::hello_object::{formatter::DefaultFormatter as ObjectFormatter, hello::Hello};

mod hello_func;
mod hello_object;

fn main() {
    object_way();
    func_way();
}

fn object_way() {
    let formatter = Box::new(ObjectFormatter);
    let hello = Hello::new(formatter);

    let name = String::from("OOP");
    let to_display = hello.to(name);

    println!("{}", to_display);
}

fn func_way() {
    let formatter = &FuncFormatter;
    let hello_to = hello_factory(formatter);

    let name = String::from("Functional programming");
    let to_display = hello_to(name);

    println!("{}", to_display);
}
