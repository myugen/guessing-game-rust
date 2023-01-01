use std::io::{BufRead, Write};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait IO {
    fn read(&mut self) -> String;
    fn write(&mut self, message: &str);
}

pub struct StandardIO<R, W> {
    reader: R,
    writer: W,
}

impl<R, W> StandardIO<R, W> {
    pub fn new(reader: R, writer: W) -> StandardIO<R, W> {
        StandardIO { reader, writer }
    }
}

impl<R, W> IO for StandardIO<R, W>
where
    R: BufRead,
    W: Write,
{
    fn read(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }

    fn write(&mut self, message: &str) {
        writeln!(&mut self.writer, "{}", message).expect("Failed to write a line")
    }
}

#[cfg(test)]
mod tests {
    use crate::io::IO;

    use super::StandardIO;

    #[test]
    fn test_read_should_return_the_value_from_read_buffer() {
        let input = b"input test";
        let mut output: Vec<u8> = Vec::new();

        let actual = {
            let mut sut = StandardIO::new(&input[..], &mut output);

            sut.read()
        };

        assert_eq!(actual, "input test")
    }

    #[test]
    fn test_write_should_write_a_line_in_the_buffer() {
        let input = b"";
        let mut output: Vec<u8> = Vec::new();

        let mut sut = StandardIO::new(&input[..], &mut output);

        sut.write("write a test message");
        let actual = String::from_utf8(output).expect("None UTF-8 format");

        assert_eq!(actual, "write a test message\n")
    }
}
