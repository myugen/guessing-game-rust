use crate::io::IO;

pub struct App {
    io: Box<dyn IO>,
}

impl App {
    pub const fn new(io: Box<dyn IO>) -> Self {
        Self { io }
    }

    pub fn run(&mut self) {
        self.io.write("Guess the number!");
        self.io.write("Please input your guess: ");

        let input = self.io.read();
        let foo = String::from("selected input: ") + &*input;
        self.io.write(foo.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    #[test]
    fn test_run_should() {}
}
