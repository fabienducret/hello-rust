#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use crate::greetings::greetings::{say_hello_with, Decoration};

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
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "empty name");
        assert_eq!(error.kind(), ErrorKind::InvalidData);
    }
}
