use ascii_canvas::{Canvas, Figure, Point};
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let mut canvas = Canvas::new();
    Canvas::init(&mut stdout);

    const FIGURE: Figure = Figure::new("abc123\nhej");

    canvas.add_drawing(FIGURE, Point { x: 0, y: 10 });
    canvas.draw(&mut stdout);
    sleep(Duration::from_secs(2));
    Canvas::revert(&mut stdout);
}
