use crossterm::execute;
use crossterm::terminal::{DisableLineWrap, EnterAlternateScreen};
use std::io::{stdout, Stdout};
use std::mem::take;
use std::ops::{Add, Div, Mul, Sub};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
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

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Failed")]
    Fail,
}

pub struct Game<'a> {
    stdout: Stdout,
    canvas: Canvas<'a>,
    pub camera_pos: Point,
}

impl<'a> Game<'a> {
    pub fn new(size: (u16, u16)) -> Result<Game<'a>, InitError> {
        Ok(Game {
            stdout: stdout(),
            canvas: Canvas::new(size),
            camera_pos: Point { x: 0, y: 0 },
        })
    }

    pub fn init(&mut self) {
        execute!(self.stdout, DisableLineWrap, EnterAlternateScreen).unwrap();
    }

    pub fn borrow_canvas_mut(&'a mut self) -> &'a mut Canvas {
        &mut self.canvas
    }

    pub fn draw_canvas(&mut self) {
        let canvas = &self.canvas;
        let width = canvas.size.0 as usize;
        for i in 0..canvas.size.1 as usize {
            print!("{}", &canvas.canvas[(width * i)..(width * i + width)]);
        }
    }
}

pub struct Canvas<'a> {
    pub canvas: String,
    pub size: (u16, u16),
    to_draw: Vec<(&'a Figure, Point)>,
}

#[derive(Debug, Error)]
pub enum DrawError {}

impl<'a> Canvas<'a> {
    pub fn new(size: (u16, u16)) -> Canvas<'a> {
        Canvas {
            canvas: Self::empty(size),
            size,
            to_draw: Vec::new(),
        }
    }

    fn empty(size: (u16, u16)) -> String {
        let size = (size.0 * size.1) as usize;
        let mut canvas = String::with_capacity(size);
        for _ in 0..size {
            canvas.push(' ');
        }
        canvas
    }

    ///Adds the figure to the draw list. The figure will be drawned to the screen when draw_canvas is called.
    pub fn add_figure(&mut self, figure: &'a Figure, pos: Point) {
        self.to_draw.push((figure, pos));
    }

    ///Draws all the figures added by Draw to the canvas
    ///
    /// Currently using a negative offset will cause a panic
    pub fn draw_canvas(&mut self, offset: &Point) {
        let mut temp_vec = take(&mut self.to_draw);

        for figure in temp_vec.drain(..) {
            let figure = figure;
            let (figure, pos) = figure;

            let new_canvas_start = *offset;

            let new_canvas_end = Point {
                x: self.size.0 as i32,
                y: self.size.1 as i32,
            } + *offset;

            let corrected_figure_dim = figure.dim - Point::new(1, 0);

            if !(pos.x + corrected_figure_dim.x <= new_canvas_end.x
                || pos.x >= new_canvas_start.x
                    && pos.y + corrected_figure_dim.y <= new_canvas_end.y
                || pos.y >= new_canvas_start.y)
            {
                continue;
            }

            let mut cursor_position = pos;
            for instruction in &figure.instructions {
                match *instruction {
                    FigureInstruction::SkipTo(p) => cursor_position = pos + p,
                    FigureInstruction::Draw(s) => {
                        if cursor_position.y <= new_canvas_end.y
                            && cursor_position.y >= new_canvas_start.y
                        {
                            if cursor_position.x < new_canvas_start.x
                                && cursor_position.x + s.len() as i32 - 1 < new_canvas_start.x
                                || cursor_position.x > new_canvas_end.x
                                    && cursor_position.x + s.len() as i32 - 1 > new_canvas_end.x
                            {
                                continue;
                            }

                            {
                                self.execute_instruction(
                                    s,
                                    &cursor_position,
                                    &new_canvas_start,
                                    &new_canvas_end,
                                    offset,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn execute_instruction(
        &mut self,
        string: &str,
        cursor_position: &Point,
        canvas_start: &Point,
        canvas_end: &Point,
        offset: &Point,
    ) {
        //The first character on the line that should be drawn
        let to_draw_lower_bound = if cursor_position.x >= canvas_start.x {
            0
        } else {
            (canvas_start.x - cursor_position.x).abs()
        };

        //The lowerbound together with the upperbound
        let to_draw_upper_bound = if string.len() as i32 - 1 <= canvas_end.x {
            string.len() as i32
        } else {
            canvas_end.x - cursor_position.x + 1
        };

        let place_at = (cursor_position.x
            + to_draw_lower_bound
            + offset.x
            + (cursor_position.y + offset.y) * i32::from(self.size.0))
            as usize;

        // dbg!(
        //     canvas_start,
        //     canvas_end,
        //     cursor_position,
        //     offset,
        //     to_draw_lower_bound,
        //     to_draw_upper_bound,
        //     place_at,
        //     string.len(),
        //     self.canvas.len(),
        //     self.size,
        // );

        self.canvas.replace_range(
            place_at..place_at + to_draw_upper_bound as usize,
            &string[to_draw_lower_bound as usize..to_draw_upper_bound as usize],
        );
    }
}

pub struct Figure {
    instructions: Vec<FigureInstruction>,
    pub dim: Point,
}

#[derive(Error, Debug)]
pub enum CreateFigureError {
    #[error("The provided string is empty")]
    EmptyString,
}

#[derive(Debug)]
enum FigureInstruction {
    Draw(&'static str),
    SkipTo(Point),
}

impl Figure {
    /// # Errors
    ///
    /// Will return an error if the string is empty
    pub fn new(figure: &'static str) -> Result<Figure, CreateFigureError> {
        if figure.is_empty() {
            return Err(CreateFigureError::EmptyString);
        }
        let mut instructions = Vec::new();

        //let mut figure = String::from(figure);

        let mut width = 0;
        let mut height = 0;

        let mut last_find = 0;
        while let Some(v) = figure[last_find..].find('\n') {
            height += 1;

            instructions.push(FigureInstruction::Draw(&figure[last_find..v + last_find]));
            instructions.push(FigureInstruction::SkipTo(Point::new(0, height)));
            last_find = v + 1;
        }

        if figure.len() != last_find {
            instructions.push(FigureInstruction::Draw(&figure[last_find..]));
        }

        //Calculate the width of the figure
        for instruction in &instructions {
            if let FigureInstruction::Draw(v) = instruction {
                if v.len() > width {
                    width = v.len();
                }
            }
        }

        Ok(Figure {
            instructions,
            dim: Point {
                x: width as i32,
                y: height + 1,
            },
        })
    }
}
