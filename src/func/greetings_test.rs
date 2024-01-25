#[cfg(test)]
mod tests {
    use crate::func::greetings::{say_hello_to_factory, Decoration};

    struct StubDecoration;
    impl Decoration for StubDecoration {
        fn apply_to(&self, value: String) -> String {
            format!("--> {} <--", value)
        }
    }

    #[test]
    fn say_hello_to() {
        // Arrange
        let say_hello_to = say_hello_to_factory(&StubDecoration);
        let name = String::from("Fabien");

        // Act
        let to_display = say_hello_to(name);

        // Assert
        assert_eq!(to_display, "--> Hello, Fabien <--");
    }
}
