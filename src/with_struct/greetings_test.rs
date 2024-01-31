#[cfg(test)]
mod tests {
    use crate::with_struct::greetings::{Decoration, Greetings};

    struct StubDecoration;
    impl Decoration for StubDecoration {
        fn apply_to(&self, value: String) -> String {
            format!("--> {} <--", value)
        }
    }

    #[test]
    fn say_hello_to() {
        // Arrange
        let greetings = Greetings::new(Box::new(StubDecoration));
        let name = String::from("Fabien");

        // Act
        let to_display = greetings.say_hello_to(name);

        // Assert
        assert_eq!(to_display, "--> Hello Fabien <--");
    }
}
