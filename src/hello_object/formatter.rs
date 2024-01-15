use crate::hello_object::hello::Formatter;

pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, value: String) -> String {
        format!("---> {} <---", &value)
    }
}
