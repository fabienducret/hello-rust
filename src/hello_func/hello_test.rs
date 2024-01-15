#[cfg(test)]
mod tests {
    use crate::hello_func::hello::{hello_factory, Formatter};

    struct StubFormatter;
    impl Formatter for StubFormatter {
        fn format(&self, value: String) -> String {
            format!("{}, formatted", value)
        }
    }

    #[test]
    fn say_hello() {
        // Arrange
        let formatter = &StubFormatter;
        let hello_to = hello_factory(formatter);

        // Act
        let to_display = hello_to(String::from("Fabien"));

        // Assert
        assert_eq!(to_display, "Hello, Fabien, formatted");
    }
}
