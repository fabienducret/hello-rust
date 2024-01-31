pub trait Decoration {
    fn apply_to(&self, value: String) -> String;
}

pub struct Greetings {
    decoration: Box<dyn Decoration>,
}

impl Greetings {
    pub fn new(decoration: Box<dyn Decoration>) -> Self {
        Self { decoration }
    }

    pub fn say_hello_to(&self, name: String) -> String {
        let hello = format!("Hello {}", &name);

        self.decoration.apply_to(hello)
    }
}
