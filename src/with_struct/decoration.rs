use crate::with_struct::greetings::Decoration;

pub struct ColorDecoration;

impl Decoration for ColorDecoration {
    fn apply_to(&self, value: String) -> String {
        format!("\x1b[31m{}\x1b[0m", &value)
    }
}
