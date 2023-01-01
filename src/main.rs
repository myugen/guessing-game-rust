use crate::app::App;
use crate::io::StandardIO;
use std::io::{stdin, stdout};

mod app;

mod io;

fn main() {
    let reader = stdin().lock();
    let writer = stdout().lock();
    let io = StandardIO::new(reader, writer);

    let mut app = App::new(Box::new(io));
    app.run();
}
