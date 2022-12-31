use std::io::{stdin, BufRead};

pub trait Reader {
    fn read(&mut self) -> String;
}

pub struct IOReader {
    buffer: Box<dyn BufRead>,
}

impl IOReader {
    pub fn new() -> Self {
        Self {
            buffer: Box::new(stdin().lock()),
        }
    }
}

impl Reader for IOReader {
    fn read(&mut self) -> String {
        let mut input = String::new();
        self.buffer
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }
}

#[cfg(test)]
mod test {
    use crate::reader::Reader;

    use super::IOReader;

    #[test]
    fn should_return_the_read_buffer() {
        let input = b"test";
        let mut reader = IOReader {
            buffer: Box::new(&input[..]),
        };
        let actual = reader.read();
        assert_eq!(actual, "test")
    }
}
