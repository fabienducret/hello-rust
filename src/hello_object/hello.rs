pub trait Formatter {
    fn format(&self, value: String) -> String;
}

pub struct Hello {
    formatter: Box<dyn Formatter>,
}

impl Hello {
    pub fn new(formatter: Box<dyn Formatter>) -> Hello {
        Hello { formatter }
    }

    pub fn to(&self, name: String) -> String {
        let hello = format!("Hello, {}", &name);

        self.formatter.format(hello)
    }
}
