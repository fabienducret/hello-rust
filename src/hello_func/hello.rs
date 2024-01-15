pub trait Formatter {
    fn format(&self, value: String) -> String;
}

pub fn hello_factory<'a>(formatter: &'a impl Formatter) -> Box<dyn Fn(String) -> String + 'a> {
    Box::new(move |name: String| {
        let hello = format!("Hello, {}", &name);

        formatter.format(hello)
    })
}
