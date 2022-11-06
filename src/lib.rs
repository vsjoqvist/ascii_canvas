use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{
    size, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::terminal::{Clear, ClearType::All};
use crossterm::{execute, QueueableCommand};
use std::io::{Stdout, Write};
use std::mem::take;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    #[must_use]
    pub fn new(x: u16, y: u16) -> Point {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

pub trait Draw {
    fn draw(&mut self, point: Point, stdout: &mut Stdout);
}

#[derive(Debug)]
pub struct Figure {
    figure: &'static str,
}

impl Figure {
    #[must_use]
    pub const fn new(str: &'static str) -> Figure {
        Figure { figure: str }
    }
}

impl Draw for Figure {
    fn draw(&mut self, pos: Point, stdout: &mut Stdout) {
        let str = self.figure;
        let size = size().unwrap();

        for line in str.lines().enumerate() {
            if pos.y + (line.0 as u16) < size.1 {
                stdout.queue(MoveTo(pos.x, pos.y + line.0 as u16)).unwrap();
                print!("{}", line.1)
            }
        }
    }
}

pub struct Canvas {
    draw_list: Vec<(Box<dyn Draw>, Point)>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    #[must_use]
    pub fn new() -> Canvas {
        Canvas { draw_list: vec![] }
    }

    /// Initializes the terminal for drawing.
    pub fn init(stdout: &mut Stdout) {
        execute!(stdout, EnterAlternateScreen, DisableLineWrap, Hide).unwrap();
    }

    /// Reverts thechanges made by init()
    pub fn revert(stdout: &mut Stdout) {
        execute!(stdout, LeaveAlternateScreen, EnableLineWrap, Show).unwrap();
    }

    pub fn add_figure<T: Draw + 'static>(&mut self, figure: T, position: Point) {
        self.draw_list.push((Box::new(figure), position));
    }

    pub fn draw(&mut self, stdout: &mut Stdout) {
        let draw_list = take(&mut self.draw_list);
        stdout.queue(Clear(All)).unwrap();

        for mut figure in draw_list {
            figure.0.draw(figure.1, stdout);
        }

        stdout.flush().unwrap();
    }
}
