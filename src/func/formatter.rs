use crate::func::greeter::Formatter;

pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, value: String) -> String {
        format!("\x1b[34m---> {}\x1b[0m", &value)
    }
}
