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
        let hello = Hello::new(Box::new(StubFormatter));
        let name = String::from("Fabien");

        // Act
        let to_display = hello.to(name);

        // Assert
        assert_eq!(to_display, "<formatted>Hello, Fabien</formatted>");
    }
}
