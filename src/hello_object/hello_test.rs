#[cfg(test)]
mod tests {
    use crate::hello_object::hello::{Formatter, Hello};

    struct StubFormatter;
    impl Formatter for StubFormatter {
        fn format(&self, value: String) -> String {
            format!("{}, formatted", value)
        }
    }

    #[test]
    fn say_hello() {
        let formatter = Box::new(StubFormatter);
        let hello = Hello::new(formatter);

        let to_display = hello.to(String::from("Fabien"));

        assert_eq!(to_display, "Hello, Fabien, formatted");
    }
}
