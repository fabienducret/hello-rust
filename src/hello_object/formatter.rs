use crate::hello_object::hello::Formatter;

pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, value: String) -> String {
        format!("\x1b[31m---> {}\x1b[0m", &value)
    }
}
