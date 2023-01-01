use std::io::{stdout, Write};

pub trait Writer {
    fn write(&mut self, message: String);
}

pub struct IOWriter {
    buffer: Box<dyn Write>,
}

impl IOWriter {
    pub fn new() -> Self {
        Self {
            buffer: Box::new(stdout().lock()),
        }
    }
}

impl Writer for IOWriter {
    fn write(&mut self, message: String) {
        writeln!(&mut self.buffer, "{}", message).expect("Failed to write a line")
    }
}

#[cfg(test)]
mod tests {
    use crate::writer::Writer;

    use super::IOWriter;

    #[test]
    fn should_write_in_the_buffer() {
        let output = Vec::new();
        let mut writer = IOWriter {
            buffer: Box::new(output),
        };

        writer.write(String::from("test"));

        assert_eq!(
            String::from_utf8(output).expect("Not an UTF-8 message"),
            "test"
        )
    }
}
