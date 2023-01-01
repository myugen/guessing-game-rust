use crate::app::App;

mod app;

mod reader;
mod writer;

fn main() {
    let reader = Box::new(reader::IOReader::new());
    let writer = Box::new(writer::IOWriter::new());
    let mut app = App::new(reader, writer);
    app.run();
}
