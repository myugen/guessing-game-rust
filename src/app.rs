use crate::reader::Reader;
use crate::writer::Writer;

pub struct App {
    reader: Box<dyn Reader>,
    writer: Box<dyn Writer>,
}

impl App {
    pub const fn new(reader: Box<dyn Reader>, writer: Box<dyn Writer>) -> Self {
        Self { reader, writer }
    }

    pub fn run(&mut self) {
        self.writer.write(String::from("Guess the number!"));
        self.writer.write(String::from("Please input your guess: "));

        let input = self.reader.read();

        self.writer.write(String::from("selected input ") + &input)
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn test_run_should() {}
}
