use ascii_canvas::{Canvas, Figure, Point};
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let mut canvas = Canvas::new();
    Canvas::init(&mut stdout);

    const FIGURE: Figure = Figure::new("abc123\n hej");

    canvas.add_figure(FIGURE, Point { x: 205, y: 0 });
    canvas.add_figure(FIGURE, Point { x: 205, y: 0 });
    canvas.add_figure(FIGURE, Point { x: 205, y: 0 });
    canvas.add_figure(FIGURE, Point { x: 205, y: 0 });
    canvas.draw(&mut stdout);
    sleep(Duration::from_secs(2));
    Canvas::revert(&mut stdout);
}
