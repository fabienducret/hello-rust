#[cfg(test)]
mod tests {
    use crate::grettings::greetings::{say_hello_with, Decoration};

    struct StubDecoration;
    impl Decoration for StubDecoration {
        fn apply_to(&self, value: String) -> String {
            format!("--> {} <--", value)
        }
    }

    #[test]
    fn say_hello_to_name() {
        // Arrange
        let say_hello_to = say_hello_with(&StubDecoration);
        let name = String::from("Fabien");

        // Act
        let result = say_hello_to(name);

        // Assert
        assert_eq!(result.unwrap(), "--> Hello Fabien <--");
    }

    #[test]
    fn get_error_for_empty_name() {
        // Arrange
        let say_hello_to = say_hello_with(&StubDecoration);
        let empty_name = String::from("");

        // Act
        let result = say_hello_to(empty_name);

        // Assert
        assert_eq!(result.is_err(), true);
    }
}
