use crate::computer::{Computer, Program};

use std::rc::Rc;
use std::cell::{Cell, RefCell, Ref, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

pub struct ArcdeCabinet<'a> {
    screen: Screen,
    score: Rc<Cell<i64>>,
    computer: Computer<'a>,
    prog: Program
}

type ScreenData = HashMap<Position,TileId>;

#[derive(Clone)]
struct Screen {
    data: Rc<RefCell<ScreenData>>
}

pub struct ScreenDataRef<'a> {
    data_ref: Ref<'a, ScreenData>
}

struct ScreenDataRefMut<'a> {
    data_ref: RefMut<'a, ScreenData>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileId {
    Enpty,
    Wall,
    Block,
    Paddle,
    Ball
}

enum OutputMode {
    XCord,
    YCord,
    TileType
}

#[derive(Clone, Copy)]
enum JoystickPos {
    LEFT = -1,
    NEUTRAL,
    RIGHT
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64
}

impl<'a> ArcdeCabinet<'a> {
    pub fn new(prog: Program) -> Self {
        let screen = Screen::new();
        let mut screen_clone_1 = screen.clone();
        let screen_clone_2 = screen.clone();

        let score = Rc::new(Cell::new(0));
        let score_clone = score.clone();

        let mut computer = Computer::new();
        computer.load_program(&prog);

        let mut output_type = Box::new(OutputMode::XCord);
        let mut pos = Box::new(Position::new(0, 0));
        let set_output = move |o: i64| {
            match *output_type {
                OutputMode::XCord => {
                    pos.x = o;
                    *output_type = OutputMode::YCord;
                },
                OutputMode::YCord => {
                    pos.y = o;
                    *output_type = OutputMode::TileType;
                },
                OutputMode::TileType => {
                    if *pos == Position::new(-1, 0) {
                        score_clone.set(o);
                    } else {
                        let tile_id = match o {
                            0 => TileId::Enpty,
                            1 => TileId::Wall,
                            2 => TileId::Block,
                            3 => TileId::Paddle,
                            4 => TileId::Ball,
                            _ => panic!("Invalid tile id")
                        };
                        screen_clone_1.set_tile(*pos, tile_id);
                    }

                    *output_type = OutputMode::XCord;
                }
            }
        };

        let joystick_pos = Box::new(RefCell::new(JoystickPos::NEUTRAL));
        let get_input = move || {
            let paddle_pos = screen_clone_2.find_tile(TileId::Paddle);
            let ball_pos = screen_clone_2.find_tile(TileId::Ball);

            if paddle_pos.x < ball_pos.x {
                *joystick_pos.borrow_mut() = JoystickPos::RIGHT;
            } else if paddle_pos.x > ball_pos.x {
                *joystick_pos.borrow_mut() = JoystickPos::LEFT;
            } else {
                *joystick_pos.borrow_mut() = JoystickPos::NEUTRAL;
            }

            return *joystick_pos.borrow() as i64;
        };

        computer.set_output(set_output);
        computer.set_input(get_input);

        Self {screen, score, computer, prog}
    }

    pub fn run(&mut self) {
        self.computer.run();
    }

    pub fn step(&mut self) -> bool {
        self.computer.step()
    }

    pub fn insert_coins(&mut self) {
        self.computer.set_value(0, 2);
    }

    pub fn get_screen(&self) -> ScreenDataRef {
        self.screen.borrow_data()
    }

    pub fn get_score(&self) -> i64 {
        self.score.get()
    }

    pub fn reset(&mut self) {
        self.computer.load_program(&self.prog)
    }
}

impl Screen {
    fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    fn set_tile(&mut self, pos: Position, tile: TileId) {
        self.borrow_data_mut().insert(pos, tile);
    }

    fn find_tile(&self, tile_id: TileId) -> Position {
        return *self.borrow_data().iter().filter(|&(_, t)| {*t == tile_id}).collect::<Vec<(&Position,&TileId)>>()[0].0;
    }

    fn borrow_data(&self) -> ScreenDataRef {
        ScreenDataRef {
            data_ref: self.data.borrow()
        }
    }

    fn borrow_data_mut(&self) -> ScreenDataRefMut {
        ScreenDataRefMut {
            data_ref: self.data.borrow_mut()
        }
    }
}

impl<'a> Deref for ScreenDataRef<'a> {
    type Target = ScreenData;

    fn deref(&self) -> &Self::Target {
        &*self.data_ref
    }
}

impl<'a> Deref for ScreenDataRefMut<'a> {
    type Target = ScreenData;

    fn deref(&self) -> &Self::Target {
        &*self.data_ref
    }
}

impl<'a> DerefMut for ScreenDataRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.data_ref
    }
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self {x, y}
    }
}
