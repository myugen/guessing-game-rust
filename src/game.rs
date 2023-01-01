use crate::io::IO;

pub struct GameEngine {
    io: Box<dyn IO>,
}

impl GameEngine {
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
    use super::GameEngine;
    use crate::io::MockIO;
    use mockall::predicate;

    #[test]
    fn test_run_should() {
        let mut mock = MockIO::new();
        mock.expect_write()
            .times(1)
            .with(predicate::eq("Greetings"));
        let io = Box::new(mock);
        let mut game_engine = GameEngine::new(io);

        game_engine.run()
    }
}
