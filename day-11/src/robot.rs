use crate::computer::{Computer, Program};

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::ops::Deref;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Clone, Copy, FromPrimitive)]
pub enum Color {
    Black,
    White
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

enum OutputType {
    Paint,
    Turn
}

pub struct Grid {
    current_pos: Point,
    cells: HashMap<Point,Color>,
    dir: Direction
}

pub struct GridRef<'a> {
    grid: Ref<'a, Grid>
}

pub struct PaintingRobot<'a> {
    computer: Computer<'a>,
    grid: Rc<RefCell<Grid>>
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {x, y}
    }

    fn move_pt(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1
        }
    }
}

impl Direction {
    fn turn_left(&mut self) {
        match self {
            Self::Up => *self = Self::Left,
            Self::Right => *self = Self::Up,
            Self::Down => *self = Self::Right,
            Self::Left => *self = Self::Down
        }
    }

    fn turn_right(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up
        }
    }
}

impl OutputType {
    fn switch(&mut self) {
        match self {
            Self::Paint => *self = Self::Turn,
            Self::Turn => *self = Self::Paint
        }
    }
}

impl Grid {
    fn new() -> Self {
        Self {
            current_pos: Point::new(0,0),
            cells: HashMap::new(),
            dir: Direction::Up
        }
    }

    pub fn len(&self) -> usize {
        return self.cells.len();
    }

    pub fn show(&self) {
        // find bounds
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for k in self.cells.keys() {
            if k.x < min_x {
                min_x = k.x;
            }
            if k.x > max_x {
                max_x = k.x;
            }
            if k.y < min_y {
                min_y = k.y;
            }
            if k.y > max_y {
                max_y = k.y;
            }
        }

        for y in (min_y..(max_y+1)).rev() {
            for x in min_x..(max_x+1) {
                let p = Point::new(x, y);
                if let Some(color) = self.cells.get(&p) {
                    match color {
                        Color::Black => print!(" "),
                        Color::White => print!("*")
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl<'a> Deref for GridRef<'a> {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<'a> PaintingRobot<'a> {
    pub fn new(prog: Program, initial_color: Color) -> Self {
        let mut computer = Computer::new();
        computer.load_program(&prog);

        let grid = Rc::new(RefCell::new(Grid::new()));
        let grid_clone_1 = grid.clone();
        let grid_clone_2 = grid.clone();

        let get_input = move || {
            let grid = grid_clone_1.borrow();

            if let Some(color) = grid.cells.get(&grid.current_pos) {
                return *color as i64;
            } else {
                if grid.current_pos == Point::new(0,0) {
                    return initial_color as i64;
                } else {
                    return Color::Black as i64;
                }
            }
        };

        // using refcell in box so set_output can be Fn not FnMut
        let out_type = Box::new(RefCell::new(OutputType::Paint));
        let set_output = move |val: i64| {
            let mut grid = grid_clone_2.borrow_mut();
            let pos = grid.current_pos;

            match *out_type.borrow() {
                OutputType::Paint => {
                    grid.cells.insert(pos, FromPrimitive::from_i64(val).unwrap());
                },
                OutputType::Turn => {
                    match val {
                        0 => grid.dir.turn_left(),
                        1 => grid.dir.turn_right(),
                        _ => panic!("Bad turn val {}", val)
                    }
                    let new_dir = grid.dir;
                    grid.current_pos.move_pt(new_dir);
                }
            }

            out_type.borrow_mut().switch();
        };

        computer.set_input(get_input);
        computer.set_output(set_output);

        Self {
            computer,
            grid
        }
    }

    pub fn run(&mut self) {
        self.computer.run();
    }

    pub fn get_grid(&self) -> GridRef {
        GridRef {
            grid: self.grid.borrow()
        }
    }
}
