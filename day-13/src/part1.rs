use crate::computer::Program;
use crate::arcade::{ArcdeCabinet, TileId};

use std::fs::read_to_string;

pub fn part1() {
    let mut intcode_str = read_to_string("input/input.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();
    let program = Program::new_from_str(intcode_str);

    let mut arcade = ArcdeCabinet::new(program);
    arcade.run();

    let num_blocks = arcade.get_screen().values().map(|&t| {if t == TileId::Block {1} else {0}}).collect::<Vec<i64>>().iter().sum::<i64>();
    println!("Number of block tiles: {}", num_blocks);
}
