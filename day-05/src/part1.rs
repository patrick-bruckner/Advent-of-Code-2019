use std::fs::read_to_string;
use crate::computer::Computer;
use crate::computer::Program;

pub fn part1()
{
    let mut intcode_str = read_to_string("input/part1.txt").unwrap();
    intcode_str = intcode_str.trim_end().to_string();

    let program = Program::new_from_str(intcode_str);
    let mut computer = Computer::new();
    computer.load_program(program);

    println!("Press 1 at the prompt");
    computer.run();
}
