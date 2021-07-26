use crate::computer::Program;
use crate::arcade::ArcdeCabinet;

use std::fs::read_to_string;

pub fn part2() {
    let mut intcode_str = read_to_string("input/input.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();
    let program = Program::new_from_str(intcode_str);

    let mut arcade = ArcdeCabinet::new(program);
    arcade.insert_coins();
    arcade.run();

    println!("Final Score: {}", arcade.get_score());
}
