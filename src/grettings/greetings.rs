use std::io::{Error, ErrorKind};

pub trait Decoration {
    fn apply_to(&self, value: String) -> String;
}

pub fn say_hello_with<'a>(
    decoration: &'a impl Decoration,
) -> Box<dyn Fn(String) -> Result<String, Error> + 'a> {
    let say_hello_to = |name: String| {
        let hello = format!("Hello {}", &name);
        decoration.apply_to(hello)
    };

    Box::new(move |name| match name.as_str() {
        "" => Err(Error::new(ErrorKind::InvalidData, "empty name")),
        _ => Ok(say_hello_to(name)),
    })
}
