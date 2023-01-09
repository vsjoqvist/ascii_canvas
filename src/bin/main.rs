//This is a very simple example on how this could be used

use asciicanvas::{Canvas, Figure, Point};
use std::io::stdout;
use std::thread::sleep;
use std::time::Duration;

const CAT_FIGURE: Figure = Figure::new(
    r"
 /\_/\
( o.o ) 
 > ^ < ");

const OWL_FIGURE: Figure = Figure::new(r"
_ _/|
\'o.0'
=(___)=
   U");

fn main() {
    let mut stdout = stdout();
    let mut canvas = Canvas::new();
    Canvas::init(&mut stdout);

    canvas.add_drawing(CAT_FIGURE, Point { x: 0, y: 2 });
    canvas.add_drawing(OWL_FIGURE, Point { x: 20, y: 5 });
    canvas.draw(&mut stdout);
    sleep(Duration::from_secs(2));
    Canvas::revert(&mut stdout);
}
