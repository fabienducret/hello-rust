#[cfg(test)]
mod tests {
    use crate::hello_object::hello::{Formatter, Hello};

    struct StubFormatter;
    impl Formatter for StubFormatter {
        fn format(&self, value: String) -> String {
            format!("<formatted>{}</formatted>", value)
        }
    }

    #[test]
    fn say_hello() {
        // Arrange
        let formatter = Box::new(StubFormatter);
        let hello = Hello::new(formatter);

        // Act
        let to_display = hello.to(String::from("Fabien"));

        // Assert
        assert_eq!(to_display, "<formatted>Hello, Fabien</formatted>");
    }
}
