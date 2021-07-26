use std::fs::read_to_string;
use crate::computer::Program;
use crate::robot::{PaintingRobot, Color};

pub fn part1() {
    let mut intcode_str = read_to_string("input/input.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();
    let program = Program::new_from_str(intcode_str);
    let mut robo = PaintingRobot::new(program, Color::Black);

    robo.run();
    println!("Painted {} cells at least once", robo.get_grid().len())
}
