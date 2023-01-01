use crate::game::GameEngine;
use crate::io::StandardIO;
use std::io::{stdin, stdout};

mod game;

mod io;

fn main() {
    let reader = stdin().lock();
    let writer = stdout().lock();
    let io = StandardIO::new(reader, writer);

    let mut game_engine = GameEngine::new(Box::new(io));
    game_engine.run();
}
