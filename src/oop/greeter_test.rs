#[cfg(test)]
mod tests {
    use crate::oop::greeter::{Formatter, Greeter};

    struct StubFormatter;
    impl Formatter for StubFormatter {
        fn format(&self, value: String) -> String {
            format!("<formatted>{}</formatted>", value)
        }
    }

    #[test]
    fn say_hello_to() {
        // Arrange
        let greeter = Greeter::new(Box::new(StubFormatter));
        let name = String::from("Fabien");

        // Act
        let to_display = greeter.say_hello_to(name);

        // Assert
        assert_eq!(to_display, "<formatted>Hello, Fabien</formatted>");
    }
}
