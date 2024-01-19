#[cfg(test)]
mod tests {
    use crate::func::greeter::{say_hello_to_factory, Formatter};

    struct StubFormatter;
    impl Formatter for StubFormatter {
        fn format(&self, value: String) -> String {
            format!("<formatted>{}</formatted>", value)
        }
    }

    #[test]
    fn say_hello_to() {
        // Arrange
        let say_hello_to = say_hello_to_factory(&StubFormatter);
        let name = String::from("Fabien");

        // Act
        let to_display = say_hello_to(name);

        // Assert
        assert_eq!(to_display, "<formatted>Hello, Fabien</formatted>");
    }
}
