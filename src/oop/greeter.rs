pub trait Formatter {
    fn format(&self, value: String) -> String;
}

pub struct Greeter {
    formatter: Box<dyn Formatter>,
}

impl Greeter {
    pub fn new(formatter: Box<dyn Formatter>) -> Greeter {
        Greeter { formatter }
    }

    pub fn say_hello_to(&self, name: String) -> String {
        let hello = format!("Hello, {}", &name);

        self.formatter.format(hello)
    }
}
