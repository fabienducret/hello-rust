pub trait Decoration {
    fn apply_to(&self, value: String) -> String;
}

pub fn say_hello_with<'a>(decoration: &'a impl Decoration) -> Box<dyn Fn(String) -> String + 'a> {
    Box::new(move |name: String| {
        let hello = format!("Hello, {}", &name);

        decoration.apply_to(hello)
    })
}
