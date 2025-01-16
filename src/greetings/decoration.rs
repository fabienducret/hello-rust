use crate::greetings::hello::Decoration;

pub struct StyleDecoration;

impl Decoration for StyleDecoration {
    fn apply_to(&self, value: String) -> String {
        format!("\x1B[1m{}, decorated with style\x1B[0m", &value)
    }
}
